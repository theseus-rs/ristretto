use crate::JavaError::ClassFormatError;
use crate::Result;
use crate::java_object::JavaObject;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::ClassFile;
use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::{Class, Value};
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::io::Cursor;
use std::sync::Arc;
use tracing::error;
use zerocopy::transmute_ref;

#[intrinsic_method(
    "java/lang/reflect/Proxy.defineClass0(Ljava/lang/ClassLoader;Ljava/lang/String;[BII)Ljava/lang/Class;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn define_class_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let length = parameters.pop_int()?;
    let offset = parameters.pop_int()?;
    let bytes = parameters.pop()?;
    let _name = parameters.pop()?;
    let class_loader = parameters.pop()?;

    let bytes = {
        let bytes = bytes.as_byte_vec_ref()?;
        let bytes: &[u8] = transmute_ref!(&*bytes);
        bytes.to_vec()
    };
    let offset = usize::try_from(offset)?;
    let length = usize::try_from(length)?;

    let mut cursor = Cursor::new(bytes[offset..offset + length].to_vec());
    let class_file = match ClassFile::from_bytes(&mut cursor) {
        Ok(class_file) => class_file,
        Err(e) => {
            error!("ClassFormatError in Proxy.defineClass0: {e}");
            return Err(ClassFormatError(e.to_string()).into());
        }
    };

    let class = Class::from(None, class_file)?;
    thread.register_class(class.clone()).await?;
    let class_object = class.to_object(&thread).await?;

    if !class_loader.is_null() {
        let mut object = class_object.as_object_mut()?;
        object.set_value("classLoader", class_loader)?;
    }

    Ok(Some(class_object))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_define_class_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        // Test with empty parameters should return an error (not panic)
        let result = define_class_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
