use crate::arguments::Arguments;
use crate::Error::InternalError;
use crate::{native_methods, Frame, Result, VM};
use ristretto_classloader::Error::MethodNotFound;
use ristretto_classloader::{Class, Method, Value};
use std::sync::{Arc, Weak};
use tokio::sync::RwLock;
use tracing::{debug, event_enabled, Level};

/// A thread is a single sequential flow of control within a program. It has its own call stack
/// and program counter.
///
/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-2.html#jvms-2.5.2>
#[expect(clippy::struct_field_names)]
#[derive(Debug)]
pub struct Thread {
    vm: Weak<VM>,
    thread: Weak<Thread>,
    name: Arc<RwLock<String>>,
    frames: Arc<RwLock<Vec<Arc<Frame>>>>,
}

impl Thread {
    /// Create a new thread.
    #[must_use]
    pub fn new(vm: &Weak<VM>) -> Arc<Self> {
        Arc::new_cyclic(|thread| Thread {
            vm: vm.clone(),
            thread: thread.clone(),
            name: Arc::new(RwLock::new(String::new())),
            frames: Arc::new(RwLock::new(Vec::new())),
        })
    }

    /// Get the virtual machine that owns the thread.
    ///
    /// # Errors
    /// if the virtual machine cannot be accessed.
    pub fn vm(&self) -> Result<Arc<VM>> {
        match self.vm.upgrade() {
            Some(vm) => Ok(vm),
            None => Err(InternalError("VM is not available".to_string())),
        }
    }

    /// Get the name of the thread.
    pub async fn name(&self) -> String {
        let name = self.name.read().await;
        name.clone()
    }

    /// Set the name of the thread.
    pub async fn set_name<S: AsRef<str>>(&self, name: S) {
        let new_name = name.as_ref();
        let mut name = self.name.write().await;
        *name = new_name.to_string();
    }

    /// Get the frames in the thread.
    ///
    /// # Errors
    /// if the frames cannot be accessed.
    pub async fn frames(&self) -> Result<Vec<Arc<Frame>>> {
        let frames = self.frames.read().await;
        Ok(frames.clone())
    }

    /// Add a new frame to the thread and invoke the method. To invoke a method on an object
    /// reference, the object reference must be the first argument in the arguments vector.
    ///
    /// # Errors
    /// if the method cannot be invoked.
    pub async fn execute(
        &self,
        class: &Arc<Class>,
        method: &Arc<Method>,
        arguments: Vec<Value>,
        remove_frame: bool,
    ) -> Result<Option<Value>> {
        let class_name = class.name();
        let method_name = method.name();
        let method_descriptor = method.descriptor();

        if event_enabled!(Level::DEBUG) {
            let access_flags = method.access_flags();
            debug!("execute: {class_name}.{method_name}{method_descriptor} {access_flags}");
        }

        let registry = native_methods::registry();
        let rust_method = registry.get(class_name, method_name, method_descriptor);

        let (result, frame_added) = if let Some(rust_method) = rust_method {
            let arguments = Arguments::new(arguments);
            let Some(thread) = self.thread.upgrade() else {
                return Err(InternalError("Call stack is not available".to_string()));
            };
            let result = rust_method(thread, arguments).await;
            (result, false)
        } else if method.is_native() {
            return Err(MethodNotFound {
                class_name: class_name.to_string(),
                method_name: method_name.to_string(),
                method_descriptor: method_descriptor.to_string(),
            }
            .into());
        } else {
            let arguments = Thread::adjust_arguments(arguments);
            let frame = Arc::new(Frame::new(&self.thread, class, method, arguments)?);

            // Limit the scope of the write lock to just adding the frame to the thread. This
            // is necessary because the thread is re-entrant.
            {
                let mut frames = self.frames.write().await;
                frames.push(frame.clone());
            }
            let result = frame.execute().await;
            (result, remove_frame)
        };

        if event_enabled!(Level::DEBUG) {
            match &result {
                Ok(Some(value)) => {
                    let value = value.to_string();
                    if value.len() > 100 {
                        debug!("result: {}...", &value.as_str()[..97]);
                    } else {
                        debug!("result: {value}");
                    }
                }
                Ok(None) => {}
                Err(error) => {
                    debug!("error: {error}");
                }
            }
        }

        if frame_added {
            let mut frames = self.frames.write().await;
            frames.pop();
        }

        result
    }

    /// The JVM specification requires that Long and Double take two places in the arguments list
    /// when passed to a method. This method adjusts the arguments list to account for this.
    ///
    /// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-2.html#jvms-2.6.1>
    fn adjust_arguments(mut arguments: Vec<Value>) -> Vec<Value> {
        let mut index = arguments.len();
        while index > 0 {
            index -= 1;
            match &arguments[index] {
                Value::Long(_) | Value::Double(_) => {
                    arguments.insert(index + 1, Value::Unused);
                }
                _ => {}
            }
        }
        arguments
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ristretto_classloader::Value;

    #[test]
    fn test_adjust_arguments() {
        let arguments = vec![
            Value::Int(1),
            Value::Long(2),
            Value::Float(3.0),
            Value::Double(4.0),
        ];
        let adjusted_arguments = Thread::adjust_arguments(arguments);
        assert_eq!(
            adjusted_arguments,
            vec![
                Value::Int(1),
                Value::Long(2),
                Value::Unused,
                Value::Float(3.0),
                Value::Double(4.0),
                Value::Unused,
            ]
        );
    }
}
