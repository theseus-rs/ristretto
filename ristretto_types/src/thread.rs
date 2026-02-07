use crate::Result;
use crate::frame::Frame;
use ristretto_classloader::{Class, Method, Value};
use std::sync::Arc;
use std::time::Duration;

/// Trait representing a VM thread interface.
pub trait Thread: Send + Sync {
    /// The concrete VM type.
    type Vm: crate::VM;

    /// The concrete stack frame type.
    type Frame: Frame;

    /// Get the identifier of the thread.
    fn id(&self) -> u64;

    /// Get the virtual machine that owns the thread.
    ///
    /// # Errors
    /// Returns an error if the VM cannot be accessed.
    fn vm(&self) -> Result<Arc<Self::Vm>>;

    /// Get the name of the thread.
    fn name(&self) -> crate::BoxFuture<'_, String>;

    /// Set the name of the thread.
    fn set_name<'a>(&'a self, name: &'a str) -> crate::BoxFuture<'a, ()>;

    /// Get the Java object associated with this thread.
    fn java_object(&self) -> crate::BoxFuture<'_, Value>;

    /// Set the Java object associated with this thread.
    fn set_java_object(&self, value: Value) -> crate::BoxFuture<'_, ()>;

    /// Get the call stack frames.
    ///
    /// # Errors
    /// Returns an error if the frames cannot be accessed.
    fn frames(&self) -> crate::BoxFuture<'_, Result<Vec<Arc<Self::Frame>>>>;

    /// Interrupt the thread.
    fn interrupt(&self);

    /// Check if the thread is interrupted.
    fn is_interrupted(&self, clear_interrupt: bool) -> bool;

    /// Sleep the thread for the specified duration.
    /// Returns `true` if the sleep was interrupted.
    fn sleep(&self, duration: Duration) -> crate::BoxFuture<'_, bool>;

    /// Park the thread.
    ///
    /// # Errors
    /// Returns an error if the thread cannot be parked.
    fn park(&self, is_absolute: bool, time: u64) -> crate::BoxFuture<'_, Result<()>>;

    /// Unpark the thread.
    fn unpark(&self);

    /// Load, link, and initialize a class by name.
    ///
    /// # Errors
    /// Returns an error if the class cannot be loaded.
    fn class<'a>(&'a self, class_name: &'a str) -> crate::BoxFuture<'a, Result<Arc<Class>>>;

    /// Load and link a class by name without initializing it.
    ///
    /// # Errors
    /// Returns an error if the class cannot be loaded.
    fn load_and_link_class<'a>(
        &'a self,
        class_name: &'a str,
    ) -> crate::BoxFuture<'a, Result<Arc<Class>>>;

    /// Register a class with the class loader.
    ///
    /// # Errors
    /// Returns an error if the class cannot be registered.
    fn register_class(&self, class: Arc<Class>) -> crate::BoxFuture<'_, Result<()>>;

    /// Invoke a method by class name and method signature.
    ///
    /// # Errors
    /// Returns an error if the method cannot be invoked.
    fn invoke<'a>(
        &'a self,
        class: &'a str,
        method: &'a str,
        parameters: &'a [Value],
    ) -> crate::BoxFuture<'a, Result<Option<Value>>>;

    /// Invoke a method, returning an error if no value is returned.
    ///
    /// # Errors
    /// Returns an error if the method cannot be invoked or returns no value.
    fn try_invoke<'a>(
        &'a self,
        class: &'a str,
        method: &'a str,
        parameters: &'a [Value],
    ) -> crate::BoxFuture<'a, Result<Value>>;

    /// Execute a method on a class.
    ///
    /// # Errors
    /// Returns an error if the method cannot be executed.
    fn execute<'a>(
        &'a self,
        class: &'a Arc<Class>,
        method: &'a Arc<Method>,
        parameters: &'a [Value],
    ) -> crate::BoxFuture<'a, Result<Option<Value>>>;

    /// Execute a method, returning an error if no value is returned.
    ///
    /// # Errors
    /// Returns an error if the method cannot be executed or returns no value.
    fn try_execute<'a>(
        &'a self,
        class: &'a Arc<Class>,
        method: &'a Arc<Method>,
        parameters: &'a [Value],
    ) -> crate::BoxFuture<'a, Result<Value>>;

    /// Create a new VM object by invoking a constructor.
    ///
    /// # Errors
    /// Returns an error if the object cannot be created.
    fn object<'a>(
        &'a self,
        class_name: &'a str,
        descriptor: &'a str,
        parameters: &'a [Value],
    ) -> crate::BoxFuture<'a, Result<Value>>;

    /// Intern a string, returning the associated Java String value.
    ///
    /// # Errors
    /// Returns an error if the string cannot be interned.
    fn intern_string<'a>(&'a self, string: &'a str) -> crate::BoxFuture<'a, Result<Value>>;
}

/// Blanket implementation of Thread for `Arc<T>` where `T: Thread`.
impl<T: Thread> Thread for Arc<T> {
    type Vm = T::Vm;
    type Frame = T::Frame;

    fn id(&self) -> u64 {
        (**self).id()
    }

    fn vm(&self) -> Result<Arc<Self::Vm>> {
        (**self).vm()
    }

    fn name(&self) -> crate::BoxFuture<'_, String> {
        (**self).name()
    }

    fn set_name<'a>(&'a self, name: &'a str) -> crate::BoxFuture<'a, ()> {
        (**self).set_name(name)
    }

    fn java_object(&self) -> crate::BoxFuture<'_, Value> {
        (**self).java_object()
    }

    fn set_java_object(&self, value: Value) -> crate::BoxFuture<'_, ()> {
        (**self).set_java_object(value)
    }

    fn frames(&self) -> crate::BoxFuture<'_, Result<Vec<Arc<Self::Frame>>>> {
        (**self).frames()
    }

    fn interrupt(&self) {
        (**self).interrupt();
    }

    fn is_interrupted(&self, clear_interrupt: bool) -> bool {
        (**self).is_interrupted(clear_interrupt)
    }

    fn sleep(&self, duration: Duration) -> crate::BoxFuture<'_, bool> {
        (**self).sleep(duration)
    }

    fn park(&self, is_absolute: bool, time: u64) -> crate::BoxFuture<'_, Result<()>> {
        (**self).park(is_absolute, time)
    }

    fn unpark(&self) {
        (**self).unpark();
    }

    fn class<'a>(&'a self, class_name: &'a str) -> crate::BoxFuture<'a, Result<Arc<Class>>> {
        (**self).class(class_name)
    }

    fn load_and_link_class<'a>(
        &'a self,
        class_name: &'a str,
    ) -> crate::BoxFuture<'a, Result<Arc<Class>>> {
        (**self).load_and_link_class(class_name)
    }

    fn register_class(&self, class: Arc<Class>) -> crate::BoxFuture<'_, Result<()>> {
        (**self).register_class(class)
    }

    fn invoke<'a>(
        &'a self,
        class: &'a str,
        method: &'a str,
        parameters: &'a [Value],
    ) -> crate::BoxFuture<'a, Result<Option<Value>>> {
        (**self).invoke(class, method, parameters)
    }

    fn try_invoke<'a>(
        &'a self,
        class: &'a str,
        method: &'a str,
        parameters: &'a [Value],
    ) -> crate::BoxFuture<'a, Result<Value>> {
        (**self).try_invoke(class, method, parameters)
    }

    fn execute<'a>(
        &'a self,
        class: &'a Arc<Class>,
        method: &'a Arc<Method>,
        parameters: &'a [Value],
    ) -> crate::BoxFuture<'a, Result<Option<Value>>> {
        (**self).execute(class, method, parameters)
    }

    fn try_execute<'a>(
        &'a self,
        class: &'a Arc<Class>,
        method: &'a Arc<Method>,
        parameters: &'a [Value],
    ) -> crate::BoxFuture<'a, Result<Value>> {
        (**self).try_execute(class, method, parameters)
    }

    fn object<'a>(
        &'a self,
        class_name: &'a str,
        descriptor: &'a str,
        parameters: &'a [Value],
    ) -> crate::BoxFuture<'a, Result<Value>> {
        (**self).object(class_name, descriptor, parameters)
    }

    fn intern_string<'a>(&'a self, string: &'a str) -> crate::BoxFuture<'a, Result<Value>> {
        (**self).intern_string(string)
    }
}
