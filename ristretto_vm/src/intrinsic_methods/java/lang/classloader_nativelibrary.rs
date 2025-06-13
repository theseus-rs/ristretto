use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "java/lang/ClassLoader$NativeLibrary.find(Ljava/lang/String;)J",
    LessThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn find(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.ClassLoader$NativeLibrary.find(Ljava/lang/String;)J")
}

#[intrinsic_method(
    "java/lang/ClassLoader$NativeLibrary.findEntry(Ljava/lang/String;)J",
    LessThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn find_entry(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.ClassLoader$NativeLibrary.findEntry(Ljava/lang/String;)J")
}

#[intrinsic_method(
    "java/lang/ClassLoader$NativeLibrary.load(Ljava/lang/String;Z)V",
    LessThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn load(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.ClassLoader$NativeLibrary.load(Ljava/lang/String;Z)V")
}

#[intrinsic_method(
    "java/lang/ClassLoader$NativeLibrary.load0(Ljava/lang/String;ZZ)Z",
    LessThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn load_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.ClassLoader$NativeLibrary.load0(Ljava/lang/String;ZZ)Z")
}

#[intrinsic_method(
    "java/lang/ClassLoader$NativeLibrary.unload(Ljava/lang/String;Z)V",
    LessThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn unload_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.ClassLoader$NativeLibrary.unload(Ljava/lang/String;Z)V")
}

#[intrinsic_method(
    "java/lang/ClassLoader$NativeLibrary.unload(Ljava/lang/String;ZJ)V",
    LessThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn unload_1(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.ClassLoader$NativeLibrary.unload(Ljava/lang/String;ZJ)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.ClassLoader$NativeLibrary.find(Ljava/lang/String;)J"
    )]
    async fn test_find() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = find(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.ClassLoader$NativeLibrary.findEntry(Ljava/lang/String;)J"
    )]
    async fn test_find_entry() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = find_entry(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.ClassLoader$NativeLibrary.load(Ljava/lang/String;Z)V"
    )]
    async fn test_load() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = load(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.ClassLoader$NativeLibrary.load0(Ljava/lang/String;ZZ)Z"
    )]
    async fn test_load_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = load_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.ClassLoader$NativeLibrary.unload(Ljava/lang/String;Z)V"
    )]
    async fn test_unload_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = unload_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.ClassLoader$NativeLibrary.unload(Ljava/lang/String;ZJ)V"
    )]
    async fn test_unload_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = unload_1(thread, Parameters::default()).await;
    }
}
