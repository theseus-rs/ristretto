//! Build-related types.

use crate::configuration::Configuration;
use crate::dependency::Dependencies;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Represents build configuration.
#[non_exhaustive]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Build {
    /// Maven 4 source directories and their compilation/resource semantics.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sources: Option<Sources>,
    /// The source directory.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_directory: Option<String>,
    /// The script source directory.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_source_directory: Option<String>,
    /// The test source directory.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_source_directory: Option<String>,
    /// The output directory.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_directory: Option<String>,
    /// The test output directory.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_output_directory: Option<String>,
    /// The extensions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Extensions>,
    /// The default goal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_goal: Option<String>,
    /// The resources.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<Resources>,
    /// The test resources.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub test_resources: Option<TestResources>,
    /// The directory.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory: Option<String>,
    /// The final name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_name: Option<String>,
    /// The filters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filters: Option<Filters>,
    /// The plugin management.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugin_management: Option<PluginManagement>,
    /// The plugins.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plugins: Option<BuildPlugins>,
}

/// Represents Maven 4 source declarations.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct Sources {
    /// The source declarations.
    #[serde(rename = "source", default)]
    pub sources: Vec<Source>,
}

/// A Maven 4 source directory.
#[non_exhaustive]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Source {
    /// Source usage, normally `main` or `test`.
    #[serde(default = "default_source_scope")]
    pub scope: String,
    /// Source language, normally `java` or `resources`.
    #[serde(default = "default_source_language")]
    pub lang: String,
    /// Optional module name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module: Option<String>,
    /// Optional target platform version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_version: Option<String>,
    /// Optional explicit output path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_path: Option<String>,
    /// Whether string filtering is enabled.
    #[serde(default)]
    pub string_filtering: bool,
    /// Whether this source is enabled.
    #[serde(default = "source_enabled_by_default")]
    pub enabled: bool,
    /// Source directory relative to the POM.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory: Option<String>,
    /// Included file patterns.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub includes: Option<Includes>,
    /// Excluded file patterns.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub excludes: Option<Excludes>,
}

impl Default for Source {
    fn default() -> Self {
        Self {
            scope: default_source_scope(),
            lang: default_source_language(),
            module: None,
            target_version: None,
            target_path: None,
            string_filtering: false,
            enabled: true,
            directory: None,
            includes: None,
            excludes: None,
        }
    }
}

fn default_source_scope() -> String {
    "main".to_string()
}

fn default_source_language() -> String {
    "java".to_string()
}

const fn source_enabled_by_default() -> bool {
    true
}

impl Build {
    /// Creates a builder for `Build`.
    #[must_use]
    pub fn builder() -> BuildBuilder {
        BuildBuilder::new()
    }
}

/// Builder for `Build`.
#[derive(Debug, Clone, Default)]
pub struct BuildBuilder {
    build: Build,
}

impl BuildBuilder {
    /// Creates a new builder.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the source directory.
    #[must_use]
    pub fn source_directory(mut self, dir: impl Into<String>) -> Self {
        self.build.source_directory = Some(dir.into());
        self
    }

    /// Sets the test source directory.
    #[must_use]
    pub fn test_source_directory(mut self, dir: impl Into<String>) -> Self {
        self.build.test_source_directory = Some(dir.into());
        self
    }

    /// Sets the output directory.
    #[must_use]
    pub fn output_directory(mut self, dir: impl Into<String>) -> Self {
        self.build.output_directory = Some(dir.into());
        self
    }

    /// Sets the final name.
    #[must_use]
    pub fn final_name(mut self, name: impl Into<String>) -> Self {
        self.build.final_name = Some(name.into());
        self
    }

    /// Sets the directory.
    #[must_use]
    pub fn directory(mut self, dir: impl Into<String>) -> Self {
        self.build.directory = Some(dir.into());
        self
    }

    /// Sets the default goal.
    #[must_use]
    pub fn default_goal(mut self, goal: impl Into<String>) -> Self {
        self.build.default_goal = Some(goal.into());
        self
    }

    /// Adds a Maven 4 source declaration.
    #[must_use]
    pub fn source(mut self, source: Source) -> Self {
        self.build
            .sources
            .get_or_insert_with(Sources::default)
            .sources
            .push(source);
        self
    }

    /// Adds a plugin.
    #[must_use]
    pub fn plugin(mut self, plugin: Plugin) -> Self {
        if self.build.plugins.is_none() {
            self.build.plugins = Some(BuildPlugins::default());
        }
        if let Some(ref mut plugins) = self.build.plugins {
            plugins.plugins.push(plugin);
        }
        self
    }

    /// Sets the plugin management.
    #[must_use]
    pub fn plugin_management(mut self, pm: PluginManagement) -> Self {
        self.build.plugin_management = Some(pm);
        self
    }

    /// Adds a resource.
    #[must_use]
    pub fn resource(mut self, resource: Resource) -> Self {
        if self.build.resources.is_none() {
            self.build.resources = Some(Resources::default());
        }
        if let Some(ref mut resources) = self.build.resources {
            resources.resources.push(resource);
        }
        self
    }

    /// Builds the `Build`.
    #[must_use]
    pub fn build(self) -> Build {
        self.build
    }
}

/// Represents a list of extensions.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct Extensions {
    /// The extensions.
    #[serde(rename = "extension", default)]
    pub extensions: Vec<Extension>,
}

/// Represents an extension.
#[non_exhaustive]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Extension {
    /// The group ID of the extension.
    pub group_id: String,
    /// The artifact ID of the extension.
    pub artifact_id: String,
    /// The version of the extension.
    pub version: String,
    /// Maven 4 extension configuration.
    #[serde(default, skip_serializing_if = "Configuration::is_empty")]
    pub configuration: Configuration,
}

impl Extension {
    /// Creates a new `Extension`.
    #[must_use]
    pub fn new(
        group_id: impl Into<String>,
        artifact_id: impl Into<String>,
        version: impl Into<String>,
    ) -> Self {
        Self {
            group_id: group_id.into(),
            artifact_id: artifact_id.into(),
            version: version.into(),
            configuration: Configuration::default(),
        }
    }
}

/// Represents a list of resources.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct Resources {
    /// The resources.
    #[serde(rename = "resource", default)]
    pub resources: Vec<Resource>,
}

/// Represents a list of test resources.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct TestResources {
    /// The test resources.
    #[serde(rename = "testResource", default)]
    pub resources: Vec<Resource>,
}

/// Represents a resource.
#[non_exhaustive]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Resource {
    /// The target path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_path: Option<String>,
    /// Whether filtering is enabled.
    #[serde(default, deserialize_with = "crate::types::deserialize_maven_boolean")]
    pub filtering: bool,
    /// The directory.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory: Option<String>,
    /// The includes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub includes: Option<Includes>,
    /// The excludes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub excludes: Option<Excludes>,
}

impl Resource {
    /// Creates a builder for `Resource`.
    #[must_use]
    pub fn builder() -> ResourceBuilder {
        ResourceBuilder::new()
    }
}

/// Builder for `Resource`.
#[derive(Debug, Clone, Default)]
pub struct ResourceBuilder {
    resource: Resource,
}

impl ResourceBuilder {
    /// Creates a new builder.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the directory.
    #[must_use]
    pub fn directory(mut self, dir: impl Into<String>) -> Self {
        self.resource.directory = Some(dir.into());
        self
    }

    /// Sets the target path.
    #[must_use]
    pub fn target_path(mut self, path: impl Into<String>) -> Self {
        self.resource.target_path = Some(path.into());
        self
    }

    /// Sets whether filtering is enabled.
    #[must_use]
    pub fn filtering(mut self, filtering: bool) -> Self {
        self.resource.filtering = filtering;
        self
    }

    /// Adds an include pattern.
    #[must_use]
    pub fn include(mut self, pattern: impl Into<String>) -> Self {
        if self.resource.includes.is_none() {
            self.resource.includes = Some(Includes::default());
        }
        if let Some(ref mut includes) = self.resource.includes {
            includes.includes.push(pattern.into());
        }
        self
    }

    /// Adds an exclude pattern.
    #[must_use]
    pub fn exclude(mut self, pattern: impl Into<String>) -> Self {
        if self.resource.excludes.is_none() {
            self.resource.excludes = Some(Excludes::default());
        }
        if let Some(ref mut excludes) = self.resource.excludes {
            excludes.excludes.push(pattern.into());
        }
        self
    }

    /// Builds the `Resource`.
    #[must_use]
    pub fn build(self) -> Resource {
        self.resource
    }
}

/// Represents a list of includes.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct Includes {
    /// The includes.
    #[serde(rename = "include", default)]
    pub includes: Vec<String>,
}

/// Represents a list of excludes.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct Excludes {
    /// The excludes.
    #[serde(rename = "exclude", default)]
    pub excludes: Vec<String>,
}

/// Represents a list of filters.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct Filters {
    /// The filters.
    #[serde(rename = "filter", default)]
    pub filters: Vec<String>,
}

/// Represents a list of build plugins.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct BuildPlugins {
    /// The plugins.
    #[serde(rename = "plugin", default)]
    pub plugins: Vec<Plugin>,
}

/// Represents plugin management.
#[non_exhaustive]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PluginManagement {
    /// The plugins.
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        serialize_with = "serialize_managed_plugins",
        deserialize_with = "deserialize_managed_plugins"
    )]
    pub plugins: Vec<Plugin>,
}

fn serialize_managed_plugins<S>(plugins: &[Plugin], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    #[derive(Serialize)]
    struct ManagedPlugins<'a> {
        #[serde(rename = "plugin")]
        plugins: &'a [Plugin],
    }

    ManagedPlugins { plugins }.serialize(serializer)
}

fn deserialize_managed_plugins<'de, D>(deserializer: D) -> Result<Vec<Plugin>, D::Error>
where
    D: Deserializer<'de>,
{
    BuildPlugins::deserialize(deserializer).map(|plugins| plugins.plugins)
}

impl PluginManagement {
    /// Creates a new `PluginManagement` with the given plugins.
    #[must_use]
    pub fn new(plugins: Vec<Plugin>) -> Self {
        Self { plugins }
    }

    /// Creates an empty `PluginManagement`.
    #[must_use]
    pub fn empty() -> Self {
        Self { plugins: vec![] }
    }
}

/// Represents a plugin.
#[non_exhaustive]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Plugin {
    /// The group ID of the plugin.
    #[serde(
        default = "default_plugin_group",
        skip_serializing_if = "Option::is_none"
    )]
    pub group_id: Option<String>,
    /// The artifact ID of the plugin.
    pub artifact_id: String,
    /// The version of the plugin.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Whether to enable extensions.
    #[serde(default, deserialize_with = "crate::types::deserialize_maven_boolean")]
    pub extensions: bool,
    /// The plugin executions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub executions: Option<PluginExecutions>,
    /// The plugin dependencies.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<Dependencies>,
    /// Whether the plugin is inherited.
    #[serde(
        default = "inherited_by_default",
        deserialize_with = "crate::types::deserialize_maven_boolean"
    )]
    pub inherited: bool,
    /// The plugin configuration.
    #[serde(default, skip_serializing_if = "Configuration::is_empty")]
    pub configuration: Configuration,
}

impl Plugin {
    /// Creates a new `Plugin` with the minimum required fields.
    #[must_use]
    pub fn new(artifact_id: impl Into<String>) -> Self {
        Self {
            group_id: default_plugin_group(),
            artifact_id: artifact_id.into(),
            version: None,
            extensions: false,
            executions: None,
            dependencies: None,
            inherited: true,
            configuration: Configuration::default(),
        }
    }

    /// Creates a builder for `Plugin`.
    #[must_use]
    pub fn builder(artifact_id: impl Into<String>) -> PluginBuilder {
        PluginBuilder::new(artifact_id)
    }
}

impl Default for Plugin {
    fn default() -> Self {
        Self::new("")
    }
}

#[expect(
    clippy::unnecessary_wraps,
    reason = "serde default function must return the optional field type"
)]
fn default_plugin_group() -> Option<String> {
    Some("org.apache.maven.plugins".to_string())
}

/// Builder for `Plugin`.
#[derive(Debug, Clone)]
pub struct PluginBuilder {
    plugin: Plugin,
}

impl PluginBuilder {
    /// Creates a new builder with the required artifact ID.
    #[must_use]
    pub fn new(artifact_id: impl Into<String>) -> Self {
        Self {
            plugin: Plugin::new(artifact_id),
        }
    }

    /// Sets the group ID.
    #[must_use]
    pub fn group_id(mut self, group_id: impl Into<String>) -> Self {
        self.plugin.group_id = Some(group_id.into());
        self
    }

    /// Sets the version.
    #[must_use]
    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.plugin.version = Some(version.into());
        self
    }

    /// Sets whether extensions are enabled.
    #[must_use]
    pub fn extensions(mut self, extensions: bool) -> Self {
        self.plugin.extensions = extensions;
        self
    }

    /// Sets whether the plugin is inherited.
    #[must_use]
    pub fn inherited(mut self, inherited: bool) -> Self {
        self.plugin.inherited = inherited;
        self
    }

    /// Adds a configuration entry.
    #[must_use]
    pub fn config(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.plugin.configuration.insert(key, value);
        self
    }

    /// Adds an execution.
    #[must_use]
    pub fn execution(mut self, execution: PluginExecution) -> Self {
        if self.plugin.executions.is_none() {
            self.plugin.executions = Some(PluginExecutions::default());
        }
        if let Some(ref mut executions) = self.plugin.executions {
            executions.executions.push(execution);
        }
        self
    }

    /// Builds the `Plugin`.
    #[must_use]
    pub fn build(self) -> Plugin {
        self.plugin
    }
}

/// Represents a list of plugin executions.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct PluginExecutions {
    /// The executions.
    #[serde(rename = "execution", default)]
    pub executions: Vec<PluginExecution>,
}

/// Represents a plugin execution.
#[non_exhaustive]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PluginExecution {
    /// The ID of the execution.
    #[serde(
        default = "default_execution_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub id: Option<String>,
    /// The phase of the execution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
    /// The goals of the execution.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub goals: Option<Goals>,
    /// Whether the execution is inherited.
    #[serde(
        default = "inherited_by_default",
        deserialize_with = "crate::types::deserialize_maven_boolean"
    )]
    pub inherited: bool,
    /// The configuration.
    #[serde(default, skip_serializing_if = "Configuration::is_empty")]
    pub configuration: Configuration,
}

impl PluginExecution {
    /// Creates a builder for `PluginExecution`.
    #[must_use]
    pub fn builder() -> PluginExecutionBuilder {
        PluginExecutionBuilder::new()
    }
}

impl Default for PluginExecution {
    fn default() -> Self {
        Self {
            id: default_execution_id(),
            phase: None,
            goals: None,
            inherited: true,
            configuration: Configuration::default(),
        }
    }
}

const fn inherited_by_default() -> bool {
    true
}

#[expect(
    clippy::unnecessary_wraps,
    reason = "serde default function must return the optional field type"
)]
fn default_execution_id() -> Option<String> {
    Some("default".to_string())
}

/// Builder for `PluginExecution`.
#[derive(Debug, Clone, Default)]
pub struct PluginExecutionBuilder {
    execution: PluginExecution,
}

impl PluginExecutionBuilder {
    /// Creates a new builder.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the ID.
    #[must_use]
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.execution.id = Some(id.into());
        self
    }

    /// Sets the phase.
    #[must_use]
    pub fn phase(mut self, phase: impl Into<String>) -> Self {
        self.execution.phase = Some(phase.into());
        self
    }

    /// Adds a goal.
    #[must_use]
    pub fn goal(mut self, goal: impl Into<String>) -> Self {
        if self.execution.goals.is_none() {
            self.execution.goals = Some(Goals::default());
        }
        if let Some(ref mut goals) = self.execution.goals {
            goals.goals.push(goal.into());
        }
        self
    }

    /// Adds a configuration entry.
    #[must_use]
    pub fn config(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.execution.configuration.insert(key, value);
        self
    }

    /// Builds the `PluginExecution`.
    #[must_use]
    pub fn build(self) -> PluginExecution {
        self.execution
    }
}

/// Represents a list of goals.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct Goals {
    /// The goals.
    #[serde(rename = "goal", default)]
    pub goals: Vec<String>,
}

impl Goals {
    /// Creates a new `Goals` from a vector of goal strings.
    #[must_use]
    pub fn from_vec(goals: Vec<String>) -> Self {
        Self { goals }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_builder() {
        let build = Build::builder()
            .source_directory("src/main/java")
            .final_name("my-app")
            .plugin(
                Plugin::builder("maven-compiler-plugin")
                    .group_id("org.apache.maven.plugins")
                    .version("3.8.1")
                    .config("source", "17")
                    .config("target", "17")
                    .build(),
            )
            .build();

        assert_eq!(build.source_directory, Some("src/main/java".to_string()));
        assert_eq!(build.final_name, Some("my-app".to_string()));
        assert!(build.plugins.is_some());
        assert_eq!(build.plugins.as_ref().unwrap().plugins.len(), 1);
    }

    #[test]
    fn test_plugin_builder() {
        let plugin = Plugin::builder("maven-surefire-plugin")
            .group_id("org.apache.maven.plugins")
            .version("3.0.0")
            .execution(
                PluginExecution::builder()
                    .id("default-test")
                    .phase("test")
                    .goal("test")
                    .build(),
            )
            .build();

        assert_eq!(plugin.artifact_id, "maven-surefire-plugin");
        assert!(plugin.executions.is_some());
    }

    #[test]
    fn test_resource_builder() {
        let resource = Resource::builder()
            .directory("src/main/resources")
            .filtering(true)
            .include("**/*.xml")
            .exclude("**/*.txt")
            .build();

        assert_eq!(resource.directory, Some("src/main/resources".to_string()));
        assert!(resource.filtering);
        assert!(resource.includes.is_some());
        assert!(resource.excludes.is_some());
    }
}
