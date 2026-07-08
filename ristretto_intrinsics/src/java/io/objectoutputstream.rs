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
    "java/io/ObjectOutputStream.doublesToBytes([DI[BII)V",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn doubles_to_bytes<T: Thread + 'static>(
    _thread: Arc<T>,
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
        let source_index = source_position
            .checked_add(i)
            .ok_or_else(|| IllegalArgumentException("source index overflow".into()))?;
        let value = *bounds::index(source, source_index, "doublesToBytes source")?;
        let bytes = value.to_bits().to_be_bytes();
        let bytes: &[i8] = zerocopy::transmute_ref!(bytes.as_slice());
        let byte_offset = i
            .checked_mul(8)
            .ok_or_else(|| IllegalArgumentException("destination index overflow".into()))?;
        let dest_offset = destination_position
            .checked_add(byte_offset)
            .ok_or_else(|| IllegalArgumentException("destination index overflow".into()))?;
        let dest_end = dest_offset
            .checked_add(8)
            .ok_or_else(|| IllegalArgumentException("destination index overflow".into()))?;
        bounds::range_mut(
            destination,
            dest_offset..dest_end,
            "doublesToBytes destination",
        )?
        .copy_from_slice(bytes);
    }
    Ok(None)
}

#[intrinsic_method(
    "java/io/ObjectOutputStream.floatsToBytes([FI[BII)V",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn floats_to_bytes<T: Thread + 'static>(
    _thread: Arc<T>,
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
        let source_index = source_position
            .checked_add(i)
            .ok_or_else(|| IllegalArgumentException("source index overflow".into()))?;
        let value = *bounds::index(source, source_index, "floatsToBytes source")?;
        let bytes = value.to_bits().to_be_bytes();
        let bytes: &[i8] = zerocopy::transmute_ref!(bytes.as_slice());
        let byte_offset = i
            .checked_mul(4)
            .ok_or_else(|| IllegalArgumentException("destination index overflow".into()))?;
        let dest_offset = destination_position
            .checked_add(byte_offset)
            .ok_or_else(|| IllegalArgumentException("destination index overflow".into()))?;
        let dest_end = dest_offset
            .checked_add(4)
            .ok_or_else(|| IllegalArgumentException("destination index overflow".into()))?;
        bounds::range_mut(
            destination,
            dest_offset..dest_end,
            "floatsToBytes destination",
        )?
        .copy_from_slice(bytes);
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ristretto_classloader::Reference;

    #[tokio::test]
    async fn test_doubles_to_bytes() -> Result<()> {
        let (_vm, thread) = crate::test::java11_thread().await.expect("thread");
        let mut parameters = Parameters::default();
        let source = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![3.0f64, 42.0f64]),
        );
        let destination = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![0i8; 16]),
        ); // 2 doubles * 8 bytes each
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
        let (_vm, thread) = crate::test::java11_thread().await.expect("thread");
        let mut parameters = Parameters::default();
        let source = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![3.0f32, 42.0f32]),
        );
        let destination = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![0i8; 8]),
        ); // 2 floats * 4 bytes each
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
