use super::*;
#[cfg(not(target_family = "wasm"))]
use std::sync::mpsc;
use std::task::Wake;
#[cfg(not(target_family = "wasm"))]
use std::time::Duration;

#[cfg(not(target_family = "wasm"))]
struct Parked {
    entered: mpsc::Sender<()>,
    release: Mutex<mpsc::Receiver<()>>,
}
#[cfg(not(target_family = "wasm"))]
impl Wake for Parked {
    fn wake(self: Arc<Self>) {
        assert!(self.entered.send(()).is_ok(), "test receiver disconnected");
        assert!(
            lock(&self.release)
                .recv_timeout(Duration::from_secs(10))
                .is_ok(),
            "callback release timed out"
        );
    }
}

#[cfg(not(target_family = "wasm"))]
struct Notice(AtomicUsize);
#[cfg(not(target_family = "wasm"))]
impl Wake for Notice {
    fn wake(self: Arc<Self>) {
        self.0.fetch_add(1, Ordering::SeqCst);
    }
}

#[cfg(not(target_family = "wasm"))]
thread_local! {
    static AFTER_PHYSICAL_PROBE: std::cell::Cell<Option<Box<dyn FnOnce()>>> =
        const { std::cell::Cell::new(None) };
}

#[cfg(not(target_family = "wasm"))]
pub(super) fn after_physical_probe() {
    if let Some(hook) = AFTER_PHYSICAL_PROBE.take() {
        hook();
    }
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn a_paused_physical_probe_leaves_logical_wakes_and_cancellation_available() {
    let gc =
        GarbageCollector::with_config(crate::Configuration::builder().compact_heap(true).build());
    let logical = Arc::new(Notice(AtomicUsize::new(0)));
    let physical = Arc::new(Notice(AtomicUsize::new(0)));
    gc.recovery
        .register(1, None, Some(1), &Waker::from(logical.clone()), None, None)
        .unwrap();
    let layout = Layout::new::<[u8; 16]>();
    assert!(gc.region_heap().unwrap().can_allocate(layout));
    let id = gc
        .recovery
        .register(
            1,
            None,
            None,
            &Waker::from(physical.clone()),
            None,
            Some(layout),
        )
        .unwrap();
    let (entered, parked) = mpsc::channel();
    let (release, resume) = mpsc::channel();
    let (done, delivered) = mpsc::channel();
    std::thread::scope(|scope| {
        let first = gc.clone();
        scope.spawn(move || {
            AFTER_PHYSICAL_PROBE.set(Some(Box::new(move || {
                entered.send(()).unwrap();
                resume.recv_timeout(Duration::from_secs(10)).unwrap();
            })));
            first.recovery.notify_progress(&first);
        });
        parked.recv_timeout(Duration::from_secs(10)).unwrap();
        scope.spawn(|| {
            gc.recovery.remove(1, Some(id));
            gc.recovery.notify_progress(&gc);
            done.send(logical.0.load(Ordering::SeqCst)).unwrap();
        });
        let before_resume = delivered.recv_timeout(Duration::from_secs(2));
        release.send(()).unwrap();
        assert_eq!(
            before_resume,
            Ok(1),
            "a paused physical probe held up an unrelated wake"
        );
    });
    assert_eq!(
        physical.0.load(Ordering::SeqCst),
        0,
        "stale sample woke a cancelled request"
    );
    assert_eq!(gc.recovery.waiter_count(), 0);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn another_helper_delivers_wakes_while_a_callback_is_parked() {
    for worker_exit in [false, true] {
        let gc = GarbageCollector::new();
        let (entered, parked) = mpsc::channel();
        let (release, resume) = mpsc::channel();
        let blocker = Waker::from(Arc::new(Parked {
            entered,
            release: Mutex::new(resume),
        }));
        let waiting = Arc::new(Notice(AtomicUsize::new(0)));
        let cancelled = Arc::new(Notice(AtomicUsize::new(0)));
        let register = |waker: &Waker| {
            gc.recovery
                .register(1, None, Some(1), waker, None, None)
                .unwrap()
        };
        register(&blocker);
        register(&Waker::from(waiting.clone()));
        let cancelled_id = register(&Waker::from(cancelled.clone()));
        std::thread::scope(|scope| {
            scope.spawn(|| {
                if worker_exit {
                    gc.recovery.notify_all();
                } else {
                    gc.recovery.notify_progress(&gc);
                }
            });
            parked.recv_timeout(Duration::from_secs(10)).unwrap();
            gc.recovery.remove(1, Some(cancelled_id));
            gc.recovery.notify_progress(&gc);
            let delivered = waiting.0.load(Ordering::SeqCst);
            release.send(()).unwrap();
            assert_eq!(delivered, 1, "another helper must reach ready waiters");
        });
        assert_eq!(waiting.0.load(Ordering::SeqCst), 1);
        assert_eq!(cancelled.0.load(Ordering::SeqCst), 0);
        assert_eq!(gc.recovery.waiter_count(), 0);
    }
}

#[test]
fn synchronous_reentry_drains_the_ready_set_without_recursive_wakes() {
    struct Reenter {
        gc: Arc<GarbageCollector>,
        inside: AtomicBool,
        count: AtomicUsize,
    }
    impl Wake for Reenter {
        fn wake(self: Arc<Self>) {
            assert!(!self.inside.swap(true, Ordering::SeqCst));
            self.count.fetch_add(1, Ordering::SeqCst);
            self.gc.recovery.notify_progress(&self.gc);
            self.inside.store(false, Ordering::SeqCst);
        }
    }
    let gc = GarbageCollector::new();
    let owner = Arc::new(Reenter {
        gc: gc.clone(),
        inside: AtomicBool::new(false),
        count: AtomicUsize::new(0),
    });
    let waker = Waker::from(owner.clone());
    for _ in 0..128 {
        gc.recovery
            .register(1, None, Some(1), &waker, None, None)
            .unwrap();
    }
    gc.recovery.notify_progress(&gc);
    assert_eq!(owner.count.load(Ordering::SeqCst), 128);
    assert_eq!(gc.recovery.waiter_count(), 0);
}
