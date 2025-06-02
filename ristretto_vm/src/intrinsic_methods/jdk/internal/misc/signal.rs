use crate::Error::InternalError;
use crate::Result;
use crate::intrinsic_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "jdk/internal/misc/Signal";

/// Register all intrinsic methods for `jdk.internal.misc.Signal`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "findSignal0",
        "(Ljava/lang/String;)I",
        find_signal_0,
    );
    registry.register(CLASS_NAME, "handle0", "(IJ)J", handle_0);
    registry.register(CLASS_NAME, "raise0", "(I)V", raise_0);
}

#[async_recursion(?Send)]
async fn find_signal_0(_thread: Arc<Thread>, mut parameters: Parameters) -> Result<Option<Value>> {
    let signal_name: String = parameters.pop_object()?.try_into()?;

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

#[async_recursion(?Send)]
async fn handle_0(_thread: Arc<Thread>, mut parameters: Parameters) -> Result<Option<Value>> {
    let _handler = parameters.pop_long()?;
    let _signal = parameters.pop_int()?;
    // TODO: implement signal handling
    Ok(Some(Value::Long(0)))
}

#[async_recursion(?Send)]
async fn raise_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Signal.raise0(I)V")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::java_object::JavaObject;

    #[tokio::test]
    async fn test_find_signal_0() -> Result<()> {
        let (vm, thread) = crate::test::thread().await?;
        let signal_name = "INT".to_object(&vm).await?;
        let parameters = Parameters::new(vec![signal_name]);
        let value = find_signal_0(thread, parameters).await?;
        assert_eq!(value, Some(Value::Int(2)));
        Ok(())
    }

    #[tokio::test]
    async fn test_handle_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let signal = Value::Int(2);
        let handler = Value::Long(0);
        let parameters = Parameters::new(vec![signal, handler]);
        let value = handle_0(thread, parameters).await?;
        assert_eq!(value, Some(Value::Long(0)));
        Ok(())
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.internal.misc.Signal.raise0(I)V")]
    async fn test_raise_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = raise_0(thread, Parameters::default()).await;
    }
}
