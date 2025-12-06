use crate::JavaError::{IllegalArgumentException, NullPointerException};
use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "java/io/ObjectInputStream.bytesToDoubles([BI[DII)V",
    LessThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn bytes_to_doubles(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let number_of_doubles = usize::try_from(parameters.pop_int()?)?;
    let destination_position = usize::try_from(parameters.pop_int()?)?;
    let Some(destination) = parameters.pop_reference()? else {
        return Err(NullPointerException("destination cannot be null".into()).into());
    };
    let mut destination_guard = destination.write();
    let destination = destination_guard.as_double_vec_mut()?;
    let source_position = usize::try_from(parameters.pop_int()?)?;
    let Some(source) = parameters.pop_reference()? else {
        return Err(NullPointerException("source cannot be null".into()).into());
    };
    let source_guard = source.read();
    let source = source_guard.as_byte_vec_ref()?;

    if source_position.saturating_add(number_of_doubles) > source.len() {
        return Err(IllegalArgumentException("source index out of bounds".into()).into());
    }
    if destination_position.saturating_sub(number_of_doubles.saturating_mul(8)) > destination.len()
    {
        return Err(IllegalArgumentException("destination index out of bounds".into()).into());
    }

    for i in 0..number_of_doubles {
        let start = source_position.saturating_add(i.saturating_mul(8));
        let end = start + 8;
        let mut bytes = [0i8; 8];
        bytes.copy_from_slice(&source[start..end]);
        let bytes: &[u8; 8] = zerocopy::transmute_ref!(&bytes);
        let value = f64::from_be_bytes(*bytes);
        destination[destination_position + i] = value;
    }
    Ok(None)
}

#[intrinsic_method(
    "java/io/ObjectInputStream.bytesToFloats([BI[FII)V",
    LessThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn bytes_to_floats(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let number_of_floats = usize::try_from(parameters.pop_int()?)?;
    let destination_position = usize::try_from(parameters.pop_int()?)?;
    let Some(destination) = parameters.pop_reference()? else {
        return Err(NullPointerException("destination cannot be null".into()).into());
    };
    let mut destination_guard = destination.write();
    let destination = destination_guard.as_float_vec_mut()?;
    let source_position = usize::try_from(parameters.pop_int()?)?;
    let Some(source) = parameters.pop_reference()? else {
        return Err(NullPointerException("source cannot be null".into()).into());
    };
    let source_guard = source.read();
    let source = source_guard.as_byte_vec_ref()?;

    if source_position.saturating_add(number_of_floats) > source.len() {
        return Err(IllegalArgumentException("source index out of bounds".into()).into());
    }
    if destination_position.saturating_sub(number_of_floats.saturating_mul(8)) > destination.len() {
        return Err(IllegalArgumentException("destination index out of bounds".into()).into());
    }

    for i in 0..number_of_floats {
        let start = source_position.saturating_add(i.saturating_mul(4));
        let end = start + 4;
        let mut bytes = [0i8; 4];
        bytes.copy_from_slice(&source[start..end]);
        let bytes: &[u8; 4] = zerocopy::transmute_ref!(&bytes);
        let value = f32::from_be_bytes(*bytes);
        destination[destination_position + i] = value;
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_bytes_to_doubles() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut parameters = Parameters::default();
        let bytes: Vec<i8> = vec![64, 8, 0, 0, 0, 0, 0, 0, 64, 69, 0, 0, 0, 0, 0, 0];
        let source = Value::from(bytes);
        let destination = Value::from(vec![0f64; 2]);
        parameters.push(source);
        parameters.push_int(0); // source position
        parameters.push(destination.clone());
        parameters.push_int(0); // destination position
        parameters.push_int(2); // number of doubles

        let _ = bytes_to_doubles(thread, parameters).await?;
        let bytes = destination.as_double_vec_ref()?;
        assert_eq!(&*bytes, vec![3.0f64, 42.0f64]);
        Ok(())
    }

    #[tokio::test]
    async fn test_bytes_to_floats() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut parameters = Parameters::default();
        let bytes: Vec<i8> = vec![64, 64, 0, 0, 66, 40, 0, 0];
        let source = Value::from(bytes);
        let destination = Value::from(vec![0f32; 2]);
        parameters.push(source);
        parameters.push_int(0); // source position
        parameters.push(destination.clone());
        parameters.push_int(0); // destination position
        parameters.push_int(2); // number of floats

        let _ = bytes_to_floats(thread, parameters).await?;
        let bytes = destination.as_float_vec_ref()?;
        assert_eq!(&*bytes, vec![3.0f32, 42.0f32]);
        Ok(())
    }
}
