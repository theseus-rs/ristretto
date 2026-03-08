//! Java string types that store raw Modified UTF-8 (MUTF-8) bytes.
//!
//! Java class files use a Modified UTF-8 encoding that differs from standard UTF-8 in two ways:
//!
//! 1. The null character (U+0000) is encoded as `[0xC0, 0x80]` instead of `[0x00]`
//! 2. Supplementary characters are encoded as surrogate pairs instead of 4-byte sequences
//!
//! These types preserve the raw MUTF-8 bytes, enabling lossless round-trip serialization of Java
//! class file string constants. Conversion to Rust strings is provided via [`std::fmt::Display`] and
//! [`JavaStr::to_rust_string`].
//!
//! # Type Relationship
//!
//! `JavaStr` and `JavaString` mirror the relationship between `str` and `String`:
//! - [`JavaStr`] is an unsized, borrowed type (like `str`)
//! - [`JavaString`] is an owned type (like `String`)
//!
//! # Examples
//!
//! ```rust
//! use ristretto_classfile::{JavaStr, JavaString};
//!
//! // Create from a Rust string (converts UTF-8 -> MUTF-8)
//! let java_string = JavaString::from("Hello, World!");
//! assert_eq!(java_string, "Hello, World!");
//! assert_eq!(java_string.len(), 13);
//!
//! // Borrow as JavaStr
//! let java_str: &JavaStr = &java_string;
//! assert_eq!(java_str.as_bytes(), b"Hello, World!");
//!
//! // Convert back to Rust string
//! assert_eq!(java_str.to_rust_string(), "Hello, World!");
//! # Ok::<(), ristretto_classfile::Error>(())
//! ```

use crate::Result;
use crate::error::Error::FromUtf8Error;
use crate::mutf8;
use std::borrow::{Borrow, Cow};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::Deref;

/// A borrowed Java Modified UTF-8 string slice.
///
/// This is an unsized type wrapping `[u8]` of validated MUTF-8 bytes. It cannot be created
/// directly; use [`JavaStr::from_mutf8`] or borrow from a [`JavaString`].
///
/// # Examples
///
/// ```rust
/// use ristretto_classfile::{JavaStr, JavaString};
///
/// let java_string = JavaString::from("Hello");
/// let java_str: &JavaStr = &java_string;
/// assert_eq!(java_str.len(), 5);
/// assert_eq!(java_str.as_str(), Some("Hello"));
/// ```
#[derive(Eq)]
#[repr(transparent)]
pub struct JavaStr([u8]);

impl JavaStr {
    /// Creates a `&JavaStr` from a byte slice, validating that it contains valid MUTF-8.
    ///
    /// # Errors
    ///
    /// Returns an error if the bytes are not valid Modified UTF-8.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::JavaStr;
    ///
    /// let java_str = JavaStr::from_mutf8(b"Hello")?;
    /// assert_eq!(java_str.len(), 5);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    #[inline]
    pub fn from_mutf8(bytes: &[u8]) -> Result<&JavaStr> {
        mutf8::validate(bytes)?;
        // SAFETY: JavaStr is #[repr(transparent)] over [u8], and we just validated the bytes.
        #[expect(unsafe_code)]
        Ok(unsafe { JavaStr::from_mutf8_unchecked(bytes) })
    }

    /// Creates a `&JavaStr` from a byte slice without validation.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the bytes are valid Modified UTF-8.
    #[must_use]
    #[inline]
    #[expect(unsafe_code, clippy::ref_as_ptr)]
    pub unsafe fn from_mutf8_unchecked(bytes: &[u8]) -> &JavaStr {
        // SAFETY: caller guarantees bytes are valid MUTF-8
        unsafe { &*(bytes as *const [u8] as *const JavaStr) }
    }

    /// Creates a `&JavaStr` from a Rust `&str`.
    ///
    /// If the string is pure ASCII (no null bytes, no multi-byte), the bytes are shared directly
    /// since ASCII is identical in UTF-8 and MUTF-8.
    ///
    /// For strings containing null bytes or supplementary characters, use [`JavaString::from`]
    /// instead and borrow the result.
    ///
    /// # Errors
    ///
    /// Returns an error if the string contains characters that require different encoding in
    /// MUTF-8 (null bytes or supplementary characters). In such cases, use [`JavaString::from`].
    pub fn try_from_str(s: &str) -> Result<&JavaStr> {
        let bytes = s.as_bytes();
        // Check for characters that require different encoding in MUTF-8:
        // - Null bytes (0x00) need to be encoded as [0xC0, 0x80]
        // - Supplementary characters (4-byte UTF-8) need surrogate pair encoding
        if bytes.contains(&0x00) || bytes.iter().any(|&b| b >= 0xF0) {
            return Err(FromUtf8Error(
                "string contains characters requiring MUTF-8 conversion; use JavaString::from() instead".to_string(),
            ));
        }
        // The bytes are identical in UTF-8 and MUTF-8
        // SAFETY: JavaStr is #[repr(transparent)] over [u8]
        #[expect(unsafe_code, clippy::ref_as_ptr)]
        Ok(unsafe { &*(bytes as *const [u8] as *const JavaStr) })
    }

    /// Returns the raw MUTF-8 bytes.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Converts a `&str` to `Cow<'_, JavaStr>`.
    ///
    /// For strings whose UTF-8 bytes are identical to their MUTF-8 representation
    /// (no null bytes and no supplementary characters), this borrows with zero cost.
    /// Otherwise, allocates a `JavaString` with proper MUTF-8 encoding.
    ///
    /// This is useful for creating `Constant::Utf8` values from string literals:
    /// ```
    /// use ristretto_classfile::{Constant, JavaStr};
    ///
    /// let constant = Constant::Utf8(JavaStr::cow_from_str("(IDJ)V"));
    /// ```
    #[must_use]
    pub fn cow_from_str(s: &str) -> Cow<'_, JavaStr> {
        match Self::try_from_str(s) {
            Ok(js) => Cow::Borrowed(js),
            Err(_) => Cow::Owned(JavaString::from(s)),
        }
    }

    /// Attempts to return this `JavaStr` as a Rust `&str`.
    ///
    /// This succeeds when the MUTF-8 bytes are also valid UTF-8, which is the case for strings
    /// that don't contain null bytes (`U+0000`) or supplementary characters (above `U+FFFF`).
    /// This includes all ASCII strings and most BMP strings.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::JavaStr;
    ///
    /// // ASCII strings work
    /// let java_str = JavaStr::from_mutf8(b"Hello")?;
    /// assert_eq!(java_str.as_str(), Some("Hello"));
    ///
    /// // Strings with MUTF-8 null encoding don't
    /// let java_str = JavaStr::from_mutf8(&[0xC0, 0x80])?;
    /// assert_eq!(java_str.as_str(), None);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    #[must_use]
    pub fn as_str(&self) -> Option<&str> {
        // If there are no MUTF-8-specific sequences (null encoding, surrogates), the bytes
        // are valid UTF-8 because our strict validation rejects overlongs and bare nulls.
        if mutf8::has_mutf8_specials(&self.0) {
            None
        } else {
            // SAFETY: MUTF-8 without special sequences is valid UTF-8 because:
            // - validate() rejects bare 0x00 bytes (null must be 0xC0 0x80)
            // - validate() rejects overlong 2-byte encodings (0xC1, 0xC0 except 0xC0 0x80)
            // - validate() rejects overlong 3-byte encodings (0xE0 with byte2 < 0xA0)
            // - validate() rejects 4-byte sequences (0xF0+)
            // - has_mutf8_specials returns false (no 0xC0 0x80 null, no 0xED surrogates)
            // Therefore the remaining bytes are standard UTF-8.
            #[expect(unsafe_code)]
            Some(unsafe { std::str::from_utf8_unchecked(&self.0) })
        }
    }

    /// Converts this `JavaStr` to a Rust `String`.
    ///
    /// This performs a full MUTF-8 -> UTF-8 conversion. Lone surrogates (which are valid in Java
    /// strings but not in Unicode) are replaced with the Unicode replacement character (U+FFFD).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::JavaStr;
    ///
    /// let java_str = JavaStr::from_mutf8(b"Hello")?;
    /// assert_eq!(java_str.to_rust_string(), "Hello");
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    #[must_use]
    pub fn to_rust_string(&self) -> String {
        if let Some(s) = self.as_str() {
            return s.to_owned();
        }
        mutf8::from_bytes(&self.0).unwrap_or_else(|_| {
            // Fallback: if MUTF-8 decoding fails, use lossy UTF-8
            String::from_utf8_lossy(&self.0).into_owned()
        })
    }

    /// Returns this `JavaStr` as a borrowed or owned Rust string.
    ///
    /// For ASCII and BMP strings without null bytes (the vast majority of constant pool strings),
    /// this returns a borrowed `&str` with zero allocation. For strings containing supplementary
    /// characters or MUTF-8 null encoding, this allocates a new `String`.
    ///
    /// This is the preferred method for converting `JavaStr` to a Rust string type when you
    /// need a `&str` reference.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::JavaString;
    ///
    /// let java_string = JavaString::from("Hello");
    /// let cow = java_string.to_str_lossy();
    /// assert_eq!(&*cow, "Hello");
    /// ```
    #[must_use]
    pub fn to_str_lossy(&self) -> Cow<'_, str> {
        if let Some(s) = self.as_str() {
            Cow::Borrowed(s)
        } else {
            Cow::Owned(self.to_rust_string())
        }
    }

    /// Converts this `JavaStr` to UTF-16 code units, preserving lone surrogates.
    ///
    /// This produces the exact UTF-16 code units that a Java `char[]` would contain.
    /// Unlike [`to_rust_string`](Self::to_rust_string) which replaces lone surrogates with
    /// U+FFFD, this preserves all values losslessly.
    ///
    /// # Errors
    /// Returns an error if the MUTF-8 bytes are malformed.
    pub fn to_utf16(&self) -> crate::Result<Vec<u16>> {
        mutf8::to_utf16(&self.0)
    }

    /// Converts this `JavaStr` to an owned `JavaString`.
    #[must_use]
    pub fn to_java_string(&self) -> JavaString {
        JavaString(self.0.to_vec())
    }

    /// Returns the length of the MUTF-8 bytes.
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the string is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl fmt::Debug for JavaStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"", self.to_rust_string())
    }
}

impl fmt::Display for JavaStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_rust_string())
    }
}

impl PartialEq for JavaStr {
    fn eq(&self, other: &JavaStr) -> bool {
        self.0 == other.0
    }
}

impl PartialEq<str> for JavaStr {
    fn eq(&self, other: &str) -> bool {
        // Fast path: if MUTF-8 bytes are also valid UTF-8, compare directly
        if let Some(s) = self.as_str() {
            return s == other;
        }
        // Allocation-free path: decode MUTF-8 code points on the fly and compare
        // against the other string's chars
        let mut mutf8_iter = Mutf8CharIter::new(&self.0);
        let mut other_iter = other.chars();
        loop {
            match (mutf8_iter.next(), other_iter.next()) {
                (Some(a), Some(b)) => {
                    if a != b {
                        return false;
                    }
                }
                (None, None) => return true,
                _ => return false,
            }
        }
    }
}

/// Iterator that decodes MUTF-8 bytes into `char` values.
///
/// Surrogate pairs are combined into supplementary characters. Lone surrogates
/// are yielded as U+FFFD (replacement character), matching `to_rust_string()` behavior.
struct Mutf8CharIter<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> Mutf8CharIter<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, pos: 0 }
    }
}

impl Iterator for Mutf8CharIter<'_> {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        if self.pos >= self.bytes.len() {
            return None;
        }
        let byte1 = self.bytes[self.pos];
        match byte1 {
            0x00 => {
                self.pos += 1;
                Some('\0')
            }
            0x01..=0x7F => {
                self.pos += 1;
                Some(char::from(byte1))
            }
            0xC0..=0xDF => {
                if self.pos + 1 >= self.bytes.len() {
                    self.pos = self.bytes.len();
                    return Some('\u{FFFD}');
                }
                let byte2 = self.bytes[self.pos + 1];
                let code = u32::from(byte1 & 0x1F) << 6 | u32::from(byte2 & 0x3F);
                self.pos += 2;
                Some(char::from_u32(code).unwrap_or('\u{FFFD}'))
            }
            0xE0..=0xEF => {
                if self.pos + 2 >= self.bytes.len() {
                    self.pos = self.bytes.len();
                    return Some('\u{FFFD}');
                }
                let byte2 = self.bytes[self.pos + 1];
                let byte3 = self.bytes[self.pos + 2];
                let ch = u32::from(byte1 & 0x0F) << 12
                    | u32::from(byte2 & 0x3F) << 6
                    | u32::from(byte3 & 0x3F);

                // Check for surrogate pair
                if (0xD800..=0xDBFF).contains(&ch) && self.pos + 5 < self.bytes.len() {
                    let next1 = self.bytes[self.pos + 3];
                    // MUTF-8 low surrogates always start with 0xED (1110 1101)
                    if next1 == 0xED {
                        let next2 = self.bytes[self.pos + 4];
                        let next3 = self.bytes[self.pos + 5];
                        let low = u32::from(next1 & 0x0F) << 12
                            | u32::from(next2 & 0x3F) << 6
                            | u32::from(next3 & 0x3F);
                        if (0xDC00..=0xDFFF).contains(&low) {
                            let code = 0x1_0000 + ((ch - 0xD800) << 10) + (low - 0xDC00);
                            self.pos += 6;
                            return Some(char::from_u32(code).unwrap_or('\u{FFFD}'));
                        }
                    }
                }
                self.pos += 3;
                // Lone surrogate -> replacement character
                Some(char::from_u32(ch).unwrap_or('\u{FFFD}'))
            }
            _ => {
                self.pos += 1;
                Some('\u{FFFD}')
            }
        }
    }
}

impl PartialEq<&str> for JavaStr {
    fn eq(&self, other: &&str) -> bool {
        self == *other
    }
}

impl PartialEq<String> for JavaStr {
    fn eq(&self, other: &String) -> bool {
        self == other.as_str()
    }
}

impl PartialEq<JavaStr> for str {
    fn eq(&self, other: &JavaStr) -> bool {
        other == self
    }
}

impl PartialOrd for JavaStr {
    fn partial_cmp(&self, other: &JavaStr) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for JavaStr {
    fn cmp(&self, other: &JavaStr) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl Hash for JavaStr {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl AsRef<[u8]> for JavaStr {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsRef<JavaStr> for JavaStr {
    fn as_ref(&self) -> &JavaStr {
        self
    }
}

impl ToOwned for JavaStr {
    type Owned = JavaString;

    fn to_owned(&self) -> JavaString {
        self.to_java_string()
    }
}

/// An owned Java Modified UTF-8 string.
///
/// This is the owned counterpart to [`JavaStr`], analogous to the relationship between `String`
/// and `str`. It stores validated MUTF-8 bytes in a `Vec<u8>`.
///
/// # Examples
///
/// ```rust
/// use ristretto_classfile::JavaString;
///
/// // Create from a Rust string
/// let java_string = JavaString::from("Hello, Java!");
/// assert_eq!(java_string, "Hello, Java!");
///
/// // Create from raw MUTF-8 bytes
/// let java_string = JavaString::from_mutf8(vec![0xC0, 0x80])?;
/// assert_eq!(java_string.len(), 2);
/// # Ok::<(), ristretto_classfile::Error>(())
/// ```
#[derive(Clone, Eq)]
pub struct JavaString(Vec<u8>);

impl JavaString {
    /// Creates a new empty `JavaString`.
    #[must_use]
    pub const fn new() -> JavaString {
        JavaString(Vec::new())
    }

    /// Creates a `JavaString` from raw MUTF-8 bytes, validating the encoding.
    ///
    /// # Errors
    ///
    /// Returns an error if the bytes are not valid Modified UTF-8.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::JavaString;
    ///
    /// // MUTF-8 null encoding
    /// let java_string = JavaString::from_mutf8(vec![0xC0, 0x80])?;
    /// assert_eq!(java_string.len(), 2);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    pub fn from_mutf8(bytes: Vec<u8>) -> Result<JavaString> {
        mutf8::validate(&bytes)?;
        Ok(JavaString(bytes))
    }

    /// Creates a `JavaString` from raw MUTF-8 bytes without validation.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the bytes are valid Modified UTF-8.
    #[must_use]
    #[expect(unsafe_code)]
    pub unsafe fn from_mutf8_unchecked(bytes: Vec<u8>) -> JavaString {
        JavaString(bytes)
    }

    /// Returns a reference to the underlying bytes as a `JavaStr`.
    #[must_use]
    pub fn as_java_str(&self) -> &JavaStr {
        // SAFETY: JavaStr is #[repr(transparent)] over [u8], and self.0 is validated MUTF-8.
        #[expect(unsafe_code)]
        unsafe {
            JavaStr::from_mutf8_unchecked(&self.0)
        }
    }

    /// Consumes this `JavaString` and returns the underlying MUTF-8 bytes.
    #[must_use]
    pub fn into_bytes(self) -> Vec<u8> {
        self.0
    }
}

impl Default for JavaString {
    fn default() -> Self {
        JavaString::new()
    }
}

impl Deref for JavaString {
    type Target = JavaStr;

    fn deref(&self) -> &JavaStr {
        self.as_java_str()
    }
}

impl Borrow<JavaStr> for JavaString {
    fn borrow(&self) -> &JavaStr {
        self.as_java_str()
    }
}

impl AsRef<JavaStr> for JavaString {
    fn as_ref(&self) -> &JavaStr {
        self.as_java_str()
    }
}

impl AsRef<[u8]> for JavaString {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl From<&str> for JavaString {
    fn from(s: &str) -> JavaString {
        let bytes = s.as_bytes();
        // Fast path: ASCII-only strings are identical in UTF-8 and MUTF-8.
        if s.is_ascii() && !bytes.contains(&0) {
            return JavaString(bytes.to_vec());
        }
        // Need to convert UTF-8 -> MUTF-8
        match mutf8::to_bytes(s) {
            Ok(mutf8_bytes) => JavaString(mutf8_bytes),
            Err(_) => JavaString(bytes.to_vec()),
        }
    }
}

impl From<String> for JavaString {
    fn from(s: String) -> JavaString {
        JavaString::from(s.as_str())
    }
}

impl From<&JavaStr> for JavaString {
    fn from(s: &JavaStr) -> JavaString {
        s.to_java_string()
    }
}

impl From<Cow<'_, str>> for JavaString {
    fn from(cow: Cow<'_, str>) -> JavaString {
        JavaString::from(cow.as_ref())
    }
}

impl From<JavaString> for Cow<'static, JavaStr> {
    fn from(s: JavaString) -> Cow<'static, JavaStr> {
        Cow::Owned(s)
    }
}

impl fmt::Debug for JavaString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.as_java_str(), f)
    }
}

impl fmt::Display for JavaString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.as_java_str(), f)
    }
}

impl PartialEq for JavaString {
    fn eq(&self, other: &JavaString) -> bool {
        self.0 == other.0
    }
}

impl PartialEq<str> for JavaString {
    fn eq(&self, other: &str) -> bool {
        self.as_java_str() == other
    }
}

impl PartialEq<&str> for JavaString {
    fn eq(&self, other: &&str) -> bool {
        self.as_java_str() == *other
    }
}

impl PartialEq<String> for JavaString {
    fn eq(&self, other: &String) -> bool {
        self.as_java_str() == other.as_str()
    }
}

impl PartialEq<JavaStr> for JavaString {
    fn eq(&self, other: &JavaStr) -> bool {
        self.as_java_str() == other
    }
}

impl PartialEq<JavaString> for str {
    fn eq(&self, other: &JavaString) -> bool {
        other.as_java_str() == self
    }
}

impl PartialEq<JavaString> for &str {
    fn eq(&self, other: &JavaString) -> bool {
        other.as_java_str() == *self
    }
}

impl PartialOrd for JavaString {
    fn partial_cmp(&self, other: &JavaString) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for JavaString {
    fn cmp(&self, other: &JavaString) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl Hash for JavaString {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_java_string_from_str() {
        let js = JavaString::from("Hello");
        assert_eq!(js.len(), 5);
        assert_eq!(js, "Hello");
        assert_eq!(js.to_rust_string(), "Hello");
    }

    #[test]
    fn test_java_string_from_string() {
        let js = JavaString::from(String::from("World"));
        assert_eq!(js, "World");
    }

    #[test]
    fn test_java_str_as_str_ascii() {
        let js = JavaString::from("ascii");
        let java_str: &JavaStr = &js;
        assert_eq!(java_str.as_str(), Some("ascii"));
    }

    #[test]
    fn test_java_str_as_str_bmp() {
        let js = JavaString::from("βγδ");
        let java_str: &JavaStr = &js;
        assert_eq!(java_str.as_str(), Some("βγδ"));
    }

    #[test]
    fn test_java_string_null_encoding() {
        // Null character gets MUTF-8 encoding [0xC0, 0x80]
        let js = JavaString::from("\0");
        assert_eq!(js.as_bytes(), &[0xC0, 0x80]);
        assert_eq!(js.as_str(), None); // Not valid UTF-8
        assert_eq!(js.to_rust_string(), "\0");
    }

    #[test]
    fn test_java_string_supplementary() {
        // Emoji (supplementary character) gets surrogate pair encoding in MUTF-8
        let js = JavaString::from("😀");
        assert_ne!(js.as_bytes(), "😀".as_bytes()); // Different encoding
        assert_eq!(js.as_str(), None); // Not valid UTF-8
        assert_eq!(js.to_rust_string(), "😀"); // But converts back correctly
    }

    #[test]
    fn test_java_str_from_mutf8_valid() {
        let js = JavaStr::from_mutf8(b"Hello").expect("should be valid");
        assert_eq!(js.len(), 5);
        assert_eq!(js, "Hello");
    }

    #[test]
    fn test_java_str_from_mutf8_null() {
        let js = JavaStr::from_mutf8(&[0xC0, 0x80]).expect("should be valid MUTF-8");
        assert_eq!(js.len(), 2);
    }

    #[test]
    fn test_java_str_from_mutf8_invalid() {
        // 4-byte UTF-8 sequence is invalid MUTF-8
        assert!(JavaStr::from_mutf8(&[0xF0, 0x9F, 0x98, 0x80]).is_err());
    }

    #[test]
    fn test_java_string_from_mutf8() {
        let js = JavaString::from_mutf8(vec![0x48, 0x65, 0x6C, 0x6C, 0x6F]).expect("valid");
        assert_eq!(js, "Hello");
    }

    #[test]
    fn test_java_string_equality() {
        let js1 = JavaString::from("Hello");
        let js2 = JavaString::from("Hello");
        let js3 = JavaString::from("World");
        assert_eq!(js1, js2);
        assert_ne!(js1, js3);
    }

    #[test]
    fn test_java_string_cross_type_equality() {
        let js = JavaString::from("Hello");
        assert_eq!(js, "Hello");
        assert_eq!("Hello", js);
        assert_eq!(js, String::from("Hello"));
        assert_eq!(js, *"Hello");
    }

    #[test]
    fn test_java_str_cross_type_equality() {
        let js = JavaString::from("Hello");
        let java_str: &JavaStr = &js;
        assert_eq!(java_str, "Hello");
        assert_eq!(java_str, &"Hello");
    }

    #[test]
    fn test_java_string_debug() {
        let js = JavaString::from("Hello");
        assert_eq!(format!("{js:?}"), "\"Hello\"");
    }

    #[test]
    fn test_java_string_display() {
        let js = JavaString::from("Hello");
        assert_eq!(format!("{js}"), "Hello");
    }

    #[test]
    fn test_java_string_hash() {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        let js = JavaString::from("key");
        map.insert(js.clone(), 42);
        assert_eq!(map.get(&js), Some(&42));
    }

    #[test]
    fn test_java_string_cow() {
        let js = JavaString::from("Hello");
        let cow: Cow<'_, JavaStr> = Cow::Owned(js.clone());
        assert_eq!(&*cow, &*js);

        let borrowed: Cow<'_, JavaStr> = Cow::Borrowed(&js);
        assert_eq!(&*borrowed, &*js);
    }

    #[test]
    fn test_java_string_default() {
        let js = JavaString::default();
        assert!(js.is_empty());
        assert_eq!(js.len(), 0);
    }

    #[test]
    fn test_java_string_into_bytes() {
        let js = JavaString::from("Hello");
        let bytes = js.into_bytes();
        assert_eq!(bytes, b"Hello");
    }

    #[test]
    fn test_java_str_to_java_string() {
        let js = JavaString::from("Hello");
        let java_str: &JavaStr = &js;
        let owned = java_str.to_java_string();
        assert_eq!(owned, js);
    }

    #[test]
    fn test_java_string_ordering() {
        let a = JavaString::from("a");
        let b = JavaString::from("b");
        assert!(a < b);
        assert!(b > a);
    }

    #[test]
    fn test_java_str_ordering() {
        let a = JavaString::from("a");
        let b = JavaString::from("b");
        let a_str: &JavaStr = &a;
        let b_str: &JavaStr = &b;
        assert!(a_str < b_str);
    }

    #[test]
    fn test_java_string_clone() {
        let js = JavaString::from("Hello");
        let cloned = js.clone();
        assert_eq!(js, cloned);
    }

    #[test]
    fn test_java_str_empty() {
        let js = JavaString::from("");
        assert!(js.is_empty());
        assert_eq!(js.len(), 0);
    }

    #[test]
    fn test_java_string_from_cow_borrowed() {
        let cow: Cow<'_, str> = Cow::Borrowed("Hello");
        let js = JavaString::from(cow);
        assert_eq!(js, "Hello");
    }

    #[test]
    fn test_java_string_from_cow_owned() {
        let cow: Cow<'_, str> = Cow::Owned("Hello".to_string());
        let js = JavaString::from(cow);
        assert_eq!(js, "Hello");
    }

    #[test]
    fn test_java_str_from_str_ascii() {
        let java_str = JavaStr::try_from_str("Hello").expect("should succeed for ASCII");
        assert_eq!(java_str, "Hello");
    }

    #[test]
    fn test_java_str_from_str_bmp() {
        let java_str = JavaStr::try_from_str("βγδ").expect("should succeed for BMP");
        assert_eq!(java_str, "βγδ");
    }

    #[test]
    fn test_java_str_from_str_null() {
        // Null byte requires MUTF-8 conversion, so from_str should fail
        assert!(JavaStr::try_from_str("\0").is_err());
    }

    #[test]
    fn test_java_str_from_str_supplementary() {
        // Supplementary characters require MUTF-8 conversion, so from_str should fail
        assert!(JavaStr::try_from_str("😀").is_err());
    }

    #[test]
    fn test_java_string_as_ref_bytes() {
        let js = JavaString::from("Hello");
        let bytes: &[u8] = js.as_ref();
        assert_eq!(bytes, b"Hello");
    }

    #[test]
    fn test_java_string_as_ref_java_str() {
        let js = JavaString::from("Hello");
        let java_str: &JavaStr = js.as_ref();
        assert_eq!(java_str, "Hello");
    }

    #[test]
    fn test_java_str_partial_eq_java_string() {
        let js = JavaString::from("Hello");
        let java_str: &JavaStr = &js;
        assert_eq!(java_str, js.as_java_str());
    }
}
