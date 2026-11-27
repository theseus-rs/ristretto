//! Measures ordinary scalar or single-edge graph allocation during an active, cooperative collection.
#![expect(clippy::print_stdout, clippy::panic_in_result_fn)]

use ristretto_gc::{Configuration, GarbageCollector, Gc, GcRootGuard, Result, Trace};
use std::sync::Arc;
use std::time::{Duration, Instant};

const ROUNDS: usize = 128;
const BATCH: usize = 512;

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let shape = args.next();
    let young = match args.next().as_deref() {
        None | Some("full") => false,
        Some("young") => true,
        Some(_) => {
            return Err(ristretto_gc::Error::RootError(
                "expected full or young".into(),
            ));
        }
    };
    let config = Configuration::builder()
        .concurrent_publication(true)
        .allocation_threshold(usize::MAX)
        .incremental_step_size(1)
        .max_pause_time_us(0);
    let config = if young {
        config.promotion_age(1).full_collection_interval(usize::MAX)
    } else {
        config
    };
    let gc = GarbageCollector::with_config(config.build());
    match shape.as_deref() {
        None | Some("scalar") => run(&gc, young, |n, _| n, |value| *value),
        Some("graph") => run(
            &gc,
            young,
            |n, anchor| (n, anchor.clone().into_rooted()),
            |(value, edge)| *value + **edge,
        ),
        Some(_) => Err(ristretto_gc::Error::RootError(
            "expected scalar or graph".into(),
        )),
    }
}

fn run<T: Trace + Send + Sync + 'static>(
    gc: &Arc<GarbageCollector>,
    young: bool,
    make: impl Fn(usize, &GcRootGuard<usize>) -> T,
    read: impl Fn(&T) -> usize,
) -> Result<()> {
    let anchor = Gc::try_new(gc, 0_usize)?;
    if young {
        gc.collect_full()?;
        gc.collect_young();
        while !gc.step() {}
    }
    let mut promoted = 0;
    let mut roots = Vec::with_capacity(BATCH);
    let mut allocation = Duration::ZERO;
    let mut finish = Duration::ZERO;
    let mut inline_marks = 0;
    let mut checksum = 0;
    for _ in 0..ROUNDS {
        let prior = gc.statistics()?;
        if young {
            gc.collect_young();
        } else {
            gc.collect();
        }
        // The first bounded step starts the snapshot. With no workers or
        // automatic allocation requests, every birth sees that same strong phase.
        assert!(!gc.step());
        let begun = gc.statistics()?;
        assert_eq!(begun.collections_started, prior.collections_started + 1);
        assert_eq!(begun.collections_completed, prior.collections_completed);
        let started = Instant::now();
        for value in 1..=BATCH {
            roots.push(Gc::try_new(gc, make(value, &anchor))?);
        }
        allocation += started.elapsed();
        inline_marks += gc.statistics()?.objects_marked - begun.objects_marked;
        let started = Instant::now();
        while !gc.step() {}
        finish += started.elapsed();
        let finished = gc.statistics()?;
        assert_eq!(
            finished.objects_marked - prior.objects_marked,
            BATCH + usize::from(!young)
        );
        assert_eq!(
            finished.young_collections - prior.young_collections,
            usize::from(young)
        );
        assert_eq!(
            finished.full_collections - prior.full_collections,
            usize::from(!young)
        );
        if young {
            assert_eq!(finished.objects_promoted - prior.objects_promoted, BATCH);
        }
        promoted += finished.objects_promoted - prior.objects_promoted;
        checksum += roots.iter().map(|root| read(root)).sum::<usize>();
        roots.clear();
        gc.collect_full()?;
        assert_eq!(gc.statistics()?.live_objects, 1);
    }
    drop(anchor);
    gc.collect_full()?;
    let teardown_bytes = gc.managed_bytes();
    assert_eq!(teardown_bytes, 0);
    assert_eq!(checksum, ROUNDS * BATCH * (BATCH + 1) / 2);
    let collection = if young { "young" } else { "full" };
    println!(
        "{{\"collection\":\"{collection}\",\"promoted\":{promoted},\"operations\":{},\"batch\":{BATCH},\"allocation_ns\":{},\"finish_ns\":{},\
         \"inline_marks\":{inline_marks},\"checksum\":{checksum},\"teardown_bytes\":{teardown_bytes}}}",
        ROUNDS * BATCH,
        allocation.as_nanos(),
        finish.as_nanos(),
    );
    Ok(())
}
