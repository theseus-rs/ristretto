mod bootstrap;
mod models;
mod util;

pub use bootstrap::{
    ALL_LTS_VERSIONS, DEFAULT_JAVA_VERSION, JAVA_8_VERSION, JAVA_11_VERSION, JAVA_17_VERSION,
    JAVA_21_VERSION, JAVA_25_VERSION, home_class_loader,
};
#[cfg(not(target_family = "wasm"))]
pub use bootstrap::{default_class_loader, version_class_loader, version_class_loader_for_os};
