use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `jdk.jfr.internal.JVM`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "jdk/jfr/internal/JVM";
    registry.register(class_name, "abort", "(Ljava/lang/String;)V", abort);
    registry.register(
        class_name,
        "addStringConstant",
        "(JLjava/lang/String;)Z",
        add_string_constant,
    );
    registry.register(class_name, "beginRecording", "()V", begin_recording);
    registry.register(class_name, "counterTime", "()J", counter_time);
    registry.register(class_name, "createJFR", "(Z)Z", create_jfr);
    registry.register(class_name, "destroyJFR", "()Z", destroy_jfr);
    registry.register(class_name, "emitEvent", "(JJJ)Z", emit_event);
    registry.register(
        class_name,
        "emitOldObjectSamples",
        "(JZ)V",
        emit_old_object_samples,
    );
    registry.register(class_name, "endRecording", "()V", end_recording);
    registry.register(
        class_name,
        "flush",
        "(Ljdk/jfr/internal/EventWriter;II)Z",
        flush,
    );
    registry.register(
        class_name,
        "getAllEventClasses",
        "()Ljava/util/List;",
        get_all_event_classes,
    );
    registry.register(
        class_name,
        "getAllowedToDoEventRetransforms",
        "()Z",
        get_allowed_to_do_event_retransforms,
    );
    registry.register(
        class_name,
        "getClassId",
        "(Ljava/lang/Class;)J",
        get_class_id,
    );
    registry.register(
        class_name,
        "getClassIdNonIntrinsic",
        "(Ljava/lang/Class;)J",
        get_class_id_non_intrinsic,
    );
    registry.register(
        class_name,
        "getEventWriter",
        "()Ljava/lang/Object;",
        get_event_writer,
    );
    registry.register(class_name, "getPid", "()Ljava/lang/String;", get_pid);
    registry.register(class_name, "getStackTraceId", "(I)J", get_stack_trace_id);
    registry.register(
        class_name,
        "getThreadId",
        "(Ljava/lang/Thread;)J",
        get_thread_id,
    );
    registry.register(class_name, "getTicksFrequency", "()J", get_ticks_frequency);
    registry.register(
        class_name,
        "getTimeConversionFactor",
        "()D",
        get_time_conversion_factor,
    );
    registry.register(class_name, "getTypeId", "(Ljava/lang/Class;)J", get_type_id);
    registry.register(
        class_name,
        "getUnloadedEventClassCount",
        "()J",
        get_unloaded_event_class_count,
    );
    registry.register(class_name, "isAvailable", "()Z", is_available);
    registry.register(class_name, "log", "(IILjava/lang/String;)V", log);
    registry.register(
        class_name,
        "newEventWriter",
        "()Ljdk/jfr/internal/EventWriter;",
        new_event_writer,
    );
    registry.register(class_name, "registerNatives", "()V", register_natives);
    registry.register(
        class_name,
        "retransformClasses",
        "([Ljava/lang/Class;)V",
        retransform_classes,
    );
    registry.register(
        class_name,
        "setCompressedIntegers",
        "(Z)V",
        set_compressed_integers,
    );
    registry.register(class_name, "setCutoff", "(JJ)Z", set_cutoff);
    registry.register(class_name, "setEnabled", "(JZ)V", set_enabled);
    registry.register(
        class_name,
        "setFileNotification",
        "(J)V",
        set_file_notification,
    );
    registry.register(
        class_name,
        "setForceInstrumentation",
        "(Z)V",
        set_force_instrumentation,
    );
    registry.register(
        class_name,
        "setGlobalBufferCount",
        "(J)V",
        set_global_buffer_count,
    );
    registry.register(
        class_name,
        "setGlobalBufferSize",
        "(J)V",
        set_global_buffer_size,
    );
    registry.register(class_name, "setMemorySize", "(J)V", set_memory_size);
    registry.register(
        class_name,
        "setMethodSamplingInterval",
        "(JJ)V",
        set_method_sampling_interval,
    );
    registry.register(class_name, "setOutput", "(Ljava/lang/String;)V", set_output);
    registry.register(
        class_name,
        "setRepositoryLocation",
        "(Ljava/lang/String;)V",
        set_repository_location,
    );
    registry.register(class_name, "setSampleThreads", "(Z)V", set_sample_threads);
    registry.register(class_name, "setStackDepth", "(I)V", set_stack_depth);
    registry.register(
        class_name,
        "setStackTraceEnabled",
        "(JZ)V",
        set_stack_trace_enabled,
    );
    registry.register(
        class_name,
        "setThreadBufferSize",
        "(J)V",
        set_thread_buffer_size,
    );
    registry.register(class_name, "setThreshold", "(JJ)Z", set_threshold);
    registry.register(class_name, "shouldRotateDisk", "()Z", should_rotate_disk);
    registry.register(
        class_name,
        "storeMetadataDescriptor",
        "([B)V",
        store_metadata_descriptor,
    );
    registry.register(
        class_name,
        "subscribeLogLevel",
        "(Ljdk/jfr/internal/LogTag;I)V",
        subscribe_log_level,
    );
    registry.register(
        class_name,
        "uncaughtException",
        "(Ljava/lang/Thread;Ljava/lang/Throwable;)V",
        uncaught_exception,
    );
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn abort(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn add_string_constant(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn begin_recording(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn counter_time(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn create_jfr(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn destroy_jfr(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn emit_event(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn emit_old_object_samples(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn end_recording(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn flush(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_all_event_classes(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_allowed_to_do_event_retransforms(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_class_id(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_class_id_non_intrinsic(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_event_writer(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_pid(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_stack_trace_id(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_thread_id(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_ticks_frequency(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_time_conversion_factor(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_type_id(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_unloaded_event_class_count(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn is_available(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn log(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn new_event_writer(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn register_natives(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn retransform_classes(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn set_compressed_integers(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn set_cutoff(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn set_enabled(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn set_file_notification(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn set_force_instrumentation(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn set_global_buffer_count(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn set_global_buffer_size(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn set_memory_size(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn set_method_sampling_interval(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn set_output(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn set_repository_location(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn set_sample_threads(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn set_stack_depth(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn set_stack_trace_enabled(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn set_thread_buffer_size(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn set_threshold(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn should_rotate_disk(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn store_metadata_descriptor(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn subscribe_log_level(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn uncaught_exception(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
