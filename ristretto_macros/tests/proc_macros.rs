use ristretto_classfile::VersionSpecification::Any;
use ristretto_macros::{async_method, intrinsic_method};

mod intrinsic_methods {
    pub(crate) type IntrinsicMethod = fn();
}

#[cfg(target_family = "wasm")]
mod ristretto_intrinsics {
    pub(crate) mod wasm_only {
        pub(crate) fn wasm_only<T>() {}
    }
}

#[cfg(target_family = "wasm")]
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
