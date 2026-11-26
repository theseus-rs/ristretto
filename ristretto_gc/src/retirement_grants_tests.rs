use super::*;
use crate::Configuration;
use std::future::Future;
use std::task::{Context, Wake, Waker};

#[cfg(all(not(target_family = "wasm"), not(miri)))]
mod hooks {
    use std::{cell::Cell, sync::mpsc, time::Duration};

    type Gate = (mpsc::Sender<()>, mpsc::Receiver<()>);
    thread_local! {
        pub(super) static LINK: Cell<Option<Gate>> = const { Cell::new(None) };
        pub(super) static BIND: Cell<Option<Gate>> = const { Cell::new(None) };
    }

    pub(super) fn park(hook: &'static std::thread::LocalKey<Cell<Option<Gate>>>) {
        if let Some((entered, release)) = hook.take() {
            entered.send(()).expect("announce grant test hook");
            release
                .recv_timeout(Duration::from_secs(5))
                .expect("release grant test hook");
        }
    }
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
pub(super) fn after_link() {
    hooks::park(&hooks::LINK);
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
pub(super) fn before_binding_commit() {
    hooks::park(&hooks::BIND);
}

// Stand in for the exclusively owned, already charged allocation. The
// native integration fixture separately exercises the actual destructor.
fn fixture() -> crate::Result<Arc<GarbageCollector>> {
    let gc = GarbageCollector::with_config(Configuration::builder().max_heap_bytes(32).build());
    gc.reserve_allocation(32)?;
    Ok(gc)
}

fn capture(gc: &Arc<GarbageCollector>) -> Selection {
    gc.grant_registry.selection(Arc::downgrade(gc), 32)
}

#[test]
#[expect(
    clippy::panic_in_result_fn,
    reason = "verify ledger transitions alongside fallible allocations"
)]
fn cancellations_and_consumption_refund_each_share_once() -> crate::Result<()> {
    let gc = fixture()?;
    let early = gc.grant_registry.register(&gc, 8)?;
    let late = gc.grant_registry.register(&gc, 16)?;
    let spender = gc.grant_registry.register(&gc, 8)?;
    let mut selected = capture(&gc);
    assert!(selected.advance(&mut 31));
    selected.mark_private();
    assert!(spender.take().is_none());
    drop(early);
    assert_eq!(gc.managed_bytes(), 32);
    let paid = selected.pay();
    assert_eq!(paid, 24);
    gc.cancel_allocation(32 - paid);
    drop(late);
    assert_eq!(gc.managed_bytes(), 8);
    let credit = spender.take().ok_or(crate::Error::OutOfMemory)?;
    assert!(spender.take().is_none());
    drop(spender);
    assert_eq!(gc.managed_bytes(), 8);
    drop(credit);
    assert_eq!(gc.managed_bytes(), 0);
    Ok(())
}

#[derive(Default)]
struct Runnable(AtomicBool);
impl Wake for Runnable {
    fn wake(self: Arc<Self>) {
        self.0.store(true, Ordering::Release);
    }
}

#[test]
#[expect(
    clippy::panic_in_result_fn,
    reason = "check poll and accounting results on the same collector"
)]
fn covered_workerless_waiter_assists_until_the_donor_becomes_private() -> crate::Result<()> {
    let gc = fixture()?;
    let first = gc.grant_registry.register(&gc, 8)?;
    let second = gc.grant_registry.register(&gc, 8)?;
    let mut selected = capture(&gc);
    assert!(!selected.advance(&mut 0));
    let mut opportunity = gc.observe_space(8)?;
    let mut wait = std::pin::pin!(
        crate::recovery::CollectionWait::for_space(&gc, u64::MAX, &mut opportunity).with_grant(
            &first.request,
            true,
            false
        )
    );
    let runnable = Arc::new(Runnable::default());
    let waker = Waker::from(runnable.clone());
    let mut cx = Context::from_waker(&waker);
    assert!(wait.as_mut().poll(&mut cx).is_pending());
    assert!(runnable.0.swap(false, Ordering::AcqRel));
    assert!(selected.advance(&mut 0));
    selected.mark_private();
    assert!(wait.as_mut().poll(&mut cx).is_pending());
    assert!(!runnable.0.load(Ordering::Acquire));
    drop(selected);
    gc.recovery.notify_progress(&gc);
    assert!(wait.as_mut().poll(&mut cx).is_ready());
    assert!(first.take().is_none());
    assert!(first.finish_uncovered());
    drop(second);
    assert_eq!(gc.managed_bytes(), 32);
    gc.cancel_allocation(32);
    assert_eq!(gc.managed_bytes(), 0);
    Ok(())
}
#[cfg(all(not(target_family = "wasm"), not(miri)))]
#[test]
#[expect(clippy::panic_in_result_fn, reason = "assert each forced handoff")]
fn a_peer_commits_the_parked_enrollment_before_taking_the_next_slot() -> crate::Result<()> {
    use std::{sync::mpsc, time::Duration};
    let gc = fixture()?;
    std::thread::scope(|scope| -> crate::Result<()> {
        let (entered, parked) = mpsc::channel();
        let (resume, release) = mpsc::channel();
        let gc_ref = &gc;
        let enroller = scope.spawn(move || {
            hooks::LINK.set(Some((entered, release)));
            gc_ref.grant_registry.register(gc_ref, 8)
        });
        parked
            .recv_timeout(Duration::from_secs(5))
            .expect("first slot linked");
        let mut earlier = capture(&gc);
        let peer = gc.grant_registry.register(&gc, 32)?;
        let mut later = capture(&gc);
        drop(peer);
        let late = gc.grant_registry.register(&gc, 8)?;
        assert!(earlier.advance(&mut 31));
        assert_eq!(earlier.pay(), 0);
        assert!(later.advance(&mut 31));
        let paid = later.pay();
        assert_eq!(paid, 8);
        assert!(late.finish_uncovered());
        gc.cancel_allocation(32 - paid);
        resume.send(()).expect("resume enroller");
        let demand = enroller.join().expect("enrollment returned")?;
        drop(demand.take().ok_or(crate::Error::OutOfMemory)?);
        assert_eq!(gc.managed_bytes(), 0);
        Ok(())
    })
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
#[test]
#[expect(
    clippy::panic_in_result_fn,
    clippy::unwrap_in_result,
    reason = "force cancellation between the two assignment transitions"
)]
fn cancellation_of_a_private_assignment_leaves_its_share_with_the_donor() -> crate::Result<()> {
    use std::{sync::mpsc, time::Duration};
    let gc = fixture()?;
    let first = gc.grant_registry.register(&gc, 8)?;
    let second = gc.grant_registry.register(&gc, 32)?;
    let mut selection = capture(&gc);
    let (entered, parked) = mpsc::channel();
    let (resume, release) = mpsc::channel();
    let worker = std::thread::spawn(move || {
        hooks::BIND.set(Some((entered, release)));
        assert!(selection.advance(&mut 31));
        selection.pay()
    });
    parked
        .recv_timeout(Duration::from_secs(5))
        .expect("assignment started");
    drop(first);
    assert_eq!(gc.managed_bytes(), 32);
    resume.send(()).expect("finish assignment");
    let paid = worker.join().expect("selector returned");
    assert_eq!(paid, 32);
    gc.cancel_allocation(32 - paid);
    drop(second.take().ok_or(crate::Error::OutOfMemory)?);
    assert_eq!(gc.managed_bytes(), 0);
    Ok(())
}

#[test]
#[expect(
    clippy::panic_in_result_fn,
    reason = "check retirement of the final partial page"
)]
fn exhausted_positions_leave_no_resurrected_page_or_demand() -> crate::Result<()> {
    let gc = fixture()?;
    let registry = &gc.grant_registry;
    let base = (u64::MAX / PAGE_WIDTH as u64) * PAGE_WIDTH as u64;
    registry.published.store(base, Ordering::Release);
    for _ in base..u64::MAX {
        drop(registry.register(&gc, 1)?);
    }
    assert!(matches!(
        registry.register(&gc, 1),
        Err(crate::Error::OutOfMemory)
    ));
    assert!(registry.page(base)?.is_none());
    assert_eq!(registry.pages.load().len(), 0);
    assert_eq!(registry.active.load(Ordering::Acquire), 0);
    gc.cancel_allocation(32);
    Ok(())
}

#[test]
#[expect(
    clippy::panic_in_result_fn,
    reason = "check grant authority through several abandoned donors"
)]
fn repeated_abandonment_never_reuses_a_previous_donors_receipt() -> crate::Result<()> {
    let gc = fixture()?;
    let request = gc.grant_registry.register(&gc, 32)?;
    let mut retired = Vec::new();
    for _ in 0..3 {
        let mut selection = capture(&gc);
        assert!(selection.advance(&mut 0));
        retired.push(selection.donor.clone().ok_or(crate::Error::OutOfMemory)?);
        drop(selection);
        assert!(request.take().is_none());
        assert!(request.request.waiting());
    }
    let mut final_donor = capture(&gc);
    assert!(final_donor.advance(&mut 0));
    assert_eq!(final_donor.pay(), 32);
    assert!(
        retired
            .iter()
            .all(|receipt| receipt.canceled.load(Ordering::Acquire) == ABANDONED)
    );
    drop(request.take().ok_or(crate::Error::OutOfMemory)?);
    assert_eq!(gc.managed_bytes(), 0);
    Ok(())
}
