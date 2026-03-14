//! Functions to convert between Rust strings and Java Modified UTF-8 byte arrays.
//!
//! Java Modified UTF-8 (MUTF-8) is a variant of UTF-8 used in Java class files and for
//! serialization in Java. It differs from standard UTF-8 in two key ways:
//!
//! 1. The null character (U+0000) is encoded as the two-byte sequence `[0xC0, 0x80]` instead of the
//!    single-byte `0x00`. This avoids problems with C-style string handling where null bytes are
//!    used as terminators.
//!
//! 2. Supplementary characters (those outside the Basic Multilingual Plane) are encoded as
//!    surrogate pairs, rather than using the standard 4-byte UTF-8 encoding.
//!
//! This module provides bidirectional conversion between Rust strings (which use standard UTF-8)
//! and Java's Modified UTF-8 format.
//!
//! # Examples
//!
//! ```rust
//! use ristretto_classfile::mutf8;
//!
//! // Convert a Rust string to Modified UTF-8 bytes
//! let string = "Hello\u{0000}World😃";
//! let mutf8_bytes = mutf8::to_bytes(string)?;
//!
//! // Convert Modified UTF-8 bytes back to a Rust string
//! let rust_string = mutf8::from_bytes(&mutf8_bytes)?;
//! assert_eq!(rust_string, string);
//!
//! // The null character is specially encoded in MUTF-8
//! let null_string = "\u{0000}";
//! let null_bytes = mutf8::to_bytes(null_string)?;
//! assert_eq!(null_bytes, vec![0xC0, 0x80]);
//! # Ok::<(), ristretto_classfile::Error>(())
//! ```
//!
//! # References
//!
//! See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4.7>

use crate::Error::FromUtf8Error;
use crate::Result;

/// Converts a Rust string to a Java Modified UTF-8 byte array.
///
/// Java Modified UTF-8 differs from standard UTF-8 in two ways:
/// 1. The null character (U+0000) is encoded as the two-byte sequence 0xC0, 0x80 instead of the
///    single-byte 0x00
/// 2. Supplementary characters are represented as surrogate pairs
///
/// # Errors
///
/// Should not occur; reserved for future use.
///
/// # Examples
///
/// ```rust
/// use ristretto_classfile::mutf8;
///
/// // Convert a simple string to Modified UTF-8
/// let result = mutf8::to_bytes("Hello, Java!")?;
/// assert_eq!(result, "Hello, Java!".as_bytes());
///
/// // The null character gets special encoding
/// let result = mutf8::to_bytes("\u{0000}")?;
/// assert_eq!(result, vec![0xC0, 0x80]);
///
/// // String with various character types
/// let mixed = "A\u{0000}β\u{1F600}";  // ASCII, null, Greek, emoji
/// let result = mutf8::to_bytes(mixed)?;
/// # Ok::<(), ristretto_classfile::Error>(())
/// ```
#[expect(clippy::cast_possible_truncation)]
pub fn to_bytes(data: &str) -> Result<Vec<u8>> {
    let bytes = data.as_bytes();

    // Fast path for ASCII without nulls
    if bytes.iter().all(|&b| b > 0 && b < 128) {
        return Ok(bytes.to_vec());
    }

    // Calculate capacity: worst case is 6 bytes per char (supplementary as surrogate pair)
    let mut encoded = Vec::with_capacity(data.len() * 2);
    for ch in data.chars() {
        let code = ch as u32;
        match code {
            0 => encoded.extend_from_slice(&[0xC0, 0x80]),
            1..=0x7F => encoded.push(code as u8),
            0x80..=0x7FF => {
                encoded.push(0xC0 | ((code >> 6) as u8));
                encoded.push(0x80 | ((code & 0x3F) as u8));
            }
            0x800..=0xFFFF => {
                encoded.push(0xE0 | ((code >> 12) as u8));
                encoded.push(0x80 | (((code >> 6) & 0x3F) as u8));
                encoded.push(0x80 | ((code & 0x3F) as u8));
            }
            _ => {
                // Supplementary character: encode as surrogate pair (inline to avoid loop)
                let u = code - 0x1_0000;
                let high = 0xD800 + (u >> 10);
                let low = 0xDC00 + (u & 0x3FF);
                // High surrogate
                encoded.push(0xE0 | ((high >> 12) as u8));
                encoded.push(0x80 | (((high >> 6) & 0x3F) as u8));
                encoded.push(0x80 | ((high & 0x3F) as u8));
                // Low surrogate
                encoded.push(0xE0 | ((low >> 12) as u8));
                encoded.push(0x80 | (((low >> 6) & 0x3F) as u8));
                encoded.push(0x80 | ((low & 0x3F) as u8));
            }
        }
    }

    Ok(encoded)
}

/// Converts MUTF-8 bytes to UTF-16 code units, preserving lone surrogates.
///
/// Unlike [`from_bytes`] which converts to UTF-8 (replacing lone surrogates with U+FFFD),
/// this function produces UTF-16 code units that exactly match Java's internal `char[]`
/// representation. Surrogate pairs remain as two separate `u16` code units, and lone
/// surrogates are preserved.
///
/// # Errors
/// Returns an error if the byte sequence is not valid Modified UTF-8.
///
/// # Examples
///
/// ```rust
/// use ristretto_classfile::mutf8;
///
/// // ASCII
/// let chars = mutf8::to_utf16(b"Hi")?;
/// assert_eq!(chars, vec![0x0048, 0x0069]);
///
/// // Null character (0xC0 0x80)
/// let chars = mutf8::to_utf16(&[0xC0, 0x80])?;
/// assert_eq!(chars, vec![0x0000]);
///
/// // Surrogate pair for U+1F603 (😃): high=0xD83D, low=0xDE03
/// let chars = mutf8::to_utf16(&[0xED, 0xA0, 0xBD, 0xED, 0xB8, 0x83])?;
/// assert_eq!(chars, vec![0xD83D, 0xDE03]);
/// # Ok::<(), ristretto_classfile::Error>(())
/// ```
pub fn to_utf16(input: &[u8]) -> Result<Vec<u16>> {
    let mut result = Vec::with_capacity(input.len());
    let mut iter = input.iter();

    while let Some(&byte1) = iter.next() {
        match byte1 {
            // 1-byte sequence (ASCII)
            0x01..=0x7F => {
                result.push(u16::from(byte1));
            }
            // Bare null byte
            0x00 => {
                result.push(0);
            }
            // 2-byte sequence
            0xC0..=0xDF => {
                let Some(&byte2) = iter.next() else {
                    return Err(FromUtf8Error("Invalid MUTF-8 byte sequence".to_string()));
                };
                let ch = u16::from(byte1 & 0x1F) << 6 | u16::from(byte2 & 0x3F);
                result.push(ch);
            }
            // 3-byte sequence — surrogates are preserved as raw u16 values
            0xE0..=0xEF => {
                let Some(&byte2) = iter.next() else {
                    return Err(FromUtf8Error("Invalid MUTF-8 byte sequence".to_string()));
                };
                let Some(&byte3) = iter.next() else {
                    return Err(FromUtf8Error("Invalid MUTF-8 byte sequence".to_string()));
                };
                let ch = u16::from(byte1 & 0x0F) << 12
                    | u16::from(byte2 & 0x3F) << 6
                    | u16::from(byte3 & 0x3F);
                result.push(ch);
            }
            _ => {
                return Err(FromUtf8Error(
                    "MUTF-8 does not use 4-byte sequences".to_string(),
                ));
            }
        }
    }

    Ok(result)
}

/// Converts a Java Modified UTF-8 byte array to a Rust string.
///
/// This function properly handles the Modified UTF-8 encoding used by Java, including the special
/// encoding of null characters and the representation of supplementary characters.
///
/// # Examples
///
/// ```rust
/// use ristretto_classfile::mutf8;
///
/// // Convert from Modified UTF-8 bytes to a Rust string
/// let bytes = "Hello, Java!".as_bytes();
/// let result = mutf8::from_bytes(bytes)?;
/// assert_eq!(result, "Hello, Java!");
///
/// // Handle null character (encoded as 0xC0, 0x80)
/// let bytes = vec![0xC0, 0x80];
/// let result = mutf8::from_bytes(&bytes)?;
/// assert_eq!(result, "\u{0000}");
///
/// // Complex example with multiple character types
/// let bytes = vec![
///     0x41,                                // 'A' (ASCII)
///     0xC0, 0x80,                          // Null character
///     0xCE, 0xB2,                          // 'β' (Greek)
///     0xED, 0xA0, 0xBD, 0xED, 0xB8, 0x83   // '😃' (Emoji)
/// ];
/// let result = mutf8::from_bytes(&bytes)?;
/// assert_eq!(result, "A\u{0000}β😃");
/// # Ok::<(), ristretto_classfile::Error>(())
/// ```
///
/// # Errors
/// Returns an error if the byte sequence is not valid Modified UTF-8.
pub fn from_bytes(input: &[u8]) -> Result<String> {
    // if let Ok(s) = std::str::from_utf8(input) {
    //     return Ok(s.to_owned());
    // }

    // Fast path: if all bytes are ASCII (and non-zero in MUTF-8 means regular ASCII),
    // we can skip all decoding. ASCII bytes are always valid UTF-8.
    if input.is_ascii() {
        // SAFETY: We just verified all bytes are ASCII (0x00..=0x7F), which is valid UTF-8.
        #[expect(unsafe_code)]
        let s = unsafe { std::str::from_utf8_unchecked(input) };
        return Ok(s.to_owned());
    }

    // Check if input contains no MUTF-8 special sequences (no 0xC0 0x80 null encoding,
    // no surrogate pairs). In that case, the bytes are already valid UTF-8.
    if !has_mutf8_special_sequences(input) {
        return std::str::from_utf8(input)
            .map(std::borrow::ToOwned::to_owned)
            .map_err(|e| FromUtf8Error(e.to_string()));
    }

    // Slow path: decode MUTF-8 to UTF-8
    decode_mutf8(input)
}

/// Zero-copy version of `from_bytes` that returns a `Cow<'a, str>`.
///
/// For ASCII input (the vast majority of constant pool strings), this returns a borrowed
/// `&str` pointing directly into the input slice — no allocation at all.
/// For non-ASCII MUTF-8, it allocates a new `String`.
///
/// # Errors
/// Returns an error if the input contains invalid MUTF-8 sequences.
pub fn from_bytes_cow(input: &[u8]) -> Result<std::borrow::Cow<'_, str>> {
    use std::borrow::Cow;

    // Fast path: if all bytes are ASCII, return a zero-copy borrowed &str.
    if input.is_ascii() {
        // SAFETY: We just verified all bytes are ASCII (0x00..=0x7F), which is valid UTF-8.
        #[expect(unsafe_code)]
        let s = unsafe { std::str::from_utf8_unchecked(input) };
        return Ok(Cow::Borrowed(s));
    }

    // Check if input is valid standard UTF-8 without MUTF-8 special sequences.
    if !has_mutf8_special_sequences(input) {
        return std::str::from_utf8(input)
            .map(Cow::Borrowed)
            .map_err(|e| FromUtf8Error(e.to_string()));
    }

    // Slow path: decode MUTF-8 to UTF-8 (must allocate)
    decode_mutf8(input).map(Cow::Owned)
}

/// Validates that the input bytes are valid Modified UTF-8 without converting.
///
/// This is more efficient than [`from_bytes`] when you only need to check validity, as it
/// avoids allocating a `String`. This is used by [`JavaStr::from_mutf8`](crate::JavaStr::from_mutf8).
///
/// # Errors
///
/// Returns an error if the byte sequence is not valid Modified UTF-8.
///
/// # Examples
///
/// ```rust
/// use ristretto_classfile::mutf8;
///
/// // Valid ASCII
/// assert!(mutf8::validate(b"Hello").is_ok());
///
/// // Valid MUTF-8 null encoding
/// assert!(mutf8::validate(&[0xC0, 0x80]).is_ok());
///
/// // Invalid: 4-byte UTF-8 sequence
/// assert!(mutf8::validate(&[0xF0, 0x9F, 0x98, 0x80]).is_err());
/// # Ok::<(), ristretto_classfile::Error>(())
/// ```
#[inline]
pub fn validate(input: &[u8]) -> Result<()> {
    // Fast path: check all bytes are non-null ASCII (0x01..=0x7F) in a single pass.
    // Covers the vast majority of constant pool strings in class files.
    if input.iter().all(|&b| b.wrapping_sub(1) < 0x7F) {
        return Ok(());
    }
    validate_slow(input)
}

/// Full MUTF-8 validation for non-ASCII or null-containing inputs.
fn validate_slow(input: &[u8]) -> Result<()> {
    let mut iter = input.iter();

    while let Some(&byte1) = iter.next() {
        match byte1 {
            // Per JVM spec §4.4.7: "No byte may have the value (byte)0"
            0x00 => {
                return Err(FromUtf8Error(
                    "Invalid MUTF-8: bare null byte (U+0000 must be encoded as 0xC0 0x80)"
                        .to_string(),
                ));
            }
            // 1-byte sequence (ASCII, excluding null)
            0x01..=0x7F => {}
            // 2-byte sequence
            0xC0..=0xDF => {
                let Some(&byte2) = iter.next() else {
                    return Err(FromUtf8Error(
                        "Invalid MUTF-8: truncated 2-byte sequence".to_string(),
                    ));
                };
                if byte2 & 0xC0 != 0x80 {
                    return Err(FromUtf8Error(
                        "Invalid MUTF-8: invalid continuation byte".to_string(),
                    ));
                }
                // Reject overlong encodings: 0xC0 with byte2 != 0x80 encodes U+0001..U+003F
                // (should be 1-byte), and 0xC1 encodes U+0040..U+007F (should be 1-byte).
                // Only 0xC0 0x80 (U+0000 null) is valid as a 2-byte encoding of an ASCII-range
                // code point per JVM spec §4.4.7.
                if byte1 == 0xC0 && byte2 != 0x80 {
                    return Err(FromUtf8Error(
                        "Invalid MUTF-8: overlong 2-byte encoding".to_string(),
                    ));
                }
                if byte1 == 0xC1 {
                    return Err(FromUtf8Error(
                        "Invalid MUTF-8: overlong 2-byte encoding".to_string(),
                    ));
                }
            }
            // 3-byte sequence
            0xE0..=0xEF => {
                let Some(&byte2) = iter.next() else {
                    return Err(FromUtf8Error(
                        "Invalid MUTF-8: truncated 3-byte sequence".to_string(),
                    ));
                };
                let Some(&byte3) = iter.next() else {
                    return Err(FromUtf8Error(
                        "Invalid MUTF-8: truncated 3-byte sequence".to_string(),
                    ));
                };
                if byte2 & 0xC0 != 0x80 || byte3 & 0xC0 != 0x80 {
                    return Err(FromUtf8Error(
                        "Invalid MUTF-8: invalid continuation byte".to_string(),
                    ));
                }
                // Reject overlong 3-byte encodings: 0xE0 with byte2 < 0xA0 encodes
                // U+0000..U+07FF which should use 1 or 2-byte encoding per JVM spec §4.4.7.
                if byte1 == 0xE0 && byte2 < 0xA0 {
                    return Err(FromUtf8Error(
                        "Invalid MUTF-8: overlong 3-byte encoding".to_string(),
                    ));
                }
            }
            // 4-byte sequences and continuation bytes as lead are invalid in MUTF-8
            _ => {
                return Err(FromUtf8Error(
                    "Invalid MUTF-8: invalid lead byte".to_string(),
                ));
            }
        }
    }

    Ok(())
}

/// Check if the input contains MUTF-8 special sequences that differ from standard UTF-8.
///
/// Returns `true` if the input contains null encoding (`0xC0 0x80`), surrogate pairs
/// (`0xED 0xA0..0xBF`), or other sequences that differ between MUTF-8 and standard UTF-8.
#[inline]
#[must_use]
pub fn has_mutf8_specials(input: &[u8]) -> bool {
    has_mutf8_special_sequences(input)
}

/// Check if the input contains MUTF-8 special sequences that differ from standard UTF-8.
/// Returns true if the input contains null encoding (0xC0 0x80) or surrogate pairs (0xED 0xA0..0xBF).
#[inline]
fn has_mutf8_special_sequences(input: &[u8]) -> bool {
    let len = input.len();
    let mut i = 0;
    while i < len {
        let b = input[i];
        if b < 0x80 {
            // ASCII byte
            i += 1;
        } else if b < 0xC0 {
            // Continuation byte as lead; invalid but let the decoder handle it
            return true;
        } else if b < 0xE0 {
            // 2-byte sequence: check for null encoding (0xC0 0x80) or overlong (0xC1)
            let next = if i + 1 < len { input[i + 1] } else { 0 };
            if (b == 0xC0 && next == 0x80) || b == 0xC1 {
                return true;
            }
            i += 2;
        } else if b < 0xF0 {
            // 3-byte sequence: check for surrogates (0xED followed by 0xA0..0xBF)
            let next = if i + 1 < len { input[i + 1] } else { 0 };
            if b == 0xED && next >= 0xA0 {
                return true;
            }
            i += 3;
        } else {
            // 4-byte sequences not allowed in MUTF-8
            return true;
        }
    }
    false
}

/// Decode MUTF-8 bytes to a Rust String (slow path for inputs with special sequences).
#[expect(clippy::cast_possible_truncation)]
fn decode_mutf8(input: &[u8]) -> Result<String> {
    let mut result = Vec::with_capacity(input.len());
    let mut i = 0;
    let len = input.len();

    while i < len {
        let byte1 = input[i];
        match byte1 {
            // 1-byte sequence (ASCII)
            0x00..=0x7F => {
                result.push(byte1);
                i += 1;
            }
            // 2-byte sequence
            0xC0..=0xDF => {
                if i + 1 >= len {
                    return Err(FromUtf8Error("Invalid MUTF-8 byte sequence".to_string()));
                }
                let byte2 = input[i + 1];
                if byte1 == 0xC0 && byte2 == 0x80 {
                    // Null character; encode as standard UTF-8 null
                    result.push(0);
                } else {
                    // Standard 2-byte sequence; copy directly (valid UTF-8)
                    result.push(byte1);
                    result.push(byte2);
                }
                i += 2;
            }
            // 3-byte sequence
            0xE0..=0xEF => {
                if i + 2 >= len {
                    return Err(FromUtf8Error("Invalid MUTF-8 byte sequence".to_string()));
                }
                let byte2 = input[i + 1];
                let byte3 = input[i + 2];
                let ch = u32::from(byte1 & 0x0F) << 12
                    | u32::from(byte2 & 0x3F) << 6
                    | u32::from(byte3 & 0x3F);

                // Check if this is a surrogate (needs special handling)
                if (0xD800..=0xDFFF).contains(&ch) {
                    // High surrogate; look for low surrogate
                    if (0xD800..=0xDBFF).contains(&ch) && i + 5 < len {
                        let next1 = input[i + 3];
                        if next1 & 0xF0 == 0xE0 {
                            let next2 = input[i + 4];
                            let next3 = input[i + 5];
                            let low = u32::from(next1 & 0x0F) << 12
                                | u32::from(next2 & 0x3F) << 6
                                | u32::from(next3 & 0x3F);
                            if (0xDC00..=0xDFFF).contains(&low) {
                                // Valid surrogate pair; decode to code point
                                let code = 0x1_0000 + ((ch - 0xD800) << 10) + (low - 0xDC00);
                                // Encode as 4-byte UTF-8
                                result.push(0xF0 | ((code >> 18) as u8));
                                result.push(0x80 | (((code >> 12) & 0x3F) as u8));
                                result.push(0x80 | (((code >> 6) & 0x3F) as u8));
                                result.push(0x80 | ((code & 0x3F) as u8));
                                i += 6;
                                continue;
                            }
                        }
                    }
                    // Lone surrogate; use replacement character
                    result.extend_from_slice(&[0xEF, 0xBF, 0xBD]);
                } else {
                    // Regular 3-byte sequence; copy directly
                    result.push(byte1);
                    result.push(byte2);
                    result.push(byte3);
                }
                i += 3;
            }
            // 4-byte sequences not allowed in MUTF-8
            _ => {
                return Err(FromUtf8Error(
                    "MUTF-8 does not use 4-byte sequences".to_string(),
                ));
            }
        }
    }

    String::from_utf8(result).map_err(|e| FromUtf8Error(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test all valid UTF-8 characters, the only two invalid characters are U+D800 and U+DFFF
    ///
    /// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4.7:~:text=the%20resulting%20string).-,bytes%5B%5D,-The%20bytes%20array>
    #[test]
    fn test_all_utf8_chars() -> Result<()> {
        for i in 0..=0x10_FFFF {
            if (0xD800..=0xDFFF).contains(&i) {
                // Surrogates: not valid scalar values
                assert!(char::from_u32(i).is_none());
                continue;
            }
            if let Some(ch) = char::from_u32(i) {
                let s = ch.to_string();
                let rust_encoded_bytes = s.as_bytes().to_vec();
                let mutf8_encoded_bytes = to_bytes(&s)?;

                match i {
                    0 => {
                        // Special null encoding in MUTF-8
                        assert_eq!(mutf8_encoded_bytes, vec![0xC0, 0x80]);
                    }
                    0x10000..=0x10_FFFF => {
                        // Supplementary characters (encoded as surrogate pairs in MUTF-8). Don't
                        // assert that encoding matches Rust UTF-8. Instead, check round-trip
                        // correctness below.
                    }
                    _ => {
                        // BMP (non-null, non-surrogate)
                        assert_eq!(rust_encoded_bytes, mutf8_encoded_bytes);
                    }
                }

                let rust_encoded_result = String::from_utf8(rust_encoded_bytes)?;
                let mutf8_encoded_result = from_bytes(mutf8_encoded_bytes.as_slice())?;
                assert_eq!(rust_encoded_result, mutf8_encoded_result);
            } else {
                assert!((0xD800..=0xDFFF).contains(&i));
            }
        }
        Ok(())
    }

    /// Test the encoding of CESU-8 character from `X11GB18030_0$Encoder.class` Java 8 rt.jar
    /// that fails with CESU-8 implementations.
    #[test]
    fn test_utf8_encoding() {
        let bytes = &[237, 162, 162];
        assert!(from_bytes(bytes).is_ok());
    }

    #[test]
    fn test_to_bytes() -> Result<()> {
        let data = "\u{0000}\u{007F}\u{0080}\u{07FF}\u{0800}\u{FFFF}\u{10000}";
        let expected = vec![
            0xC0, 0x80, // '\u{0000}'
            0x7F, // '\u{007F}'
            0xC2, 0x80, // '\u{0080}'
            0xDF, 0xBF, // '\u{07FF}'
            0xE0, 0xA0, 0x80, // '\u{0800}'
            0xEF, 0xBF, 0xBF, // '\u{FFFF}'
            0xED, 0xA0, 0x80, // High surrogate (for '\u{10000}')
            0xED, 0xB0, 0x80, // Low surrogate (for '\u{10000}')
        ];
        assert_eq!(to_bytes(data)?, expected);
        Ok(())
    }

    #[test]
    fn test_from_bytes() -> Result<()> {
        let bytes = &[
            0xC0, 0x80, // '\u{0000}'
            0x7F, // '\u{007F}'
            0xC2, 0x80, // '\u{0080}'
            0xDF, 0xBF, // '\u{07FF}'
            0xE0, 0xA0, 0x80, // '\u{0800}'
            0xEF, 0xBF, 0xBF, // '\u{FFFF}'
            0xED, 0xA0, 0x80, // High surrogate (for '\u{10000}')
            0xED, 0xB0, 0x80, // Low surrogate (for '\u{10000}')
        ];
        let expected = "\u{0000}\u{007F}\u{0080}\u{07FF}\u{0800}\u{FFFF}\u{10000}";
        let result = from_bytes(bytes)?;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_from_bytes_invalid() {
        assert!(from_bytes(&[0x59, 0xd9]).is_err());
        assert!(from_bytes(&[0x56, 0xe7]).is_err());
        assert!(from_bytes(&[0x56, 0xa8]).is_err());
        assert!(from_bytes(&[0x7e, 0xff, 0xff, 0x2a]).is_err());
    }

    #[test]
    fn test_encode_decode_supplementary_character() -> Result<()> {
        // 😀 U+1F600
        let s = String::from("\u{1F600}");
        let mutf8_encoded_bytes = to_bytes(&s)?;
        let decoded = from_bytes(&mutf8_encoded_bytes)?;
        assert_eq!(decoded, s);
        Ok(())
    }
}
