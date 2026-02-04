//! License types.

use serde::{Deserialize, Serialize};

/// Represents a list of licenses.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct Licenses {
    /// The licenses.
    #[serde(rename = "license", default)]
    pub licenses: Vec<License>,
}

impl Licenses {
    /// Creates an empty `Licenses`.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a `Licenses` from a vector of licenses.
    #[must_use]
    pub fn from_vec(licenses: Vec<License>) -> Self {
        Self { licenses }
    }

    /// Adds a license.
    pub fn add(&mut self, license: License) {
        self.licenses.push(license);
    }
}

/// Represents a license.
#[non_exhaustive]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct License {
    /// The name of the license.
    pub name: String,
    /// The URL of the license.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// The distribution of the license.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution: Option<String>,
    /// Comments about the license.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
}

impl License {
    /// Creates a new `License` with the given name.
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            url: None,
            distribution: None,
            comments: None,
        }
    }

    /// Creates a builder for `License`.
    #[must_use]
    pub fn builder(name: impl Into<String>) -> LicenseBuilder {
        LicenseBuilder::new(name)
    }

    /// Creates a common MIT license.
    #[must_use]
    pub fn mit() -> Self {
        Self {
            name: "MIT License".to_string(),
            url: Some("https://opensource.org/licenses/MIT".to_string()),
            distribution: Some("repo".to_string()),
            comments: None,
        }
    }

    /// Creates a common Apache 2.0 license.
    #[must_use]
    pub fn apache2() -> Self {
        Self {
            name: "Apache License, Version 2.0".to_string(),
            url: Some("https://www.apache.org/licenses/LICENSE-2.0".to_string()),
            distribution: Some("repo".to_string()),
            comments: None,
        }
    }
}

/// Builder for `License`.
#[derive(Debug, Clone)]
pub struct LicenseBuilder {
    license: License,
}

impl LicenseBuilder {
    /// Creates a new builder with the required name.
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            license: License::new(name),
        }
    }

    /// Sets the URL.
    #[must_use]
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.license.url = Some(url.into());
        self
    }

    /// Sets the distribution.
    #[must_use]
    pub fn distribution(mut self, distribution: impl Into<String>) -> Self {
        self.license.distribution = Some(distribution.into());
        self
    }

    /// Sets the comments.
    #[must_use]
    pub fn comments(mut self, comments: impl Into<String>) -> Self {
        self.license.comments = Some(comments.into());
        self
    }

    /// Builds the `License`.
    #[must_use]
    pub fn build(self) -> License {
        self.license
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_license_builder() {
        let license = License::builder("GPL-3.0")
            .url("https://www.gnu.org/licenses/gpl-3.0.html")
            .distribution("repo")
            .build();

        assert_eq!(license.name, "GPL-3.0");
        assert_eq!(
            license.url,
            Some("https://www.gnu.org/licenses/gpl-3.0.html".to_string())
        );
    }

    #[test]
    fn test_common_licenses() {
        let mit = License::mit();
        assert_eq!(mit.name, "MIT License");

        let apache = License::apache2();
        assert_eq!(apache.name, "Apache License, Version 2.0");
    }
}
