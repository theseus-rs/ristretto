//! POM version type.

use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

use crate::{Error, Result};

/// Version of the Maven POM model in `major.minor.patch` form.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PomVersion {
    /// The major version.
    pub major: u32,
    /// The minor version.
    pub minor: u32,
    /// The patch version.
    pub patch: u32,
}

impl PomVersion {
    /// Maven POM model version 3.0.0.
    pub const MODEL_3_0_0: Self = Self::new(3, 0, 0);
    /// Maven POM model version 4.0.0.
    pub const MODEL_4_0_0: Self = Self::new(4, 0, 0);
    /// Maven POM model version 4.1.0.
    pub const MODEL_4_1_0: Self = Self::new(4, 1, 0);
    /// The Default Maven POM model version used by this crate.
    pub const DEFAULT_MODEL: Self = Self::MODEL_4_0_0;
    /// Maven POM model versions accepted by project syntax validation.
    pub const SUPPORTED_MODELS: [Self; 3] =
        [Self::MODEL_3_0_0, Self::MODEL_4_0_0, Self::MODEL_4_1_0];

    /// Creates a new `PomVersion`.
    #[must_use]
    pub const fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Returns `true` when this is a supported Maven POM model version.
    #[must_use]
    pub fn is_supported_model(self) -> bool {
        Self::SUPPORTED_MODELS.contains(&self)
    }

    fn parse_component(version: &str, component: Option<&str>) -> Result<u32> {
        let component = component.ok_or_else(|| Error::InvalidPomVersion(version.to_string()))?;
        if component.is_empty() || !component.bytes().all(|byte| byte.is_ascii_digit()) {
            return Err(Error::InvalidPomVersion(version.to_string()));
        }
        component
            .parse()
            .map_err(|_| Error::InvalidPomVersion(version.to_string()))
    }
}

impl Default for PomVersion {
    fn default() -> Self {
        Self::DEFAULT_MODEL
    }
}

impl fmt::Display for PomVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl FromStr for PomVersion {
    type Err = Error;

    fn from_str(version: &str) -> Result<Self> {
        let mut components = version.split('.');
        let major = Self::parse_component(version, components.next())?;
        let minor = Self::parse_component(version, components.next())?;
        let patch = Self::parse_component(version, components.next())?;
        if components.next().is_some() {
            return Err(Error::InvalidPomVersion(version.to_string()));
        }
        Ok(Self::new(major, minor, patch))
    }
}

impl Serialize for PomVersion {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for PomVersion {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let version = String::deserialize(deserializer)?;
        version.parse().map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    struct VersionWrapper {
        #[serde(rename = "modelVersion")]
        version: PomVersion,
    }

    #[test]
    fn parses_major_minor_patch() -> Result<()> {
        let version = "4.0.0".parse::<PomVersion>()?;

        assert_eq!(version.major, 4);
        assert_eq!(version.minor, 0);
        assert_eq!(version.patch, 0);
        assert_eq!(version.to_string(), "4.0.0");
        Ok(())
    }

    #[test]
    fn identifies_supported_model_versions() {
        assert!(PomVersion::MODEL_3_0_0.is_supported_model());
        assert!(PomVersion::MODEL_4_0_0.is_supported_model());
        assert!(PomVersion::MODEL_4_1_0.is_supported_model());
        assert!(!PomVersion::new(5, 0, 0).is_supported_model());
    }

    #[test]
    fn rejects_invalid_versions() {
        for version in ["4.0", "4.0.0.0", "4.x.0", "4..0", "4294967296.0.0", ""] {
            assert!(version.parse::<PomVersion>().is_err());
        }
    }

    #[test]
    fn serializes_as_text() -> Result<()> {
        let wrapper = VersionWrapper {
            version: PomVersion::new(1, 2, 3),
        };
        let xml = quick_xml::se::to_string(&wrapper)?;

        assert!(xml.contains("<modelVersion>1.2.3</modelVersion>"));
        Ok(())
    }

    #[test]
    fn deserializes_from_text() {
        let wrapper: VersionWrapper = quick_xml::de::from_str(
            "<VersionWrapper><modelVersion>1.2.3</modelVersion></VersionWrapper>",
        )
        .expect("version wrapper should deserialize");
        let version = wrapper.version;

        assert_eq!(version, PomVersion::new(1, 2, 3));
        assert_eq!(version.to_string(), "1.2.3");
    }
}
