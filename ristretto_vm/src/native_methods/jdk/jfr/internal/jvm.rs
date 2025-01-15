use crate::arguments::Arguments;
use crate::native_methods::registry::{
    MethodRegistry, JAVA_11, JAVA_17, JAVA_18, JAVA_19, JAVA_20, JAVA_21, JAVA_22, JAVA_23,
};
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "jdk/jfr/internal/JVM";

/// Register all native methods for `jdk.jfr.internal.JVM`.
#[expect(clippy::too_many_lines)]
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() <= JAVA_11 {
        registry.register(
            CLASS_NAME,
            "emitOldObjectSamples",
            "(JZ)V",
            emit_old_object_samples,
        );
        registry.register(
            CLASS_NAME,
            "getClassIdNonIntrinsic",
            "(Ljava/lang/Class;)J",
            get_class_id_non_intrinsic,
        );
        registry.register(
            CLASS_NAME,
            "setMethodSamplingInterval",
            "(JJ)V",
            set_method_sampling_interval,
        );
    } else {
        registry.register(
            CLASS_NAME,
            "emitOldObjectSamples",
            "(JZZ)V",
            emit_old_object_samples,
        );
        registry.register(CLASS_NAME, "exclude", "(Ljava/lang/Thread;)V", exclude);
        registry.register(CLASS_NAME, "flush", "()V", flush);
        registry.register(
            CLASS_NAME,
            "getChunkStartNanos",
            "()J",
            get_chunk_start_nanos,
        );
        registry.register(
            CLASS_NAME,
            "getHandler",
            "(Ljava/lang/Class;)Ljava/lang/Object;",
            get_handler,
        );
        registry.register(
            CLASS_NAME,
            "getTypeId",
            "(Ljava/lang/String;)J",
            get_type_id,
        );
        registry.register(CLASS_NAME, "include", "(Ljava/lang/Thread;)V", include);
        registry.register(
            CLASS_NAME,
            "isExcluded",
            "(Ljava/lang/Thread;)Z",
            is_excluded,
        );
        registry.register(CLASS_NAME, "isRecording", "()Z", is_recording);
        registry.register(
            CLASS_NAME,
            "logEvent",
            "(I[Ljava/lang/String;Z)V",
            log_event,
        );
        registry.register(CLASS_NAME, "markChunkFinal", "()V", mark_chunk_final);
        registry.register(
            CLASS_NAME,
            "setHandler",
            "(Ljava/lang/Class;Ljdk/jfr/internal/handlers/EventHandler;)Z",
            set_handler,
        );
        registry.register(CLASS_NAME, "setThrottle", "(JJJ)Z", set_throttle);
    }

    if registry.java_major_version() >= JAVA_11 {
        registry.register(
            CLASS_NAME,
            "flush",
            "(Ljdk/jfr/internal/EventWriter;II)Z",
            flush,
        );
        registry.register(
            CLASS_NAME,
            "getEventWriter",
            "()Ljava/lang/Object;",
            get_event_writer,
        );
        registry.register(
            CLASS_NAME,
            "newEventWriter",
            "()Ljdk/jfr/internal/EventWriter;",
            new_event_writer,
        );
        registry.register(CLASS_NAME, "setSampleThreads", "(Z)V", set_sample_threads);
    }

    if registry.java_major_version() == JAVA_17 {
        registry.register(CLASS_NAME, "emitDataLoss", "(J)V", emit_data_loss);
        registry.register(
            CLASS_NAME,
            "setMethodSamplingPeriod",
            "(JJ)V",
            set_method_sampling_period,
        );
    }

    if registry.java_major_version() >= JAVA_18 {
        registry.register(
            CLASS_NAME,
            "getDumpPath",
            "()Ljava/lang/String;",
            get_dump_path,
        );
        registry.register(
            CLASS_NAME,
            "setDumpPath",
            "(Ljava/lang/String;)V",
            set_dump_path,
        );
        registry.register(
            CLASS_NAME,
            "setMethodSamplingInterval",
            "(JJ)V",
            set_method_sampling_interval,
        );
    } else {
        registry.register(CLASS_NAME, "exclude", "(Ljava/lang/Thread;)V", exclude);
        registry.register(CLASS_NAME, "flush", "()V", flush);
    }

    if registry.java_major_version() >= JAVA_19 {
        registry.register(
            CLASS_NAME,
            "flush",
            "(Ljdk/jfr/internal/event/EventWriter;II)Z",
            flush,
        );
        registry.register(
            CLASS_NAME,
            "getConfiguration",
            "(Ljava/lang/Class;)Ljava/lang/Object;",
            get_configuration,
        );
        registry.register(
            CLASS_NAME,
            "getEventWriter",
            "()Ljdk/jfr/internal/event/EventWriter;",
            get_event_writer,
        );
        registry.register(CLASS_NAME, "isContainerized", "()Z", is_containerized);
        registry.register(
            CLASS_NAME,
            "isExcluded",
            "(Ljava/lang/Class;)Z",
            is_excluded,
        );
        registry.register(
            CLASS_NAME,
            "isExcluded",
            "(Ljava/lang/Thread;)Z",
            is_excluded,
        );
        registry.register(
            CLASS_NAME,
            "isInstrumented",
            "(Ljava/lang/Class;)Z",
            is_instrumented,
        );
        registry.register(CLASS_NAME, "isRecording", "()Z", is_recording);
        registry.register(
            CLASS_NAME,
            "newEventWriter",
            "()Ljdk/jfr/internal/event/EventWriter;",
            new_event_writer,
        );
        registry.register(
            CLASS_NAME,
            "setConfiguration",
            "(Ljava/lang/Class;Ljdk/jfr/internal/event/EventConfiguration;)Z",
            set_configuration,
        );
    }

    if registry.java_major_version() >= JAVA_20 {
        registry.register(CLASS_NAME, "hostTotalMemory", "()J", host_total_memory);
    }

    if registry.java_major_version() >= JAVA_21 {
        registry.register(CLASS_NAME, "commit", "(J)J", commit);
        registry.register(CLASS_NAME, "emitDataLoss", "(J)V", emit_data_loss);
        registry.register(
            CLASS_NAME,
            "flush",
            "(Ljdk/jfr/internal/event/EventWriter;II)V",
            flush,
        );
    }

    if registry.java_major_version() <= JAVA_21 {
        registry.register(CLASS_NAME, "getStackTraceId", "(I)J", get_stack_trace_id);
    } else {
        registry.register(CLASS_NAME, "getStackTraceId", "(IJ)J", get_stack_trace_id);
    }

    if registry.java_major_version() >= JAVA_22 {
        registry.register(
            CLASS_NAME,
            "registerStackFilter",
            "([Ljava/lang/String;[Ljava/lang/String;)J",
            register_stack_filter,
        );
        registry.register(CLASS_NAME, "setMiscellaneous", "(JJ)V", set_miscellaneous);
        registry.register(
            CLASS_NAME,
            "unregisterStackFilter",
            "(J)V",
            unregister_stack_filter,
        );
    }

    if registry.java_major_version() >= JAVA_23 {
        registry.register(
            CLASS_NAME,
            "hostTotalSwapMemory",
            "()J",
            host_total_swap_memory,
        );
        registry.register(CLASS_NAME, "nanosNow", "()J", nanos_now);
    }

    registry.register(CLASS_NAME, "abort", "(Ljava/lang/String;)V", abort);
    registry.register(
        CLASS_NAME,
        "addStringConstant",
        "(JLjava/lang/String;)Z",
        add_string_constant,
    );
    registry.register(CLASS_NAME, "beginRecording", "()V", begin_recording);
    registry.register(CLASS_NAME, "counterTime", "()J", counter_time);
    registry.register(CLASS_NAME, "createJFR", "(Z)Z", create_jfr);
    registry.register(CLASS_NAME, "destroyJFR", "()Z", destroy_jfr);
    registry.register(CLASS_NAME, "emitEvent", "(JJJ)Z", emit_event);
    registry.register(CLASS_NAME, "endRecording", "()V", end_recording);
    registry.register(
        CLASS_NAME,
        "getAllEventClasses",
        "()Ljava/util/List;",
        get_all_event_classes,
    );
    registry.register(
        CLASS_NAME,
        "getAllowedToDoEventRetransforms",
        "()Z",
        get_allowed_to_do_event_retransforms,
    );
    registry.register(
        CLASS_NAME,
        "getClassId",
        "(Ljava/lang/Class;)J",
        get_class_id,
    );
    registry.register(CLASS_NAME, "getPid", "()Ljava/lang/String;", get_pid);
    registry.register(
        CLASS_NAME,
        "getThreadId",
        "(Ljava/lang/Thread;)J",
        get_thread_id,
    );
    registry.register(CLASS_NAME, "getTicksFrequency", "()J", get_ticks_frequency);
    registry.register(
        CLASS_NAME,
        "getTimeConversionFactor",
        "()D",
        get_time_conversion_factor,
    );
    registry.register(CLASS_NAME, "getTypeId", "(Ljava/lang/Class;)J", get_type_id);
    registry.register(
        CLASS_NAME,
        "getUnloadedEventClassCount",
        "()J",
        get_unloaded_event_class_count,
    );
    registry.register(CLASS_NAME, "isAvailable", "()Z", is_available);
    registry.register(CLASS_NAME, "log", "(IILjava/lang/String;)V", log);
    registry.register(CLASS_NAME, "registerNatives", "()V", register_natives);
    registry.register(
        CLASS_NAME,
        "retransformClasses",
        "([Ljava/lang/Class;)V",
        retransform_classes,
    );
    registry.register(
        CLASS_NAME,
        "setCompressedIntegers",
        "(Z)V",
        set_compressed_integers,
    );
    registry.register(CLASS_NAME, "setCutoff", "(JJ)Z", set_cutoff);
    registry.register(CLASS_NAME, "setEnabled", "(JZ)V", set_enabled);
    registry.register(
        CLASS_NAME,
        "setFileNotification",
        "(J)V",
        set_file_notification,
    );
    registry.register(
        CLASS_NAME,
        "setForceInstrumentation",
        "(Z)V",
        set_force_instrumentation,
    );
    registry.register(
        CLASS_NAME,
        "setGlobalBufferCount",
        "(J)V",
        set_global_buffer_count,
    );
    registry.register(
        CLASS_NAME,
        "setGlobalBufferSize",
        "(J)V",
        set_global_buffer_size,
    );
    registry.register(CLASS_NAME, "setMemorySize", "(J)V", set_memory_size);
    registry.register(
        CLASS_NAME,
        "setMethodSamplingPeriod",
        "(JJ)V",
        set_method_sampling_period,
    );
    registry.register(CLASS_NAME, "setOutput", "(Ljava/lang/String;)V", set_output);
    registry.register(
        CLASS_NAME,
        "setRepositoryLocation",
        "(Ljava/lang/String;)V",
        set_repository_location,
    );
    registry.register(CLASS_NAME, "setStackDepth", "(I)V", set_stack_depth);
    registry.register(
        CLASS_NAME,
        "setStackTraceEnabled",
        "(JZ)V",
        set_stack_trace_enabled,
    );
    registry.register(
        CLASS_NAME,
        "setThreadBufferSize",
        "(J)V",
        set_thread_buffer_size,
    );
    registry.register(CLASS_NAME, "setThreshold", "(JJ)Z", set_threshold);
    registry.register(CLASS_NAME, "setThrottle", "(JJJ)Z", set_throttle);
    registry.register(CLASS_NAME, "shouldRotateDisk", "()Z", should_rotate_disk);
    registry.register(
        CLASS_NAME,
        "storeMetadataDescriptor",
        "([B)V",
        store_metadata_descriptor,
    );
    registry.register(
        CLASS_NAME,
        "subscribeLogLevel",
        "(Ljdk/jfr/internal/LogTag;I)V",
        subscribe_log_level,
    );
    registry.register(
        CLASS_NAME,
        "uncaughtException",
        "(Ljava/lang/Thread;Ljava/lang/Throwable;)V",
        uncaught_exception,
    );
}

#[async_recursion(?Send)]
async fn abort(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.abort(Ljava/lang/String;)V")
}

#[async_recursion(?Send)]
async fn add_string_constant(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.addStringConstant(JLjava/lang/String;)Z")
}

#[async_recursion(?Send)]
async fn begin_recording(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.beginRecording()V")
}

#[async_recursion(?Send)]
async fn commit(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.commit(J)J")
}

#[async_recursion(?Send)]
async fn counter_time(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.counterTime()J")
}

#[async_recursion(?Send)]
async fn create_jfr(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.createJFR(Z)Z")
}

#[async_recursion(?Send)]
async fn destroy_jfr(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.destroyJFR()Z")
}

#[async_recursion(?Send)]
async fn emit_data_loss(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.emitDataLoss(J)V")
}

#[async_recursion(?Send)]
async fn emit_event(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.emitEvent(JJJ)Z")
}

#[async_recursion(?Send)]
async fn emit_old_object_samples(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.emitOldObjectSamples(JZZ)V")
}

#[async_recursion(?Send)]
async fn end_recording(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.endRecording()V")
}

#[async_recursion(?Send)]
async fn exclude(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.exclude(Ljava/lang/Thread;)V")
}

#[async_recursion(?Send)]
async fn flush(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.flush()")
}

#[async_recursion(?Send)]
async fn get_all_event_classes(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.getAllEventClasses()Ljava/util/List;")
}

#[async_recursion(?Send)]
async fn get_allowed_to_do_event_retransforms(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.getAllowedToDoEventRetransforms()Z")
}

#[async_recursion(?Send)]
async fn get_chunk_start_nanos(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.getChunkStartNanos()J")
}

#[async_recursion(?Send)]
async fn get_class_id(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.getClassId(Ljava/lang/Class;)J")
}

#[async_recursion(?Send)]
async fn get_class_id_non_intrinsic(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.getClassIdNonIntrinsic(Ljava/lang/Class;)J")
}

#[async_recursion(?Send)]
async fn get_configuration(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.getConfiguration(Ljava/lang/Class;)Ljava/lang/Object;")
}

#[async_recursion(?Send)]
async fn get_dump_path(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.getDumpPath()Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn get_event_writer(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.getEventWriter()Ljava/lang/Object;")
}

#[async_recursion(?Send)]
async fn get_handler(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.getHandler(Ljava/lang/Class;)Ljava/lang")
}

#[async_recursion(?Send)]
async fn get_pid(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.getPid()Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn get_stack_trace_id(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.getStackTraceId(IJ)J")
}

#[async_recursion(?Send)]
async fn get_thread_id(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.getThreadId(Ljava/lang/Thread;)J")
}

#[async_recursion(?Send)]
async fn get_ticks_frequency(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.getTicksFrequency()J")
}

#[async_recursion(?Send)]
async fn get_time_conversion_factor(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.getTimeConversionFactor()D")
}

#[async_recursion(?Send)]
async fn get_type_id(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.getTypeId(Ljava/lang/String;)J")
}

#[async_recursion(?Send)]
async fn get_unloaded_event_class_count(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.getUnloadedEventClassCount()J")
}

#[async_recursion(?Send)]
async fn host_total_memory(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.hostTotalMemory()J")
}

#[async_recursion(?Send)]
async fn host_total_swap_memory(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.hostTotalSwapMemory()J")
}

#[async_recursion(?Send)]
async fn include(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.include(Ljava/lang/Thread;)V")
}

#[async_recursion(?Send)]
async fn is_available(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.isAvailable()Z")
}

#[async_recursion(?Send)]
async fn is_containerized(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.isContainerized()Z")
}

#[async_recursion(?Send)]
async fn is_excluded(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.isExcluded(Ljava/lang/Thread;)Z")
}

#[async_recursion(?Send)]
async fn is_instrumented(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.isInstrumented(Ljava/lang/Class;)Z")
}

#[async_recursion(?Send)]
async fn is_recording(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.isRecording()Z")
}

#[async_recursion(?Send)]
async fn log(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.log(IILjava/lang/String;)V")
}

#[async_recursion(?Send)]
async fn log_event(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.logEvent(I[Ljava/lang/String;Z)V")
}

#[async_recursion(?Send)]
async fn mark_chunk_final(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.markChunkFinal()V")
}

#[async_recursion(?Send)]
async fn nanos_now(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.nanosNow()J")
}

#[async_recursion(?Send)]
async fn new_event_writer(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.newEventWriter()Ljdk/jfr/internal/EventWriter;")
}

#[async_recursion(?Send)]
async fn register_natives(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn register_stack_filter(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.registerStackFilter([Ljava/lang/String;[Ljava/lang/String;)J")
}

#[async_recursion(?Send)]
async fn retransform_classes(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.retransformClasses([Ljava/lang/Class;)V")
}

#[async_recursion(?Send)]
async fn set_compressed_integers(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.setCompressedIntegers(Z)V")
}

#[async_recursion(?Send)]
async fn set_configuration(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.setConfiguration(Ljava/lang/Class;Ljdk/jfr/internal/event/EventConfiguration;)Z")
}

#[async_recursion(?Send)]
async fn set_cutoff(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.setCutoff(JJ)Z")
}

#[async_recursion(?Send)]
async fn set_dump_path(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.setDumpPath(Ljava/lang/String;)V")
}

#[async_recursion(?Send)]
async fn set_enabled(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.setEnabled(JZ)V")
}

#[async_recursion(?Send)]
async fn set_file_notification(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.setFileNotification(J)V")
}

#[async_recursion(?Send)]
async fn set_force_instrumentation(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.setForceInstrumentation(Z)V")
}

#[async_recursion(?Send)]
async fn set_global_buffer_count(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.setGlobalBufferCount(J)V")
}

#[async_recursion(?Send)]
async fn set_global_buffer_size(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.setGlobalBufferSize(J)V")
}

#[async_recursion(?Send)]
async fn set_handler(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.setHandler(Ljava/lang/Class;Ljdk/jfr/internal/handlers/EventHandler;)Z")
}

#[async_recursion(?Send)]
async fn set_memory_size(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.setMemorySize(J)V")
}

#[async_recursion(?Send)]
async fn set_method_sampling_interval(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.setMethodSamplingInterval(JJ)V")
}

#[async_recursion(?Send)]
async fn set_method_sampling_period(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.setMethodSamplingPeriod(JJ)V")
}

#[async_recursion(?Send)]
async fn set_miscellaneous(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.setMiscellaneous(JJ)V")
}

#[async_recursion(?Send)]
async fn set_output(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.setOutput(Ljava/lang/String;)V")
}

#[async_recursion(?Send)]
async fn set_repository_location(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.setRepositoryLocation(Ljava/lang/String;)V")
}

#[async_recursion(?Send)]
async fn set_sample_threads(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.setSampleThreads(Z)V")
}

#[async_recursion(?Send)]
async fn set_stack_depth(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.setStackDepth(I)V")
}

#[async_recursion(?Send)]
async fn set_stack_trace_enabled(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.setStackTraceEnabled(JZ)V")
}

#[async_recursion(?Send)]
async fn set_thread_buffer_size(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.setThreadBufferSize(J)V")
}

#[async_recursion(?Send)]
async fn set_threshold(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.setThreshold(JJ)Z")
}

#[async_recursion(?Send)]
async fn set_throttle(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.setThrottle(JJJ)Z")
}

#[async_recursion(?Send)]
async fn should_rotate_disk(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.shouldRotateDisk()Z")
}

#[async_recursion(?Send)]
async fn store_metadata_descriptor(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.storeMetadataDescriptor([B)V")
}

#[async_recursion(?Send)]
async fn subscribe_log_level(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.subscribeLogLevel(Ljdk/jfr/internal/LogTag;I)V")
}

#[async_recursion(?Send)]
async fn uncaught_exception(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.uncaughtException(Ljava/lang/Thread;Ljava/lang/Throwable;)V")
}

#[async_recursion(?Send)]
async fn unregister_stack_filter(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.unregisterStackFilter(J)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.abort(Ljava/lang/String;)V"
    )]
    async fn test_abort() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = abort(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.addStringConstant(JLjava/lang/String;)Z"
    )]
    async fn test_add_string_constant() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = add_string_constant(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.jfr.internal.JVM.beginRecording()V")]
    async fn test_begin_recording() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = begin_recording(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.jfr.internal.JVM.commit(J)J")]
    async fn test_commit() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = commit(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.jfr.internal.JVM.counterTime()J")]
    async fn test_counter_time() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = counter_time(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.jfr.internal.JVM.createJFR(Z)Z")]
    async fn test_create_jfr() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = create_jfr(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.jfr.internal.JVM.destroyJFR()Z")]
    async fn test_destroy_jfr() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = destroy_jfr(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.jfr.internal.JVM.emitDataLoss(J)V")]
    async fn test_emit_data_loss() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = emit_data_loss(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.jfr.internal.JVM.emitEvent(JJJ)Z")]
    async fn test_emit_event() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = emit_event(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.emitOldObjectSamples(JZZ)V"
    )]
    async fn test_emit_old_object_samples() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = emit_old_object_samples(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.jfr.internal.JVM.endRecording()V")]
    async fn test_end_recording() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = end_recording(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.exclude(Ljava/lang/Thread;)V"
    )]
    async fn test_exclude() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = exclude(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.jfr.internal.JVM.flush()")]
    async fn test_flush() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = flush(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.getAllEventClasses()Ljava/util/List;"
    )]
    async fn test_get_all_event_classes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_all_event_classes(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.getAllowedToDoEventRetransforms()Z"
    )]
    async fn test_get_allowed_to_do_event_retransforms() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_allowed_to_do_event_retransforms(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.jfr.internal.JVM.getChunkStartNanos()J")]
    async fn test_get_chunk_start_nanos() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_chunk_start_nanos(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.getClassId(Ljava/lang/Class;)J"
    )]
    async fn test_get_class_id() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_class_id(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.getClassIdNonIntrinsic(Ljava/lang/Class;)J"
    )]
    async fn test_get_class_id_non_intrinsic() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_class_id_non_intrinsic(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.getConfiguration(Ljava/lang/Class;)Ljava/lang/Object;"
    )]
    async fn test_get_configuration() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_configuration(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.getDumpPath()Ljava/lang/String;"
    )]
    async fn test_get_dump_path() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_dump_path(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.getEventWriter()Ljava/lang/Object;"
    )]
    async fn test_get_event_writer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_event_writer(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.getHandler(Ljava/lang/Class;)Ljava/lang"
    )]
    async fn test_get_handler() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_handler(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.getPid()Ljava/lang/String;"
    )]
    async fn test_get_pid() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_pid(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.jfr.internal.JVM.getStackTraceId(IJ)J")]
    async fn test_get_stack_trace_id() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_stack_trace_id(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.getThreadId(Ljava/lang/Thread;)J"
    )]
    async fn test_get_thread_id() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_thread_id(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.jfr.internal.JVM.getTicksFrequency()J")]
    async fn test_get_ticks_frequency() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_ticks_frequency(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.getTimeConversionFactor()D"
    )]
    async fn test_get_time_conversion_factor() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_time_conversion_factor(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.getTypeId(Ljava/lang/String;)J"
    )]
    async fn test_get_type_id() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_type_id(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.getUnloadedEventClassCount()J"
    )]
    async fn test_get_unloaded_event_class_count() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_unloaded_event_class_count(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.jfr.internal.JVM.hostTotalMemory()J")]
    async fn test_host_total_memory() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = host_total_memory(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.jfr.internal.JVM.hostTotalSwapMemory()J")]
    async fn test_host_total_swap_memory() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = host_total_swap_memory(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.include(Ljava/lang/Thread;)V"
    )]
    async fn test_include() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = include(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.jfr.internal.JVM.isAvailable()Z")]
    async fn test_is_available() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_available(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.jfr.internal.JVM.isContainerized()Z")]
    async fn test_is_containerized() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_containerized(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.isExcluded(Ljava/lang/Thread;)Z"
    )]
    async fn test_is_excluded() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_excluded(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.isInstrumented(Ljava/lang/Class;)Z"
    )]
    async fn test_is_instrumented() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_instrumented(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.jfr.internal.JVM.isRecording()Z")]
    async fn test_is_recording() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_recording(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.log(IILjava/lang/String;)V"
    )]
    async fn test_log() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = log(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.logEvent(I[Ljava/lang/String;Z)V"
    )]
    async fn test_log_event() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = log_event(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.jfr.internal.JVM.markChunkFinal()V")]
    async fn test_mark_chunk_final() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = mark_chunk_final(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.jfr.internal.JVM.nanosNow()J")]
    async fn test_nanos_now() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = nanos_now(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.newEventWriter()Ljdk/jfr/internal/EventWriter;"
    )]
    async fn test_new_event_writer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = new_event_writer(thread, Arguments::default()).await;
    }

    #[tokio::test]
    async fn test_register_natives() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = register_natives(thread, Arguments::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.registerStackFilter([Ljava/lang/String;[Ljava/lang/String;)J"
    )]
    async fn test_register_stack_filter() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = register_stack_filter(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.retransformClasses([Ljava/lang/Class;)V"
    )]
    async fn test_retransform_classes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = retransform_classes(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.setCompressedIntegers(Z)V"
    )]
    async fn test_set_compressed_integers() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_compressed_integers(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.setConfiguration(Ljava/lang/Class;Ljdk/jfr/internal/event/EventConfiguration;)Z"
    )]
    async fn test_set_configuration() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_configuration(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.jfr.internal.JVM.setCutoff(JJ)Z")]
    async fn test_set_cutoff() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_cutoff(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.setDumpPath(Ljava/lang/String;)V"
    )]
    async fn test_set_dump_path() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_dump_path(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.jfr.internal.JVM.setEnabled(JZ)V")]
    async fn test_set_enabled() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_enabled(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.jfr.internal.JVM.setFileNotification(J)V")]
    async fn test_set_file_notification() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_file_notification(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.setForceInstrumentation(Z)V"
    )]
    async fn test_set_force_instrumentation() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_force_instrumentation(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.jfr.internal.JVM.setGlobalBufferCount(J)V")]
    async fn test_set_global_buffer_count() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_global_buffer_count(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.jfr.internal.JVM.setGlobalBufferSize(J)V")]
    async fn test_set_global_buffer_size() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_global_buffer_size(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.setHandler(Ljava/lang/Class;Ljdk/jfr/internal/handlers/EventHandler;)Z"
    )]
    async fn test_set_handler() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_handler(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.jfr.internal.JVM.setMemorySize(J)V")]
    async fn test_set_memory_size() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_memory_size(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.setMethodSamplingInterval(JJ)V"
    )]
    async fn test_set_method_sampling_interval() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_method_sampling_interval(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.setMethodSamplingPeriod(JJ)V"
    )]
    async fn test_set_method_sampling_period() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_method_sampling_period(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.jfr.internal.JVM.setMiscellaneous(JJ)V")]
    async fn test_set_miscellaneous() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_miscellaneous(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.setOutput(Ljava/lang/String;)V"
    )]
    async fn test_set_output() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_output(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.setRepositoryLocation(Ljava/lang/String;)V"
    )]
    async fn test_set_repository_location() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_repository_location(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.jfr.internal.JVM.setSampleThreads(Z)V")]
    async fn test_set_sample_threads() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_sample_threads(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.jfr.internal.JVM.setStackDepth(I)V")]
    async fn test_set_stack_depth() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_stack_depth(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.setStackTraceEnabled(JZ)V"
    )]
    async fn test_set_stack_trace_enabled() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_stack_trace_enabled(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.jfr.internal.JVM.setThreadBufferSize(J)V")]
    async fn test_set_thread_buffer_size() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_thread_buffer_size(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.jfr.internal.JVM.setThreshold(JJ)Z")]
    async fn test_set_threshold() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_threshold(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.jfr.internal.JVM.setThrottle(JJJ)Z")]
    async fn test_set_throttle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_throttle(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.jfr.internal.JVM.shouldRotateDisk()Z")]
    async fn test_should_rotate_disk() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = should_rotate_disk(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.storeMetadataDescriptor([B)V"
    )]
    async fn test_store_metadata_descriptor() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = store_metadata_descriptor(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.subscribeLogLevel(Ljdk/jfr/internal/LogTag;I)V"
    )]
    async fn test_subscribe_log_level() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = subscribe_log_level(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.uncaughtException(Ljava/lang/Thread;Ljava/lang/Throwable;)V"
    )]
    async fn test_uncaught_exception() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = uncaught_exception(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.unregisterStackFilter(J)V"
    )]
    async fn test_unregister_stack_filter() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = unregister_stack_filter(thread, Arguments::default()).await;
    }
}
