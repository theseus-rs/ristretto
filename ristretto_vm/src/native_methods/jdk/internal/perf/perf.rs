use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_18: Version = Version::Java18 { minor: 0 };

/// Register all native methods for `jdk.internal.perf.Perf`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "jdk/internal/perf/Perf";
    let java_version = registry.java_version();

    if java_version <= &JAVA_18 {
        registry.register(
            class_name,
            "attach",
            "(Ljava/lang/String;II)Ljava/nio/ByteBuffer;",
            attach,
        );
    } else {
        registry.register(class_name, "attach0", "(I)Ljava/nio/ByteBuffer;", attach_0);
    }

    registry.register(
        class_name,
        "createByteArray",
        "(Ljava/lang/String;II[BI)Ljava/nio/ByteBuffer;",
        create_byte_array,
    );
    registry.register(
        class_name,
        "createLong",
        "(Ljava/lang/String;IIJ)Ljava/nio/ByteBuffer;",
        create_long,
    );
    registry.register(class_name, "detach", "(Ljava/nio/ByteBuffer;)V", detach);
    registry.register(class_name, "highResCounter", "()J", high_res_counter);
    registry.register(class_name, "highResFrequency", "()J", high_res_frequency);
    registry.register(class_name, "registerNatives", "()V", register_natives);
}

#[async_recursion(?Send)]
async fn attach(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.internal.perf.Perf.attach(Ljava/lang/String;II)Ljava/nio/ByteBuffer;")
}

#[async_recursion(?Send)]
async fn attach_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.internal.perf.Perf.attach0(I)Ljava/nio/ByteBuffer;")
}

#[async_recursion(?Send)]
async fn create_byte_array(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.internal.perf.Perf.createByteArray(Ljava/lang/String;II[BI)Ljava/nio/ByteBuffer;")
}

#[async_recursion(?Send)]
async fn create_long(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.internal.perf.Perf.createLong(Ljava/lang/String;IIJ)Ljava/nio/ByteBuffer;")
}

#[async_recursion(?Send)]
async fn detach(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.internal.perf.Perf.detach(Ljava/nio/ByteBuffer;)V")
}

#[async_recursion(?Send)]
async fn high_res_counter(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.internal.perf.Perf.highResCounter()J")
}

#[async_recursion(?Send)]
async fn high_res_frequency(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.internal.perf.Perf.highResFrequency()J")
}

#[async_recursion(?Send)]
async fn register_natives(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::new(&Version::Java19 { minor: 0 }, true);
        register(&mut registry);
        let class_name = "jdk/internal/perf/Perf";
        assert!(registry
            .method(class_name, "attach0", "(I)Ljava/nio/ByteBuffer;")
            .is_some());
        assert!(registry
            .method(
                class_name,
                "createByteArray",
                "(Ljava/lang/String;II[BI)Ljava/nio/ByteBuffer;"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "createLong",
                "(Ljava/lang/String;IIJ)Ljava/nio/ByteBuffer;"
            )
            .is_some());
        assert!(registry
            .method(class_name, "detach", "(Ljava/nio/ByteBuffer;)V")
            .is_some());
        assert!(registry
            .method(class_name, "highResCounter", "()J")
            .is_some());
        assert!(registry
            .method(class_name, "highResFrequency", "()J")
            .is_some());
        assert!(registry
            .method(class_name, "registerNatives", "()V")
            .is_some());
    }

    #[test]
    fn test_register_java_18() {
        let mut registry = MethodRegistry::new(&Version::Java18 { minor: 0 }, true);
        register(&mut registry);
        let class_name = "jdk/internal/perf/Perf";
        assert!(registry
            .method(
                class_name,
                "attach",
                "(Ljava/lang/String;II)Ljava/nio/ByteBuffer;"
            )
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "jdk.internal.perf.Perf.attach(Ljava/lang/String;II)Ljava/nio/ByteBuffer;"
    )]
    async fn test_attach() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = attach(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "jdk.internal.perf.Perf.attach0(I)Ljava/nio/ByteBuffer;")]
    async fn test_attach_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = attach_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "jdk.internal.perf.Perf.createByteArray(Ljava/lang/String;II[BI)Ljava/nio/ByteBuffer;"
    )]
    async fn test_create_byte_array() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = create_byte_array(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "jdk.internal.perf.Perf.createLong(Ljava/lang/String;IIJ)Ljava/nio/ByteBuffer;"
    )]
    async fn test_create_long() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = create_long(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "jdk.internal.perf.Perf.detach(Ljava/nio/ByteBuffer;)V")]
    async fn test_detach() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = detach(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "jdk.internal.perf.Perf.highResCounter()J")]
    async fn test_high_res_counter() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = high_res_counter(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "jdk.internal.perf.Perf.highResFrequency()J")]
    async fn test_high_res_frequency() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = high_res_frequency(thread, Arguments::default()).await;
    }

    #[tokio::test]
    async fn test_register_natives() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = register_natives(thread, Arguments::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }
}
