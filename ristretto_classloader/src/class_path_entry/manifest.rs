use crate::Result;
use ahash::RandomState;
use indexmap::IndexMap;
use std::fmt;
use std::fmt::{Display, Write};
use std::str::FromStr;
/// Maximum byte length of a single line in a manifest file.
///
/// Per the specification, no line may be longer than 72 bytes (in its UTF-8 encoded form).
const MAX_LINE_BYTES: usize = 72;
// Well-known main attribute names
/// The `Manifest-Version` main attribute. Must be the first attribute in the main section.
pub const MANIFEST_VERSION: &str = "Manifest-Version";
/// The `Created-By` main attribute.
pub const CREATED_BY: &str = "Created-By";
/// The `Signature-Version` main attribute.
pub const SIGNATURE_VERSION: &str = "Signature-Version";
/// The `Class-Path` main attribute.
pub const CLASS_PATH: &str = "Class-Path";
/// The `Main-Class` main attribute.
pub const MAIN_CLASS: &str = "Main-Class";
/// The `Extension-Name` main attribute.
pub const EXTENSION_NAME: &str = "Extension-Name";
/// The `Implementation-Title` main attribute.
pub const IMPLEMENTATION_TITLE: &str = "Implementation-Title";
/// The `Implementation-Version` main attribute.
pub const IMPLEMENTATION_VERSION: &str = "Implementation-Version";
/// The `Implementation-Vendor` main attribute.
pub const IMPLEMENTATION_VENDOR: &str = "Implementation-Vendor";
/// The `Specification-Title` main attribute.
pub const SPECIFICATION_TITLE: &str = "Specification-Title";
/// The `Specification-Version` main attribute.
pub const SPECIFICATION_VERSION: &str = "Specification-Version";
/// The `Specification-Vendor` main attribute.
pub const SPECIFICATION_VENDOR: &str = "Specification-Vendor";
/// The `Sealed` main attribute.
pub const SEALED: &str = "Sealed";
/// The `Multi-Release` main attribute.
pub const MULTI_RELEASE: &str = "Multi-Release";
/// The `Automatic-Module-Name` main attribute.
pub const AUTOMATIC_MODULE_NAME: &str = "Automatic-Module-Name";
// Well-known per-entry (section) attribute names
/// The `Content-Type` per-entry attribute.
pub const CONTENT_TYPE: &str = "Content-Type";
/// The `Java-Bean` per-entry attribute.
pub const JAVA_BEAN: &str = "Java-Bean";
// Digest attribute names
/// The `SHA-256-Digest` attribute (used in signed JARs).
pub const SHA_256_DIGEST: &str = "SHA-256-Digest";
/// The `SHA1-Digest` attribute (used in signed JARs, legacy).
pub const SHA1_DIGEST: &str = "SHA1-Digest";
/// An insertion-ordered map of attribute key-value pairs, using `ahash` for faster hashing.
pub type AttributeMap = IndexMap<String, String, RandomState>;
/// An insertion-ordered map of named sections, each containing an [`AttributeMap`].
pub type SectionMap = IndexMap<String, AttributeMap, RandomState>;

/// Represents a JAR manifest file.
///
/// A manifest consists of a main section of attributes followed by zero or more
/// individual sections. Each individual section is identified by a `Name` attribute.
///
/// # Line Length and Continuation Lines
///
/// Per the specification, no line may be longer than 72 bytes (in its UTF-8 encoded form). Longer
/// values are split across multiple lines using continuation lines that begin with a single space
/// character.
///
/// # References
/// - [JAR File Specification](https://docs.oracle.com/en/java/javase/25/docs/specs/jar/jar.html#jar-manifest)
#[derive(Debug, Default)]
pub struct Manifest {
    /// Main section attributes (key-value pairs before any named section).
    pub attributes: AttributeMap,
    /// Individual sections, keyed by the `Name` attribute value.
    /// Each section contains its own set of key-value attributes.
    pub sections: SectionMap,
}

impl Manifest {
    /// Get a main section attribute value by key.
    ///
    /// Attribute name lookup is case-insensitive per the specification.
    pub fn attribute<S: AsRef<str>>(&self, key: S) -> Option<&str> {
        let key = key.as_ref();
        self.attributes
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case(key))
            .map(|(_, v)| v.as_str())
    }

    /// Get a section's attribute value by section name and attribute key.
    ///
    /// Section name lookup is exact; attribute key lookup is case-insensitive.
    pub fn section_attribute<S: AsRef<str>, K: AsRef<str>>(
        &self,
        section: S,
        key: K,
    ) -> Option<&str> {
        let key = key.as_ref();
        self.sections.get(section.as_ref()).and_then(|attrs| {
            attrs
                .iter()
                .find(|(k, _)| k.eq_ignore_ascii_case(key))
                .map(|(_, v)| v.as_str())
        })
    }
}

/// Write a manifest header line, wrapping at 72 bytes using continuation lines.
///
/// The first line has the format `key: value\n`. If this exceeds 72 bytes, the value is split and
/// subsequent continuation lines begin with a single space.
fn write_wrapped_line(out: &mut impl Write, key: &str, value: &str) -> fmt::Result {
    let prefix_len = key.len() + 2; // "key: "
    let total_len = prefix_len + value.len();

    if total_len <= MAX_LINE_BYTES {
        out.write_str(key)?;
        out.write_str(": ")?;
        out.write_str(value)?;
        out.write_char('\n')
    } else {
        // First line: key + ": " + as much value as fits within MAX_LINE_BYTES
        let max_first_value = MAX_LINE_BYTES.saturating_sub(prefix_len);
        let first_end = find_utf8_split(value.as_bytes(), max_first_value);
        out.write_str(key)?;
        out.write_str(": ")?;
        out.write_str(&value[..first_end])?;
        out.write_char('\n')?;

        // Continuation lines: " " prefix + up to (MAX_LINE_BYTES - 1) bytes of value
        let mut pos = first_end;
        let value_bytes = value.as_bytes();
        while pos < value.len() {
            let chunk_end = find_utf8_split(value_bytes, pos + MAX_LINE_BYTES - 1);
            out.write_char(' ')?;
            out.write_str(&value[pos..chunk_end])?;
            out.write_char('\n')?;
            pos = chunk_end;
        }
        Ok(())
    }
}

/// Find the largest byte index <= `max_pos` that is on a UTF-8 character boundary.
/// Returns `bytes.len()` if `max_pos >= bytes.len()`.
fn find_utf8_split(bytes: &[u8], max_pos: usize) -> usize {
    if max_pos >= bytes.len() {
        return bytes.len();
    }
    let mut pos = max_pos;
    // Walk backwards to find a valid UTF-8 char boundary
    while pos > 0 && is_utf8_continuation(bytes[pos]) {
        pos -= 1;
    }
    pos
}

/// Returns true if the byte is a UTF-8 continuation byte (10xxxxxx).
const fn is_utf8_continuation(b: u8) -> bool {
    b & 0b1100_0000 == 0b1000_0000
}

/// Flush a completed header into the appropriate map.
fn flush_header(
    key: &str,
    value: &str,
    attributes: &mut AttributeMap,
    sections: &mut SectionMap,
    current_section: &mut Option<String>,
) {
    if key.eq_ignore_ascii_case("Name") {
        let name = value.to_string();
        sections.entry(name.clone()).or_default();
        *current_section = Some(name);
    } else if let Some(ref section) = *current_section {
        if let Some(section_map) = sections.get_mut(section.as_str()) {
            section_map.insert(key.to_string(), value.to_string());
        }
    } else {
        attributes.insert(key.to_string(), value.to_string());
    }
}

/// Parse a manifest from its string representation in a single pass.
///
/// Handles continuation lines (lines starting with a single space), both `\r\n` and `\n` line
/// endings, and multiple named sections.
fn parse_manifest(input: &str) -> Manifest {
    let mut attributes = AttributeMap::default();
    let mut sections = SectionMap::default();
    let mut current_section: Option<String> = None;

    // Pending key borrows directly from `input`, avoiding an allocation.
    let mut pending_key: Option<&str> = None;
    let mut pending_value = String::new();

    for line in input.lines() {
        let line = line.strip_suffix('\r').unwrap_or(line);

        if let Some(continuation) = line.strip_prefix(' ') {
            // Continuation line: append to the pending value
            pending_value.push_str(continuation);
            continue;
        }

        // Non-continuation line -- flush any pending header first
        if let Some(key) = pending_key.take() {
            flush_header(
                key,
                &pending_value,
                &mut attributes,
                &mut sections,
                &mut current_section,
            );
            pending_value.clear();
        }

        if line.is_empty() {
            continue;
        }

        if let Some((key, value)) = line.split_once(": ") {
            pending_key = Some(key);
            pending_value.push_str(value);
        }
        // Malformed lines (no ": ") are silently skipped
    }

    // Flush the last pending header
    if let Some(key) = pending_key {
        flush_header(
            key,
            &pending_value,
            &mut attributes,
            &mut sections,
            &mut current_section,
        );
    }

    Manifest {
        attributes,
        sections,
    }
}

impl FromStr for Manifest {
    type Err = crate::Error;

    /// Parse the manifest from a string.
    ///
    /// Supports continuation lines (lines starting with a single space), both `\r\n` and `\n` line
    /// endings, and multiple named sections.
    ///
    /// # Errors
    ///
    /// Returns an error if the manifest cannot be parsed.
    fn from_str(input: &str) -> Result<Self> {
        Ok(parse_manifest(input))
    }
}

impl Display for Manifest {
    /// Format the manifest for display.
    ///
    /// Lines longer than 72 bytes are wrapped using continuation lines per the specification.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (key, value) in &self.attributes {
            write_wrapped_line(f, key, value)?;
        }
        for (section, attrs) in &self.sections {
            f.write_char('\n')?;
            write_wrapped_line(f, "Name", section)?;
            for (key, value) in attrs {
                write_wrapped_line(f, key, value)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXPECTED: &str = indoc! {r"
        Manifest-Version: 1.0
        Created-By: 0.0.0 (ristretto)
        Main-Class: com.example.Main

        Name: com/example/FooClass.class
        SHA-256-Digest: abcdef1234567890

        Name: com/example/AnotherClass.class
        SHA-256-Digest: 0987654321fedcba
    "};

    #[test]
    fn test_attribute() -> Result<()> {
        let manifest = Manifest::from_str(EXPECTED)?;
        let main_class = manifest.attribute(MAIN_CLASS).expect("main class");
        assert_eq!("com.example.Main", main_class);
        Ok(())
    }

    #[test]
    fn test_roundtrip_display() -> Result<()> {
        let manifest = Manifest::from_str(EXPECTED)?;
        let serialized = manifest.to_string();
        assert_eq!(EXPECTED, serialized);
        Ok(())
    }

    #[test]
    fn test_default_manifest_is_empty() {
        let manifest = Manifest::default();
        assert!(manifest.attributes.is_empty());
        assert!(manifest.sections.is_empty());
    }

    #[test]
    fn test_debug_impl() {
        let manifest = Manifest::default();
        let debug = format!("{manifest:?}");
        assert!(debug.contains("Manifest"));
    }

    #[test]
    fn test_attribute_not_found() -> Result<()> {
        let manifest = Manifest::from_str(EXPECTED)?;
        assert!(manifest.attribute("Non-Existent-Key").is_none());
        Ok(())
    }

    #[test]
    fn test_attribute_case_insensitive() -> Result<()> {
        let manifest = Manifest::from_str(EXPECTED)?;
        assert_eq!(Some("1.0"), manifest.attribute("manifest-version"));
        assert_eq!(Some("1.0"), manifest.attribute("MANIFEST-VERSION"));
        assert_eq!(Some("com.example.Main"), manifest.attribute("main-class"));
        Ok(())
    }

    #[test]
    fn test_manifest_version_attribute() -> Result<()> {
        let manifest = Manifest::from_str(EXPECTED)?;
        assert_eq!(Some("1.0"), manifest.attribute(MANIFEST_VERSION));
        Ok(())
    }

    #[test]
    fn test_created_by_attribute() -> Result<()> {
        let manifest = Manifest::from_str(EXPECTED)?;
        assert_eq!(Some("0.0.0 (ristretto)"), manifest.attribute(CREATED_BY));
        Ok(())
    }

    #[test]
    fn test_section_attribute() -> Result<()> {
        let manifest = Manifest::from_str(EXPECTED)?;
        assert_eq!(
            Some("abcdef1234567890"),
            manifest.section_attribute("com/example/FooClass.class", SHA_256_DIGEST)
        );
        assert_eq!(
            Some("0987654321fedcba"),
            manifest.section_attribute("com/example/AnotherClass.class", SHA_256_DIGEST)
        );
        Ok(())
    }

    #[test]
    fn test_section_attribute_not_found() -> Result<()> {
        let manifest = Manifest::from_str(EXPECTED)?;
        assert!(
            manifest
                .section_attribute("com/example/FooClass.class", "Non-Existent")
                .is_none()
        );
        Ok(())
    }

    #[test]
    fn test_section_not_found() -> Result<()> {
        let manifest = Manifest::from_str(EXPECTED)?;
        assert!(
            manifest
                .section_attribute("nonexistent/section", SHA_256_DIGEST)
                .is_none()
        );
        Ok(())
    }

    #[test]
    fn test_section_attribute_case_insensitive() -> Result<()> {
        let manifest = Manifest::from_str(EXPECTED)?;
        assert_eq!(
            Some("abcdef1234567890"),
            manifest.section_attribute("com/example/FooClass.class", "sha-256-digest")
        );
        Ok(())
    }

    #[test]
    fn test_multiple_sections() -> Result<()> {
        let manifest = Manifest::from_str(EXPECTED)?;
        assert_eq!(2, manifest.sections.len());
        assert!(manifest.sections.contains_key("com/example/FooClass.class"));
        assert!(
            manifest
                .sections
                .contains_key("com/example/AnotherClass.class")
        );
        Ok(())
    }

    #[test]
    fn test_no_sections() -> Result<()> {
        let input = "Manifest-Version: 1.0\nMain-Class: Hello\n";
        let manifest = Manifest::from_str(input)?;
        assert_eq!(2, manifest.attributes.len());
        assert!(manifest.sections.is_empty());
        Ok(())
    }

    #[test]
    fn test_section_with_multiple_attributes() -> Result<()> {
        let input = indoc! {"
            Manifest-Version: 1.0

            Name: com/example/Test.class
            SHA-256-Digest: abc123
            Content-Type: application/java
            Java-Bean: true
        "};
        let manifest = Manifest::from_str(input)?;
        let section = manifest.sections.get("com/example/Test.class").unwrap();
        assert_eq!(3, section.len());
        assert_eq!(
            Some("abc123"),
            section.get(SHA_256_DIGEST).map(String::as_str)
        );
        assert_eq!(
            Some("application/java"),
            section.get(CONTENT_TYPE).map(String::as_str)
        );
        assert_eq!(Some("true"), section.get(JAVA_BEAN).map(String::as_str));
        Ok(())
    }

    #[test]
    fn test_parse_continuation_lines() -> Result<()> {
        let input = "Manifest-Version: 1.0\nClass-Path: lib/a.jar lib/b.jar lib/c.jar lib/d\n .jar lib/e.jar\n";
        let manifest = Manifest::from_str(input)?;
        assert_eq!(
            Some("lib/a.jar lib/b.jar lib/c.jar lib/d.jar lib/e.jar"),
            manifest.attribute(CLASS_PATH)
        );
        Ok(())
    }

    #[test]
    fn test_parse_multiple_continuation_lines() -> Result<()> {
        let input =
            "Manifest-Version: 1.0\nLong-Value: aaaaaaaaa\n bbbbbbbbb\n ccccccccc\n ddddddddd\n";
        let manifest = Manifest::from_str(input)?;
        assert_eq!(
            Some("aaaaaaaaabbbbbbbbbcccccccccddddddddd"),
            manifest.attribute("Long-Value")
        );
        Ok(())
    }

    #[test]
    fn test_serialize_long_line_wrapping() {
        let mut manifest = Manifest::default();
        manifest
            .attributes
            .insert(MANIFEST_VERSION.to_string(), "1.0".to_string());
        let long_value = "a".repeat(200);
        manifest
            .attributes
            .insert("Long-Key".to_string(), long_value.clone());
        let serialized = manifest.to_string();
        for line in serialized.lines() {
            assert!(
                line.len() <= MAX_LINE_BYTES,
                "Line exceeds {MAX_LINE_BYTES} bytes: ({} bytes) {:?}",
                line.len(),
                line
            );
        }
        let reparsed = Manifest::from_str(&serialized).unwrap();
        assert_eq!(Some(long_value.as_str()), reparsed.attribute("Long-Key"));
    }

    #[test]
    fn test_serialize_short_lines_not_wrapped() {
        let mut manifest = Manifest::default();
        manifest
            .attributes
            .insert(MANIFEST_VERSION.to_string(), "1.0".to_string());
        manifest
            .attributes
            .insert(MAIN_CLASS.to_string(), "Hello".to_string());
        let serialized = manifest.to_string();
        assert!(!serialized.contains("\n "));
    }

    #[test]
    fn test_roundtrip_long_value() -> Result<()> {
        let mut manifest = Manifest::default();
        manifest
            .attributes
            .insert(MANIFEST_VERSION.to_string(), "1.0".to_string());
        let long_cp = (0..20)
            .map(|i| format!("lib/dependency-{i}.jar"))
            .collect::<Vec<_>>()
            .join(" ");
        manifest
            .attributes
            .insert(CLASS_PATH.to_string(), long_cp.clone());
        let serialized = manifest.to_string();
        let reparsed = Manifest::from_str(&serialized)?;
        assert_eq!(Some(long_cp.as_str()), reparsed.attribute(CLASS_PATH));
        Ok(())
    }

    #[test]
    fn test_parse_crlf_line_endings() -> Result<()> {
        let input = "Manifest-Version: 1.0\r\nMain-Class: com.example.Main\r\n\r\nName: section\r\nKey: Value\r\n";
        let manifest = Manifest::from_str(input)?;
        assert_eq!(Some("1.0"), manifest.attribute(MANIFEST_VERSION));
        assert_eq!(Some("com.example.Main"), manifest.attribute(MAIN_CLASS));
        assert_eq!(Some("Value"), manifest.section_attribute("section", "Key"));
        Ok(())
    }

    #[test]
    fn test_parse_crlf_with_continuation() -> Result<()> {
        let input = "Manifest-Version: 1.0\r\nLong-Key: abcdef\r\n ghijkl\r\n";
        let manifest = Manifest::from_str(input)?;
        assert_eq!(Some("abcdefghijkl"), manifest.attribute("Long-Key"));
        Ok(())
    }

    #[test]
    fn test_empty_manifest() -> Result<()> {
        let manifest = Manifest::from_str("")?;
        assert!(manifest.attributes.is_empty());
        assert!(manifest.sections.is_empty());
        Ok(())
    }

    #[test]
    fn test_manifest_only_newlines() -> Result<()> {
        let manifest = Manifest::from_str("\n\n\n")?;
        assert!(manifest.attributes.is_empty());
        assert!(manifest.sections.is_empty());
        Ok(())
    }

    #[test]
    fn test_malformed_lines_are_skipped() -> Result<()> {
        let input = "Manifest-Version: 1.0\nthis line has no colon-space\nMain-Class: Hello\n";
        let manifest = Manifest::from_str(input)?;
        assert_eq!(Some("1.0"), manifest.attribute(MANIFEST_VERSION));
        assert_eq!(Some("Hello"), manifest.attribute(MAIN_CLASS));
        assert_eq!(2, manifest.attributes.len());
        Ok(())
    }

    #[test]
    fn test_value_containing_colon_space() -> Result<()> {
        let input = "Manifest-Version: 1.0\nDescription: This has: a colon in it\n";
        let manifest = Manifest::from_str(input)?;
        assert_eq!(
            Some("This has: a colon in it"),
            manifest.attribute("Description")
        );
        Ok(())
    }

    #[test]
    fn test_section_name_preserved() -> Result<()> {
        let input = "Manifest-Version: 1.0\n\nName: com/example/Foo Bar.class\nSealed: true\n";
        let manifest = Manifest::from_str(input)?;
        assert!(manifest.sections.contains_key("com/example/Foo Bar.class"));
        Ok(())
    }

    #[test]
    fn test_sealed_attribute() -> Result<()> {
        let manifest = Manifest::from_str("Manifest-Version: 1.0\nSealed: true\n")?;
        assert_eq!(Some("true"), manifest.attribute(SEALED));
        Ok(())
    }

    #[test]
    fn test_multi_release_attribute() -> Result<()> {
        let manifest = Manifest::from_str("Manifest-Version: 1.0\nMulti-Release: true\n")?;
        assert_eq!(Some("true"), manifest.attribute(MULTI_RELEASE));
        Ok(())
    }

    #[test]
    fn test_automatic_module_name() -> Result<()> {
        let manifest = Manifest::from_str(
            "Manifest-Version: 1.0\nAutomatic-Module-Name: com.example.module\n",
        )?;
        assert_eq!(
            Some("com.example.module"),
            manifest.attribute(AUTOMATIC_MODULE_NAME)
        );
        Ok(())
    }

    #[test]
    fn test_extension_name_attribute() -> Result<()> {
        let manifest =
            Manifest::from_str("Manifest-Version: 1.0\nExtension-Name: javax.servlet\n")?;
        assert_eq!(Some("javax.servlet"), manifest.attribute(EXTENSION_NAME));
        Ok(())
    }

    #[test]
    fn test_implementation_attributes() -> Result<()> {
        let input = indoc! {"
            Manifest-Version: 1.0
            Implementation-Title: Example
            Implementation-Version: 2.0
            Implementation-Vendor: ACME Corp
        "};
        let manifest = Manifest::from_str(input)?;
        assert_eq!(Some("Example"), manifest.attribute(IMPLEMENTATION_TITLE));
        assert_eq!(Some("2.0"), manifest.attribute(IMPLEMENTATION_VERSION));
        assert_eq!(Some("ACME Corp"), manifest.attribute(IMPLEMENTATION_VENDOR));
        Ok(())
    }

    #[test]
    fn test_specification_attributes() -> Result<()> {
        let input = indoc! {"
            Manifest-Version: 1.0
            Specification-Title: Example API
            Specification-Version: 3.1
            Specification-Vendor: ACME Corp
        "};
        let manifest = Manifest::from_str(input)?;
        assert_eq!(Some("Example API"), manifest.attribute(SPECIFICATION_TITLE));
        assert_eq!(Some("3.1"), manifest.attribute(SPECIFICATION_VERSION));
        assert_eq!(Some("ACME Corp"), manifest.attribute(SPECIFICATION_VENDOR));
        Ok(())
    }

    #[test]
    fn test_class_path_attribute() -> Result<()> {
        let manifest =
            Manifest::from_str("Manifest-Version: 1.0\nClass-Path: lib/a.jar lib/b.jar\n")?;
        assert_eq!(Some("lib/a.jar lib/b.jar"), manifest.attribute(CLASS_PATH));
        Ok(())
    }

    #[test]
    fn test_signature_version_attribute() -> Result<()> {
        let manifest = Manifest::from_str("Manifest-Version: 1.0\nSignature-Version: 1.0\n")?;
        assert_eq!(Some("1.0"), manifest.attribute(SIGNATURE_VERSION));
        Ok(())
    }

    #[test]
    fn test_per_entry_content_type() -> Result<()> {
        let input = indoc! {"
            Manifest-Version: 1.0

            Name: images/logo.png
            Content-Type: image/png
        "};
        let manifest = Manifest::from_str(input)?;
        assert_eq!(
            Some("image/png"),
            manifest.section_attribute("images/logo.png", CONTENT_TYPE)
        );
        Ok(())
    }

    #[test]
    fn test_per_entry_java_bean() -> Result<()> {
        let input = indoc! {"
            Manifest-Version: 1.0

            Name: com/example/MyBean.class
            Java-Bean: true
        "};
        let manifest = Manifest::from_str(input)?;
        assert_eq!(
            Some("true"),
            manifest.section_attribute("com/example/MyBean.class", JAVA_BEAN)
        );
        Ok(())
    }

    #[test]
    fn test_per_entry_sealed() -> Result<()> {
        let input = indoc! {"
            Manifest-Version: 1.0

            Name: com/example/
            Sealed: true
        "};
        let manifest = Manifest::from_str(input)?;
        assert_eq!(
            Some("true"),
            manifest.section_attribute("com/example/", SEALED)
        );
        Ok(())
    }

    #[test]
    fn test_per_entry_digest_attributes() -> Result<()> {
        let input = indoc! {"
            Manifest-Version: 1.0

            Name: com/example/Foo.class
            SHA-256-Digest: abcdef1234567890
            SHA1-Digest: 0123456789abcdef
        "};
        let manifest = Manifest::from_str(input)?;
        assert_eq!(
            Some("abcdef1234567890"),
            manifest.section_attribute("com/example/Foo.class", SHA_256_DIGEST)
        );
        assert_eq!(
            Some("0123456789abcdef"),
            manifest.section_attribute("com/example/Foo.class", SHA1_DIGEST)
        );
        Ok(())
    }

    #[test]
    fn test_serialize_empty_manifest() {
        let manifest = Manifest::default();
        assert_eq!("", manifest.to_string());
    }

    #[test]
    fn test_serialize_main_attributes_only() {
        let mut manifest = Manifest::default();
        manifest
            .attributes
            .insert(MANIFEST_VERSION.to_string(), "1.0".to_string());
        manifest
            .attributes
            .insert(MAIN_CLASS.to_string(), "com.example.Main".to_string());
        assert_eq!(
            "Manifest-Version: 1.0\nMain-Class: com.example.Main\n",
            manifest.to_string()
        );
    }

    #[test]
    fn test_serialize_with_sections() {
        let mut manifest = Manifest::default();
        manifest
            .attributes
            .insert(MANIFEST_VERSION.to_string(), "1.0".to_string());
        let mut section = AttributeMap::default();
        section.insert(SEALED.to_string(), "true".to_string());
        manifest
            .sections
            .insert("com/example/".to_string(), section);
        assert_eq!(
            "Manifest-Version: 1.0\n\nName: com/example/\nSealed: true\n",
            manifest.to_string()
        );
    }

    #[test]
    fn test_roundtrip_complex_manifest() -> Result<()> {
        let mut manifest = Manifest::default();
        manifest
            .attributes
            .insert(MANIFEST_VERSION.to_string(), "1.0".to_string());
        manifest
            .attributes
            .insert(CREATED_BY.to_string(), "25.0 (test)".to_string());
        manifest
            .attributes
            .insert(MAIN_CLASS.to_string(), "com.example.Main".to_string());
        manifest
            .attributes
            .insert(MULTI_RELEASE.to_string(), "true".to_string());
        let mut section1 = AttributeMap::default();
        section1.insert(SHA_256_DIGEST.to_string(), "abc123".to_string());
        manifest
            .sections
            .insert("com/example/A.class".to_string(), section1);
        let mut section2 = AttributeMap::default();
        section2.insert(SEALED.to_string(), "true".to_string());
        manifest
            .sections
            .insert("com/example/".to_string(), section2);

        let serialized = manifest.to_string();
        let reparsed = Manifest::from_str(&serialized)?;
        assert_eq!(manifest.attributes.len(), reparsed.attributes.len());
        assert_eq!(manifest.sections.len(), reparsed.sections.len());
        for (key, value) in &manifest.attributes {
            assert_eq!(Some(value.as_str()), reparsed.attribute(key));
        }
        for (section_name, attrs) in &manifest.sections {
            for (key, value) in attrs {
                assert_eq!(
                    Some(value.as_str()),
                    reparsed.section_attribute(section_name, key)
                );
            }
        }
        Ok(())
    }

    #[test]
    fn test_roundtrip_with_continuation() -> Result<()> {
        let mut manifest = Manifest::default();
        manifest
            .attributes
            .insert(MANIFEST_VERSION.to_string(), "1.0".to_string());
        let long_cp = (0..30)
            .map(|i| format!("lib/very-long-dependency-name-{i}.jar"))
            .collect::<Vec<_>>()
            .join(" ");
        manifest
            .attributes
            .insert(CLASS_PATH.to_string(), long_cp.clone());
        let serialized = manifest.to_string();
        for line in serialized.lines() {
            assert!(
                line.len() <= MAX_LINE_BYTES,
                "Line exceeds limit: {} bytes",
                line.len()
            );
        }
        let reparsed = Manifest::from_str(&serialized)?;
        assert_eq!(Some(long_cp.as_str()), reparsed.attribute(CLASS_PATH));
        Ok(())
    }

    #[test]
    fn test_find_utf8_split_ascii() {
        assert_eq!(5, find_utf8_split("Hello, World!".as_bytes(), 5));
    }

    #[test]
    fn test_find_utf8_split_at_end() {
        assert_eq!(5, find_utf8_split("Hello".as_bytes(), 10));
    }

    #[test]
    fn test_find_utf8_split_multibyte() {
        let s = "caf\u{00e9}";
        let bytes = s.as_bytes();
        assert_eq!(3, find_utf8_split(bytes, 4));
        assert_eq!(5, find_utf8_split(bytes, 5));
    }

    #[test]
    fn test_is_utf8_continuation() {
        assert!(is_utf8_continuation(0b1000_0000));
        assert!(is_utf8_continuation(0b1011_1111));
        assert!(!is_utf8_continuation(0b0000_0000));
        assert!(!is_utf8_continuation(0b1100_0000));
        assert!(!is_utf8_continuation(0b0111_1111));
    }

    #[test]
    fn test_write_wrapped_line_short() {
        let mut out = String::new();
        write_wrapped_line(&mut out, "Key", "Value").unwrap();
        assert_eq!("Key: Value\n", out);
    }

    #[test]
    fn test_write_wrapped_line_exactly_72() {
        let mut out = String::new();
        let value = "a".repeat(67);
        write_wrapped_line(&mut out, "Key", &value).unwrap();
        assert_eq!(format!("Key: {value}\n"), out);
        assert_eq!(1, out.lines().count());
    }

    #[test]
    fn test_write_wrapped_line_73_bytes() {
        let mut out = String::new();
        let value = "a".repeat(68);
        write_wrapped_line(&mut out, "Key", &value).unwrap();
        assert!(out.lines().count() > 1);
        for line in out.lines() {
            assert!(line.len() <= MAX_LINE_BYTES);
        }
    }

    #[test]
    fn test_parse_manifest_continuation_and_sections() {
        let input = "Manifest-Version: 1.0\nKey: abc\n def\n\nName: section\nAttr: val\n";
        let manifest = parse_manifest(input);
        assert_eq!(Some("abcdef"), manifest.attribute("Key"));
        assert_eq!(Some("val"), manifest.section_attribute("section", "Attr"));
    }

    #[test]
    fn test_parse_manifest_crlf() {
        let manifest = parse_manifest("key1: value1\r\nkey2: value2\r\n");
        assert_eq!(Some("value1"), manifest.attribute("key1"));
        assert_eq!(Some("value2"), manifest.attribute("key2"));
    }

    #[test]
    fn test_parse_manifest_crlf_continuation() {
        let manifest = parse_manifest("key: abc\r\n def\r\n");
        assert_eq!(Some("abcdef"), manifest.attribute("key"));
    }

    #[test]
    fn test_parse_manifest_empty() {
        let manifest = parse_manifest("");
        assert!(manifest.attributes.is_empty());
        assert!(manifest.sections.is_empty());
    }

    #[test]
    fn test_parse_manifest_multiple_continuations() {
        let manifest = parse_manifest("key: aaa\n bbb\n ccc\n");
        assert_eq!(Some("aaabbbccc"), manifest.attribute("key"));
    }

    #[test]
    fn test_spring_boot_manifest() -> Result<()> {
        let input = indoc! {"
            Manifest-Version: 1.0
            Created-By: Maven JAR Plugin 3.3.0
            Build-Jdk-Spec: 17
            Implementation-Title: my-app
            Implementation-Version: 0.0.1-SNAPSHOT
            Main-Class: org.springframework.boot.loader.JarLauncher
            Start-Class: com.example.demo.DemoApplication
            Spring-Boot-Version: 3.2.0
            Spring-Boot-Classes: BOOT-INF/classes/
            Spring-Boot-Lib: BOOT-INF/lib/
            Spring-Boot-Classpath-Index: BOOT-INF/classpath.idx
            Spring-Boot-Layers-Index: BOOT-INF/layers.idx
        "};
        let manifest = Manifest::from_str(input)?;
        assert_eq!(Some("1.0"), manifest.attribute(MANIFEST_VERSION));
        assert_eq!(
            Some("org.springframework.boot.loader.JarLauncher"),
            manifest.attribute(MAIN_CLASS)
        );
        assert_eq!(Some("my-app"), manifest.attribute(IMPLEMENTATION_TITLE));
        assert_eq!(12, manifest.attributes.len());
        Ok(())
    }

    #[test]
    fn test_signed_jar_manifest() -> Result<()> {
        let input = indoc! {"
            Manifest-Version: 1.0
            Created-By: 1.8.0 (Test)

            Name: com/example/Foo.class
            SHA-256-Digest: dGVzdCBkaWdlc3QgdmFsdWUgZm9yIGZvbw==

            Name: com/example/Bar.class
            SHA-256-Digest: dGVzdCBkaWdlc3QgdmFsdWUgZm9yIGJhcg==
        "};
        let manifest = Manifest::from_str(input)?;
        assert_eq!(2, manifest.sections.len());
        assert_eq!(
            Some("dGVzdCBkaWdlc3QgdmFsdWUgZm9yIGZvbw=="),
            manifest.section_attribute("com/example/Foo.class", SHA_256_DIGEST)
        );
        assert_eq!(
            Some("dGVzdCBkaWdlc3QgdmFsdWUgZm9yIGJhcg=="),
            manifest.section_attribute("com/example/Bar.class", SHA_256_DIGEST)
        );
        Ok(())
    }

    #[test]
    fn test_multi_release_jar_manifest() -> Result<()> {
        let input = indoc! {"
            Manifest-Version: 1.0
            Multi-Release: true
            Main-Class: com.example.App
        "};
        let manifest = Manifest::from_str(input)?;
        assert_eq!(Some("true"), manifest.attribute(MULTI_RELEASE));
        assert_eq!(Some("com.example.App"), manifest.attribute(MAIN_CLASS));
        Ok(())
    }

    #[test]
    fn test_class_path_with_continuation() -> Result<()> {
        let mut manifest = Manifest::default();
        manifest
            .attributes
            .insert(MANIFEST_VERSION.to_string(), "1.0".to_string());
        let jars: Vec<String> = (0..50).map(|i| format!("lib/dep-{i}.jar")).collect();
        let class_path = jars.join(" ");
        manifest
            .attributes
            .insert(CLASS_PATH.to_string(), class_path.clone());
        let serialized = manifest.to_string();
        for line in serialized.lines() {
            assert!(
                line.len() <= MAX_LINE_BYTES,
                "Line exceeds limit: {} bytes",
                line.len()
            );
        }
        let reparsed = Manifest::from_str(&serialized)?;
        assert_eq!(Some(class_path.as_str()), reparsed.attribute(CLASS_PATH));
        Ok(())
    }

    #[test]
    fn test_attribute_with_underscore() -> Result<()> {
        let manifest = Manifest::from_str("Manifest-Version: 1.0\nCustom_Attr: hello\n")?;
        assert_eq!(Some("hello"), manifest.attribute("Custom_Attr"));
        Ok(())
    }

    #[test]
    fn test_attribute_with_digits() -> Result<()> {
        let manifest = Manifest::from_str("Manifest-Version: 1.0\nX-Attr-123: test\n")?;
        assert_eq!(Some("test"), manifest.attribute("X-Attr-123"));
        Ok(())
    }

    #[test]
    fn test_serialize_section_with_long_name() -> Result<()> {
        let mut manifest = Manifest::default();
        manifest
            .attributes
            .insert(MANIFEST_VERSION.to_string(), "1.0".to_string());
        let long_name = format!(
            "com/example/very/long/package/name/{}.class",
            "A".repeat(100)
        );
        let mut section = AttributeMap::default();
        section.insert(SHA_256_DIGEST.to_string(), "abc123".to_string());
        manifest.sections.insert(long_name.clone(), section);
        let serialized = manifest.to_string();
        for line in serialized.lines() {
            assert!(
                line.len() <= MAX_LINE_BYTES,
                "Line exceeds limit: {} bytes: {:?}",
                line.len(),
                line
            );
        }
        let reparsed = Manifest::from_str(&serialized)?;
        assert!(reparsed.sections.contains_key(long_name.as_str()));
        assert_eq!(
            Some("abc123"),
            reparsed.section_attribute(&long_name, SHA_256_DIGEST)
        );
        Ok(())
    }

    #[test]
    fn test_serialize_section_with_long_value() -> Result<()> {
        let mut manifest = Manifest::default();
        manifest
            .attributes
            .insert(MANIFEST_VERSION.to_string(), "1.0".to_string());
        let long_digest = "a".repeat(200);
        let mut section = AttributeMap::default();
        section.insert(SHA_256_DIGEST.to_string(), long_digest.clone());
        manifest
            .sections
            .insert("com/example/Test.class".to_string(), section);
        let serialized = manifest.to_string();
        for line in serialized.lines() {
            assert!(
                line.len() <= MAX_LINE_BYTES,
                "Line exceeds limit: {} bytes",
                line.len()
            );
        }
        let reparsed = Manifest::from_str(&serialized)?;
        assert_eq!(
            Some(long_digest.as_str()),
            reparsed.section_attribute("com/example/Test.class", SHA_256_DIGEST)
        );
        Ok(())
    }
}
