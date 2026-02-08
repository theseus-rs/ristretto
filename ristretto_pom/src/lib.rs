//! # Ristretto POM
//!
//! [![Code Coverage](https://codecov.io/gh/theseus-rs/ristretto/branch/main/graph/badge.svg)](https://codecov.io/gh/theseus-rs/ristretto)
//! [![Benchmarks](https://img.shields.io/badge/%F0%9F%90%B0_bencher-enabled-6ec241)](https://bencher.dev/perf/theseus-rs-ristretto)
//! [![License](https://img.shields.io/crates/l/ristretto_pom)](https://github.com/theseus-rs/ristretto#license)
//! [![Semantic Versioning](https://img.shields.io/badge/%E2%9A%99%EF%B8%8F_SemVer-2.0.0-blue)](https://semver.org/spec/v2.0.0.html)
//!
//! A standalone library for parsing and manipulating
//! [Maven Project Object Model (POM)](https://maven.apache.org/pom.html) files.
//!
//! ## Example
//!
//! ```rust,ignore
//! use ristretto_pom::Project;
//!
//! # fn main() -> ristretto_pom::Result<()> {
//! let project = Project::from_file("pom.xml")?;
//! println!("Artifact ID: {}", project.artifact_id);
//! # Ok(())
//! # }
//! ```

#![forbid(unsafe_code)]

pub mod build;
pub mod dependency;
pub mod developer;
pub mod distribution;
mod error;
pub mod license;
pub mod organization;
pub mod profile;
pub mod project;
pub mod reporting;
pub mod repository;
pub mod scm;
pub mod types;

pub use error::{Error, Result};

pub use build::{
    Build, BuildBuilder, BuildPlugins, Excludes, Extension, Extensions, Filters, Goals, Includes,
    Plugin, PluginBuilder, PluginExecution, PluginExecutionBuilder, PluginExecutions,
    PluginManagement, Resource, ResourceBuilder, Resources, TestResources,
};
pub use dependency::{
    Dependencies, Dependency, DependencyBuilder, DependencyManagement, DependencyManagementBuilder,
    Exclusion, Exclusions,
};
pub use developer::{
    Contributor, ContributorBuilder, Contributors, Developer, DeveloperBuilder, Developers, Roles,
};
pub use distribution::{
    DistributionManagement, DistributionManagementBuilder, Relocation, RelocationBuilder, Site,
    SiteBuilder,
};
pub use license::{License, LicenseBuilder, Licenses};
pub use organization::{
    MailingList, MailingListBuilder, MailingLists, Organization, OrganizationBuilder, Parent,
    ParentBuilder, Prerequisites,
};
pub use profile::{
    Activation, ActivationBuilder, ActivationFile, ActivationOs, ActivationOsBuilder,
    ActivationProperty, Modules, Profile, ProfileBuilder, Profiles,
};
pub use project::{Project, ProjectBuilder};
pub use reporting::{
    ReportPlugin, ReportPluginBuilder, ReportSet, ReportSetBuilder, ReportSets, Reporting,
    ReportingBuilder, ReportingPlugins, Reports,
};
pub use repository::{
    Repositories, Repository, RepositoryBuilder, RepositoryPolicy, RepositoryPolicyBuilder,
};
pub use scm::{
    CiManagement, CiManagementBuilder, IssueManagement, Notifier, NotifierBuilder, Notifiers, Scm,
    ScmBuilder,
};
pub use types::{ChecksumPolicy, DependencyScope, Packaging, RepositoryLayout, UpdatePolicy};
