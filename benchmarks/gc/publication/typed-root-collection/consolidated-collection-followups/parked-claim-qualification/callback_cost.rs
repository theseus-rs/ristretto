//! Time a complete native worker-driven collection through physical refund.
use ristretto_gc::{Configuration, Finalize, GarbageCollector, Gc, Result, Trace};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::task::{Context, Poll, Wake, Waker};
use std::time::{Duration, Instant};

struct Payload(Arc<AtomicUsize>);
// SAFETY: the payload contains only a host-side counter.
#[expect(unsafe_code, reason = "the fixture declares an edge-free trace")]
unsafe impl Trace for Payload {
    fn trace(&self, _: &GarbageCollector) {}
}
impl Finalize for Payload {
    fn finalize(&self) { self.0.fetch_add(1, Ordering::Relaxed); }
}
struct Executor(std::thread::Thread);
impl Wake for Executor {
    fn wake(self: Arc<Self>) { self.0.unpark(); }
    fn wake_by_ref(self: &Arc<Self>) { self.0.unpark(); }
}

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let count: usize = args.next().unwrap().parse().unwrap();
    let workers: usize = args.next().unwrap().parse().unwrap();
    let gc = GarbageCollector::with_config(Configuration::builder()
        .threads(workers).incremental_step_size(64).max_pause_time_us(0)
        .allocation_threshold(usize::MAX).build());
    let ran = Arc::new(AtomicUsize::new(0));
    let waker = Waker::from(Arc::new(Executor(std::thread::current())));
    let mut ctx = Context::from_waker(&waker);
    let mut samples = Vec::new();
    gc.start();
    for round in 0..19 {
        let roots: Vec<_>=(0..count).map(|_| Gc::new_with_finalizer(&gc, Payload(ran.clone()))).collect();
        drop(roots);
        let start = Instant::now();
        let deadline = start + Duration::from_secs(10);
        let mut wait = std::pin::pin!(gc.collect_full_async());
        loop {
            if let Poll::Ready(r) = wait.as_mut().poll(&mut ctx) { r?; break; }
            std::thread::park_timeout(deadline.checked_duration_since(Instant::now()).unwrap());
        }
        while gc.managed_bytes()!=0 {
            assert!(Instant::now()<deadline);
            std::thread::yield_now();
        }
        let elapsed = start.elapsed().as_nanos();
        assert_eq!(ran.load(Ordering::Relaxed),(round+1)*count);
        if round>=3 { samples.push(elapsed); }
    }
    gc.stop()?;
    println!("{samples:?}");
    Ok(())
}
