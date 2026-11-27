//! Count and time physical cleanup turns for a Java reference array.
use ristretto_classfile::{ClassFile, ConstantPool};
use ristretto_classloader::{Class, Reference, Value};
use ristretto_gc::{Configuration, GarbageCollector, Trace, TraceProgress};
use std::{error::Error, hint::black_box, sync::Arc, time::Instant};

fn main() -> Result<(), Box<dyn Error>> {
    let shape = std::env::args().nth(1).ok_or("array shape")?;
    let width = 4097usize;
    let heap = GarbageCollector::with_config(
        Configuration::builder().threads(0).allocation_threshold(usize::MAX).build(),
    );
    let mut pool = ConstantPool::default();
    let this_class = pool.add_class("[Ljava/lang/Object;")?;
    let class = Class::from(None, ClassFile {constant_pool: pool, this_class, ..ClassFile::default()})?;
    let target = Value::new_object(&heap, Reference::IntArray(vec![42].into()));
    let mut ns = Vec::new();
    let mut turns = Vec::new();
    for round in 0..35 {
        let mut values = if shape == "dense" {vec![target.clone(); width]} else {vec![Value::Object(None); width]};
        match shape.as_str() {
            "null" | "dense" => {},
            "endpoints" => {values[0]=target.clone();values[width-1]=target.clone();},
            "strided" => {for slot in values.iter_mut().step_by(33) {*slot=target.clone();}},
            _=>return Err("unknown shape".into()),
        }
        let mut reference = Reference::try_from((Arc::clone(&class),values))?;
        let started=Instant::now();
        let mut calls=1;
        while black_box(&mut reference).clear_step(0,32)!=TraceProgress::Complete {
            calls+=1;
            assert!(calls<width);
        }
        let elapsed=started.elapsed().as_nanos();
        if round >= 3 {ns.push(elapsed);turns.push(calls);}
        assert_eq!(target.as_int_vec_ref()?[0],42);
    }
    drop(target);
    heap.collect_full()?;
    assert_eq!(heap.managed_bytes(),0);
    println!("{{\"times_ns\":{ns:?},\"turns\":{turns:?},\"reference_size\":{},\"value_size\":{}}}",size_of::<Reference>(),size_of::<Value>());
    Ok(())
}
