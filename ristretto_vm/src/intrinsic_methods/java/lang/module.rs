use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::{Equal, GreaterThan, GreaterThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "java/lang/Module.addExports0(Ljava/lang/Module;Ljava/lang/String;Ljava/lang/Module;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn add_exports_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.Module.addExports0(Ljava/lang/Module;Ljava/lang/String;Ljava/lang/Module;)V")
}

#[intrinsic_method(
    "java/lang/Module.addExportsToAll0(Ljava/lang/Module;Ljava/lang/String;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn add_exports_to_all_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.Module.addExportsToAll0(Ljava/lang/Module;Ljava/lang/String;)V")
}

#[intrinsic_method(
    "java/lang/Module.addExportsToAllUnnamed0(Ljava/lang/Module;Ljava/lang/String;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn add_exports_to_all_unnamed_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.Module.addExportsToAllUnnamed0(Ljava/lang/Module;Ljava/lang/String;)V")
}

#[intrinsic_method(
    "java/lang/Module.addReads0(Ljava/lang/Module;Ljava/lang/Module;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn add_reads_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.Module.addReads0(Ljava/lang/Module;Ljava/lang/Module;)V")
}

#[intrinsic_method(
    "java/lang/Module.defineModule0(Ljava/lang/Module;ZLjava/lang/String;Ljava/lang/String;[Ljava/lang/String;)V",
    Equal(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn define_module_0_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.Module.defineModule0(Ljava/lang/Module;ZLjava/lang/String;Ljava/lang/String;[Ljava/lang/String;)V"
    )
}

#[intrinsic_method(
    "java/lang/Module.defineModule0(Ljava/lang/Module;ZLjava/lang/String;Ljava/lang/String;[Ljava/lang/Object;)V",
    GreaterThan(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn define_module_0_1(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.Module.defineModule0(Ljava/lang/Module;ZLjava/lang/String;Ljava/lang/String;[Ljava/lang/Object;)V"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.Module.addExports0(Ljava/lang/Module;Ljava/lang/String;Ljava/lang/Module;)V"
    )]
    async fn test_add_exports_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = add_exports_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.Module.addExportsToAll0(Ljava/lang/Module;Ljava/lang/String;)V"
    )]
    async fn test_add_exports_to_all_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = add_exports_to_all_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.Module.addExportsToAllUnnamed0(Ljava/lang/Module;Ljava/lang/String;)V"
    )]
    async fn test_add_exports_to_all_unnamed_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = add_exports_to_all_unnamed_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.Module.addReads0(Ljava/lang/Module;Ljava/lang/Module;)V"
    )]
    async fn test_add_reads_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = add_reads_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.Module.defineModule0(Ljava/lang/Module;ZLjava/lang/String;Ljava/lang/String;[Ljava/lang/String;)V"
    )]
    async fn test_define_module_0_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = define_module_0_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.Module.defineModule0(Ljava/lang/Module;ZLjava/lang/String;Ljava/lang/String;[Ljava/lang/Object;)V"
    )]
    async fn test_define_module_0_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = define_module_0_1(thread, Parameters::default()).await;
    }
}
