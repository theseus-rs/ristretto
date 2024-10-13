use crate::call_stack::CallStack;
use crate::native::{java_lang_class, java_lang_object, java_lang_shutdown, java_lang_system};
use crate::Error::MethodNotFound;
use crate::{Result, VM};
use lazy_static::lazy_static;
use ristretto_classloader::Value;
use std::collections::HashMap;

lazy_static! {
    pub static ref REGISTRY: NativeRegistry = NativeRegistry::default();
}

/// A native method is a method that is implemented in Rust and is called from Java code where the
/// method is marked as `native`.
pub type NativeMethod =
    fn(vm: &VM, call_stack: &CallStack, arguments: Vec<Value>) -> Result<Option<Value>>;

#[expect(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct NativeRegistry {
    methods: HashMap<String, NativeMethod>,
}

impl NativeRegistry {
    /// Create a new native registry.
    #[must_use]
    pub fn new() -> Self {
        NativeRegistry {
            methods: HashMap::new(),
        }
    }

    /// Register a new native method.
    pub fn register(
        &mut self,
        class_name: &str,
        method_name: &str,
        method_descriptor: &str,
        method: NativeMethod,
    ) {
        self.methods.insert(
            format!("{class_name}.{method_name}{method_descriptor}"),
            method,
        );
    }

    /// Get a native method by class and method name.
    ///
    /// # Errors
    /// if the method is not found.
    pub fn get(
        &self,
        class_name: &str,
        method_name: &str,
        method_descriptor: &str,
    ) -> Result<&NativeMethod> {
        let native_method_signature = format!("{class_name}.{method_name}{method_descriptor}");
        let Some(method) = self.methods.get(&native_method_signature) else {
            return Err(MethodNotFound {
                class_name: class_name.to_string(),
                method_name: method_name.to_string(),
                method_descriptor: method_descriptor.to_string(),
            });
        };
        Ok(method)
    }
}

impl Default for NativeRegistry {
    fn default() -> Self {
        let mut registry = NativeRegistry::new();
        java_lang_class::register(&mut registry);
        java_lang_object::register(&mut registry);
        java_lang_system::register(&mut registry);
        java_lang_shutdown::register(&mut registry);
        registry
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = NativeRegistry::new();
        let method: NativeMethod = |_, _, _| Ok(None);
        registry.register("java.lang.Object", "hashCode", "()I", method);
        assert_eq!(registry.methods.len(), 1);
    }

    #[test]
    fn test_get() {
        let mut registry = NativeRegistry::new();
        let method: NativeMethod = |_, _, _| Ok(None);
        registry.register("java.lang.Object", "hashCode", "()I", method);

        let result = registry.get("java.lang.Object", "hashCode", "()I");
        assert!(result.is_ok());

        let result = registry.get("foo", "hashCode", "()I");
        assert!(result.is_err());
    }
}
