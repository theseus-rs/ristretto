//! Timed construction and first publication of ordinary Java instances.
use ristretto_classfile::{ClassFile, ConstantPool, Field, FieldAccessFlags, FieldType};
use ristretto_classloader::{Class, Object, Reference, Value};
use ristretto_gc::{Configuration, GarbageCollector};
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let width: usize = std::env::args()
        .nth(1)
        .ok_or("missing field count")?
        .parse()?;
    let mut pool = ConstantPool::default();
    let this_class = pool.add_class("DefaultFields")?;
    let descriptor_index = pool.add_utf8("Ljava/lang/Object;")?;
    let mut fields = Vec::new();
    for i in 0..width {
        fields.push(Field {
            access_flags: FieldAccessFlags::PUBLIC,
            name_index: pool.add_utf8(format!("f{i}"))?,
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
            ..Default::default()
        },
    )?;
    let heap = GarbageCollector::with_config(
        Configuration::builder()
            .allocation_threshold(usize::MAX)
            .threads(0)
            .build(),
    );
    let mut times = Vec::new();
    for round in 0..19 {
        let mut roots = Vec::with_capacity(64);
        let start = Instant::now();
        for _ in 0..64 {
            let object = Object::new_in(&heap, class.clone())?;
            roots.push(Value::new_object(&heap, Reference::Object(object)));
        }
        let elapsed = start.elapsed().as_nanos();
        for root in &roots {
            let object = root.as_object_ref()?;
            for field in 0..width {
                assert!(matches!(object.value_at_slot(field)?, Value::Object(None)));
            }
        }
        drop(roots);
        heap.collect_full()?;
        assert_eq!(heap.managed_bytes(), 0);
        if round >= 3 {
            times.push(elapsed);
        }
    }
    // A reference written before first publication must acquire its actual owner.
    if width != 0 {
        let mut obj = Object::new_in(&heap, class.clone())?;
        obj.set_value_at_slot(
            width - 1,
            Value::new_object(&heap, Reference::from(vec![47_i32])),
        )?;
        let owner = Value::new_object(&heap, Reference::Object(obj));
        heap.collect_full()?;
        assert_eq!(
            owner
                .as_object_ref()?
                .value_at_slot(width - 1)?
                .as_int_vec_ref()?[0],
            47
        );
        drop(owner);
        heap.collect_full()?;
        assert_eq!(heap.managed_bytes(), 0);
    }
    println!("{times:?}");
    Ok(())
}
