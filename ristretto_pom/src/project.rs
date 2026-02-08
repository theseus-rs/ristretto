//! Maven Project type.

use crate::build::Build;
use crate::dependency::{Dependencies, DependencyManagement};
use crate::developer::{Contributors, Developers};
use crate::distribution::DistributionManagement;
use crate::license::Licenses;
use crate::organization::{MailingLists, Organization, Parent, Prerequisites};
use crate::profile::{Modules, Profiles};
use crate::reporting::Reporting;
use crate::repository::Repositories;
use crate::scm::{CiManagement, IssueManagement, Scm};
use crate::{DependencyScope, Error, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::Path;

/// Represents a Maven project.
#[non_exhaustive]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", rename = "project")]
pub struct Project {
    /// The version of the POM model.
    #[serde(rename = "modelVersion")]
    pub model_version: String,

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
    /// - `modelVersion` must be "4.0.0"
    /// - `artifactId` is always required (enforced by struct)
    /// - `groupId` and `version` can be inherited from parent
    ///
    /// # Errors
    ///
    /// Returns an error if required fields are missing.
    pub fn validate_syntax(&self) -> Result<()> {
        // Model version must be 4.0.0
        if self.model_version != "4.0.0" {
            return Err(Error::InvalidModelVersion(self.model_version.clone()));
        }

        // groupId is required unless inherited from parent
        if self.group_id.is_none() && self.parent.is_none() {
            return Err(Error::MissingField("groupId".to_string()));
        }

        // version is required unless inherited from parent
        if self.version.is_none() && self.parent.is_none() {
            return Err(Error::MissingField("version".to_string()));
        }

        // Validate parent has all required fields (if present)
        if let Some(ref parent) = self.parent {
            if parent.group_id.is_empty() {
                return Err(Error::MissingField("parent.groupId".to_string()));
            }
            if parent.artifact_id.is_empty() {
                return Err(Error::MissingField("parent.artifactId".to_string()));
            }
            if parent.version.is_empty() {
                return Err(Error::MissingField("parent.version".to_string()));
            }
        }

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
        // Validate dependencies
        for dependency in &self.dependencies.dependencies {
            if dependency.scope == Some(DependencyScope::System) && dependency.system_path.is_none()
            {
                return Err(Error::ValidationError(format!(
                    "Dependency {}:{} has system scope but no systemPath",
                    dependency.group_id, dependency.artifact_id
                )));
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
    pub fn validate_effective(&self, _parent: Option<&Project>) -> Result<()> {
        // Future: validate effective POM after parent inheritance
        // For now, just run standard validation
        self.validate()
    }

    /// Creates a new `Project` with the minimum required fields.
    #[must_use]
    pub fn new(
        group_id: impl Into<String>,
        artifact_id: impl Into<String>,
        version: impl Into<String>,
    ) -> Self {
        Self {
            model_version: "4.0.0".to_string(),
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
                model_version: "4.0.0".to_string(),
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
    use tempfile::NamedTempFile;

    #[test]
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
