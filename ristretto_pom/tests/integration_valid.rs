use ristretto_pom::{Project, Result};
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_valid_pom_parsing() -> Result<()> {
    let xml = r"
<project>
    <modelVersion>4.0.0</modelVersion>
    <groupId>com.example</groupId>
    <artifactId>valid-app</artifactId>
    <version>1.0.0</version>
</project>
";
    let mut file = NamedTempFile::new()?;
    file.write_all(xml.as_bytes())?;
    let project = Project::from_file(file.path())?;

    assert_eq!(project.group_id, Some("com.example".to_string()));
    assert_eq!(project.artifact_id, "valid-app");
    assert_eq!(project.version, Some("1.0.0".to_string()));
    Ok(())
}

#[test]
fn test_pom_serialization() -> Result<()> {
    let project = Project::new("com.test", "test-artifact", "0.0.1");

    let file = NamedTempFile::new()?;
    project.to_file(file.path())?;

    let content = std::fs::read_to_string(file.path())?;
    println!("Serialized XML: {content}");

    let loaded = Project::from_file(file.path())?;
    assert_eq!(project, loaded);
    Ok(())
}

#[test]
fn test_project_builder() {
    let project = Project::builder("my-artifact")
        .group_id("com.example")
        .version("1.0.0")
        .name("My Project")
        .description("A test project")
        .property("java.version", "17")
        .build_project();

    assert_eq!(project.group_id, Some("com.example".to_string()));
    assert_eq!(project.artifact_id, "my-artifact");
    assert_eq!(project.version, Some("1.0.0".to_string()));
    assert_eq!(project.name, Some("My Project".to_string()));
    assert_eq!(
        project.properties.get("java.version"),
        Some(&"17".to_string())
    );
}
