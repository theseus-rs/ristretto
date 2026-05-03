use crate::java_object::JavaObject;
use crate::thread::Thread;
use crate::{Result, RustValue, Value};

/// Convert a vector of Rust values to a vector of `Value`. Rust value
pub async fn process_values(thread: &Thread, values: &[impl RustValue]) -> Result<Vec<Value>> {
    if values.is_empty() {
        return Ok(Vec::new());
    }

    let mut results = Vec::with_capacity(values.len());
    let vm = thread.vm()?;
    let garbage_collector = vm.garbage_collector();
    for value in values {
        if let Some(string_value) = value.as_rust_string() {
            let value = string_value.to_object(thread).await?;
            results.push(value);
        } else {
            results.push(value.to_value(garbage_collector));
        }
    }
    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_process_values() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");

        let empty: [Value; 0] = [];
        assert!(process_values(&thread, &empty).await?.is_empty());

        let strings = ["hello"];
        let values = process_values(&thread, &strings).await?;
        let value = values.first().expect("processed value");
        assert_eq!("hello", value.as_string()?);

        let processed_again = process_values(&thread, &values).await?;
        assert_eq!(values, processed_again);
        Ok(())
    }
}
