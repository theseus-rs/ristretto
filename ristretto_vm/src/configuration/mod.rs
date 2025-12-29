//! Configuration module for the Ristretto Virtual Machine.
//!
//! This module provides all configuration-related types for the VM, including:
//!
//! - [`Configuration`] - The main configuration struct
//! - [`ConfigurationBuilder`] - Builder for creating configurations
//! - JPMS module configuration types:
//!   - [`MainModule`] - Main module specification (--module or -m)
//!   - [`ModuleRead`] - Read edge between modules (--add-reads)
//!   - [`ModuleExport`] - Export directive (--add-exports)
//!   - [`ModuleOpens`] - Opens directive for reflection (--add-opens)
//!   - [`ModulePatch`] - Module patch (--patch-module)
//!
//! # Example
//!
//! ```rust
//! use ristretto_vm::{ConfigurationBuilder, MainModule, ModuleRead, ModuleExport};
//! use ristretto_classloader::ClassPath;
//! use std::path::PathBuf;
//!
//! let config = ConfigurationBuilder::new()
//!     .class_path(ClassPath::from(&["."]))
//!     .main_class("com.example.Main")
//!     .java_version("21")
//!     .module_path(vec![PathBuf::from("/mods")])
//!     .main_module(MainModule::new("my.module"))
//!     .add_read(ModuleRead::new("my.module", "java.sql"))
//!     .add_export(ModuleExport::new("java.base", "java.lang", "ALL-UNNAMED"))
//!     .build()
//!     .unwrap();
//! ```

mod builder;
#[expect(clippy::module_inception)]
mod configuration;
mod main_module;
mod module_export;
mod module_opens;
mod module_patch;
mod module_read;

pub use builder::ConfigurationBuilder;
pub use configuration::{Configuration, VerifyMode};
pub use main_module::MainModule;
pub use module_export::ModuleExport;
pub use module_opens::ModuleOpens;
pub use module_patch::ModulePatch;
pub use module_read::ModuleRead;
