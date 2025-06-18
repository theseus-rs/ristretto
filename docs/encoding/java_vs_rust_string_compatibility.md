# Java vs Rust String Compatibility

This document describes important differences between Java and Rust with respect to valid string values, character
encoding, and what may be represented safely in each language. It specifically addresses the handling of UTF-16
surrogates, MUTF-8, CESU-8, and Rust's strict UTF-8 enforcement.

---

## Unicode Scalar Values and Surrogate Handling

### Unicode Scalar Values

A **Unicode scalar value** is any Unicode code point except high and low surrogates. Formally:

- **Scalar values:** `U+0000..=U+D7FF` and `U+E000..=U+10FFFF`
- **Surrogates (not scalar):** `U+D800..=U+DFFF`

### Java String Representation

- Java's `String` type is a sequence of UTF-16 code units (`u16`).
- A Java string **may contain any sequence of `u16` values**, including unpaired surrogates.
- Java-specific UTF encodings (MUTF-8, CESU-8) can encode and decode unpaired surrogates, as well as valid pairs.

### Rust String Representation

- Rust's `String` and `char` types only accept **valid Unicode scalar values**.
- Any attempt to construct a Rust `String` or `char` with a surrogate (`U+D800..=U+DFFF`) will fail or be replaced with
  `�` (U+FFFD, the replacement character).

---

## Examples of (Non-)Representable Characters

| UTF-16 Code Units              | Description                        | Java `String`? | Rust `String`? |
|--------------------------------|------------------------------------|:--------------:|:--------------:|
| `U+0041`                       | Normal character 'A'               |      Yes       |      Yes       |
| `U+1F600` (`[U+D83D, U+DE00]`) | Supplementary char (emoji 😀)      |      Yes       |      Yes       |
| `U+D800`                       | Lone high surrogate                |      Yes       |       No       |
| `U+DC00`                       | Lone low surrogate                 |      Yes       |       No       |
| `[U+D800, U+DC00]`             | Valid surrogate pair (`U+10000`)   |      Yes       |      Yes       |
| `[U+DC00, U+D800]`             | Misordered surrogates (ill-formed) |      Yes       |       No       |

> **Note:** Any lone surrogate, or misordered surrogates, are allowed in Java strings and MUTF-8/CESU-8 encodings, but
> will be rejected by Rust `String::from_utf16` (error) or replaced with `�` by `String::from_utf16_lossy`. Ristretto
> uses `String::from_utf16_lossy` to decode UTF-16 data, which replaces invalid sequences with `�`.

---

## Encoding Implications (MUTF-8 / CESU-8)

| Bytes                                  | Decoded UTF-16     | Rust `String` Result | Java `String` Result      |
|----------------------------------------|--------------------|----------------------|---------------------------|
| `[0xED, 0xA0, 0x80]`                   | `[0xD800]`         | `�` (U+FFFD)         | `U+D800` (lone surrogate) |
| `[0xED, 0xB0, 0x80]`                   | `[0xDC00]`         | `�` (U+FFFD)         | `U+DC00` (lone surrogate) |
| `[0xED, 0xA0, 0x80, 0xED, 0xB0, 0x80]` | `[0xD800, 0xDC00]` | `U+10000` (😀)       | `U+10000` (😀)            |
| `[0xF0, 0x90, 0x80, 0x80]`             | `[0x10000]`        | `U+10000` (😀)       | `U+10000` (😀)            |

---

## Summary Table

| UTF-16 value       | Can Rust represent? | Can Java represent? | Notes                              |
|--------------------|:-------------------:|:-------------------:|------------------------------------|
| `U+0041`           |         Yes         |         Yes         | Normal character                   |
| `U+1F600`          |         Yes         |         Yes         | Supplementary character (emoji)    |
| `U+D800`           |       **No**        |         Yes         | Lone high surrogate                |
| `U+DC00`           |       **No**        |         Yes         | Lone low surrogate                 |
| `[U+D800, U+DC00]` |  Yes (as U+10000)   |         Yes         | Proper surrogate pair              |
| `[U+DC00, U+D800]` |       **No**        |         Yes         | Misordered surrogates (ill-formed) |

---

## Conclusion

When working with strings across Java and Rust, be aware of the differences in how each language handles Unicode scalar
values, surrogates, and encoding. Rust's strict UTF-8 enforcement means that any invalid sequences (like lone
surrogates) will not be accepted, while Java allows more flexibility with its `String` type. Ristretto currently uses
`String::from_utf16_lossy` to decode UTF-16 data, which replaces invalid sequences with the replacement character `�`.
This ensures that Rust strings remain valid UTF-8, but may lose some fidelity, and Java compatibility in the process.

---
