//! Measures actual Java array mutation and cooperative collection, including large-array disposal.
#![expect(clippy::print_stdout, clippy::panic_in_result_fn)]
use ristretto_classfile::ClassFile;
use ristretto_classloader::{Class, ObjectArray, Reference, Value};
use ristretto_gc::{Configuration, GarbageCollector};
use std::time::{Duration, Instant};

fn report(name: &str, samples: &mut [Duration]) {
    samples.sort_unstable();
    let at = |n: usize, d: usize| {
        samples
            .get((samples.len() - 1) * n / d)
            .copied()
            .unwrap_or_default()
            .as_secs_f64()
            * 1_000_000.0
    };
    println!(
        "{name}: n={} p50={:.3}us p99={:.3}us p99.9={:.3}us max={:.3}us",
        samples.len(),
        at(50, 100),
        at(99, 100),
        at(999, 1000),
        at(1, 1)
    );
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let gc = GarbageCollector::with_config(
        Configuration::builder()
            .allocation_threshold(usize::MAX)
            .incremental_step_size(128)
            .max_pause_time_us(100)
            .full_collection_interval(usize::MAX)
            .build(),
    );
    let mut file = ClassFile::default();
    file.this_class = file.constant_pool.add_class("[Ljava/lang/Object;")?;
    let class = Class::from(None, file)?;
    let array = Value::try_new_object(
        &gc,
        Reference::Array(ObjectArray {
            class,
            elements: vec![Value::Object(None); 1_000_000].into(),
        }),
    )?;
    gc.collect_full()?;
    gc.collect_full()?;
    gc.collect_young();
    while !gc.step() {}
    let mut stores = Vec::new();
    let mut minor = Vec::new();
    for i in 0..10_000 {
        let child = Value::try_new_object(&gc, Reference::ByteArray(vec![42; 128].into()))?;
        let start = Instant::now();
        if let Reference::Array(data) = &mut *array.as_reference_mut()? {
            *data.elements.get_mut(i % 1_000_000).ok_or("array index")? = child;
        }
        stores.push(start.elapsed());
        if i % 100 == 0 {
            gc.collect_young();
            loop {
                let start = Instant::now();
                let idle = gc.step();
                minor.push(start.elapsed());
                if idle {
                    break;
                }
            }
        }
    }
    report("million-slot old Java array point store", &mut stores);
    report("young collection step", &mut minor);
    drop(array);
    gc.collect();
    let mut disposal = Vec::new();
    loop {
        let start = Instant::now();
        let idle = gc.step();
        disposal.push(start.elapsed());
        if idle {
            break;
        }
    }
    assert_eq!(gc.statistics()?.live_objects, 0);
    report(
        "large Java array collection and disposal step",
        &mut disposal,
    );
    Ok(())
}
