#![forbid(unsafe_code)]
#![allow(dead_code)]
#![deny(clippy::pedantic)]
#![deny(clippy::unwrap_in_result)]
#![deny(clippy::unwrap_used)]

pub mod attributes;
mod base_type;
mod class_access_flags;
mod class_file;
mod constant;
mod constant_pool;
mod error;
mod field;
mod field_access_flags;
mod field_type;
mod method;
mod method_access_flags;
pub(crate) mod mutf8;
mod reference_kind;
mod verifiers;
mod version;

pub use base_type::BaseType;
pub use class_access_flags::ClassAccessFlags;
pub use class_file::ClassFile;
pub use constant::Constant;
pub use constant_pool::ConstantPool;
pub use error::{Error, Result};
pub use field::Field;
pub use field_access_flags::FieldAccessFlags;
pub use field_type::FieldType;
pub use method::Method;
pub use method_access_flags::MethodAccessFlags;
pub use reference_kind::ReferenceKind;
pub use version::Version;
