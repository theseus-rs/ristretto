use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::{Reference, Value};
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/security/provider/NativeSeedGenerator.nativeGenerateSeed([B)Z",
    Any
)]
#[async_method]
pub async fn native_generate_seed<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let Some(rand_array) = parameters.pop_reference()? else {
        return Ok(Some(Value::Int(0)));
    };
    let mut guard = rand_array.write();
    let Reference::ByteArray(ref mut bytes) = *guard else {
        return Ok(Some(Value::Int(0)));
    };
    let mut buf = vec![0u8; bytes.len()];
    if getrandom::fill(&mut buf).is_err() {
        return Ok(Some(Value::Int(0)));
    }
    for (dst, src) in bytes.iter_mut().zip(buf.iter()) {
        *dst = (*src).cast_signed();
    }
    Ok(Some(Value::Int(1)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_native_generate_seed() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let arr: Box<[i8]> = vec![0i8; 32].into_boxed_slice();
        let value = Value::new_object(
            thread.vm().expect("vm").garbage_collector(),
            Reference::ByteArray(arr),
        );
        let result = native_generate_seed(thread, Parameters::new(vec![value]))
            .await
            .expect("seed");
        assert_eq!(Some(Value::Int(1)), result);
    }
}
