# CESU-8 Edge Cases and Compatibility Notes

This document details important edge cases, compatibility pitfalls, and examples when working with CESU-8 (Compatibility
Encoding Scheme for UTF-16: 8-Bit). It is intended as a practical reference for implementors of JVM, JNI, or
cross-language tools, and for interoperability with Rust.

---

## What is CESU-8?

CESU-8 is a Unicode transformation format similar to UTF-8, but with crucial differences:

- CESU-8 encodes UTF-16 code units using a UTF-8-like scheme, **not Unicode scalar values**.
- As a result, **supplementary characters** (`U+10000` and above) are always encoded as *surrogate pairs* (each as 3
  bytes), **never as a single 4-byte UTF-8 sequence**.
- CESU-8 is used by Java's internal APIs (sometimes in older JDBC drivers, some JVM serialization, and Android) but is
  not a standard Internet encoding.

---

## Key Differences and Edge Cases

### 1. **Supplementary Characters Encoding**

- **UTF-8**: `U+1F600` (😀) is `[0xF0, 0x9F, 0x98, 0x80]` (4 bytes)
- **CESU-8**: `U+1F600` is `[0xED, 0xA0, 0xBD, 0xED, 0xB8, 0x80]` (6 bytes, as two surrogates)
    - `U+1F600` in UTF-16 is `[0xD83D, 0xDE00]`
    - Each surrogate is encoded as a 3-byte UTF-8 sequence.

### 2. **Lone Surrogates Are Allowed**

- CESU-8 may encode and decode **unpaired surrogates** (e.g., `[0xED, 0xA0, 0x80]` for `U+D800`), which are not allowed
  in Unicode strings or Rust `String`.
- This allows round-tripping arbitrary UTF-16, but is incompatible with strict UTF-8 consumers.

### 3. **Null Character**

- **Null (`U+0000`)** is encoded as `[0x00]` (same as UTF-8), not as `[0xC0, 0x80]` (like MUTF-8).

### 4. **Invalid Surrogate Sequences**

- CESU-8 decoders must be able to accept and emit invalid or ill-formed sequences (e.g., a low surrogate not preceded
  by a high surrogate).
- This is unlike strict Unicode, but matches Java's "any sequence of UTF-16 code units" model.

---

## Common Edge Cases and Examples

| UTF-16 Sequence    | CESU-8 Bytes                           | Notes                                   |
|--------------------|----------------------------------------|-----------------------------------------|
| `[0x0000]`         | `[0x00]`                               | Null, encoded as in UTF-8               |
| `[0x0041]` (`'A'`) | `[0x41]`                               | ASCII                                   |
| `[0x0080]`         | `[0xC2, 0x80]`                         | Two bytes, same as UTF-8                |
| `[0x07FF]`         | `[0xDF, 0xBF]`                         | Two bytes, same as UTF-8                |
| `[0x0800]`         | `[0xE0, 0xA0, 0x80]`                   | Three bytes, same as UTF-8              |
| `[0xFFFF]`         | `[0xEF, 0xBF, 0xBF]`                   | Three bytes, same as UTF-8              |
| `[0xD800]`         | `[0xED, 0xA0, 0x80]`                   | Lone high surrogate, not legal in Rust  |
| `[0xD83D, 0xDE00]` | `[0xED, 0xA0, 0xBD, 0xED, 0xB8, 0x80]` | Surrogate pair (😀), not a 4-byte UTF-8 |
| `[0xDC00]`         | `[0xED, 0xB0, 0x80]`                   | Lone low surrogate                      |

### Example: Unpaired Surrogate

- CESU-8 bytes: `[0xED, 0xA2, 0xA2]`
- Decoded UTF-16: `[0xD8A2]` (unpaired high surrogate)
- **Rust's `String::from_utf16` will reject this**, but Java will allow it.

---

## Compatibility Warnings

- **Rust**:
    - Rust's `String` and `char` types **cannot represent unpaired surrogates**. These will be lost or replaced if
      CESU-8 is decoded into Rust `String`.
    - Use `Vec<u16>` or a custom type if lossless round-tripping is needed.

- **UTF-8 Tools**:
    - CESU-8 is **not valid UTF-8** for supplementary characters.  
      Feeding CESU-8 to a strict UTF-8 parser will result in errors for surrogate sequences.
    - Never use CESU-8 for external data or standard Internet protocols.

- **Java/JVM**:
    - Java will happily encode/decode lone surrogates in CESU-8.
    - CESU-8 is *almost* MUTF-8, but MUTF-8 reserves `[0xC0, 0x80]` for null and never encodes a null byte.

---

## Practical Guidance

- For **Java-Rust interoperability**, decode CESU-8 to `Vec<u16>`, not `String`, to avoid information loss.
- **Never feed CESU-8 directly to Rust's `String::from_utf8` or `from_utf16`** unless you are certain there are no lone
  surrogates.
- When re-encoding, remember that Java may emit CESU-8 for supplementary characters and expect round-trip of unpaired
  surrogates.

---

## References

- [Unicode Consortium: CESU-8 Definition](https://www.unicode.org/reports/tr26/)
- [Wikipedia: CESU-8](https://en.wikipedia.org/wiki/CESU-8)
- [Java String Encoding](https://docs.oracle.com/en/java/javase/24/docs/api/java.base/java/nio/charset/Charset.html)
- [Rust String Unicode Docs](https://doc.rust-lang.org/std/string/struct.String.html#utf-8)

---
