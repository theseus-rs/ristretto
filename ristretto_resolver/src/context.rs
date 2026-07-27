//! External inputs used to build effective Maven models.
//!
//! [`ResolutionContext`] supplies user properties, environment variables, explicitly selected
//! profiles, Java and operating-system facts, and the base directory used by file activation.
//! Pass an explicit context for reproducible resolution, or start with
//! [`ResolutionContext::from_host`] when host-derived activation is desired.

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

/// External inputs used while constructing effective dependency models.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct ResolutionContext {
    /// User properties, with higher precedence than project properties.
    pub properties: BTreeMap<String, String>,
    /// Environment variables exposed as `env.NAME`.
    pub environment: BTreeMap<String, String>,
    /// Explicitly enabled profile identifiers.
    pub active_profiles: BTreeSet<String>,
    /// Explicitly disabled profile identifiers.
    pub inactive_profiles: BTreeSet<String>,
    /// Java version used for JDK profile activation.
    pub java_version: Option<String>,
    /// Operating-system family or name.
    pub os_name: Option<String>,
    /// Operating-system architecture.
    pub os_arch: Option<String>,
    /// Operating-system version.
    pub os_version: Option<String>,
    /// Base directory used for file profile activation on byte-backed projects.
    pub base_directory: Option<PathBuf>,
}

impl ResolutionContext {
    /// Captures the current process environment and operating system.
    ///
    /// The Java version is read from `JAVA_VERSION` when present; this method never launches a
    /// Java process.
    #[must_use]
    pub fn from_host() -> Self {
        Self {
            environment: std::env::vars().collect(),
            java_version: std::env::var("JAVA_VERSION").ok(),
            os_name: Some(std::env::consts::OS.to_string()),
            os_arch: Some(std::env::consts::ARCH.to_string()),
            ..Self::default()
        }
    }

    /// Adds a user property.
    #[must_use]
    pub fn with_property(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.properties.insert(key.into(), value.into());
        self
    }

    /// Explicitly activates a profile.
    #[must_use]
    pub fn activate_profile(mut self, id: impl Into<String>) -> Self {
        self.active_profiles.insert(id.into());
        self
    }

    /// Explicitly deactivates a profile.
    #[must_use]
    pub fn deactivate_profile(mut self, id: impl Into<String>) -> Self {
        self.inactive_profiles.insert(id.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::ResolutionContext;

    #[test]
    fn constructs_host_and_explicit_activation_contexts() {
        let host = ResolutionContext::from_host();
        assert_eq!(host.os_name.as_deref(), Some(std::env::consts::OS));
        assert_eq!(host.os_arch.as_deref(), Some(std::env::consts::ARCH));

        let context = ResolutionContext::default()
            .with_property("revision", "1.0")
            .activate_profile("release")
            .deactivate_profile("development");
        assert_eq!(
            context.properties.get("revision").map(String::as_str),
            Some("1.0")
        );
        assert!(context.active_profiles.contains("release"));
        assert!(context.inactive_profiles.contains("development"));
    }
}
