//! Configuration and statistics for garbage collection.

pub(crate) const AUTO_THREADS: usize = 0;
const DEFAULT_ALLOCATION_THRESHOLD_BYTES: usize = 8 * 1024 * 1024;
const DEFAULT_MAX_PAUSE_TIME_US: u64 = 100;
const DEFAULT_INCREMENTAL_STEP_SIZE: usize = 1_000;
const DEFAULT_PARALLEL_THRESHOLD: usize = 1_000_000;

/// Storage policy for owning roots; explicit numeric root IDs retain their tree registry.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum RootStorage {
    /// Existing shared tree. Default until slot performance is qualified.
    #[default]
    Tree,
    /// Reusable slots in sharded pages, independent of OS-thread or task identity.
    Slots,
    /// Registered execution contexts own slots; unscoped operations use shared slots.
    Contexts,
}

/// Configuration for the garbage collector
#[derive(Debug, Clone)]
#[expect(
    clippy::struct_excessive_bools,
    reason = "independent feature switches"
)]
pub struct Configuration {
    /// Partial Linux pause telemetry, preallocated before collector construction.
    #[cfg(all(
        feature = "experimental-pause-recorder",
        target_os = "linux",
        target_has_atomic = "64"
    ))]
    pub pause_recorder: Option<std::sync::Arc<crate::pause_recorder::PauseRecorder>>,
    /// Enable detailed timing and contention counters. Disabled to avoid diagnostic overhead.
    pub diagnostics: bool,
    /// Opt in to sampled full-cycle headroom prediction. Disabled until qualified.
    /// Ignored for an unbounded or zero-size managed heap.
    pub predictive_collection: bool,
    /// Experimental native worker activation/pacing from debt, queued work and available CPUs.
    /// Ignored on cooperative wasm; disabled until performance qualification.
    pub adaptive_workers: bool,
    /// Owning-root storage policy. The slot backend is experimental.
    pub root_storage: RootStorage,
    /// Maximum records per execution context, in 0..=64. Zero keeps barriers immediate.
    pub barrier_buffer_size: usize,
    /// Opt in to bounded context-local payload/descriptor ranges and logical credits.
    pub local_allocation_buffers: bool,
    /// Number of native workers. Zero selects one worker. Wasm uses cooperative callers.
    pub threads: usize,
    /// Native pacing between productive batches; bypassed under heap/recovery pressure.
    /// Also caps backoff after coordinator contention, including under pressure.
    pub worker_yield_time_us: u64,
    /// Target allocation rate threshold before triggering collection (bytes)
    pub allocation_threshold: usize,
    /// Soft time target per step (microseconds); checked between bounded work units.
    /// Zero disables time checks. Bare wasm uses only the work budget.
    pub max_pause_time_us: u64,
    /// Work units per step, including reference slots, roots, and sweep entries.
    pub incremental_step_size: usize,
    /// Opt in to bounded marking steps that claim work without the coordinator.
    /// Phase transitions and reference processing remain coordinated. Default: false.
    pub independent_marking: bool,
    /// Experimental root/reference/sweep units outside the global coordinator.
    /// Per-domain cycle locks serialize cursors; phase transitions remain coordinated.
    pub independent_phases: bool,
    /// Experimental counted publication admission without the global writer. Default: false.
    pub concurrent_publication: bool,
    /// Experimental young-cycle progress during full sweeping and relocation.
    /// Strong tracing and reference decisions retain their generation interlocks.
    pub overlapping_generations: bool,
    /// Experimental old-only passes between precise full collections. Zero disables them.
    /// Nonzero values request an old pass every N young completions; full intervals win.
    /// Old passes conservatively retain young snapshots and conditional old referents.
    /// Enables independent phase units and generation overlap; full APIs remain precise.
    pub old_collection_interval: usize,
    /// Experimental relocation claims without the phase coordinator. Default: false.
    /// Uses persistent local queues when work stealing is enabled.
    pub independent_relocation: bool,
    /// Experimental persistent worker tracing queues with bounded stealing. Default: false.
    /// At most sixteen local queues plus an injector per generation; wasm uses one queue.
    pub work_stealing: bool,
    /// Object, identity, young and remembered index shards. Rounded up to a power of two in 1..=16.
    /// Default: 1; larger counts are experimental.
    pub metadata_shards: usize,
    /// Independently locked payload/descriptor region registries, rounded to 1..=16.
    /// Experimental; all shards share the same physical cap and evacuation quota.
    pub region_shards: usize,
    /// Maximum tracing tasks claimed together, clamped to 1..=8. Default: 1.
    /// Tasks share the step work/time budgets; larger batches are experimental.
    pub mark_batch_size: usize,
    /// Reserved for source compatibility. Worker count controls parallel tracing.
    pub parallel_threshold: usize,
    /// Maximum live managed bytes. `usize::MAX` disables the limit.
    pub max_heap_bytes: usize,
    /// Number of young collections an allocation survives before promotion.
    pub promotion_age: usize,
    /// Run a full collection after this many young collections.
    pub full_collection_interval: usize,
    /// Opt in to region-backed Java payloads and incremental compaction.
    pub compact_heap: bool,
    /// Experimental cost-ranked full-cycle evacuation. Default: false.
    pub cost_aware_relocation: bool,
    /// Shared backing reserved for evacuation, clamped to half the heap limit.
    /// Effective only with compaction and cost-aware relocation. Default: zero.
    pub relocation_reserve_bytes: usize,
    /// Normal region size in bytes.
    pub region_size: usize,
    /// Maximum payload bytes copied per relocation work unit.
    pub relocation_chunk_bytes: usize,
}

impl Default for Configuration {
    /// Creates a default configuration for the garbage collector.
    /// This sets:
    /// - `threads` to `0` (one native worker)
    /// - `allocation_threshold` to 8MB
    /// - `max_pause_time_us` to 100 microseconds
    /// - `incremental_step_size` to 1000 work units
    fn default() -> Self {
        Self {
            #[cfg(all(
                feature = "experimental-pause-recorder",
                target_os = "linux",
                target_has_atomic = "64"
            ))]
            pause_recorder: None,
            diagnostics: false,
            predictive_collection: false,
            adaptive_workers: false,
            root_storage: RootStorage::Tree,
            barrier_buffer_size: 0,
            local_allocation_buffers: false,
            threads: AUTO_THREADS,
            worker_yield_time_us: 50,
            allocation_threshold: DEFAULT_ALLOCATION_THRESHOLD_BYTES,
            max_pause_time_us: DEFAULT_MAX_PAUSE_TIME_US,
            incremental_step_size: DEFAULT_INCREMENTAL_STEP_SIZE,
            independent_marking: false,
            independent_phases: false,
            concurrent_publication: false,
            overlapping_generations: false,
            old_collection_interval: 0,
            independent_relocation: false,
            work_stealing: false,
            metadata_shards: 1,
            region_shards: 1,
            mark_batch_size: 1,
            parallel_threshold: DEFAULT_PARALLEL_THRESHOLD,
            max_heap_bytes: if cfg!(target_family = "wasm") {
                256 * 1024 * 1024
            } else {
                512 * 1024 * 1024
            },
            promotion_age: 2,
            full_collection_interval: 16,
            compact_heap: false,
            cost_aware_relocation: false,
            relocation_reserve_bytes: 0,
            region_size: 1024 * 1024,
            relocation_chunk_bytes: 16 * 1024,
        }
    }
}

impl Configuration {
    /// Creates a new `ConfigurationBuilder` with default values.
    #[must_use]
    pub fn builder() -> ConfigurationBuilder {
        ConfigurationBuilder::new()
    }
}

/// Builder for creating a `Configuration` with a fluent interface.
///
/// # Example
///
/// ```rust
/// use ristretto_gc::ConfigurationBuilder;
///
/// let config = ConfigurationBuilder::new()
///     .threads(4)
///     .allocation_threshold(16 * 1024 * 1024)
///     .max_pause_time_us(50)
///     .build();
/// ```
#[derive(Debug, Clone)]
#[expect(
    clippy::struct_excessive_bools,
    reason = "independent feature switches"
)]
pub struct ConfigurationBuilder {
    diagnostics: bool,
    predictive_collection: bool,
    adaptive_workers: bool,
    root_storage: RootStorage,
    barrier_buffer_size: usize,
    local_allocation_buffers: bool,
    threads: usize,
    worker_yield_time_us: u64,
    allocation_threshold: usize,
    max_pause_time_us: u64,
    incremental_step_size: usize,
    independent_marking: bool,
    independent_phases: bool,
    concurrent_publication: bool,
    overlapping_generations: bool,
    old_collection_interval: usize,
    independent_relocation: bool,
    work_stealing: bool,
    metadata_shards: usize,
    region_shards: usize,
    mark_batch_size: usize,
    parallel_threshold: usize,
    max_heap_bytes: usize,
    promotion_age: usize,
    full_collection_interval: usize,
    compact_heap: bool,
    cost_aware_relocation: bool,
    relocation_reserve_bytes: usize,
    region_size: usize,
    relocation_chunk_bytes: usize,
}

impl Default for ConfigurationBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfigurationBuilder {
    /// Creates a new `ConfigurationBuilder` with default values.
    ///
    /// Default values:
    /// - `threads`: `0` (one native worker)
    /// - `allocation_threshold`: 8MB
    /// - `max_pause_time_us`: 100 microseconds
    /// - `incremental_step_size`: 1000 work units
    /// - `parallel_threshold`: 1,000,000 objects
    #[must_use]
    pub fn new() -> Self {
        let defaults = Configuration::default();
        Self {
            diagnostics: defaults.diagnostics,
            predictive_collection: defaults.predictive_collection,
            adaptive_workers: defaults.adaptive_workers,
            root_storage: defaults.root_storage,
            barrier_buffer_size: defaults.barrier_buffer_size,
            local_allocation_buffers: defaults.local_allocation_buffers,
            threads: defaults.threads,
            worker_yield_time_us: defaults.worker_yield_time_us,
            allocation_threshold: defaults.allocation_threshold,
            max_pause_time_us: defaults.max_pause_time_us,
            incremental_step_size: defaults.incremental_step_size,
            independent_marking: defaults.independent_marking,
            independent_phases: defaults.independent_phases,
            concurrent_publication: defaults.concurrent_publication,
            overlapping_generations: defaults.overlapping_generations,
            old_collection_interval: defaults.old_collection_interval,
            independent_relocation: defaults.independent_relocation,
            work_stealing: defaults.work_stealing,
            metadata_shards: defaults.metadata_shards,
            region_shards: defaults.region_shards,
            mark_batch_size: defaults.mark_batch_size,
            parallel_threshold: defaults.parallel_threshold,
            max_heap_bytes: defaults.max_heap_bytes,
            promotion_age: defaults.promotion_age,
            full_collection_interval: defaults.full_collection_interval,
            compact_heap: defaults.compact_heap,
            cost_aware_relocation: defaults.cost_aware_relocation,
            relocation_reserve_bytes: defaults.relocation_reserve_bytes,
            region_size: defaults.region_size,
            relocation_chunk_bytes: defaults.relocation_chunk_bytes,
        }
    }

    /// Enables optional allocation-free timing and contention diagnostics.
    #[must_use]
    pub fn diagnostics(mut self, enabled: bool) -> Self {
        self.diagnostics = enabled;
        self
    }

    /// Enables sampled headroom prediction for full collections. The default is false.
    /// Finite heaps use allocation rate and observed reclamation latency; clock-free
    /// hosts use a deterministic occupancy policy. Allocation recovery remains enabled.
    #[must_use]
    pub fn predictive_collection(mut self, enabled: bool) -> Self {
        self.predictive_collection = enabled;
        self
    }

    /// Enable native debt/backlog-based worker activation with hysteresis.
    /// Worker count is capped by configured threads and available CPU parallelism.
    /// Cooperative wasm retains its deterministic work budget. Default: false.
    #[must_use]
    pub fn adaptive_workers(mut self, enabled: bool) -> Self {
        self.adaptive_workers = enabled;
        self
    }

    /// Selects storage for owning handles and suspended roots. Explicit numeric
    /// root IDs keep their compatibility registry under either policy.
    #[must_use]
    pub fn root_storage(mut self, storage: RootStorage) -> Self {
        self.root_storage = storage;
        self
    }

    /// Buffer up to 64 mutation records per registered execution context. Default: 0.
    /// Buffers remain visible to collection while their task is suspended or cancelled.
    #[must_use]
    pub fn barrier_buffer_size(mut self, size: usize) -> Self {
        self.barrier_buffer_size = size.min(crate::context::MAX_RECORDS);
        self
    }

    /// Reserve at most two 16 KiB ranges and 4 KiB of unused logical credit
    /// per execution context. Default: false, pending performance qualification.
    #[must_use]
    pub fn local_allocation_buffers(mut self, enabled: bool) -> Self {
        self.local_allocation_buffers = enabled;
        self
    }

    /// Sets the number of native tracing workers (zero selects one).
    #[must_use]
    pub fn threads(mut self, threads: usize) -> Self {
        self.threads = threads;
        self
    }

    /// Sets native worker pacing in microseconds (capped at 1 ms). Zero yields without sleeping.
    #[must_use]
    pub fn worker_yield_time_us(mut self, micros: u64) -> Self {
        self.worker_yield_time_us = micros;
        self
    }

    /// Sets the allocation threshold in bytes before triggering collection.
    #[must_use]
    pub fn allocation_threshold(mut self, threshold: usize) -> Self {
        self.allocation_threshold = threshold;
        self
    }

    /// Sets the soft time target per incremental step in microseconds.
    #[must_use]
    pub fn max_pause_time_us(mut self, pause_time: u64) -> Self {
        self.max_pause_time_us = pause_time;
        self
    }

    /// Sets the maximum work units per incremental step.
    #[must_use]
    pub fn incremental_step_size(mut self, step_size: usize) -> Self {
        self.incremental_step_size = step_size;
        self
    }

    /// Enables bounded marking steps without the coordinator. Default: false.
    /// Claims share a short read lock; phase closure excludes claims and checks in-flight work.
    #[must_use]
    pub fn independent_marking(mut self, enabled: bool) -> Self {
        self.independent_marking = enabled;
        self
    }

    /// Process bounded per-domain roots, references and reclamation without the coordinator.
    #[must_use]
    pub fn independent_phases(mut self, enabled: bool) -> Self {
        self.independent_phases = enabled;
        self
    }

    /// Enables experimental counted publication admission. Disabled until qualified.
    #[must_use]
    pub fn concurrent_publication(mut self, enabled: bool) -> Self {
        self.concurrent_publication = enabled;
        self
    }

    /// Allow young reclamation during full-cycle sweeping/relocation. Default: false.
    /// This does not enable old-only tracing or remove reference-stage interlocks.
    #[must_use]
    pub fn overlapping_generations(mut self, enabled: bool) -> Self {
        self.overlapping_generations = enabled;
        self
    }

    /// Enable old-only passes every N young completions; zero disables them.
    /// Precise full collections still process references and cross-generation cycles.
    #[must_use]
    pub fn old_collection_interval(mut self, interval: usize) -> Self {
        self.old_collection_interval = interval;
        self
    }

    /// Enable distributed relocation continuations without the phase coordinator.
    /// Phase enumeration and completion retain coordinated checks. Default: false.
    #[must_use]
    pub fn independent_relocation(mut self, enabled: bool) -> Self {
        self.independent_relocation = enabled;
        self
    }

    /// Enables persistent worker tracing queues and bounded stealing. Default: false.
    /// Compatible with either marking policy; cooperative wasm retains a single queue.
    #[must_use]
    pub fn work_stealing(mut self, enabled: bool) -> Self {
        self.work_stealing = enabled;
        self
    }

    /// Sets object, identity, young and remembered index shards (power of two in 1..=16).
    /// One shard is the default until the experimental policy is qualified.
    #[must_use]
    pub fn metadata_shards(mut self, count: usize) -> Self {
        self.metadata_shards = crate::index::shard_count(count);
        self
    }

    /// Select independently locked region registries; default is one.
    #[must_use]
    pub fn region_shards(mut self, count: usize) -> Self {
        self.region_shards = crate::index::shard_count(count);
        self
    }

    /// Sets the maximum tracing tasks claimed together (1..=8). Default: 1.
    /// All tasks still share the step work and time budgets.
    #[must_use]
    pub fn mark_batch_size(mut self, tasks: usize) -> Self {
        self.mark_batch_size = tasks.clamp(1, 8);
        self
    }

    /// Stores the legacy threshold; worker count controls parallel tracing.
    #[must_use]
    pub fn parallel_threshold(mut self, threshold: usize) -> Self {
        self.parallel_threshold = threshold;
        self
    }

    /// Sets the maximum number of live managed bytes.
    #[must_use]
    pub fn max_heap_bytes(mut self, bytes: usize) -> Self {
        self.max_heap_bytes = bytes;
        self
    }

    /// Sets the number of young collections before promotion (at least one).
    #[must_use]
    pub fn promotion_age(mut self, age: usize) -> Self {
        self.promotion_age = age.max(1);
        self
    }

    /// Sets the interval between full collections (at least one).
    #[must_use]
    pub fn full_collection_interval(mut self, interval: usize) -> Self {
        self.full_collection_interval = interval.max(1);
        self
    }

    /// Enables region-backed payloads and incremental compaction.
    #[must_use]
    pub fn compact_heap(mut self, enabled: bool) -> Self {
        self.compact_heap = enabled;
        self
    }

    /// Enables experimental region selection and evacuation during full cycles.
    #[must_use]
    pub fn cost_aware_relocation(mut self, enabled: bool) -> Self {
        self.cost_aware_relocation = enabled;
        self
    }

    /// Reserves shared payload/descriptor backing for evacuation. The collector
    /// clamps this to half the heap limit and enables it only with the cost policy.
    #[must_use]
    pub fn relocation_reserve_bytes(mut self, bytes: usize) -> Self {
        self.relocation_reserve_bytes = bytes;
        self
    }

    /// Sets region size and the per-work-unit copy budget in bytes.
    #[must_use]
    pub fn region_layout(mut self, region_size: usize, copy_bytes: usize) -> Self {
        self.region_size = region_size.max(1);
        self.relocation_chunk_bytes = copy_bytes.max(1);
        self
    }

    /// Builds the `Configuration` from this builder.
    #[must_use]
    pub fn build(self) -> Configuration {
        Configuration {
            #[cfg(all(
                feature = "experimental-pause-recorder",
                target_os = "linux",
                target_has_atomic = "64"
            ))]
            pause_recorder: None,
            diagnostics: self.diagnostics,
            predictive_collection: self.predictive_collection,
            adaptive_workers: self.adaptive_workers,
            root_storage: self.root_storage,
            barrier_buffer_size: self.barrier_buffer_size,
            local_allocation_buffers: self.local_allocation_buffers,
            threads: self.threads,
            worker_yield_time_us: self.worker_yield_time_us,
            allocation_threshold: self.allocation_threshold,
            max_pause_time_us: self.max_pause_time_us,
            incremental_step_size: self.incremental_step_size,
            independent_marking: self.independent_marking,
            independent_phases: self.independent_phases || self.old_collection_interval != 0,
            concurrent_publication: self.concurrent_publication,
            overlapping_generations: self.overlapping_generations
                || self.old_collection_interval != 0,
            old_collection_interval: self.old_collection_interval,
            independent_relocation: self.independent_relocation,
            work_stealing: self.work_stealing,
            metadata_shards: self.metadata_shards,
            region_shards: self.region_shards,
            mark_batch_size: self.mark_batch_size,
            parallel_threshold: self.parallel_threshold,
            max_heap_bytes: self.max_heap_bytes,
            promotion_age: self.promotion_age,
            full_collection_interval: self.full_collection_interval,
            compact_heap: self.compact_heap,
            cost_aware_relocation: self.cost_aware_relocation,
            relocation_reserve_bytes: self.relocation_reserve_bytes,
            region_size: self.region_size,
            relocation_chunk_bytes: self.relocation_chunk_bytes,
        }
    }
}

/// Statistics about garbage collection performance.
/// Worker counters are merged at step boundaries and before cycle completion.
/// Mark counts also include reference-free objects traced at allocation.
#[derive(Clone, Debug, Default)]
pub struct Statistics {
    pub collections_started: usize,
    pub collections_completed: usize,
    pub objects_marked: usize,
    pub objects_swept: usize,
    /// Published logical payload bytes allocated since the most recent cycle start.
    pub bytes_allocated: usize,
    /// Published logical payload bytes whose storage remains live.
    pub bytes_live: usize,
    /// Total managed/private reservation charge, including allocator wrapper storage.
    pub bytes_managed: usize,
    /// Cumulative logical payload bytes reclaimed by ordinary collection.
    pub bytes_freed: usize,
    /// Cumulative physical managed charge reclaimed by ordinary collection.
    pub managed_bytes_freed: usize,
    /// Legacy name: elapsed caller assistance time, including synchronous recovery.
    /// This is not a measurement of global application pauses.
    pub total_pause_time_us: u64,
    /// Time spent in background-worker collection steps.
    pub concurrent_time_us: u64,
    pub last_collection_start: Option<std::time::Instant>,
    pub last_collection_duration: Option<std::time::Duration>,
    pub total_collection_time: std::time::Duration,
    /// Ordinary physical allocation charge since the most recent cycle start.
    /// This is the local scheduling debt, distinct from logical `bytes_allocated`
    /// and from live managed/private reservations.
    pub allocation_debt: usize,
    /// Number of allocations still owned by the heap, including physically indexed
    /// constructors whose traversal obligations are being assigned.
    pub live_objects: usize,
    /// Completed young collections.
    pub young_collections: usize,
    /// Completed full collections.
    pub full_collections: usize,
    /// Completed conservative old-only passes, excluding precise full collections.
    pub old_collections: usize,
    /// Number of objects promoted to the old generation.
    pub objects_promoted: usize,
    /// Conditional-reference slots examined, including each ordered processing phase.
    pub references_examined: usize,
    /// Number of incremental steps performed.
    pub collection_steps: usize,
    /// Longest measured collection step. Unavailable on hosts without a clock.
    pub max_step_duration: Option<std::time::Duration>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_configuration() {
        let config = Configuration::default();
        assert_eq!(config.threads, 0);
        assert_eq!(config.allocation_threshold, 8 * 1024 * 1024);
        assert_eq!(config.max_pause_time_us, 100);
        assert_eq!(config.incremental_step_size, 1000);
        assert_eq!(config.parallel_threshold, 1_000_000);
    }

    #[test]
    fn default_statistics() {
        let stats = Statistics::default();
        assert_eq!(stats.collections_started, 0);
        assert_eq!(stats.collections_completed, 0);
        assert_eq!(stats.objects_marked, 0);
        assert_eq!(stats.objects_swept, 0);
        assert_eq!(stats.bytes_allocated, 0);
        assert_eq!(stats.bytes_freed, 0);
        assert_eq!(stats.total_pause_time_us, 0);
        assert_eq!(stats.concurrent_time_us, 0);
        assert!(stats.last_collection_start.is_none());
        assert!(stats.last_collection_duration.is_none());
        assert_eq!(stats.total_collection_time, std::time::Duration::new(0, 0));
    }

    #[test]
    fn configuration_builder_defaults() {
        let config = ConfigurationBuilder::new().build();
        assert_eq!(config.threads, 0);
        assert_eq!(config.allocation_threshold, 8 * 1024 * 1024);
        assert_eq!(config.max_pause_time_us, 100);
        assert_eq!(config.incremental_step_size, 1000);
        assert_eq!(config.parallel_threshold, 1_000_000);
    }

    #[test]
    fn configuration_builder_threads() {
        let config = ConfigurationBuilder::new().threads(4).build();
        assert_eq!(config.threads, 4);
    }

    #[test]
    fn configuration_builder_allocation_threshold() {
        let config = ConfigurationBuilder::new()
            .allocation_threshold(16 * 1024 * 1024)
            .build();
        assert_eq!(config.allocation_threshold, 16 * 1024 * 1024);
    }

    #[test]
    fn configuration_builder_max_pause_time_us() {
        let config = ConfigurationBuilder::new().max_pause_time_us(50).build();
        assert_eq!(config.max_pause_time_us, 50);
    }

    #[test]
    fn configuration_builder_incremental_step_size() {
        let config = ConfigurationBuilder::new()
            .incremental_step_size(2000)
            .build();
        assert_eq!(config.incremental_step_size, 2000);
    }

    #[test]
    fn configuration_builder_parallel_threshold() {
        let config = ConfigurationBuilder::new()
            .parallel_threshold(500_000)
            .build();
        assert_eq!(config.parallel_threshold, 500_000);
    }

    #[test]
    fn configuration_builder_fluent_chain() {
        let config = ConfigurationBuilder::new()
            .threads(2)
            .allocation_threshold(4 * 1024 * 1024)
            .max_pause_time_us(200)
            .incremental_step_size(500)
            .parallel_threshold(100_000)
            .build();

        assert_eq!(config.threads, 2);
        assert_eq!(config.allocation_threshold, 4 * 1024 * 1024);
        assert_eq!(config.max_pause_time_us, 200);
        assert_eq!(config.incremental_step_size, 500);
        assert_eq!(config.parallel_threshold, 100_000);
    }

    #[test]
    fn configuration_builder_from_configuration() {
        let config = Configuration::builder().threads(1).build();
        assert_eq!(config.threads, 1);
    }

    #[test]
    fn configuration_builder_default_impl() {
        let builder = ConfigurationBuilder::default();
        let config = builder.build();
        assert_eq!(config.threads, 0);
        assert_eq!(config.allocation_threshold, 8 * 1024 * 1024);
    }
}
