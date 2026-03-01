use ristretto_classfile::VersionSpecification::{Between, Equal, GreaterThan};
use ristretto_classfile::{JAVA_11, JAVA_17, JAVA_21};
use ristretto_classloader::{Object, Reference, Value};
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Frame, JavaObject, Parameters, Result, Thread, VM};
use std::sync::Arc;

/// Populate the frames array with actual stack frame data, then call doStackWalk.
async fn do_call_stack_walk<T: Thread + 'static>(
    thread: &Arc<T>,
    this: Value,
    mut anchor: i64,
    skip_frames: i32,
    batch_size: i32,
    start_index: i32,
    frames_array: Value,
) -> Result<Option<Value>> {
    struct FrameData {
        rmn_value: Value,
        method_name: Value,
        method_descriptor: Value,
        access_flags: i32,
        bci: i32,
    }

    let vm = thread.vm()?;
    let collector = vm.garbage_collector();
    let vm_frames = thread.frames().await?;
    let skip = usize::try_from(skip_frames).unwrap_or(0);
    let batch = usize::try_from(batch_size).unwrap_or(8);
    let start = usize::try_from(start_index).unwrap_or(0);

    let usable_frames: Vec<_> = vm_frames.iter().rev().skip(skip).take(batch).collect();
    let num_frames = usable_frames.len();

    let rmn_class = thread.class("java.lang.invoke.ResolvedMethodName").await?;

    let mut frame_data = Vec::with_capacity(num_frames);
    for vm_frame in &usable_frames {
        let class_object = vm_frame.class().to_object(thread.as_ref()).await?;
        let method_name = vm_frame.method().name().to_string();
        let method_descriptor = vm_frame.method().descriptor().to_string();
        let name_value = method_name.to_object(thread.as_ref()).await?;
        let desc_value = method_descriptor.to_object(thread.as_ref()).await?;
        let access_flags = i32::from(vm_frame.method().access_flags().bits());
        let bci = i32::try_from(vm_frame.program_counter()).unwrap_or(0);

        // Create ResolvedMethodName with vmholder = declaring Class mirror
        let mut rmn = Object::new(rmn_class.clone())?;
        rmn.set_value("vmholder", class_object)?;
        let rmn_value = Value::new_object(collector, Reference::Object(rmn));

        frame_data.push(FrameData {
            rmn_value,
            method_name: name_value,
            method_descriptor: desc_value,
            access_flags,
            bci,
        });
    }

    // Phase 2: Write to frames array (sync — no await points)
    let mut frames_written: usize = 0;
    {
        let mut frames_ref = frames_array.as_reference_mut()?;
        let Reference::Array(object_array) = &mut *frames_ref else {
            return Err(ristretto_types::Error::InternalError(
                "frames is not an array".into(),
            ));
        };

        // Cap frames to what fits in the array after start_index
        let max_frames = object_array.elements.len().saturating_sub(start);

        for (i, data) in frame_data.into_iter().enumerate() {
            if i >= max_frames {
                break;
            }
            let frame_idx = start + i;
            let element = &object_array.elements[frame_idx];
            if !element.is_object() {
                continue;
            }
            let mut frame_obj = element.as_object_mut()?;

            // Set classOrMemberName to ResolvedMethodName (inherited from ClassFrameInfo)
            frame_obj.set_value("classOrMemberName", data.rmn_value)?;

            // Merge method access flags into existing flags (preserve retainClassRef bit)
            let existing_flags = frame_obj.value("flags")?.as_i32().unwrap_or(0);
            let merged_flags = existing_flags | (data.access_flags & 0x00FF_FFFF);
            frame_obj.set_value("flags", Value::Int(merged_flags))?;

            // Set StackFrameInfo fields
            frame_obj.set_value("name", data.method_name)?;
            frame_obj.set_value("type", data.method_descriptor)?;
            frame_obj.set_value("bci", Value::Int(data.bci))?;
            frames_written += 1;
        }
    }

    let actual_batch = i32::try_from(frames_written).unwrap_or(batch_size);
    let end_index = i32::try_from(start + frames_written).unwrap_or(start_index);

    // Ensure anchor is non-zero — doStackWalk itself sets this.anchor from this parameter
    // after checking checkState(NEW) which requires this.anchor == 0
    if anchor == 0 {
        anchor = 1;
    }

    let walker_class = thread
        .class("java.lang.StackStreamFactory$AbstractStackWalker")
        .await?;
    let method = walker_class
        .method("doStackWalk", "(JIIII)Ljava/lang/Object;")
        .ok_or_else(|| {
            ristretto_types::Error::InternalError("doStackWalk method not found".into())
        })?;
    thread
        .execute(
            &walker_class,
            &method,
            &[
                this,
                Value::Long(anchor),
                Value::Int(skip_frames),
                Value::Int(actual_batch),
                Value::Int(start_index),
                Value::Int(end_index),
            ],
        )
        .await
}

#[intrinsic_method(
    "java/lang/StackStreamFactory$AbstractStackWalker.callStackWalk(JIII[Ljava/lang/Object;)Ljava/lang/Object;",
    Between(JAVA_11, JAVA_17)
)]
#[async_method]
pub async fn call_stack_walk_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let frames = parameters.pop()?;
    let start_index = parameters.pop_int()?;
    let batch_size = parameters.pop_int()?;
    let skip_frames = parameters.pop_int()?;
    let anchor = parameters.pop_long()?;
    let this = parameters.pop()?;
    do_call_stack_walk(
        &thread,
        this,
        anchor,
        skip_frames,
        batch_size,
        start_index,
        frames,
    )
    .await
}

#[intrinsic_method(
    "java/lang/StackStreamFactory$AbstractStackWalker.callStackWalk(JILjdk/internal/vm/ContinuationScope;Ljdk/internal/vm/Continuation;II[Ljava/lang/Object;)Ljava/lang/Object;",
    Equal(JAVA_21)
)]
#[async_method]
pub async fn call_stack_walk_1<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let frames = parameters.pop()?;
    let start_index = parameters.pop_int()?;
    let batch_size = parameters.pop_int()?;
    let _cont = parameters.pop()?;
    let _scope = parameters.pop()?;
    let skip_frames = parameters.pop_int()?;
    let anchor = parameters.pop_long()?;
    let this = parameters.pop()?;
    do_call_stack_walk(
        &thread,
        this,
        anchor,
        skip_frames,
        batch_size,
        start_index,
        frames,
    )
    .await
}

#[intrinsic_method(
    "java/lang/StackStreamFactory$AbstractStackWalker.callStackWalk(IILjdk/internal/vm/ContinuationScope;Ljdk/internal/vm/Continuation;II[Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThan(JAVA_21)
)]
#[async_method]
pub async fn call_stack_walk_2<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let frames = parameters.pop()?;
    let start_index = parameters.pop_int()?;
    let batch_size = parameters.pop_int()?;
    let _cont = parameters.pop()?;
    let _scope = parameters.pop()?;
    let skip_frames = parameters.pop_int()?;
    let _mode = parameters.pop_int()?;
    let this = parameters.pop()?;
    let anchor = 0i64;
    do_call_stack_walk(
        &thread,
        this,
        anchor,
        skip_frames,
        batch_size,
        start_index,
        frames,
    )
    .await
}

#[intrinsic_method(
    "java/lang/StackStreamFactory$AbstractStackWalker.fetchStackFrames(JJII[Ljava/lang/Object;)I",
    Between(JAVA_11, JAVA_21)
)]
#[async_method]
pub async fn fetch_stack_frames_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    // Return 0 frames fetched
    Ok(Some(Value::Int(0)))
}

#[intrinsic_method(
    "java/lang/StackStreamFactory$AbstractStackWalker.fetchStackFrames(IJIII[Ljava/lang/Object;)I",
    GreaterThan(JAVA_21)
)]
#[async_method]
pub async fn fetch_stack_frames_1<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    // Return 0 frames fetched
    Ok(Some(Value::Int(0)))
}

#[intrinsic_method(
    "java/lang/StackStreamFactory$AbstractStackWalker.setContinuation(J[Ljava/lang/Object;Ljdk/internal/vm/Continuation;)V",
    GreaterThan(JAVA_17)
)]
#[async_method]
pub async fn set_continuation<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    // No-op
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_stack_frames_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = fetch_stack_frames_0(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::Int(0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_fetch_stack_frames_1() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = fetch_stack_frames_1(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::Int(0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_set_continuation() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_continuation(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }
}
