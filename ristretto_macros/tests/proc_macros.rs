use ristretto_classfile::VersionSpecification::Any;
use ristretto_macros::{async_method, intrinsic_method};

mod intrinsic_methods {
    use std::{future::Future, pin::Pin};
    #[cfg(not(target_family = "wasm"))]
    type BoxFuture = Pin<Box<dyn Future<Output = u8> + Send>>;
    #[cfg(target_family = "wasm")]
    type BoxFuture = Pin<Box<dyn Future<Output = u8>>>;

    #[derive(Clone, Copy, Debug)]
    pub(crate) enum IntrinsicMethod {
        Sync(fn() -> u8),
        Async(fn() -> BoxFuture),
    }
}

mod ristretto_intrinsics {
    pub(crate) mod mixed {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/mixed_intrinsics/mixed.rs"
        ));
    }
    #[cfg(target_family = "wasm")]
    pub(crate) mod wasm_only {
        pub(crate) fn wasm_only<T>() -> u8 {
            let _ = std::marker::PhantomData::<T>;
            7
        }
    }
}

mod thread {
    pub(crate) struct Thread;
}

mod generated_empty_registry {
    ristretto_macros::generate_intrinsic_registry!(
        "ristretto_macros/tests/fixtures/empty_intrinsics"
    );
}

mod generated_filtered_registry {
    #[cfg(target_family = "wasm")]
    use crate::ristretto_intrinsics;

    ristretto_macros::generate_intrinsic_registry!(
        "ristretto_macros/tests/fixtures/filtered_intrinsics"
    );
}

#[intrinsic_method("pkg/Example.intrinsic()V", Any)]
pub fn intrinsic_fixture() {}

#[async_method]
pub async fn async_fixture(value: u8) -> u8 {
    value
}

#[test]
fn intrinsic_method_macro_preserves_function() {
    intrinsic_fixture();
}

#[test]
fn async_method_macro_returns_future() {
    drop(async_fixture(7));
}

#[test]
fn generate_intrinsic_registry_macro_generates_empty_maps() {
    assert!(generated_empty_registry::JAVA_8.is_empty());
    assert!(generated_empty_registry::JAVA_11.is_empty());
    assert!(generated_empty_registry::JAVA_17.is_empty());
    assert!(generated_empty_registry::JAVA_21.is_empty());
    assert!(generated_empty_registry::JAVA_25.is_empty());
    assert!(generated_empty_registry::JAVA_8_MACOS_SIGNATURES.is_empty());
    assert!(generated_empty_registry::JAVA_25_WINDOWS_SIGNATURES.is_empty());
}

#[test]
fn generate_intrinsic_registry_macro_handles_filtered_intrinsics() {
    #[cfg(not(target_family = "wasm"))]
    assert!(generated_filtered_registry::JAVA_8.is_empty());
    #[cfg(target_family = "wasm")]
    assert_eq!(generated_filtered_registry::JAVA_8.len(), 1);

    #[cfg(not(target_family = "wasm"))]
    assert!(generated_filtered_registry::JAVA_25.is_empty());
    #[cfg(target_family = "wasm")]
    assert_eq!(generated_filtered_registry::JAVA_25.len(), 1);

    assert!(generated_filtered_registry::JAVA_8_MACOS_SIGNATURES.is_empty());
    assert!(generated_filtered_registry::JAVA_25_WINDOWS_SIGNATURES.is_empty());
}

mod generated_mixed_registry {
    use crate::ristretto_intrinsics;
    ristretto_macros::generate_intrinsic_registry!(
        "ristretto_macros/tests/fixtures/mixed_intrinsics"
    );
}

#[intrinsic_method("pkg/Example.recursive(I)I", Any)]
#[async_method]
pub async fn recursive_fixture(depth: u8) -> u8 {
    if depth == 0 {
        0
    } else {
        recursive_fixture(depth - 1).await + 1
    }
}

#[test]
fn mixed_registry_dispatches_sync_and_suspending_async_functions() {
    use intrinsic_methods::IntrinsicMethod;
    use std::task::{Context, Poll, Waker};
    #[cfg(not(target_family = "wasm"))]
    fn assert_send(_: &impl Send) {}
    let mut context = Context::from_waker(Waker::noop());
    let sync = generated_mixed_registry::JAVA_8.get("pkg/Mixed.sync()I");
    assert!(matches!(sync, Some(IntrinsicMethod::Sync(function)) if function() == 7));
    assert!(!generated_mixed_registry::JAVA_8.contains_key("pkg/Mixed.async()I"));
    let asynchronous = generated_mixed_registry::JAVA_11.get("pkg/Mixed.async()I");
    assert!(matches!(asynchronous, Some(IntrinsicMethod::Async(_))));
    if let Some(IntrinsicMethod::Async(function)) = asynchronous {
        let mut future = function();
        #[cfg(not(target_family = "wasm"))]
        assert_send(&future);
        assert_eq!(future.as_mut().poll(&mut context), Poll::Pending);
        assert_eq!(future.as_mut().poll(&mut context), Poll::Ready(11));
    }
    assert_eq!(
        recursive_fixture(4).as_mut().poll(&mut context),
        Poll::Ready(4)
    );
}
