//! A demand has one stable first registration. Install it locally until a
//! real offer, ticket or settlement calls for ordered wake selection.
use super::{Entry, GarbageCollector, GrantWatch, Ordering, Ready, Recovery, WaiterId, Waker};
use crate::retirement_grants::Request;
use arc_swap::ArcSwapOption;
use std::sync::atomic::{AtomicBool, AtomicU8};
use std::sync::{Arc, Mutex, OnceLock};
use std::task::RawWakerVTable;

const DIRECT: u8 = 0;
const MOVING: u8 = 1;
const INDEXED: u8 = 2;
const CLOSED: u8 = 3;

// Registration on a given future has one writer. Only the transition out
// of FIRST owns the inline waker; a parked writer cannot hold up a wake.
const FIRST: u8 = 0;
const REPLACED: u8 = 1;
const TAKEN: u8 = 2;

#[derive(Clone, Copy)]
pub(super) struct WakeTarget {
    address: usize,
    vtable: &'static RawWakerVTable,
}

impl WakeTarget {
    pub(super) fn of(owned: &Waker) -> Self {
        Self {
            address: owned.data().addr(),
            vtable: owned.vtable(),
        }
    }

    fn matches(self, waker: &Waker) -> bool {
        self.address == waker.data().addr() && std::ptr::eq(self.vtable, waker.vtable())
    }
}

#[derive(Clone, Copy)]
pub(crate) struct Scan {
    pub(crate) available: usize,
    pub(crate) ticket: u64,
    pub(crate) grants: u64,
    pub(crate) all: bool,
}

#[derive(Debug)]
pub(crate) struct Arm {
    pub(crate) ticket: u64,
    pub(crate) bytes: usize,
    pub(super) id: WaiterId,
    settlement_only: bool,
    receipt: Option<Arc<AtomicBool>>,
    state: AtomicU8,
    destination: AtomicU8,
    first: Mutex<Option<Waker>>,
    replacement: OnceLock<ArcSwapOption<Waker>>,
}

impl Arm {
    pub(super) fn new(
        id: WaiterId,
        ticket: u64,
        bytes: usize,
        settlement_only: bool,
        receipt: Option<Arc<AtomicBool>>,
        waker: Waker,
    ) -> Self {
        Self {
            id,
            ticket,
            bytes,
            settlement_only,
            receipt,
            state: AtomicU8::new(DIRECT),
            destination: AtomicU8::new(FIRST),
            first: Mutex::new(Some(waker)),
            replacement: OnceLock::new(),
        }
    }

    fn take_first(&self) -> Option<Waker> {
        self.first
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .take()
    }

    pub(super) fn refresh(&self, target: &mut WakeTarget, incoming: &Waker) -> bool {
        if target.matches(incoming) {
            return self
                .state
                .compare_exchange(DIRECT, DIRECT, Ordering::AcqRel, Ordering::Acquire)
                .is_ok();
        }
        if self.state.load(Ordering::Acquire) != DIRECT {
            return false;
        }
        let next = incoming.clone();
        // A custom RawWaker clone can use a different data address or vtable.
        let next_target = WakeTarget::of(&next);
        if let Some(slot) = self.replacement.get() {
            let previous = slot.load();
            if previous.is_none() {
                return false;
            }
            #[cfg(all(test, not(target_family = "wasm")))]
            super::wake_progress_tests::before_destination_commit();
            // This future is the sole writer. Only terminal delivery can
            // replace the sampled callback concurrently, and it stores None.
            if slot
                .compare_and_swap(&*previous, Some(Arc::new(next)))
                .is_none()
            {
                return false;
            }
        } else {
            let slot = self
                .replacement
                .get_or_init(|| ArcSwapOption::from(Some(Arc::new(next))));
            #[cfg(all(test, not(target_family = "wasm")))]
            super::wake_progress_tests::before_destination_commit();
            if self
                .destination
                .compare_exchange(FIRST, REPLACED, Ordering::AcqRel, Ordering::Acquire)
                .is_err()
            {
                slot.swap(None);
                return false;
            }
            drop(self.take_first());
        }
        *target = next_target;
        true
    }

    fn prepare(&self, request: &Request, gc: &GarbageCollector, all: bool) -> bool {
        let state = self.state.load(Ordering::Acquire);
        if state == MOVING {
            return true;
        }
        if state != DIRECT
            || !(all
                || self.ticket <= gc.completed_full_ticket()
                || self.bytes <= gc.allocation_headroom()
                || request.paid()
                || self.settlement_only && request.waiting())
        {
            return false;
        }
        // The hint precedes the transition, so a helper can finish even if
        // the original offer is stolen while its publisher is parked.
        request.mark_arm_ready();
        matches!(
            self.state
                .compare_exchange(DIRECT, MOVING, Ordering::AcqRel, Ordering::Acquire),
            Ok(_) | Err(MOVING)
        )
    }

    pub(super) fn indexed(&self, request: &Request) {
        let _ = self
            .state
            .compare_exchange(MOVING, INDEXED, Ordering::AcqRel, Ordering::Acquire);
        request.clear_arm();
    }

    pub(super) fn close(&self, recovery: &Recovery) -> (Option<Waker>, bool) {
        let previous = self.state.swap(CLOSED, Ordering::AcqRel);
        if previous == CLOSED {
            return (None, false);
        }
        recovery.armed.fetch_sub(1, Ordering::AcqRel);
        let callback = match self.destination.swap(TAKEN, Ordering::AcqRel) {
            FIRST => self.take_first(),
            REPLACED => self
                .replacement
                .get()
                .and_then(|slot| slot.swap(None))
                .map(Arc::unwrap_or_clone),
            _ => None,
        };
        (callback, previous != DIRECT)
    }
}

impl Recovery {
    pub(super) fn materialize(&self, gc: &GarbageCollector, all: bool) {
        if self.armed.fetch_add(0, Ordering::AcqRel) == 0 {
            return;
        }
        let scan = Scan {
            available: gc.allocation_headroom(),
            ticket: gc.completed_full_ticket(),
            grants: self.grant_generation.load(Ordering::Acquire),
            all,
        };
        let mut moving = Vec::new();
        gc.grant_registry.visit_arms(scan, |request, arm| {
            if arm.prepare(request, gc, all) {
                let entry = Arc::new(Entry {
                    bytes: Some(arm.bytes),
                    grant: Some(GrantWatch {
                        request: request.clone(),
                        settlement_only: arm.settlement_only,
                    }),
                    callback: super::index::Callback::first(request.clone()),
                    space_seen: arm.receipt.clone(),
                });
                moving.push((request.clone(), entry));
            }
        });
        if moving.is_empty() {
            return;
        }
        self.used.store(true, Ordering::Release);
        self.update(|w| {
            for (request, entry) in &moving {
                let Some(arm) = request.arm() else {
                    continue;
                };
                let key = (arm.ticket, arm.id);
                // Cancellation forces a root CAS even if the old entry was
                // absent. An obsolete publisher must retry this exact check.
                if arm.state.load(Ordering::Acquire) != MOVING
                    || w.entries.get(key).is_some()
                    || w.ready.get(key).is_some()
                {
                    continue;
                }
                w.first_count += 1;
                if all || entry.grant.as_ref().is_some_and(GrantWatch::ready) {
                    w.ready.insert(
                        key,
                        Ready {
                            entry: entry.clone(),
                            offered: false,
                        },
                    );
                } else {
                    w.entries.insert(key, entry.clone());
                    w.space.insert((arm.bytes as u64, arm.id), arm.ticket);
                    w.recheck = true;
                }
            }
        });
        for (request, _) in moving {
            if let Some(arm) = request.arm() {
                arm.indexed(&request);
            }
        }
    }
}
