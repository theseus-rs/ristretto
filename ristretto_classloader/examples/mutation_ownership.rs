//! Mechanism costs of replacement stores versus mutable values that may escape.
//! Usage: `mutation_ownership` MODE OPERATIONS LENGTH COUNTED COMPACT CONTEXTS
//! MODE: field-store/element-borrow/slice-borrow/load, or primitive-borrow/bytes/whole/load.
#![expect(clippy::print_stdout)]
use ristretto_classfile::{ClassFile, ConstantPool, Field, FieldAccessFlags, FieldType};
use ristretto_classloader::{Class, Object, ObjectArray, Reference, ReferenceWriteGuardExt, Value};
use ristretto_gc::{Configuration, GarbageCollector, RootStorage};
use std::hint::black_box;
use std::time::Instant;

#[expect(
    clippy::too_many_lines,
    reason = "standalone measurement and integrity driver"
)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().collect();
    let mode = args.get(1).ok_or("missing mode")?;
    let operations: u64 = args.get(2).ok_or("missing operations")?.parse()?;
    let length: usize = args.get(3).ok_or("missing length")?.parse()?;
    let counted: bool = args.get(4).ok_or("missing counted")?.parse()?;
    let compact: bool = args.get(5).ok_or("missing compact")?.parse()?;
    let contexts: bool = args.get(6).ok_or("missing contexts")?.parse()?;
    if operations == 0
        || length == 0
        || !matches!(
            mode.as_str(),
            "field-store"
                | "element-borrow"
                | "slice-borrow"
                | "load"
                | "primitive-borrow"
                | "primitive-bytes"
                | "primitive-whole"
                | "primitive-load"
        )
    {
        return Err("invalid workload".into());
    }
    let gc = GarbageCollector::with_config(
        Configuration::builder()
            .concurrent_publication(counted)
            .compact_heap(compact)
            .root_storage(if contexts {
                RootStorage::Contexts
            } else {
                RootStorage::Tree
            })
            .barrier_buffer_size(usize::from(contexts))
            .allocation_threshold(usize::MAX)
            .full_collection_interval(usize::MAX)
            .build(),
    );
    let context = gc.register_context();
    let elapsed = context.with(|| -> Result<_, Box<dyn std::error::Error>> {
        let mut pool = ConstantPool::default();
        let this_class = pool.add_class("MutationOwnership")?;
        let descriptor_index = pool.add_utf8("Ljava/lang/Object;")?;
        let mut fields = Vec::with_capacity(length);
        for index in 0..length {
            fields.push(Field {
                access_flags: FieldAccessFlags::PUBLIC,
                name_index: pool.add_utf8(format!("field{index}"))?,
                descriptor_index,
                field_type: FieldType::Object("java/lang/Object".into()),
                attributes: Vec::new(),
            });
        }
        let class = Class::from(
            None,
            ClassFile {
                constant_pool: pool,
                this_class,
                fields,
                ..ClassFile::default()
            },
        )?;
        let first = Value::new_object(&gc, Reference::from(vec![1_i8; 16]));
        let second = Value::new_object(&gc, Reference::from(vec![2_i8; 16]));
        let source = if mode.starts_with("primitive-") {
            Value::new_object(&gc, Reference::IntArray(vec![7_i32; length].into()))
        } else if mode == "field-store" {
            let mut object = Object::new(class)?;
            for index in 0..length {
                object.set_value_at_slot(index, first.clone())?;
            }
            Value::new_object(&gc, Reference::Object(object))
        } else {
            Value::new_object(
                &gc,
                Reference::Array(ObjectArray {
                    class,
                    elements: vec![first.clone(); length].into(),
                }),
            )
        };
        gc.collect_full()?;
        let run = |count| -> Result<(), Box<dyn std::error::Error>> {
            for index in 0..count {
                let slot = usize::try_from(index % u64::try_from(length)?)?;
                match mode.as_str() {
                    "primitive-borrow" => {
                        let mut guard = source.as_int_vec_mut()?;
                        *guard.get_mut(black_box(slot)).ok_or("index")? = black_box(7);
                    }
                    "primitive-bytes" => {
                        let mut guard = source.as_bytes_mut()?;
                        let value = guard
                            .get_mut(black_box(slot * size_of::<i32>()))
                            .ok_or("index")?;
                        *value = black_box(*value);
                    }
                    "primitive-whole" => {
                        let mut guard = source.as_reference_mut()?;
                        let Reference::IntArray(array) = &mut *guard else {
                            return Err("int array expected".into());
                        };
                        *array.get_mut(black_box(slot)).ok_or("index")? = black_box(7);
                    }
                    "primitive-load" => {
                        let guard = source.as_int_vec_ref()?;
                        black_box(*guard.get(black_box(slot)).ok_or("index")?);
                    }
                    "field-store" => {
                        let input = if index % 2 == 0 { &first } else { &second };
                        let mut guard = source.as_reference_mut()?;
                        guard.set_value_at_slot(black_box(slot), input.clone())?;
                    }
                    "element-borrow" => {
                        let mut guard = source.as_reference_mut()?;
                        let value = guard.array_element_mut(black_box(slot)).ok_or("index")?;
                        black_box(std::mem::replace(value, first.clone()));
                    }
                    "slice-borrow" => {
                        let mut guard = source.as_class_vec_mut()?;
                        let value = guard.1.get_mut(black_box(slot)).ok_or("index")?;
                        black_box(std::mem::replace(value, first.clone()));
                    }
                    "load" => {
                        let guard = source.as_class_vec_ref()?;
                        black_box(guard.1.get(black_box(slot)).ok_or("index")?.clone());
                    }
                    _ => return Err("unknown workload".into()),
                }
            }
            Ok(())
        };
        run(operations.min(10_000))?;
        let started = Instant::now();
        run(operations)?;
        let elapsed = started.elapsed().as_nanos();
        gc.collect_full()?;
        if mode.starts_with("primitive-") {
            if !source.as_int_vec_ref()?.iter().all(|value| *value == 7) {
                return Err("bad primitive contents".into());
            }
        } else if mode == "field-store" {
            let guard = source.as_reference()?;
            let Reference::Object(object) = &*guard else {
                return Err("object expected".into());
            };
            for index in 0..length {
                let value = object.value_at_slot(index)?;
                let bytes = value.as_byte_vec_ref()?;
                if !matches!(bytes.first(), Some(1 | 2)) {
                    return Err("bad field contents".into());
                }
            }
        } else {
            let guard = source.as_class_vec_ref()?;
            for value in guard.1.iter() {
                if value.as_byte_vec_ref()?.first() != Some(&1) {
                    return Err("bad array contents".into());
                }
            }
        }
        Ok(elapsed)
    })?;
    drop(context);
    gc.stop()?;
    let stats = gc.statistics()?;
    if stats.bytes_managed != 0 || stats.live_objects != 0 {
        return Err("teardown leak".into());
    }
    let cell_size = size_of::<ristretto_gc::cell::RwLock<Reference>>();
    let writer_size = size_of::<ristretto_gc::cell::RwLockWriteGuard<'static, Reference>>();
    let mapped_writer_size =
        size_of::<ristretto_gc::cell::MappedRwLockWriteGuard<'static, [i32], Reference>>();
    let reference_size = size_of::<Reference>();
    println!(
        "{{\"mode\":\"{mode}\",\"operations\":{operations},\"length\":{length},\"counted\":{counted},\"compact\":{compact},\"contexts\":{contexts},\"elapsed_ns\":{elapsed},\"live_objects\":0,\"bytes_allocated\":0,\"cell_size\":{cell_size},\"writer_size\":{writer_size},\"mapped_writer_size\":{mapped_writer_size},\"reference_size\":{reference_size}}}"
    );
    Ok(())
}
