use super::Error;
use super::objects::array_kind;
use super::primitive_array::Kind;
use super::statics::StaticLayout;
use ristretto_classfile::{BaseType, ClassFile, FieldAccessFlags, FieldType, JavaString};
use ristretto_classloader::Class;
use std::collections::HashMap;
use std::sync::Arc;

pub(super) const MAX_INTEGER_FIELDS: usize = 8;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum ReferenceType {
    Object(usize),
    Array(Kind),
}

pub(super) fn reference_type(
    ty: &FieldType,
    names: &HashMap<JavaString, usize>,
) -> Result<ReferenceType, Error> {
    match ty {
        FieldType::Object(name) => names
            .get(name)
            .copied()
            .map(ReferenceType::Object)
            .ok_or_else(|| Error::Unsupported(format!("field class {name}"))),
        FieldType::Array(element) => array_kind(element).map(ReferenceType::Array),
        FieldType::Base(_) => Err(Error::Boundary("primitive reference type")),
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum Field {
    Int(usize),
    Reference { edge: usize, target: ReferenceType },
}

/// Fixed slots for fields of a verified non-inheriting Java instance.
#[derive(Debug)]
pub(super) struct Layout {
    pub(super) class: Arc<Class>,
    fields: HashMap<(JavaString, JavaString), Field>,
    pub(super) references: usize,
    pub(super) statics: StaticLayout,
}

impl Layout {
    pub(super) fn new(
        file: ClassFile<'static>,
        names: &HashMap<JavaString, usize>,
    ) -> Result<Self, Error> {
        let mut fields = HashMap::new();
        let mut statics = StaticLayout::default();
        let mut integers = 0;
        let mut references = 0;
        for field in &file.fields {
            let name = file.constant_pool.try_get_utf8(field.name_index)?;
            let descriptor = file.constant_pool.try_get_utf8(field.descriptor_index)?;
            if field.access_flags.contains(FieldAccessFlags::STATIC) {
                statics.admit(field, &file.constant_pool, names)?;
                continue;
            }
            if field
                .access_flags
                .intersects(FieldAccessFlags::FINAL | FieldAccessFlags::VOLATILE)
            {
                return Err(Error::Unsupported(format!(
                    "instance field {name} {descriptor}"
                )));
            }
            let storage = match FieldType::parse_java_str(descriptor)? {
                FieldType::Base(BaseType::Int) if integers < MAX_INTEGER_FIELDS => {
                    let field = Field::Int(integers);
                    integers += 1;
                    field
                }
                FieldType::Object(target) if references < 8 => {
                    let class = names
                        .get(&target)
                        .copied()
                        .ok_or_else(|| Error::Unsupported(format!("field class {target}")))?;
                    let field = Field::Reference {
                        edge: references,
                        target: ReferenceType::Object(class),
                    };
                    references += 1;
                    field
                }
                FieldType::Array(component) if references < 8 => {
                    let field = Field::Reference {
                        edge: references,
                        target: ReferenceType::Array(array_kind(&component)?),
                    };
                    references += 1;
                    field
                }
                _ => {
                    return Err(Error::Unsupported(format!(
                        "instance field {name} {descriptor}"
                    )));
                }
            };
            fields.insert((name.to_owned(), descriptor.to_owned()), storage);
        }
        let class = Class::from(None, file)?;
        statics.initializer = class
            .methods()
            .into_iter()
            .find(|method| method.name() == "<clinit>");
        Ok(Self {
            class,
            fields,
            references,
            statics,
        })
    }

    pub(super) fn field(&self, name: &str, descriptor: &str) -> Option<Field> {
        self.fields.get(&(name.into(), descriptor.into())).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integer_slots_follow_classfile_order() -> Result<(), Error> {
        let file = ClassFile::from_bytes(include_bytes!(
            "../../tests/fixtures/canonical/CanonicalIntLayout.class"
        ))?;
        let layout = Layout::new(file, &HashMap::new())?;
        for (slot, name) in ["x", "y", "z", "a", "b", "c", "d", "e"].iter().enumerate() {
            assert_eq!(layout.field(name, "I"), Some(Field::Int(slot)));
        }
        assert_eq!(layout.field("missing", "I"), None);
        Ok(())
    }
}
