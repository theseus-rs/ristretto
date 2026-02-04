//! Profile types.

use crate::build::Build;
use crate::dependency::{Dependencies, DependencyManagement};
use crate::distribution::DistributionManagement;
use crate::reporting::Reporting;
use crate::repository::Repositories;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Represents a list of modules.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct Modules {
    /// The modules (sub-projects).
    #[serde(rename = "module", default)]
    pub modules: Vec<String>,
}

impl Modules {
    /// Creates an empty `Modules`.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a `Modules` from a vector of module names.
    #[must_use]
    pub fn from_vec(modules: Vec<String>) -> Self {
        Self { modules }
    }

    /// Adds a module.
    pub fn add(&mut self, module: impl Into<String>) {
        self.modules.push(module.into());
    }
}

/// Represents a list of profiles.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct Profiles {
    /// The profiles.
    #[serde(rename = "profile", default)]
    pub profiles: Vec<Profile>,
}

impl Profiles {
    /// Creates an empty `Profiles`.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a `Profiles` from a vector of profiles.
    #[must_use]
    pub fn from_vec(profiles: Vec<Profile>) -> Self {
        Self { profiles }
    }

    /// Adds a profile.
    pub fn add(&mut self, profile: Profile) {
        self.profiles.push(profile);
    }
}

/// Represents a profile.
#[non_exhaustive]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    /// The ID of the profile.
    pub id: String,
    /// The activation conditions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activation: Option<Activation>,
    /// The build configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build: Option<Build>,
    /// The modules.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modules: Option<Modules>,
    /// The repositories.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repositories: Option<Repositories>,
    /// The plugin repositories.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plugin_repositories: Option<Repositories>,
    /// The dependencies.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<Dependencies>,
    /// The dependency management.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dependency_management: Option<DependencyManagement>,
    /// The distribution management.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution_management: Option<DistributionManagement>,
    /// The properties.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub properties: BTreeMap<String, String>,
    /// The reporting configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reporting: Option<Reporting>,
}

impl Profile {
    /// Creates a new `Profile` with the given ID.
    #[must_use]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            activation: None,
            build: None,
            modules: None,
            repositories: None,
            plugin_repositories: None,
            dependencies: None,
            dependency_management: None,
            distribution_management: None,
            properties: BTreeMap::new(),
            reporting: None,
        }
    }

    /// Creates a builder for `Profile`.
    #[must_use]
    pub fn builder(id: impl Into<String>) -> ProfileBuilder {
        ProfileBuilder::new(id)
    }
}

/// Builder for `Profile`.
#[derive(Debug, Clone)]
pub struct ProfileBuilder {
    profile: Profile,
}

impl ProfileBuilder {
    /// Creates a new builder with the required ID.
    #[must_use]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            profile: Profile::new(id),
        }
    }

    /// Sets the activation.
    #[must_use]
    pub fn activation(mut self, activation: Activation) -> Self {
        self.profile.activation = Some(activation);
        self
    }

    /// Sets the build configuration.
    #[must_use]
    pub fn build_config(mut self, build: Build) -> Self {
        self.profile.build = Some(build);
        self
    }

    /// Sets the dependencies.
    #[must_use]
    pub fn dependencies(mut self, deps: Dependencies) -> Self {
        self.profile.dependencies = Some(deps);
        self
    }

    /// Adds a property.
    #[must_use]
    pub fn property(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.profile.properties.insert(key.into(), value.into());
        self
    }

    /// Sets the repositories.
    #[must_use]
    pub fn repositories(mut self, repos: Repositories) -> Self {
        self.profile.repositories = Some(repos);
        self
    }

    /// Builds the `Profile`.
    #[must_use]
    pub fn build(self) -> Profile {
        self.profile
    }
}

/// Represents activation conditions.
#[non_exhaustive]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Activation {
    /// Whether the profile is active by default.
    #[serde(default)]
    pub active_by_default: bool,
    /// The JDK version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jdk: Option<String>,
    /// The OS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<ActivationOs>,
    /// The property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<ActivationProperty>,
    /// The file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<ActivationFile>,
}

impl Activation {
    /// Creates an activation that is active by default.
    #[must_use]
    pub fn active_by_default() -> Self {
        Self {
            active_by_default: true,
            jdk: None,
            os: None,
            property: None,
            file: None,
        }
    }

    /// Creates an activation that activates on a specific JDK version.
    #[must_use]
    pub fn jdk(version: impl Into<String>) -> Self {
        Self {
            active_by_default: false,
            jdk: Some(version.into()),
            os: None,
            property: None,
            file: None,
        }
    }

    /// Creates an activation that activates on a specific property.
    #[must_use]
    pub fn property(name: impl Into<String>, value: Option<String>) -> Self {
        Self {
            active_by_default: false,
            jdk: None,
            os: None,
            property: Some(ActivationProperty {
                name: name.into(),
                value,
            }),
            file: None,
        }
    }

    /// Creates a builder for `Activation`.
    #[must_use]
    pub fn builder() -> ActivationBuilder {
        ActivationBuilder::new()
    }
}

/// Builder for `Activation`.
#[derive(Debug, Clone, Default)]
pub struct ActivationBuilder {
    activation: Activation,
}

impl ActivationBuilder {
    /// Creates a new builder.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets whether the profile is active by default.
    #[must_use]
    pub fn active_by_default(mut self, active: bool) -> Self {
        self.activation.active_by_default = active;
        self
    }

    /// Sets the JDK version.
    #[must_use]
    pub fn jdk(mut self, version: impl Into<String>) -> Self {
        self.activation.jdk = Some(version.into());
        self
    }

    /// Sets the OS activation.
    #[must_use]
    pub fn os(mut self, os: ActivationOs) -> Self {
        self.activation.os = Some(os);
        self
    }

    /// Sets the property activation.
    #[must_use]
    pub fn property(mut self, name: impl Into<String>, value: Option<String>) -> Self {
        self.activation.property = Some(ActivationProperty {
            name: name.into(),
            value,
        });
        self
    }

    /// Sets the file activation.
    #[must_use]
    pub fn file(mut self, file: ActivationFile) -> Self {
        self.activation.file = Some(file);
        self
    }

    /// Builds the `Activation`.
    #[must_use]
    pub fn build(self) -> Activation {
        self.activation
    }
}

/// Represents OS activation conditions.
#[non_exhaustive]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ActivationOs {
    /// The name of the OS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The family of the OS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    /// The architecture of the OS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arch: Option<String>,
    /// The version of the OS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

impl ActivationOs {
    /// Creates an activation for a specific OS family.
    #[must_use]
    pub fn family(family: impl Into<String>) -> Self {
        Self {
            name: None,
            family: Some(family.into()),
            arch: None,
            version: None,
        }
    }

    /// Creates an activation for Windows.
    #[must_use]
    pub fn windows() -> Self {
        Self::family("windows")
    }

    /// Creates an activation for Unix.
    #[must_use]
    pub fn unix() -> Self {
        Self::family("unix")
    }

    /// Creates an activation for macOS.
    #[must_use]
    pub fn mac() -> Self {
        Self::family("mac")
    }

    /// Creates a builder for `ActivationOs`.
    #[must_use]
    pub fn builder() -> ActivationOsBuilder {
        ActivationOsBuilder::new()
    }
}

/// Builder for `ActivationOs`.
#[derive(Debug, Clone, Default)]
pub struct ActivationOsBuilder {
    os: ActivationOs,
}

impl ActivationOsBuilder {
    /// Creates a new builder.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the name.
    #[must_use]
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.os.name = Some(name.into());
        self
    }

    /// Sets the family.
    #[must_use]
    pub fn family(mut self, family: impl Into<String>) -> Self {
        self.os.family = Some(family.into());
        self
    }

    /// Sets the architecture.
    #[must_use]
    pub fn arch(mut self, arch: impl Into<String>) -> Self {
        self.os.arch = Some(arch.into());
        self
    }

    /// Sets the version.
    #[must_use]
    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.os.version = Some(version.into());
        self
    }

    /// Builds the `ActivationOs`.
    #[must_use]
    pub fn build(self) -> ActivationOs {
        self.os
    }
}

/// Represents property activation conditions.
#[non_exhaustive]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ActivationProperty {
    /// The name of the property.
    pub name: String,
    /// The value of the property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl ActivationProperty {
    /// Creates a new `ActivationProperty`.
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: None,
        }
    }

    /// Creates a new `ActivationProperty` with a value.
    #[must_use]
    pub fn with_value(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: Some(value.into()),
        }
    }
}

/// Represents file activation conditions.
#[non_exhaustive]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ActivationFile {
    /// The file must be missing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing: Option<String>,
    /// The file must exist.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exists: Option<String>,
}

impl ActivationFile {
    /// Creates an activation that requires a file to exist.
    #[must_use]
    pub fn exists(path: impl Into<String>) -> Self {
        Self {
            missing: None,
            exists: Some(path.into()),
        }
    }

    /// Creates an activation that requires a file to be missing.
    #[must_use]
    pub fn missing(path: impl Into<String>) -> Self {
        Self {
            missing: Some(path.into()),
            exists: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile_builder() {
        let profile = Profile::builder("release")
            .activation(Activation::property(
                "performRelease",
                Some("true".to_string()),
            ))
            .property("maven.test.skip", "true")
            .build();

        assert_eq!(profile.id, "release");
        assert!(profile.activation.is_some());
        assert!(profile.properties.contains_key("maven.test.skip"));
    }

    #[test]
    fn test_activation_helpers() {
        let default = Activation::active_by_default();
        assert!(default.active_by_default);

        let jdk = Activation::jdk("17");
        assert_eq!(jdk.jdk, Some("17".to_string()));

        let prop = Activation::property("env", Some("prod".to_string()));
        assert!(prop.property.is_some());
    }

    #[test]
    fn test_activation_os() {
        let windows = ActivationOs::windows();
        assert_eq!(windows.family, Some("windows".to_string()));

        let unix = ActivationOs::unix();
        assert_eq!(unix.family, Some("unix".to_string()));
    }

    #[test]
    fn test_activation_file() {
        let exists = ActivationFile::exists("build.properties");
        assert_eq!(exists.exists, Some("build.properties".to_string()));

        let missing = ActivationFile::missing("skip.txt");
        assert_eq!(missing.missing, Some("skip.txt".to_string()));
    }
}
