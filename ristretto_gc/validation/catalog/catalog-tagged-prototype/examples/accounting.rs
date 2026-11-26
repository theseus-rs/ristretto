//! Separate single-thread allocator/credit audit; never a timing benchmark.
use ristretto_catalog_tagged_prototype::{
    Membership, RETIREMENT_FLOOR, TraceRetention, audit,
    bounded::{
        BoundedCatalog, Budget, Capacities, Command, ConfigError, Footprint, Participant, Progress,
    },
};
#[path = "support/counting.rs"]
mod counting;
fn run(helper: &mut Participant, command: Command) -> Progress {
    helper.submit(command).unwrap();
    for _ in 0..4096 {
        let result = helper.poll();
        if result != Progress::Pending {
            return result;
        }
    }
    panic!("tracing audit request did not complete");
}
fn trace_cycle(helper: &mut Participant) {
    let initial = helper.phase().unwrap();
    assert_eq!(initial.domains[0].floor, 0);
    let Progress::Created(key) = run(
        helper,
        Command::Create {
            slot: 0,
            address: 8,
        },
    ) else {
        panic!()
    };
    assert_eq!(run(helper, Command::TraceInsert(key)), Progress::Committed);
    for retention in [TraceRetention::Phantom, TraceRetention::Strong] {
        assert_eq!(
            run(
                helper,
                Command::Shade {
                    key,
                    domain: 0,
                    retention
                }
            ),
            Progress::Committed
        );
    }
    let Membership::Tracing(marks) = helper.observe(key).unwrap().membership else {
        panic!()
    };
    assert_eq!(marks[0].retention, Some(TraceRetention::Strong));
    assert_ne!(marks[0].ticket, 0);
    assert_eq!(helper.phase().unwrap().domains[0].pending, 1);
    assert_eq!(
        run(
            helper,
            Command::TraceAcknowledge {
                key,
                domain: 0,
                ticket: marks[0].ticket
            }
        ),
        Progress::Committed
    );
    // Close the marked epoch, then retire this unmarked record in the next one.
    for _ in 0..(2 * RETIREMENT_FLOOR + 1) {
        assert_eq!(advance_cycle(helper, 0), Progress::Committed);
    }
    assert_eq!(
        run(helper, Command::TraceRetire { key, domain: 0 }),
        Progress::Committed
    );
    assert!(helper.release_retired(key).unwrap());
    assert_eq!(advance_cycle(helper, 0), Progress::Committed);
    assert!(helper.discover(0).unwrap().is_none());
    let final_phase = helper.phase().unwrap();
    assert_eq!(final_phase.domains[0].epoch, initial.domains[0].epoch + 2);
    assert_eq!(final_phase.domains[0].floor, 0);
    assert_eq!(final_phase.domains.map(|domain| domain.pending), [0, 0]);
}
fn main() {
    println!(
        "registry,mutators,helpers,backing,participants,total,requests,refund_after_last_free"
    );
    let tracing = std::env::args().any(|arg| arg == "--trace");
    let mut traced_cycles = 0;
    let registries: &[usize] = if std::env::args().any(|arg| arg == "--quick") {
        &[1, 64]
    } else {
        &[1, 64, 1024, 16384]
    };
    for &registry in registries {
        for mutators in [0, 1, 7] {
            let capacities = Capacities::derive(registry, mutators, 1).unwrap();
            let footprint = Footprint::for_capacities(capacities).unwrap();
            // The caller owns this budget outside the catalog's measured backing.
            let budget = Budget::new(footprint.total());
            let before = audit::snapshot();
            let catalog = BoundedCatalog::new_in(capacities, &budget).unwrap();
            let after = audit::snapshot();
            assert_eq!(
                after.requested - before.requested,
                footprint.backing() as u64,
                "computed backing differs from allocator requests"
            );
            assert_eq!(after.frees, before.frees);
            assert_eq!(budget.used(), footprint.total());
            let clone = catalog.clone();
            let mut helper = catalog.register_helper().unwrap();
            if tracing {
                trace_cycle(&mut helper);
                traced_cycles += 1;
            }
            let held = audit::snapshot();
            assert_eq!(
                held.requests, after.requests,
                "registration or tracing allocated backing"
            );
            assert_eq!(held.frees, after.frees, "tracing released catalog backing");
            assert_eq!(
                held.refunds, after.refunds,
                "tracing refunded live catalog credits"
            );
            drop((catalog, clone));
            assert_eq!(budget.used(), footprint.total());
            drop(helper);
            let freed = audit::snapshot();
            assert_eq!(budget.used(), 0);
            assert_eq!(freed.refunds - before.refunds, 1);
            assert_eq!(freed.refunded - before.refunded, footprint.total() as u64);
            assert_eq!(freed.released - before.released, footprint.backing() as u64);
            assert_eq!(freed.frees - before.frees, after.requests - before.requests);
            assert!(
                freed.last_refund > freed.last_free,
                "credit returned before final backing free"
            );
            println!(
                "{registry},{mutators},1,{},{},{},{},true",
                footprint.backing(),
                footprint.participants(),
                footprint.total(),
                after.requests - before.requests
            );
            let short = Budget::new(footprint.total() - 1);
            let before = audit::snapshot();
            assert!(matches!(
                BoundedCatalog::new_in(capacities, &short),
                Err(ConfigError::BudgetExceeded)
            ));
            let after = audit::snapshot();
            assert_eq!(
                after.requests, before.requests,
                "over-budget constructor allocated"
            );
            assert_eq!(after.refunds, before.refunds);
            assert_eq!(short.used(), 0);
        }
    }
    if tracing {
        println!("TRACING_AUDIT completed_cycles={traced_cycles}");
    }
}

fn advance_cycle(participant: &mut Participant, domain: usize) -> Progress {
    let entry = participant.phase().unwrap().domains[domain];
    if entry.floor == RETIREMENT_FLOOR {
        assert_eq!(
            run(participant, Command::Finish { domain }),
            Progress::Committed
        );
        run(
            participant,
            Command::Start {
                domain,
                mode: entry.mode,
            },
        )
    } else {
        run(participant, Command::Advance { domain })
    }
}
