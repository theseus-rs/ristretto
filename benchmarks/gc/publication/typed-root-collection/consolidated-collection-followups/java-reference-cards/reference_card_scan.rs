//! Time ordinary full collections of a live Java reference array.
use ristretto_classfile::{ClassFile, ConstantPool};
use ristretto_classloader::{Class, Reference, Value};
use ristretto_gc::{Configuration, GarbageCollector};
use std::hint::black_box;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let shape = std::env::args().nth(1).ok_or("missing graph shape")?;
    let width = 4096;
    let heap = GarbageCollector::with_config(
        Configuration::builder()
            .threads(0)
            .allocation_threshold(usize::MAX)
            .build(),
    );
    let mut pool = ConstantPool::default();
    let this_class = pool.add_class("[Ljava/lang/Object;")?;
    let class = Class::from(
        None,
        ClassFile {
            constant_pool: pool,
            this_class,
            ..Default::default()
        },
    )?;
    let leaf = |n| Value::new_object(&heap, Reference::from(vec![n]));
    let values = match shape.as_str() {
        "alias" => vec![leaf(1_i32); width],
        "unique" => (0..width).map(|i| leaf(i as i32)).collect(),
        "null" => vec![Value::Object(None); width],
        _ => return Err("unknown graph shape".into()),
    };
    let root = Value::new_object(&heap, Reference::try_from((class, values))?);
    let mut times = Vec::new();
    let live = heap.managed_bytes();
    for round in 0..19 {
        let start = Instant::now();
        heap.collect_full()?;
        let elapsed = start.elapsed().as_nanos();
        assert_eq!(heap.managed_bytes(), live);
        if round >= 3 {
            times.push(elapsed);
        }
    }
    let (_, values) = root.as_class_vec_ref()?;
    assert_eq!(values.len(), width);
    for (i, value) in values.iter().enumerate() {
        match shape.as_str() {
            "null" => assert!(matches!(value, Value::Object(None))),
            "unique" => assert_eq!(value.as_int_vec_ref()?[0], i as i32),
            _ => assert_eq!(value.as_int_vec_ref()?[0], 1),
        }
    }
    black_box(&root);
    drop(values);
    drop(root);
    heap.collect_full()?;
    assert_eq!(heap.managed_bytes(), 0);
    println!("{times:?}");
    Ok(())
}
