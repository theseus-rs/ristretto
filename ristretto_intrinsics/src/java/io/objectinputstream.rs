use crate::bounds;
use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError::{IllegalArgumentException, NullPointerException};
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "java/io/ObjectInputStream.bytesToDoubles([BI[DII)V",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn bytes_to_doubles<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let number_of_doubles = usize::try_from(parameters.pop_int()?)?;
    let destination_position = usize::try_from(parameters.pop_int()?)?;
    let Some(destination) = parameters.pop_reference()? else {
        return Err(NullPointerException(Some("destination cannot be null".to_string())).into());
    };
    let mut destination_guard = destination.write();
    let destination = destination_guard.as_double_vec_mut()?;
    let source_position = usize::try_from(parameters.pop_int()?)?;
    let Some(source) = parameters.pop_reference()? else {
        return Err(NullPointerException(Some("source cannot be null".to_string())).into());
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
        let byte_offset = i
            .checked_mul(8)
            .ok_or_else(|| IllegalArgumentException("source index overflow".into()))?;
        let start = source_position
            .checked_add(byte_offset)
            .ok_or_else(|| IllegalArgumentException("source index overflow".into()))?;
        let end = start
            .checked_add(8)
            .ok_or_else(|| IllegalArgumentException("source index overflow".into()))?;
        let mut bytes = [0i8; 8];
        bytes.copy_from_slice(bounds::range(source, start..end, "bytesToDoubles source")?);
        let bytes: &[u8; 8] = zerocopy::transmute_ref!(&bytes);
        let value = f64::from_be_bytes(*bytes);
        let destination_index = destination_position
            .checked_add(i)
            .ok_or_else(|| IllegalArgumentException("destination index overflow".into()))?;
        *bounds::index_mut(destination, destination_index, "bytesToDoubles destination")? = value;
    }
    Ok(None)
}

#[intrinsic_method(
    "java/io/ObjectInputStream.bytesToFloats([BI[FII)V",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn bytes_to_floats<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let number_of_floats = usize::try_from(parameters.pop_int()?)?;
    let destination_position = usize::try_from(parameters.pop_int()?)?;
    let Some(destination) = parameters.pop_reference()? else {
        return Err(NullPointerException(Some("destination cannot be null".to_string())).into());
    };
    let mut destination_guard = destination.write();
    let destination = destination_guard.as_float_vec_mut()?;
    let source_position = usize::try_from(parameters.pop_int()?)?;
    let Some(source) = parameters.pop_reference()? else {
        return Err(NullPointerException(Some("source cannot be null".to_string())).into());
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
        let byte_offset = i
            .checked_mul(4)
            .ok_or_else(|| IllegalArgumentException("source index overflow".into()))?;
        let start = source_position
            .checked_add(byte_offset)
            .ok_or_else(|| IllegalArgumentException("source index overflow".into()))?;
        let end = start
            .checked_add(4)
            .ok_or_else(|| IllegalArgumentException("source index overflow".into()))?;
        let mut bytes = [0i8; 4];
        bytes.copy_from_slice(bounds::range(source, start..end, "bytesToFloats source")?);
        let bytes: &[u8; 4] = zerocopy::transmute_ref!(&bytes);
        let value = f32::from_be_bytes(*bytes);
        let destination_index = destination_position
            .checked_add(i)
            .ok_or_else(|| IllegalArgumentException("destination index overflow".into()))?;
        *bounds::index_mut(destination, destination_index, "bytesToFloats destination")? = value;
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ristretto_classloader::Reference;

    #[tokio::test]
    async fn test_bytes_to_doubles() -> Result<()> {
        let (_vm, thread) = crate::test::java11_thread().await.expect("thread");
        let mut parameters = Parameters::default();
        let bytes: Vec<i8> = vec![64, 8, 0, 0, 0, 0, 0, 0, 64, 69, 0, 0, 0, 0, 0, 0];
        let source = Value::new_object(thread.vm()?.garbage_collector(), Reference::from(bytes));
        let destination = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![0f64; 2]),
        );
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
        let (_vm, thread) = crate::test::java11_thread().await.expect("thread");
        let mut parameters = Parameters::default();
        let bytes: Vec<i8> = vec![64, 64, 0, 0, 66, 40, 0, 0];
        let source = Value::new_object(thread.vm()?.garbage_collector(), Reference::from(bytes));
        let destination = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![0f32; 2]),
        );
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
