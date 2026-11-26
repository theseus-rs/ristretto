//! Forced object/root collision audit; allocator accounting, not timing.
use ristretto_catalog_tagged_prototype::{
    Entered, Record, RecordError, audit,
    bounded::{
        BoundedCatalog, Budget, Capacities, Footprint, GuardedCommand, Participant, Progress,
        RootHandle,
    },
};
#[path = "support/counting.rs"]
mod counting;
struct Inline<const N: usize>([u8; N]);
impl<const N: usize> Default for Inline<N> {
    fn default() -> Self {
        Self([0; N])
    }
}
impl<const N: usize> Record for Inline<N> {
    type Context = ();
    type Guard<'a> = ();
    fn enter<'a>(&'a self, context: &'a ()) -> Result<Entered<'a, ()>, RecordError> {
        assert_eq!(self.0[0], 0);
        ().enter(context)
    }
}
#[repr(align(64))]
#[derive(Default)]
struct Aligned(Inline<65>);
impl Record for Aligned {
    type Context = ();
    type Guard<'a> = ();
    fn enter<'a>(&'a self, context: &'a ()) -> Result<Entered<'a, ()>, RecordError> {
        self.0.enter(context)
    }
}
fn finish<R: Record<Context = ()>>(p: &mut Participant<R, GuardedCommand>) -> RootHandle<R> {
    for _ in 0..4096 {
        match p.poll_in(&()) {
            Progress::Pending => {}
            Progress::RootCreated(_) => return p.take_root().unwrap(),
            other => panic!("unexpected result {other:?}"),
        }
    }
    panic!("request did not settle");
}
fn run<R: Record<Context = ()> + Default>(name: &str) {
    for registry in [2, 64, 1024, 16384] {
        for mutators in [2, 7] {
            for roots in [2, 64] {
                let cap = Capacities::derive(registry, mutators, 1)
                    .unwrap()
                    .with_roots(roots)
                    .unwrap();
                let footprint = Footprint::for_records::<R>(cap).unwrap();
                let budget = Budget::new(footprint.total());
                let before = audit::snapshot();
                let catalog = BoundedCatalog::with_records_in(cap, &budget).unwrap();
                let setup = audit::snapshot();
                assert_eq!(
                    setup.requested - before.requested,
                    footprint.backing() as u64
                );
                let mut first = catalog.register_record_mutator().unwrap();
                let mut second = catalog.register_record_mutator().unwrap();
                first.submit_rooted_record(1, 1, 8, R::default()).unwrap();
                second.submit_rooted_record_any(16, R::default()).unwrap();
                for p in [&mut first, &mut second] {
                    for _ in 0..3 {
                        assert_eq!(p.poll_in(&()), Progress::Pending);
                    }
                }
                assert_eq!(first.poll_in(&()), Progress::Pending);
                assert_eq!(second.poll_in(&()), Progress::Pending); // lost object CAS
                assert_eq!(second.poll_in(&()), Progress::Pending); // next object slot
                for p in [&mut first, &mut second] {
                    assert_eq!(p.poll_in(&()), Progress::Pending);
                    assert_eq!(p.poll_in(&()), Progress::Pending);
                }
                assert_eq!(first.poll_in(&()), Progress::Pending);
                assert_eq!(second.poll_in(&()), Progress::Pending); // lost root CAS
                assert_eq!(second.poll_in(&()), Progress::Pending); // next root slot
                let a = finish(&mut first);
                let b = finish(&mut second);
                assert_eq!(a.key().slot(), 1);
                assert_eq!(b.key().slot(), if roots == 2 { 0 } else { 2 });
                assert_eq!(b.observe().target.slot(), if registry == 2 { 0 } else { 2 });
                let held = audit::snapshot();
                assert_eq!(held.requests, setup.requests, "retry lifecycle allocated");
                assert_eq!(held.frees, setup.frees, "retry lifecycle freed backing");
                drop((catalog, first, second, a));
                assert_eq!(budget.used(), footprint.total());
                assert_eq!(audit::snapshot().frees, setup.frees);
                drop(b);
                let end = audit::snapshot();
                assert_eq!(budget.used(), 0);
                assert_eq!(end.refunds - before.refunds, 1);
                assert_eq!(end.refunded - before.refunded, footprint.total() as u64);
                assert_eq!(end.released - before.released, footprint.backing() as u64);
                assert_eq!(end.frees - before.frees, setup.requests - before.requests);
                assert!(end.last_refund > end.last_free);
                println!(
                    "{name},{registry},{mutators},1,{roots},{},{},{},{},{},0,true",
                    footprint.backing(),
                    footprint.participants(),
                    footprint.root_tokens(),
                    footprint.total(),
                    setup.requests - before.requests
                );
            }
        }
    }
}
fn main() {
    println!(
        "record,registry,mutators,helpers,roots,backing,participants,root_tokens,total,requests,lifecycle_requests,refund_after_last_free"
    );
    run::<Inline<1>>("byte");
    run::<Inline<512>>("large");
    run::<Aligned>("aligned");
}
