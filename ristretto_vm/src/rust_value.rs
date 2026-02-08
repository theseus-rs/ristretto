use crate::java_object::JavaObject;
use crate::thread::Thread;
use crate::{Result, RustValue, Value};

const STRING_PREFIX: &str = "str:";

/// Convert a vector of Rust values to a vector of `Value`. Rust value
pub async fn process_values(thread: &Thread, values: &[impl RustValue]) -> Result<Vec<Value>> {
    let mut results = Vec::with_capacity(values.len());
    let vm = thread.vm()?;
    for value in values {
        let value = value.to_value(vm.garbage_collector());

        // Extract string_value before await to avoid holding guard across await point
        let string_value_opt = {
            if matches!(value, Value::Object(Some(_)))
                && let Ok(object) = value.as_object_ref()
            {
                let class_name = object.class().name();
                if class_name.starts_with(STRING_PREFIX) {
                    Some(
                        class_name
                            .strip_prefix(STRING_PREFIX)
                            .unwrap_or_default()
                            .to_string(),
                    )
                } else {
                    None
                }
            } else {
                None
            }
        };

        if let Some(string_value) = string_value_opt {
            let value = string_value.to_object(thread).await?;
            results.push(value);
        } else {
            results.push(value);
        }
    }
    Ok(results)
}
