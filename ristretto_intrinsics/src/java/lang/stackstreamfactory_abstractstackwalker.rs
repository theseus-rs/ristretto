use crate::java::lang::invoke::methodhandlenatives::MemberNameFlags;
use ristretto_classfile::VersionSpecification::{Between, Equal, GreaterThan};
use ristretto_classfile::{JAVA_11, JAVA_17, JAVA_21, JAVA_25};
use ristretto_classloader::{Object, Reference, Value};
use ristretto_gc::GarbageCollector;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Error::InternalError;
use ristretto_types::{Frame, JavaObject, Parameters, Result, Thread, VM};
use std::sync::Arc;

struct FrameData {
    class_mirror: Value,
    member: Value,
    method_name: Value,
    descriptor: Value,
    bytecode_index: i32,
}

/// Builds per-frame data from the filtered VM frames.
async fn build_frame_data<T: Thread + 'static>(
    thread: &Arc<T>,
    gc: &Arc<GarbageCollector>,
    filtered_frames: &[&Arc<T::Frame>],
    modern_layout: bool,
) -> Result<Vec<FrameData>> {
    let member_class = if modern_layout {
        thread.class("java/lang/invoke/ResolvedMethodName").await?
    } else {
        thread.class("java/lang/invoke/MemberName").await?
    };
    let mut frame_data = Vec::with_capacity(filtered_frames.len());
    for frame in filtered_frames {
        let class = frame.class();
        let method = frame.method();
        let pc = frame.program_counter();
        let class_mirror = class.to_object(thread).await?;
        let method_name = method.name().to_object(thread).await?;
        let descriptor = method.descriptor().to_object(thread).await?;

        let mut member = Object::new(member_class.clone())?;
        if modern_layout {
            member.set_value("vmholder", class_mirror.clone())?;
        } else {
            member.set_value("clazz", class_mirror.clone())?;
            member.set_value("name", method_name.clone())?;
            member.set_value("type", descriptor.clone())?;
            let member_kind = if method.name() == "<init>" {
                MemberNameFlags::IS_CONSTRUCTOR
            } else {
                MemberNameFlags::IS_METHOD
            };
            let flags = i32::from(method.access_flags().bits()) | member_kind.bits();
            member.set_value("flags", Value::Int(flags))?;
        }
        let member = Value::new_object(gc, Reference::Object(member));

        let bytecode_index = i32::try_from(pc).unwrap_or(0);
        frame_data.push(FrameData {
            class_mirror,
            member,
            method_name,
            descriptor,
            bytecode_index,
        });
    }
    Ok(frame_data)
}

/// Populates the pre-allocated frame objects in the Java frames array.
///
/// Returns the end index (exclusive) of the populated frames.
fn populate_frames(
    frame_data: Vec<FrameData>,
    frames_value: &Value,
    start: usize,
    modern_layout: bool,
) -> Result<i32> {
    let frames_guard = frames_value.as_reference()?;
    let elements = match &*frames_guard {
        Reference::Array(arr) => &arr.elements,
        _ => return Err(InternalError("frames is not an array".to_string())),
    };

    let mut end = start;
    for (i, frame) in frame_data.into_iter().enumerate() {
        let array_idx = start + i;
        if array_idx >= elements.len() {
            break;
        }
        if let Some(Value::Object(Some(sfi_gc))) = elements.get(array_idx) {
            let mut sfi = sfi_gc.write();
            if let Reference::Object(ref mut obj) = *sfi {
                let obj_class_name = obj.class().name().to_string();
                if obj_class_name == "java/lang/ClassFrameInfo" {
                    obj.set_value("classOrMemberName", frame.class_mirror)?;
                } else if modern_layout {
                    // JDK 25 keeps the resolved member and its expanded name and type directly on
                    // StackFrameInfo.
                    obj.set_value("classOrMemberName", frame.member)?;
                    obj.set_value("name", frame.method_name)?;
                    obj.set_value("type", frame.descriptor)?;
                    obj.set_value("bci", Value::Int(frame.bytecode_index))?;
                } else {
                    // JDK 11 through 21 keep all member metadata in MemberName.
                    obj.set_value("memberName", frame.member)?;
                    obj.set_value("bci", Value::Int(frame.bytecode_index))?;
                }
            }
        }
        end = array_idx + 1;
    }
    Ok(i32::try_from(end).unwrap_or(0))
}

/// Shared implementation for all callStackWalk variants.
///
/// Walks the thread's call stack, populates `StackFrameInfo` objects in the
/// pre-allocated frames array, and calls back into `doStackWalk()` on the
/// `AbstractStackWalker` instance.
async fn call_stack_walk_impl<T: Thread + 'static>(
    thread: Arc<T>,
    this: Value,
    skip_frames: i32,
    _batch_size: i32,
    start_index: i32,
    frames_value: Value,
) -> Result<Option<Value>> {
    let vm = thread.vm()?;
    let gc = vm.garbage_collector();
    let modern_layout = vm.java_major_version() >= JAVA_25.java();

    let vm_frames = thread.frames().await?;
    let filtered_frames: Vec<_> = vm_frames
        .iter()
        .rev()
        .filter(|f| !f.class().name().starts_with("java/lang/StackStreamFactory"))
        .skip(usize::try_from(skip_frames).unwrap_or(0))
        .collect();

    let frame_data = build_frame_data(&thread, gc, &filtered_frames, modern_layout).await?;
    let start = usize::try_from(start_index).unwrap_or(0);
    let end_index = populate_frames(frame_data, &frames_value, start, modern_layout)?;

    // Set total frame depth on the walker; used by frameBuffer.setBatch().
    let total_depth = i32::try_from(filtered_frames.len()).unwrap_or(0);
    {
        let mut walker_obj = this.as_object_mut()?;
        walker_obj.set_value("depth", Value::Int(total_depth))?;
    }

    // Pre-load WalkerState so its enum constants are initialized before
    // doStackWalk runs (doStackWalk does `getstatic WalkerState.NEW`).
    let _ = thread
        .class("java/lang/StackStreamFactory$WalkerState")
        .await?;

    // Look up `doStackWalk` on the walker's actual class, falling back to
    // `AbstractStackWalker` (where the private method is declared).
    let walker_class = {
        let walker_ref = this.as_object_ref()?;
        walker_ref.class().clone()
    };
    let abstract_walker_class = thread
        .class("java/lang/StackStreamFactory$AbstractStackWalker")
        .await?;
    let (dispatch_class, do_stack_walk) = walker_class
        .methods()
        .into_iter()
        .find(|m| m.name() == "doStackWalk")
        .map(|m| (walker_class.clone(), m))
        .or_else(|| {
            abstract_walker_class
                .methods()
                .into_iter()
                .find(|m| m.name() == "doStackWalk")
                .map(|m| (abstract_walker_class.clone(), m))
        })
        .ok_or_else(|| InternalError("doStackWalk method not found".to_string()))?;

    // Anchor must be a positive non-(-1) value; `consumeFrames()` checks
    // that the walker is active via `anchor != 0 && anchor != -1`.
    let anchor = Value::Long(1);
    let num_frames = end_index - start_index;
    let params = if do_stack_walk.descriptor().contains("JIIIII") {
        vec![
            this,
            anchor,
            Value::Int(0),
            Value::Int(0),
            Value::Int(num_frames),
            Value::Int(start_index),
            Value::Int(end_index),
        ]
    } else {
        vec![
            this,
            anchor,
            Value::Int(0),
            Value::Int(num_frames),
            Value::Int(start_index),
            Value::Int(end_index),
        ]
    };

    thread
        .execute(&dispatch_class, &do_stack_walk, &params)
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
    let _mode = parameters.pop_long()?;
    let this = parameters.pop()?;

    call_stack_walk_impl(thread, this, skip_frames, batch_size, start_index, frames).await
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
    let _continuation = parameters.pop()?;
    let _cont_scope = parameters.pop()?;
    let skip_frames = parameters.pop_int()?;
    let _mode = parameters.pop_long()?;
    let this = parameters.pop()?;

    call_stack_walk_impl(thread, this, skip_frames, batch_size, start_index, frames).await
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
    let _continuation = parameters.pop()?;
    let _cont_scope = parameters.pop()?;
    let skip_frames = parameters.pop_int()?;
    let _mode = parameters.pop_int()?;
    let this = parameters.pop()?;

    call_stack_walk_impl(thread, this, skip_frames, batch_size, start_index, frames).await
}

#[intrinsic_method(
    "java/lang/StackStreamFactory$AbstractStackWalker.fetchStackFrames(JJII[Ljava/lang/Object;)I",
    Between(JAVA_11, JAVA_21)
)]
#[async_method]
pub async fn fetch_stack_frames_0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _frames = parameters.pop()?;
    let _start_index = parameters.pop_int()?;
    let _batch_size = parameters.pop_int()?;
    let _anchor = parameters.pop_long()?;
    let _mode = parameters.pop_long()?;
    let _this = parameters.pop()?;
    // All frames were provided in callStackWalk; no additional frames to fetch.
    Ok(Some(Value::Int(0)))
}

#[intrinsic_method(
    "java/lang/StackStreamFactory$AbstractStackWalker.fetchStackFrames(IJIII[Ljava/lang/Object;)I",
    GreaterThan(JAVA_21)
)]
#[async_method]
pub async fn fetch_stack_frames_1<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _frames = parameters.pop()?;
    let _start_index = parameters.pop_int()?;
    let _batch_size = parameters.pop_int()?;
    let _anchor = parameters.pop_int()?;
    let _mode = parameters.pop_long()?;
    let _skip_frames = parameters.pop_int()?;
    let _this = parameters.pop()?;
    // All frames were provided in callStackWalk; no additional frames to fetch.
    Ok(Some(Value::Int(0)))
}

#[intrinsic_method(
    "java/lang/StackStreamFactory$AbstractStackWalker.setContinuation(J[Ljava/lang/Object;Ljdk/internal/vm/Continuation;)V",
    GreaterThan(JAVA_17)
)]
#[async_method]
pub async fn set_continuation<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _continuation = parameters.pop()?;
    let _frames = parameters.pop()?;
    let _anchor = parameters.pop_long()?;
    let _this = parameters.pop()?;
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_stack_frames_0() {
        let (_vm, thread) = crate::test::java21_thread().await.expect("thread");
        let params = Parameters::new(vec![
            Value::Object(None), // this
            Value::Long(0),      // mode
            Value::Long(0),      // anchor
            Value::Int(0),       // batchSize
            Value::Int(0),       // startIndex
            Value::Object(None), // frames
        ]);
        let result = fetch_stack_frames_0(thread, params)
            .await
            .expect("fetchStackFrames should succeed");
        assert_eq!(Some(Value::Int(0)), result);
    }

    #[tokio::test]
    async fn test_fetch_stack_frames_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let params = Parameters::new(vec![
            Value::Object(None), // this
            Value::Int(0),       // skipFrames
            Value::Long(0),      // mode
            Value::Int(0),       // anchor
            Value::Int(0),       // batchSize
            Value::Int(0),       // startIndex
            Value::Object(None), // frames
        ]);
        let result = fetch_stack_frames_1(thread, params)
            .await
            .expect("fetchStackFrames should succeed");
        assert_eq!(Some(Value::Int(0)), result);
    }

    #[tokio::test]
    async fn test_set_continuation() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let params = Parameters::new(vec![
            Value::Object(None), // this
            Value::Long(0),      // anchor
            Value::Object(None), // frames
            Value::Object(None), // continuation
        ]);
        let result = set_continuation(thread, params)
            .await
            .expect("setContinuation should succeed");
        assert_eq!(None, result);
    }
}
