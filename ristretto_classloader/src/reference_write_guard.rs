//! Narrow mutations retain the managed guard's barriers without rooting unrelated edges.
use crate::{Error::InvalidValueType, Reference, Result, Value};
use ristretto_gc::sync::RwLockWriteGuard;

/// Point operations on a managed reference. Reading through the guard remains
/// available for validation under the same lock. Unrestricted mutable borrows
/// still acquire ownership for the entire reference before it can be moved.
pub trait ReferenceWriteGuardExt {
    /// Replaces a resolved instance field without returning the discarded value.
    /// Callers remain responsible for Java access and assignment checks.
    ///
    /// # Errors
    /// Returns an error for a non-object receiver, invalid slot, or invalid referent.
    fn set_value_at_slot(&mut self, slot: usize, value: Value) -> Result<()>;

    /// Replaces a reference-array element without returning the discarded value.
    /// Callers remain responsible for Java array component assignment checks.
    ///
    /// # Errors
    /// Returns the unchanged input for a non-reference-array receiver or invalid index.
    fn set_array_element(&mut self, index: usize, value: Value) -> Result<(), Value>;

    /// Borrows one owning element, which can be moved independently of its array.
    /// Returns `None` for a non-reference-array receiver or invalid index.
    fn array_element_mut(&mut self, index: usize) -> Option<&mut Value>;
}

#[expect(
    unsafe_code,
    reason = "typed operations never expose unrooted managed edges"
)]
impl ReferenceWriteGuardExt for RwLockWriteGuard<'_, Reference> {
    fn set_value_at_slot(&mut self, slot: usize, value: Value) -> Result<()> {
        // SAFETY: the setter drops the previous field in place and exposes neither
        // the object nor its edges. Conditional referents retain Object's semantics.
        match unsafe { self.unrooted_mut() } {
            Reference::Object(object) => object.set_value_at_slot(slot, value),
            _ => Err(InvalidValueType("Expected object".to_owned())),
        }
    }

    fn set_array_element(&mut self, index: usize, value: Value) -> Result<(), Value> {
        // SAFETY: set drops the previous edge in place; on failure only the caller's
        // unchanged input is returned. The managed guard still publishes dirty cards.
        match unsafe { self.unrooted_mut() } {
            Reference::Array(array) => array.elements.set(index, value),
            _ => Err(value),
        }
    }

    fn array_element_mut(&mut self, index: usize) -> Option<&mut Value> {
        // SAFETY: get_mut roots exactly the selected value before exposing it, and
        // records it for demotion/publication at guard release, including unwinding.
        match unsafe { self.unrooted_mut() } {
            Reference::Array(array) => array.elements.get_mut(index),
            _ => None,
        }
    }
}
