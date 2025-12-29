//! Java Platform Module System (JPMS) implementation.
//!
//! This module provides a specification compliant implementation of the Java module system as
//! defined by JSR 376 and the JVMS. It supports:
//!
//! - Module descriptors parsed from `module-info.class`
//! - Module discovery from system modules (jimage), module path, and upgrade module path
//! - Module resolution and readability graph construction
//! - Encapsulation rules (exports/opens)
//! - Services (uses/provides)
//! - Command-line overrides (`--add-exports`, `--add-opens`, `--add-reads`, etc.)

mod access;
mod descriptor;
mod error;
mod finder;
mod graph;
mod layer;
mod reference;
mod resolution;

pub use access::{AccessCheck, AccessCheckResult, JAVA_BASE_MODULE, UNNAMED_MODULE};
pub use descriptor::{
    Exports, ExportsFlags, ModuleDescriptor, ModuleFlags, Opens, OpensFlags, Provides, Requires,
    RequiresFlags,
};
pub use error::{ModuleError, Result};
pub use finder::{ModuleFinder, ModuleFinderChain, ModulePathFinder, SystemModuleFinder};
pub use graph::ModuleGraph;
pub use layer::ModuleLayer;
pub use reference::ModuleReference;
pub use resolution::{ResolvedConfiguration, ResolvedModule, Resolver};
