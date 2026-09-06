use super::support::{self, READER};
use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::{Any, GreaterThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;
use std::sync::atomic::Ordering;

#[intrinsic_method("com/sun/imageio/plugins/jpeg/JPEGImageReader.abortRead(J)V", Any)]
#[async_method]
pub async fn abort_read<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let id = parameters.pop_long()?;
    support::context(thread.as_ref(), id, false)
        .await?
        .abort(true);
    Ok(None)
}

#[intrinsic_method(
    "com/sun/imageio/plugins/jpeg/JPEGImageReader.clearNativeReadAbortFlag(J)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn clear_native_read_abort_flag<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let id = parameters.pop_long()?;
    support::context(thread.as_ref(), id, false)
        .await?
        .abort(false);
    Ok(None)
}

#[intrinsic_method("com/sun/imageio/plugins/jpeg/JPEGImageReader.disposeReader(J)V", Any)]
#[async_method]
pub async fn dispose_reader<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let id = parameters.pop_long()?;
    let state = support::state(thread.as_ref())?;
    let mut contexts = state.contexts.write();
    if contexts.get(&id).is_some_and(|context| !context.writer) {
        contexts.remove(&id);
    }
    Ok(None)
}

#[intrinsic_method(
    "com/sun/imageio/plugins/jpeg/JPEGImageReader.initJPEGImageReader()J",
    Any
)]
#[async_method]
pub async fn init_jpeg_image_reader<T: Thread + 'static>(
    thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Long(support::create(thread.as_ref(), false)?)))
}

#[intrinsic_method(
    "com/sun/imageio/plugins/jpeg/JPEGImageReader.initReaderIDs(Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;)V",
    Any
)]
#[async_method]
pub async fn init_reader_i_ds<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _huff_class = parameters.pop_reference()?;
    let _q_table_class = parameters.pop_reference()?;
    let _image_input_stream_class = parameters.pop_reference()?;
    // Callbacks and fields are resolved symbolically, so JNI ID caching is unnecessary.
    let _ = thread;
    Ok(None)
}

#[intrinsic_method(
    "com/sun/imageio/plugins/jpeg/JPEGImageReader.readImage(IJ[BI[I[IIIIIII[Ljavax/imageio/plugins/jpeg/JPEGQTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;IIZ)Z",
    Any
)]
#[async_method]
#[expect(
    clippy::too_many_lines,
    reason = "ImageIO native operation marshals its Java arguments and runs the scanline callback protocol"
)]
pub async fn read_image<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let want_updates = parameters.pop_bool()?;
    let max_pass = parameters.pop_int()?;
    let min_pass = parameters.pop_int()?;
    let ac = parameters.pop()?;
    let dc = parameters.pop()?;
    let q = parameters.pop()?;
    let step_y = parameters.pop_int()?;
    let step_x = parameters.pop_int()?;
    let height = parameters.pop_int()?;
    let width = parameters.pop_int()?;
    let y = parameters.pop_int()?;
    let x = parameters.pop_int()?;
    let _band_sizes = parameters.pop()?;
    let bands = parameters.pop()?;
    let num_bands = parameters.pop_int()?;
    let buffer = parameters.pop()?;
    let id = parameters.pop_long()?;
    let _image_index = parameters.pop_int()?;
    let ctx = support::context(thread.as_ref(), id, false).await?;
    support::nonnull(&buffer)?;
    support::nonnull(&bands)?;
    let receiver = parameters.pop()?;
    let [image_width, image_height, _, _, _, progressive] = *ctx.info.lock();
    if !(1..=4).contains(&num_bands)
        || x < 0
        || y < 0
        || width < 1
        || height < 1
        || x.checked_add(width).is_none_or(|end| end > image_width)
        || y.checked_add(height).is_none_or(|end| end > image_height)
        || step_x < 1
        || step_y < 1
        || min_pass < 0
        || max_pass < min_pass
    {
        return Err(
            support::io_error(thread.as_ref(), "Invalid argument to native readImage").await,
        );
    }
    let bands = bands.as_int_vec_ref()?.to_vec();
    let num_bands = usize::try_from(num_bands)?;
    if bands.len() < num_bands
        || bands
            .iter()
            .take(num_bands)
            .any(|band| *band < 0 || usize::try_from(*band).is_ok_and(|band| band >= num_bands))
    {
        return Err(
            support::io_error(thread.as_ref(), "Invalid argument to native readImage").await,
        );
    }
    let output_width = usize::try_from((width - 1) / step_x + 1)?;
    if buffer.as_byte_vec_ref()?.len() < output_width * num_bands {
        return Err(
            support::io_error(thread.as_ref(), "Invalid argument to native readImage").await,
        );
    }
    let tables = support::tables(&q, &dc, &ac)?;
    if ctx.aborted() {
        return Ok(Some(Value::from(true)));
    }
    let mut data = ctx.header.lock().clone();
    if data.is_empty() {
        return Err(support::io_error(thread.as_ref(), "JPEG header has not been read").await);
    }
    let mut input = support::Input::new(thread.as_ref(), &receiver)?;
    let tail = input.image_tail(&mut data).await;
    let finish = input.finish().await;
    tail?;
    finish?;
    let result = ctx.codec.lock().header(&data);
    support::checked(thread.as_ref(), result).await?;
    let result = ctx
        .codec
        .lock()
        .start_read(ctx.color.load(Ordering::Acquire), &tables);
    let [decoded_width, decoded_height, components] =
        support::checked(thread.as_ref(), result).await?;
    if usize::try_from(components)? != num_bands {
        return Err(
            support::io_error(thread.as_ref(), "Invalid argument to native readImage").await,
        );
    }
    let mut row = vec![0; usize::try_from(decoded_width)? * num_bands];
    let mut pass = if progressive != 0 { min_pass } else { 0 };
    let result: Result<()> = async {
        loop {
            if ctx.aborted() {
                break;
            }
            let result = ctx.codec.lock().start_pass(pass);
            let actual_pass = support::checked(thread.as_ref(), result).await?;
            if want_updates {
                thread
                    .invoke(
                        READER,
                        "passStarted(I)V",
                        &[receiver.clone(), Value::Int(actual_pass)],
                    )
                    .await?;
            }
            let mut target_y = 0;
            for source_y in 0..decoded_height {
                if ctx.aborted() {
                    break;
                }
                let result = ctx.codec.lock().read_row(&mut row);
                support::checked(thread.as_ref(), result).await?;
                if source_y < y || source_y >= y + height || (source_y - y) % step_y != 0 {
                    continue;
                }
                {
                    let mut output = buffer.as_byte_vec_mut()?;
                    let mut samples = output.iter_mut();
                    for source_x in (x..x + width).step_by(usize::try_from(step_x)?) {
                        for band in bands.iter().take(num_bands) {
                            let offset =
                                usize::try_from(source_x)? * num_bands + usize::try_from(*band)?;
                            if let (Some(sample), Some(dest)) = (row.get(offset), samples.next()) {
                                *dest = i8::from_ne_bytes([*sample]);
                            }
                        }
                    }
                }
                thread
                    .invoke(
                        READER,
                        "acceptPixels(IZ)V",
                        &[
                            receiver.clone(),
                            Value::Int(target_y),
                            Value::from(progressive != 0),
                        ],
                    )
                    .await?;
                target_y += 1;
            }
            if ctx.aborted() {
                break;
            }
            let result = ctx.codec.lock().finish_pass();
            let next_pass = support::checked(thread.as_ref(), result).await?;
            if want_updates {
                thread
                    .invoke(READER, "passComplete()V", std::slice::from_ref(&receiver))
                    .await?;
            }
            support::warning(thread.as_ref(), &receiver, &ctx).await?;
            let Some(next_pass) = next_pass else {
                break;
            };
            if next_pass > max_pass {
                break;
            }
            pass = next_pass;
        }
        Ok(())
    }
    .await;
    ctx.codec.lock().reset();
    result?;
    Ok(Some(Value::from(ctx.aborted())))
}

#[intrinsic_method(
    "com/sun/imageio/plugins/jpeg/JPEGImageReader.readImageHeader(JZZ)Z",
    Any
)]
#[async_method]
pub async fn read_image_header<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let reset = parameters.pop_bool()?;
    let _clear_buffer = parameters.pop_bool()?;
    let id = parameters.pop_long()?;
    let ctx = support::context(thread.as_ref(), id, false).await?;
    let receiver = parameters.pop()?;
    support::nonnull(&receiver)?;
    let mut input = support::Input::new(thread.as_ref(), &receiver)?;
    let header = input.header().await;
    let finish = input.finish().await;
    let header = header?;
    finish?;
    let result = ctx.codec.lock().header(&header);
    let (info, tables_only) = support::checked(thread.as_ref(), result).await?;
    support::warning(thread.as_ref(), &receiver, &ctx).await?;
    if !tables_only {
        let profile = support::checked(thread.as_ref(), support::icc_profile(&header)).await?;
        let profile = match profile {
            Some(data) => support::bytes(thread.as_ref(), &data)?,
            None => Value::Object(None),
        };
        let [width, height, color, output, components, _] = info;
        *ctx.info.lock() = info;
        ctx.color.store(output, Ordering::Release);
        *ctx.header.lock() = header;
        thread
            .invoke(
                READER,
                "setImageData(IIIII[B)V",
                &[
                    receiver,
                    Value::Int(width),
                    Value::Int(height),
                    Value::Int(color),
                    Value::Int(output),
                    Value::Int(components),
                    profile,
                ],
            )
            .await?;
        if reset {
            ctx.codec.lock().reset();
        }
    }
    Ok(Some(Value::from(tables_only)))
}

#[intrinsic_method(
    "com/sun/imageio/plugins/jpeg/JPEGImageReader.resetLibraryState(J)V",
    Any
)]
#[async_method]
pub async fn reset_library_state<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let id = parameters.pop_long()?;
    support::context(thread.as_ref(), id, false)
        .await?
        .codec
        .lock()
        .reset();
    Ok(None)
}

#[intrinsic_method("com/sun/imageio/plugins/jpeg/JPEGImageReader.resetReader(J)V", Any)]
#[async_method]
pub async fn reset_reader<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let id = parameters.pop_long()?;
    support::context(thread.as_ref(), id, false).await?.reset();
    Ok(None)
}

#[intrinsic_method(
    "com/sun/imageio/plugins/jpeg/JPEGImageReader.setOutColorSpace(JI)V",
    Any
)]
#[async_method]
pub async fn set_out_color_space<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let color = parameters.pop_int()?;
    let id = parameters.pop_long()?;
    support::context(thread.as_ref(), id, false)
        .await?
        .color
        .store(color, Ordering::Release);
    Ok(None)
}

#[intrinsic_method("com/sun/imageio/plugins/jpeg/JPEGImageReader.setSource(J)V", Any)]
#[async_method]
pub async fn set_source<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let id = parameters.pop_long()?;
    support::context(thread.as_ref(), id, false).await?.reset();
    Ok(None)
}
#[cfg(test)]
mod tests {
    use super::*;
    use ristretto_types::Error;

    #[tokio::test]
    async fn lifecycle_and_disposed_reader() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let id = init_jpeg_image_reader(thread.clone(), Parameters::default())
            .await?
            .expect("reader handle")
            .as_i64()?;
        let other = init_jpeg_image_reader(thread.clone(), Parameters::default())
            .await?
            .expect("reader handle")
            .as_i64()?;
        assert!(id > 0 && other > 0 && id != other);
        let ctx = support::context(thread.as_ref(), id, false).await?;
        abort_read(thread.clone(), Parameters::new(vec![Value::Long(id)])).await?;
        assert!(ctx.aborted());
        clear_native_read_abort_flag(thread.clone(), Parameters::new(vec![Value::Long(id)]))
            .await?;
        assert!(!ctx.aborted());
        set_out_color_space(
            thread.clone(),
            Parameters::new(vec![Value::Long(id), Value::Int(1)]),
        )
        .await?;
        assert_eq!(ctx.color.load(Ordering::Acquire), 1);
        reset_library_state(thread.clone(), Parameters::new(vec![Value::Long(id)])).await?;
        reset_reader(thread.clone(), Parameters::new(vec![Value::Long(id)])).await?;
        set_source(thread.clone(), Parameters::new(vec![Value::Long(id)])).await?;
        dispose_reader(thread.clone(), Parameters::new(vec![Value::Long(id)])).await?;
        dispose_reader(thread.clone(), Parameters::new(vec![Value::Long(id)])).await?;
        let error = abort_read(thread.clone(), Parameters::new(vec![Value::Long(id)])).await;
        assert!(
            matches!(error, Err(Error::Throwable(ref value)) if value.as_object_ref()?.class().name() == "java/lang/IllegalStateException")
        );
        assert!(
            support::context(thread.as_ref(), other, false)
                .await
                .is_ok()
        );
        Ok(())
    }
}
