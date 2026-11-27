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
            results.push(value.try_to_value_async(garbage_collector).await?);
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
        assert_eq!(process_values(&thread, &empty).await?, [] as [Value; 0]);

        let strings = ["hello"];
        let values = process_values(&thread, &strings).await?;
        let value = values.first().expect("processed value");
        assert_eq!("hello", value.as_string()?);

        let processed_again = process_values(&thread, &values).await?;
        assert_eq!(values, processed_again);

        let vectors = [vec![0x81_u8; 65_537]];
        let arguments = process_values(&thread, &vectors).await?;
        assert!(
            arguments
                .first()
                .expect("vector argument")
                .as_byte_vec_ref()?
                .iter()
                .all(|&byte| byte == -127)
        );
        assert!(
            vectors
                .first()
                .expect("input vector")
                .iter()
                .all(|&byte| byte == 0x81)
        );
        Ok(())
    }
}
