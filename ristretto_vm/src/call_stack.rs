use crate::arguments::Arguments;
use crate::{native_methods, Frame, Result, VM};
use ristretto_classloader::Error::MethodNotFound;
use ristretto_classloader::{Class, Method, Value};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use tracing::{debug, event_enabled, Level};

/// A call stack is a stack of frames that are executed in order.
#[derive(Debug, Default)]
pub struct CallStack {
    pub(crate) frames: Vec<Rc<RefCell<Frame>>>,
}

impl CallStack {
    /// Create a new call stack.
    #[must_use]
    pub fn new() -> Self {
        CallStack { frames: Vec::new() }
    }

    /// Add a new frame to the call stack and invoke the method. To invoke a method on an object
    /// reference, the object reference must be the first argument in the arguments vector.
    ///
    /// # Errors
    /// if the method cannot be invoked.
    pub fn execute(
        &mut self,
        vm: &VM,
        class: &Arc<Class>,
        method: &Arc<Method>,
        arguments: Vec<Value>,
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
            let result = rust_method(vm, self, arguments);
            (result, false)
        } else if method.is_native() {
            return Err(MethodNotFound {
                class_name: class_name.to_string(),
                method_name: method_name.to_string(),
                method_descriptor: method_descriptor.to_string(),
            }
            .into());
        } else {
            let arguments = CallStack::adjust_arguments(arguments);
            let frame = Rc::new(RefCell::new(Frame::new(class, method, arguments)?));
            self.frames.push(frame.clone());
            let mut frame = frame.borrow_mut();
            let result = frame.execute(vm, self);
            (result, true)
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
            self.frames.pop();
        }

        match result {
            Ok(result) => Ok(result),
            Err(error) => {
                // TODO: Handle exceptions
                Err(error)
            }
        }
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
        let adjusted_arguments = CallStack::adjust_arguments(arguments);
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
