use ristretto_pom::{DependencyScope, Project, Result};
use std::path::PathBuf;

#[test]
fn test_mockito_core() -> Result<()> {
    let path = PathBuf::from("tests/fixtures/mockito-core.pom");
    let project = Project::from_file(&path)?;

    assert_eq!(project.group_id.as_deref(), Some("org.mockito"));
    assert_eq!(project.artifact_id, "mockito-core");
    assert_eq!(project.version.as_deref(), Some("5.8.0"));
    assert_eq!(project.name.as_deref(), Some("mockito-core"));
    assert_eq!(
        project.url.as_deref(),
        Some("https://github.com/mockito/mockito")
    );

    assert!(project.licenses.is_some());
    let licenses = project.licenses.unwrap();
    assert_eq!(licenses.licenses.len(), 1);
    assert_eq!(licenses.licenses[0].name, "MIT");

    assert!(project.developers.is_some());
    let developers = project.developers.unwrap();
    assert_eq!(developers.developers.len(), 4);

    assert!(project.scm.is_some());
    let scm = project.scm.unwrap();
    assert_eq!(
        scm.url.as_deref(),
        Some("https://github.com/mockito/mockito.git")
    );

    assert_eq!(project.dependencies.dependencies.len(), 3);
    let dep = &project.dependencies.dependencies[0];
    assert_eq!(dep.group_id, "net.bytebuddy");
    assert_eq!(dep.artifact_id, "byte-buddy");
    assert_eq!(dep.version.as_deref(), Some("1.14.10"));
    assert_eq!(dep.scope, Some(DependencyScope::Compile));

    Ok(())
}

#[test]
fn test_checker_qual() -> Result<()> {
    let path = PathBuf::from("tests/fixtures/checker-qual.pom");
    let project = Project::from_file(&path)?;

    assert_eq!(project.group_id.as_deref(), Some("org.checkerframework"));
    assert_eq!(project.artifact_id, "checker-qual");
    assert_eq!(project.version.as_deref(), Some("3.42.0"));
    assert_eq!(project.name.as_deref(), Some("Checker Qual"));

    assert!(project.licenses.is_some());
    assert_eq!(
        project.licenses.unwrap().licenses[0].name,
        "The MIT License"
    );

    assert!(project.developers.is_some());
    assert_eq!(project.developers.unwrap().developers.len(), 2);

    assert!(project.scm.is_some());
    let scm = project.scm.unwrap();
    assert_eq!(
        scm.url.as_deref(),
        Some("https://github.com/typetools/checker-framework.git")
    );

    Ok(())
}

#[test]
fn test_jsoup() -> Result<()> {
    let path = PathBuf::from("tests/fixtures/jsoup.pom");
    let project = Project::from_file(&path)?;

    assert_eq!(project.group_id.as_deref(), Some("org.jsoup"));
    assert_eq!(project.artifact_id, "jsoup");
    assert_eq!(project.version.as_deref(), Some("1.17.1"));
    assert_eq!(project.inception_year.as_deref(), Some("2009"));

    assert!(project.licenses.is_some());
    assert_eq!(
        project.licenses.unwrap().licenses[0].name,
        "The MIT License"
    );

    assert!(project.organization.is_some());

    assert!(project.build.is_some());
    let build = project.build.unwrap();
    assert!(build.plugins.is_some());
    assert!(!build.plugins.unwrap().plugins.is_empty());

    assert!(project.profiles.is_some());
    let profiles = project.profiles.unwrap();
    assert_eq!(profiles.profiles.len(), 3);

    assert!(!project.dependencies.dependencies.is_empty());
    assert_eq!(project.dependencies.dependencies.len(), 6);

    Ok(())
}
