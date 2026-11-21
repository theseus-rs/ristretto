use super::objects::{LinkedMembers, array_kind};
use super::primitive_array::Kind;
use super::reference_array::Component;
use super::statics::StaticLayout;
use super::{Error, Result};
use ristretto_classfile::{BaseType, ClassFile, FieldAccessFlags, FieldType, JavaString};
use ristretto_classloader::Class;
use std::collections::HashMap;
use std::sync::Arc;

pub(super) const MAX_INTEGER_FIELDS: usize = 8;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum ReferenceType {
    AnyObject,
    Object(usize),
    Array(Kind),
    ReferenceArray(Component),
}
impl ReferenceType {
    pub(super) fn accepts(self, actual: Self) -> bool {
        self == Self::AnyObject
            || self == actual
            || matches!(
                (self, actual),
                (
                    Self::ReferenceArray(Component::Object),
                    Self::ReferenceArray(_)
                )
            )
    }
}
pub(super) fn reference_type(
    field: &FieldType,
    names: &HashMap<JavaString, usize>,
) -> Result<ReferenceType> {
    match field {
        FieldType::Object(name) if name == "java/lang/Object" => Ok(ReferenceType::AnyObject),
        FieldType::Object(name) => names
            .get(name)
            .copied()
            .map(ReferenceType::Object)
            .ok_or_else(|| Error::Unsupported(format!("class {name}"))),
        FieldType::Array(component) => match component.as_ref() {
            FieldType::Base(_) => array_kind(component).map(ReferenceType::Array),
            FieldType::Object(_) => match reference_type(component, names)? {
                ReferenceType::AnyObject => Ok(ReferenceType::ReferenceArray(Component::Object)),
                ReferenceType::Object(class) => {
                    Ok(ReferenceType::ReferenceArray(Component::Class(class)))
                }
                _ => Err(Error::Boundary("reference component")),
            },
            FieldType::Array(_) => Err(Error::Unsupported(
                "multidimensional array descriptor".to_owned(),
            )),
        },
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
    pub(super) linked: LinkedMembers,
}

impl Layout {
    pub(super) fn new(
        file: ClassFile<'static>,
        names: &HashMap<JavaString, usize>,
    ) -> Result<Self> {
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
                reference @ (FieldType::Object(_) | FieldType::Array(_)) if references < 8 => {
                    let field = Field::Reference {
                        edge: references,
                        target: reference_type(&reference, names)?,
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
            linked: LinkedMembers::default(),
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
    fn integer_slots_follow_classfile_order() -> Result<()> {
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
