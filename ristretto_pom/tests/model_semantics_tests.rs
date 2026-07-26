#![cfg(not(target_family = "wasm"))]
//! Tests for model construction, validation, and effective-model semantics.

use ristretto_pom::*;
use serde::Deserialize;
use serde::de::value::{Error as ValueError, StrDeserializer};

fn valid_project() -> Project {
    Project::new("com.example", "app", "1.0")
}

fn assert_invalid(project: &Project) {
    assert!(project.validate().is_err());
}

#[test]
fn model_builders_preserve_configured_values() {
    let mut subprojects = Subprojects::new();
    subprojects.add("cli");
    assert_eq!(
        Subprojects::from_vec(subprojects.subprojects).subprojects,
        ["cli"]
    );

    let source = Source::default();
    let build = Build::builder().source(source).build();
    assert_eq!(
        build
            .sources
            .as_ref()
            .and_then(|sources| sources.sources.first())
            .map(|source| (source.scope.as_str(), source.lang.as_str(), source.enabled)),
        Some(("main", "java", true))
    );
    assert_eq!(Plugin::default().artifact_id, "");

    let site = Site::builder("docs", "https://docs.example")
        .child_url_append_path(false)
        .build();
    assert_eq!(
        site.child_site_url_inherit_append_path.as_deref(),
        Some("false")
    );

    let repository = Repository::builder("deploy", "https://repo.example")
        .unique_version(false)
        .build();
    assert_eq!(repository.unique_version, Some(false));

    let scm = Scm::builder()
        .child_connection_append_path(false)
        .child_developer_connection_append_path(true)
        .child_url_append_path(false)
        .build();
    assert_eq!(
        scm.child_scm_connection_inherit_append_path.as_deref(),
        Some("false")
    );
    assert_eq!(
        scm.child_scm_developer_connection_inherit_append_path
            .as_deref(),
        Some("true")
    );
    assert_eq!(
        scm.child_scm_url_inherit_append_path.as_deref(),
        Some("false")
    );
    assert_eq!(Notifier::default().r#type, "mail");

    let report = ReportPlugin::builder("report").inherited(false).build();
    assert!(!report.inherited);
    assert_eq!(ReportPlugin::default().artifact_id, "");

    let project = Project::builder("app")
        .group_id("com.example")
        .version("1.0")
        .root(true)
        .preserve_model_version(true)
        .build_project();
    assert!(project.root);
    assert!(project.preserve_model_version);

    let mut dependency = Dependency::new("com.example", "library");
    dependency.optional = true;
    dependency.optional_explicit = true;
    let xml = quick_xml::se::to_string(&dependency).expect("serialize dependency");
    assert!(xml.contains("<optional>true</optional>"));

    let invalid = Configuration::deserialize(StrDeserializer::<ValueError>::new("not a map"));
    assert!(invalid.is_err());
}

#[test]
#[expect(
    clippy::too_many_lines,
    reason = "one table-like test enumerates independent validation failures"
)]
fn project_validation_rejects_invalid_model_structure() {
    let mut project = valid_project();
    project.artifact_id = " ".to_string();
    assert_invalid(&project);

    let mut project = valid_project();
    project.group_id = Some(" ".to_string());
    assert_invalid(&project);

    let mut project = valid_project();
    project.version = Some(" ".to_string());
    assert_invalid(&project);

    for parent in [
        Parent::new("", "parent", "1"),
        Parent::new("com.example", "", "1"),
        Parent::new("com.example", "parent", ""),
    ] {
        let mut project = valid_project();
        project.parent = Some(parent);
        assert_invalid(&project);
    }

    let mut project = valid_project();
    project.dependencies = Dependencies::from_vec(vec![Dependency::new("", "library")]);
    assert_invalid(&project);

    let mut project = valid_project();
    project.dependencies = Dependencies::from_vec(vec![Dependency::new("com.example", "")]);
    assert_invalid(&project);

    let mut dependency = Dependency::new("com.example", "library");
    dependency.version = Some(" ".to_string());
    let mut project = valid_project();
    project.dependencies = Dependencies::from_vec(vec![dependency]);
    assert_invalid(&project);

    for exclusion in [
        Exclusion::new("", "excluded"),
        Exclusion::new("com.example", ""),
    ] {
        let dependency = Dependency::builder("com.example", "library")
            .exclusion(exclusion)
            .build();
        let mut project = valid_project();
        project.dependencies = Dependencies::from_vec(vec![dependency]);
        assert_invalid(&project);
    }

    let mut project = valid_project();
    project.dependency_management = Some(DependencyManagement::new(Dependencies::from_vec(vec![
        Dependency::new("", "managed"),
    ])));
    assert_invalid(&project);

    let mut project = valid_project();
    project.profiles = Some(Profiles::from_vec(vec![Profile::new("")]));
    assert_invalid(&project);

    let mut project = valid_project();
    project.profiles = Some(Profiles::from_vec(vec![
        Profile::new("duplicate"),
        Profile::new("duplicate"),
    ]));
    assert_invalid(&project);

    let mut profile = Profile::new("dependencies");
    profile.dependencies = Some(Dependencies::from_vec(vec![Dependency::new("", "library")]));
    let mut project = valid_project();
    project.profiles = Some(Profiles::from_vec(vec![profile]));
    assert_invalid(&project);

    let mut profile = Profile::new("management");
    profile.dependency_management = Some(DependencyManagement::new(Dependencies::from_vec(vec![
        Dependency::new("", "managed"),
    ])));
    let mut project = valid_project();
    project.profiles = Some(Profiles::from_vec(vec![profile]));
    assert_invalid(&project);

    for repositories in [
        Repositories::from_vec(vec![Repository::new("", "https://repo.example")]),
        Repositories::from_vec(vec![Repository::new("repo", "")]),
        Repositories::from_vec(vec![
            Repository::new("repo", "https://one.example"),
            Repository::new("repo", "https://two.example"),
        ]),
    ] {
        let mut project = valid_project();
        project.repositories = Some(repositories);
        assert_invalid(&project);
    }

    let mut profile = Profile::new("repositories");
    profile.repositories = Some(Repositories::from_vec(vec![Repository::new("", "url")]));
    let mut project = valid_project();
    project.profiles = Some(Profiles::from_vec(vec![profile]));
    assert_invalid(&project);

    let mut profile = Profile::new("plugin-repositories");
    profile.plugin_repositories = Some(Repositories::from_vec(vec![Repository::new("", "url")]));
    let mut project = valid_project();
    project.profiles = Some(Profiles::from_vec(vec![profile]));
    assert_invalid(&project);

    let invalid_extension: Build = quick_xml::de::from_str(
        "<build><extensions><extension><groupId/><artifactId>extension</artifactId>\
         <version>1</version></extension></extensions></build>",
    )
    .expect("build extension");
    let mut project = valid_project();
    project.build = Some(invalid_extension);
    assert_invalid(&project);

    for extension in [
        "<build><extensions><extension><groupId>com.example</groupId><artifactId/>\
         <version>1</version></extension></extensions></build>",
        "<build><extensions><extension><groupId>com.example</groupId>\
         <artifactId>extension</artifactId><version/></extension></extensions></build>",
    ] {
        let mut project = valid_project();
        project.build =
            Some(quick_xml::de::from_str(extension).expect("invalid build extension fixture"));
        assert_invalid(&project);
    }

    let mut project = valid_project();
    project.build = Some(
        quick_xml::de::from_str(
            "<build><extensions><extension><groupId>com.example</groupId>\
             <artifactId>extension</artifactId><version>1</version></extension>\
             </extensions></build>",
        )
        .expect("valid build extension fixture"),
    );
    assert!(project.validate().is_ok());

    let invalid_plugin: Build =
        quick_xml::de::from_str("<build><plugins><plugin><artifactId/></plugin></plugins></build>")
            .expect("empty plugin");
    let mut project = valid_project();
    project.build = Some(invalid_plugin);
    assert_invalid(&project);

    let invalid_plugin_dependency: Build = quick_xml::de::from_str(
        "<build><plugins><plugin><artifactId>plugin</artifactId><dependencies><dependency>\
         <groupId/><artifactId>library</artifactId></dependency></dependencies></plugin>\
         </plugins></build>",
    )
    .expect("plugin dependency");
    let mut project = valid_project();
    project.build = Some(invalid_plugin_dependency);
    assert_invalid(&project);

    let invalid_report: Reporting = quick_xml::de::from_str(
        "<reporting><plugins><plugin><artifactId/></plugin></plugins></reporting>",
    )
    .expect("empty report plugin");
    let mut project = valid_project();
    project.reporting = Some(invalid_report);
    assert_invalid(&project);
}

#[test]
fn profiles_and_effective_parent_models_are_validated() {
    let invalid_profile_dependency = Dependency::builder("com.example", "system")
        .system_path("/tmp/system.jar")
        .build();
    let mut profile = Profile::new("dependencies");
    profile.dependencies = Some(Dependencies::from_vec(vec![invalid_profile_dependency]));
    let mut project = valid_project();
    project.profiles = Some(Profiles::from_vec(vec![profile]));
    assert_invalid(&project);

    let mut profile = Profile::new("management");
    profile.dependency_management = Some(DependencyManagement::new(Dependencies::from_vec(vec![
        Dependency::builder("com.example", "bom")
            .version("1")
            .scope(DependencyScope::Import)
            .build(),
    ])));
    let mut project = valid_project();
    project.profiles = Some(Profiles::from_vec(vec![profile]));
    assert_invalid(&project);

    let mut profile = Profile::new("management-version");
    profile.dependency_management = Some(DependencyManagement::new(Dependencies::from_vec(vec![
        Dependency::builder("com.example", "bom")
            .r#type("pom")
            .scope(DependencyScope::Import)
            .build(),
    ])));
    let mut project = valid_project();
    project.profiles = Some(Profiles::from_vec(vec![profile]));
    assert_invalid(&project);

    assert!(valid_project().validate_effective(None).is_ok());
    let child = Project::builder("child")
        .parent(Parent::new("com.example", "parent", "1"))
        .build_project();
    assert!(child.validate_effective(None).is_err());
    assert!(
        valid_project()
            .validate_effective(Some(&valid_project()))
            .is_err()
    );

    let inherited_parent = Project::builder("parent")
        .parent(Parent::new("com.example", "grandparent", "1"))
        .build_project();
    assert!(child.validate_effective(Some(&inherited_parent)).is_ok());

    let ranged_child = Project::builder("child")
        .parent(Parent::new("com.example", "parent", "[1,2)"))
        .build_project();
    let resolved_parent = Project::new("com.example", "parent", "1.5");
    assert!(
        ranged_child
            .validate_effective(Some(&resolved_parent))
            .is_ok()
    );
}

#[test]
fn typed_dependency_and_repository_policy_values_are_validated() {
    assert!(
        quick_xml::de::from_str::<Dependency>(
            "<dependency><groupId>com.example</groupId><artifactId>library</artifactId>\
             <scope>unknown</scope></dependency>"
        )
        .is_err()
    );
    assert!(
        quick_xml::de::from_str::<RepositoryPolicy>(
            "<releases><checksumPolicy>unknown</checksumPolicy></releases>"
        )
        .is_err()
    );
    assert!(
        quick_xml::de::from_str::<RepositoryPolicy>(
            "<releases><updatePolicy>interval:invalid</updatePolicy></releases>"
        )
        .is_err()
    );
    assert!(
        quick_xml::de::from_str::<UpdatePolicy>("<updatePolicy>sometimes</updatePolicy>").is_err()
    );
}
