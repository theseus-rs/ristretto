use ristretto_classfile::VersionSpecification::{
    Between, Equal, GreaterThan, GreaterThanOrEqual, LessThanOrEqual,
};
use ristretto_classfile::{JAVA_11, JAVA_17, JAVA_21, JAVA_25};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "jdk/jfr/internal/JVM.abort(Ljava/lang/String;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn abort<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.jfr.internal.JVM.abort(Ljava/lang/String;)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.addStringConstant(JLjava/lang/String;)Z",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn add_string_constant<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.jfr.internal.JVM.addStringConstant(JLjava/lang/String;)Z".to_string(),
    )
    .into())
}

#[intrinsic_method("jdk/jfr/internal/JVM.beginRecording()V", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn begin_recording<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("jdk.jfr.internal.JVM.beginRecording()V".to_string())
            .into(),
    )
}

#[intrinsic_method("jdk/jfr/internal/JVM.commit(J)J", GreaterThanOrEqual(JAVA_21))]
#[async_method]
pub async fn commit<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("jdk.jfr.internal.JVM.commit(J)J".to_string()).into())
}

#[intrinsic_method("jdk/jfr/internal/JVM.counterTime()J", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn counter_time<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("jdk.jfr.internal.JVM.counterTime()J".to_string()).into())
}

#[intrinsic_method("jdk/jfr/internal/JVM.createJFR(Z)Z", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn create_jfr<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("jdk.jfr.internal.JVM.createJFR(Z)Z".to_string()).into())
}

#[intrinsic_method("jdk/jfr/internal/JVM.destroyJFR()Z", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn destroy_jfr<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("jdk.jfr.internal.JVM.destroyJFR()Z".to_string()).into())
}

#[intrinsic_method("jdk/jfr/internal/JVM.emitDataLoss(J)V", GreaterThanOrEqual(JAVA_17))]
#[async_method]
pub async fn emit_data_loss<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("jdk.jfr.internal.JVM.emitDataLoss(J)V".to_string()).into())
}

#[intrinsic_method("jdk/jfr/internal/JVM.emitEvent(JJJ)Z", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn emit_event<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("jdk.jfr.internal.JVM.emitEvent(JJJ)Z".to_string()).into())
}

#[intrinsic_method("jdk/jfr/internal/JVM.emitOldObjectSamples(JZ)V", Equal(JAVA_11))]
#[async_method]
pub async fn emit_old_object_samples_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.jfr.internal.JVM.emitOldObjectSamples(JZ)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.emitOldObjectSamples(JZZ)V",
    GreaterThan(JAVA_11)
)]
#[async_method]
pub async fn emit_old_object_samples_1<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.jfr.internal.JVM.emitOldObjectSamples(JZZ)V".to_string(),
    )
    .into())
}

#[intrinsic_method("jdk/jfr/internal/JVM.endRecording()V", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn end_recording<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("jdk.jfr.internal.JVM.endRecording()V".to_string()).into())
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.exclude(Ljava/lang/Thread;)V",
    GreaterThan(JAVA_11)
)]
#[async_method]
pub async fn exclude<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.jfr.internal.JVM.exclude(Ljava/lang/Thread;)V".to_string(),
    )
    .into())
}

#[intrinsic_method("jdk/jfr/internal/JVM.flush()V", GreaterThan(JAVA_11))]
#[async_method]
pub async fn flush_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("jdk.jfr.internal.JVM.flush()V".to_string()).into())
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.flush(Ljdk/jfr/internal/EventWriter;II)Z",
    Between(JAVA_11, JAVA_17)
)]
#[async_method]
pub async fn flush_1<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.jfr.internal.JVM.flush(Ljdk/jfr/internal/EventWriter;II)Z".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.flush(Ljdk/jfr/internal/event/EventWriter;II)V",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn flush_2<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.jfr.internal.JVM.flush(Ljdk/jfr/internal/event/EventWriter;II)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.getAllEventClasses()Ljava/util/List;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_all_event_classes<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.jfr.internal.JVM.getAllEventClasses()Ljava/util/List;".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.getAllowedToDoEventRetransforms()Z",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_allowed_to_do_event_retransforms<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.jfr.internal.JVM.getAllowedToDoEventRetransforms()Z".to_string(),
    )
    .into())
}

#[intrinsic_method("jdk/jfr/internal/JVM.getChunkStartNanos()J", GreaterThan(JAVA_11))]
#[async_method]
pub async fn get_chunk_start_nanos<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("jdk.jfr.internal.JVM.getChunkStartNanos()J".to_string())
            .into(),
    )
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.getClassId(Ljava/lang/Class;)J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_class_id<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.jfr.internal.JVM.getClassId(Ljava/lang/Class;)J".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.getClassIdNonIntrinsic(Ljava/lang/Class;)J",
    Equal(JAVA_11)
)]
#[async_method]
pub async fn get_class_id_non_intrinsic<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.jfr.internal.JVM.getClassIdNonIntrinsic(Ljava/lang/Class;)J".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.getConfiguration(Ljava/lang/Class;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn get_configuration<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.jfr.internal.JVM.getConfiguration(Ljava/lang/Class;)Ljava/lang/Object;".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.getDumpPath()Ljava/lang/String;",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn get_dump_path<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.jfr.internal.JVM.getDumpPath()Ljava/lang/String;".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.getEventWriter()Ljava/lang/Object;",
    Between(JAVA_11, JAVA_17)
)]
#[async_method]
pub async fn get_event_writer_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.jfr.internal.JVM.getEventWriter()Ljava/lang/Object;".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.getEventWriter()Ljdk/jfr/internal/event/EventWriter;",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn get_event_writer_1<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.jfr.internal.JVM.getEventWriter()Ljdk/jfr/internal/event/EventWriter;".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.getHandler(Ljava/lang/Class;)Ljava/lang/Object;",
    Equal(JAVA_17)
)]
#[async_method]
pub async fn get_handler<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.jfr.internal.JVM.getHandler(Ljava/lang/Class;)Ljava/lang/Object;".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.getPid()Ljava/lang/String;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_pid<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.jfr.internal.JVM.getPid()Ljava/lang/String;".to_string(),
    )
    .into())
}

#[intrinsic_method("jdk/jfr/internal/JVM.getStackTraceId(I)J", LessThanOrEqual(JAVA_21))]
#[async_method]
pub async fn get_stack_trace_id_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("jdk.jfr.internal.JVM.getStackTraceId(I)J".to_string())
            .into(),
    )
}

#[intrinsic_method("jdk/jfr/internal/JVM.getStackTraceId(IJ)J", GreaterThan(JAVA_21))]
#[async_method]
pub async fn get_stack_trace_id_1<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("jdk.jfr.internal.JVM.getStackTraceId(IJ)J".to_string())
            .into(),
    )
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.getThreadId(Ljava/lang/Thread;)J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_thread_id<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.jfr.internal.JVM.getThreadId(Ljava/lang/Thread;)J".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.getTicksFrequency()J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_ticks_frequency<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("jdk.jfr.internal.JVM.getTicksFrequency()J".to_string())
            .into(),
    )
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.getTimeConversionFactor()D",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_time_conversion_factor<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.jfr.internal.JVM.getTimeConversionFactor()D".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.getTypeId(Ljava/lang/String;)J",
    GreaterThan(JAVA_11)
)]
#[async_method]
pub async fn get_type_id_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.jfr.internal.JVM.getTypeId(Ljava/lang/String;)J".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.getTypeId(Ljava/lang/Class;)J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_type_id_1<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.jfr.internal.JVM.getTypeId(Ljava/lang/Class;)J".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.getUnloadedEventClassCount()J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_unloaded_event_class_count<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.jfr.internal.JVM.getUnloadedEventClassCount()J".to_string(),
    )
    .into())
}

#[intrinsic_method("jdk/jfr/internal/JVM.hostTotalMemory()J", GreaterThanOrEqual(JAVA_21))]
#[async_method]
pub async fn host_total_memory<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("jdk.jfr.internal.JVM.hostTotalMemory()J".to_string())
            .into(),
    )
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.hostTotalSwapMemory()J",
    GreaterThanOrEqual(JAVA_25)
)]
#[async_method]
pub async fn host_total_swap_memory<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("jdk.jfr.internal.JVM.hostTotalSwapMemory()J".to_string())
            .into(),
    )
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.include(Ljava/lang/Thread;)V",
    GreaterThan(JAVA_11)
)]
#[async_method]
pub async fn include<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.jfr.internal.JVM.include(Ljava/lang/Thread;)V".to_string(),
    )
    .into())
}

#[intrinsic_method("jdk/jfr/internal/JVM.isAvailable()Z", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn is_available<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("jdk.jfr.internal.JVM.isAvailable()Z".to_string()).into())
}

#[intrinsic_method("jdk/jfr/internal/JVM.isContainerized()Z", GreaterThanOrEqual(JAVA_21))]
#[async_method]
pub async fn is_containerized<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("jdk.jfr.internal.JVM.isContainerized()Z".to_string())
            .into(),
    )
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.isExcluded(Ljava/lang/Thread;)Z",
    GreaterThan(JAVA_11)
)]
#[async_method]
pub async fn is_excluded_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.jfr.internal.JVM.isExcluded(Ljava/lang/Thread;)Z".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.isExcluded(Ljava/lang/Class;)Z",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn is_excluded_1<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.jfr.internal.JVM.isExcluded(Ljava/lang/Class;)Z".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.isInstrumented(Ljava/lang/Class;)Z",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn is_instrumented<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.jfr.internal.JVM.isInstrumented(Ljava/lang/Class;)Z".to_string(),
    )
    .into())
}

#[intrinsic_method("jdk/jfr/internal/JVM.isRecording()Z", GreaterThan(JAVA_11))]
#[async_method]
pub async fn is_recording<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("jdk.jfr.internal.JVM.isRecording()Z".to_string()).into())
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.log(IILjava/lang/String;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn log<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.jfr.internal.JVM.log(IILjava/lang/String;)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.logEvent(I[Ljava/lang/String;Z)V",
    GreaterThan(JAVA_11)
)]
#[async_method]
pub async fn log_event<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.jfr.internal.JVM.logEvent(I[Ljava/lang/String;Z)V".to_string(),
    )
    .into())
}

#[intrinsic_method("jdk/jfr/internal/JVM.markChunkFinal()V", GreaterThan(JAVA_11))]
#[async_method]
pub async fn mark_chunk_final<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("jdk.jfr.internal.JVM.markChunkFinal()V".to_string())
            .into(),
    )
}

#[intrinsic_method("jdk/jfr/internal/JVM.nanosNow()J", GreaterThanOrEqual(JAVA_25))]
#[async_method]
pub async fn nanos_now<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("jdk.jfr.internal.JVM.nanosNow()J".to_string()).into())
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.newEventWriter()Ljdk/jfr/internal/EventWriter;",
    Between(JAVA_11, JAVA_17)
)]
#[async_method]
pub async fn new_event_writer_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.jfr.internal.JVM.newEventWriter()Ljdk/jfr/internal/EventWriter;".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.newEventWriter()Ljdk/jfr/internal/event/EventWriter;",
    GreaterThan(JAVA_11)
)]
#[async_method]
pub async fn new_event_writer_1<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.jfr.internal.JVM.newEventWriter()Ljdk/jfr/internal/event/EventWriter;".to_string(),
    )
    .into())
}

#[intrinsic_method("jdk/jfr/internal/JVM.registerNatives()V", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn register_natives<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.registerStackFilter([Ljava/lang/String;[Ljava/lang/String;)J",
    GreaterThanOrEqual(JAVA_25)
)]
#[async_method]
pub async fn register_stack_filter<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.jfr.internal.JVM.registerStackFilter([Ljava/lang/String;[Ljava/lang/String;)J"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.retransformClasses([Ljava/lang/Class;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn retransform_classes<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.jfr.internal.JVM.retransformClasses([Ljava/lang/Class;)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.setCompressedIntegers(Z)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn set_compressed_integers<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.jfr.internal.JVM.setCompressedIntegers(Z)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.setConfiguration(Ljava/lang/Class;Ljdk/jfr/internal/event/EventConfiguration;)Z",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn set_configuration<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("jdk.jfr.internal.JVM.setConfiguration(Ljava/lang/Class;Ljdk/jfr/internal/event/EventConfiguration;)Z".to_string()).into())
}

#[intrinsic_method("jdk/jfr/internal/JVM.setCutoff(JJ)Z", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn set_cutoff<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("jdk.jfr.internal.JVM.setCutoff(JJ)Z".to_string()).into())
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.setDumpPath(Ljava/lang/String;)V",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn set_dump_path<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.jfr.internal.JVM.setDumpPath(Ljava/lang/String;)V".to_string(),
    )
    .into())
}

#[intrinsic_method("jdk/jfr/internal/JVM.setEnabled(JZ)V", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn set_enabled<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("jdk.jfr.internal.JVM.setEnabled(JZ)V".to_string()).into())
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.setFileNotification(J)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn set_file_notification<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("jdk.jfr.internal.JVM.setFileNotification(J)V".to_string())
            .into(),
    )
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.setForceInstrumentation(Z)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn set_force_instrumentation<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.jfr.internal.JVM.setForceInstrumentation(Z)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.setGlobalBufferCount(J)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn set_global_buffer_count<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.jfr.internal.JVM.setGlobalBufferCount(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.setGlobalBufferSize(J)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn set_global_buffer_size<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("jdk.jfr.internal.JVM.setGlobalBufferSize(J)V".to_string())
            .into(),
    )
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.setHandler(Ljava/lang/Class;Ljdk/jfr/internal/handlers/EventHandler;)Z",
    Equal(JAVA_17)
)]
#[async_method]
pub async fn set_handler<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("jdk.jfr.internal.JVM.setHandler(Ljava/lang/Class;Ljdk/jfr/internal/handlers/EventHandler;)Z".to_string()).into())
}

#[intrinsic_method("jdk/jfr/internal/JVM.setMemorySize(J)V", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn set_memory_size<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("jdk.jfr.internal.JVM.setMemorySize(J)V".to_string())
            .into(),
    )
}

#[intrinsic_method("jdk/jfr/internal/JVM.setMethodSamplingInterval(JJ)V", Equal(JAVA_11))]
#[async_method]
pub async fn set_method_sampling_interval<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.jfr.internal.JVM.setMethodSamplingInterval(JJ)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.setMethodSamplingPeriod(JJ)V",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn set_method_sampling_period<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.jfr.internal.JVM.setMethodSamplingPeriod(JJ)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.setMiscellaneous(JJ)V",
    GreaterThanOrEqual(JAVA_25)
)]
#[async_method]
pub async fn set_miscellaneous<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("jdk.jfr.internal.JVM.setMiscellaneous(JJ)V".to_string())
            .into(),
    )
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.setOutput(Ljava/lang/String;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn set_output<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.jfr.internal.JVM.setOutput(Ljava/lang/String;)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.setRepositoryLocation(Ljava/lang/String;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn set_repository_location<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.jfr.internal.JVM.setRepositoryLocation(Ljava/lang/String;)V".to_string(),
    )
    .into())
}

#[intrinsic_method("jdk/jfr/internal/JVM.setSampleThreads(Z)V", Between(JAVA_11, JAVA_17))]
#[async_method]
pub async fn set_sample_threads<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("jdk.jfr.internal.JVM.setSampleThreads(Z)V".to_string())
            .into(),
    )
}

#[intrinsic_method("jdk/jfr/internal/JVM.setStackDepth(I)V", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn set_stack_depth<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("jdk.jfr.internal.JVM.setStackDepth(I)V".to_string())
            .into(),
    )
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.setStackTraceEnabled(JZ)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn set_stack_trace_enabled<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.jfr.internal.JVM.setStackTraceEnabled(JZ)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.setThreadBufferSize(J)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn set_thread_buffer_size<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("jdk.jfr.internal.JVM.setThreadBufferSize(J)V".to_string())
            .into(),
    )
}

#[intrinsic_method("jdk/jfr/internal/JVM.setThreshold(JJ)Z", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn set_threshold<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("jdk.jfr.internal.JVM.setThreshold(JJ)Z".to_string())
            .into(),
    )
}

#[intrinsic_method("jdk/jfr/internal/JVM.setThrottle(JJJ)Z", GreaterThan(JAVA_11))]
#[async_method]
pub async fn set_throttle<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("jdk.jfr.internal.JVM.setThrottle(JJJ)Z".to_string())
            .into(),
    )
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.shouldRotateDisk()Z",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn should_rotate_disk<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("jdk.jfr.internal.JVM.shouldRotateDisk()Z".to_string())
            .into(),
    )
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.storeMetadataDescriptor([B)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn store_metadata_descriptor<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.jfr.internal.JVM.storeMetadataDescriptor([B)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.subscribeLogLevel(Ljdk/jfr/internal/LogTag;I)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn subscribe_log_level<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.jfr.internal.JVM.subscribeLogLevel(Ljdk/jfr/internal/LogTag;I)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.uncaughtException(Ljava/lang/Thread;Ljava/lang/Throwable;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn uncaught_exception<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.jfr.internal.JVM.uncaughtException(Ljava/lang/Thread;Ljava/lang/Throwable;)V"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.unregisterStackFilter(J)V",
    GreaterThanOrEqual(JAVA_25)
)]
#[async_method]
pub async fn unregister_stack_filter<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.jfr.internal.JVM.unregisterStackFilter(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.drainStaleMethodTracerIds()[J",
    GreaterThanOrEqual(JAVA_25)
)]
#[async_method]
pub async fn drain_stale_method_tracer_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.jfr.internal.JVM.drainStaleMethodTracerIds()[J".to_string(),
    )
    .into())
}

#[intrinsic_method("jdk/jfr/internal/JVM.isProduct()Z", GreaterThanOrEqual(JAVA_25))]
#[async_method]
pub async fn is_product<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("jdk.jfr.internal.JVM.isProduct()Z".to_string()).into())
}

#[intrinsic_method("jdk/jfr/internal/JVM.setCPUPeriod(J)V", GreaterThanOrEqual(JAVA_25))]
#[async_method]
pub async fn set_cpu_period<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _period = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("jdk.jfr.internal.JVM.setCPUPeriod(J)V".to_string()).into())
}

#[intrinsic_method("jdk/jfr/internal/JVM.setCPURate(D)V", GreaterThanOrEqual(JAVA_25))]
#[async_method]
pub async fn set_cpu_rate<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _rate = parameters.pop_double()?;
    Err(JavaError::UnsatisfiedLinkError("jdk.jfr.internal.JVM.setCPURate(D)V".to_string()).into())
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.setMethodTraceFilters([Ljava/lang/String;[Ljava/lang/String;[Ljava/lang/String;[I)[J",
    GreaterThanOrEqual(JAVA_25)
)]
#[async_method]
pub async fn set_method_trace_filters<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _flags = parameters.pop_reference()?;
    let _patterns = parameters.pop_reference()?;
    let _classes = parameters.pop_reference()?;
    let _methods = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("jdk.jfr.internal.JVM.setMethodTraceFilters([Ljava/lang/String;[Ljava/lang/String;[Ljava/lang/String;[I)[J".to_string()).into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_abort() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = abort(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_add_string_constant() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = add_string_constant(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_begin_recording() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = begin_recording(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_commit() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = commit(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_counter_time() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = counter_time(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_jfr() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = create_jfr(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_destroy_jfr() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = destroy_jfr(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_emit_data_loss() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = emit_data_loss(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_emit_event() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = emit_event(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_emit_old_object_samples_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = emit_old_object_samples_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_emit_old_object_samples_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = emit_old_object_samples_1(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_end_recording() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = end_recording(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_exclude() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = exclude(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_flush_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = flush_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_flush_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = flush_1(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_flush_2() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = flush_2(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_all_event_classes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_all_event_classes(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_allowed_to_do_event_retransforms() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_allowed_to_do_event_retransforms(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_chunk_start_nanos() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_chunk_start_nanos(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_class_id() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_class_id(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_class_id_non_intrinsic() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_class_id_non_intrinsic(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_configuration() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_configuration(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_dump_path() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_dump_path(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_event_writer_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_event_writer_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_event_writer_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_event_writer_1(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_handler() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_handler(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_pid() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_pid(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_stack_trace_id_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_stack_trace_id_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_stack_trace_id_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_stack_trace_id_1(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_thread_id() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_thread_id(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_ticks_frequency() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_ticks_frequency(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_time_conversion_factor() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_time_conversion_factor(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_type_id_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_type_id_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_type_id_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_type_id_1(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_unloaded_event_class_count() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_unloaded_event_class_count(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_host_total_memory() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = host_total_memory(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_host_total_swap_memory() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = host_total_swap_memory(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_include() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = include(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_is_available() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = is_available(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_is_containerized() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = is_containerized(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_is_excluded_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = is_excluded_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_is_excluded_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = is_excluded_1(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_is_instrumented() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = is_instrumented(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_is_recording() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = is_recording(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_log() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = log(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_log_event() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = log_event(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_mark_chunk_final() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = mark_chunk_final(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_nanos_now() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = nanos_now(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_new_event_writer_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = new_event_writer_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_new_event_writer_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = new_event_writer_1(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_register_natives() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = register_natives(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_register_stack_filter() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = register_stack_filter(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_retransform_classes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = retransform_classes(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_compressed_integers() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_compressed_integers(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_configuration() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_configuration(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_cutoff() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_cutoff(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_dump_path() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_dump_path(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_enabled() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_enabled(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_file_notification() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_file_notification(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_force_instrumentation() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_force_instrumentation(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_global_buffer_count() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_global_buffer_count(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_global_buffer_size() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_global_buffer_size(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_handler() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_handler(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_memory_size() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_memory_size(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_method_sampling_interval() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_method_sampling_interval(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_method_sampling_period() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_method_sampling_period(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_miscellaneous() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_miscellaneous(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_output() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_output(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_repository_location() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_repository_location(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_sample_threads() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_sample_threads(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_stack_depth() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_stack_depth(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_stack_trace_enabled() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_stack_trace_enabled(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_thread_buffer_size() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_thread_buffer_size(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_threshold() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_threshold(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_throttle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_throttle(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_should_rotate_disk() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = should_rotate_disk(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_store_metadata_descriptor() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = store_metadata_descriptor(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_subscribe_log_level() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = subscribe_log_level(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_uncaught_exception() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = uncaught_exception(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_unregister_stack_filter() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = unregister_stack_filter(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
