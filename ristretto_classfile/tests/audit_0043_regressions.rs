//! Both modified UTF-8 decoding entry points must reject standard UTF-8 shortcuts.
#![expect(
    clippy::panic_in_result_fn,
    reason = "regression tests use assertions with fallible fixture setup"
)]
use ristretto_classfile::Result;
use ristretto_classfile::mutf8;

#[test]
fn modified_utf8_owned_decoder_rejects_invalid_encodings() {
    for bytes in [
        &[0][..],
        &[b'a', 0, b'b'],
        &[0xF0, 0x9F, 0x98, 0x80],
        &[0xC0, 0x81],
        &[0xC1, 0xBF],
        &[0xE0, 0x80, 0x80],
    ] {
        assert!(mutf8::from_bytes(bytes).is_err(), "{bytes:x?}");
    }
}

#[test]
fn modified_utf8_borrowing_decoder_rejects_invalid_encodings() {
    for bytes in [
        &[0][..],
        &[b'a', 0, b'b'],
        &[0xF0, 0x9F, 0x98, 0x80],
        &[0xC0, 0x81],
        &[0xC1, 0xBF],
        &[0xE0, 0x80, 0x80],
    ] {
        assert!(mutf8::from_bytes_cow(bytes).is_err(), "{bytes:x?}");
    }
}

#[test]
fn modified_utf8_decoders_accept_encoded_null_and_surrogate_pair() -> Result<()> {
    let bytes = [b'a', 0xC0, 0x80, 0xED, 0xA0, 0xBD, 0xED, 0xB8, 0x80];
    assert_eq!(mutf8::from_bytes(&bytes)?, "a\0😀");
    assert_eq!(mutf8::from_bytes_cow(&bytes)?, "a\0😀");
    assert!(matches!(
        mutf8::from_bytes_cow(b"ascii")?,
        std::borrow::Cow::Borrowed("ascii")
    ));
    Ok(())
}
