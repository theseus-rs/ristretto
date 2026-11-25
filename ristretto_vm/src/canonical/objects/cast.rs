//! Closed-layout casts preserve the owning reference and actual array component.
use super::{
    Component, Error, Execution, FieldType, Program, ReferenceType, Value, reference_type,
};

use ristretto_classfile::{ClassAccessFlags, attributes::Instruction};
use std::collections::HashMap;

impl Program {
    pub(in crate::canonical) fn link_cast_targets(&mut self) -> Result<(), Error> {
        let mut linked = Vec::with_capacity(self.classes.len());
        for owner in 0..self.classes.len() {
            let mut casts = HashMap::new();
            for method in self.layout(owner)?.class.methods() {
                for op in method.code() {
                    if let Instruction::Checkcast(index) = op
                        && let std::collections::hash_map::Entry::Vacant(entry) =
                            casts.entry(*index)
                    {
                        entry.insert(self.resolve_cast_target(owner, *index)?);
                    }
                }
            }
            linked.push(casts);
        }
        for (layout, casts) in self.classes.iter_mut().zip(linked) {
            layout.casts = casts;
        }
        Ok(())
    }

    pub(super) fn resolve_cast_target(
        &self,
        owner: usize,
        index: u16,
    ) -> Result<ReferenceType, Error> {
        let name = self
            .layout(owner)?
            .class
            .constant_pool()
            .try_get_class(index)?;
        let field = if name.as_bytes().first() == Some(&b'[') {
            FieldType::parse_java_str(name)?
        } else {
            FieldType::Object(name.to_owned())
        };
        let target = reference_type(&field, &self.names)?;
        match target {
            ReferenceType::Object(class)
            | ReferenceType::ReferenceArray(Component::Class(class)) => {
                self.admit_class_access(owner, class)?;
            }
            _ => {}
        }
        Ok(target)
    }

    pub(super) fn admit_class_access(&self, owner: usize, target: usize) -> Result<(), Error> {
        let current = self.layout(owner)?.class.name();
        let target = &self.layout(target)?.class;
        if target
            .class_file()
            .access_flags
            .contains(ClassAccessFlags::PUBLIC)
            || current.rsplit_once('/').map(|(package, _)| package)
                == target.name().rsplit_once('/').map(|(package, _)| package)
        {
            Ok(())
        } else {
            Err(Error::Unsupported(
                "inaccessible canonical class reference".to_owned(),
            ))
        }
    }

    pub(super) fn cast_target(&self, owner: usize, index: u16) -> Result<ReferenceType, Error> {
        self.layout(owner)?
            .casts
            .get(&index)
            .copied()
            .ok_or(Error::Boundary("unlinked cast target"))
    }
}

impl Execution<'_, '_, '_> {
    pub(super) fn cast_reference(
        &self,
        value: Value,
        target: ReferenceType,
    ) -> Result<Value, Error> {
        let reference = value.reference()?;
        // Check public payload kind before AnyObject can accept it. Hidden
        // pages and foreign roots must fail even for the broadest target.
        if let Some(root) = &reference
            && !target.accepts(self.reference_kind(root)?)
        {
            return Err(Error::TypeFault("ClassCastException"));
        }
        Ok(Value::Reference(reference))
    }
}

#[cfg(test)]
#[path = "cast_tests.rs"]
mod tests;
