use ristretto_gc::{Configuration, GarbageCollector};
use std::{hint::black_box, time::Instant};

fn main() {
    let n = 2_000_000_u64;
    let mut records = Vec::new();
    for _ in 0..4 {
        let gc = GarbageCollector::with_config(Configuration::builder().threads(0)
            .predictive_collection(true).max_heap_bytes(64 << 20)
            .allocation_threshold(usize::MAX).build());
        let begin = Instant::now();
        for _ in 0..n { gc.record_allocation(black_box(8)); }
        let ns = begin.elapsed().as_nanos();
        let model = gc.scheduling_statistics();
        assert!(model.enabled && model.samples > 0 && model.requests == 0);
        assert_eq!(gc.statistics().expect("statistics").bytes_allocated, (n * 8) as usize);
        records.push((ns, model.samples));
    }
    for (ns, samples) in &records[1..] { println!("{ns} {n} {samples}"); }
}
