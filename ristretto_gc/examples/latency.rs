//! Reproducible latency experiment; includes allocation/rooting/destruction stalls.
#![expect(clippy::print_stdout, clippy::panic_in_result_fn)]
use ristretto_gc::{Configuration, GarbageCollector, Gc, Result};
use std::time::{Duration, Instant};

fn report(name: &str, samples: &mut [Duration]) {
    samples.sort_unstable();
    let percentile = |numerator: usize, denominator: usize| {
        samples
            .get(samples.len().saturating_sub(1) * numerator / denominator)
            .copied()
            .unwrap_or_default()
            .as_secs_f64()
            * 1_000_000.0
    };
    println!(
        "{name}: n={} p50={:.3}us p99={:.3}us p99.9={:.3}us max={:.3}us",
        samples.len(),
        percentile(50, 100),
        percentile(99, 100),
        percentile(999, 1000),
        percentile(1, 1)
    );
}
fn main() -> Result<()> {
    let gc = GarbageCollector::with_config(
        Configuration::builder()
            .allocation_threshold(256 * 1024)
            .max_pause_time_us(100)
            .incremental_step_size(128)
            .full_collection_interval(64)
            .build(),
    );
    // A resident old graph. The graph's elements become traced edges during allocation.
    let children: Vec<_> = (0..10_000).map(|i| Gc::new(&gc, i).into_rooted()).collect();
    let resident = Gc::new(&gc, children).into_rooted();
    gc.collect_full()?;
    gc.collect_full()?;
    gc.collect_young();
    while !gc.step() {}
    let before = gc.statistics()?;
    gc.start();
    let mut mutator = Vec::with_capacity(100_000);
    for i in 0..100_000_u64 {
        let start = Instant::now();
        let allocation = Gc::new(&gc, vec![i; 64]).into_rooted();
        assert_eq!(allocation.first(), Some(&i));
        drop(allocation);
        mutator.push(start.elapsed());
    }
    gc.collect_full()?;
    gc.stop()?;
    let stats = gc.statistics()?;
    report("native allocation + root + release", &mut mutator);
    println!(
        "resident_objects={} young_cycles={} full_cycles={} marked={} max_collector_step={:?}",
        resident.len() + 1,
        stats.young_collections - before.young_collections,
        stats.full_collections - before.full_collections,
        stats.objects_marked - before.objects_marked,
        stats.max_step_duration
    );

    let cooperative = GarbageCollector::with_config(
        Configuration::builder()
            .allocation_threshold(usize::MAX)
            .max_pause_time_us(100)
            .incremental_step_size(128)
            .build(),
    );
    let mut steps = Vec::new();
    for _ in 0..20 {
        for i in 0..10_000 {
            drop(Gc::new(&cooperative, vec![i; 32]));
        }
        cooperative.collect();
        loop {
            let start = Instant::now();
            let finished = cooperative.step();
            steps.push(start.elapsed());
            if finished {
                break;
            }
        }
    }
    report("cooperative collection step", &mut steps);
    Ok(())
}
