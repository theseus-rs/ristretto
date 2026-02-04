//! Reporting types.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Represents reporting configuration.
#[non_exhaustive]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Reporting {
    /// Whether to exclude defaults.
    #[serde(default)]
    pub exclude_defaults: bool,
    /// The output directory.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_directory: Option<String>,
    /// The reporting plugins.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plugins: Option<ReportingPlugins>,
}

impl Reporting {
    /// Creates a builder for `Reporting`.
    #[must_use]
    pub fn builder() -> ReportingBuilder {
        ReportingBuilder::new()
    }
}

/// Builder for `Reporting`.
#[derive(Debug, Clone, Default)]
pub struct ReportingBuilder {
    reporting: Reporting,
}

impl ReportingBuilder {
    /// Creates a new builder.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets whether to exclude defaults.
    #[must_use]
    pub fn exclude_defaults(mut self, exclude: bool) -> Self {
        self.reporting.exclude_defaults = exclude;
        self
    }

    /// Sets the output directory.
    #[must_use]
    pub fn output_directory(mut self, dir: impl Into<String>) -> Self {
        self.reporting.output_directory = Some(dir.into());
        self
    }

    /// Adds a plugin.
    #[must_use]
    pub fn plugin(mut self, plugin: ReportPlugin) -> Self {
        if self.reporting.plugins.is_none() {
            self.reporting.plugins = Some(ReportingPlugins::default());
        }
        if let Some(ref mut plugins) = self.reporting.plugins {
            plugins.plugins.push(plugin);
        }
        self
    }

    /// Builds the `Reporting`.
    #[must_use]
    pub fn build(self) -> Reporting {
        self.reporting
    }
}

/// Represents a list of reporting plugins.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct ReportingPlugins {
    /// The plugins.
    #[serde(rename = "plugin", default)]
    pub plugins: Vec<ReportPlugin>,
}

/// Represents a reporting plugin.
#[non_exhaustive]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReportPlugin {
    /// The group ID of the plugin.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// The artifact ID of the plugin.
    pub artifact_id: String,
    /// The version of the plugin.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// The report sets.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub report_sets: Option<ReportSets>,
    /// The configuration.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub configuration: BTreeMap<String, String>,
}

impl ReportPlugin {
    /// Creates a new `ReportPlugin` with the given artifact ID.
    #[must_use]
    pub fn new(artifact_id: impl Into<String>) -> Self {
        Self {
            group_id: None,
            artifact_id: artifact_id.into(),
            version: None,
            report_sets: None,
            configuration: BTreeMap::new(),
        }
    }

    /// Creates a builder for `ReportPlugin`.
    #[must_use]
    pub fn builder(artifact_id: impl Into<String>) -> ReportPluginBuilder {
        ReportPluginBuilder::new(artifact_id)
    }
}

/// Builder for `ReportPlugin`.
#[derive(Debug, Clone)]
pub struct ReportPluginBuilder {
    plugin: ReportPlugin,
}

impl ReportPluginBuilder {
    /// Creates a new builder with the required artifact ID.
    #[must_use]
    pub fn new(artifact_id: impl Into<String>) -> Self {
        Self {
            plugin: ReportPlugin::new(artifact_id),
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

    /// Adds a configuration entry.
    #[must_use]
    pub fn config(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.plugin.configuration.insert(key.into(), value.into());
        self
    }

    /// Adds a report set.
    #[must_use]
    pub fn report_set(mut self, report_set: ReportSet) -> Self {
        if self.plugin.report_sets.is_none() {
            self.plugin.report_sets = Some(ReportSets::default());
        }
        if let Some(ref mut sets) = self.plugin.report_sets {
            sets.report_sets.push(report_set);
        }
        self
    }

    /// Builds the `ReportPlugin`.
    #[must_use]
    pub fn build(self) -> ReportPlugin {
        self.plugin
    }
}

/// Represents a list of report sets.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct ReportSets {
    /// The report sets.
    #[serde(rename = "reportSet", default)]
    pub report_sets: Vec<ReportSet>,
}

/// Represents a report set.
#[non_exhaustive]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReportSet {
    /// The ID of the report set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The reports.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reports: Option<Reports>,
    /// Whether the report set is inherited.
    #[serde(default)]
    pub inherited: bool,
    /// The configuration.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub configuration: BTreeMap<String, String>,
}

impl ReportSet {
    /// Creates a builder for `ReportSet`.
    #[must_use]
    pub fn builder() -> ReportSetBuilder {
        ReportSetBuilder::new()
    }
}

/// Builder for `ReportSet`.
#[derive(Debug, Clone, Default)]
pub struct ReportSetBuilder {
    report_set: ReportSet,
}

impl ReportSetBuilder {
    /// Creates a new builder.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the ID.
    #[must_use]
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.report_set.id = Some(id.into());
        self
    }

    /// Adds a report.
    #[must_use]
    pub fn report(mut self, report: impl Into<String>) -> Self {
        if self.report_set.reports.is_none() {
            self.report_set.reports = Some(Reports::default());
        }
        if let Some(ref mut reports) = self.report_set.reports {
            reports.reports.push(report.into());
        }
        self
    }

    /// Sets whether the report set is inherited.
    #[must_use]
    pub fn inherited(mut self, inherited: bool) -> Self {
        self.report_set.inherited = inherited;
        self
    }

    /// Builds the `ReportSet`.
    #[must_use]
    pub fn build(self) -> ReportSet {
        self.report_set
    }
}

/// Represents a list of reports.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct Reports {
    /// The reports.
    #[serde(rename = "report", default)]
    pub reports: Vec<String>,
}

impl Reports {
    /// Creates a `Reports` from a vector of report names.
    #[must_use]
    pub fn from_vec(reports: Vec<String>) -> Self {
        Self { reports }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reporting_builder() {
        let reporting = Reporting::builder()
            .output_directory("target/site")
            .exclude_defaults(true)
            .plugin(
                ReportPlugin::builder("maven-javadoc-plugin")
                    .group_id("org.apache.maven.plugins")
                    .version("3.3.1")
                    .build(),
            )
            .build();

        assert!(reporting.exclude_defaults);
        assert_eq!(reporting.output_directory, Some("target/site".to_string()));
        assert!(reporting.plugins.is_some());
    }

    #[test]
    fn test_report_plugin_builder() {
        let plugin = ReportPlugin::builder("maven-project-info-reports-plugin")
            .group_id("org.apache.maven.plugins")
            .report_set(
                ReportSet::builder()
                    .id("default")
                    .report("dependencies")
                    .report("license")
                    .build(),
            )
            .build();

        assert!(plugin.report_sets.is_some());
        let sets = plugin.report_sets.unwrap();
        assert_eq!(sets.report_sets.len(), 1);
    }
}
