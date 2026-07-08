#![cfg(not(target_family = "wasm"))]
#![expect(
    clippy::indexing_slicing,
    reason = "integration tests assert fixture positions by index"
)]
//! Tests for builder helpers and collection constructors.

use ristretto_pom::{
    Activation, ActivationFile, ActivationOs, ActivationProperty, Build, CiManagement, Contributor,
    Contributors, Dependencies, Dependency, DependencyManagement, DependencyScope, Developer,
    Developers, DistributionManagement, Exclusion, Exclusions, License, Licenses, MailingList,
    MailingLists, Modules, Notifier, Organization, Parent, PluginManagement, Profile, Profiles,
    Project, Relocation, Repositories, Repository, RepositoryLayout, RepositoryPolicy, Scm, Site,
};

#[test]
fn build_and_dependency_collection_paths() {
    let resource = ristretto_pom::Resource::builder()
        .directory("src/main/resources")
        .build();
    let build = Build::builder()
        .plugin_management(PluginManagement::empty())
        .resource(resource)
        .build();
    assert!(build.plugin_management.is_some());
    assert_eq!(
        build
            .resources
            .as_ref()
            .map_or(0, |items| items.resources.len()),
        1
    );

    let dependency = Dependency::builder("com.example", "lib")
        .scope(DependencyScope::System)
        .system_path("/opt/lib.jar")
        .build();
    let dependencies = Dependencies::from_vec(vec![dependency]);
    let mgmt = DependencyManagement::new(dependencies);
    assert_eq!(mgmt.dependencies.len(), 1);

    let empty_exclusions = Exclusions::new();
    let exclusions = Exclusions::from_vec(vec![Exclusion::new("com.example", "excluded")]);
    assert!(empty_exclusions.exclusions.is_empty());
    assert_eq!(exclusions.exclusions[0].artifact_id, "excluded");
}

#[test]
fn developer_distribution_and_organization_collections() {
    let developer = Developer::builder().name("Dev").build();
    let developers = Developers::from_vec(vec![developer]);
    assert_eq!(developers.developers.len(), 1);

    let mut contributors = Contributors::new();
    contributors.add(Contributor::builder().name("Contributor").build());
    let contributors = Contributors::from_vec(contributors.contributors);
    assert_eq!(contributors.contributors.len(), 1);

    let relocation = Relocation::builder()
        .group_id("com.new")
        .artifact_id("new-artifact")
        .version("2.0.0")
        .build();
    let dist = DistributionManagement::builder()
        .relocation(relocation)
        .download_url("https://downloads.example.com")
        .status("deployed")
        .build();
    assert!(dist.relocation.is_some());
    assert_eq!(dist.status.as_deref(), Some("deployed"));

    let mailing_lists = MailingLists::from_vec(vec![MailingList::builder().name("Users").build()]);
    assert_eq!(mailing_lists.mailing_lists.len(), 1);
}

#[test]
fn profile_builders_and_shortcuts() {
    let modules = Modules::from_vec(vec!["core".to_string()]);
    assert_eq!(modules.modules[0], "core");

    let activation = Activation::builder()
        .active_by_default(true)
        .jdk("21")
        .os(ActivationOs::builder()
            .name("macOS")
            .family("mac")
            .arch("aarch64")
            .version("14")
            .build())
        .property("env", Some("ci".to_string()))
        .file(ActivationFile::exists("pom.xml"))
        .build();
    assert!(activation.active_by_default);
    assert_eq!(activation.jdk.as_deref(), Some("21"));

    let mac = ActivationOs::mac();
    assert_eq!(mac.family.as_deref(), Some("mac"));

    let property = ActivationProperty::new("skipTests");
    let property_with_value = ActivationProperty::with_value("env", "ci");
    assert_eq!(property.name, "skipTests");
    assert_eq!(property_with_value.value.as_deref(), Some("ci"));

    let profile = Profile::builder("dev")
        .activation(Activation::active_by_default())
        .build_config(Build::builder().default_goal("verify").build())
        .dependencies(Dependencies::from_vec(vec![Dependency::new(
            "com.example",
            "lib",
        )]))
        .repositories(Repositories::from_vec(vec![Repository::new(
            "central",
            "https://repo.maven.apache.org/maven2",
        )]))
        .build();
    let profiles = Profiles::from_vec(vec![profile]);
    assert_eq!(profiles.profiles.len(), 1);
}

#[test]
fn project_builder_optional_sections() {
    let project = Project::builder("my-app")
        .parent(Parent::new("com.parent", "parent", "1.0.0"))
        .packaging("pom")
        .url("https://example.com")
        .inception_year("2026")
        .organization(Organization::new("Example Org"))
        .licenses(Licenses::from_vec(vec![License::mit()]))
        .developers(Developers::from_vec(vec![
            Developer::builder().id("dev").build(),
        ]))
        .scm(Scm::github("theseus-rs", "ristretto"))
        .issue_management(ristretto_pom::IssueManagement::new(
            "GitHub",
            "https://github.com/theseus-rs/ristretto/issues",
        ))
        .ci_management(
            CiManagement::builder("GitHub Actions", "https://github.com/actions").build(),
        )
        .dependencies(Dependencies::from_vec(vec![Dependency::new(
            "com.example",
            "lib",
        )]))
        .dependency_management(DependencyManagement::new(Dependencies::new()))
        .repositories(Repositories::from_vec(vec![Repository::new(
            "central",
            "https://repo.maven.apache.org/maven2",
        )]))
        .build(Build::builder().default_goal("verify").build())
        .reporting(
            ristretto_pom::Reporting::builder()
                .output_directory("target/site")
                .build(),
        )
        .profiles(Profiles::from_vec(vec![Profile::new("release")]))
        .distribution_management(
            DistributionManagement::builder()
                .site(Site::new("docs", "https://docs.example.com"))
                .build(),
        )
        .build_project();

    assert_eq!(project.packaging.as_deref(), Some("pom"));
    assert!(project.parent.is_some());
    assert!(project.distribution_management.is_some());
}

#[test]
fn repository_and_scm_shortcuts() {
    let repositories = Repositories::from_vec(vec![
        Repository::builder("central", "https://repo.maven.apache.org/maven2")
            .layout(RepositoryLayout::Default)
            .releases(RepositoryPolicy::enabled())
            .build(),
    ]);
    assert_eq!(repositories.repositories.len(), 1);

    let notifier = Notifier::mail("team@example.com");
    assert_eq!(notifier.r#type, "mail");
    assert!(notifier.send_on_error);
}
