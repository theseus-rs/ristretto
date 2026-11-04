use crate::Result;
use ristretto_classfile::{JAVA_8, JAVA_17, Version};
use ristretto_classloader::{Class, Object, Reference, Value};
use ristretto_gc::GarbageCollector;
use std::sync::Arc;
use zerocopy::IntoBytes;

type Utf8Constructor = fn(Arc<Class>, &Arc<GarbageCollector>, &str) -> Result<Value>;
type Utf16Constructor = fn(Arc<Class>, &Arc<GarbageCollector>, Vec<u16>) -> Result<Value>;

/// String constructors selected once for a VM's immutable runtime version.
///
/// Both VM and intrinsic conversions share these constructors. Function pointers
/// dispatch directly to the appropriate layout without testing the version for
/// each string. Content-dependent Latin-1/UTF-16 selection remains per string.
#[derive(Clone, Debug)]
pub struct StringFactory {
    utf8: Utf8Constructor,
    utf16: Utf16Constructor,
}

impl StringFactory {
    /// Selects the constructors for the configured runtime during VM creation.
    #[must_use]
    pub fn new(version: &Version) -> Self {
        if version <= &JAVA_8 {
            Self {
                utf8: legacy_utf8,
                utf16: legacy_utf16,
            }
        } else if version >= &JAVA_17 {
            Self {
                utf8: compact_utf8::<true>,
                utf16: compact_utf16::<true>,
            }
        } else {
            Self {
                utf8: compact_utf8::<false>,
                utf16: compact_utf16::<false>,
            }
        }
    }

    /// Creates a string from Rust UTF-8 using the selected layout.
    ///
    /// # Errors
    /// Returns an error if the string class cannot be instantiated or its fields cannot be set.
    pub fn create_utf8(
        &self,
        class: Arc<Class>,
        collector: &Arc<GarbageCollector>,
        string: &str,
    ) -> Result<Value> {
        (self.utf8)(class, collector, string)
    }

    /// Creates a string from UTF-16 code units, preserving lone surrogates.
    ///
    /// # Errors
    /// Returns an error if the string class cannot be instantiated or its fields cannot be set.
    pub fn create_utf16(
        &self,
        class: Arc<Class>,
        collector: &Arc<GarbageCollector>,
        units: Vec<u16>,
    ) -> Result<Value> {
        (self.utf16)(class, collector, units)
    }
}

fn legacy_utf8(
    class: Arc<Class>,
    collector: &Arc<GarbageCollector>,
    string: &str,
) -> Result<Value> {
    legacy_utf16(class, collector, string.encode_utf16().collect())
}

fn legacy_utf16(
    class: Arc<Class>,
    collector: &Arc<GarbageCollector>,
    units: Vec<u16>,
) -> Result<Value> {
    let mut object = Object::new(class)?;
    let array = Value::new_object(collector, Reference::CharArray(units.into()));
    object.set_value("value", array)?;
    object.set_value("hash", Value::Int(0))?;
    Ok(Value::from_object(collector, object))
}

fn compact_utf8<const HASH_IS_ZERO: bool>(
    class: Arc<Class>,
    collector: &Arc<GarbageCollector>,
    string: &str,
) -> Result<Value> {
    let (coder, bytes) = if string.is_ascii() {
        let bytes: &[i8] = zerocopy::transmute_ref!(string.as_bytes());
        (0, bytes.to_vec())
    } else if string.as_bytes().iter().all(|&byte| byte < 0xC4) {
        // Latin-1's non-ASCII characters have UTF-8 lead bytes C2/C3.
        (0, string.chars().map(|c| c as i8).collect())
    } else {
        // UTF-8's byte length covers non-ASCII UTF-16 text without repeatedly
        // growing the buffer. Mixed ASCII text may grow it.
        let mut bytes = Vec::with_capacity(string.len());
        for unit in string.encode_utf16() {
            bytes.extend_from_slice(&unit.to_ne_bytes().map(u8::cast_signed));
        }
        (1, bytes)
    };
    compact::<HASH_IS_ZERO>(class, collector, coder, bytes)
}

#[expect(
    clippy::needless_pass_by_value,
    reason = "shared constructor signature lets Java 8 reuse the UTF-16 allocation"
)]
fn compact_utf16<const HASH_IS_ZERO: bool>(
    class: Arc<Class>,
    collector: &Arc<GarbageCollector>,
    units: Vec<u16>,
) -> Result<Value> {
    #[expect(
        clippy::cast_possible_truncation,
        reason = "Latin-1 fits in a Java byte"
    )]
    let (coder, bytes) = if units.iter().all(|&unit| unit <= 0xFF) {
        (0, units.iter().map(|&unit| unit as i8).collect())
    } else {
        let bytes: &[i8] = zerocopy::transmute_ref!(units.as_bytes());
        (1, bytes.to_vec())
    };
    compact::<HASH_IS_ZERO>(class, collector, coder, bytes)
}

fn compact<const HASH_IS_ZERO: bool>(
    class: Arc<Class>,
    collector: &Arc<GarbageCollector>,
    coder: i32,
    bytes: Vec<i8>,
) -> Result<Value> {
    let mut object = Object::new(class)?;
    // This condition is a compile-time constant in each selected constructor.
    if HASH_IS_ZERO {
        object.set_value("hashIsZero", Value::Int(0))?;
    }
    object.set_value("coder", Value::Int(coder))?;
    let array = Value::new_object(collector, Reference::from(bytes));
    object.set_value("value", array)?;
    object.set_value("hash", Value::Int(0))?;
    Ok(Value::from_object(collector, object))
}
