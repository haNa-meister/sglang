//! Load-aware policy: always picks the least loaded worker.
//!
//! Unlike power-of-two which samples 2 random workers, this policy scans all
//! healthy workers and selects the one with the minimum load. It maintains:
//!
//! 1. **Cached loads** from LoadMonitor (real server data, overwritten each poll)
//! 2. **Local dispatch deltas** incremented on each dispatch, reset on poll
//!
//! Effective load = cached_load + local_delta (between polls)
//! On poll: cached_load = real server data, local_delta = 0
//!
//! Falls back to worker.load() (active request count) when no cached data is available.

use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicIsize, Ordering},
        Arc, RwLock,
    },
};

use tracing::debug;

use super::{get_healthy_worker_indices, LoadBalancingPolicy, SelectWorkerInfo};
use crate::core::Worker;

/// Per-worker load tracking state
struct WorkerLoadState {
    /// Last known load from LoadMonitor (overwritten each poll)
    cached_load: AtomicIsize,
    /// Local dispatch delta since last poll (reset on each poll)
    dispatch_delta: AtomicIsize,
}

impl WorkerLoadState {
    fn new() -> Self {
        Self {
            cached_load: AtomicIsize::new(-1), // -1 = no cached data yet
            dispatch_delta: AtomicIsize::new(0),
        }
    }

    /// Get effective load: cached + delta, or -1 if no cached data
    fn effective_load(&self) -> isize {
        let cached = self.cached_load.load(Ordering::Relaxed);
        if cached < 0 {
            return -1; // No cached data
        }
        cached + self.dispatch_delta.load(Ordering::Relaxed)
    }

    /// Add estimated tokens to dispatch delta (called on each request dispatch).
    /// Poll overwrites and resets to 0.
    fn add_delta(&self, estimated_tokens: isize) {
        self.dispatch_delta.fetch_add(estimated_tokens, Ordering::Relaxed);
    }

    /// Overwrite with real data from LoadMonitor and reset delta
    fn overwrite(&self, real_load: isize) {
        self.cached_load.store(real_load, Ordering::Relaxed);
        self.dispatch_delta.store(0, Ordering::Relaxed);
    }
}

impl std::fmt::Debug for WorkerLoadState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WorkerLoadState")
            .field("cached_load", &self.cached_load.load(Ordering::Relaxed))
            .field(
                "dispatch_delta",
                &self.dispatch_delta.load(Ordering::Relaxed),
            )
            .finish()
    }
}

/// Load-aware policy that always routes to the least loaded worker.
///
/// Combines real server load data from LoadMonitor with local dispatch tracking
/// to provide accurate load estimation between polling intervals.
#[derive(Debug)]
pub struct LoadAwarePolicy {
    /// Per-worker load state, keyed by worker URL
    worker_loads: RwLock<HashMap<String, Arc<WorkerLoadState>>>,
}

impl LoadAwarePolicy {
    pub fn new() -> Self {
        Self {
            worker_loads: RwLock::new(HashMap::new()),
        }
    }

    /// Get or create load state for a worker URL
    fn get_or_create_state(&self, url: &str) -> Arc<WorkerLoadState> {
        // Fast path: read lock
        if let Ok(loads) = self.worker_loads.read() {
            if let Some(state) = loads.get(url) {
                return Arc::clone(state);
            }
        }
        // Slow path: write lock
        if let Ok(mut loads) = self.worker_loads.write() {
            loads
                .entry(url.to_string())
                .or_insert_with(|| Arc::new(WorkerLoadState::new()))
                .clone()
        } else {
            Arc::new(WorkerLoadState::new())
        }
    }
}

impl LoadBalancingPolicy for LoadAwarePolicy {
    fn select_worker(
        &self,
        workers: &[Arc<dyn Worker>],
        info: &SelectWorkerInfo,
    ) -> Option<usize> {
        let healthy_indices = get_healthy_worker_indices(workers);

        if healthy_indices.is_empty() {
            return None;
        }

        // Estimate tokens from request text (chars / 4 is a rough approximation)
        let estimated_tokens = info
            .request_text
            .map(|text| (text.len() / 4).max(1) as isize)
            .unwrap_or(1);

        if healthy_indices.len() == 1 {
            let idx = healthy_indices[0];
            let state = self.get_or_create_state(workers[idx].url());
            state.add_delta(estimated_tokens);
            workers[idx].increment_processed();
            return Some(idx);
        }

        // Find the worker with the minimum token load across ALL healthy workers
        let mut min_load = isize::MAX;
        let mut min_idx = healthy_indices[0];
        let mut used_cached = false;

        for &idx in &healthy_indices {
            let worker = &workers[idx];
            let state = self.get_or_create_state(worker.url());
            let effective = state.effective_load();

            let load = if effective >= 0 {
                // Use cached num_tokens + local token delta
                used_cached = true;
                effective
            } else {
                // No cached data yet: use local token delta only
                state.dispatch_delta.load(Ordering::Relaxed)
            };

            if load < min_load {
                min_load = load;
                min_idx = idx;
            }
        }

        // Add estimated tokens to dispatch delta for the selected worker
        let state = self.get_or_create_state(workers[min_idx].url());
        state.add_delta(estimated_tokens);

        debug!(
            "Load-aware selection: selected {} with load {} (est_tokens={}, out of {} healthy, cached={})",
            workers[min_idx].url(),
            min_load,
            estimated_tokens,
            healthy_indices.len(),
            used_cached,
        );

        workers[min_idx].increment_processed();
        Some(min_idx)
    }

    fn needs_request_text(&self) -> bool {
        true
    }

    fn name(&self) -> &'static str {
        "load_aware"
    }

    fn update_loads(&self, loads: &HashMap<String, isize>) {
        // Overwrite local state with real server data and reset dispatch deltas.
        // First ensure all workers have state entries (write lock)
        if let Ok(mut worker_loads) = self.worker_loads.write() {
            for (url, &real_load) in loads {
                if real_load < 0 {
                    continue; // Skip failed fetches
                }
                let state = worker_loads
                    .entry(url.clone())
                    .or_insert_with(|| Arc::new(WorkerLoadState::new()));
                state.overwrite(real_load);
            }
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Default for LoadAwarePolicy {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{BasicWorkerBuilder, WorkerType};

    fn create_workers(urls: &[&str]) -> Vec<Arc<dyn Worker>> {
        urls.iter()
            .map(|url| {
                Arc::new(
                    BasicWorkerBuilder::new(*url)
                        .worker_type(WorkerType::Regular)
                        .build(),
                ) as Arc<dyn Worker>
            })
            .collect()
    }

    #[test]
    fn test_selects_least_loaded() {
        let policy = LoadAwarePolicy::new();
        let workers = create_workers(&["http://w1:8000", "http://w2:8000", "http://w3:8000"]);

        let mut loads = HashMap::new();
        loads.insert("http://w1:8000".to_string(), 100);
        loads.insert("http://w2:8000".to_string(), 50);
        loads.insert("http://w3:8000".to_string(), 10);
        policy.update_loads(&loads);

        let info = SelectWorkerInfo::default();
        let idx = policy.select_worker(&workers, &info).unwrap();
        assert_eq!(idx, 2, "Should select worker with lowest load (w3)");
    }

    #[test]
    fn test_dispatch_delta_distributes_requests() {
        let policy = LoadAwarePolicy::new();
        let workers = create_workers(&["http://w1:8000", "http://w2:8000"]);

        // Both at 0 load
        let mut loads = HashMap::new();
        loads.insert("http://w1:8000".to_string(), 0);
        loads.insert("http://w2:8000".to_string(), 0);
        policy.update_loads(&loads);

        let info = SelectWorkerInfo::default();

        // First request -> w1 (tie: first wins, delta becomes 1)
        let idx1 = policy.select_worker(&workers, &info).unwrap();
        assert_eq!(idx1, 0);

        // Second request -> w2 (w1=1, w2=0)
        let idx2 = policy.select_worker(&workers, &info).unwrap();
        assert_eq!(idx2, 1);

        // Third request -> w1 (both at 1 now, tie: first wins)
        let idx3 = policy.select_worker(&workers, &info).unwrap();
        assert_eq!(idx3, 0);
    }

    #[test]
    fn test_poll_overwrites_and_resets_delta() {
        let policy = LoadAwarePolicy::new();
        let workers = create_workers(&["http://w1:8000", "http://w2:8000"]);

        // Initial poll
        let mut loads = HashMap::new();
        loads.insert("http://w1:8000".to_string(), 0);
        loads.insert("http://w2:8000".to_string(), 0);
        policy.update_loads(&loads);

        let info = SelectWorkerInfo::default();

        // Dispatch 5 requests (alternating due to delta tracking)
        for _ in 0..5 {
            policy.select_worker(&workers, &info);
        }

        // New poll with real data - should overwrite and reset deltas
        loads.insert("http://w1:8000".to_string(), 2);
        loads.insert("http://w2:8000".to_string(), 10);
        policy.update_loads(&loads);

        // Next request should go to w1 (load 2 < load 10)
        let idx = policy.select_worker(&workers, &info).unwrap();
        assert_eq!(idx, 0, "After poll overwrite, should select w1 (lower real load)");
    }

    #[test]
    fn test_token_estimation_from_text() {
        let policy = LoadAwarePolicy::new();
        let workers = create_workers(&["http://w1:8000", "http://w2:8000"]);

        // Both at 0 token load
        let mut loads = HashMap::new();
        loads.insert("http://w1:8000".to_string(), 0);
        loads.insert("http://w2:8000".to_string(), 0);
        policy.update_loads(&loads);

        // Send a request with ~400 chars -> ~100 estimated tokens
        let text = "a".repeat(400);
        let info = SelectWorkerInfo {
            request_text: Some(&text),
            ..Default::default()
        };

        // First request -> w1 (tie), adds ~100 token delta
        let idx1 = policy.select_worker(&workers, &info).unwrap();
        assert_eq!(idx1, 0);

        // Second request -> w2 (w1 has 100 token delta, w2 has 0)
        let idx2 = policy.select_worker(&workers, &info).unwrap();
        assert_eq!(idx2, 1);
    }

    #[test]
    fn test_distributes_before_first_poll() {
        let policy = LoadAwarePolicy::new();
        let workers = create_workers(&["http://w1:8000", "http://w2:8000", "http://w3:8000"]);

        // No update_loads called - should still distribute via dispatch deltas
        let info = SelectWorkerInfo::default();
        let mut counts = [0usize; 3];
        for _ in 0..6 {
            let idx = policy.select_worker(&workers, &info).unwrap();
            counts[idx] += 1;
        }

        // Should distribute evenly even without cached data
        assert_eq!(counts, [2, 2, 2], "Should distribute evenly before first poll");
    }

    #[test]
    fn test_single_worker() {
        let policy = LoadAwarePolicy::new();
        let workers = create_workers(&["http://w1:8000"]);

        assert_eq!(
            policy.select_worker(&workers, &SelectWorkerInfo::default()),
            Some(0)
        );
    }

    #[test]
    fn test_skips_negative_loads() {
        let policy = LoadAwarePolicy::new();

        let mut loads = HashMap::new();
        loads.insert("http://w1:8000".to_string(), 10);
        loads.insert("http://w2:8000".to_string(), -1); // failed fetch
        policy.update_loads(&loads);

        // w1 should have cached data, w2 should not
        let w1_state = policy.get_or_create_state("http://w1:8000");
        let w2_state = policy.get_or_create_state("http://w2:8000");
        assert_eq!(w1_state.effective_load(), 10);
        assert_eq!(w2_state.effective_load(), -1); // no cached data
    }

    #[test]
    fn test_rapid_dispatches_between_polls() {
        let policy = LoadAwarePolicy::new();
        let workers =
            create_workers(&["http://w1:8000", "http://w2:8000", "http://w3:8000"]);

        // Initial poll: all at 0
        let mut loads = HashMap::new();
        loads.insert("http://w1:8000".to_string(), 0);
        loads.insert("http://w2:8000".to_string(), 0);
        loads.insert("http://w3:8000".to_string(), 0);
        policy.update_loads(&loads);

        let info = SelectWorkerInfo::default();

        // Dispatch 6 requests rapidly (should round-robin via deltas)
        let mut counts = [0usize; 3];
        for _ in 0..6 {
            let idx = policy.select_worker(&workers, &info).unwrap();
            counts[idx] += 1;
        }

        // Each worker should get exactly 2 requests
        assert_eq!(counts, [2, 2, 2], "Should distribute evenly between polls");
    }
}
