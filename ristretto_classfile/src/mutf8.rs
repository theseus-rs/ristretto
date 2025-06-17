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
pub fn to_bytes<S: AsRef<str>>(data: S) -> Result<Vec<u8>> {
    let data = data.as_ref();
    let mut encoded = Vec::with_capacity(data.len());

    for ch in data.chars() {
        let code = ch as u32;
        if code == 0 {
            encoded.extend_from_slice(&[0xC0, 0x80]);
        } else if code <= 0x7F {
            encoded.push(code as u8);
        } else if code <= 0x7FF {
            encoded.push(0xC0 | ((code >> 6) as u8));
            encoded.push(0x80 | ((code & 0x3F) as u8));
        } else if code <= 0xFFFF {
            encoded.push(0xE0 | ((code >> 12) as u8));
            encoded.push(0x80 | (((code >> 6) & 0x3F) as u8));
            encoded.push(0x80 | ((code & 0x3F) as u8));
        } else {
            // Supplementary character: encode as surrogate pair
            let u = code - 0x1_0000;
            let high = 0xD800 + ((u >> 10) as u16);
            let low = 0xDC00 + ((u & 0x3FF) as u16);
            // Encode each surrogate as 3-byte UTF-8
            for unit in [high, low] {
                encoded.push(0xE0 | ((unit >> 12) as u8));
                encoded.push(0x80 | (((unit >> 6) & 0x3F) as u8));
                encoded.push(0x80 | ((unit & 0x3F) as u8));
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
///     0x41,                                // 'A' (ASCII)
///     0xC0, 0x80,                          // Null character
///     0xCE, 0xB2,                          // 'β' (Greek)
///     0xED, 0xA0, 0xBD, 0xED, 0xB8, 0x83   // '😃' (Emoji)
/// ];
/// let result = mutf8::from_bytes(bytes)?;
/// assert_eq!(result, "A\u{0000}β😃");
/// # Ok::<(), ristretto_classfile::Error>(())
/// ```
///
/// # Errors
/// Returns an error if the byte sequence is not valid Modified UTF-8.
#[expect(clippy::similar_names)]
pub fn from_bytes<V: AsRef<[u8]>>(bytes: V) -> Result<String> {
    let bytes = bytes.as_ref();
    let mut utf16: Vec<u16> = Vec::with_capacity(bytes.len());
    let mut i = 0;

    while i < bytes.len() {
        let byte1 = bytes[i];
        if byte1 & 0x80 == 0 {
            utf16.push(u16::from(byte1));
            i += 1;
        } else if byte1 & 0xE0 == 0xC0 {
            if i + 1 >= bytes.len() {
                return Err(FromUtf8Error("Invalid MUTF-8 byte sequence".to_string()));
            }
            let byte2 = bytes[i + 1];
            if byte1 == 0xC0 && byte2 == 0x80 {
                utf16.push(0);
            } else {
                let ch = ((u16::from(byte1 & 0x1F)) << 6) | u16::from(byte2 & 0x3F);
                utf16.push(ch);
            }
            i += 2;
        } else if byte1 & 0xF0 == 0xE0 {
            if i + 2 >= bytes.len() {
                return Err(FromUtf8Error("Invalid MUTF-8 byte sequence".to_string()));
            }
            let byte2 = bytes[i + 1];
            let byte3 = bytes[i + 2];
            let ch = ((u16::from(byte1 & 0x0F)) << 12)
                | ((u16::from(byte2 & 0x3F)) << 6)
                | (u16::from(byte3 & 0x3F));
            utf16.push(ch);
            i += 3;
        } else {
            return Err(FromUtf8Error(
                "MUTF-8 does not use 4-byte sequences".to_string(),
            ));
        }
    }

    // Use `from_utf16_lossy` to accept surrogate code units (as Java would)
    Ok(String::from_utf16_lossy(&utf16))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test all valid UTF-8 characters, the only two invalid characters are U+D800 and U+DFFF
    ///
    /// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.4.7:~:text=the%20resulting%20string).-,bytes%5B%5D,-The%20bytes%20array>
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
        assert!(from_bytes([0x59, 0xd9]).is_err());
        assert!(from_bytes([0x56, 0xe7]).is_err());
        assert!(from_bytes([0x56, 0xa8]).is_err());
        assert!(from_bytes([0x7e, 0xff, 0xff, 0x2a]).is_err());
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
