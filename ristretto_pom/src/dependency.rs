//! Dependency-related types.

use crate::types::DependencyScope;
use serde::{Deserialize, Serialize};

/// Represents dependency management information.
#[non_exhaustive]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct DependencyManagement {
    /// The dependencies.
    #[serde(default)]
    pub dependencies: Dependencies,
}

impl DependencyManagement {
    /// Creates a new `DependencyManagement` with the given dependencies.
    #[must_use]
    pub fn new(dependencies: Dependencies) -> Self {
        Self { dependencies }
    }

    /// Creates a builder for `DependencyManagement`.
    #[must_use]
    pub fn builder() -> DependencyManagementBuilder {
        DependencyManagementBuilder::new()
    }
}

/// Builder for `DependencyManagement`.
#[derive(Debug, Clone, Default)]
pub struct DependencyManagementBuilder {
    dependencies: Vec<Dependency>,
}

impl DependencyManagementBuilder {
    /// Creates a new builder.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a dependency.
    #[must_use]
    pub fn dependency(mut self, dependency: Dependency) -> Self {
        self.dependencies.push(dependency);
        self
    }

    /// Builds the `DependencyManagement`.
    #[must_use]
    pub fn build(self) -> DependencyManagement {
        DependencyManagement {
            dependencies: Dependencies {
                dependencies: self.dependencies,
            },
        }
    }
}

/// Represents a list of dependencies.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct Dependencies {
    /// The dependencies.
    #[serde(rename = "dependency", default)]
    pub dependencies: Vec<Dependency>,
}

impl Dependencies {
    /// Creates an empty `Dependencies`.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a `Dependencies` from a vector of dependencies.
    #[must_use]
    pub fn from_vec(dependencies: Vec<Dependency>) -> Self {
        Self { dependencies }
    }

    /// Returns true if there are no dependencies.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.dependencies.is_empty()
    }

    /// Returns the number of dependencies.
    #[must_use]
    pub fn len(&self) -> usize {
        self.dependencies.len()
    }

    /// Adds a dependency.
    pub fn add(&mut self, dependency: Dependency) {
        self.dependencies.push(dependency);
    }
}

/// Represents a dependency.
#[non_exhaustive]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Dependency {
    /// The group ID of the dependency.
    pub group_id: String,
    /// The artifact ID of the dependency.
    pub artifact_id: String,
    /// The version of the dependency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// The type of the dependency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// The classifier of the dependency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classifier: Option<String>,
    /// The scope of the dependency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<DependencyScope>,
    /// The system path of the dependency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_path: Option<String>,
    /// The exclusions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exclusions: Option<Exclusions>,
    /// Whether the dependency is optional.
    #[serde(default)]
    pub optional: bool,
}

impl Dependency {
    /// Creates a new `Dependency` with the minimum required fields.
    #[must_use]
    pub fn new(group_id: impl Into<String>, artifact_id: impl Into<String>) -> Self {
        Self {
            group_id: group_id.into(),
            artifact_id: artifact_id.into(),
            version: None,
            r#type: None,
            classifier: None,
            scope: None,
            system_path: None,
            exclusions: None,
            optional: false,
        }
    }

    /// Creates a builder for `Dependency`.
    #[must_use]
    pub fn builder(
        group_id: impl Into<String>,
        artifact_id: impl Into<String>,
    ) -> DependencyBuilder {
        DependencyBuilder::new(group_id, artifact_id)
    }
}

/// Builder for `Dependency`.
#[derive(Debug, Clone)]
pub struct DependencyBuilder {
    dependency: Dependency,
}

impl DependencyBuilder {
    /// Creates a new builder with the required fields.
    #[must_use]
    pub fn new(group_id: impl Into<String>, artifact_id: impl Into<String>) -> Self {
        Self {
            dependency: Dependency::new(group_id, artifact_id),
        }
    }

    /// Sets the version.
    #[must_use]
    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.dependency.version = Some(version.into());
        self
    }

    /// Sets the type.
    #[must_use]
    pub fn r#type(mut self, r#type: impl Into<String>) -> Self {
        self.dependency.r#type = Some(r#type.into());
        self
    }

    /// Sets the classifier.
    #[must_use]
    pub fn classifier(mut self, classifier: impl Into<String>) -> Self {
        self.dependency.classifier = Some(classifier.into());
        self
    }

    /// Sets the scope.
    #[must_use]
    pub fn scope(mut self, scope: DependencyScope) -> Self {
        self.dependency.scope = Some(scope);
        self
    }

    /// Sets the system path.
    #[must_use]
    pub fn system_path(mut self, system_path: impl Into<String>) -> Self {
        self.dependency.system_path = Some(system_path.into());
        self
    }

    /// Sets whether the dependency is optional.
    #[must_use]
    pub fn optional(mut self, optional: bool) -> Self {
        self.dependency.optional = optional;
        self
    }

    /// Adds an exclusion.
    #[must_use]
    pub fn exclusion(mut self, exclusion: Exclusion) -> Self {
        if self.dependency.exclusions.is_none() {
            self.dependency.exclusions = Some(Exclusions::default());
        }
        if let Some(ref mut exclusions) = self.dependency.exclusions {
            exclusions.exclusions.push(exclusion);
        }
        self
    }

    /// Builds the `Dependency`.
    #[must_use]
    pub fn build(self) -> Dependency {
        self.dependency
    }
}

/// Represents a list of exclusions.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct Exclusions {
    /// The exclusions.
    #[serde(rename = "exclusion", default)]
    pub exclusions: Vec<Exclusion>,
}

impl Exclusions {
    /// Creates an empty `Exclusions`.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an `Exclusions` from a vector of exclusions.
    #[must_use]
    pub fn from_vec(exclusions: Vec<Exclusion>) -> Self {
        Self { exclusions }
    }
}

/// Represents an exclusion.
#[non_exhaustive]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Exclusion {
    /// The group ID to exclude.
    pub group_id: String,
    /// The artifact ID to exclude.
    pub artifact_id: String,
}

impl Exclusion {
    /// Creates a new `Exclusion`.
    #[must_use]
    pub fn new(group_id: impl Into<String>, artifact_id: impl Into<String>) -> Self {
        Self {
            group_id: group_id.into(),
            artifact_id: artifact_id.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::DependencyScope;

    #[test]
    fn test_dependency_builder() {
        let dep = Dependency::builder("org.junit", "junit")
            .version("4.13")
            .scope(DependencyScope::Test)
            .optional(true)
            .exclusion(Exclusion::new("org.hamcrest", "hamcrest-core"))
            .build();

        assert_eq!(dep.group_id, "org.junit");
        assert_eq!(dep.artifact_id, "junit");
        assert_eq!(dep.version, Some("4.13".to_string()));
        assert_eq!(dep.scope, Some(DependencyScope::Test));
        assert!(dep.optional);
        assert!(dep.exclusions.is_some());
        assert_eq!(dep.exclusions.as_ref().unwrap().exclusions.len(), 1);
    }

    #[test]
    fn test_dependencies_helpers() {
        let mut deps = Dependencies::new();
        assert!(deps.is_empty());

        deps.add(Dependency::new("com.example", "lib"));
        assert_eq!(deps.len(), 1);
    }
}
