# MUTF-8 Specification Notes

This document summarizes the Modified UTF-8 (MUTF-8) encoding used by the Java Virtual Machine (JVM), highlighting how
it differs from standard UTF-8, handling of special characters, surrogate pairs, and implementation edge cases.

---

## What is MUTF-8?

**MUTF-8** (Modified UTF-8) is a variant of UTF-8 encoding used by Java for representing strings in class files and
certain serialized data formats (notably DataInput/DataOutput, JNI, and the constant pool). Its purpose is to ensure
that all encoded strings are compatible with null-terminated C strings and legacy Java expectations.

---

## Key Properties and Differences from Standard UTF-8

1. **Encoding of the Null Character (`U+0000`):**
    - **Standard UTF-8:** Encoded as the single byte `0x00`.
    - **MUTF-8:** Encoded as the two-byte sequence `[0xC0, 0x80]`.
        - This prevents embedded nulls in encoded data, preserving C-string compatibility.

2. **BMP Code Points (excluding null and surrogates):**
    - Encoded identically in UTF-8 and MUTF-8.

3. **Supplementary Characters (`U+10000` and above):**
    - **Standard UTF-8:** Encoded as a four-byte sequence.
    - **MUTF-8:** Encoded as *two* three-byte sequences, representing the character’s UTF-16 surrogate pair, each
      encoded as if they were BMP code points.
        - Example: `U+10000` in UTF-16 is `[0xD800, 0xDC00]` and in MUTF-8 is `[0xED,0xA0,0x80, 0xED,0xB0,0x80]`.

4. **Surrogate Code Points (`U+D800..=U+DFFF`):**
    - Although not valid Unicode scalar values, MUTF-8 permits these as individual code units for round-trip
      compatibility with Java's `String` (which is a sequence of UTF-16 units).

5. **Absence of 4-Byte Sequences:**
    - MUTF-8 *never* uses 4-byte UTF-8 sequences (`0xF0..0xF7`). All non-BMP characters are always encoded as surrogate
      pairs.

---

## Encoding Rules

| Unicode Code Point  | MUTF-8 Encoding                    | Example                                        |
|---------------------|------------------------------------|------------------------------------------------|
| `U+0000`            | `0xC0 0x80`                        | `\0` encoded as `[0xC0, 0x80]`                 |
| `U+0001..U+007F`    | `0x01..0x7F`                       | `'A'` encoded as `[0x41]`                      |
| `U+0080..U+07FF`    | `0xC2..0xDF 0x80..0xBF`            | `U+0080` as `[0xC2, 0x80]`                     |
| `U+0800..U+FFFF`    | `0xE0..0xEF 0x80..0xBF 0x80..0xBF` | `U+0800` as `[0xE0,0xA0,0x80]`                 |
| `U+10000..U+10FFFF` | Each surrogate as 3 bytes          | `U+10000` as `[0xED,0xA0,0x80,0xED,0xB0,0x80]` |

---

## Surrogate Handling and Supplementary Characters

- **Java's UTF-16** represents supplementary code points (`U+10000` and above) as *two* `u16` code units (a surrogate
  pair).
- **MUTF-8** encodes each `u16` separately as a three-byte sequence (as if it were in the BMP).
- **Round-trip Guarantee:** Any sequence of UTF-16 code units (including unpaired surrogates) can be encoded and decoded
  without loss.

### Example: Encoding `U+1F600` (😀)

1. UTF-16: `[0xD83D, 0xDE00]`
2. MUTF-8: `[0xED, 0xA0, 0xBD, 0xED, 0xB8, 0x80]`
3. Standard UTF-8: `[0xF0, 0x9F, 0x98, 0x80]`

### Example: Encoding an unpaired surrogate

- UTF-16: `[0xD800]`
- MUTF-8: `[0xED, 0xA0, 0x80]`

---

## Edge Cases and Implementation Notes

- **Unpaired Surrogates:** MUTF-8 may legally encode lone high or low surrogates; this is forbidden in Rust `String`,
  but permitted in Java.
- **Embedded Nulls:** MUTF-8 always encodes null as `[0xC0, 0x80]`; the byte `0x00` never appears except as a
  terminator.
- **Decoding Strictness:** MUTF-8 decoders (such as in the JVM) must accept and preserve all valid UTF-16 code units,
  not just scalar values.

---

## Reference Table: Comparison with UTF-8

| Code Point(s) | Java UTF-16      | UTF-8 Encoding           | MUTF-8 Encoding                      |
|---------------|------------------|--------------------------|--------------------------------------|
| U+0000        | [0x0000]         | [0x00]                   | [0xC0, 0x80]                         |
| U+0041        | [0x0041]         | [0x41]                   | [0x41]                               |
| U+07FF        | [0x07FF]         | [0xDF, 0xBF]             | [0xDF, 0xBF]                         |
| U+0800        | [0x0800]         | [0xE0, 0xA0, 0x80]       | [0xE0, 0xA0, 0x80]                   |
| U+10000       | [0xD800, 0xDC00] | [0xF0, 0x90, 0x80, 0x80] | [0xED, 0xA0, 0x80, 0xED, 0xB0, 0x80] |
| U+D800        | [0xD800]         | — (not allowed)          | [0xED, 0xA0, 0x80]                   |

---

## Practical Guidance

- **When implementing or consuming MUTF-8:**
    - Always handle nulls and surrogate pairs per MUTF-8 rules.
    - Be aware that decoded data may include lone surrogates; these cannot be stored in Rust `String`.
    - For interoperability, consider working directly with UTF-16 (`Vec<u16>`) for lossless round-trip with Java.

---

## References

- [Java Virtual Machine Specification, §4.4.7: Modified UTF-8 Strings](https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.4.7)
- [Wikipedia: Modified UTF-8](https://en.wikipedia.org/wiki/Modified_UTF-8)
- [Unicode Standard, Section 3.9: Unicode Encoding Forms](https://www.unicode.org/versions/Unicode15.0.0/ch03.pdf#G2630)

---
