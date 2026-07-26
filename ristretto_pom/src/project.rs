//! Maven Project type.

use crate::build::Build;
use crate::dependency::{Dependencies, DependencyManagement};
use crate::developer::{Contributors, Developers};
use crate::distribution::DistributionManagement;
use crate::license::Licenses;
use crate::organization::{MailingLists, Organization, Parent, Prerequisites};
use crate::profile::{Modules, Profiles, Subprojects};
use crate::reporting::Reporting;
use crate::repository::Repositories;
use crate::scm::{CiManagement, IssueManagement, Scm};
use crate::version::PomVersion;
use crate::{DependencyScope, Error, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::Path;

/// Represents a Maven project.
#[non_exhaustive]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", rename = "project")]
pub struct Project {
    /// Whether child project URLs append the child's path while inheriting.
    #[serde(
        rename = "@child.project.url.inherit.append.path",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub child_project_url_inherit_append_path: Option<String>,

    /// Whether this Maven 4 project is the root of the source tree.
    #[serde(rename = "@root", default, skip_serializing_if = "std::ops::Not::not")]
    pub root: bool,

    /// Whether Maven should preserve this project's model version.
    #[serde(
        rename = "@preserve.model.version",
        default,
        skip_serializing_if = "std::ops::Not::not"
    )]
    pub preserve_model_version: bool,

    /// The version of the POM model.
    #[serde(rename = "modelVersion")]
    pub model_version: PomVersion,

    /// The parent project of this project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<Parent>,

    /// The group ID of the project.
    #[serde(rename = "groupId", skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,

    /// The artifact ID of the project.
    #[serde(rename = "artifactId")]
    pub artifact_id: String,

    /// The version of the project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /// The packaging type of the project (e.g., "jar", "war", "pom").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub packaging: Option<String>,

    /// The name of the project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The description of the project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The URL of the project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// The year of the project's inception.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inception_year: Option<String>,

    /// The organization that produced this project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,

    /// The licenses for this project.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub licenses: Option<Licenses>,

    /// The developers of this project.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub developers: Option<Developers>,

    /// The contributors to this project.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contributors: Option<Contributors>,

    /// The mailing lists for this project.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mailing_lists: Option<MailingLists>,

    /// The prerequisites for this project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prerequisites: Option<Prerequisites>,

    /// The modules (sub-projects) of this project.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modules: Option<Modules>,

    /// The Maven 4 subprojects of this project.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subprojects: Option<Subprojects>,

    /// The Source, Control, Management (SCM) information for this project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scm: Option<Scm>,

    /// The issue management information for this project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_management: Option<IssueManagement>,

    /// The Continuous Integration (CI) management information for this project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ci_management: Option<CiManagement>,

    /// The distribution management information for this project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution_management: Option<DistributionManagement>,

    /// The properties of this project.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub properties: BTreeMap<String, String>,

    /// The dependency management information for this project.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dependency_management: Option<DependencyManagement>,

    /// The dependencies of this project.
    #[serde(default)]
    pub dependencies: Dependencies,

    /// The repositories for this project.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repositories: Option<Repositories>,

    /// The plugin repositories for this project.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plugin_repositories: Option<Repositories>,

    /// The build configuration for this project.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub build: Option<Build>,

    /// The reporting configuration for this project.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reporting: Option<Reporting>,

    /// The profiles for this project.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profiles: Option<Profiles>,
}

impl Project {
    /// Parses a `Project` from a file.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read or if the XML is invalid.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        Self::from_reader(reader)
    }

    /// Parses a `Project` from a reader.
    ///
    /// This enables parsing POMs from any source (in-memory, network, etc.).
    ///
    /// # Errors
    ///
    /// Returns an error if the XML is invalid or validation fails.
    pub fn from_reader<R: Read>(reader: R) -> Result<Self> {
        let project: Project = quick_xml::de::from_reader(BufReader::new(reader))?;
        project.validate()?;
        Ok(project)
    }

    /// Writes the `Project` to a file.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be written or if serialization fails.
    pub fn to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let mut file = File::create(path)?;
        self.to_writer(&mut file)
    }

    /// Writes the `Project` to a writer.
    ///
    /// This enables writing POMs to any destination (in-memory, network, etc.).
    ///
    /// # Errors
    ///
    /// Returns an error if serialization fails.
    pub fn to_writer<W: Write>(&self, writer: &mut W) -> Result<()> {
        let xml = quick_xml::se::to_string(self)?;
        writer.write_all(xml.as_bytes())?;
        Ok(())
    }

    /// Validates the `Project` (combines syntax and semantic validation).
    ///
    /// # Errors
    ///
    /// Returns an error if required fields are missing or have invalid values.
    pub fn validate(&self) -> Result<()> {
        self.validate_syntax()?;
        self.validate_semantics()?;
        Ok(())
    }

    /// Validates the syntactic structure of the `Project`.
    ///
    /// This checks that required fields are present according to Maven rules:
    /// - `modelVersion` must be "3.0.0", "4.0.0", or "4.1.0"
    /// - `artifactId` is always required (enforced by struct)
    /// - `groupId` and `version` can be inherited from parent
    ///
    /// # Errors
    ///
    /// Returns an error if required fields are missing.
    pub fn validate_syntax(&self) -> Result<()> {
        // Model version must be supported by this crate.
        if !self.model_version.is_supported_model() {
            return Err(Error::InvalidModelVersion(self.model_version.to_string()));
        }

        // groupId is required unless inherited from parent
        if self.artifact_id.trim().is_empty() {
            return Err(Error::MissingField("artifactId".to_string()));
        }

        if self.group_id.is_none() && self.parent.is_none() {
            return Err(Error::MissingField("groupId".to_string()));
        }
        if self
            .group_id
            .as_ref()
            .is_some_and(|value| value.trim().is_empty())
        {
            return Err(Error::MissingField("groupId".to_string()));
        }

        // version is required unless inherited from parent
        if self.version.is_none() && self.parent.is_none() {
            return Err(Error::MissingField("version".to_string()));
        }
        if self
            .version
            .as_ref()
            .is_some_and(|value| value.trim().is_empty())
        {
            return Err(Error::MissingField("version".to_string()));
        }

        // Validate parent has all required fields (if present)
        if let Some(ref parent) = self.parent {
            if parent.group_id.trim().is_empty() {
                return Err(Error::MissingField("parent.groupId".to_string()));
            }
            if parent.artifact_id.trim().is_empty() {
                return Err(Error::MissingField("parent.artifactId".to_string()));
            }
            if parent.version.trim().is_empty() {
                return Err(Error::MissingField("parent.version".to_string()));
            }
        }

        validate_dependency_syntax(&self.dependencies, "dependencies")?;
        if let Some(management) = &self.dependency_management {
            validate_dependency_syntax(
                &management.dependencies,
                "dependencyManagement.dependencies",
            )?;
        }
        if let Some(profiles) = &self.profiles {
            let mut profile_ids = BTreeSet::new();
            for profile in &profiles.profiles {
                if profile.id.trim().is_empty() {
                    return Err(Error::MissingField("profile.id".to_string()));
                }
                if !profile_ids.insert(&profile.id) {
                    return Err(Error::ValidationError(format!(
                        "Duplicate profile id '{}'",
                        profile.id
                    )));
                }
                if let Some(dependencies) = &profile.dependencies {
                    validate_dependency_syntax(dependencies, "profile.dependencies")?;
                }
                if let Some(management) = &profile.dependency_management {
                    validate_dependency_syntax(
                        &management.dependencies,
                        "profile.dependencyManagement.dependencies",
                    )?;
                }
                validate_repositories(profile.repositories.as_ref(), "profile.repositories")?;
                validate_repositories(
                    profile.plugin_repositories.as_ref(),
                    "profile.pluginRepositories",
                )?;
                validate_build(profile.build.as_ref())?;
                validate_reporting(profile.reporting.as_ref())?;
            }
        }
        validate_repositories(self.repositories.as_ref(), "repositories")?;
        validate_repositories(self.plugin_repositories.as_ref(), "pluginRepositories")?;
        validate_build(self.build.as_ref())?;
        validate_reporting(self.reporting.as_ref())?;

        Ok(())
    }

    /// Validates the semantic correctness of the `Project`.
    ///
    /// This performs additional validation beyond syntax:
    /// - System-scoped dependencies must have systemPath
    /// - Import-scoped dependencies must be of type pom
    ///
    /// # Errors
    ///
    /// Returns an error if semantic validation fails.
    pub fn validate_semantics(&self) -> Result<()> {
        validate_dependencies(&self.dependencies, false)?;
        if let Some(management) = &self.dependency_management {
            validate_dependencies(&management.dependencies, true)?;
        }
        if let Some(profiles) = &self.profiles {
            for profile in &profiles.profiles {
                if profile.activation.as_ref().is_some_and(|activation| {
                    activation
                        .file
                        .as_ref()
                        .is_some_and(|file| file.exists.is_some() && file.missing.is_some())
                }) {
                    return Err(Error::ValidationError(format!(
                        "Profile '{}' activation.file cannot define both exists and missing",
                        profile.id
                    )));
                }
                if let Some(dependencies) = &profile.dependencies {
                    validate_dependencies(dependencies, false)?;
                }
                if let Some(management) = &profile.dependency_management {
                    validate_dependencies(&management.dependencies, true)?;
                }
            }
        }

        Ok(())
    }

    /// Validates the effective POM with parent inheritance considered.
    ///
    /// This should be called after resolving parent POMs to validate
    /// the complete effective configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if effective validation fails.
    pub fn validate_effective(&self, parent: Option<&Project>) -> Result<()> {
        self.validate()?;
        match (&self.parent, parent) {
            (None, None) => Ok(()),
            (Some(declared), Some(parent)) => {
                parent.validate()?;
                let parent_group = effective_group_id(parent).unwrap_or_default();
                let parent_version = effective_version(parent).unwrap_or_default();
                if declared.group_id != parent_group
                    || declared.artifact_id != parent.artifact_id
                    || !parent_version_matches(&declared.version, parent_version)
                {
                    return Err(Error::ValidationError(format!(
                        "Resolved parent {}:{}:{} does not match declared parent {}:{}:{}",
                        parent_group,
                        parent.artifact_id,
                        parent_version,
                        declared.group_id,
                        declared.artifact_id,
                        declared.version
                    )));
                }
                Ok(())
            }
            (Some(_), None) => Err(Error::ValidationError(
                "Effective validation requires the declared parent project".to_string(),
            )),
            (None, Some(_)) => Err(Error::ValidationError(
                "A resolved parent was supplied for a project without a parent declaration"
                    .to_string(),
            )),
        }
    }

    /// Creates a new `Project` with the minimum required fields.
    #[must_use]
    pub fn new(
        group_id: impl Into<String>,
        artifact_id: impl Into<String>,
        version: impl Into<String>,
    ) -> Self {
        Self {
            child_project_url_inherit_append_path: None,
            root: false,
            preserve_model_version: false,
            model_version: PomVersion::DEFAULT_MODEL,
            parent: None,
            group_id: Some(group_id.into()),
            artifact_id: artifact_id.into(),
            version: Some(version.into()),
            packaging: None,
            name: None,
            description: None,
            url: None,
            inception_year: None,
            organization: None,
            licenses: None,
            developers: None,
            contributors: None,
            mailing_lists: None,
            prerequisites: None,
            modules: None,
            subprojects: None,
            scm: None,
            issue_management: None,
            ci_management: None,
            distribution_management: None,
            properties: BTreeMap::new(),
            dependency_management: None,
            dependencies: Dependencies::default(),
            repositories: None,
            plugin_repositories: None,
            build: None,
            reporting: None,
            profiles: None,
        }
    }

    /// Creates a builder for constructing a `Project`.
    #[must_use]
    pub fn builder(artifact_id: impl Into<String>) -> ProjectBuilder {
        ProjectBuilder::new(artifact_id)
    }
}

fn parent_version_matches(declared: &str, resolved: &str) -> bool {
    declared == resolved
        || (declared.starts_with(['[', '('])
            && declared.ends_with([']', ')'])
            && !resolved.trim().is_empty())
}

fn effective_group_id(project: &Project) -> Option<&str> {
    project.group_id.as_deref().or_else(|| {
        project
            .parent
            .as_ref()
            .map(|parent| parent.group_id.as_str())
    })
}

fn effective_version(project: &Project) -> Option<&str> {
    project.version.as_deref().or_else(|| {
        project
            .parent
            .as_ref()
            .map(|parent| parent.version.as_str())
    })
}

fn validate_dependency_syntax(dependencies: &Dependencies, location: &str) -> Result<()> {
    for dependency in &dependencies.dependencies {
        if dependency.group_id.trim().is_empty() {
            return Err(Error::MissingField(format!(
                "{location}.dependency.groupId"
            )));
        }
        if dependency.artifact_id.trim().is_empty() {
            return Err(Error::MissingField(format!(
                "{location}.dependency.artifactId"
            )));
        }
        if dependency
            .version
            .as_ref()
            .is_some_and(|value| value.trim().is_empty())
        {
            return Err(Error::MissingField(format!(
                "{location}.dependency.version"
            )));
        }
        if let Some(exclusions) = &dependency.exclusions {
            for exclusion in &exclusions.exclusions {
                if exclusion.group_id.trim().is_empty() {
                    return Err(Error::MissingField(format!(
                        "{location}.dependency.exclusions.exclusion.groupId"
                    )));
                }
                if exclusion.artifact_id.trim().is_empty() {
                    return Err(Error::MissingField(format!(
                        "{location}.dependency.exclusions.exclusion.artifactId"
                    )));
                }
            }
        }
    }
    Ok(())
}

fn validate_repositories(repositories: Option<&Repositories>, location: &str) -> Result<()> {
    let Some(repositories) = repositories else {
        return Ok(());
    };
    let mut ids = BTreeSet::new();
    for repository in &repositories.repositories {
        if repository.id.trim().is_empty() {
            return Err(Error::MissingField(format!("{location}.repository.id")));
        }
        if repository.url.trim().is_empty() {
            return Err(Error::MissingField(format!("{location}.repository.url")));
        }
        if !ids.insert(&repository.id) {
            return Err(Error::ValidationError(format!(
                "Duplicate repository id '{}' in {location}",
                repository.id
            )));
        }
    }
    Ok(())
}

fn validate_build(build: Option<&Build>) -> Result<()> {
    let Some(build) = build else {
        return Ok(());
    };
    if let Some(extensions) = &build.extensions {
        for extension in &extensions.extensions {
            if extension.group_id.trim().is_empty()
                || extension.artifact_id.trim().is_empty()
                || extension.version.trim().is_empty()
            {
                return Err(Error::ValidationError(
                    "Build extensions require non-empty groupId, artifactId, and version"
                        .to_string(),
                ));
            }
        }
    }
    let plugins = build
        .plugins
        .iter()
        .flat_map(|plugins| &plugins.plugins)
        .chain(
            build
                .plugin_management
                .iter()
                .flat_map(|management| &management.plugins),
        );
    for plugin in plugins {
        if plugin.artifact_id.trim().is_empty() {
            return Err(Error::MissingField(
                "build.plugins.plugin.artifactId".to_string(),
            ));
        }
        if let Some(dependencies) = &plugin.dependencies {
            validate_dependency_syntax(dependencies, "build.plugins.plugin.dependencies")?;
        }
    }
    Ok(())
}

fn validate_reporting(reporting: Option<&Reporting>) -> Result<()> {
    let Some(reporting) = reporting else {
        return Ok(());
    };
    if let Some(plugins) = &reporting.plugins {
        for plugin in &plugins.plugins {
            if plugin.artifact_id.trim().is_empty() {
                return Err(Error::MissingField(
                    "reporting.plugins.plugin.artifactId".to_string(),
                ));
            }
        }
    }
    Ok(())
}

fn validate_dependencies(dependencies: &Dependencies, managed: bool) -> Result<()> {
    for dependency in &dependencies.dependencies {
        let coordinate = format!("{}:{}", dependency.group_id, dependency.artifact_id);
        if dependency.scope == Some(DependencyScope::System)
            && dependency
                .system_path
                .as_ref()
                .is_none_or(|path| path.trim().is_empty())
        {
            return Err(Error::ValidationError(format!(
                "Dependency {coordinate} has system scope but no systemPath"
            )));
        }
        if dependency.scope != Some(DependencyScope::System) && dependency.system_path.is_some() {
            return Err(Error::ValidationError(format!(
                "Dependency {coordinate} has systemPath without system scope"
            )));
        }
        if dependency.scope == Some(DependencyScope::Import) {
            if !managed {
                return Err(Error::ValidationError(format!(
                    "Dependency {coordinate} uses import scope outside dependencyManagement"
                )));
            }
            if dependency.r#type.as_deref() != Some("pom") {
                return Err(Error::ValidationError(format!(
                    "Imported dependency {coordinate} must have type pom"
                )));
            }
            if dependency.version.is_none() {
                return Err(Error::ValidationError(format!(
                    "Imported dependency {coordinate} must have a version"
                )));
            }
        }
    }
    Ok(())
}

/// Builder for constructing a `Project`.
#[derive(Debug, Clone)]
pub struct ProjectBuilder {
    project: Project,
}

impl ProjectBuilder {
    /// Creates a new builder with the required artifact ID.
    #[must_use]
    pub fn new(artifact_id: impl Into<String>) -> Self {
        Self {
            project: Project {
                child_project_url_inherit_append_path: None,
                root: false,
                preserve_model_version: false,
                model_version: PomVersion::DEFAULT_MODEL,
                parent: None,
                group_id: None,
                artifact_id: artifact_id.into(),
                version: None,
                packaging: None,
                name: None,
                description: None,
                url: None,
                inception_year: None,
                organization: None,
                licenses: None,
                developers: None,
                contributors: None,
                mailing_lists: None,
                prerequisites: None,
                modules: None,
                subprojects: None,
                scm: None,
                issue_management: None,
                ci_management: None,
                distribution_management: None,
                properties: BTreeMap::new(),
                dependency_management: None,
                dependencies: Dependencies::default(),
                repositories: None,
                plugin_repositories: None,
                build: None,
                reporting: None,
                profiles: None,
            },
        }
    }

    /// Sets the group ID.
    #[must_use]
    pub fn group_id(mut self, group_id: impl Into<String>) -> Self {
        self.project.group_id = Some(group_id.into());
        self
    }

    /// Sets the version.
    #[must_use]
    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.project.version = Some(version.into());
        self
    }

    /// Marks this Maven 4 project as the source-tree root.
    #[must_use]
    pub fn root(mut self, root: bool) -> Self {
        self.project.root = root;
        self
    }

    /// Controls whether Maven preserves this project's model version.
    #[must_use]
    pub fn preserve_model_version(mut self, preserve: bool) -> Self {
        self.project.preserve_model_version = preserve;
        self
    }

    /// Sets the parent.
    #[must_use]
    pub fn parent(mut self, parent: Parent) -> Self {
        self.project.parent = Some(parent);
        self
    }

    /// Sets the packaging.
    #[must_use]
    pub fn packaging(mut self, packaging: impl Into<String>) -> Self {
        self.project.packaging = Some(packaging.into());
        self
    }

    /// Sets the name.
    #[must_use]
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.project.name = Some(name.into());
        self
    }

    /// Sets the description.
    #[must_use]
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.project.description = Some(description.into());
        self
    }

    /// Sets the URL.
    #[must_use]
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.project.url = Some(url.into());
        self
    }

    /// Sets the inception year.
    #[must_use]
    pub fn inception_year(mut self, year: impl Into<String>) -> Self {
        self.project.inception_year = Some(year.into());
        self
    }

    /// Sets the organization.
    #[must_use]
    pub fn organization(mut self, org: Organization) -> Self {
        self.project.organization = Some(org);
        self
    }

    /// Sets the licenses.
    #[must_use]
    pub fn licenses(mut self, licenses: Licenses) -> Self {
        self.project.licenses = Some(licenses);
        self
    }

    /// Sets the developers.
    #[must_use]
    pub fn developers(mut self, developers: Developers) -> Self {
        self.project.developers = Some(developers);
        self
    }

    /// Sets the SCM.
    #[must_use]
    pub fn scm(mut self, scm: Scm) -> Self {
        self.project.scm = Some(scm);
        self
    }

    /// Sets the issue management.
    #[must_use]
    pub fn issue_management(mut self, im: IssueManagement) -> Self {
        self.project.issue_management = Some(im);
        self
    }

    /// Sets the CI management.
    #[must_use]
    pub fn ci_management(mut self, ci: CiManagement) -> Self {
        self.project.ci_management = Some(ci);
        self
    }

    /// Adds a property.
    #[must_use]
    pub fn property(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.project.properties.insert(key.into(), value.into());
        self
    }

    /// Sets the dependencies.
    #[must_use]
    pub fn dependencies(mut self, dependencies: Dependencies) -> Self {
        self.project.dependencies = dependencies;
        self
    }

    /// Sets the dependency management.
    #[must_use]
    pub fn dependency_management(mut self, dm: DependencyManagement) -> Self {
        self.project.dependency_management = Some(dm);
        self
    }

    /// Sets the repositories.
    #[must_use]
    pub fn repositories(mut self, repos: Repositories) -> Self {
        self.project.repositories = Some(repos);
        self
    }

    /// Sets the build configuration.
    #[must_use]
    pub fn build(mut self, build: Build) -> Self {
        self.project.build = Some(build);
        self
    }

    /// Sets the reporting configuration.
    #[must_use]
    pub fn reporting(mut self, reporting: Reporting) -> Self {
        self.project.reporting = Some(reporting);
        self
    }

    /// Sets the profiles.
    #[must_use]
    pub fn profiles(mut self, profiles: Profiles) -> Self {
        self.project.profiles = Some(profiles);
        self
    }

    /// Sets the distribution management.
    #[must_use]
    pub fn distribution_management(mut self, dm: DistributionManagement) -> Self {
        self.project.distribution_management = Some(dm);
        self
    }

    /// Builds the `Project`.
    #[must_use]
    pub fn build_project(self) -> Project {
        self.project
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(not(target_family = "wasm"))]
    use tempfile::NamedTempFile;

    #[test]
    #[cfg(not(target_family = "wasm"))]
    fn test_parse_pom() -> Result<()> {
        let xml = r"
<project>
    <modelVersion>4.0.0</modelVersion>
    <groupId>com.example</groupId>
    <artifactId>my-app</artifactId>
    <version>1.0.0</version>
    <dependencies>
        <dependency>
            <groupId>junit</groupId>
            <artifactId>junit</artifactId>
            <version>4.12</version>
            <scope>test</scope>
        </dependency>
    </dependencies>
</project>
";
        let mut file = NamedTempFile::new()?;
        file.write_all(xml.as_bytes())?;
        let project = Project::from_file(file.path())?;

        assert_eq!(project.group_id, Some("com.example".to_string()));
        assert_eq!(project.artifact_id, "my-app");
        assert_eq!(project.version, Some("1.0.0".to_string()));
        assert_eq!(project.dependencies.dependencies.len(), 1);
        assert_eq!(project.dependencies.dependencies[0].group_id, "junit");
        Ok(())
    }

    #[test]
    fn test_project_builder() {
        let project = Project::builder("my-app")
            .group_id("com.example")
            .version("1.0.0")
            .name("My Application")
            .description("A sample application")
            .property("java.version", "17")
            .build_project();

        assert_eq!(project.group_id, Some("com.example".to_string()));
        assert_eq!(project.artifact_id, "my-app");
        assert_eq!(project.name, Some("My Application".to_string()));
        assert!(project.properties.contains_key("java.version"));
    }
}
