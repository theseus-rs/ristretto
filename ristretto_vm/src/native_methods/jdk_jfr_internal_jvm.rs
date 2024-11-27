use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_11: Version = Version::Java11 { minor: 0 };
const JAVA_17: Version = Version::Java17 { minor: 0 };
const JAVA_18: Version = Version::Java18 { minor: 0 };
const JAVA_20: Version = Version::Java20 { minor: 0 };
const JAVA_21: Version = Version::Java21 { minor: 0 };
const JAVA_22: Version = Version::Java22 { minor: 0 };
const JAVA_23: Version = Version::Java23 { minor: 0 };

/// Register all native methods for `jdk.jfr.internal.JVM`.
#[expect(clippy::too_many_lines)]
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "jdk/jfr/internal/JVM";
    let java_version = registry.java_version().clone();

    if java_version <= JAVA_11 {
        registry.register(
            class_name,
            "emitOldObjectSamples",
            "(JZ)V",
            emit_old_object_samples,
        );
        registry.register(
            class_name,
            "getClassIdNonIntrinsic",
            "(Ljava/lang/Class;)J",
            get_class_id_non_intrinsic,
        );
        registry.register(
            class_name,
            "setMethodSamplingInterval",
            "(JJ)V",
            set_method_sampling_interval,
        );
    } else {
        registry.register(
            class_name,
            "emitOldObjectSamples",
            "(JZZ)V",
            emit_old_object_samples,
        );
        registry.register(class_name, "exclude", "(Ljava/lang/Thread;)V", exclude);
        registry.register(class_name, "flush", "()V", flush);
        registry.register(
            class_name,
            "getChunkStartNanos",
            "()J",
            get_chunk_start_nanos,
        );
        registry.register(
            class_name,
            "getHandler",
            "(Ljava/lang/Class;)Ljava/lang/Object;",
            get_handler,
        );
        registry.register(
            class_name,
            "getTypeId",
            "(Ljava/lang/String;)J",
            get_type_id,
        );
        registry.register(class_name, "include", "(Ljava/lang/Thread;)V", include);
        registry.register(
            class_name,
            "isExcluded",
            "(Ljava/lang/Thread;)Z",
            is_excluded,
        );
        registry.register(class_name, "isRecording", "()Z", is_recording);
        registry.register(
            class_name,
            "logEvent",
            "(I[Ljava/lang/String;Z)V",
            log_event,
        );
        registry.register(class_name, "markChunkFinal", "()V", mark_chunk_final);
        registry.register(
            class_name,
            "setHandler",
            "(Ljava/lang/Class;Ljdk/jfr/internal/handlers/EventHandler;)Z",
            set_handler,
        );
        registry.register(class_name, "setThrottle", "(JJJ)Z", set_throttle);
    }

    if java_version == JAVA_17 {
        registry.register(class_name, "emitDataLoss", "(J)V", emit_data_loss);
        registry.register(
            class_name,
            "setMethodSamplingPeriod",
            "(JJ)V",
            set_method_sampling_period,
        );
    }

    if java_version >= JAVA_18 {
        registry.register(
            class_name,
            "getDumpPath",
            "()Ljava/lang/String;",
            get_dump_path,
        );
        registry.register(
            class_name,
            "setDumpPath",
            "(Ljava/lang/String;)V",
            set_dump_path,
        );
        registry.register(
            class_name,
            "setMethodSamplingInterval",
            "(JJ)V",
            set_method_sampling_interval,
        );
    }

    if java_version >= JAVA_18 {
        registry.register(
            class_name,
            "flush",
            "(Ljdk/jfr/internal/EventWriter;II)Z",
            flush,
        );
        registry.register(
            class_name,
            "getEventWriter",
            "()Ljava/lang/Object;",
            get_event_writer,
        );
        registry.register(
            class_name,
            "newEventWriter",
            "()Ljdk/jfr/internal/EventWriter;",
            new_event_writer,
        );
        registry.register(class_name, "setSampleThreads", "(Z)V", set_sample_threads);
    } else {
        registry.register(class_name, "exclude", "(Ljava/lang/Thread;)V", exclude);
        registry.register(class_name, "flush", "()V", flush);

        if java_version <= JAVA_20 {
            registry.register(
                class_name,
                "flush",
                "(Ljdk/jfr/internal/event/EventWriter;II)Z",
                flush,
            );
        }

        registry.register(
            class_name,
            "getConfiguration",
            "(Ljava/lang/Class;)Ljava/lang/Object;",
            get_configuration,
        );
        registry.register(
            class_name,
            "getEventWriter",
            "()Ljdk/jfr/internal/event/EventWriter;",
            get_event_writer,
        );
        registry.register(class_name, "isContainerized", "()Z", is_containerized);
        registry.register(
            class_name,
            "isExcluded",
            "(Ljava/lang/Class;)Z",
            is_excluded,
        );
        registry.register(
            class_name,
            "isExcluded",
            "(Ljava/lang/Thread;)Z",
            is_excluded,
        );
        registry.register(
            class_name,
            "isInstrumented",
            "(Ljava/lang/Class;)Z",
            is_instrumented,
        );
        registry.register(class_name, "isRecording", "()Z", is_recording);
        registry.register(
            class_name,
            "newEventWriter",
            "()Ljdk/jfr/internal/event/EventWriter;",
            new_event_writer,
        );
        registry.register(
            class_name,
            "setConfiguration",
            "(Ljava/lang/Class;Ljdk/jfr/internal/event/EventConfiguration;)Z",
            set_configuration,
        );
    }

    if java_version >= JAVA_20 {
        registry.register(class_name, "hostTotalMemory", "()J", host_total_memory);
    }

    if java_version >= JAVA_21 {
        registry.register(class_name, "commit", "(J)J", commit);
        registry.register(class_name, "emitDataLoss", "(J)V", emit_data_loss);
        registry.register(
            class_name,
            "flush",
            "(Ljdk/jfr/internal/event/EventWriter;II)V",
            flush,
        );
    }

    if java_version <= JAVA_21 {
        registry.register(class_name, "getStackTraceId", "(I)J", get_stack_trace_id);
    } else {
        registry.register(class_name, "getStackTraceId", "(IJ)J", get_stack_trace_id);
    }

    if java_version >= JAVA_22 {
        registry.register(
            class_name,
            "registerStackFilter",
            "([Ljava/lang/String;[Ljava/lang/String;)J",
            register_stack_filter,
        );
        registry.register(class_name, "setMiscellaneous", "(JJ)V", set_miscellaneous);
        registry.register(
            class_name,
            "unregisterStackFilter",
            "(J)V",
            unregister_stack_filter,
        );
    }

    if java_version >= JAVA_23 {
        registry.register(
            class_name,
            "hostTotalSwapMemory",
            "()J",
            host_total_swap_memory,
        );
        registry.register(class_name, "nanosNow", "()J", nanos_now);
    }

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
    registry.register(class_name, "endRecording", "()V", end_recording);
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
    registry.register(class_name, "getPid", "()Ljava/lang/String;", get_pid);
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
        "setMethodSamplingPeriod",
        "(JJ)V",
        set_method_sampling_period,
    );
    registry.register(class_name, "setOutput", "(Ljava/lang/String;)V", set_output);
    registry.register(
        class_name,
        "setRepositoryLocation",
        "(Ljava/lang/String;)V",
        set_repository_location,
    );
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
    registry.register(class_name, "setThrottle", "(JJJ)Z", set_throttle);
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
async fn commit(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
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
async fn emit_data_loss(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
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
async fn exclude(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
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
async fn get_chunk_start_nanos(
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
async fn get_configuration(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_dump_path(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_event_writer(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_handler(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
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
async fn host_total_memory(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn host_total_swap_memory(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn include(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn is_available(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn is_containerized(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn is_excluded(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn is_instrumented(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn is_recording(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn log(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn log_event(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn mark_chunk_final(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn nanos_now(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
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
async fn register_stack_filter(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
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
async fn set_configuration(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn set_cutoff(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn set_dump_path(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
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
async fn set_handler(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
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
async fn set_method_sampling_period(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn set_miscellaneous(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
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
async fn set_throttle(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
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

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn unregister_stack_filter(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}
