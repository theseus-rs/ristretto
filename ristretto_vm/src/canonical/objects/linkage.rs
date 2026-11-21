//! Immutable per-owner targets shared by the interpreter and native dispatch.
use super::{Component, Error, Field, Instruction, Method, Program, ReferenceType, Result};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Default)]
pub(in crate::canonical) struct LinkedMembers {
    classes: HashMap<u16, usize>,
    fields: HashMap<u16, (usize, Field)>,
    pub(in crate::canonical) statics: HashMap<u16, (usize, crate::canonical::statics::StaticField)>,
    calls: HashMap<u16, CallTarget>,
    reference_arrays: HashMap<u16, Component>,
    pub(super) casts: HashMap<u16, ReferenceType>,
}

#[derive(Clone, Debug)]
pub(super) enum CallTarget {
    ObjectConstructor,
    Method { class: usize, method: Arc<Method> },
}

impl Program {
    pub(super) fn reference_component(&self, owner: usize, index: u16) -> Result<Component> {
        self.layout(owner)?
            .linked
            .reference_arrays
            .get(&index)
            .copied()
            .ok_or(Error::Boundary("unlinked reference array component"))
    }

    fn resolve_call(&self, owner: usize, index: u16) -> Result<CallTarget> {
        let (owner, name, descriptor) = self.member(owner, index, false)?;
        if owner == "java/lang/Object" && name == "<init>" && descriptor == "()V" {
            return Ok(CallTarget::ObjectConstructor);
        }
        let class = self.class_index(&owner)?;
        let method = self
            .layout(class)?
            .class
            .try_get_method(&name, &descriptor)?;
        Ok(CallTarget::Method { class, method })
    }

    pub(in crate::canonical) fn link_members(&mut self) -> Result<()> {
        use Instruction as I;
        let mut linked = Vec::with_capacity(self.classes.len());
        for (class, layout) in self.classes.iter().enumerate() {
            let mut members = LinkedMembers::default();
            for method in layout.class.methods() {
                for instruction in method.code() {
                    match instruction {
                        I::New(index) => {
                            if let std::collections::hash_map::Entry::Vacant(entry) =
                                members.classes.entry(*index)
                            {
                                entry.insert(self.resolve_class_constant(class, *index)?);
                            }
                        }
                        I::Anewarray(index) => {
                            if let std::collections::hash_map::Entry::Vacant(entry) =
                                members.reference_arrays.entry(*index)
                            {
                                entry.insert(self.resolve_reference_component(class, *index)?);
                            }
                        }
                        I::Checkcast(index) => {
                            if let std::collections::hash_map::Entry::Vacant(entry) =
                                members.casts.entry(*index)
                            {
                                entry.insert(self.resolve_cast_target(class, *index)?);
                            }
                        }
                        I::Getstatic(index) | I::Putstatic(index) => {
                            if let std::collections::hash_map::Entry::Vacant(entry) =
                                members.statics.entry(*index)
                            {
                                entry.insert(self.resolve_static_field(class, *index)?);
                            }
                        }
                        I::Getfield(index) | I::Putfield(index) => {
                            if let std::collections::hash_map::Entry::Vacant(entry) =
                                members.fields.entry(*index)
                            {
                                entry.insert(self.resolve_field(class, *index)?);
                            }
                        }
                        I::Invokestatic(index) | I::Invokespecial(index) => {
                            if let std::collections::hash_map::Entry::Vacant(entry) =
                                members.calls.entry(*index)
                            {
                                entry.insert(self.resolve_call(class, *index)?);
                            }
                        }
                        _ => {}
                    }
                }
            }
            linked.push(members);
        }
        for (layout, linked) in self.classes.iter_mut().zip(linked) {
            layout.linked = linked;
        }
        Ok(())
    }

    pub(super) fn class_constant(&self, owner: usize, index: u16) -> Result<usize> {
        self.layout(owner)?
            .linked
            .classes
            .get(&index)
            .copied()
            .ok_or_else(|| Error::Invalid("unlinked class constant".to_owned()))
    }

    pub(super) fn field(&self, owner: usize, index: u16) -> Result<(usize, Field)> {
        self.layout(owner)?
            .linked
            .fields
            .get(&index)
            .copied()
            .ok_or_else(|| Error::Invalid("unlinked field constant".to_owned()))
    }

    pub(super) fn call_target(&self, owner: usize, index: u16) -> Result<CallTarget> {
        self.layout(owner)?
            .linked
            .calls
            .get(&index)
            .cloned()
            .ok_or_else(|| Error::Invalid("unlinked call constant".to_owned()))
    }
}

#[cfg(test)]
#[path = "linkage_tests.rs"]
mod tests;
