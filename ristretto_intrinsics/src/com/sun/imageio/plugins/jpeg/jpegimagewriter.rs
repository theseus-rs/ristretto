use super::codec::Config;
use super::support::{self, WRITER};
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("com/sun/imageio/plugins/jpeg/JPEGImageWriter.abortWrite(J)V", Any)]
#[async_method]
pub async fn abort_write<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let id = parameters.pop_long()?;
    support::context(thread.as_ref(), id, true)
        .await?
        .abort(true);
    Ok(None)
}

#[intrinsic_method("com/sun/imageio/plugins/jpeg/JPEGImageWriter.disposeWriter(J)V", Any)]
#[async_method]
pub async fn dispose_writer<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let id = parameters.pop_long()?;
    let state = support::state(thread.as_ref())?;
    let mut contexts = state.contexts.write();
    if contexts.get(&id).is_some_and(|context| context.writer) {
        contexts.remove(&id);
    }
    Ok(None)
}

#[intrinsic_method(
    "com/sun/imageio/plugins/jpeg/JPEGImageWriter.initJPEGImageWriter()J",
    Any
)]
#[async_method]
pub async fn init_jpeg_image_writer<T: Thread + 'static>(
    thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Long(support::create(thread.as_ref(), true)?)))
}

#[intrinsic_method(
    "com/sun/imageio/plugins/jpeg/JPEGImageWriter.initWriterIDs(Ljava/lang/Class;Ljava/lang/Class;)V",
    Any
)]
#[async_method]
pub async fn init_writer_ids<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _huff_class = parameters.pop_reference()?;
    let _q_table_class = parameters.pop_reference()?;
    // Rust resolves Java callbacks and fields without retaining JNI IDs.
    let _ = thread;
    Ok(None)
}

#[intrinsic_method("com/sun/imageio/plugins/jpeg/JPEGImageWriter.resetWriter(J)V", Any)]
#[async_method]
pub async fn reset_writer<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let id = parameters.pop_long()?;
    support::context(thread.as_ref(), id, true).await?.reset();
    Ok(None)
}

#[intrinsic_method("com/sun/imageio/plugins/jpeg/JPEGImageWriter.setDest(J)V", Any)]
#[async_method]
pub async fn set_dest<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let id = parameters.pop_long()?;
    support::context(thread.as_ref(), id, true).await?.reset();
    Ok(None)
}

#[intrinsic_method(
    "com/sun/imageio/plugins/jpeg/JPEGImageWriter.writeImage(J[BIII[IIIIII[Ljavax/imageio/plugins/jpeg/JPEGQTable;Z[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;ZZZI[I[I[I[I[IZI)Z",
    Any
)]
#[async_method]
#[expect(
    clippy::too_many_lines,
    reason = "ImageIO native operation marshals its Java arguments and runs the scanline callback protocol"
)]
pub async fn write_image<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let restart = parameters.pop_int()?;
    let have_metadata = parameters.pop_bool()?;
    let qsel = parameters.pop()?;
    let v = parameters.pop()?;
    let h = parameters.pop()?;
    let ids = parameters.pop()?;
    let scans = parameters.pop()?;
    let num_scans = parameters.pop_int()?;
    let progressive = parameters.pop_bool()?;
    let optimize = parameters.pop_bool()?;
    let write_h = parameters.pop_bool()?;
    let ac = parameters.pop()?;
    let dc = parameters.pop()?;
    let write_q = parameters.pop_bool()?;
    let q = parameters.pop()?;
    let step_y = parameters.pop_int()?;
    let step_x = parameters.pop_int()?;
    let height = parameters.pop_int()?;
    let width = parameters.pop_int()?;
    let src_width = parameters.pop_int()?;
    let band_sizes = parameters.pop()?;
    let num_bands = parameters.pop_int()?;
    let output_space = parameters.pop_int()?;
    let input_space = parameters.pop_int()?;
    let data = parameters.pop()?;
    let id = parameters.pop_long()?;
    let ctx = support::context(thread.as_ref(), id, true).await?;
    for value in [&data, &band_sizes, &ids, &h, &v, &qsel] {
        support::nonnull(value)?;
    }
    let receiver = parameters.pop()?;
    if !(0..=5).contains(&input_space)
        || !(0..=5).contains(&output_space)
        || !(1..=4).contains(&num_bands)
        || width < 1
        || height < 1
        || src_width < width
        || step_x < 1
        || step_y < 1
        || num_scans < 0
        || !(0..=65535).contains(&restart)
        || width
            .checked_sub(1)
            .and_then(|x| x.checked_mul(step_x))
            .is_none_or(|last| last >= src_width)
        || height
            .checked_sub(1)
            .and_then(|y| y.checked_mul(step_y))
            .is_none()
    {
        return Err(
            support::io_error(thread.as_ref(), "Invalid argument to native writeImage").await,
        );
    }
    let count = usize::try_from(num_bands)?;
    let band_sizes = band_sizes.as_int_vec_ref()?.to_vec();
    let ids = ids.as_int_vec_ref()?.to_vec();
    let h = h.as_int_vec_ref()?.to_vec();
    let v = v.as_int_vec_ref()?.to_vec();
    let qsel = qsel.as_int_vec_ref()?.to_vec();
    if [&band_sizes, &ids, &h, &v, &qsel]
        .iter()
        .any(|values| values.len() < count)
        || band_sizes
            .iter()
            .take(count)
            .any(|size| !(1..=8).contains(size))
        || data.as_byte_vec_ref()?.len() < usize::try_from(src_width)? * count
    {
        return Err(support::io_error(thread.as_ref(), "Invalid Image").await);
    }
    let scans = if progressive && num_scans != 0 {
        support::nonnull(&scans)?;
        scans.as_int_vec_ref()?.to_vec()
    } else {
        Vec::new()
    };
    support::nonnull(&q)?;
    let tables = if optimize {
        support::tables(&q, &Value::Object(None), &Value::Object(None))?
    } else {
        support::nonnull(&dc)?;
        support::nonnull(&ac)?;
        support::tables(&q, &dc, &ac)?
    };
    let result = {
        let mut config = Config {
            width,
            height,
            components: num_bands,
            input_space,
            output_space,
            optimize: i32::from(optimize),
            progressive: i32::from(progressive),
            restart,
            write_q: i32::from(write_q),
            write_h: i32::from(write_h),
            num_scans: if progressive { num_scans } else { 0 },
            ..Config::default()
        };
        for (target, source) in [
            (&mut config.ids, &ids),
            (&mut config.h, &h),
            (&mut config.v, &v),
            (&mut config.qsel, &qsel),
        ] {
            for (target, source) in target.iter_mut().zip(source.iter()).take(count) {
                *target = *source;
            }
        }
        ctx.codec.lock().start_write(config, &tables, &scans)
    };
    support::checked(thread.as_ref(), result).await?;
    let result: Result<()> = async {
        // The codec has emitted SOI. Java writes APP/COM metadata before scan data.
        support::output(thread.as_ref(), &receiver, &ctx).await?;
        if have_metadata {
            thread
                .invoke(WRITER, "writeMetadata()V", std::slice::from_ref(&receiver))
                .await?;
        }
        let mut row = vec![0; usize::try_from(width)? * count];
        for y in 0..height {
            if ctx.aborted() {
                break;
            }
            thread
                .invoke(
                    WRITER,
                    "grabPixels(I)V",
                    &[receiver.clone(), Value::Int(y * step_y)],
                )
                .await?;
            {
                let pixels = data.as_byte_vec_ref()?;
                for (x, pixel) in row.chunks_mut(count).enumerate() {
                    for (band, (target, size)) in
                        pixel.iter_mut().zip(band_sizes.iter()).enumerate()
                    {
                        let index = x * usize::try_from(step_x)? * count + band;
                        if let Some(sample) = pixels.get(index) {
                            let sample = u32::from(sample.to_ne_bytes()[0]);
                            let max = (1_u32 << u32::try_from(*size)?) - 1;
                            *target = u8::try_from((sample.min(max) * 255 + (max >> 1)) / max)?;
                        }
                    }
                }
            }
            let result = ctx.codec.lock().write_row(&row);
            support::checked(thread.as_ref(), result).await?;
            support::output(thread.as_ref(), &receiver, &ctx).await?;
        }
        if !ctx.aborted() {
            let result = ctx.codec.lock().finish_write();
            support::checked(thread.as_ref(), result).await?;
            support::output(thread.as_ref(), &receiver, &ctx).await?;
        }
        Ok(())
    }
    .await;
    ctx.codec.lock().reset();
    result?;
    Ok(Some(Value::from(ctx.aborted())))
}

#[intrinsic_method(
    "com/sun/imageio/plugins/jpeg/JPEGImageWriter.writeTables(J[Ljavax/imageio/plugins/jpeg/JPEGQTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;)V",
    Any
)]
#[async_method]
pub async fn write_tables<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let ac = parameters.pop()?;
    let dc = parameters.pop()?;
    let q = parameters.pop()?;
    let id = parameters.pop_long()?;
    let ctx = support::context(thread.as_ref(), id, true).await?;
    let receiver = parameters.pop()?;
    if !dc.is_null() {
        support::nonnull(&ac)?;
    }
    let tables = support::tables(
        &q,
        &dc,
        if dc.is_null() {
            &Value::Object(None)
        } else {
            &ac
        },
    )?;
    let result = ctx.codec.lock().write_tables(&tables);
    support::checked(thread.as_ref(), result).await?;
    support::output(thread.as_ref(), &receiver, &ctx).await?;
    Ok(None)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn lifecycle_and_vm_isolation() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let (_other_vm, other_thread) = crate::test::thread().await?;
        let id = init_jpeg_image_writer(thread.clone(), Parameters::default())
            .await?
            .expect("writer handle")
            .as_i64()?;
        assert!(id > 0);
        assert!(
            support::state(other_thread.as_ref())?
                .contexts
                .read()
                .is_empty()
        );
        let ctx = support::context(thread.as_ref(), id, true).await?;
        abort_write(thread.clone(), Parameters::new(vec![Value::Long(id)])).await?;
        assert!(ctx.aborted());
        reset_writer(thread.clone(), Parameters::new(vec![Value::Long(id)])).await?;
        assert!(!ctx.aborted());
        set_dest(thread.clone(), Parameters::new(vec![Value::Long(id)])).await?;
        dispose_writer(thread.clone(), Parameters::new(vec![Value::Long(id)])).await?;
        assert!(support::state(thread.as_ref())?.contexts.read().is_empty());
        assert!(
            reset_writer(thread.clone(), Parameters::new(vec![Value::Long(id)]))
                .await
                .is_err()
        );
        dispose_writer(thread, Parameters::new(vec![Value::Long(id)])).await?;
        Ok(())
    }
}
