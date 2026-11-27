//! Preserve roots released by synchronous construction or mutation hooks without retaining
//! publication admission across arbitrary callbacks or system allocation.
use crate::GarbageCollector;
use crate::roots::Registration;
use std::cell::{Cell, RefCell};
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::Ordering;

thread_local! {
    static CURRENT: Cell<*const ()> = const { Cell::new(std::ptr::null()) };
}

pub(crate) struct Roots<'gc> {
    collector: &'gc GarbageCollector,
    previous: Cell<*const ()>,
    construction: bool,
    registrations: RefCell<Vec<Registration>>,
    // Temporary roots protect referents while a per-slot pending count excludes
    // decisions. A stalled hook owns no collector-wide publication admission.
    reference_owners: RefCell<Vec<Arc<crate::weak::WeakSlot>>>,
}

impl<'gc> Roots<'gc> {
    pub(crate) fn new(collector: &'gc GarbageCollector) -> Self {
        Self {
            collector,
            previous: Cell::new(std::ptr::null()),
            construction: true,
            registrations: RefCell::new(Vec::new()),
            reference_owners: RefCell::new(Vec::new()),
        }
    }

    /// Transfer exact owning registrations released by a mutation hook through
    /// enrollment of the already-published owner. A parked hook holds no admission.
    pub(crate) fn finish_write(
        collector: &'gc GarbageCollector,
        address: usize,
        demote: impl FnOnce() -> bool,
    ) {
        struct Notification<'a> {
            collector: &'a GarbageCollector,
            address: usize,
            required: bool,
        }
        impl Drop for Notification<'_> {
            fn drop(&mut self) {
                if self.required {
                    self.collector.remember_address(self.address);
                }
            }
        }
        let mut roots = Self::new(collector);
        roots.construction = false;
        let _scope = roots.enter();
        // Drop order also protects partially demoted custom graphs on unwind.
        let mut notification = Notification {
            collector,
            address,
            required: true,
        };
        notification.required = demote();
    }

    /// The borrow keeps this stack location stable until the scope restores TLS.
    /// Constructors enter once, then seal protection after metadata hooks finish.
    pub(crate) fn enter(&mut self) -> Scope<'_, 'gc> {
        self.previous
            .set(CURRENT.replace(std::ptr::from_ref(self).cast()));
        Scope {
            roots: self,
            _thread: PhantomData,
        }
    }
    /// A reference-free constructor leaves any enclosing TLS scope untouched.
    /// Option uses Scope's non-null reference niche; the ordinary scope stays small.
    pub(crate) fn enter_for<T: crate::Trace>(&mut self) -> Option<Scope<'_, 'gc>> {
        T::may_contain_gc_references().then(|| self.enter())
    }
}

pub(crate) struct Scope<'a, 'gc> {
    roots: &'a Roots<'gc>,
    // A synchronous construction scope must leave TLS on the thread that entered it.
    _thread: PhantomData<Rc<()>>,
}

/// Owned construction protection after hooks leave their thread-local scope.
/// Its shared heap borrow excludes shutdown; no origin TLS borrow survives sealing.
/// Cleanup never adopts a helper's ambient construction scope.
pub(crate) struct Detached<'gc> {
    collector: &'gc GarbageCollector,
    registrations: Vec<Registration>,
    reference_owners: Vec<Arc<crate::weak::WeakSlot>>,
}

impl<'gc> Scope<'_, 'gc> {
    pub(crate) fn detach(self) -> Detached<'gc> {
        let detached = Detached {
            collector: self.roots.collector,
            registrations: std::mem::take(&mut *self.roots.registrations.borrow_mut()),
            reference_owners: std::mem::take(&mut *self.roots.reference_owners.borrow_mut()),
        };
        // Restore the origin's TLS while it is still on that thread. Empty vectors
        // leave no registrations or pending-owner counts for Scope::drop to release.
        drop(self);
        detached
    }
}

/// Exact owned tokens; the shared publication protocol supplies the heap borrow
/// for their eventual cleanup. This private type never grants payload access.
pub(crate) struct OwnedTokens {
    registrations: Vec<Registration>,
    reference_owners: Vec<Arc<crate::weak::WeakSlot>>,
}

impl OwnedTokens {
    pub(crate) fn attach(self, collector: &GarbageCollector) -> Detached<'_> {
        Detached {
            collector,
            registrations: self.registrations,
            reference_owners: self.reference_owners,
        }
    }
}

impl<'gc> Detached<'gc> {
    pub(crate) fn take_owned(&mut self) -> OwnedTokens {
        OwnedTokens {
            registrations: std::mem::take(&mut self.registrations),
            reference_owners: std::mem::take(&mut self.reference_owners),
        }
    }

    /// Only an entered scope owns captured roots. An absent leaf scope neither
    /// installs nor restores TLS and needs no `RefCell` extraction.
    pub(crate) fn from_scope(
        collector: &'gc GarbageCollector,
        scope: Option<Scope<'_, 'gc>>,
    ) -> Self {
        scope.map_or_else(
            || Self {
                collector,
                registrations: Vec::new(),
                reference_owners: Vec::new(),
            },
            Scope::detach,
        )
    }

    pub(crate) fn belongs_to(&self, collector: &GarbageCollector) -> bool {
        std::ptr::eq(self.collector, collector)
    }

    pub(crate) fn collector(&self) -> &'gc GarbageCollector {
        self.collector
    }

    /// Binding needs the exact construction batch, not the helper's current scope.
    pub(crate) fn publish_reference_owner(&self, metadata: &crate::metadata::ObjectMetadata) {
        for slot in &self.reference_owners {
            let _decision = slot.decision();
            if slot.owner.load(Ordering::Acquire) == metadata.ptr().0 {
                slot.bind_owner(Some(metadata.id));
            }
        }
    }
}

impl Drop for Detached<'_> {
    fn drop(&mut self) {
        for slot in self.reference_owners.drain(..) {
            let _decision = slot.decision();
            slot.unpublished_owners.fetch_sub(1, Ordering::Release);
        }
        for registration in self.registrations.drain(..) {
            self.collector.remove_detached_root(&registration);
        }
    }
}

impl Drop for Scope<'_, '_> {
    fn drop(&mut self) {
        CURRENT.set(self.roots.previous.get());
        for slot in self.roots.reference_owners.borrow_mut().drain(..) {
            let _decision = slot.decision();
            slot.unpublished_owners.fetch_sub(1, Ordering::Release);
        }
        let mut registrations = std::mem::take(&mut *self.roots.registrations.borrow_mut());
        if !registrations.is_empty() && !adopt_batch(self.roots.collector, &mut registrations) {
            // No matching enclosing constructor adopted the batch. Cleanup owns
            // these exact tokens and needs no further construction-TLS lookup.
            // Deletion barriers and slot reuse retain their existing admission.
            for id in registrations {
                self.roots.collector.remove_detached_root(&id);
            }
        }
    }
}

/// Reserve backing before the source registration is detached. The removal
/// returns with its publication guard released. Only a winning detach of a
/// registered object publishes this generation and transfers it into the scope.
pub(crate) fn with_removal_root<R>(
    collector: &GarbageCollector,
    remove: impl FnOnce(Option<&mut crate::roots::VacantRoot<'_>>) -> R,
) -> R {
    struct Capture<'a, 'gc> {
        registrations: &'a RefCell<Vec<Registration>>,
        root: crate::roots::VacantRoot<'gc>,
    }

    impl Drop for Capture<'_, '_> {
        fn drop(&mut self) {
            if let Some(key) = self.root.published_key() {
                // The registration itself protects the graph from publication
                // until scope cleanup, including if the deletion barrier unwinds.
                self.registrations
                    .borrow_mut()
                    .push(Registration::Tree(key));
            }
        }
    }

    with_matching_roots(collector, true, |scope| {
        let Some(roots) = scope else {
            return remove(None);
        };
        let mut capture = Capture {
            registrations: &roots.registrations,
            root: collector.reserve_tree_root(),
        };
        remove(Some(&mut capture.root))
    })
}

/// The last owning handle transfers its existing registration rather than
/// removing/reinserting it. The registration stays visible throughout the hook.
pub(crate) fn defer_owned(collector: &GarbageCollector, id: &Registration) -> bool {
    with_roots(collector, false, |roots| {
        roots.registrations.borrow_mut().push(id.clone());
    })
}

/// Move ownership once, including context/page owners. An empty destination can
/// reuse the complete vector backing instead of allocating and cloning each token.
/// Only synchronous scope exit adopts into TLS; detached cleanup never does.
fn adopt_batch(collector: &GarbageCollector, registrations: &mut Vec<Registration>) -> bool {
    with_roots(collector, false, |roots| {
        let mut destination = roots.registrations.borrow_mut();
        if destination.is_empty() {
            *destination = std::mem::take(registrations);
        } else {
            destination.append(registrations);
        }
    })
}

/// Prepare backing while the slot remains available to collectors. The TLS scope
/// is synchronous. Binding rechecks the target under phase and decision admission.
pub(crate) fn prepare_reference_owner<'gc>(
    collector: &'gc GarbageCollector,
    slot: &Arc<crate::weak::WeakSlot>,
) -> Option<crate::roots::VacantRoot<'gc>> {
    let mut needed = false;
    with_roots(collector, true, |roots| {
        let mut owners = roots.reference_owners.borrow_mut();
        if !owners.iter().any(|existing| Arc::ptr_eq(existing, slot)) {
            owners.reserve(1);
            roots.registrations.borrow_mut().reserve(1);
            needed = true;
        }
    });
    needed.then(|| collector.reserve_tree_root())
}

/// An unpublished Reference owner cannot yet participate in reference processing
/// or queue delivery. Root its eligible referent before publishing a pending
/// count, so collectors can skip this slot and finish while the hook is stalled.
/// The caller holds short publication admission and the slot's decision lock
/// (or has not registered the slot yet). Neither is held across user callbacks.
pub(crate) fn protect_reference_owner(
    collector: &GarbageCollector,
    slot: &Arc<crate::weak::WeakSlot>,
    reservation: Option<&mut crate::roots::VacantRoot<'_>>,
) {
    let Some(tree) = reservation else { return };
    with_roots(collector, true, |roots| {
        if roots
            .reference_owners
            .borrow()
            .iter()
            .any(|existing| Arc::ptr_eq(existing, slot))
        {
            return;
        }
        // set_owner holds the slot decision lock. Record ownership before
        // publishing the count so allocation unwind cannot strand a reservation.
        let pending = slot.unpublished_owners.load(Ordering::Acquire);
        assert!(
            pending < usize::MAX,
            "conditional owner publication count exhausted"
        );
        if let Some(id) = collector.retain_construction_referent(slot, tree) {
            roots
                .registrations
                .borrow_mut()
                .push(Registration::Tree(id));
        }
        roots.reference_owners.borrow_mut().push(slot.clone());
        slot.unpublished_owners
            .store(pending + 1, Ordering::Release);
    });
}

fn with_roots(
    collector: &GarbageCollector,
    construction_only: bool,
    operation: impl FnOnce(&Roots<'_>),
) -> bool {
    with_matching_roots(collector, construction_only, |roots| {
        roots.map(operation).is_some()
    })
}

fn with_matching_roots<R>(
    collector: &GarbageCollector,
    construction_only: bool,
    operation: impl FnOnce(Option<&Roots<'_>>) -> R,
) -> R {
    let mut current = CURRENT.get();
    while !current.is_null() {
        // SAFETY: enter borrows the stack Roots until Scope::drop restores the
        // previous pointer. Scopes cannot migrate, and nested scopes keep every
        // predecessor alive. This reference never escapes this synchronous call.
        let roots = unsafe { &*current.cast::<Roots<'_>>() };
        if std::ptr::eq(roots.collector, collector) && (!construction_only || roots.construction) {
            return operation(Some(roots));
        }
        current = roots.previous.get();
    }
    operation(None)
}

#[cfg(test)]
#[path = "construction_tests.rs"]
mod tests;
