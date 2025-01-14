use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Error::InternalError;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `jdk.internal.misc.Signal`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "jdk/internal/misc/Signal";
    registry.register(
        class_name,
        "findSignal0",
        "(Ljava/lang/String;)I",
        find_signal_0,
    );
    registry.register(class_name, "handle0", "(IJ)J", handle_0);
    registry.register(class_name, "raise0", "(I)V", raise_0);
}

#[async_recursion(?Send)]
async fn find_signal_0(_thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let signal_name: String = arguments.pop_object()?.try_into()?;

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
async fn handle_0(_thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let _handler = arguments.pop_long()?;
    let _signal = arguments.pop_int()?;
    // TODO: implement signal handling
    Ok(Some(Value::Long(0)))
}

#[async_recursion(?Send)]
async fn raise_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Signal.raise0(I)V")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::java_object::JavaObject;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "jdk/internal/misc/Signal";
        assert!(registry
            .method(class_name, "findSignal0", "(Ljava/lang/String;)I")
            .is_some());
        assert!(registry.method(class_name, "handle0", "(IJ)J").is_some());
        assert!(registry.method(class_name, "raise0", "(I)V").is_some());
    }

    #[tokio::test]
    async fn test_find_signal_0() -> Result<()> {
        let (vm, thread) = crate::test::thread().await?;
        let signal_name = "INT".to_object(&vm).await?;
        let arguments = Arguments::new(vec![signal_name]);
        let value = find_signal_0(thread, arguments).await?;
        assert_eq!(value, Some(Value::Int(2)));
        Ok(())
    }

    #[tokio::test]
    async fn test_handle_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let signal = Value::Int(2);
        let handler = Value::Long(0);
        let arguments = Arguments::new(vec![signal, handler]);
        let value = handle_0(thread, arguments).await?;
        assert_eq!(value, Some(Value::Long(0)));
        Ok(())
    }

    #[tokio::test]
    #[should_panic(expected = "jdk.internal.misc.Signal.raise0(I)V")]
    async fn test_raise_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = raise_0(thread, Arguments::default()).await;
    }
}
