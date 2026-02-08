//! Tests for POM validation functionality.

use ristretto_pom::{Error, Project, Result};
use std::io::Write;
use tempfile::NamedTempFile;

/// Helper to parse XML from a string
fn parse_xml(xml: &str) -> Result<Project> {
    let mut file = NamedTempFile::new()?;
    file.write_all(xml.as_bytes())?;
    Project::from_file(file.path())
}

#[test]
fn test_valid_minimal_pom() {
    let xml = r"
<project>
    <modelVersion>4.0.0</modelVersion>
    <groupId>com.example</groupId>
    <artifactId>my-app</artifactId>
    <version>1.0.0</version>
</project>
";
    let result = parse_xml(xml);
    assert!(result.is_ok());
}

#[test]
fn test_missing_group_id_without_parent() {
    let xml = r"
<project>
    <modelVersion>4.0.0</modelVersion>
    <artifactId>my-app</artifactId>
    <version>1.0.0</version>
</project>
";
    let result = parse_xml(xml);
    assert!(result.is_err());
    match result.unwrap_err() {
        Error::MissingField(field) => assert_eq!(field, "groupId"),
        e => panic!("Expected MissingField error, got: {e:?}"),
    }
}

#[test]
fn test_missing_version_without_parent() {
    let xml = r"
<project>
    <modelVersion>4.0.0</modelVersion>
    <groupId>com.example</groupId>
    <artifactId>my-app</artifactId>
</project>
";
    let result = parse_xml(xml);
    assert!(result.is_err());
    match result.unwrap_err() {
        Error::MissingField(field) => assert_eq!(field, "version"),
        e => panic!("Expected MissingField error, got: {e:?}"),
    }
}

#[test]
fn test_group_id_inherited_from_parent() {
    let xml = r"
<project>
    <modelVersion>4.0.0</modelVersion>
    <parent>
        <groupId>com.example.parent</groupId>
        <artifactId>parent-pom</artifactId>
        <version>1.0.0</version>
    </parent>
    <artifactId>my-app</artifactId>
</project>
";
    let result = parse_xml(xml);
    assert!(result.is_ok());
    let project = result.unwrap();
    assert!(project.group_id.is_none());
    assert!(project.parent.is_some());
}

#[test]
fn test_version_inherited_from_parent() {
    let xml = r"
<project>
    <modelVersion>4.0.0</modelVersion>
    <parent>
        <groupId>com.example.parent</groupId>
        <artifactId>parent-pom</artifactId>
        <version>1.0.0</version>
    </parent>
    <groupId>com.example</groupId>
    <artifactId>my-app</artifactId>
</project>
";
    let result = parse_xml(xml);
    assert!(result.is_ok());
    let project = result.unwrap();
    assert!(project.version.is_none());
}

#[test]
fn test_invalid_model_version() {
    let xml = r"
<project>
    <modelVersion>3.0.0</modelVersion>
    <groupId>com.example</groupId>
    <artifactId>my-app</artifactId>
    <version>1.0.0</version>
</project>
";
    let result = parse_xml(xml);
    assert!(result.is_err());
    match result.unwrap_err() {
        Error::InvalidModelVersion(v) => assert_eq!(v, "3.0.0"),
        e => panic!("Expected InvalidModelVersion error, got: {e:?}"),
    }
}

#[test]
fn test_empty_dependencies() {
    let xml = r"
<project>
    <modelVersion>4.0.0</modelVersion>
    <groupId>com.example</groupId>
    <artifactId>my-app</artifactId>
    <version>1.0.0</version>
    <dependencies/>
</project>
";
    let result = parse_xml(xml);
    assert!(result.is_ok());
    let project = result.unwrap();
    assert!(project.dependencies.dependencies.is_empty());
}

#[test]
fn test_empty_dependencies_full_tag() {
    let xml = r"
<project>
    <modelVersion>4.0.0</modelVersion>
    <groupId>com.example</groupId>
    <artifactId>my-app</artifactId>
    <version>1.0.0</version>
    <dependencies></dependencies>
</project>
";
    let result = parse_xml(xml);
    assert!(result.is_ok());
    let project = result.unwrap();
    assert!(project.dependencies.dependencies.is_empty());
}

#[test]
fn test_xml_round_trip() -> Result<()> {
    let original = Project::new("com.example", "my-app", "1.0.0");

    // Write to XML
    let file = NamedTempFile::new()?;
    original.to_file(file.path())?;

    // Read back
    let loaded = Project::from_file(file.path())?;

    assert_eq!(original, loaded);
    Ok(())
}

#[test]
fn test_from_reader() -> Result<()> {
    let xml = r"
<project>
    <modelVersion>4.0.0</modelVersion>
    <groupId>com.example</groupId>
    <artifactId>my-app</artifactId>
    <version>1.0.0</version>
</project>
";
    let project = Project::from_reader(xml.as_bytes())?;
    assert_eq!(project.artifact_id, "my-app");
    Ok(())
}

#[test]
fn test_to_writer() -> Result<()> {
    let project = Project::new("com.example", "my-app", "1.0.0");
    let mut buffer = Vec::new();
    project.to_writer(&mut buffer)?;

    let xml = String::from_utf8(buffer).unwrap();
    assert!(xml.contains("my-app"));
    assert!(xml.contains("com.example"));
    Ok(())
}

#[test]
fn test_profile_activation_parsing() {
    let xml = r"
<project>
    <modelVersion>4.0.0</modelVersion>
    <groupId>com.example</groupId>
    <artifactId>my-app</artifactId>
    <version>1.0.0</version>
    <profiles>
        <profile>
            <id>release</id>
            <activation>
                <activeByDefault>false</activeByDefault>
                <property>
                    <name>performRelease</name>
                    <value>true</value>
                </property>
            </activation>
        </profile>
        <profile>
            <id>jdk17</id>
            <activation>
                <jdk>17</jdk>
            </activation>
        </profile>
        <profile>
            <id>windows</id>
            <activation>
                <os>
                    <family>windows</family>
                </os>
            </activation>
        </profile>
        <profile>
            <id>file-exists</id>
            <activation>
                <file>
                    <exists>build.properties</exists>
                </file>
            </activation>
        </profile>
    </profiles>
</project>
";
    let result = parse_xml(xml);
    assert!(result.is_ok());
    let project = result.unwrap();

    let profiles = project.profiles.unwrap();
    assert_eq!(profiles.profiles.len(), 4);

    // Check property activation
    let release = &profiles.profiles[0];
    assert_eq!(release.id, "release");
    let activation = release.activation.as_ref().unwrap();
    assert!(!activation.active_by_default);
    let prop = activation.property.as_ref().unwrap();
    assert_eq!(prop.name, "performRelease");
    assert_eq!(prop.value, Some("true".to_string()));

    // Check JDK activation
    let jdk17 = &profiles.profiles[1];
    assert_eq!(
        jdk17.activation.as_ref().unwrap().jdk,
        Some("17".to_string())
    );

    // Check OS activation
    let windows = &profiles.profiles[2];
    let os = windows.activation.as_ref().unwrap().os.as_ref().unwrap();
    assert_eq!(os.family, Some("windows".to_string()));

    // Check file activation
    let file_exists = &profiles.profiles[3];
    let file = file_exists
        .activation
        .as_ref()
        .unwrap()
        .file
        .as_ref()
        .unwrap();
    assert_eq!(file.exists, Some("build.properties".to_string()));
}

#[test]
fn test_dependency_exclusions() {
    let xml = r"
<project>
    <modelVersion>4.0.0</modelVersion>
    <groupId>com.example</groupId>
    <artifactId>my-app</artifactId>
    <version>1.0.0</version>
    <dependencies>
        <dependency>
            <groupId>org.springframework</groupId>
            <artifactId>spring-core</artifactId>
            <version>5.3.0</version>
            <exclusions>
                <exclusion>
                    <groupId>commons-logging</groupId>
                    <artifactId>commons-logging</artifactId>
                </exclusion>
            </exclusions>
        </dependency>
    </dependencies>
</project>
";
    let result = parse_xml(xml);
    assert!(result.is_ok());
    let project = result.unwrap();

    let dep = &project.dependencies.dependencies[0];
    let exclusions = dep.exclusions.as_ref().unwrap();
    assert_eq!(exclusions.exclusions.len(), 1);
    assert_eq!(exclusions.exclusions[0].group_id, "commons-logging");
    assert_eq!(exclusions.exclusions[0].artifact_id, "commons-logging");
}

#[test]
fn test_dependency_scope_type() -> Result<()> {
    use ristretto_pom::DependencyScope;

    assert_eq!(DependencyScope::Compile.to_string(), "compile");
    assert_eq!(DependencyScope::Test.to_string(), "test");
    assert_eq!(DependencyScope::Runtime.to_string(), "runtime");
    assert_eq!(DependencyScope::Provided.to_string(), "provided");
    assert_eq!(DependencyScope::System.to_string(), "system");
    assert_eq!(DependencyScope::Import.to_string(), "import");

    let scope: DependencyScope = "test".parse()?;
    assert_eq!(scope, DependencyScope::Test);

    Ok(())
}

#[test]
fn test_packaging_type() -> Result<()> {
    use ristretto_pom::Packaging;

    assert_eq!(Packaging::Jar.to_string(), "jar");
    assert_eq!(Packaging::Pom.to_string(), "pom");
    assert_eq!(Packaging::War.to_string(), "war");

    let jar: Packaging = "jar".parse()?;
    assert_eq!(jar, Packaging::Jar);

    let custom: Packaging = "bundle".parse()?;
    assert_eq!(custom, Packaging::Other("bundle".to_string()));

    Ok(())
}
