use super::{Error, Result};
use ristretto_classfile::{ClassFile, FieldAccessFlags, JavaString};
use ristretto_classloader::Class;
use std::collections::HashMap;
use std::sync::Arc;

pub(super) const MAX_INTEGER_FIELDS: usize = 8;

/// Fixed slots for fields of a verified non-inheriting Java instance.
#[derive(Debug)]
pub(super) struct Layout {
    pub(super) class: Arc<Class>,
    integer_slots: HashMap<JavaString, usize>,
}

impl Layout {
    pub(super) fn new(file: ClassFile<'static>) -> Result<Self> {
        let mut integer_slots = HashMap::new();
        for (slot, field) in file.fields.iter().enumerate() {
            let name = file.constant_pool.try_get_utf8(field.name_index)?;
            let descriptor = file.constant_pool.try_get_utf8(field.descriptor_index)?;
            if descriptor != "I"
                || slot >= MAX_INTEGER_FIELDS
                || field.access_flags.intersects(
                    FieldAccessFlags::STATIC | FieldAccessFlags::FINAL | FieldAccessFlags::VOLATILE,
                )
            {
                return Err(Error::Unsupported(format!(
                    "instance field {name} {descriptor}"
                )));
            }
            integer_slots.insert(name.to_owned(), slot);
        }
        Ok(Self {
            class: Class::from(None, file)?,
            integer_slots,
        })
    }

    pub(super) fn slot(&self, name: &ristretto_classfile::JavaStr) -> Option<usize> {
        self.integer_slots.get(name).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ristretto_classfile::JavaStr;

    #[test]
    fn integer_slots_follow_classfile_order() -> Result<()> {
        let file = ClassFile::from_bytes(include_bytes!(
            "../../tests/fixtures/canonical/CanonicalIntLayout.class"
        ))?;
        let layout = Layout::new(file)?;
        for (slot, name) in ["x", "y", "z", "a", "b", "c", "d", "e"].iter().enumerate() {
            assert_eq!(layout.slot(&JavaStr::cow_from_str(name)), Some(slot));
        }
        assert_eq!(layout.slot(&JavaStr::cow_from_str("missing")), None);
        Ok(())
    }
}
