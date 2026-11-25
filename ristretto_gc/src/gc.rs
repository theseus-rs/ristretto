use crate::Finalize;
use crate::collector::{GarbageCollector, Trace};
use crate::error::{Error, Result};
use crate::pointers::SafePtr;
use crate::root_guard::GcRootGuard;
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::mem::size_of;
use std::ops::Deref;
use std::ptr::NonNull;
use std::sync::{Arc, Weak};
use triomphe::Arc as RootArc;

// Nonempty values use their own layout. An empty value still needs a distinct
// address because ordinary root and object registries use that address as a key.
#[repr(transparent)]
pub(crate) struct Allocation<T> {
    pub(crate) value: T,
}

impl<T> Allocation<T> {
    pub(crate) fn new(value: T) -> Self {
        Self { value }
    }

    pub(crate) fn layout() -> std::alloc::Layout {
        if size_of::<T>() == 0 {
            std::alloc::Layout::new::<(T, u8)>()
        } else {
            std::alloc::Layout::new::<T>()
        }
    }
}

#[derive(Debug)]
pub(crate) struct OwnedRoot {
    pub(crate) collector: Arc<GarbageCollector>,
    pub(crate) id: crate::roots::Registration,
    pub(crate) target: crate::ObjectId,
}

impl Drop for OwnedRoot {
    fn drop(&mut self) {
        if !crate::construction::defer_owned(&self.collector, &self.id) {
            self.collector.remove_detached_root(&self.id);
        }
    }
}

/// A registered root needs no shared-owner allocation to protect its graph.
/// Keep its removal guard on the stack until admission has ended. Cancellation
/// drops the exact registration; successful completion transfers it unchanged.
pub(crate) struct PreparedRoot<T> {
    ptr: NonNull<T>,
    registration: OwnedRoot,
}

impl<T: Trace> PreparedRoot<T> {
    pub(crate) fn finish(self) -> Gc<T> {
        Gc {
            id: self.registration.target,
            ptr: self.ptr,
            phantom: PhantomData,
            owner: Owner::Root(RootArc::new(self.registration)),
        }
    }
}

// Prepared publication owns metadata and graph protection across thread handoff.
#[path = "publication_owner.rs"]
mod publication_owner;
use publication_owner::Publication;

/// An owning registration already keeps the collector alive. Store exactly one
/// ownership mode so cloning an owning handle touches only its registration's
/// reference count. Heap edges retain weak collector access for deletion barriers.
#[derive(Clone)]
enum Owner {
    Edge(Weak<GarbageCollector>),
    Traced(crate::metadata::MetadataRef),
    Root(RootArc<OwnedRoot>),
}

impl Owner {
    fn edge_heap(&self) -> Option<&Weak<GarbageCollector>> {
        match self {
            Self::Edge(heap) => Some(heap),
            Self::Traced(metadata) => Some(&metadata.collector),
            Self::Root(_) => None,
        }
    }
}

/// A garbage collected pointer type for `T`.
///
/// `Gc<T>` provides shared ownership of a value of type `T`, allocated in the heap. Unlike
/// `Arc<T>`, `Gc<T>` can automatically detect and collect circular references using a low-pause,
/// generational, concurrent collector with reachability analysis.
pub struct Gc<T> {
    id: crate::ObjectId,
    pub(crate) ptr: NonNull<T>,
    pub(crate) phantom: PhantomData<T>,
    owner: Owner,
}

impl<T> Gc<T> {
    /// Own descriptor storage and logical credit until registration commits.
    #[expect(clippy::expect_used, reason = "legacy infallible allocation API")]
    fn allocate(
        collector: &GarbageCollector,
        mut data: T,
    ) -> (
        Self,
        crate::allocation::Initialized<Allocation<T>>,
        crate::collector::AllocationReservation<'_>,
    )
    where
        T: Trace,
    {
        collector
            .prepare_payload(&mut data)
            .expect("region heap exhausted; use Gc::try_new");
        let size = Self::allocation_size(&data).expect("managed allocation size overflow");
        let id = crate::ObjectId::allocate().expect("object identity exhausted");
        let reservation = collector
            .reserve_exact_bytes(size)
            .expect("managed heap limit exhausted; use Gc::try_new for fallible allocation");
        let storage = collector
            .default_descriptor(Allocation::<T>::layout())
            .expect("descriptor allocation failed; use Gc::try_new");
        // SAFETY: storage has the complete descriptor's size and alignment.
        let allocation =
            unsafe { crate::allocation::Initialized::new(storage, Allocation::new(data)) };
        // SAFETY: this initialized allocation is still exclusively owned. Both its
        // storage and logical reservation unwind if attachment or metadata hooks panic.
        unsafe {
            (*allocation.ptr())
                .value
                .attach(collector.trace_owner(allocation.ptr() as usize));
        }
        // Derive the published pointer after attachment's exclusive borrow ends.
        let ptr = NonNull::new(allocation.ptr())
            .expect("initialized descriptor is non-null")
            .cast::<T>();
        let gc = Self {
            id,
            ptr,
            phantom: PhantomData,
            owner: Owner::Edge(collector.weak()),
        };
        (gc, allocation, reservation)
    }

    /// Constructs a new `Gc<T>` and registers it as a root.
    ///
    /// This returns a `GcRootGuard<T>` which ensures the object is rooted.
    /// To get the inner `Gc<T>` for use in data structures, use `guard.clone_gc()`.
    #[expect(clippy::new_ret_no_self)]
    pub fn new(collector: &Arc<GarbageCollector>, data: T) -> GcRootGuard<T>
    where
        T: Send + Sync + Trace + 'static,
    {
        Self::with_collector(collector, data)
    }

    /// Bytes charged for a value and its exclusively owned backing storage.
    /// # Errors
    /// Returns `OutOfMemory` if the size overflows.
    pub fn allocation_size(data: &T) -> Result<usize>
    where
        T: Trace,
    {
        Allocation::<T>::layout()
            .size()
            .checked_add(data.external_size())
            .ok_or(Error::OutOfMemory)
    }

    /// Fallibly allocates and roots a value within the configured managed heap limit.
    /// Backing storage is charged through `Trace::external_size`.
    /// # Errors
    /// Returns `OutOfMemory` for heap-limit exhaustion, size overflow, or allocation failure.
    pub fn try_new(collector: &Arc<GarbageCollector>, mut data: T) -> Result<GcRootGuard<T>>
    where
        T: Send + Sync + Trace + 'static,
    {
        for attempt in 0..=2 {
            match collector.prepare_payload(&mut data) {
                Ok(()) => break,
                Err(error) if attempt == 2 || crate::cell::has_guards() => return Err(error),
                Err(_) => {
                    collector.recover_allocation()?;
                }
            }
        }
        Self::publish(collector, data).map_err(|(error, _)| error)
    }

    /// Attempts preparation and publication once, without running collection.
    /// # Errors
    /// Returns the error and unpublished value, allowing the caller to retry after
    /// releasing locks or awaiting collection. Successful preparation may change
    /// backing storage even if the later publication reservation fails.
    pub fn try_new_once(
        collector: &Arc<GarbageCollector>,
        mut data: T,
    ) -> Result<GcRootGuard<T>, (Error, T)>
    where
        T: Send + Sync + Trace + 'static,
    {
        if let Err(error) = collector.prepare_payload(&mut data) {
            return Err((error, data));
        }
        Self::publish(collector, data)
    }

    /// Allocates a rooted value, resuming as soon as reclaimed space can be reserved.
    /// Payload preparation and logical reservation fail after two recovery cycles
    /// without progress. Physical reclamation consumed by competing allocators
    /// resets preparation's failure allowance; sustained contention can delay it.
    /// The unpublished value remains owned by the future across retries and is
    /// dropped if the future is cancelled. Construction edges must be rooted as
    /// for the synchronous constructor. No publication lock spans an await.
    /// # Errors
    /// Returns the allocation error after bounded recovery, or a collection error.
    pub async fn try_new_async(
        collector: &Arc<GarbageCollector>,
        mut data: T,
    ) -> Result<GcRootGuard<T>>
    where
        T: Send + Sync + Trace + 'static,
    {
        if crate::cell::has_guards() {
            return Err(Error::CollectionPhaseError(
                "asynchronous publication requires released managed guards".into(),
            ));
        }
        let mut retry = crate::recovery::PhysicalRetry::default();
        loop {
            match collector.prepare_payload_async(&mut data).await {
                Ok(()) => break,
                Err(error) => {
                    if !matches!(error, Error::OutOfMemory)
                        || retry.exhausted(collector)
                        || crate::cell::has_guards()
                    {
                        return Err(error);
                    }
                    collector.recover_allocation_async().await?;
                }
            }
        }
        let size = Self::allocation_size(&data)?;
        for attempt in 0..=2 {
            let reservation = collector.reserve_bytes_async(size).await?;
            match Self::publish_reserved(collector, data, reservation) {
                Ok(root) => return Ok(root),
                Err((error, value)) => {
                    if !matches!(error, Error::OutOfMemory)
                        || attempt == 2
                        || crate::cell::has_guards()
                    {
                        return Err(error);
                    }
                    // Host descriptor allocation can fail even with logical credit.
                    // Its reservation was refunded before attempting full recovery.
                    data = value;
                    collector.recover_allocation_async().await?;
                }
            }
        }
        Err(Error::OutOfMemory)
    }

    fn publish(collector: &Arc<GarbageCollector>, data: T) -> Result<GcRootGuard<T>, (Error, T)>
    where
        T: Send + Sync + Trace + 'static,
    {
        let size = match Self::allocation_size(&data) {
            Ok(size) => size,
            Err(error) => return Err((error, data)),
        };
        let reservation = match collector.reserve_bytes(size) {
            Ok(reservation) => reservation,
            Err(error) => return Err((error, data)),
        };
        Self::publish_reserved(collector, data, reservation)
    }

    fn publish_reserved(
        collector: &Arc<GarbageCollector>,
        data: T,
        reservation: crate::collector::AllocationReservation<'_>,
    ) -> Result<GcRootGuard<T>, (Error, T)>
    where
        T: Send + Sync + Trace + 'static,
    {
        Self::prepare_reserved(collector, data, reservation, true)
            .map(|publication| publication.finish_rooted(collector))
    }

    /// Prepare unique storage without choosing its publication backend. Keeping
    /// root installation in Publication preserves cleanup ordering on unwind.
    fn prepare_reserved<'gc>(
        collector: &'gc GarbageCollector,
        data: T,
        reservation: crate::collector::AllocationReservation<'gc>,
        rooted: bool,
    ) -> Result<Publication<'gc, T>, (Error, T)>
    where
        T: Send + Sync + Trace + 'static,
    {
        let _measurement = collector
            .diagnostics
            .descriptor_publication
            .measure(collector.diagnostics_enabled());
        let id = match crate::ObjectId::allocate() {
            Ok(id) => id,
            Err(error) => return Err((error, data)),
        };
        // Hook-driven unrooting transfers to temporary roots until registration.
        // Allocation and arbitrary hooks do not retain global publication admission.
        let mut construction = crate::construction::Roots::new(collector);
        let construction_scope = construction.enter_for::<T>();
        let layout = Allocation::<T>::layout();
        let storage = match collector.allocate_descriptor(layout) {
            Ok(storage) => storage,
            Err(error) => return Err((error, data)),
        };
        // SAFETY: storage was reserved with the complete Allocation<T> layout.
        let allocation =
            unsafe { crate::allocation::Initialized::new(storage, Allocation::new(data)) };
        // SAFETY: the unpublished descriptor remains exclusively owned.
        unsafe {
            (*allocation.ptr())
                .value
                .attach(collector.trace_owner(allocation.ptr() as usize));
        }
        // SAFETY: both storage variants contain a non-null live allocation.
        let raw = unsafe { NonNull::new_unchecked(allocation.ptr()) };
        // Derive the published pointer from the allocation after attach's mutable
        // borrow ends. A pointer reborrowed from `data` before attach is invalidated
        // by the hook's subsequent exclusive borrow (including for scalar values).
        let ptr = raw.cast::<T>();
        let handle = Self {
            id,
            ptr,
            phantom: PhantomData,
            owner: Owner::Edge(collector.weak()),
        };
        // Metadata hooks finish before the initial parent root is installed.
        // The publication record owns root removal before private storage drop,
        // with the sealed construction graph retained through payload destruction.
        let metadata = collector.prepare_allocated_object::<T>(allocation, reservation, id);
        let publication = Publication::new(
            collector,
            handle,
            metadata,
            crate::construction::Detached::from_scope(collector, construction_scope),
            rooted,
        );
        Ok(publication)
    }

    /// Constructs a new `Gc<T>` without rooting it.
    ///
    /// # Safety
    ///
    /// The returned `Gc<T>` is not rooted. If a garbage collection cycle occurs before this `Gc<T>`
    /// is reachable from a root, it may be collected. Use this only when you are sure the object
    /// will be immediately rooted or stored in a reachable object.
    pub unsafe fn new_unrooted(collector: &GarbageCollector, data: T) -> Self
    where
        T: Send + Sync + Trace + 'static,
    {
        // Safety: The caller guarantees that the returned Gc<T> will be rooted immediately
        unsafe { Self::with_collector_unrooted(collector, data) }
    }

    /// Constructs a new `Gc<T>` with finalization support and registers it as a root.
    pub fn new_with_finalizer(collector: &Arc<GarbageCollector>, data: T) -> GcRootGuard<T>
    where
        T: Send + Sync + Finalize + Trace + 'static,
    {
        Self::with_collector_and_finalizer(collector, data)
    }

    /// Constructs a new `Gc<T>` with a specific garbage collector and registers it as a root.
    ///
    /// The root is registered **before** the object is added to the GC's tracking map.
    /// This prevents a race condition where a concurrent GC cycle could unmark and sweep
    /// the object before its root is visible.
    ///
    pub fn with_collector(collector: &Arc<GarbageCollector>, data: T) -> GcRootGuard<T>
    where
        T: Send + Sync + Trace + 'static,
    {
        let mut construction = crate::construction::Roots::new(collector);
        let construction_scope = construction.enter_for::<T>();
        let (gc, allocation, reservation) = Self::allocate(collector, data);

        // Register root BEFORE registering object to prevent race with GC cycle.
        // If the root exists first, any GC cycle will find the root during marking.
        // The object isn't in the tracking map yet, so it can't be swept.
        let metadata = collector.prepare_allocated_object::<T>(allocation, reservation, gc.id);
        let publication = Publication::new(
            collector,
            gc,
            metadata,
            crate::construction::Detached::from_scope(collector, construction_scope),
            true,
        );
        publication.finish_rooted(collector)
    }

    /// Constructs a new `Gc<T>` with a specific garbage collector without rooting it.
    ///
    /// # Safety
    ///
    /// The returned `Gc<T>` is not rooted.
    ///
    /// # Panics
    ///
    /// Panics if payload preparation, descriptor allocation, or logical reservation fails.
    pub unsafe fn with_collector_unrooted(collector: &GarbageCollector, data: T) -> Self
    where
        T: Send + Sync + Trace + 'static,
    {
        let mut construction = crate::construction::Roots::new(collector);
        let construction_scope = construction.enter_for::<T>();
        let (gc, allocation, reservation) = Self::allocate(collector, data);
        let metadata = collector.prepare_allocated_object::<T>(allocation, reservation, gc.id);
        let publication = Publication::new(
            collector,
            gc,
            metadata,
            crate::construction::Detached::from_scope(collector, construction_scope),
            false,
        );
        publication.finish_unrooted(collector)
    }

    /// Constructs a new `Gc<T>` with a specific garbage collector and finalization support.
    ///
    /// The root is registered **before** the object is added to the GC's tracking map.
    /// This prevents a race condition where a concurrent GC cycle could unmark and sweep
    /// the object before its root is visible.
    ///
    pub fn with_collector_and_finalizer(
        collector: &Arc<GarbageCollector>,
        data: T,
    ) -> GcRootGuard<T>
    where
        T: Send + Sync + Finalize + Trace + 'static,
    {
        let mut construction = crate::construction::Roots::new(collector);
        let construction_scope = construction.enter_for::<T>();
        let (gc, allocation, reservation) = Self::allocate(collector, data);

        // Register root BEFORE registering object (same rationale as with_collector)
        let metadata =
            collector.prepare_allocated_object_with_finalizer::<T>(allocation, reservation, gc.id);
        let publication = Publication::new(
            collector,
            gc,
            metadata,
            crate::construction::Detached::from_scope(collector, construction_scope),
            true,
        );
        publication.finish_rooted(collector)
    }

    /// Constructs a new `Gc<T>` with a specific garbage collector and finalization support without
    /// rooting.
    ///
    /// # Safety
    ///
    /// The returned `Gc<T>` is not rooted.
    ///
    /// # Panics
    ///
    /// Panics if payload preparation, descriptor allocation, or logical reservation fails.
    pub unsafe fn with_collector_and_finalizer_unrooted(
        collector: &GarbageCollector,
        data: T,
    ) -> Self
    where
        T: Send + Sync + Finalize + Trace + 'static,
    {
        let mut construction = crate::construction::Roots::new(collector);
        let construction_scope = construction.enter_for::<T>();
        let (gc, allocation, reservation) = Self::allocate(collector, data);
        let metadata =
            collector.prepare_allocated_object_with_finalizer::<T>(allocation, reservation, gc.id);
        let publication = Publication::new(
            collector,
            gc,
            metadata,
            crate::construction::Detached::from_scope(collector, construction_scope),
            false,
        );
        publication.finish_unrooted(collector)
    }

    pub(crate) fn collector(&self) -> Weak<GarbageCollector> {
        match &self.owner {
            Owner::Edge(collector) => collector.clone(),
            Owner::Traced(metadata) => metadata.collector.clone(),
            Owner::Root(root) => Arc::downgrade(&root.collector),
        }
    }

    /// Stable identity for managed handles, independent of storage addresses.
    /// Unassociated unsafe raw wrappers have the sentinel identity zero; resolve them
    /// through `from_raw_rooted` before using identity-based VM APIs.
    #[must_use]
    pub fn identity(&self) -> crate::ObjectId {
        self.id
    }

    /// Encodes a stable reference for VM and JIT runtime helpers.
    #[must_use]
    pub fn as_handle_i64(&self) -> i64 {
        self.id.to_i64()
    }

    /// Resolves and roots a typed identity in this heap.
    /// # Errors
    /// Rejects expired, foreign-heap, null, and incorrectly typed identities.
    pub fn from_handle_rooted(collector: &Arc<GarbageCollector>, handle: i64) -> Result<Self>
    where
        T: Trace + 'static,
    {
        let expected = crate::ObjectId::from_i64(handle);
        let address = collector
            .address_for_identity(expected)
            .ok_or_else(|| Error::InvalidPointer("unknown object identity".into()))?;
        Self::root_at_address(collector, address, Some(expected))
    }

    fn root_at_address(
        collector: &Arc<GarbageCollector>,
        address: usize,
        expected: Option<crate::ObjectId>,
    ) -> Result<Self>
    where
        T: Trace + 'static,
    {
        let (ptr, target, registration) = collector.acquire_typed_root::<T>(address, expected)?;
        Ok(PreparedRoot {
            ptr,
            registration: OwnedRoot {
                collector: collector.clone(),
                id: registration,
                target,
            },
        }
        .finish())
    }

    /// Returns `true` if the two `Gc`s point to the same allocation.
    #[must_use]
    pub fn ptr_eq(&self, other: &Self) -> bool {
        self.ptr == other.ptr
    }

    /// Returns a raw pointer to the data.
    ///
    /// The caller must ensure that the `Gc` outlives the pointer this function returns, or else it
    /// will end up pointing to garbage.
    #[must_use]
    pub fn as_ptr(&self) -> *const T {
        self.ptr.as_ptr()
    }

    /// Returns the raw pointer as an `i64` value suitable for passing through JIT-compiled code.
    ///
    /// The caller must ensure that the `Gc` outlives the usage of this pointer value.
    #[must_use]
    pub fn as_ptr_i64(&self) -> i64 {
        self.as_ptr() as i64
    }

    /// Constructs a `Gc<T>` from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must have been obtained from `Gc::as_ptr()` on a still live `Gc<T>`.
    /// The original `Gc` (or the GC root keeping the allocation alive) must still be valid.
    /// This does not create a new allocation or register with the garbage collector.
    ///
    /// # Errors
    ///
    /// Returns an error if the pointer is null.
    pub unsafe fn from_raw(ptr: *const T) -> Result<Self> {
        let ptr = NonNull::new(ptr.cast_mut()).ok_or_else(|| {
            Error::InvalidPointer("Gc::from_raw received null pointer".to_string())
        })?;
        Ok(Self {
            id: crate::ObjectId::from_i64(0),
            ptr,
            phantom: PhantomData,
            owner: Owner::Edge(Weak::new()),
        })
    }

    /// Constructs a `Gc<T>` from a raw pointer encoded as an `i64`.
    ///
    /// This is the inverse of casting `Gc::as_ptr()` to `i64`. The pointer must have been
    /// obtained from a still live `Gc<T>`. This is intended for JIT interop where pointers
    /// are passed through compiled code as integer values.
    ///
    /// # Errors
    ///
    /// Returns an error if the pointer is null, negative, or cannot be represented as a pointer.
    /// # Safety
    /// The address must identify a live T whose lifetime is protected by the caller.
    pub unsafe fn from_raw_i64(ptr: i64) -> Result<Self> {
        let address = usize::try_from(ptr).map_err(|_| {
            Error::InvalidPointer(format!("Gc::from_raw_i64 received negative pointer {ptr}"))
        })?;
        let raw = address as *const T;
        // Safety: The caller guarantees this pointer came from Gc::as_ptr() on a live Gc<T>.
        unsafe { Self::from_raw(raw) }
    }

    /// Makes a mutable reference into the given `Gc`.
    ///
    /// # Safety
    /// The caller must protect this allocation and its graph for the entire
    /// borrow and exclude all conflicting access, including collector tracing
    /// and relocation. Restore owning protection before any embedded edge or
    /// managed cell can escape through this mutable reference. Demotion and
    /// attachment must use the actual owner, and mutation barriers must be
    /// published before collector access resumes. This method supplies none of
    /// those protections; use a managed cell guard for ordinary mutation.
    #[must_use]
    pub unsafe fn get_mut_unchecked(&mut self) -> &mut T {
        // Safety: The caller guarantees no other references exist,
        // and we have a mutable reference to self, so we can safely
        // provide mutable access to the data
        unsafe { self.ptr.as_mut() }
    }

    /// Returns a reference to the inner data.
    pub(crate) fn inner(&self) -> &T {
        // Safety: self.ptr is guaranteed to be valid and non-null
        // because it was created from Box::into_raw and stored in NonNull
        unsafe { self.ptr.as_ref() }
    }

    /// Checks the allocation's associated heap without cloning a weak reference.
    pub(crate) fn belongs_to(&self, collector: &GarbageCollector) -> bool {
        let source = match &self.owner {
            Owner::Root(root) => Arc::as_ptr(&root.collector),
            Owner::Edge(source) => source.as_ptr(),
            Owner::Traced(metadata) => metadata.collector.as_ptr(),
        };
        std::ptr::eq(source, collector)
    }

    /// Clones this handle with an owning root, suitable for Rust locals and suspended futures.
    /// This is equivalent to `Clone::clone`.
    ///
    /// # Panics
    /// Panics if the handle is raw, expired, logically retired, or has a stale identity.
    #[must_use]
    #[expect(
        clippy::expect_used,
        reason = "cloning rejects unassociated or retired handles"
    )]
    pub fn rooted_clone(&self) -> Self {
        match &self.owner {
            Owner::Root(root) => Self {
                id: self.id,
                ptr: self.ptr,
                phantom: PhantomData,
                owner: Owner::Root(root.clone()),
            },
            Owner::Edge(_) | Owner::Traced(_) => {
                let collector = self
                    .owner
                    .edge_heap()
                    .and_then(Weak::upgrade)
                    .expect("rooting requires a live managed allocation");
                let id = collector
                    .clone_edge_root(self)
                    .expect("rooting requires a live managed allocation with matching identity");
                Self {
                    id: self.id,
                    ptr: self.ptr,
                    phantom: PhantomData,
                    owner: Owner::Root(RootArc::new(OwnedRoot {
                        collector,
                        id,
                        target: self.id,
                    })),
                }
            }
        }
    }

    /// Copy a guard's inner handle without acquiring a second registration.
    ///
    /// # Safety
    /// The caller must retain `registration` in the resulting root guard. A
    /// replaced handle cannot borrow the old target's protection and is rooted
    /// independently instead.
    pub(crate) unsafe fn clone_for_guard(&self, registration: &OwnedRoot) -> Self {
        if self.id == registration.target {
            Self {
                id: self.id,
                ptr: self.ptr,
                phantom: PhantomData,
                owner: self.owner.clone(),
            }
        } else {
            self.clone()
        }
    }

    /// Shares the guard's registration when it still protects this identity.
    pub(crate) fn clone_with_registration(&self, registration: RootArc<OwnedRoot>) -> Self {
        if self.id == registration.target {
            Self {
                id: self.id,
                ptr: self.ptr,
                phantom: PhantomData,
                owner: Owner::Root(registration),
            }
        } else {
            self.clone()
        }
    }

    pub(crate) fn with_registration(mut self, registration: RootArc<OwnedRoot>) -> Self
    where
        T: Trace,
    {
        if registration.target == self.id {
            self.owner = Owner::Root(registration);
            self
        } else {
            // DerefMut on a root guard permits replacing its handle. The original
            // registration then protects a different allocation and cannot transfer.
            self.rooted_clone()
        }
    }

    /// Root-guard destruction releases the matching registration immediately
    /// afterward; that removal publishes the required deletion barrier. Avoid a
    /// second edge barrier for the same allocation. Replaced/foreign handles keep
    /// their ordinary edge-drop behavior. Explicit root revocation already ran
    /// its deletion barrier and cannot be undone by dropping this stale guard.
    pub(crate) fn suppress_guard_edge_barrier(&mut self, registration: &OwnedRoot) {
        if self.id == registration.target
            && let Some(collector) = self.owner.edge_heap()
            && std::ptr::eq(collector.as_ptr(), Arc::as_ptr(&registration.collector))
        {
            self.owner = Owner::Edge(Weak::new());
        }
    }

    /// Converts a rooted handle into an edge owned by a traced heap allocation.
    ///
    /// # Safety
    /// The same heap's tracing or another root must protect this handle and its
    /// graph until its final payload access. It must regain owning protection
    /// before escaping that lifetime. Identity-only inspection and dropping the
    /// edge after reclamation are allowed; payload access is not.
    ///
    /// ```compile_fail,E0133
    /// use ristretto_gc::Gc;
    /// fn demote(value: &mut Gc<u64>) { value.unroot(); }
    /// ```
    pub unsafe fn unroot(&mut self) {
        if let Owner::Root(root) = &self.owner {
            // Establish weak access before releasing the owning registration:
            // removal may run a deletion barrier or release the last collector.
            self.owner = Owner::Edge(Arc::downgrade(&root.collector));
        }
    }

    /// Reconstructs an owning handle after validating the allocation address and Rust type.
    /// Validation and root registration exclude concurrent reclamation. Physically
    /// present allocations that are already logically dead cannot be resurrected.
    /// # Errors
    /// Returns an error for an invalid integer address.
    pub fn from_raw_rooted(collector: &Arc<GarbageCollector>, address: i64) -> Result<Self>
    where
        T: Trace + 'static,
    {
        let address = usize::try_from(address)
            .map_err(|_| Error::InvalidPointer("invalid allocation address".into()))?;
        Self::root_at_address(collector, address, None)
    }

    /// Retaining conditional edges may admit a white graph before sweep starts.
    /// The caller must first validate the slot identity and reference decision.
    pub(crate) fn from_conditional_rooted(
        collector: &Arc<GarbageCollector>,
        address: i64,
        retaining: bool,
        expected: Option<crate::ObjectId>,
    ) -> Result<Self>
    where
        T: Trace + 'static,
    {
        let address_usize = usize::try_from(address)
            .map_err(|_| Error::InvalidPointer("invalid managed address".into()))?;
        let mut reservation = collector.reserve_owned_root(address_usize);
        let publication = collector.root_publication_guard();
        let prepared = Self::prepare_conditional_root(
            collector,
            address,
            retaining,
            expected,
            &publication,
            &mut reservation,
        )?;
        drop(publication);
        Ok(prepared.finish())
    }

    /// The same admission must span reference/identity validation and registration.
    pub(crate) fn prepare_conditional_root(
        collector: &Arc<GarbageCollector>,
        address: i64,
        retaining: bool,
        expected: Option<crate::ObjectId>,
        publication: &crate::collector::RootPublication<'_>,
        reservation: &mut crate::collector::RootReservation<'_>,
    ) -> Result<PreparedRoot<T>>
    where
        T: Trace + 'static,
    {
        if !publication.protects(collector) {
            return Err(Error::InvalidPointer("foreign publication token".into()));
        }
        let address_usize = usize::try_from(address)
            .map_err(|_| Error::InvalidPointer("invalid managed address".into()))?;
        if !reservation.protects(collector, address_usize) {
            return Err(Error::InvalidPointer("foreign root reservation".into()));
        }
        let metadata = collector
            .resolve_address::<T>(address_usize, retaining, expected)
            .ok_or_else(|| {
                Error::InvalidPointer(
                    "unknown, logically dead, or incorrectly typed allocation".into(),
                )
            })?;
        let ptr = NonNull::new(metadata.trace.ptr.cast_mut().cast::<T>())
            .ok_or_else(|| Error::InvalidPointer("null conditional root".into()))?;
        let registration = OwnedRoot {
            collector: collector.clone(),
            id: publication.add_conditional_root(&metadata, reservation),
            target: metadata.id,
        };
        Ok(PreparedRoot { ptr, registration })
    }

    /// Add this `Gc` object as a root to the global garbage collector.
    /// Returns a `GcRootGuard` that automatically removes the root when dropped.
    ///
    /// # Errors
    ///
    /// Returns an error if this allocation belongs to a different collector,
    /// has no associated collector, or the borrowed edge has been retired.
    pub fn as_root(&self, collector: &Arc<GarbageCollector>) -> Result<GcRootGuard<T>>
    where
        T: Trace,
    {
        if !self.belongs_to(collector) {
            return Err(Error::InvalidPointer(
                "root guard collector does not own this allocation".into(),
            ));
        }
        let registration = match &self.owner {
            Owner::Root(_) => collector.add_owned_root(self),
            Owner::Edge(_) | Owner::Traced(_) => {
                collector.clone_edge_root(self).ok_or_else(|| {
                    Error::InvalidPointer("root guard target is no longer live".into())
                })?
            }
        };
        // The guard owns the new registration. Copying the inner handle can
        // preserve its current ownership without promoting an edge a second time.
        let root = Self {
            id: self.id,
            ptr: self.ptr,
            phantom: PhantomData,
            owner: self.owner.clone(),
        };
        Ok(GcRootGuard::new(collector.clone(), registration, root))
    }

    /// Triggers a write barrier for this `Gc`.
    ///
    /// This must be called whenever a reference to a `Gc` object is written into
    /// a field of another object during concurrent execution.
    pub fn write_barrier(&self, collector: &GarbageCollector)
    where
        T: Trace,
    {
        collector.write_barrier(self);
    }
}

impl<T> Clone for Gc<T> {
    /// Creates an owning handle. Cloning a borrowed heap edge registers a root;
    /// cloning an owning handle shares its existing registration.
    ///
    /// # Panics
    /// Panics for unassociated raw handles or expired heap edges. Use `ObjectId`
    /// or `GcWeak` to retain an identity without keeping its allocation alive.
    fn clone(&self) -> Self {
        self.rooted_clone()
    }
}

impl<T> Deref for Gc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.inner()
    }
}

impl<T> Drop for Gc<T> {
    fn drop(&mut self) {
        if let Some(collector) = self.owner.edge_heap().and_then(Weak::upgrade) {
            collector.snapshot_edge(self.ptr.as_ptr() as usize);
        }
    }
}

// Safety: Gc<T> can be sent between threads when T: Send + Sync because:
// 1. The NonNull<Gc<T>> pointer is just a pointer address
// 2. The actual data access is controlled by the garbage collector
// 3. T is required to be Send + Sync by the constructor bounds
// 4. The GC ensures proper synchronization during object access
unsafe impl<T: Sync + Send> Send for Gc<T> {}

// Safety: Gc<T> can be shared between threads when T: Send + Sync because:
// 1. The NonNull<Gc<T>> pointer is immutable after construction
// 2. Data access goes through Deref which provides shared references
// 3. T is required to be Send + Sync by the constructor bounds
// 4. The garbage collector handles thread safety for the underlying data
unsafe impl<T: Sync + Send> Sync for Gc<T> {}

impl<T: fmt::Display> fmt::Display for Gc<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&**self, f)
    }
}

impl<T: fmt::Debug> fmt::Debug for Gc<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

impl<T> fmt::Pointer for Gc<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Pointer::fmt(&self.as_ptr(), f)
    }
}

impl<T: PartialEq> PartialEq for Gc<T> {
    fn eq(&self, other: &Self) -> bool {
        **self == **other
    }
}

impl<T: Eq> Eq for Gc<T> {}

impl<T: PartialOrd> PartialOrd for Gc<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (**self).partial_cmp(&**other)
    }
}

impl<T: Ord> Ord for Gc<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        (**self).cmp(&**other)
    }
}

impl<T: Hash> Hash for Gc<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (**self).hash(state);
    }
}

impl<T> Borrow<T> for Gc<T> {
    fn borrow(&self) -> &T {
        self
    }
}

impl<T> AsRef<T> for Gc<T> {
    fn as_ref(&self) -> &T {
        self
    }
}

// SAFETY: same-heap edges are traced, owning roots protect foreign heaps, and
// root restores ownership before a handle can move independently of its owner.
unsafe impl<T: Trace> Trace for Gc<T> {
    fn edges_rooted_before_unroot(&self) -> bool {
        matches!(self.owner, Owner::Root(_))
    }

    fn root(&mut self) {
        if self.owner.edge_heap().is_some() {
            *self = self.rooted_clone();
        }
    }
    unsafe fn unroot(&mut self, owner: crate::TraceOwner<'_>) {
        let same_heap = match &self.owner {
            Owner::Root(root) => {
                std::ptr::eq(Arc::as_ptr(&root.collector), owner.collector().as_ptr())
            }
            Owner::Edge(collector) => Weak::ptr_eq(collector, owner.collector()),
            Owner::Traced(metadata) => Weak::ptr_eq(&metadata.collector, owner.collector()),
        };
        if same_heap {
            // Only edges inside a traced graph retain descriptor metadata.
            // An explicitly demoted external handle retains weak heap access.
            if let Owner::Root(root) = &self.owner {
                self.owner = root
                    .collector
                    .traced_edge(self.ptr.as_ptr().addr(), self.id)
                    .map_or_else(
                        || Owner::Edge(Arc::downgrade(&root.collector)),
                        Owner::Traced,
                    );
            }
        } else {
            // A different collector cannot trace this allocation. Retain its
            // source heap independently, including when an edge is rebound.
            self.root();
        }
    }
    fn trace_slice(items: &[Self], collector: &GarbageCollector) {
        for group in items.chunks(8) {
            let descriptors = group.iter().map(|item| match &item.owner {
                Owner::Traced(metadata) => Some(metadata),
                _ => None,
            });
            let handled = collector.try_mark_mature_slice(group.len(), descriptors);
            for item in group.iter().skip(handled) {
                item.trace(collector);
            }
        }
    }

    fn trace(&self, collector: &GarbageCollector) {
        if let Owner::Traced(metadata) = &self.owner {
            collector.mark_traced_edge(metadata);
            return;
        }
        // Mark this object as reachable in the object registry
        let ptr = SafePtr::from_ptr(self.ptr.as_ptr().cast::<u8>());

        collector.mark_object(ptr, self.id);
    }
}

#[cfg(feature = "experimental-canonical-leaves")]
pub(crate) fn leaf_lifecycle_benchmark(
    mode: &str,
    operations: u64,
    batch: usize,
) -> Result<String> {
    publication_owner::leaf_lifecycle_benchmark(mode, operations, batch)
}

#[cfg(feature = "experimental-canonical-leaves")]
pub(crate) fn graph_lifecycle_benchmark(rounds: u64, capacity: usize) -> Result<String> {
    publication_owner::graph_lifecycle_benchmark(rounds, capacity)
}

#[cfg(feature = "experimental-canonical-leaves")]
pub(crate) fn leaf_contention(
    mode: &str,
    actors: usize,
    helpers: usize,
    operations: usize,
    capacity: usize,
    retained: usize,
) -> Result<String> {
    publication_owner::leaf_contention(mode, actors, helpers, operations, capacity, retained)
}

#[cfg(feature = "experimental-canonical-leaves")]
pub(crate) fn root_alias_benchmark(
    mode: &str,
    copies: u64,
    checkpoint: impl FnMut(&'static str),
) -> Result<String> {
    publication_owner::root_alias_benchmark(mode, copies, checkpoint)
}

#[cfg(feature = "experimental-canonical-audit")]
pub(crate) fn root_refund_order(
    mode: &str,
    checkpoint: impl FnMut(&'static str, usize),
) -> Result<()> {
    publication_owner::root_refund_order(mode, checkpoint)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{HashMap, HashSet};

    #[test]
    #[expect(
        clippy::panic_in_result_fn,
        reason = "exact descriptor and refund checks"
    )]
    fn managed_edges_release_the_payload_while_descriptor_witnesses_remain() -> Result<()> {
        let gc = GarbageCollector::new();
        let child = Gc::try_new(&gc, 17_u64)?;
        let identity = child.as_handle_i64();
        let graph = Gc::try_new(&gc, vec![child.into_rooted()])?;
        let edge = graph.first().expect("child edge");
        let Owner::Traced(metadata) = &edge.owner else {
            panic!("expected a managed edge")
        };
        let saved = metadata.clone();
        gc.collect_full()?;
        assert_eq!(**edge, 17);
        drop(graph);
        gc.collect_full()?;
        assert_eq!(gc.managed_bytes(), 0);
        assert!(!saved.birth.is_live());
        assert!(!saved.birth.unpublished());
        assert!(Gc::<u64>::from_handle_rooted(&gc, identity).is_err());
        Ok(())
    }

    #[test]
    #[expect(clippy::panic_in_result_fn, reason = "private cancellation invariants")]
    fn initial_root_cancellation_releases_registration_without_scope_adoption() -> Result<()> {
        for counted in [false, true] {
            for storage in [
                crate::RootStorage::Tree,
                crate::RootStorage::Slots,
                crate::RootStorage::Contexts,
            ] {
                let collector = GarbageCollector::with_config(
                    crate::Configuration::builder()
                        .concurrent_publication(counted)
                        .root_storage(storage)
                        .allocation_threshold(usize::MAX)
                        .build(),
                );
                let context = collector.register_context();
                context.with(|| -> Result<()> {
                    let mut construction = crate::construction::Roots::new(&collector);
                    let scope = construction.enter();
                    let (handle, allocation, reservation) = Gc::allocate(&collector, 42_u64);
                    let metadata = collector.prepare_allocated_object::<u64>(
                        allocation,
                        reservation,
                        handle.id,
                    );
                    let initial =
                        Publication::new(&collector, handle, metadata, scope.detach(), true);
                    let roots = collector.root_statistics();
                    assert_eq!(roots.compatibility_roots + roots.slot_roots, 1);
                    drop(initial);
                    let roots = collector.root_statistics();
                    assert_eq!(roots.compatibility_roots + roots.slot_roots, 0);
                    assert_eq!(collector.statistics()?.live_objects, 0);
                    assert_eq!(collector.managed_bytes(), 0);
                    Ok(())
                })?;
                drop(context);
                collector.stop()?;
            }
        }
        Ok(())
    }

    #[test]
    fn test_creation_and_access() {
        let collector = GarbageCollector::new();
        let gc = Gc::new(&collector, 42);
        assert_eq!(**gc, 42);
    }

    #[test]
    fn test_creation_with_different_types() {
        let collector = GarbageCollector::new();
        let gc_int = Gc::new(&collector, 123);
        let gc_string = Gc::new(&collector, "Hello, World!".to_string());
        let gc_vec = Gc::new(&collector, vec![1, 2, 3, 4, 5]);
        let gc_tuple = Gc::new(&collector, (1, "test", 1.23));

        assert_eq!(**gc_int, 123);
        assert_eq!(**gc_string, "Hello, World!");
        assert_eq!(**gc_vec, vec![1, 2, 3, 4, 5]);
        assert_eq!(**gc_tuple, (1, "test", 1.23));
    }

    #[test]
    fn test_clone() {
        let collector = GarbageCollector::new();
        let gc1 = Gc::new(&collector, 42);
        let gc2 = gc1.clone();

        assert_eq!(**gc1, 42);
        assert_eq!(**gc2, 42);
        // Clones point to the same object
        assert!(Gc::ptr_eq(&gc1, &gc2));
    }

    #[test]
    fn test_multiple_clones() {
        let collector = GarbageCollector::new();
        let gc1 = Gc::new(&collector, "shared data".to_string());
        let gc2 = gc1.clone();
        let gc3 = gc1.clone();
        let gc4 = gc2.clone();

        // All clones should point to the same object
        assert!(Gc::ptr_eq(&gc1, &gc2));
        assert!(Gc::ptr_eq(&gc1, &gc3));
        assert!(Gc::ptr_eq(&gc1, &gc4));

        // Verify data access works correctly
        assert_eq!(**gc1, "shared data");
        assert_eq!(**gc2, "shared data");
        assert_eq!(**gc3, "shared data");
        assert_eq!(**gc4, "shared data");
    }

    #[test]
    fn test_drop_behavior() {
        // Test that dropping clones doesn't affect the data
        let collector = GarbageCollector::new();
        let gc1 = Gc::new(&collector, vec![1, 2, 3]);
        let gc2 = gc1.clone();

        drop(gc1);
        // gc2 should still be accessible
        assert_eq!(**gc2, vec![1, 2, 3]);
    }

    #[test]
    fn test_equality() {
        let collector = GarbageCollector::new();
        let gc1 = Gc::new(&collector, 42);
        let gc2 = Gc::new(&collector, 42);
        let gc3 = gc1.clone();

        // Value equality
        assert_eq!(gc1, gc2);
        assert_eq!(gc1, gc3);

        // Pointer equality: gc1 and gc3 point to same object, gc2 is different
        assert!(Gc::ptr_eq(&gc1, &gc3));
        assert!(!Gc::ptr_eq(&gc1, &gc2));
    }

    #[test]
    fn test_with_complex_types() {
        let mut map = HashMap::new();
        map.insert("key1", 10);
        map.insert("key2", 20);

        let collector = GarbageCollector::new();
        let gc_map = Gc::new(&collector, map);
        let gc_map_clone = gc_map.clone();

        assert_eq!(gc_map.get("key1"), Some(&10));
        assert_eq!(gc_map_clone.get("key2"), Some(&20));
        assert!(Gc::ptr_eq(&*gc_map, &*gc_map_clone));
    }

    #[test]
    fn test_as_ptr() {
        let collector = GarbageCollector::new();
        let gc = Gc::new(&collector, 42);
        let ptr = gc.as_ptr();

        unsafe {
            assert_eq!(*ptr, 42);
        }
    }

    #[test]
    fn test_ptr_eq() {
        let collector = GarbageCollector::new();
        let gc1 = Gc::new(&collector, 42);
        let gc2 = Gc::new(&collector, 42);
        let gc3 = gc1.clone();

        assert!(Gc::ptr_eq(&*gc1, &*gc3)); // Same allocation
        assert!(!Gc::ptr_eq(&*gc1, &*gc2)); // Different allocations
    }

    #[test]
    fn test_borrow() {
        let collector = GarbageCollector::new();
        let gc = Gc::new(&collector, "test string".to_string());
        let borrowed: &String = gc.borrow();

        assert_eq!(borrowed, "test string");
        assert_eq!(borrowed.len(), 11);
    }

    #[test]
    fn test_as_ref() {
        let collector = GarbageCollector::new();
        let gc = Gc::new(&collector, vec![1, 2, 3, 4, 5]);
        let vec_ref: &Vec<i32> = gc.as_ref();

        assert_eq!(vec_ref.len(), 5);
        assert_eq!(vec_ref[2], 3);
    }

    #[test]
    fn test_from_trait() {
        let collector = GarbageCollector::new();
        let gc = Gc::new(&collector, 42);
        assert_eq!(**gc, 42);
    }

    #[test]
    fn test_debug_display() {
        let collector = GarbageCollector::new();
        let gc = Gc::new(&collector, 42);
        let debug_str = format!("{gc:?}");
        let display_str = format!("{gc}");

        assert_eq!(debug_str, "42");
        assert_eq!(display_str, "42");
    }

    #[test]
    fn test_pointer_format() {
        let collector = GarbageCollector::new();
        let gc = Gc::new(&collector, 42);
        let ptr_str = format!("{:p}", &*gc);

        // Should format as a pointer (starts with 0x)
        assert!(ptr_str.starts_with("0x"));
    }

    #[test]
    fn test_ordering() {
        let collector = GarbageCollector::new();
        let gc1 = Gc::new(&collector, 10);
        let gc2 = Gc::new(&collector, 20);
        let gc3 = Gc::new(&collector, 10);

        assert!(gc1 < gc2);
        assert!(gc2 > gc1);
        assert_eq!(gc1, gc3);
    }

    #[test]
    fn test_hash() {
        let collector = GarbageCollector::new();
        let gc1 = Gc::new(&collector, 42);
        let gc2 = Gc::new(&collector, 42);
        let gc3 = Gc::new(&collector, 43);

        let mut set = HashSet::new();
        set.insert(gc1.clone());
        set.insert(gc2);
        set.insert(gc3);

        // Should contain 2 unique values (42 and 43)
        assert_eq!(set.len(), 2);
        assert!(set.contains(&gc1));
    }
}
