mod bootstrap;
mod http;
mod models;
mod util;

#[cfg(not(target_family = "wasm"))]
pub use bootstrap::version_class_loader_for_os;
pub use bootstrap::{
    ALL_LTS_VERSIONS, DEFAULT_JAVA_VERSION, JAVA_8_VERSION, JAVA_11_VERSION, JAVA_17_VERSION,
    JAVA_21_VERSION, JAVA_25_VERSION, default_class_loader, home_class_loader,
    version_class_loader,
};
