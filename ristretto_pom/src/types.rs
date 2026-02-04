//! Strongly typed enums and first-class types for Maven coordinates.
//!
//! This module provides type-safe representations for common Maven POM values
//! that are effectively enums, reducing stringly-typed logic and improving
//! compile-time safety.

use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

use crate::{Error, Result};

/// Dependency scope in Maven.
///
/// Defines the classpath(s) that a dependency is available on and controls
/// transitive dependency behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum DependencyScope {
    /// Compile scope (default) - available in all classpaths.
    #[default]
    Compile,
    /// Provided scope - expected to be provided by the JDK or container at runtime.
    Provided,
    /// Runtime scope - not needed for compilation, but needed for execution.
    Runtime,
    /// Test scope - only available for test compilation and execution.
    Test,
    /// System scope - similar to provided, but you must specify the JAR path explicitly.
    System,
    /// Import scope - only used on dependencies of type pom in dependencyManagement.
    Import,
}

impl fmt::Display for DependencyScope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Compile => write!(f, "compile"),
            Self::Provided => write!(f, "provided"),
            Self::Runtime => write!(f, "runtime"),
            Self::Test => write!(f, "test"),
            Self::System => write!(f, "system"),
            Self::Import => write!(f, "import"),
        }
    }
}

impl FromStr for DependencyScope {
    type Err = Error;

    fn from_str(scope: &str) -> Result<Self> {
        match scope.to_lowercase().as_str() {
            "compile" => Ok(Self::Compile),
            "provided" => Ok(Self::Provided),
            "runtime" => Ok(Self::Runtime),
            "test" => Ok(Self::Test),
            "system" => Ok(Self::System),
            "import" => Ok(Self::Import),
            _ => Err(Error::InvalidValue(format!(
                "Unknown dependency scope: {scope}"
            ))),
        }
    }
}

/// Packaging type for a Maven project.
///
/// Defines the artifact type that will be produced by the build.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum Packaging {
    /// JAR packaging (default).
    #[default]
    Jar,
    /// WAR packaging (web application).
    War,
    /// EAR packaging (enterprise application).
    Ear,
    /// POM packaging (parent/BOM projects).
    Pom,
    /// Maven plugin packaging.
    MavenPlugin,
    /// EJB packaging.
    Ejb,
    /// RAR packaging (resource adapter).
    Rar,
    /// Custom or unknown packaging type.
    Other(String),
}

impl fmt::Display for Packaging {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Jar => write!(f, "jar"),
            Self::War => write!(f, "war"),
            Self::Ear => write!(f, "ear"),
            Self::Pom => write!(f, "pom"),
            Self::MavenPlugin => write!(f, "maven-plugin"),
            Self::Ejb => write!(f, "ejb"),
            Self::Rar => write!(f, "rar"),
            Self::Other(s) => write!(f, "{s}"),
        }
    }
}

impl Serialize for Packaging {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Packaging {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let packaging = String::deserialize(deserializer)?;
        Ok(match packaging.to_lowercase().as_str() {
            "jar" => Self::Jar,
            "war" => Self::War,
            "ear" => Self::Ear,
            "pom" => Self::Pom,
            "maven-plugin" => Self::MavenPlugin,
            "ejb" => Self::Ejb,
            "rar" => Self::Rar,
            _ => Self::Other(packaging),
        })
    }
}

impl FromStr for Packaging {
    type Err = Error;

    fn from_str(packaging: &str) -> Result<Self> {
        match packaging.to_lowercase().as_str() {
            "jar" => Ok(Self::Jar),
            "war" => Ok(Self::War),
            "ear" => Ok(Self::Ear),
            "pom" => Ok(Self::Pom),
            "maven-plugin" => Ok(Self::MavenPlugin),
            "ejb" => Ok(Self::Ejb),
            "rar" => Ok(Self::Rar),
            other => Ok(Self::Other(other.to_string())),
        }
    }
}

/// Repository layout type.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum RepositoryLayout {
    /// Default Maven 2/3 layout.
    #[default]
    Default,
    /// Legacy Maven 1 layout.
    Legacy,
    /// Custom layout.
    Other(String),
}

impl fmt::Display for RepositoryLayout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Default => write!(f, "default"),
            Self::Legacy => write!(f, "legacy"),
            Self::Other(s) => write!(f, "{s}"),
        }
    }
}

impl Serialize for RepositoryLayout {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for RepositoryLayout {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let layout = String::deserialize(deserializer)?;
        Ok(match layout.to_lowercase().as_str() {
            "default" => Self::Default,
            "legacy" => Self::Legacy,
            _ => Self::Other(layout),
        })
    }
}

/// Repository update policy.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum UpdatePolicy {
    /// Always check for updates.
    Always,
    /// Check daily (default).
    #[default]
    Daily,
    /// Never check for updates.
    Never,
    /// Check at specified interval (e.g., "interval:60" for 60 minutes).
    Interval(String),
}

impl fmt::Display for UpdatePolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Always => write!(f, "always"),
            Self::Daily => write!(f, "daily"),
            Self::Never => write!(f, "never"),
            Self::Interval(s) => write!(f, "{s}"),
        }
    }
}

impl Serialize for UpdatePolicy {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for UpdatePolicy {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let policy = String::deserialize(deserializer)?;
        Ok(match policy.to_lowercase().as_str() {
            "always" => Self::Always,
            "daily" => Self::Daily,
            "never" => Self::Never,
            _ => Self::Interval(policy),
        })
    }
}

/// Repository checksum policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ChecksumPolicy {
    /// Ignore checksum failures.
    Ignore,
    /// Fail on checksum mismatch.
    Fail,
    /// Warn on checksum mismatch (default).
    #[default]
    Warn,
}

impl fmt::Display for ChecksumPolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ignore => write!(f, "ignore"),
            Self::Fail => write!(f, "fail"),
            Self::Warn => write!(f, "warn"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dependency_scope_display() {
        assert_eq!(DependencyScope::Compile.to_string(), "compile");
        assert_eq!(DependencyScope::Test.to_string(), "test");
        assert_eq!(DependencyScope::Import.to_string(), "import");
    }

    #[test]
    fn test_dependency_scope_from_str() {
        assert_eq!(
            "compile".parse::<DependencyScope>().unwrap(),
            DependencyScope::Compile
        );
        assert_eq!(
            "RUNTIME".parse::<DependencyScope>().unwrap(),
            DependencyScope::Runtime
        );
    }

    #[test]
    fn test_packaging_from_str() {
        assert_eq!("jar".parse::<Packaging>().unwrap(), Packaging::Jar);
        assert_eq!("pom".parse::<Packaging>().unwrap(), Packaging::Pom);
        assert_eq!(
            "custom".parse::<Packaging>().unwrap(),
            Packaging::Other("custom".to_string())
        );
    }
}
