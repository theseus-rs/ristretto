use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::VersionSpecification::{
    Between, Equal, GreaterThan, GreaterThanOrEqual, LessThanOrEqual,
};
use ristretto_classfile::{JAVA_11, JAVA_17, JAVA_21, JAVA_25};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "jdk/jfr/internal/JVM.abort(Ljava/lang/String;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub(crate) async fn abort(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.abort(Ljava/lang/String;)V")
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.addStringConstant(JLjava/lang/String;)Z",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub(crate) async fn add_string_constant(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.addStringConstant(JLjava/lang/String;)Z")
}

#[intrinsic_method("jdk/jfr/internal/JVM.beginRecording()V", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub(crate) async fn begin_recording(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.beginRecording()V")
}

#[intrinsic_method("jdk/jfr/internal/JVM.commit(J)J", GreaterThanOrEqual(JAVA_21))]
#[async_method]
pub(crate) async fn commit(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.commit(J)J")
}

#[intrinsic_method("jdk/jfr/internal/JVM.counterTime()J", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub(crate) async fn counter_time(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.counterTime()J")
}

#[intrinsic_method("jdk/jfr/internal/JVM.createJFR(Z)Z", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub(crate) async fn create_jfr(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.createJFR(Z)Z")
}

#[intrinsic_method("jdk/jfr/internal/JVM.destroyJFR()Z", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub(crate) async fn destroy_jfr(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.destroyJFR()Z")
}

#[intrinsic_method("jdk/jfr/internal/JVM.emitDataLoss(J)V", GreaterThanOrEqual(JAVA_17))]
#[async_method]
pub(crate) async fn emit_data_loss(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.emitDataLoss(J)V")
}

#[intrinsic_method("jdk/jfr/internal/JVM.emitEvent(JJJ)Z", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub(crate) async fn emit_event(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.emitEvent(JJJ)Z")
}

#[intrinsic_method("jdk/jfr/internal/JVM.emitOldObjectSamples(JZ)V", Equal(JAVA_11))]
#[async_method]
pub(crate) async fn emit_old_object_samples_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.emitOldObjectSamples(JZ)V")
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.emitOldObjectSamples(JZZ)V",
    GreaterThan(JAVA_11)
)]
#[async_method]
pub(crate) async fn emit_old_object_samples_1(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.emitOldObjectSamples(JZZ)V")
}

#[intrinsic_method("jdk/jfr/internal/JVM.endRecording()V", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub(crate) async fn end_recording(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.endRecording()V")
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.exclude(Ljava/lang/Thread;)V",
    GreaterThan(JAVA_11)
)]
#[async_method]
pub(crate) async fn exclude(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.exclude(Ljava/lang/Thread;)V")
}

#[intrinsic_method("jdk/jfr/internal/JVM.flush()V", GreaterThan(JAVA_11))]
#[async_method]
pub(crate) async fn flush_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.flush()V")
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.flush(Ljdk/jfr/internal/EventWriter;II)Z",
    Between(JAVA_11, JAVA_17)
)]
#[async_method]
pub(crate) async fn flush_1(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.flush(Ljdk/jfr/internal/EventWriter;II)Z")
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.flush(Ljdk/jfr/internal/event/EventWriter;II)V",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub(crate) async fn flush_2(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.flush(Ljdk/jfr/internal/event/EventWriter;II)V")
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.getAllEventClasses()Ljava/util/List;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub(crate) async fn get_all_event_classes(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.getAllEventClasses()Ljava/util/List;")
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.getAllowedToDoEventRetransforms()Z",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub(crate) async fn get_allowed_to_do_event_retransforms(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.getAllowedToDoEventRetransforms()Z")
}

#[intrinsic_method("jdk/jfr/internal/JVM.getChunkStartNanos()J", GreaterThan(JAVA_11))]
#[async_method]
pub(crate) async fn get_chunk_start_nanos(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.getChunkStartNanos()J")
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.getClassId(Ljava/lang/Class;)J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub(crate) async fn get_class_id(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.getClassId(Ljava/lang/Class;)J")
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.getClassIdNonIntrinsic(Ljava/lang/Class;)J",
    Equal(JAVA_11)
)]
#[async_method]
pub(crate) async fn get_class_id_non_intrinsic(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.getClassIdNonIntrinsic(Ljava/lang/Class;)J")
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.getConfiguration(Ljava/lang/Class;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub(crate) async fn get_configuration(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.getConfiguration(Ljava/lang/Class;)Ljava/lang/Object;")
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.getDumpPath()Ljava/lang/String;",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub(crate) async fn get_dump_path(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.getDumpPath()Ljava/lang/String;")
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.getEventWriter()Ljava/lang/Object;",
    Between(JAVA_11, JAVA_17)
)]
#[async_method]
pub(crate) async fn get_event_writer_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.getEventWriter()Ljava/lang/Object;")
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.getEventWriter()Ljdk/jfr/internal/event/EventWriter;",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub(crate) async fn get_event_writer_1(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.getEventWriter()Ljdk/jfr/internal/event/EventWriter;")
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.getHandler(Ljava/lang/Class;)Ljava/lang/Object;",
    Equal(JAVA_17)
)]
#[async_method]
pub(crate) async fn get_handler(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.getHandler(Ljava/lang/Class;)Ljava/lang/Object;")
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.getPid()Ljava/lang/String;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub(crate) async fn get_pid(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.getPid()Ljava/lang/String;")
}

#[intrinsic_method("jdk/jfr/internal/JVM.getStackTraceId(I)J", LessThanOrEqual(JAVA_21))]
#[async_method]
pub(crate) async fn get_stack_trace_id_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.getStackTraceId(I)J")
}

#[intrinsic_method("jdk/jfr/internal/JVM.getStackTraceId(IJ)J", GreaterThan(JAVA_21))]
#[async_method]
pub(crate) async fn get_stack_trace_id_1(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.getStackTraceId(IJ)J")
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.getThreadId(Ljava/lang/Thread;)J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub(crate) async fn get_thread_id(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.getThreadId(Ljava/lang/Thread;)J")
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.getTicksFrequency()J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub(crate) async fn get_ticks_frequency(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.getTicksFrequency()J")
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.getTimeConversionFactor()D",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub(crate) async fn get_time_conversion_factor(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.getTimeConversionFactor()D")
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.getTypeId(Ljava/lang/String;)J",
    GreaterThan(JAVA_11)
)]
#[async_method]
pub(crate) async fn get_type_id_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.getTypeId(Ljava/lang/String;)J")
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.getTypeId(Ljava/lang/Class;)J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub(crate) async fn get_type_id_1(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.getTypeId(Ljava/lang/Class;)J")
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.getUnloadedEventClassCount()J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub(crate) async fn get_unloaded_event_class_count(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.getUnloadedEventClassCount()J")
}

#[intrinsic_method("jdk/jfr/internal/JVM.hostTotalMemory()J", GreaterThanOrEqual(JAVA_21))]
#[async_method]
pub(crate) async fn host_total_memory(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.hostTotalMemory()J")
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.hostTotalSwapMemory()J",
    GreaterThanOrEqual(JAVA_25)
)]
#[async_method]
pub(crate) async fn host_total_swap_memory(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.hostTotalSwapMemory()J")
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.include(Ljava/lang/Thread;)V",
    GreaterThan(JAVA_11)
)]
#[async_method]
pub(crate) async fn include(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.include(Ljava/lang/Thread;)V")
}

#[intrinsic_method("jdk/jfr/internal/JVM.isAvailable()Z", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub(crate) async fn is_available(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.isAvailable()Z")
}

#[intrinsic_method("jdk/jfr/internal/JVM.isContainerized()Z", GreaterThanOrEqual(JAVA_21))]
#[async_method]
pub(crate) async fn is_containerized(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.isContainerized()Z")
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.isExcluded(Ljava/lang/Thread;)Z",
    GreaterThan(JAVA_11)
)]
#[async_method]
pub(crate) async fn is_excluded_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.isExcluded(Ljava/lang/Thread;)Z")
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.isExcluded(Ljava/lang/Class;)Z",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub(crate) async fn is_excluded_1(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.isExcluded(Ljava/lang/Class;)Z")
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.isInstrumented(Ljava/lang/Class;)Z",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub(crate) async fn is_instrumented(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.isInstrumented(Ljava/lang/Class;)Z")
}

#[intrinsic_method("jdk/jfr/internal/JVM.isRecording()Z", GreaterThan(JAVA_11))]
#[async_method]
pub(crate) async fn is_recording(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.isRecording()Z")
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.log(IILjava/lang/String;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub(crate) async fn log(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.log(IILjava/lang/String;)V")
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.logEvent(I[Ljava/lang/String;Z)V",
    GreaterThan(JAVA_11)
)]
#[async_method]
pub(crate) async fn log_event(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.logEvent(I[Ljava/lang/String;Z)V")
}

#[intrinsic_method("jdk/jfr/internal/JVM.markChunkFinal()V", GreaterThan(JAVA_11))]
#[async_method]
pub(crate) async fn mark_chunk_final(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.markChunkFinal()V")
}

#[intrinsic_method("jdk/jfr/internal/JVM.nanosNow()J", GreaterThanOrEqual(JAVA_25))]
#[async_method]
pub(crate) async fn nanos_now(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.nanosNow()J")
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.newEventWriter()Ljdk/jfr/internal/EventWriter;",
    Between(JAVA_11, JAVA_17)
)]
#[async_method]
pub(crate) async fn new_event_writer_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.newEventWriter()Ljdk/jfr/internal/EventWriter;")
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.newEventWriter()Ljdk/jfr/internal/event/EventWriter;",
    GreaterThan(JAVA_11)
)]
#[async_method]
pub(crate) async fn new_event_writer_1(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.newEventWriter()Ljdk/jfr/internal/event/EventWriter;")
}

#[intrinsic_method("jdk/jfr/internal/JVM.registerNatives()V", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub(crate) async fn register_natives(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.registerStackFilter([Ljava/lang/String;[Ljava/lang/String;)J",
    GreaterThanOrEqual(JAVA_25)
)]
#[async_method]
pub(crate) async fn register_stack_filter(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.registerStackFilter([Ljava/lang/String;[Ljava/lang/String;)J")
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.retransformClasses([Ljava/lang/Class;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub(crate) async fn retransform_classes(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.retransformClasses([Ljava/lang/Class;)V")
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.setCompressedIntegers(Z)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub(crate) async fn set_compressed_integers(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.setCompressedIntegers(Z)V")
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.setConfiguration(Ljava/lang/Class;Ljdk/jfr/internal/event/EventConfiguration;)Z",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub(crate) async fn set_configuration(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "jdk.jfr.internal.JVM.setConfiguration(Ljava/lang/Class;Ljdk/jfr/internal/event/EventConfiguration;)Z"
    )
}

#[intrinsic_method("jdk/jfr/internal/JVM.setCutoff(JJ)Z", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub(crate) async fn set_cutoff(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.setCutoff(JJ)Z")
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.setDumpPath(Ljava/lang/String;)V",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub(crate) async fn set_dump_path(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.setDumpPath(Ljava/lang/String;)V")
}

#[intrinsic_method("jdk/jfr/internal/JVM.setEnabled(JZ)V", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub(crate) async fn set_enabled(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.setEnabled(JZ)V")
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.setFileNotification(J)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub(crate) async fn set_file_notification(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.setFileNotification(J)V")
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.setForceInstrumentation(Z)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub(crate) async fn set_force_instrumentation(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.setForceInstrumentation(Z)V")
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.setGlobalBufferCount(J)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub(crate) async fn set_global_buffer_count(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.setGlobalBufferCount(J)V")
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.setGlobalBufferSize(J)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub(crate) async fn set_global_buffer_size(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.setGlobalBufferSize(J)V")
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.setHandler(Ljava/lang/Class;Ljdk/jfr/internal/handlers/EventHandler;)Z",
    Equal(JAVA_17)
)]
#[async_method]
pub(crate) async fn set_handler(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "jdk.jfr.internal.JVM.setHandler(Ljava/lang/Class;Ljdk/jfr/internal/handlers/EventHandler;)Z"
    )
}

#[intrinsic_method("jdk/jfr/internal/JVM.setMemorySize(J)V", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub(crate) async fn set_memory_size(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.setMemorySize(J)V")
}

#[intrinsic_method("jdk/jfr/internal/JVM.setMethodSamplingInterval(JJ)V", Equal(JAVA_11))]
#[async_method]
pub(crate) async fn set_method_sampling_interval(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.setMethodSamplingInterval(JJ)V")
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.setMethodSamplingPeriod(JJ)V",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub(crate) async fn set_method_sampling_period(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.setMethodSamplingPeriod(JJ)V")
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.setMiscellaneous(JJ)V",
    GreaterThanOrEqual(JAVA_25)
)]
#[async_method]
pub(crate) async fn set_miscellaneous(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.setMiscellaneous(JJ)V")
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.setOutput(Ljava/lang/String;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub(crate) async fn set_output(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.setOutput(Ljava/lang/String;)V")
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.setRepositoryLocation(Ljava/lang/String;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub(crate) async fn set_repository_location(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.setRepositoryLocation(Ljava/lang/String;)V")
}

#[intrinsic_method("jdk/jfr/internal/JVM.setSampleThreads(Z)V", Between(JAVA_11, JAVA_17))]
#[async_method]
pub(crate) async fn set_sample_threads(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.setSampleThreads(Z)V")
}

#[intrinsic_method("jdk/jfr/internal/JVM.setStackDepth(I)V", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub(crate) async fn set_stack_depth(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.setStackDepth(I)V")
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.setStackTraceEnabled(JZ)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub(crate) async fn set_stack_trace_enabled(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.setStackTraceEnabled(JZ)V")
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.setThreadBufferSize(J)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub(crate) async fn set_thread_buffer_size(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.setThreadBufferSize(J)V")
}

#[intrinsic_method("jdk/jfr/internal/JVM.setThreshold(JJ)Z", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub(crate) async fn set_threshold(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.setThreshold(JJ)Z")
}

#[intrinsic_method("jdk/jfr/internal/JVM.setThrottle(JJJ)Z", GreaterThan(JAVA_11))]
#[async_method]
pub(crate) async fn set_throttle(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.setThrottle(JJJ)Z")
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.shouldRotateDisk()Z",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub(crate) async fn should_rotate_disk(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.shouldRotateDisk()Z")
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.storeMetadataDescriptor([B)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub(crate) async fn store_metadata_descriptor(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.storeMetadataDescriptor([B)V")
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.subscribeLogLevel(Ljdk/jfr/internal/LogTag;I)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub(crate) async fn subscribe_log_level(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.subscribeLogLevel(Ljdk/jfr/internal/LogTag;I)V")
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.uncaughtException(Ljava/lang/Thread;Ljava/lang/Throwable;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub(crate) async fn uncaught_exception(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.uncaughtException(Ljava/lang/Thread;Ljava/lang/Throwable;)V")
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.unregisterStackFilter(J)V",
    GreaterThanOrEqual(JAVA_25)
)]
#[async_method]
pub(crate) async fn unregister_stack_filter(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.unregisterStackFilter(J)V")
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.drainStaleMethodTracerIds()[J",
    GreaterThanOrEqual(JAVA_25)
)]
#[async_method]
pub(crate) async fn drain_stale_method_tracer_ids(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.drainStaleMethodTracerIds()[J")
}

#[intrinsic_method("jdk/jfr/internal/JVM.isProduct()Z", GreaterThanOrEqual(JAVA_25))]
#[async_method]
pub(crate) async fn is_product(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.jfr.internal.JVM.isProduct()Z")
}

#[intrinsic_method("jdk/jfr/internal/JVM.setCPUPeriod(J)V", GreaterThanOrEqual(JAVA_25))]
#[async_method]
pub(crate) async fn set_cpu_period(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _period = parameters.pop_long()?;
    todo!("jdk.jfr.internal.JVM.setCPUPeriod(J)V")
}

#[intrinsic_method("jdk/jfr/internal/JVM.setCPURate(D)V", GreaterThanOrEqual(JAVA_25))]
#[async_method]
pub(crate) async fn set_cpu_rate(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _rate = parameters.pop_double()?;
    todo!("jdk.jfr.internal.JVM.setCPURate(D)V")
}

#[intrinsic_method(
    "jdk/jfr/internal/JVM.setMethodTraceFilters([Ljava/lang/String;[Ljava/lang/String;[Ljava/lang/String;[I)[J",
    GreaterThanOrEqual(JAVA_25)
)]
#[async_method]
pub(crate) async fn set_method_trace_filters(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _flags = parameters.pop_reference()?;
    let _patterns = parameters.pop_reference()?;
    let _classes = parameters.pop_reference()?;
    let _methods = parameters.pop_reference()?;
    todo!(
        "jdk.jfr.internal.JVM.setMethodTraceFilters([Ljava/lang/String;[Ljava/lang/String;[Ljava/lang/String;[I)[J"
    )
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
        let _ = abort(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.addStringConstant(JLjava/lang/String;)Z"
    )]
    async fn test_add_string_constant() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = add_string_constant(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.jfr.internal.JVM.beginRecording()V")]
    async fn test_begin_recording() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = begin_recording(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.jfr.internal.JVM.commit(J)J")]
    async fn test_commit() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = commit(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.jfr.internal.JVM.counterTime()J")]
    async fn test_counter_time() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = counter_time(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.jfr.internal.JVM.createJFR(Z)Z")]
    async fn test_create_jfr() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = create_jfr(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.jfr.internal.JVM.destroyJFR()Z")]
    async fn test_destroy_jfr() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = destroy_jfr(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.jfr.internal.JVM.emitDataLoss(J)V")]
    async fn test_emit_data_loss() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = emit_data_loss(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.jfr.internal.JVM.emitEvent(JJJ)Z")]
    async fn test_emit_event() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = emit_event(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.emitOldObjectSamples(JZ)V"
    )]
    async fn test_emit_old_object_samples_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = emit_old_object_samples_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.emitOldObjectSamples(JZZ)V"
    )]
    async fn test_emit_old_object_samples_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = emit_old_object_samples_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.jfr.internal.JVM.endRecording()V")]
    async fn test_end_recording() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = end_recording(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.exclude(Ljava/lang/Thread;)V"
    )]
    async fn test_exclude() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = exclude(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.jfr.internal.JVM.flush()V")]
    async fn test_flush_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = flush_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.flush(Ljdk/jfr/internal/EventWriter;II)Z"
    )]
    async fn test_flush_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = flush_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.flush(Ljdk/jfr/internal/event/EventWriter;II)V"
    )]
    async fn test_flush_2() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = flush_2(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.getAllEventClasses()Ljava/util/List;"
    )]
    async fn test_get_all_event_classes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_all_event_classes(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.getAllowedToDoEventRetransforms()Z"
    )]
    async fn test_get_allowed_to_do_event_retransforms() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_allowed_to_do_event_retransforms(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.jfr.internal.JVM.getChunkStartNanos()J")]
    async fn test_get_chunk_start_nanos() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_chunk_start_nanos(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.getClassId(Ljava/lang/Class;)J"
    )]
    async fn test_get_class_id() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_class_id(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.getClassIdNonIntrinsic(Ljava/lang/Class;)J"
    )]
    async fn test_get_class_id_non_intrinsic() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_class_id_non_intrinsic(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.getConfiguration(Ljava/lang/Class;)Ljava/lang/Object;"
    )]
    async fn test_get_configuration() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_configuration(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.getDumpPath()Ljava/lang/String;"
    )]
    async fn test_get_dump_path() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_dump_path(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.getEventWriter()Ljava/lang/Object;"
    )]
    async fn test_get_event_writer_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_event_writer_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.getEventWriter()Ljdk/jfr/internal/event/EventWriter;"
    )]
    async fn test_get_event_writer_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_event_writer_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.getHandler(Ljava/lang/Class;)Ljava/lang"
    )]
    async fn test_get_handler() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_handler(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.getPid()Ljava/lang/String;"
    )]
    async fn test_get_pid() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_pid(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.jfr.internal.JVM.getStackTraceId(I)J")]
    async fn test_get_stack_trace_id_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_stack_trace_id_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.jfr.internal.JVM.getStackTraceId(IJ)J")]
    async fn test_get_stack_trace_id_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_stack_trace_id_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.getThreadId(Ljava/lang/Thread;)J"
    )]
    async fn test_get_thread_id() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_thread_id(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.jfr.internal.JVM.getTicksFrequency()J")]
    async fn test_get_ticks_frequency() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_ticks_frequency(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.getTimeConversionFactor()D"
    )]
    async fn test_get_time_conversion_factor() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_time_conversion_factor(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.getTypeId(Ljava/lang/String;)J"
    )]
    async fn test_get_type_id_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_type_id_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.getTypeId(Ljava/lang/Class;)J"
    )]
    async fn test_get_type_id_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_type_id_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.getUnloadedEventClassCount()J"
    )]
    async fn test_get_unloaded_event_class_count() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_unloaded_event_class_count(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.jfr.internal.JVM.hostTotalMemory()J")]
    async fn test_host_total_memory() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = host_total_memory(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.jfr.internal.JVM.hostTotalSwapMemory()J")]
    async fn test_host_total_swap_memory() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = host_total_swap_memory(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.include(Ljava/lang/Thread;)V"
    )]
    async fn test_include() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = include(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.jfr.internal.JVM.isAvailable()Z")]
    async fn test_is_available() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_available(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.jfr.internal.JVM.isContainerized()Z")]
    async fn test_is_containerized() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_containerized(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.isExcluded(Ljava/lang/Thread;)Z"
    )]
    async fn test_is_excluded_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_excluded_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.isExcluded(Ljava/lang/Class;)Z"
    )]
    async fn test_is_excluded_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_excluded_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.isInstrumented(Ljava/lang/Class;)Z"
    )]
    async fn test_is_instrumented() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_instrumented(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.jfr.internal.JVM.isRecording()Z")]
    async fn test_is_recording() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_recording(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.log(IILjava/lang/String;)V"
    )]
    async fn test_log() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = log(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.logEvent(I[Ljava/lang/String;Z)V"
    )]
    async fn test_log_event() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = log_event(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.jfr.internal.JVM.markChunkFinal()V")]
    async fn test_mark_chunk_final() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = mark_chunk_final(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.jfr.internal.JVM.nanosNow()J")]
    async fn test_nanos_now() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = nanos_now(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.newEventWriter()Ljdk/jfr/internal/EventWriter;"
    )]
    async fn test_new_event_writer_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = new_event_writer_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.newEventWriter()Ljdk/jfr/internal/event/EventWriter;"
    )]
    async fn test_new_event_writer_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = new_event_writer_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_register_natives() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = register_natives(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.registerStackFilter([Ljava/lang/String;[Ljava/lang/String;)J"
    )]
    async fn test_register_stack_filter() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = register_stack_filter(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.retransformClasses([Ljava/lang/Class;)V"
    )]
    async fn test_retransform_classes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = retransform_classes(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.setCompressedIntegers(Z)V"
    )]
    async fn test_set_compressed_integers() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_compressed_integers(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.setConfiguration(Ljava/lang/Class;Ljdk/jfr/internal/event/EventConfiguration;)Z"
    )]
    async fn test_set_configuration() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_configuration(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.jfr.internal.JVM.setCutoff(JJ)Z")]
    async fn test_set_cutoff() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_cutoff(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.setDumpPath(Ljava/lang/String;)V"
    )]
    async fn test_set_dump_path() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_dump_path(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.jfr.internal.JVM.setEnabled(JZ)V")]
    async fn test_set_enabled() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_enabled(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.jfr.internal.JVM.setFileNotification(J)V")]
    async fn test_set_file_notification() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_file_notification(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.setForceInstrumentation(Z)V"
    )]
    async fn test_set_force_instrumentation() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_force_instrumentation(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.jfr.internal.JVM.setGlobalBufferCount(J)V")]
    async fn test_set_global_buffer_count() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_global_buffer_count(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.jfr.internal.JVM.setGlobalBufferSize(J)V")]
    async fn test_set_global_buffer_size() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_global_buffer_size(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.setHandler(Ljava/lang/Class;Ljdk/jfr/internal/handlers/EventHandler;)Z"
    )]
    async fn test_set_handler() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_handler(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.jfr.internal.JVM.setMemorySize(J)V")]
    async fn test_set_memory_size() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_memory_size(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.setMethodSamplingInterval(JJ)V"
    )]
    async fn test_set_method_sampling_interval() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_method_sampling_interval(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.setMethodSamplingPeriod(JJ)V"
    )]
    async fn test_set_method_sampling_period() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_method_sampling_period(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.jfr.internal.JVM.setMiscellaneous(JJ)V")]
    async fn test_set_miscellaneous() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_miscellaneous(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.setOutput(Ljava/lang/String;)V"
    )]
    async fn test_set_output() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_output(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.setRepositoryLocation(Ljava/lang/String;)V"
    )]
    async fn test_set_repository_location() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_repository_location(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.jfr.internal.JVM.setSampleThreads(Z)V")]
    async fn test_set_sample_threads() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_sample_threads(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.jfr.internal.JVM.setStackDepth(I)V")]
    async fn test_set_stack_depth() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_stack_depth(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.setStackTraceEnabled(JZ)V"
    )]
    async fn test_set_stack_trace_enabled() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_stack_trace_enabled(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.jfr.internal.JVM.setThreadBufferSize(J)V")]
    async fn test_set_thread_buffer_size() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_thread_buffer_size(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.jfr.internal.JVM.setThreshold(JJ)Z")]
    async fn test_set_threshold() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_threshold(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.jfr.internal.JVM.setThrottle(JJJ)Z")]
    async fn test_set_throttle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_throttle(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.jfr.internal.JVM.shouldRotateDisk()Z")]
    async fn test_should_rotate_disk() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = should_rotate_disk(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.storeMetadataDescriptor([B)V"
    )]
    async fn test_store_metadata_descriptor() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = store_metadata_descriptor(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.subscribeLogLevel(Ljdk/jfr/internal/LogTag;I)V"
    )]
    async fn test_subscribe_log_level() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = subscribe_log_level(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.uncaughtException(Ljava/lang/Thread;Ljava/lang/Throwable;)V"
    )]
    async fn test_uncaught_exception() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = uncaught_exception(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.jfr.internal.JVM.unregisterStackFilter(J)V"
    )]
    async fn test_unregister_stack_filter() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = unregister_stack_filter(thread, Parameters::default()).await;
    }
}
