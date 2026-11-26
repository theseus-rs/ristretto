use super::*;

type Result<T> = std::result::Result<T, Error>;
use crate::canonical::ClassFile;
use std::cell::Cell;
use std::future::Future;
use std::task::{Context, Waker};

#[test]
fn cancelling_the_outer_heap_future_drops_static_roots_before_catalog_teardown() -> Result<()> {
    let program = Program::from_classfiles([
        ClassFile::from_bytes(include_bytes!(
            "../../ristretto_vm/tests/fixtures/canonical/CanonicalStaticRootsSmall.class"
        ))?,
        ClassFile::from_bytes(include_bytes!(
            "../../ristretto_vm/tests/fixtures/canonical/CanonicalStaticRootsSmall$Node.class"
        ))?,
    ])?;
    let configuration = Configuration {
        objects: 32,
        roots: 16,
        mutators: 1,
        helpers: 1,
        edges_per_object: 8,
    };
    for finish_initializer in [false, true] {
        let budget = Budget::new(configuration.charged_bytes()?);
        let collector = GarbageCollector::new();
        let completed = Cell::new(false);
        let mut future = Box::pin(async {
            let heap = JavaHeap {
                program: &program,
                statics: States::heap(&program)?,
                runtime: Runtime::new(&collector, configuration, &budget)?,
                collector: &collector,
            };
            let mut thread = heap.attach(Limits::default())?;
            assert_eq!(
                thread
                    .invoke("CanonicalStaticRootsSmall", "prepared", 0)
                    .await?
                    .result,
                19
            );
            completed.set(true);
            std::future::pending::<()>().await;
            Ok::<(), Error>(())
        });
        let mut context = Context::from_waker(Waker::noop());
        for _ in 0..20_000 {
            assert!(future.as_mut().poll(&mut context).is_pending());
            if !finish_initializer || completed.get() {
                break;
            }
        }
        assert_eq!(completed.get(), finish_initializer);
        assert!(collector.managed_bytes() > 0);
        assert!(budget.used() > 0);
        drop(future);
        assert_eq!(collector.managed_bytes(), 0);
        assert_eq!(budget.used(), 0);
    }
    Ok(())
}
