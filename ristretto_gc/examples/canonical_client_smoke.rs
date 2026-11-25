//! Public canonical API on a bare host: no clock, threads, task executor or WASI.
#![expect(
    clippy::panic_in_result_fn,
    reason = "smoke entry point checks exact ownership"
)]
use ristretto_gc::GarbageCollector;
use ristretto_gc::canonical::{Budget, Configuration, Error, Helper, Root, Runtime};
use std::task::Poll;

type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;
fn complete<T>(
    helper: &mut Helper<'_>,
    mut poll: impl FnMut() -> std::result::Result<Poll<T>, Error>,
) -> Result<T> {
    for _ in 0..100_000 {
        if let Poll::Ready(value) = poll()? {
            return Ok(value);
        }
        helper.poll()?;
    }
    Err("canonical client poll watchdog".into())
}
fn run() -> Result {
    let gc = GarbageCollector::new();
    let config = Configuration {
        objects: 2,
        roots: 3,
        mutators: 1,
        helpers: 1,
        edges_per_object: 1,
    };
    let charge = config.charged_bytes()?;
    let budget = Budget::new(charge);
    let runtime = Runtime::new(&gc, config, &budget)?;
    let mut worker = runtime.mutator()?;
    let mut helper = runtime.helper()?;
    let a = {
        let mut allocation = worker.allocate(11_u64)?;
        complete(&mut helper, || allocation.poll())?
    };
    let b = {
        let mut allocation = worker.allocate(22_u64)?;
        complete(&mut helper, || allocation.poll())?
    };
    {
        let mut write = worker.write(&a, 0, Some(&b))?;
        complete(&mut helper, || write.poll())?;
    }
    {
        let mut write = worker.write(&b, 0, Some(&a))?;
        complete(&mut helper, || write.poll())?;
    }
    let child: Root<u64> = {
        let mut read = worker.child::<u64, u64>(&a, 0)?;
        complete(&mut helper, || read.poll())?.ok_or("missing cycle child")?
    };
    assert!(child.same_object(&b));
    assert_eq!(*runtime.borrow(&child)?, 22);
    let alias = child.clone();
    drop((a, b, child));
    for _ in 0..1_000 {
        helper.poll()?;
    }
    assert_eq!(*runtime.borrow(&alias)?, 22);
    drop(alias);
    for _ in 0..100_000 {
        helper.poll()?;
        if gc.managed_bytes() == 0 {
            break;
        }
    }
    assert_eq!(gc.managed_bytes(), 0);
    assert_eq!(budget.used(), charge);
    drop((worker, helper, runtime));
    assert_eq!(budget.used(), 0);
    Ok(())
}

/// Returns 1 after checking cyclic reachability and full payload/credit teardown.
/// # Panics
/// Traps if an operation or ownership invariant fails.
#[expect(
    unsafe_code,
    reason = "export named clock-free WebAssembly entry point"
)]
#[unsafe(no_mangle)]
pub extern "C" fn canonical_client_smoke() -> u32 {
    let result = run();
    assert!(result.is_ok(), "canonical client smoke failed: {result:?}");
    1
}
