use super::*;
use crate::Object;
use ristretto_classfile::FieldType;

fn class(name: &str, parent: Option<&Arc<Class>>, field_name: &str) -> Result<Arc<Class>> {
    class_with_field(name, parent, field_name, "I")
}

fn class_with_field(
    name: &str,
    parent: Option<&Arc<Class>>,
    field_name: &str,
    descriptor: &str,
) -> Result<Arc<Class>> {
    let mut pool = ConstantPool::default();
    let this_class = pool.add_class(name)?;
    let super_class = parent.map_or(Ok(0), |parent| pool.add_class(parent.name()))?;
    let field = ristretto_classfile::Field {
        access_flags: FieldAccessFlags::PUBLIC,
        name_index: pool.add_utf8(field_name)?,
        descriptor_index: pool.add_utf8(descriptor)?,
        field_type: FieldType::parse(descriptor)?,
        attributes: vec![],
    };
    Class::from(
        None,
        ClassFile {
            constant_pool: pool,
            access_flags: ClassAccessFlags::PUBLIC,
            this_class,
            super_class,
            fields: vec![field],
            ..Default::default()
        },
    )
}

#[test]
fn linked_layouts_are_shared_and_keep_ancestor_slots() -> Result<()> {
    let base = class("Base", None, "value")?;
    base.finalize_field_layout()?;
    let child = class("Child", Some(&base), "value")?;
    assert!(child.finalize_field_layout().is_err());
    child.set_parent(Some(base.clone()))?;
    child.finalize_field_layout()?;
    let first = child.object_field_layout()?;
    let second = child.object_field_layout()?;
    assert!(matches!(first, Cow::Borrowed(_)));
    assert!(std::ptr::eq(first.as_ptr(), second.as_ptr()));
    assert_eq!(first.len(), 2);
    assert!(Arc::ptr_eq(
        first.first().expect("inherited field"),
        base.object_field_layout()?.first().expect("base field")
    ));
    let mut object = Object::new(child.clone())?;
    object.set_value_at_slot(0, Value::Int(7))?;
    object.set_value_at_slot(1, Value::Int(9))?;
    assert_eq!(object.value_in_class(&base, "value")?, Value::Int(7));
    assert_eq!(object.value("value")?, Value::Int(9));
    assert!(object.value_at_slot(2).is_err());
    assert!(object.set_value_at_slot(2, Value::Int(0)).is_err());
    child.set_parent(Some(base))?;
    child.set_interfaces(vec![])?;
    assert!(child.set_parent(None).is_err());
    assert!(
        child
            .set_interfaces(vec![class("Other", None, "other")?])
            .is_err()
    );
    Ok(())
}

#[test]
fn conditional_tracing_and_native_access_share_the_declared_reference_slot() -> Result<()> {
    use JavaReferenceField::{Queue, Referent};

    let descriptor = "Ljava/lang/Object;";
    let ancestor = class_with_field("Ancestor", None, "referent", descriptor)?;
    ancestor.finalize_field_layout()?;
    for declared in [descriptor, "I"] {
        let base = class_with_field(
            "java/lang/ref/Reference",
            Some(&ancestor),
            "referent",
            declared,
        )?;
        base.set_parent(Some(ancestor.clone()))?;
        base.finalize_field_layout()?;
        let weak = class_with_field(
            "java/lang/ref/WeakReference",
            Some(&base),
            "referent",
            descriptor,
        )?;
        weak.set_parent(Some(base))?;
        weak.finalize_field_layout()?;
        let child = class_with_field(
            "Child",
            Some(&weak),
            "queue",
            "Ljava/lang/ref/ReferenceQueue;",
        )?;
        child.set_parent(Some(weak))?;
        let layout = child.object_field_layout()?;
        if declared == descriptor {
            assert_eq!(child.conditional_referent_slot(&layout)?, Some(1));
        } else {
            assert!(Object::new(child.clone()).is_err());
        }
        drop(layout);
        child.finalize_field_layout()?;
        assert!(child.java_reference_field_slot(Queue).is_err());
        if declared == descriptor {
            assert_eq!(child.java_reference_field_slot(Referent)?, 1);
            let gc = ristretto_gc::GarbageCollector::new();
            let mut object = Object::new_in(&gc, child)?;
            for (slot, marker) in [(0, 1_i8), (1, 2), (2, 3)] {
                let value =
                    Value::new_object(&gc, crate::Reference::ByteArray(vec![marker].into()));
                object.set_value_at_slot(slot, value)?;
            }
            let root = Value::new_object(&gc, crate::Reference::Object(object));
            gc.collect_full()?;
            gc.collect_full()?;
            let object = root.as_object_ref()?;
            assert!(object.value_at_slot(1)?.is_null());
            for (slot, marker) in [(0, 1_i8), (2, 3)] {
                assert_eq!(&*object.value_at_slot(slot)?.as_byte_vec_ref()?, &[marker]);
            }
        } else {
            assert!(child.java_reference_field_slot(Referent).is_err());
            assert!(Object::new(child).is_err());
        }
    }
    Ok(())
}

#[test]
fn unlinked_layouts_follow_hierarchy_changes() -> Result<()> {
    let base = class("Base", None, "base")?;
    let child = class("Child", Some(&base), "child")?;
    assert!(matches!(child.object_field_layout()?, Cow::Owned(_)));
    assert_eq!(child.object_field_layout()?.len(), 1);
    child.set_parent(Some(base.clone()))?;
    assert_eq!(child.object_field_layout()?.len(), 2);
    assert!(child.finalize_field_layout().is_err());
    base.finalize_field_layout()?;
    child.finalize_field_layout()?;
    assert!(matches!(child.object_field_layout()?, Cow::Borrowed(_)));
    Ok(())
}

#[test]
fn verification_is_cached_and_invalidated_by_constant_pool_mutation() -> Result<()> {
    let definition = ClassFile::from_bytes(include_bytes!("../../../classes/Minimum.class"))?;
    let mut class = Class::from(None, definition)?;
    assert!(class.verify_cached().is_ok());
    assert!(class.verification.get().is_some());
    let class = Arc::get_mut(&mut class).expect("unique class");
    let index = class.constant_pool_mut().add(Constant::Class(u16::MAX))?;
    assert!(class.verification.get().is_none());
    assert!(class.verify_cached().is_err());
    assert_eq!(class.verify_cached(), class.verify_cached());
    // Repairing the definition must also discard the cached failure.
    let name_index = class.constant_pool_mut().add_utf8("Other")?;
    class
        .constant_pool_mut()
        .set(index, Constant::Class(name_index))?;
    assert!(class.verification.get().is_none());
    assert!(class.verify_cached().is_ok());
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn concurrent_layout_finalization_preserves_shared_storage() -> Result<()> {
    let base = class("Base", None, "base")?;
    base.finalize_field_layout()?;
    let child = class("Child", Some(&base), "child")?;
    child.set_parent(Some(base))?;
    std::thread::scope(|scope| -> Result<()> {
        let handles = (0..8)
            .map(|_| {
                let child = &child;
                scope.spawn(move || child.finalize_field_layout())
            })
            .collect::<Vec<_>>();
        for handle in handles {
            handle.join().expect("layout thread must finish")?;
        }
        Ok(())
    })?;
    assert_eq!(child.object_field_layout()?.len(), 2);
    assert!(child.set_parent(None).is_err());
    Ok(())
}

#[test]
fn linked_reference_classes_keep_the_nearest_kind_and_its_classfile_version() -> Result<()> {
    let ordinary = class("Ordinary", None, "referent")?;
    ordinary.finalize_field_layout()?;
    assert_eq!(ordinary.reference_kind()?, None);
    for (version, expected) in [
        (ristretto_classfile::JAVA_8, ReferenceKind::PhantomRetained),
        (ristretto_classfile::JAVA_17, ReferenceKind::Phantom),
    ] {
        let mut definition = class("java/lang/ref/PhantomReference", None, "referent")?
            .class_file()
            .clone();
        definition.version = version;
        let phantom = Class::from(None, definition)?;
        phantom.finalize_field_layout()?;
        let child = class("Child", Some(&phantom), "referent")?;
        child.set_parent(Some(ordinary.clone()))?;
        assert_eq!(child.reference_kind()?, None);
        child.set_parent(Some(phantom))?;
        assert_eq!(child.reference_kind()?, Some(expected));
        child.finalize_field_layout()?;
        assert_eq!(child.reference_kind()?, Some(expected));
        let weak = class("java/lang/ref/WeakReference", Some(&child), "other")?;
        weak.set_parent(Some(child))?;
        weak.finalize_field_layout()?;
        assert_eq!(weak.reference_kind()?, Some(ReferenceKind::Weak));
    }
    Ok(())
}
