use crate::JavaError::{IllegalArgumentException, NullPointerException};
use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "java/io/ObjectOutputStream.doublesToBytes([DI[BII)V",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub(crate) async fn doubles_to_bytes(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let number_of_doubles = usize::try_from(parameters.pop_int()?)?;
    let destination_position = usize::try_from(parameters.pop_int()?)?;
    let Some(destination) = parameters.pop_reference()? else {
        return Err(NullPointerException(Some("destination cannot be null".to_string())).into());
    };
    let mut destination_guard = destination.write();
    let destination = destination_guard.as_byte_vec_mut()?;
    let source_position = usize::try_from(parameters.pop_int()?)?;
    let Some(source) = parameters.pop_reference()? else {
        return Err(NullPointerException(Some("source cannot be null".to_string())).into());
    };
    let source_guard = source.read();
    let source = source_guard.as_double_vec_ref()?;

    if source_position.saturating_add(number_of_doubles) > source.len() {
        return Err(IllegalArgumentException("source index out of bounds".into()).into());
    }
    if destination_position.saturating_sub(number_of_doubles.saturating_mul(8)) > destination.len()
    {
        return Err(IllegalArgumentException("destination index out of bounds".into()).into());
    }

    for i in 0..number_of_doubles {
        let value = source[source_position + i];
        let bytes = value.to_bits().to_be_bytes();
        let bytes: &[i8] = zerocopy::transmute_ref!(bytes.as_slice());
        let dest_offset = destination_position.saturating_add(i.saturating_mul(8));
        destination[dest_offset..dest_offset + 8].copy_from_slice(bytes);
    }
    Ok(None)
}

#[intrinsic_method(
    "java/io/ObjectOutputStream.floatsToBytes([FI[BII)V",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub(crate) async fn floats_to_bytes(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let number_of_floats = usize::try_from(parameters.pop_int()?)?;
    let destination_position = usize::try_from(parameters.pop_int()?)?;
    let Some(destination) = parameters.pop_reference()? else {
        return Err(NullPointerException(Some("destination cannot be null".to_string())).into());
    };
    let mut destination_guard = destination.write();
    let destination = destination_guard.as_byte_vec_mut()?;
    let source_position = usize::try_from(parameters.pop_int()?)?;
    let Some(source) = parameters.pop_reference()? else {
        return Err(NullPointerException(Some("source cannot be null".to_string())).into());
    };
    let source_guard = source.read();
    let source = source_guard.as_float_vec_ref()?;

    if source_position.saturating_add(number_of_floats) > source.len() {
        return Err(IllegalArgumentException("source index out of bounds".into()).into());
    }
    if destination_position.saturating_sub(number_of_floats.saturating_mul(8)) > destination.len() {
        return Err(IllegalArgumentException("destination index out of bounds".into()).into());
    }

    for i in 0..number_of_floats {
        let value = source[source_position + i];
        let bytes = value.to_bits().to_be_bytes();
        let bytes: &[i8] = zerocopy::transmute_ref!(bytes.as_slice());
        let dest_offset = destination_position.saturating_add(i.saturating_mul(4));
        destination[dest_offset..dest_offset + 4].copy_from_slice(bytes);
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_doubles_to_bytes() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut parameters = Parameters::default();
        let source = Value::from(vec![3.0f64, 42.0f64]);
        let destination = Value::from(vec![0i8; 16]); // 2 doubles * 8 bytes each
        parameters.push(source);
        parameters.push_int(0); // source position
        parameters.push(destination.clone());
        parameters.push_int(0); // destination position
        parameters.push_int(2); // number of doubles

        let _ = doubles_to_bytes(thread, parameters).await?;
        let bytes = destination.as_byte_vec_ref()?;
        assert_eq!(
            &*bytes,
            vec![64, 8, 0, 0, 0, 0, 0, 0, 64, 69, 0, 0, 0, 0, 0, 0]
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_floats_to_bytes() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut parameters = Parameters::default();
        let source = Value::from(vec![3.0f32, 42.0f32]);
        let destination = Value::from(vec![0i8; 8]); // 2 floats * 4 bytes each
        parameters.push(source);
        parameters.push_int(0); // source position
        parameters.push(destination.clone());
        parameters.push_int(0); // destination position
        parameters.push_int(2); // number of floats

        let _ = floats_to_bytes(thread, parameters).await?;
        let bytes = destination.as_byte_vec_ref()?;
        assert_eq!(&*bytes, vec![64, 64, 0, 0, 66, 40, 0, 0]);
        Ok(())
    }
}
