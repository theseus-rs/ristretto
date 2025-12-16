use crate::Finalize;
use crate::config::{Configuration, Statistics};
use crate::error::{Error, Result};
use crate::gc::Gc;
use crate::metadata::ObjectMetadata;
use crate::pointers::{SafePtr, TracePtr};
use crate::root_guard::GcRootGuard;
use dashmap::DashMap;
use rayon::prelude::*;
use rayon::{ThreadPool, ThreadPoolBuilder};
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, LazyLock, Mutex, RwLock, Weak};
use std::thread;
use sysinfo::System;
use tracing::{debug, error, info, trace, warn};

/// Global garbage collector instance
pub static GC: LazyLock<Arc<GarbageCollector>> = LazyLock::new(|| {
    let collector = GarbageCollector::new();
    collector.start();
    collector
});

/// Trait for objects that can be traced by the garbage collector.
///
/// Objects that contain `Gc<T>` pointers must implement this trait to enable the garbage
/// collector to traverse object references during the marking phase.
pub trait Trace {
    /// Trace all `Gc<T>` references contained within this object.
    ///
    /// This method should call `trace()` on all `Gc<T>` fields contained within the object.
    /// The implementation should be careful not to trace non-`Gc` references or perform
    /// any allocation during tracing.
    fn trace(&self, collector: &GarbageCollector);
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum GcPhase {
    Idle,
    InitialMark,
    ConcurrentMark,
    FinalMark,
    ConcurrentSweep,
}

/// A low pause, parallel, concurrent garbage collector using reachability analysis.
///
/// This collector implements a concurrent mark-and-sweep algorithm with the following phases:
/// 1. Initial Mark: Brief pause to mark root objects
/// 2. Concurrent Mark: Mark reachable objects concurrently with mutator
/// 3. Final Mark: Brief pause to handle objects modified during concurrent marking
/// 4. Concurrent Sweep: Reclaim unmarked objects concurrently
#[derive(Debug)]
pub struct GarbageCollector {
    this: Weak<Self>,
    configuration: Configuration,
    thread_pool: ThreadPool,
    statistics: Arc<RwLock<Statistics>>,
    roots: Arc<DashMap<usize, TracePtr>>,
    next_root_id: AtomicUsize,

    // Object registry for reachability analysis
    objects: Arc<DashMap<SafePtr, ObjectMetadata>>,

    // Concurrent collection state
    phase: Arc<RwLock<GcPhase>>,
    collection_active: Arc<AtomicBool>,
    mark_queue: Arc<Mutex<VecDeque<TracePtr>>>,

    // Allocation tracking
    bytes_allocated: Arc<AtomicUsize>,

    // Background thread coordination
    collector_thread: Arc<RwLock<Option<thread::JoinHandle<()>>>>,
    shutdown: Arc<AtomicBool>,
    collection_trigger: Arc<(Mutex<bool>, Condvar)>,
}

impl GarbageCollector {
    /// Creates a new low pause, parallel, concurrent garbage collector.
    #[must_use]
    pub fn new() -> Arc<Self> {
        Self::with_config(Configuration::default())
    }

    /// Creates a new garbage collector with custom configuration.
    ///
    /// # Panics
    ///
    /// Panics if the thread pool cannot be created.
    #[must_use]
    pub fn with_config(configuration: Configuration) -> Arc<Self> {
        let threads = if configuration.threads == 0 {
            let cpus = System::physical_core_count().unwrap_or(1);
            // Default to 50% of available CPU cores, but at least 1 thread
            (cpus >> 1).max(1)
        } else {
            configuration.threads
        };
        info!("garbage collector configured with {threads} threads");
        let thread_pool = match ThreadPoolBuilder::new()
            .num_threads(threads)
            .thread_name(|thread_index| format!("gc-{thread_index}"))
            .build()
        {
            Ok(thread_pool) => thread_pool,
            Err(error) => {
                panic!("Failed to create thread pool for GC: {error}");
            }
        };

        Arc::new_cyclic(|this| Self {
            this: this.clone(),
            configuration,
            thread_pool,
            statistics: Arc::new(RwLock::new(Statistics::default())),
            roots: Arc::new(DashMap::new()),
            next_root_id: AtomicUsize::new(0),
            objects: Arc::new(DashMap::new()),
            phase: Arc::new(RwLock::new(GcPhase::Idle)),
            collection_active: Arc::new(AtomicBool::new(false)),
            mark_queue: Arc::new(Mutex::new(VecDeque::new())),
            bytes_allocated: Arc::new(AtomicUsize::new(0)),
            collector_thread: Arc::new(RwLock::new(None)),
            shutdown: Arc::new(AtomicBool::new(false)),
            collection_trigger: Arc::new((Mutex::new(false), Condvar::new())),
        })
    }

    /// Registers a new object with the garbage collector for reachability analysis.
    pub(crate) fn register_object<T: Send + Sync>(&self, ptr: *const T, size: usize) {
        // Store the raw pointer without casting to ensure type information is preserved
        let safe_ptr = SafePtr::from_ptr(ptr);

        // Create type-safe metadata that knows how to properly drop the Gc<T>
        // Here T is the inner type, so we create a dropper for Gc<T>
        let metadata = ObjectMetadata::new_for_gc::<T>(safe_ptr, size);

        trace!("registering object at {:#x} with size {size}", safe_ptr.0);
        self.objects.insert(safe_ptr, metadata);
    }

    /// Registers a new object with finalizer support for reachability analysis. This is used when
    /// `T` implements the `Finalize` trait.
    pub(crate) fn register_object_with_finalizer<T>(&self, ptr: *const T, size: usize)
    where
        T: Send + Sync + Finalize,
    {
        let safe_ptr = SafePtr::from_ptr(ptr);

        // Create metadata with finalizer support
        let metadata = ObjectMetadata::new_for_gc_with_finalizer::<T>(safe_ptr, size);

        trace!(
            "registering object with finalizer at {:#x} with size {size}",
            safe_ptr.0
        );
        self.objects.insert(safe_ptr, metadata);
    }

    /// Starts the background collector thread.
    pub fn start(&self) {
        let Ok(mut collector_thread) = self.collector_thread.write() else {
            return;
        };
        if collector_thread.is_some() {
            return;
        }
        let Some(collector) = self.this.upgrade() else {
            error!("Failed to upgrade Weak reference to GarbageCollector");
            return;
        };

        let collector = Arc::clone(&collector);
        let stats = Arc::clone(&self.statistics);
        let roots = Arc::clone(&self.roots);
        let objects = Arc::clone(&self.objects);
        let phase = Arc::clone(&self.phase);
        let collection_active = Arc::clone(&self.collection_active);
        let mark_queue = Arc::clone(&self.mark_queue);
        let shutdown = Arc::clone(&self.shutdown);
        let collection_trigger = Arc::clone(&self.collection_trigger);
        let bytes_allocated = Arc::clone(&self.bytes_allocated);

        let handle = thread::spawn(move || {
            Self::collector_thread_main(
                &collector,
                &stats,
                &roots,
                &phase,
                &collection_active,
                &mark_queue,
                &shutdown,
                &collection_trigger,
                &objects,
                &bytes_allocated,
            );
        });

        *collector_thread = Some(handle);
        debug!("collector started");
    }

    /// Stops the background collector thread.
    ///
    /// # Errors
    ///
    /// Returns an error if the thread fails to join or if the shutdown signal fails.
    pub fn stop(&self) -> Result<()> {
        let Ok(mut collector_thread) = self.collector_thread.write() else {
            return Ok(());
        };
        let Some(handle) = collector_thread.take() else {
            return Ok(());
        };
        self.shutdown.store(true, Ordering::Release);

        // Wake up the collector thread
        let (lock, cvar) = &*self.collection_trigger;
        let mut triggered = lock.lock().map_err(|error| {
            Error::LockError(format!(
                "Failed to acquire collection trigger lock: {error}"
            ))
        })?;
        *triggered = true;
        cvar.notify_one();
        drop(triggered);

        // Wait for thread to finish
        handle
            .join()
            .map_err(|_| Error::SyncError("Failed to join collector thread".to_string()))?;
        debug!("collector stopped");
        Ok(())
    }

    /// Triggers a garbage collection cycle. This is non-blocking and will wake up the background
    /// collector.
    pub fn collect(&self) {
        let (lock, cvar) = &*self.collection_trigger;
        match lock.lock() {
            Ok(mut triggered) => {
                *triggered = true;
                cvar.notify_one();
            }
            Err(error) => {
                error!("Failed to acquire collection trigger lock: {error}");
            }
        }
    }

    /// Records an allocation for GC threshold tracking.
    pub fn record_allocation(&self, size: usize) {
        let old_size = self.bytes_allocated.fetch_add(size, Ordering::Relaxed);

        // Trigger collection if threshold exceeded (use saturating arithmetic to prevent overflow)
        if old_size.saturating_add(size) >= self.configuration.allocation_threshold {
            self.collect();
        }
    }

    /// Gets current garbage collection statistics.
    ///
    /// # Errors
    ///
    /// Returns an error if the stats lock acquisition fails.
    pub fn statistics(&self) -> Result<Statistics> {
        let mut statistics = self
            .statistics
            .read()
            .map_err(|error| Error::StatsError(format!("Failed to read stats: {error}")))?
            .clone();

        // Update bytes_allocated with current value
        statistics.bytes_allocated = self.bytes_allocated.load(Ordering::Relaxed);

        Ok(statistics)
    }

    /// Adds a `Gc<T>` root object for garbage collection and returns its ID.
    pub fn add_root<T: Trace>(&self, root: &Gc<T>) -> usize {
        let root_id = self.next_root_id.fetch_add(1, Ordering::Relaxed);
        // Instead of storing a TracePtr to the Gc<T> struct, store the Gc<T> pointer directly
        let gc_ptr = root.ptr.as_ptr();
        let gc_trace_ptr = TracePtr::new_from_ptr(gc_ptr);

        self.roots.insert(root_id, gc_trace_ptr);

        // Ensure the insertion is visible to other threads before proceeding
        std::sync::atomic::fence(Ordering::SeqCst);

        trace!("adding root {:#x} with id {root_id}", gc_ptr as usize);
        root_id
    }

    /// Creates a new root guard that automatically manages the lifetime of a `Gc<T>` root.
    /// The returned guard will automatically remove the root when dropped.
    ///
    /// # Errors
    ///
    /// If the garbage collector cannot be upgraded from a Weak reference.
    pub fn create_root_guard<T: Trace>(&self, root: Gc<T>) -> Result<GcRootGuard<T>> {
        let Some(collector) = self.this.upgrade() else {
            return Err(Error::SyncError(
                "Failed to upgrade Weak reference to GarbageCollector".to_string(),
            ));
        };
        Ok(GcRootGuard::new(collector, root))
    }

    /// Internal method to remove a root by its ID.
    /// Used by `GcRootGuard` to clean up when dropped.
    pub fn remove_root_by_id(&self, root_id: usize) {
        if let Some((_id, gc_trace_ptr)) = self.roots.remove(&root_id) {
            let ptr = gc_trace_ptr.as_raw_ptr() as usize;
            trace!("removed root {ptr:#x} with id {root_id}");
        }
    }

    /// Internal method to remove a root by its index.
    /// Used by `GcRootGuard` to clean up when dropped.
    pub fn remove_root<T: Trace>(&self, root: &T) {
        let root_ptr = std::ptr::from_ref::<T>(root).cast::<()>();
        // Remove a root object from the roots hashmap
        self.roots.retain(|_, gc_trace_ptr| {
            // Use pointer equality to check if the root matches
            if std::ptr::eq(gc_trace_ptr.as_raw_ptr(), root_ptr) {
                trace!("removed root {:#x}", root_ptr as usize);
                false // Remove this entry
            } else {
                true // Keep this entry
            }
        });
    }

    /// Checks if the garbage collector is currently in the concurrent marking phase. This is used
    /// by the write barrier to determine if marking is needed.
    pub(crate) fn is_concurrent_marking(&self) -> Result<bool> {
        let phase = self.phase.read().map_err(|error| {
            Error::CollectionPhaseError(format!("Failed to read phase: {error}"))
        })?;
        Ok(matches!(*phase, GcPhase::ConcurrentMark))
    }

    /// Adds an object to the mark queue for concurrent marking. This is used by the write barrier
    /// when a reference is stored during concurrent marking.
    pub(crate) fn add_to_mark_queue<T: Trace>(&self, obj: &T) {
        if let Ok(mut queue) = self.mark_queue.try_lock() {
            queue.push_back(TracePtr::new(obj));
        }
        // If we can't get the lock immediately, it's okay to skip the object will be traced in the
        // next collection cycle
    }

    /// Checks if an object is already marked to avoid infinite loops during tracing. Used by
    /// the `Trace` implementations for cycle detection.
    pub(crate) fn is_object_marked(&self, ptr: SafePtr) -> bool {
        if let Some(metadata) = self.objects.get(&ptr) {
            return metadata.is_marked();
        }
        false
    }

    /// Adds an object to the mark queue for processing during collection. Used by the `Trace`
    /// implementations to queue objects for reachability analysis.
    pub(crate) fn add_gc_to_mark_queue<T: Trace>(&self, ptr: *const Gc<T>) {
        if let Ok(mut queue) = self.mark_queue.try_lock() {
            queue.push_back(TracePtr::new_from_ptr(ptr));
        }
    }

    /// Marks an object as reachable in the object registry. Used during garbage collection to mark
    /// objects that are reachable from roots.
    pub(crate) fn mark_object(&self, ptr: SafePtr) {
        if let Some(metadata) = self.objects.get(&ptr) {
            trace!("object: {:#x} marked reachable", ptr.0);
            metadata.mark();
        }
    }

    /// Attempts to mark an object as reachable. Returns `true` if this is the first time marking
    /// the object, `false` if already marked. This is used for cycle detection during tracing to
    /// prevent infinite recursion.
    pub(crate) fn try_mark_object(&self, ptr: SafePtr) -> bool {
        if let Some(metadata) = self.objects.get(&ptr) {
            let was_unmarked = metadata.mark();
            if was_unmarked {
                trace!("object: {:#x} marked reachable (first time)", ptr.0);
            } else {
                trace!("object: {:#x} already marked, skipping trace", ptr.0);
            }
            return was_unmarked;
        }
        false
    }

    /// Write barrier to be called when a `Gc` reference is modified. This ensures that the target
    /// object is marked if the collector is in the concurrent marking phase.
    pub fn write_barrier<T: Trace>(&self, obj: &Gc<T>) {
        if self.is_concurrent_marking().unwrap_or(false) {
            self.add_to_mark_queue(obj.inner());
        }
    }

    /// Main loop for the background collector thread.
    #[expect(clippy::too_many_arguments)]
    fn collector_thread_main(
        collector: &GarbageCollector,
        stats: &Arc<RwLock<Statistics>>,
        roots: &Arc<DashMap<usize, TracePtr>>,
        phase: &Arc<RwLock<GcPhase>>,
        collection_active: &Arc<AtomicBool>,
        mark_queue: &Arc<Mutex<VecDeque<TracePtr>>>,
        shutdown: &Arc<AtomicBool>,
        collection_trigger: &Arc<(Mutex<bool>, Condvar)>,
        objects: &Arc<DashMap<SafePtr, ObjectMetadata>>,
        bytes_allocated: &Arc<AtomicUsize>,
    ) {
        debug!(
            "collector thread {:?}:{:?} started",
            thread::current().id(),
            thread::current().name()
        );

        while !shutdown.load(Ordering::Acquire) {
            // Wait for collection trigger or shutdown
            let wait_result = {
                let (lock, cvar) = &**collection_trigger;
                let Ok(mut triggered) = lock.lock() else {
                    error!("Failed to acquire collection trigger lock");
                    continue;
                };

                // Wait until triggered or shutdown
                while !*triggered && !shutdown.load(Ordering::Acquire) {
                    triggered = if let Ok(guard) = cvar.wait(triggered) {
                        guard
                    } else {
                        error!("Collection trigger condition variable wait failed");
                        return; // Exit the thread on condition variable error
                    };
                }

                // Check if we should collect before resetting the trigger
                let should_collect = *triggered && !shutdown.load(Ordering::Acquire);
                if should_collect {
                    *triggered = false;
                }
                should_collect
            };

            if shutdown.load(Ordering::Acquire) {
                break;
            }

            if wait_result {
                // Perform garbage collection cycle
                Self::perform_collection_cycle(
                    collector,
                    stats,
                    roots,
                    phase,
                    collection_active,
                    mark_queue,
                    objects,
                    bytes_allocated,
                );
            }
        }

        debug!(
            "collector thread {:?} shutting down",
            thread::current().id()
        );
    }

    /// Performs a complete garbage collection cycle
    #[expect(clippy::too_many_arguments)]
    fn perform_collection_cycle(
        collector: &GarbageCollector,
        stats: &Arc<RwLock<Statistics>>,
        roots: &Arc<DashMap<usize, TracePtr>>,
        phase: &Arc<RwLock<GcPhase>>,
        collection_active: &Arc<AtomicBool>,
        mark_queue: &Arc<Mutex<VecDeque<TracePtr>>>,
        objects: &Arc<DashMap<SafePtr, ObjectMetadata>>,
        bytes_allocated: &Arc<AtomicUsize>,
    ) {
        let start_time = std::time::Instant::now();
        collection_active.store(true, Ordering::Release);

        if let Ok(mut stats_guard) = stats.write() {
            stats_guard.collections_started += 1;
            stats_guard.last_collection_start = Some(start_time);
        }

        debug!("starting garbage collection cycle");

        // Phase 1: Initial Mark - mark all root objects
        Self::initial_mark_phase(collector, phase, roots, mark_queue, objects);

        // Phase 2: Concurrent Mark - mark all reachable objects
        Self::concurrent_mark_phase(collector, phase, mark_queue, objects);

        // Phase 3: Final Mark - handle any objects modified during concurrent marking
        Self::final_mark_phase(phase, mark_queue, objects);

        // Phase 4: Concurrent Sweep - reclaim unmarked objects
        let (bytes_freed, objects_swept) =
            Self::concurrent_sweep_phase(collector, phase, objects, bytes_allocated);

        // Update phase back to idle
        if let Ok(mut phase_guard) = phase.write() {
            *phase_guard = GcPhase::Idle;
        }

        collection_active.store(false, Ordering::Release);

        let duration = start_time.elapsed();
        if let Ok(mut stats_guard) = stats.write() {
            stats_guard.collections_completed += 1;
            stats_guard.bytes_freed += bytes_freed;
            stats_guard.objects_swept += objects_swept;
            stats_guard.last_collection_duration = Some(duration);
            stats_guard.total_collection_time += duration;
        }

        debug!(
            "garbage collection cycle completed in {:?}, freed {} objects ({} bytes)",
            duration, objects_swept, bytes_freed
        );
    }

    /// Phase 1: Initial mark - mark all root objects
    fn initial_mark_phase(
        collector: &GarbageCollector,
        phase: &Arc<RwLock<GcPhase>>,
        roots: &Arc<DashMap<usize, TracePtr>>,
        mark_queue: &Arc<Mutex<VecDeque<TracePtr>>>,
        objects: &Arc<DashMap<SafePtr, ObjectMetadata>>,
    ) {
        if let Ok(mut phase_guard) = phase.write() {
            *phase_guard = GcPhase::InitialMark;
        }

        trace!("Initial mark phase started");

        if let Ok(mut queue) = mark_queue.lock() {
            queue.clear();
        }

        // Unmark all objects first
        let number_of_objects = objects.len();

        let configuration = &collector.configuration;
        if number_of_objects > configuration.parallel_threshold {
            debug!("unmarking {number_of_objects} objects (parallel)");
            collector.thread_pool.install(|| {
                objects.par_iter().for_each(|entry| {
                    entry.value().unmark();
                });
            });
        } else if number_of_objects > 0 {
            debug!("unmarking {number_of_objects} objects (sequential)");
            for entry in objects.iter() {
                entry.value().unmark();
            }
        }

        // Add all root objects to the mark queue
        for root_entry in roots.iter() {
            let gc_trace_ptr = root_entry.value();
            if let Ok(mut queue) = mark_queue.lock() {
                queue.push_back(gc_trace_ptr.clone());
            }
        }

        trace!("Initial mark phase completed, {} roots queued", roots.len());
    }

    /// Phase 2: Concurrent mark - mark all reachable objects
    fn concurrent_mark_phase(
        collector: &GarbageCollector,
        phase: &Arc<RwLock<GcPhase>>,
        mark_queue: &Arc<Mutex<VecDeque<TracePtr>>>,
        objects: &Arc<DashMap<SafePtr, ObjectMetadata>>,
    ) {
        if let Ok(mut phase_guard) = phase.write() {
            *phase_guard = GcPhase::ConcurrentMark;
        }

        trace!("Concurrent mark phase started");

        let mut processed_count = 0;

        loop {
            let next_object = {
                if let Ok(mut queue) = mark_queue.lock() {
                    queue.pop_front()
                } else {
                    break;
                }
            };

            let Some(gc_trace_ptr) = next_object else {
                break;
            };

            // Mark this object and trace its references
            let ptr = SafePtr::from_ptr(gc_trace_ptr.as_raw_ptr());

            // Check if object exists and mark it
            let should_trace = {
                if let Some(metadata) = objects.get(&ptr) {
                    metadata.mark() // Returns true if this is first time marking
                } else {
                    false
                }
            };

            if should_trace {
                // First time marking this object - trace its contents
                processed_count += 1;

                // Safely trace the object by calling its trace method
                // This will add any referenced objects to the mark queue
                // Safety: This is safe because:
                // 1. The TracePtr was created from a valid object reference
                // 2. We're in the concurrent marking phase where objects are stable
                // 3. The trace method is designed to be called during GC
                // 4. TracePtr includes null pointer checks internally
                unsafe {
                    gc_trace_ptr.trace(collector);
                }
            }
        }

        trace!(
            "Concurrent mark phase completed, processed {} objects",
            processed_count
        );
    }

    /// Phase 3: Final mark - handle objects modified during concurrent marking
    fn final_mark_phase(
        phase: &Arc<RwLock<GcPhase>>,
        mark_queue: &Arc<Mutex<VecDeque<TracePtr>>>,
        objects: &Arc<DashMap<SafePtr, ObjectMetadata>>,
    ) {
        // Set phase to final mark
        if let Ok(mut phase_guard) = phase.write() {
            *phase_guard = GcPhase::FinalMark;
        }

        trace!("Final mark phase started");

        // Process any remaining objects in the mark queue that were added during concurrent marking
        let mut final_processed = 0;
        loop {
            let next_object = {
                if let Ok(mut queue) = mark_queue.lock() {
                    queue.pop_front()
                } else {
                    break;
                }
            };

            let Some(gc_trace_ptr) = next_object else {
                break;
            };

            let ptr = SafePtr::from_ptr(gc_trace_ptr.as_raw_ptr());
            if let Some(metadata) = objects.get(&ptr)
                && metadata.mark()
            {
                final_processed += 1;
            }
        }

        trace!(
            "Final mark phase completed, processed {} additional objects",
            final_processed
        );
    }

    /// Phase 4: Concurrent sweep - reclaim unmarked objects
    fn concurrent_sweep_phase(
        collector: &GarbageCollector,
        phase: &Arc<RwLock<GcPhase>>,
        objects: &Arc<DashMap<SafePtr, ObjectMetadata>>,
        bytes_allocated: &Arc<AtomicUsize>,
    ) -> (usize, usize) {
        // Set phase to concurrent sweep
        if let Ok(mut phase_guard) = phase.write() {
            *phase_guard = GcPhase::ConcurrentSweep;
        }

        trace!("Concurrent sweep phase started");

        let configuration = &collector.configuration;
        let mut bytes_freed = 0;
        let mut objects_freed = 0;

        // Collect unmarked objects for removal
        let to_remove: Vec<(SafePtr, ObjectMetadata)> = {
            let number_of_objects = objects.len();
            if number_of_objects > configuration.parallel_threshold {
                debug!("sweeping {number_of_objects} objects (parallel)");
                collector.thread_pool.install(|| {
                    objects
                        .par_iter()
                        .filter_map(|entry| {
                            let metadata = entry.value();
                            if metadata.is_marked() {
                                None // Keep marked objects
                            } else {
                                Some((
                                    *entry.key(),
                                    ObjectMetadata::new_for_gc::<u8>(*entry.key(), metadata.size()),
                                ))
                            }
                        })
                        .collect()
                })
            } else {
                debug!("sweeping {number_of_objects} objects (sequential)");
                objects
                    .iter()
                    .filter_map(|entry| {
                        let metadata = entry.value();
                        if metadata.is_marked() {
                            None // Keep marked objects
                        } else {
                            Some((
                                *entry.key(),
                                ObjectMetadata::new_for_gc::<u8>(*entry.key(), metadata.size()),
                            ))
                        }
                    })
                    .collect()
            }
        };

        // Remove unmarked objects
        for (ptr, _metadata) in to_remove {
            if let Some((_, removed_metadata)) = objects.remove(&ptr) {
                bytes_freed += removed_metadata.size();
                objects_freed += 1;

                // Call the drop function to properly deallocate the object
                removed_metadata.drop_object();
            }
        }

        // Update bytes allocated to reflect freed memory
        bytes_allocated.fetch_sub(bytes_freed, Ordering::Relaxed);

        trace!(
            "Concurrent sweep phase completed, freed {} objects ({} bytes)",
            objects_freed, bytes_freed
        );

        (bytes_freed, objects_freed)
    }
}

impl Drop for GarbageCollector {
    fn drop(&mut self) {
        if let Err(error) = self.stop() {
            warn!("Failed to stop garbage collector cleanly: {error}");
        }
    }
}

impl<T: Trace> Trace for Option<T> {
    fn trace(&self, collector: &GarbageCollector) {
        if let Some(value) = self {
            value.trace(collector);
        }
    }
}

impl<T: Trace> Trace for Vec<T> {
    fn trace(&self, collector: &GarbageCollector) {
        for item in self {
            item.trace(collector);
        }
    }
}

impl<K: Trace, V: Trace, S: ::std::hash::BuildHasher> Trace for HashMap<K, V, S> {
    fn trace(&self, collector: &GarbageCollector) {
        for (key, value) in self {
            key.trace(collector);
            value.trace(collector);
        }
    }
}

impl<T: Trace, S: ::std::hash::BuildHasher> Trace for HashSet<T, S> {
    fn trace(&self, collector: &GarbageCollector) {
        for item in self {
            item.trace(collector);
        }
    }
}

impl<A: Trace, B: Trace> Trace for (A, B) {
    fn trace(&self, collector: &GarbageCollector) {
        self.0.trace(collector);
        self.1.trace(collector);
    }
}

impl<A: Trace, B: Trace, C: Trace> Trace for (A, B, C) {
    fn trace(&self, collector: &GarbageCollector) {
        self.0.trace(collector);
        self.1.trace(collector);
        self.2.trace(collector);
    }
}

impl Trace for String {
    fn trace(&self, _collector: &GarbageCollector) {}
}

impl Trace for &str {
    fn trace(&self, _collector: &GarbageCollector) {}
}

macro_rules! impl_trace_primitive {
    ($($t:ty),*) => {
        $(
            impl Trace for $t {
                fn trace(&self, _collector: &GarbageCollector) {}
            }
        )*
    };
}

impl_trace_primitive!(
    bool, char, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64
);

impl<T> Trace for Mutex<T>
where
    T: Trace,
{
    fn trace(&self, collector: &GarbageCollector) {
        match self.lock() {
            Ok(guard) => guard.trace(collector),
            Err(poisoned) => poisoned.into_inner().trace(collector),
        }
    }
}

impl<T> Trace for RwLock<T>
where
    T: Trace,
{
    fn trace(&self, collector: &GarbageCollector) {
        match self.read() {
            Ok(guard) => guard.trace(collector),
            Err(poisoned) => poisoned.into_inner().trace(collector),
        }
    }
}

impl<T> Trace for parking_lot::Mutex<T>
where
    T: Trace,
{
    fn trace(&self, collector: &GarbageCollector) {
        self.lock().trace(collector);
    }
}

impl<T> Trace for parking_lot::RwLock<T>
where
    T: Trace,
{
    fn trace(&self, collector: &GarbageCollector) {
        self.read().trace(collector);
    }
}
