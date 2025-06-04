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
//! # Example
//!
//! ```rust
//! use ristretto_classfile::mutf8;
//!
//! // Convert a Rust string to Modified UTF-8 bytes
//! let string = "Hello\u{0000}World🚀";
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
//! See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.4.7>

use crate::Error::FromUtf8Error;
use crate::Result;

/// Converts a Rust string to a Java Modified UTF-8 byte array.
///
/// Java Modified UTF-8 differs from standard UTF-8 in two ways:
/// 1. The null character (U+0000) is encoded as the two-byte sequence 0xC0, 0x80 instead of the
///    single-byte 0x00
/// 2. Supplementary characters are represented as surrogate pairs
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
///
/// # Errors
/// Should not occur; reserved for future use.
pub fn to_bytes<S: AsRef<str>>(data: S) -> Result<Vec<u8>> {
    let data = data.as_ref();
    let mut encoded = Vec::with_capacity(data.len());

    for ch in data.chars() {
        match ch {
            '\u{0000}' => encoded.extend_from_slice(&[0xC0, 0x80]),
            '\u{0001}'..='\u{007F}' => encoded.push(ch as u8),
            '\u{0080}'..='\u{07FF}' => {
                encoded.push(0xC0 | u8::try_from((ch as u32) >> 6)?);
                encoded.push(0x80 | u8::try_from((ch as u32) & 0x3F)?);
            }
            '\u{0800}'..='\u{FFFF}' => {
                encoded.push(0xE0 | u8::try_from((ch as u32) >> 12)?);
                encoded.push(0x80 | u8::try_from(((ch as u32) >> 6) & 0x3F)?);
                encoded.push(0x80 | u8::try_from((ch as u32) & 0x3F)?);
            }
            _ => {
                let mut buf = [0u8; 4];
                let encoded_bytes = ch.encode_utf8(&mut buf);
                for b in encoded_bytes.bytes() {
                    encoded.push(b);
                }
            }
        }
    }

    Ok(encoded)
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
/// let result = mutf8::from_bytes(bytes)?;
/// assert_eq!(result, "\u{0000}");
///
/// // Complex example with multiple character types
/// let bytes = vec![
///     0x41,                    // 'A' (ASCII)
///     0xC0, 0x80,              // Null character
///     0xCE, 0xB2,              // 'β' (Greek)
///     0xF0, 0x9F, 0x98, 0x80   // '😀' (Emoji)
/// ];
/// let result = mutf8::from_bytes(bytes)?;
/// assert_eq!(result, "A\u{0000}β😀");
/// # Ok::<(), ristretto_classfile::Error>(())
/// ```
///
/// # Errors
/// Returns an error if the byte sequence is not valid Modified UTF-8.
#[expect(clippy::similar_names)]
pub fn from_bytes<V: AsRef<[u8]>>(bytes: V) -> Result<String> {
    let bytes = bytes.as_ref();
    let mut decoded = String::with_capacity(bytes.len());
    let mut i = 0;

    while i < bytes.len() {
        let byte1 = bytes[i];
        if byte1 & 0x80 == 0 {
            decoded.push(byte1 as char);
            i += 1;
        } else if byte1 & 0xE0 == 0xC0 {
            if i + 1 >= bytes.len() {
                return Err(FromUtf8Error("Invalid UTF-8 byte sequence".to_string()));
            }
            let byte2 = bytes[i + 1];
            if byte1 == 0xC0 && byte2 == 0x80 {
                decoded.push('\u{0000}');
            } else {
                let ch = (u32::from(byte1 & 0x1F) << 6) | u32::from(byte2 & 0x3F);
                decoded.push(char::from_u32(ch).unwrap_or_default());
            }
            i += 2;
        } else if byte1 & 0xF0 == 0xE0 {
            if i + 2 >= bytes.len() {
                return Err(FromUtf8Error("Invalid UTF-8 byte sequence".to_string()));
            }
            let byte2 = bytes[i + 1];
            let byte3 = bytes[i + 2];
            let ch = (u32::from(byte1 & 0x0F) << 12)
                | (u32::from(byte2 & 0x3F) << 6)
                | u32::from(byte3 & 0x3F);
            decoded.push(char::from_u32(ch).unwrap_or_default());
            i += 3;
        } else {
            if i + 3 >= bytes.len() {
                return Err(FromUtf8Error("Invalid UTF-8 byte sequence".to_string()));
            }
            // Handle surrogate pairs and supplementary characters
            let byte2 = bytes[i + 1];
            let byte3 = bytes[i + 2];
            let byte4 = bytes[i + 3];
            let ch = (u32::from(byte1 & 0x07) << 18)
                | (u32::from(byte2 & 0x3F) << 12)
                | (u32::from(byte3 & 0x3F) << 6)
                | u32::from(byte4 & 0x3F);
            decoded.push(char::from_u32(ch).unwrap_or_default());
            i += 4;
        }
    }

    Ok(decoded)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test all valid UTF-8 characters, the only two invalid characters are U+D800 and U+DFFF
    ///
    /// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.4.7:~:text=the%20resulting%20string).-,bytes%5B%5D,-The%20bytes%20array>
    #[test]
    fn test_all_utf8_chars() -> Result<()> {
        for i in 0..=0x0010_FFFF {
            if let Some(ch) = char::from_u32(i) {
                let s = ch.to_string();
                let rust_encoded_bytes = s.as_bytes().to_vec();
                let mutf8_encoded_bytes = to_bytes(&s)?;
                match i {
                    0 => assert_eq!(mutf8_encoded_bytes, vec![0xC0, 0x80]),
                    _ => assert_eq!(rust_encoded_bytes, mutf8_encoded_bytes),
                }

                let rust_encoded_result = String::from_utf8(rust_encoded_bytes)?;
                let mutf8_encoded_result = from_bytes(mutf8_encoded_bytes.as_slice())?;
                assert_eq!(rust_encoded_result, mutf8_encoded_result);
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
            0xF0, 0x90, 0x80, 0x80, // '\u{10000}'
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
            0xF0, 0x90, 0x80, 0x80, // '\u{10000}'
        ];
        let expected = "\u{0000}\u{007F}\u{0080}\u{07FF}\u{0800}\u{FFFF}\u{10000}";
        let result = from_bytes(bytes)?;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_from_bytes_invalid() {
        assert!(from_bytes([0x59, 0xd9]).is_err());
        assert!(from_bytes([0x56, 0xe7]).is_err());
        assert!(from_bytes([0x56, 0xa8]).is_err());
        assert!(from_bytes([0x7e, 0xff, 0xff, 0x2a]).is_err());
    }
}
