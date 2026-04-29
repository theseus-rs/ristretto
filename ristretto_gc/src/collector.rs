use crate::Finalize;
use crate::config::{Configuration, Statistics};
use crate::error::{Error, Result};
use crate::gc::Gc;
use crate::metadata::ObjectMetadata;
use crate::pointers::{SafePtr, TracePtr};
use crate::root_guard::GcRootGuard;
use dashmap::DashMap;
#[cfg(not(target_family = "wasm"))]
use rayon::prelude::*;
#[cfg(not(target_family = "wasm"))]
use rayon::{ThreadPool, ThreadPoolBuilder};
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex, RwLock, Weak};
#[cfg(not(target_family = "wasm"))]
use std::thread;
#[cfg(not(target_family = "wasm"))]
use sysinfo::System;
use tracing::{debug, error, info, trace, warn};

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
/// This collector implements a concurrent mark and sweep algorithm with the following phases:
/// 1. Initial Mark: Brief pause to mark root objects
/// 2. Concurrent Mark: Mark reachable objects concurrently with mutator
/// 3. Final Mark: Brief pause to handle objects modified during concurrent marking
/// 4. Concurrent Sweep: Reclaim unmarked objects concurrently
///
/// # Shutdown Behavior
///
/// When the collector is dropped, it stops the background thread and runs destructors
/// for all remaining tracked objects via a two-phase cleanup: finalizers run first
/// (while all objects are still alive), then drop closures deallocate memory. This ensures
/// `Drop` and `Finalize` implementations execute during shutdown. Callers that embed
/// the collector (e.g. a VM) must ensure the collector field is declared **last** so
/// all other resources drop before the collector runs its cleanup.
#[derive(Debug)]
pub struct GarbageCollector {
    this: Weak<Self>,
    configuration: Configuration,
    #[cfg(not(target_family = "wasm"))]
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
    #[cfg(not(target_family = "wasm"))]
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
        #[cfg(not(target_family = "wasm"))]
        let threads = if configuration.threads == 0 {
            let cpus = System::physical_core_count().unwrap_or(1);
            // Default to 50% of available CPU cores, but at least 1 thread
            (cpus >> 1).max(1)
        } else {
            configuration.threads
        };
        #[cfg(target_family = "wasm")]
        let threads = if configuration.threads == 0 {
            1
        } else {
            configuration.threads
        };
        info!("garbage collector configured with {threads} threads");
        #[cfg(not(target_family = "wasm"))]
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
            #[cfg(not(target_family = "wasm"))]
            thread_pool,
            statistics: Arc::new(RwLock::new(Statistics::default())),
            roots: Arc::new(DashMap::new()),
            next_root_id: AtomicUsize::new(0),
            objects: Arc::new(DashMap::new()),
            phase: Arc::new(RwLock::new(GcPhase::Idle)),
            collection_active: Arc::new(AtomicBool::new(false)),
            mark_queue: Arc::new(Mutex::new(VecDeque::new())),
            bytes_allocated: Arc::new(AtomicUsize::new(0)),
            #[cfg(not(target_family = "wasm"))]
            collector_thread: Arc::new(RwLock::new(None)),
            shutdown: Arc::new(AtomicBool::new(false)),
            collection_trigger: Arc::new((Mutex::new(false), Condvar::new())),
        })
    }

    /// Registers a new object with the garbage collector for reachability analysis.
    pub(crate) fn register_object<T: Send + Sync>(&self, ptr: *const T, size: usize) {
        // Store the raw pointer without casting to ensure type information is preserved
        let safe_ptr = SafePtr::from_ptr(ptr);

        // Create type-safe metadata that knows how to properly drop the T data
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
    #[cfg(not(target_family = "wasm"))]
    pub fn start(&self) {
        let Ok(mut collector_thread) = self.collector_thread.write() else {
            return;
        };
        if collector_thread.is_some() {
            return;
        }

        // Use a Weak reference to avoid a reference cycle: if the thread held
        // Arc<GarbageCollector>, Drop::drop() could never fire because the
        // thread's Arc would keep the refcount above zero.
        let weak_collector = self.this.clone();
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
                &weak_collector,
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

    /// Starts the background collector thread (no-op on wasm).
    #[cfg(target_family = "wasm")]
    pub fn start(&self) {}

    /// Stops the background collector thread.
    ///
    /// # Errors
    ///
    /// Returns an error if the thread fails to join or if the shutdown signal fails.
    #[cfg(not(target_family = "wasm"))]
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

        // Wait for thread to finish, unless we ARE the collector thread.
        // This can happen when the collector thread holds the last Arc<GarbageCollector>
        // (from upgrading a Weak during a collection cycle) and drops it, triggering
        // Drop::drop() on the collector thread itself. In that case, the thread will
        // exit naturally once this function returns and the remaining stack unwinds;
        // the detached JoinHandle is dropped without joining.
        if handle.thread().id() != thread::current().id() {
            handle
                .join()
                .map_err(|_| Error::SyncError("Failed to join collector thread".to_string()))?;
        }

        self.cleanup_remaining_objects();
        debug!("collector stopped");
        Ok(())
    }

    /// Stops the background collector thread (no-op on wasm).
    ///
    /// # Errors
    ///
    /// This implementation never returns an error; the result type matches the
    /// non-wasm signature for API consistency.
    #[cfg(target_family = "wasm")]
    pub fn stop(&self) -> Result<()> {
        self.cleanup_remaining_objects();
        Ok(())
    }

    /// Removes all remaining objects from tracking, runs their destructors, and frees memory.
    /// Must only be called after the collector thread has been stopped.
    ///
    /// Uses a two-phase approach to ensure safe destructor ordering:
    /// 1. **Remove + Finalizer phase**: removes all objects from the tracking map (releasing
    ///    `DashMap` shard locks), then runs all finalizers while every object's memory is still
    ///    valid so finalizers may safely read other GC-managed data.
    /// 2. **Drop phase**: runs drop closures to deallocate memory.
    ///
    /// Objects are removed from the map before any destructors run so that finalizers cannot
    /// deadlock on `DashMap` shard locks.
    ///
    /// Safe to call multiple times: both `run_finalizer()` and `run_drop_fn()` use
    /// `Option::take()` guards, so closures run at most once.
    fn cleanup_remaining_objects(&self) {
        // Fail-fast in debug builds to catch bugs during development.
        debug_assert!(
            !self.collection_active.load(Ordering::Acquire),
            "cleanup_remaining_objects called while a collection cycle is still active"
        );
        // Fail-safe in release builds: warn and skip rather than risk double-free / UB.
        if self.collection_active.load(Ordering::Acquire) {
            warn!(
                "cleanup_remaining_objects called while a collection cycle is still active; \
                 skipping to avoid double-free"
            );
            return;
        }
        let keys: Vec<SafePtr> = self.objects.iter().map(|entry| *entry.key()).collect();

        // Remove all objects from the tracking map first so no DashMap shard locks are
        // held while destructors execute. This prevents deadlocks if a finalizer
        // interacts with the collector (e.g. looking up other objects).
        let mut removed: Vec<ObjectMetadata> = Vec::with_capacity(keys.len());
        for key in keys {
            if let Some((_, metadata)) = self.objects.remove(&key) {
                removed.push(metadata);
            }
        }

        // Phase 1: Run all finalizers while every object's memory is still valid.
        // Finalizers may safely read other GC managed objects because no drop
        // closures have executed yet.
        for metadata in &removed {
            metadata.run_finalizer();
        }

        // Phase 2: Run drop closures to deallocate memory.
        let mut bytes_freed = 0;
        let mut objects_freed = 0;
        for metadata in removed {
            bytes_freed += metadata.size();
            objects_freed += 1;
            metadata.run_drop_fn();
        }
        if objects_freed > 0 {
            self.bytes_allocated
                .fetch_sub(bytes_freed, Ordering::Relaxed);
            debug!(
                "cleanup freed {} remaining objects ({} bytes)",
                objects_freed, bytes_freed
            );
        }
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

        // Ensure the root insertion is globally visible before register_object() adds
        // the object to the tracking map. Without this fence, the compiler or CPU could
        // reorder the `roots.insert` past the subsequent `objects.insert`, allowing the
        // GC thread to observe the object (eligible for sweep) before seeing the root
        // that keeps it alive.
        //
        // Synchronization chain: roots.insert -> this fence -> objects.insert (releases
        // DashMap shard lock) -> GC thread acquires DashMap shard lock on objects (Acquire)
        // -> GC thread sees both the object and all prior writes, including the root.
        //
        // We use SeqCst for defense-in-depth: this path is infrequent (only during
        // allocation) and the stronger ordering removes any doubt about cross-map
        // visibility on weakly-ordered architectures.
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
            // Ensure root removal is visible to other threads before any dependent
            // operations (symmetric with the SeqCst fence in add_root).
            std::sync::atomic::fence(Ordering::Release);
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
        // If we can't acquire the lock immediately, the object is skipped for *this* cycle.
        // This is a liveness property, not a safety property: the object won't be prematurely
        // collected (allocation-color-black keeps new objects alive), but its reachable
        // subgraph may remain unmarked until the next collection cycle, increasing floating
        // garbage temporarily.
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

    /// Returns this collector as a raw pointer suitable for passing to JIT-compiled code.
    #[must_use]
    pub fn as_context_ptr(&self) -> *const u8 {
        let ptr: *const Self = self;
        ptr.cast::<u8>()
    }

    /// Reconstructs a `&GarbageCollector` from a raw pointer.
    ///
    /// This is intended for JIT interop where the GC pointer is passed through compiled
    /// code as a raw pointer. The caller must ensure the pointer was obtained from a valid
    /// `&GarbageCollector` reference that is still alive.
    ///
    /// # Panics
    ///
    /// Panics if the pointer is null.
    #[must_use]
    #[expect(clippy::cast_ptr_alignment)]
    pub fn from_raw_ptr<'a>(ptr: *const u8) -> &'a Self {
        assert!(
            !ptr.is_null(),
            "GarbageCollector::from_raw_ptr: null pointer"
        );
        let gc_ptr = ptr.cast::<Self>();
        // Safety: The caller guarantees this pointer was obtained from a live &GarbageCollector.
        unsafe { &*gc_ptr }
    }

    /// Reconstructs a `&GarbageCollector` from a pointer to a `#[repr(C)]` context struct
    /// whose first field is `*const u8` pointing to the `GarbageCollector`.
    ///
    /// This is intended for JIT interop where a `RuntimeContext` struct is passed to compiled
    /// code and the GC pointer is the first field.
    ///
    /// # Panics
    ///
    /// Panics if either the context pointer or the contained GC pointer is null.
    #[must_use]
    #[expect(clippy::cast_ptr_alignment)]
    pub fn from_context_struct_ptr<'a>(context_ptr: *const u8) -> &'a Self {
        assert!(
            !context_ptr.is_null(),
            "GarbageCollector::from_context_struct_ptr: null context pointer"
        );
        // Safety: The caller guarantees context_ptr points to a #[repr(C)] struct
        // whose first field is *const u8 (the GC pointer).
        let gc_ptr_ptr = context_ptr.cast::<*const u8>();
        let gc_ptr = unsafe { *gc_ptr_ptr };
        Self::from_raw_ptr(gc_ptr)
    }

    /// Write barrier to be called when a `Gc` reference is modified. This ensures that the target
    /// object is marked if the collector is in the concurrent marking phase.
    pub fn write_barrier<T: Trace>(&self, obj: &Gc<T>) {
        if self.is_concurrent_marking().unwrap_or(false) {
            self.add_to_mark_queue(obj.inner());
        }
    }

    /// Main loop for the background collector thread.
    #[cfg(not(target_family = "wasm"))]
    #[expect(clippy::too_many_arguments)]
    fn collector_thread_main(
        weak_collector: &Weak<GarbageCollector>,
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
                // If the collector has been dropped, exit the thread.
                let Some(collector) = weak_collector.upgrade() else {
                    debug!("collector dropped, exiting collector thread");
                    break;
                };
                Self::perform_collection_cycle(
                    &collector,
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

        // Phase 1: Initial Mark; mark all root objects
        Self::initial_mark_phase(collector, phase, roots, mark_queue, objects);

        // Phase 2: Concurrent Mark; mark all reachable objects
        Self::concurrent_mark_phase(collector, phase, mark_queue, objects);

        // Phase 3: Final Mark; handle any objects modified during concurrent marking
        Self::final_mark_phase(collector, phase, roots, mark_queue, objects);

        // Phase 4: Concurrent Sweep; free unmarked objects
        Self::concurrent_sweep_phase(phase, objects, bytes_allocated, stats);

        // Update phase back to idle
        if let Ok(mut phase_guard) = phase.write() {
            *phase_guard = GcPhase::Idle;
        }

        collection_active.store(false, Ordering::Release);

        let duration = start_time.elapsed();
        if let Ok(mut stats_guard) = stats.write() {
            stats_guard.collections_completed += 1;
            stats_guard.last_collection_duration = Some(duration);
            stats_guard.total_collection_time += duration;
        }

        debug!("garbage collection cycle completed in {duration:?}");
    }

    /// Phase 1: Initial mark; mark all root objects
    fn initial_mark_phase(
        collector: &GarbageCollector,
        phase: &Arc<RwLock<GcPhase>>,
        roots: &Arc<DashMap<usize, TracePtr>>,
        mark_queue: &Arc<Mutex<VecDeque<TracePtr>>>,
        objects: &Arc<DashMap<SafePtr, ObjectMetadata>>,
    ) {
        #[cfg(target_family = "wasm")]
        let _ = &collector;
        if let Ok(mut phase_guard) = phase.write() {
            *phase_guard = GcPhase::InitialMark;
        }

        trace!("Initial mark phase started");

        if let Ok(mut queue) = mark_queue.lock() {
            queue.clear();
        }

        // Unmark all objects first
        let number_of_objects = objects.len();

        #[cfg(not(target_family = "wasm"))]
        {
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
        }
        #[cfg(target_family = "wasm")]
        if number_of_objects > 0 {
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

    /// Phase 2: Concurrent mark; mark all reachable objects
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
                // First time marking this object; trace its contents
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

    /// Phase 3: Final mark; handle objects modified during concurrent marking
    ///
    /// Re-scans roots to catch any roots registered after the initial mark phase
    /// (e.g. objects allocated during concurrent marking). Then processes remaining
    /// mark queue items.
    fn final_mark_phase(
        collector: &GarbageCollector,
        phase: &Arc<RwLock<GcPhase>>,
        roots: &Arc<DashMap<usize, TracePtr>>,
        mark_queue: &Arc<Mutex<VecDeque<TracePtr>>>,
        objects: &Arc<DashMap<SafePtr, ObjectMetadata>>,
    ) {
        // Set phase to final mark
        if let Ok(mut phase_guard) = phase.write() {
            *phase_guard = GcPhase::FinalMark;
        }

        trace!("Final mark phase started");

        let mut final_processed = 0;

        // Re-scan roots to catch any roots added after the initial mark phase queued
        // the original root set. If a new object was allocated and rooted during
        // concurrent marking, its root won't have been in the initial mark's snapshot.
        // Without this re-scan, such objects survive (allocation-color-black) but their
        // children might not be traced, potentially leaving a reachable subgraph unmarked.
        //
        // Snapshot root TracePtr values first so we don't hold DashMap shard locks
        // during `trace()`.  Holding a shard read lock while `trace()` acquires a
        // `RwLock<Reference>` read lock can deadlock with a mutator thread that holds
        // the same `RwLock` write lock and tries to insert into the same DashMap shard.
        let root_snapshot: Vec<TracePtr> =
            roots.iter().map(|entry| entry.value().clone()).collect();
        for gc_trace_ptr in &root_snapshot {
            let ptr = SafePtr::from_ptr(gc_trace_ptr.as_raw_ptr());
            let should_trace = objects.get(&ptr).is_some_and(|metadata| metadata.mark());
            if should_trace {
                final_processed += 1;
                unsafe {
                    gc_trace_ptr.trace(collector);
                }
            }
        }

        // Process any remaining objects in the mark queue that were added during
        // concurrent marking or from the root re-scan above
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
            // Release the objects shard lock before calling trace() to avoid
            // the same lock-ordering deadlock as in the root re-scan above.
            let should_trace = objects.get(&ptr).is_some_and(|metadata| metadata.mark());
            if should_trace {
                final_processed += 1;
                unsafe {
                    gc_trace_ptr.trace(collector);
                }
            }
        }

        trace!(
            "Final mark phase completed, processed {} additional objects",
            final_processed
        );
    }

    /// Phase 4: Concurrent sweep; reclaim unmarked objects.
    ///
    /// Objects that were not marked during the mark phases are unreachable and safe to free.
    /// Objects start with `marked = true` (allocation-color-black), so any object allocated
    /// during this cycle will survive. Only objects that existed before the cycle began and
    /// were not reached from any root are swept.
    ///
    /// Sweep proceeds in two steps per object:
    /// 1. Remove from the tracking map (releases `DashMap` shard locks)
    /// 2. Run finalizer then drop closure
    fn concurrent_sweep_phase(
        phase: &Arc<RwLock<GcPhase>>,
        objects: &Arc<DashMap<SafePtr, ObjectMetadata>>,
        bytes_allocated: &Arc<AtomicUsize>,
        stats: &Arc<RwLock<Statistics>>,
    ) {
        if let Ok(mut phase_guard) = phase.write() {
            *phase_guard = GcPhase::ConcurrentSweep;
        }

        trace!("Concurrent sweep phase started");

        // Collect keys of objects that are unmarked AND were previously reachable.
        // Objects that were never marked as reachable during any GC mark phase are
        // owned by non-GC containers (e.g., ClassLoader, StringPool) and must not
        // be swept; their lifetime is managed by Rust ownership, not GC tracing.
        let unmarked_keys: Vec<SafePtr> = objects
            .iter()
            .filter(|entry| {
                let metadata = entry.value();
                !metadata.is_marked() && metadata.was_ever_marked()
            })
            .map(|entry| *entry.key())
            .collect();

        let mut swept_objects = 0;
        let mut swept_bytes = 0;

        // Remove unmarked objects from the map and collect them for destruction
        let mut to_destroy: Vec<ObjectMetadata> = Vec::with_capacity(unmarked_keys.len());
        for key in &unmarked_keys {
            if let Some((_, metadata)) = objects.remove(key) {
                // Double-check the object is still unmarked after removal.
                // A concurrent mutator could have rooted/marked it between our
                // snapshot and removal, but this is impossible because:
                // 1. We're in the sweep phase (after final mark)
                // 2. New allocations start marked=true and won't appear in unmarked_keys
                // 3. Existing objects can't become marked after final mark
                swept_bytes += metadata.size();
                swept_objects += 1;
                to_destroy.push(metadata);
            }
        }

        // Run finalizers first (while all other objects' memory is still valid)
        for metadata in &to_destroy {
            metadata.run_finalizer();
        }

        // Then run drop closures to deallocate memory
        for metadata in to_destroy {
            metadata.run_drop_fn();
        }

        if swept_objects > 0 {
            bytes_allocated.fetch_sub(swept_bytes, Ordering::Relaxed);
            debug!("swept {swept_objects} objects ({swept_bytes} bytes)");
        }

        if let Ok(mut stats_guard) = stats.write() {
            stats_guard.objects_swept += swept_objects;
            stats_guard.bytes_freed += swept_bytes;
        }

        // Reset the allocation counter so the threshold isn't hit on every subsequent
        // allocation. The counter only drives heuristic scheduling of the next cycle.
        bytes_allocated.store(0, Ordering::Relaxed);

        trace!("Concurrent sweep phase completed, swept {swept_objects} objects");
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

impl<T> Trace for crate::sync::Mutex<T>
where
    T: Trace,
{
    fn trace(&self, collector: &GarbageCollector) {
        self.lock().trace(collector);
    }
}

impl<T> Trace for crate::sync::RwLock<T>
where
    T: Trace,
{
    fn trace(&self, collector: &GarbageCollector) {
        self.read().trace(collector);
    }
}
