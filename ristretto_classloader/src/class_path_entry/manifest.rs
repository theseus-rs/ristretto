use crate::Error::SerdeError;
use crate::Result;
use indexmap::IndexMap;
use serde::de::{self, Visitor};
use serde::ser::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::fmt::Display;
use std::fmt::Write as _;
use std::str::FromStr;

pub const MANIFEST_VERSION: &str = "Manifest-Version";
pub const MAIN_CLASS: &str = "Main-Class";

/// Represents a Jar manifest.
///
/// # References
/// - [JAR File Specification](https://docs.oracle.com/en/java/javase/22/docs/specs/jar/jar.html#jar-manifest)
#[derive(Debug, Default)]
pub struct Manifest {
    pub attributes: IndexMap<String, String>,
    pub sections: IndexMap<String, IndexMap<String, String>>,
}

impl Manifest {
    /// Get attribute.
    pub fn attribute<S: AsRef<str>>(&self, key: S) -> Option<&str> {
        self.attributes.get(key.as_ref()).map(String::as_str)
    }
}

impl FromStr for Manifest {
    type Err = crate::Error;

    /// Parse the manifest from a string.
    ///
    /// # Errors
    ///
    /// if the manifest cannot be deserialized.
    fn from_str(value: &str) -> Result<Self> {
        let manifest = serde_plain::from_str::<Manifest>(value)
            .map_err(|error| SerdeError(error.to_string()))?;
        Ok(manifest)
    }
}

impl Serialize for Manifest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut result = String::new();

        // Serialize main attributes
        for (key, value) in &self.attributes {
            writeln!(result, "{key}: {value}").map_err(Error::custom)?;
        }

        // Serialize sections
        for (section, attributes) in &self.sections {
            writeln!(result, "\nName: {section}").map_err(Error::custom)?;
            for (key, value) in attributes {
                writeln!(result, "{key}: {value}").map_err(Error::custom)?;
            }
        }

        serializer.serialize_str(&result)
    }
}

/// Deserialize the manifest from a string.
impl<'de> Deserialize<'de> for Manifest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ManifestVisitor;

        impl Visitor<'_> for ManifestVisitor {
            type Value = Manifest;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a valid MANIFEST.MF format")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let mut attributes = IndexMap::new();
                let mut sections = IndexMap::new();
                let mut current_section = None;

                for line in v.lines() {
                    let trimmed = line.trim();
                    if trimmed.is_empty() {
                        continue;
                    }
                    let Some((key, value)) = trimmed.split_once(": ") else {
                        continue;
                    };

                    if key == "Name" {
                        current_section = Some(value.to_string());
                        sections.insert(value.to_string(), IndexMap::new());
                    } else if current_section.is_none() {
                        attributes.insert(key.to_string(), value.to_string());
                    } else if let Some(section) = &current_section
                        && let Some(section_map) = sections.get_mut(section)
                    {
                        section_map.insert(key.to_string(), value.to_string());
                    }
                }

                Ok(Manifest {
                    attributes,
                    sections,
                })
            }
        }

        deserializer.deserialize_str(ManifestVisitor)
    }
}

impl Display for Manifest {
    /// Display the manifest.
    ///
    /// # Errors
    ///
    /// if the manifest cannot be serialized.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = serde_plain::to_string(self).map_err(|_| fmt::Error)?;
        write!(f, "{value}")
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
    fn test_serde() -> Result<()> {
        let manifest = Manifest::from_str(EXPECTED)?;
        let serialized = manifest.to_string();
        assert_eq!(EXPECTED, serialized);
        Ok(())
    }
}
