use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::{Reference, Value};
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::VM;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "jdk/internal/jimage/NativeImageBuffer.getNativeMap(Ljava/lang/String;)Ljava/nio/ByteBuffer;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_native_map<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let name = parameters.pop()?;
    let path = name.as_string()?;

    let bytes = std::fs::read(&path)
        .map_err(|e| JavaError::IoException(format!("Failed to read file '{path}': {e}")))?;

    #[expect(clippy::cast_possible_wrap)]
    let bytes: Vec<i8> = bytes.into_iter().map(|b| b as i8).collect();

    let vm = thread.vm()?;
    let collector = vm.garbage_collector();
    let byte_array = Value::new_object(collector, Reference::from(bytes));
    let result = thread
        .try_invoke(
            "java.nio.ByteBuffer",
            "wrap([B)Ljava/nio/ByteBuffer;",
            &[byte_array],
        )
        .await?;
    Ok(Some(result))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ristretto_types::JavaObject;
    use std::io::Write;

    fn new_named_tempfile() -> std::io::Result<tempfile::NamedTempFile> {
        ristretto_test_util::init_wasi_tempdir();
        tempfile::NamedTempFile::new()
    }
    #[tokio::test]
    async fn test_get_native_map() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;

        // Create a temporary file with known content
        let mut temp_file = new_named_tempfile().expect("create temp file");
        let content = b"Hello, jimage!";
        temp_file.write_all(content).expect("write temp file");

        let path = temp_file.path().to_string_lossy().to_string();
        let name = path.to_object(&thread).await?;
        let parameters = Parameters::new(vec![name]);

        let result = get_native_map(thread, parameters).await?;
        assert!(result.is_some());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_native_map_file_not_found() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;

        let name = "/nonexistent/path/to/file".to_object(&thread).await?;
        let parameters = Parameters::new(vec![name]);

        let result = get_native_map(thread, parameters).await;
        assert!(result.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_native_map_empty_parameters() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_native_map(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
