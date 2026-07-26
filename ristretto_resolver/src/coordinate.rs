//! Validated Maven artifact identities and coordinates.
//!
//! [`ArtifactKey`] identifies a dependency independently of its version and is therefore the unit
//! of conflict mediation. [`ArtifactCoordinate`] adds a concrete version. Both types validate
//! every component before producing repository paths, preventing classifiers, extensions, or
//! identifiers from escaping a repository root.

use crate::{Error, Result};
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;
use std::str::FromStr;

/// The version-independent identity used for dependency conflict mediation.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct ArtifactKey {
    /// Namespace of the artifact.
    pub group_id: String,
    /// Name of the artifact within its group.
    pub artifact_id: String,
    /// Repository file extension, normally `jar`.
    pub extension: String,
    /// Optional attached-artifact classifier.
    pub classifier: Option<String>,
}

impl<'de> Deserialize<'de> for ArtifactKey {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RawArtifactKey {
            group_id: String,
            artifact_id: String,
            extension: String,
            classifier: Option<String>,
        }

        let raw = RawArtifactKey::deserialize(deserializer)?;
        let key = Self {
            group_id: raw.group_id,
            artifact_id: raw.artifact_id,
            extension: raw.extension,
            classifier: raw.classifier,
        };
        key.validate().map_err(serde::de::Error::custom)?;
        Ok(key)
    }
}

impl ArtifactKey {
    /// Creates a key for an ordinary JAR artifact.
    ///
    /// # Errors
    ///
    /// Returns an error if either identifier is unsafe for repository paths.
    pub fn new(group_id: impl Into<String>, artifact_id: impl Into<String>) -> Result<Self> {
        let key = Self {
            group_id: group_id.into(),
            artifact_id: artifact_id.into(),
            extension: "jar".to_string(),
            classifier: None,
        };
        key.validate()?;
        Ok(key)
    }

    /// Uses an alternate repository file extension.
    ///
    /// # Errors
    ///
    /// Returns an error if the extension is unsafe for repository paths.
    pub fn with_extension(mut self, extension: impl Into<String>) -> Result<Self> {
        self.extension = extension.into();
        self.validate()?;
        Ok(self)
    }

    /// Uses an attached-artifact classifier.
    ///
    /// # Errors
    ///
    /// Returns an error if the classifier is unsafe for repository paths.
    pub fn with_classifier(mut self, classifier: impl Into<String>) -> Result<Self> {
        self.classifier = Some(classifier.into());
        self.validate()?;
        Ok(self)
    }

    /// Returns the group/artifact path used by default repositories.
    #[must_use]
    pub fn metadata_path(&self) -> String {
        format!(
            "{}/{}/maven-metadata.xml",
            self.group_id.replace('.', "/"),
            self.artifact_id
        )
    }

    /// Returns a display key without a version.
    #[must_use]
    pub fn conflict_id(&self) -> String {
        match &self.classifier {
            Some(classifier) => format!(
                "{}:{}:{}:{classifier}",
                self.group_id, self.artifact_id, self.extension
            ),
            None => format!("{}:{}:{}", self.group_id, self.artifact_id, self.extension),
        }
    }

    pub(crate) fn validate(&self) -> Result<()> {
        if self.group_id.is_empty()
            || self
                .group_id
                .split('.')
                .any(|segment| !safe_segment(segment))
        {
            return Err(Error::InvalidCoordinate(self.group_id.clone()));
        }
        for segment in [
            Some(self.artifact_id.as_str()),
            Some(self.extension.as_str()),
            self.classifier.as_deref(),
        ]
        .into_iter()
        .flatten()
        {
            if !safe_segment(segment) {
                return Err(Error::InvalidCoordinate(segment.to_string()));
            }
        }
        Ok(())
    }
}

impl fmt::Display for ArtifactKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.classifier {
            Some(classifier) => write!(
                formatter,
                "{}:{}:{}:{classifier}",
                self.group_id, self.artifact_id, self.extension
            ),
            None if self.extension == "jar" => {
                write!(formatter, "{}:{}", self.group_id, self.artifact_id)
            }
            None => write!(
                formatter,
                "{}:{}:{}",
                self.group_id, self.artifact_id, self.extension
            ),
        }
    }
}

/// A fully addressable artifact coordinate.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct ArtifactCoordinate {
    /// Version-independent artifact identity.
    pub key: ArtifactKey,
    /// Requested or resolved version.
    pub version: String,
}

impl<'de> Deserialize<'de> for ArtifactCoordinate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RawArtifactCoordinate {
            key: ArtifactKey,
            version: String,
        }

        let raw = RawArtifactCoordinate::deserialize(deserializer)?;
        Self::from_key(raw.key, raw.version).map_err(serde::de::Error::custom)
    }
}

impl ArtifactCoordinate {
    /// Creates an ordinary JAR coordinate.
    ///
    /// # Errors
    ///
    /// Returns an error if any coordinate component is unsafe.
    pub fn new(
        group_id: impl Into<String>,
        artifact_id: impl Into<String>,
        version: impl Into<String>,
    ) -> Result<Self> {
        Self::from_key(ArtifactKey::new(group_id, artifact_id)?, version)
    }

    /// Adds a version to an artifact key.
    ///
    /// # Errors
    ///
    /// Returns an error if the version is unsafe for repository paths.
    pub fn from_key(key: ArtifactKey, version: impl Into<String>) -> Result<Self> {
        let version = version.into();
        key.validate()?;
        if !safe_segment(&version) {
            return Err(Error::InvalidCoordinate(version));
        }
        Ok(Self { key, version })
    }

    /// Returns the repository path of the POM descriptor.
    #[must_use]
    pub fn pom_path(&self) -> String {
        self.path_for("pom", None, &self.version)
    }

    /// Returns the POM path using a timestamped snapshot filename.
    #[must_use]
    pub fn pom_path_with_version(&self, file_version: &str) -> String {
        self.path_for("pom", None, file_version)
    }

    /// Returns the path of this coordinate's selected artifact.
    #[must_use]
    pub fn artifact_path(&self) -> String {
        self.path_for(
            &self.key.extension,
            self.key.classifier.as_deref(),
            &self.version,
        )
    }

    /// Returns an artifact path using a timestamped snapshot filename.
    #[must_use]
    pub fn artifact_path_with_version(&self, file_version: &str) -> String {
        self.path_for(
            &self.key.extension,
            self.key.classifier.as_deref(),
            file_version,
        )
    }

    /// Returns the version-level snapshot metadata path.
    #[must_use]
    pub fn snapshot_metadata_path(&self) -> String {
        format!(
            "{}/{}/{}/maven-metadata.xml",
            self.key.group_id.replace('.', "/"),
            self.key.artifact_id,
            self.version
        )
    }

    fn path_for(&self, extension: &str, classifier: Option<&str>, file_version: &str) -> String {
        let classifier = classifier.map_or_else(String::new, |value| format!("-{value}"));
        format!(
            "{}/{}/{}/{}-{}{classifier}.{extension}",
            self.key.group_id.replace('.', "/"),
            self.key.artifact_id,
            self.version,
            self.key.artifact_id,
            file_version
        )
    }
}

impl fmt::Display for ArtifactCoordinate {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.key.classifier {
            Some(classifier) => write!(
                formatter,
                "{}:{}:{}:{classifier}:{}",
                self.key.group_id, self.key.artifact_id, self.key.extension, self.version
            ),
            None if self.key.extension == "jar" => write!(
                formatter,
                "{}:{}:{}",
                self.key.group_id, self.key.artifact_id, self.version
            ),
            None => write!(
                formatter,
                "{}:{}:{}:{}",
                self.key.group_id, self.key.artifact_id, self.key.extension, self.version
            ),
        }
    }
}

impl FromStr for ArtifactCoordinate {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self> {
        let parts = value.split(':').collect::<Vec<_>>();
        match parts.as_slice() {
            [group, artifact, version] => Self::new(*group, *artifact, *version),
            [group, artifact, extension, version] => Self::from_key(
                ArtifactKey::new(*group, *artifact)?.with_extension(*extension)?,
                *version,
            ),
            [group, artifact, extension, classifier, version] => Self::from_key(
                ArtifactKey::new(*group, *artifact)?
                    .with_extension(*extension)?
                    .with_classifier(*classifier)?,
                *version,
            ),
            _ => Err(Error::InvalidCoordinate(value.to_string())),
        }
    }
}

fn safe_segment(value: &str) -> bool {
    !value.is_empty()
        && value.is_ascii()
        && value != "."
        && value != ".."
        && !value.contains(['/', '\\', '%', '?', '#', ':'])
        && !value
            .chars()
            .any(|character| character.is_whitespace() || character.is_control())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[expect(
        clippy::panic_in_result_fn,
        reason = "assertions provide clearer diagnostics in unit tests"
    )]
    fn coordinate_round_trip() -> Result<()> {
        for value in [
            "org.example:demo:1.0",
            "org.example:demo:zip:1.0",
            "org.example:demo:jar:tests:1.0",
        ] {
            assert_eq!(value.parse::<ArtifactCoordinate>()?.to_string(), value);
        }
        Ok(())
    }

    #[test]
    fn rejects_unsafe_paths() {
        assert!(ArtifactCoordinate::new("org.example", "../demo", "1.0").is_err());
        assert!(ArtifactCoordinate::new("org..example", "demo", "1.0").is_err());
        assert!(ArtifactCoordinate::new("org.example", "demo", "../1.0").is_err());
        for value in [
            "%2e%2e",
            "%2E%2E",
            "demo%2fescape",
            "demo%5cescape",
            "demo?query",
            "demo#fragment",
            "https:escape",
            "demo\0escape",
            "démø",
        ] {
            assert!(ArtifactCoordinate::new("org.example", value, "1.0").is_err());
            assert!(ArtifactCoordinate::new("org.example", "demo", value).is_err());
        }
    }

    #[test]
    fn serde_rejects_coordinates_that_bypass_constructors() {
        assert!(
            serde_json::from_str::<ArtifactKey>(
                r#"{"group_id":"org.example","artifact_id":"%2e%2e","extension":"jar","classifier":null}"#
            )
            .is_err()
        );
        assert!(
            serde_json::from_str::<ArtifactCoordinate>(
                r#"{"key":{"group_id":"org.example","artifact_id":"demo","extension":"jar","classifier":null},"version":"../1"}"#
            )
            .is_err()
        );
    }

    #[test]
    #[expect(
        clippy::panic_in_result_fn,
        reason = "assertions provide clearer diagnostics in unit tests"
    )]
    fn builds_all_repository_paths_and_display_forms() -> Result<()> {
        let key = ArtifactKey::new("org.example", "demo")?
            .with_extension("zip")?
            .with_classifier("tests")?;
        assert_eq!(key.metadata_path(), "org/example/demo/maven-metadata.xml");
        assert_eq!(key.conflict_id(), "org.example:demo:zip:tests");
        assert_eq!(key.to_string(), "org.example:demo:zip:tests");

        let coordinate = ArtifactCoordinate::from_key(key, "1.0-SNAPSHOT")?;
        assert_eq!(
            coordinate.pom_path(),
            "org/example/demo/1.0-SNAPSHOT/demo-1.0-SNAPSHOT.pom"
        );
        assert_eq!(
            coordinate.pom_path_with_version("1.0-20260725.000000-1"),
            "org/example/demo/1.0-SNAPSHOT/demo-1.0-20260725.000000-1.pom"
        );
        assert_eq!(
            coordinate.artifact_path(),
            "org/example/demo/1.0-SNAPSHOT/demo-1.0-SNAPSHOT-tests.zip"
        );
        assert_eq!(
            coordinate.artifact_path_with_version("1.0-20260725.000000-1"),
            "org/example/demo/1.0-SNAPSHOT/demo-1.0-20260725.000000-1-tests.zip"
        );
        assert_eq!(
            coordinate.snapshot_metadata_path(),
            "org/example/demo/1.0-SNAPSHOT/maven-metadata.xml"
        );

        let plain = ArtifactKey::new("org.example", "demo")?;
        assert_eq!(plain.conflict_id(), "org.example:demo:jar");
        assert_eq!(plain.to_string(), "org.example:demo");
        assert_eq!(
            plain.clone().with_extension("zip")?.to_string(),
            "org.example:demo:zip"
        );
        assert_eq!(
            ArtifactCoordinate::from_key(plain.with_extension("pom")?, "1.0")?.to_string(),
            "org.example:demo:pom:1.0"
        );
        Ok(())
    }

    #[test]
    fn rejects_malformed_coordinate_forms_and_components() {
        for value in ["", "a:b", "a:b:c:d:e:f"] {
            assert!(value.parse::<ArtifactCoordinate>().is_err());
        }
        for (group, artifact, version) in [
            ("", "demo", "1"),
            (".", "demo", "1"),
            ("org.éxample", "demo", "1"),
            ("org.example", "", "1"),
            ("org.example", "demo", ""),
            ("org.example", "demo", "1:2"),
        ] {
            assert!(ArtifactCoordinate::new(group, artifact, version).is_err());
        }
        assert!(
            ArtifactKey::new("org.example", "demo")
                .and_then(|key| key.with_extension(""))
                .is_err()
        );
        assert!(
            ArtifactKey::new("org.example", "demo")
                .and_then(|key| key.with_classifier("bad value"))
                .is_err()
        );
    }
}
