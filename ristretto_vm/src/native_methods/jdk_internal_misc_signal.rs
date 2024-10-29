use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Error::{InternalError, InvalidOperand};
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for jdk.internal.misc.Signal.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "jdk/internal/misc/Signal";
    registry.register(
        class_name,
        "findSignal0",
        "(Ljava/lang/String;)I",
        find_signal_0,
    );
    registry.register(class_name, "handle0", "(IJ)J", handle_0);
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn handle_0(_thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let _handler = arguments.pop_long()?;
    let _signal = arguments.pop_int()?;
    // TODO: implement signal handling
    Ok(Some(Value::Long(0)))
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn find_signal_0(_thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let value = arguments.pop()?;
    let signal_name = match value {
        Value::Object(_) => value.as_string()?,
        value => {
            return Err(InvalidOperand {
                expected: "object".to_string(),
                actual: value.to_string(),
            });
        }
    };

    // See: https://github.com/torvalds/linux/blob/master/arch/x86/include/uapi/asm/signal.h
    let signal = match signal_name.as_str() {
        "HUP" => 1,
        "INT" => 2,
        "QUIT" => 3,
        "ILL" => 4,
        "TRAP" => 5,
        "ABRT" | "IOT" => 6,
        "BUS" => 7,
        "FPE" => 8,
        "KILL" => 9,
        "USR1" => 10,
        "SEGV" => 11,
        "USR2" => 12,
        "PIPE" => 13,
        "ALRM" => 14,
        "TERM" => 15,
        "STKFLT" => 16,
        "CHLD" => 17,
        "CONT" => 18,
        "STOP" => 19,
        "TSTP" => 20,
        "TTIN" => 21,
        "TTOU" => 22,
        "URG" => 23,
        "XCPU" => 24,
        "XFSZ" => 25,
        "VTALRM" => 26,
        "PROF" => 27,
        "WINCH" => 28,
        "IO" | "POLL" | "LOST" => 29,
        "PWR" => 30,
        "SYS" | "UNUSED" => 31,
        _ => {
            return Err(InternalError(format!("Unknown signal: {signal_name}")));
        }
    };
    Ok(Some(Value::Int(signal)))
}
