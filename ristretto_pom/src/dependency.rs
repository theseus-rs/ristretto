//! Dependency-related types.

use crate::types::DependencyScope;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::borrow::Cow;

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
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Dependency {
    /// The group ID of the dependency.
    pub group_id: String,
    /// The artifact ID of the dependency.
    pub artifact_id: String,
    /// The version of the dependency.
    pub version: Option<String>,
    /// The type of the dependency.
    pub r#type: Option<String>,
    /// The classifier of the dependency.
    pub classifier: Option<String>,
    /// The scope of the dependency.
    pub scope: Option<DependencyScope>,
    /// Original `scope` expression, when it was explicitly present.
    scope_expression: Option<String>,
    /// The system path of the dependency.
    pub system_path: Option<String>,
    /// The exclusions.
    pub exclusions: Option<Exclusions>,
    /// Whether the dependency is optional.
    pub optional: bool,
    /// Whether `optional` was explicitly present in the source model.
    ///
    /// Effective-model merging uses this to distinguish an omitted value from an
    /// explicit `false`.
    pub optional_explicit: bool,
    /// Original `optional` expression, when it was explicitly present.
    optional_expression: Option<String>,
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
            scope_expression: None,
            system_path: None,
            exclusions: None,
            optional: false,
            optional_explicit: false,
            optional_expression: None,
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

    /// Returns the original explicit `optional` value.
    ///
    /// This can contain a Maven property expression such as `${dependency.optional}`.
    #[must_use]
    pub fn optional_expression(&self) -> Option<&str> {
        self.optional_expression.as_deref()
    }

    /// Returns the original explicit `scope` value.
    ///
    /// This can contain a Maven property expression such as `${dependency.scope}`.
    #[must_use]
    pub fn scope_expression(&self) -> Option<&str> {
        self.scope_expression.as_deref()
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
        self.dependency.scope_expression = Some(scope.to_string());
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
        self.dependency.optional_explicit = true;
        self.dependency.optional_expression = Some(optional.to_string());
        self
    }

    /// Sets an interpolatable Maven `optional` expression.
    #[must_use]
    pub fn optional_expression(mut self, expression: impl Into<String>) -> Self {
        let expression = expression.into();
        self.dependency.optional = parse_maven_boolean(&expression);
        self.dependency.optional_explicit = true;
        self.dependency.optional_expression = Some(expression);
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

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DependencyXml {
    group_id: String,
    artifact_id: String,
    version: Option<String>,
    r#type: Option<String>,
    classifier: Option<String>,
    scope: Option<String>,
    system_path: Option<String>,
    exclusions: Option<Exclusions>,
    optional: Option<String>,
}

impl<'de> Deserialize<'de> for Dependency {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let dependency = DependencyXml::deserialize(deserializer)?;
        let scope = dependency
            .scope
            .as_deref()
            .and_then(|scope| scope.parse().ok());
        if let Some(expression) = dependency.scope.as_deref()
            && scope.is_none()
            && !expression.contains("${")
        {
            return Err(serde::de::Error::custom(format!(
                "unknown dependency scope '{expression}'"
            )));
        }
        Ok(Self {
            group_id: dependency.group_id,
            artifact_id: dependency.artifact_id,
            version: dependency.version,
            r#type: dependency.r#type,
            classifier: dependency.classifier,
            scope,
            scope_expression: dependency.scope,
            system_path: dependency.system_path,
            exclusions: dependency.exclusions,
            optional: dependency
                .optional
                .as_deref()
                .is_some_and(parse_maven_boolean),
            optional_explicit: dependency.optional.is_some(),
            optional_expression: dependency.optional,
        })
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct DependencyXmlRef<'a> {
    group_id: &'a str,
    artifact_id: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    r#type: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    classifier: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system_path: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclusions: Option<&'a Exclusions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    optional: Option<Cow<'a, str>>,
}

impl Serialize for Dependency {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        DependencyXmlRef {
            group_id: &self.group_id,
            artifact_id: &self.artifact_id,
            version: self.version.as_deref(),
            r#type: self.r#type.as_deref(),
            classifier: self.classifier.as_deref(),
            scope: self.scope_expression.as_deref().map_or_else(
                || self.scope.map(|scope| Cow::Owned(scope.to_string())),
                |expression| Some(Cow::Borrowed(expression)),
            ),
            system_path: self.system_path.as_deref(),
            exclusions: self.exclusions.as_ref(),
            optional: self.optional_explicit.then(|| {
                self.optional_expression.as_deref().map_or_else(
                    || Cow::Borrowed(if self.optional { "true" } else { "false" }),
                    Cow::Borrowed,
                )
            }),
        }
        .serialize(serializer)
    }
}

fn parse_maven_boolean(value: &str) -> bool {
    value.trim().eq_ignore_ascii_case("true")
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

    #[test]
    fn covers_expression_fallback_serialization_and_repeated_exclusions() {
        let mut dependency = Dependency::builder("com.example", "library")
            .scope(DependencyScope::Runtime)
            .optional(false)
            .exclusion(Exclusion::new("first", "excluded"))
            .exclusion(Exclusion::new("second", "excluded"))
            .build();
        dependency.scope_expression = None;
        dependency.optional_expression = None;
        let xml = quick_xml::se::to_string(&dependency).expect("dependency XML");
        assert!(xml.contains("<scope>runtime</scope>"));
        assert!(xml.contains("<optional>false</optional>"));
        assert_eq!(
            dependency
                .exclusions
                .as_ref()
                .map(|exclusions| exclusions.exclusions.len()),
            Some(2)
        );
    }
}
