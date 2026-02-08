use ristretto_pom::{DependencyScope, Project, Result};
use std::fs;
use std::io::Write;
use tempfile::NamedTempFile;

/// Helper to perform round-trip test: parse -> serialize -> parse again
fn round_trip_pom(xml: &str) -> Result<(Project, Project)> {
    // First parse
    let mut file1 = NamedTempFile::new()?;
    file1.write_all(xml.as_bytes())?;
    let project1 = Project::from_file(file1.path())?;

    // Serialize back
    let file2 = NamedTempFile::new()?;
    project1.to_file(file2.path())?;

    // Parse again
    let project2 = Project::from_file(file2.path())?;

    Ok((project1, project2))
}

/// Verify two projects are semantically equal
fn assert_projects_equal(p1: &Project, p2: &Project) {
    assert_eq!(p1.model_version, p2.model_version, "model_version mismatch");
    assert_eq!(p1.group_id, p2.group_id, "group_id mismatch");
    assert_eq!(p1.artifact_id, p2.artifact_id, "artifact_id mismatch");
    assert_eq!(p1.version, p2.version, "version mismatch");
    assert_eq!(p1.packaging, p2.packaging, "packaging mismatch");
    assert_eq!(p1.name, p2.name, "name mismatch");
    assert_eq!(p1.description, p2.description, "description mismatch");
    assert_eq!(p1.url, p2.url, "url mismatch");
    assert_eq!(p1.properties, p2.properties, "properties mismatch");
    assert_eq!(
        p1.dependencies.dependencies.len(),
        p2.dependencies.dependencies.len(),
        "dependencies count mismatch"
    );
}

#[test]
fn test_round_trip_minimal_pom() -> Result<()> {
    let xml = r"
<project>
    <modelVersion>4.0.0</modelVersion>
    <groupId>com.example</groupId>
    <artifactId>minimal-app</artifactId>
    <version>1.0.0</version>
</project>
";
    let (p1, p2) = round_trip_pom(xml)?;
    assert_projects_equal(&p1, &p2);
    assert_eq!(p1, p2);
    Ok(())
}

#[test]
fn test_round_trip_with_packaging() -> Result<()> {
    let xml = r"
<project>
    <modelVersion>4.0.0</modelVersion>
    <groupId>com.example</groupId>
    <artifactId>war-app</artifactId>
    <version>1.0.0</version>
    <packaging>war</packaging>
</project>
";
    let (p1, p2) = round_trip_pom(xml)?;
    assert_eq!(p1.packaging, Some("war".to_string()));
    assert_eq!(p1, p2);
    Ok(())
}

#[test]
fn test_round_trip_with_parent() -> Result<()> {
    let xml = r"
<project>
    <modelVersion>4.0.0</modelVersion>
    <parent>
        <groupId>com.example.parent</groupId>
        <artifactId>parent-pom</artifactId>
        <version>2.0.0</version>
        <relativePath>../parent/pom.xml</relativePath>
    </parent>
    <artifactId>child-module</artifactId>
</project>
";
    let (p1, p2) = round_trip_pom(xml)?;
    assert!(p1.parent.is_some());
    let parent = p1.parent.as_ref().unwrap();
    assert_eq!(parent.group_id, "com.example.parent");
    assert_eq!(parent.artifact_id, "parent-pom");
    assert_eq!(parent.version, "2.0.0");
    assert_eq!(parent.relative_path, Some("../parent/pom.xml".to_string()));
    assert_eq!(p1, p2);
    Ok(())
}

#[test]
fn test_round_trip_with_dependencies() -> Result<()> {
    let xml = r"
<project>
    <modelVersion>4.0.0</modelVersion>
    <groupId>com.example</groupId>
    <artifactId>deps-app</artifactId>
    <version>1.0.0</version>
    <dependencies>
        <dependency>
            <groupId>org.junit.jupiter</groupId>
            <artifactId>junit-jupiter</artifactId>
            <version>5.9.0</version>
            <scope>test</scope>
        </dependency>
        <dependency>
            <groupId>org.slf4j</groupId>
            <artifactId>slf4j-api</artifactId>
            <version>2.0.0</version>
        </dependency>
        <dependency>
            <groupId>com.google.guava</groupId>
            <artifactId>guava</artifactId>
            <version>31.1-jre</version>
            <optional>true</optional>
        </dependency>
    </dependencies>
</project>
";
    let (p1, p2) = round_trip_pom(xml)?;
    assert_eq!(p1.dependencies.dependencies.len(), 3);

    let junit = &p1.dependencies.dependencies[0];
    assert_eq!(junit.group_id, "org.junit.jupiter");
    assert_eq!(junit.artifact_id, "junit-jupiter");
    assert_eq!(junit.scope, Some(DependencyScope::Test));

    let guava = &p1.dependencies.dependencies[2];
    assert!(guava.optional);

    assert_eq!(p1, p2);
    Ok(())
}

#[test]
fn test_round_trip_with_exclusions() -> Result<()> {
    let xml = r"
<project>
    <modelVersion>4.0.0</modelVersion>
    <groupId>com.example</groupId>
    <artifactId>exclusions-app</artifactId>
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
                <exclusion>
                    <groupId>org.springframework</groupId>
                    <artifactId>spring-jcl</artifactId>
                </exclusion>
            </exclusions>
        </dependency>
    </dependencies>
</project>
";
    let (p1, p2) = round_trip_pom(xml)?;
    let dep = &p1.dependencies.dependencies[0];
    assert!(dep.exclusions.is_some());
    let exclusions = dep.exclusions.as_ref().unwrap();
    assert_eq!(exclusions.exclusions.len(), 2);
    assert_eq!(p1, p2);
    Ok(())
}

#[test]
fn test_round_trip_with_dependency_management() -> Result<()> {
    let xml = r"
<project>
    <modelVersion>4.0.0</modelVersion>
    <groupId>com.example</groupId>
    <artifactId>bom-project</artifactId>
    <version>1.0.0</version>
    <packaging>pom</packaging>
    <dependencyManagement>
        <dependencies>
            <dependency>
                <groupId>org.springframework</groupId>
                <artifactId>spring-framework-bom</artifactId>
                <version>5.3.0</version>
                <type>pom</type>
                <scope>import</scope>
            </dependency>
            <dependency>
                <groupId>com.fasterxml.jackson</groupId>
                <artifactId>jackson-bom</artifactId>
                <version>2.14.0</version>
                <type>pom</type>
                <scope>import</scope>
            </dependency>
        </dependencies>
    </dependencyManagement>
</project>
";
    let (p1, p2) = round_trip_pom(xml)?;
    assert!(p1.dependency_management.is_some());
    let dm = p1.dependency_management.as_ref().unwrap();
    assert_eq!(dm.dependencies.dependencies.len(), 2);
    assert_eq!(p1, p2);
    Ok(())
}

#[test]
fn test_round_trip_with_build() -> Result<()> {
    let xml = r"
<project>
    <modelVersion>4.0.0</modelVersion>
    <groupId>com.example</groupId>
    <artifactId>build-app</artifactId>
    <version>1.0.0</version>
    <build>
        <sourceDirectory>src/main/java</sourceDirectory>
        <testSourceDirectory>src/test/java</testSourceDirectory>
        <outputDirectory>target/classes</outputDirectory>
        <finalName>my-application</finalName>
        <plugins>
            <plugin>
                <groupId>org.apache.maven.plugins</groupId>
                <artifactId>maven-compiler-plugin</artifactId>
                <version>3.11.0</version>
                <configuration>
                    <source>17</source>
                    <target>17</target>
                </configuration>
            </plugin>
        </plugins>
    </build>
</project>
";
    let (p1, p2) = round_trip_pom(xml)?;
    assert!(p1.build.is_some());
    let build = p1.build.as_ref().unwrap();
    assert_eq!(build.source_directory, Some("src/main/java".to_string()));
    assert_eq!(build.final_name, Some("my-application".to_string()));
    assert!(build.plugins.is_some());
    assert_eq!(p1, p2);
    Ok(())
}

#[test]
fn test_round_trip_with_plugin_executions() -> Result<()> {
    let xml = r"
<project>
    <modelVersion>4.0.0</modelVersion>
    <groupId>com.example</groupId>
    <artifactId>exec-app</artifactId>
    <version>1.0.0</version>
    <build>
        <plugins>
            <plugin>
                <groupId>org.apache.maven.plugins</groupId>
                <artifactId>maven-resources-plugin</artifactId>
                <version>3.3.0</version>
                <executions>
                    <execution>
                        <id>copy-resources</id>
                        <phase>validate</phase>
                        <goals>
                            <goal>copy-resources</goal>
                        </goals>
                    </execution>
                    <execution>
                        <id>copy-test-resources</id>
                        <phase>generate-test-resources</phase>
                        <goals>
                            <goal>testResources</goal>
                        </goals>
                    </execution>
                </executions>
            </plugin>
        </plugins>
    </build>
</project>
";
    let (p1, p2) = round_trip_pom(xml)?;
    let build = p1.build.as_ref().unwrap();
    let plugin = &build.plugins.as_ref().unwrap().plugins[0];
    assert!(plugin.executions.is_some());
    assert_eq!(plugin.executions.as_ref().unwrap().executions.len(), 2);
    assert_eq!(p1, p2);
    Ok(())
}

#[test]
fn test_round_trip_with_resources() -> Result<()> {
    let xml = r"
<project>
    <modelVersion>4.0.0</modelVersion>
    <groupId>com.example</groupId>
    <artifactId>resources-app</artifactId>
    <version>1.0.0</version>
    <build>
        <resources>
            <resource>
                <directory>src/main/resources</directory>
                <filtering>true</filtering>
                <includes>
                    <include>**/*.properties</include>
                    <include>**/*.xml</include>
                </includes>
                <excludes>
                    <exclude>**/*.bak</exclude>
                </excludes>
            </resource>
        </resources>
        <testResources>
            <testResource>
                <directory>src/test/resources</directory>
            </testResource>
        </testResources>
    </build>
</project>
";
    let (p1, p2) = round_trip_pom(xml)?;
    let build = p1.build.as_ref().unwrap();
    assert!(build.resources.is_some());
    assert!(build.test_resources.is_some());
    let resource = &build.resources.as_ref().unwrap().resources[0];
    assert!(resource.filtering);
    assert!(resource.includes.is_some());
    assert_eq!(p1, p2);
    Ok(())
}

#[test]
fn test_round_trip_with_profiles() -> Result<()> {
    let xml = r"
<project>
    <modelVersion>4.0.0</modelVersion>
    <groupId>com.example</groupId>
    <artifactId>profiles-app</artifactId>
    <version>1.0.0</version>
    <profiles>
        <profile>
            <id>development</id>
            <activation>
                <activeByDefault>true</activeByDefault>
            </activation>
            <properties>
                <env>dev</env>
            </properties>
        </profile>
        <profile>
            <id>production</id>
            <activation>
                <property>
                    <name>env</name>
                    <value>prod</value>
                </property>
            </activation>
            <properties>
                <env>prod</env>
            </properties>
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
            <id>file-based</id>
            <activation>
                <file>
                    <exists>build.properties</exists>
                </file>
            </activation>
        </profile>
    </profiles>
</project>
";
    let (p1, p2) = round_trip_pom(xml)?;
    assert!(p1.profiles.is_some());
    let profiles = p1.profiles.as_ref().unwrap();
    assert_eq!(profiles.profiles.len(), 5);

    // Check activeByDefault
    let dev = &profiles.profiles[0];
    assert!(dev.activation.as_ref().unwrap().active_by_default);

    // Check property activation
    let prod = &profiles.profiles[1];
    assert!(prod.activation.as_ref().unwrap().property.is_some());

    // Check JDK activation
    let jdk17 = &profiles.profiles[2];
    assert_eq!(
        jdk17.activation.as_ref().unwrap().jdk,
        Some("17".to_string())
    );

    // Check OS activation
    let windows = &profiles.profiles[3];
    assert!(windows.activation.as_ref().unwrap().os.is_some());

    // Check file activation
    let file_based = &profiles.profiles[4];
    assert!(file_based.activation.as_ref().unwrap().file.is_some());

    assert_eq!(p1, p2);
    Ok(())
}

#[test]
fn test_round_trip_with_scm() -> Result<()> {
    let xml = r"
<project>
    <modelVersion>4.0.0</modelVersion>
    <groupId>com.example</groupId>
    <artifactId>scm-app</artifactId>
    <version>1.0.0</version>
    <scm>
        <connection>scm:git:git://github.com/example/repo.git</connection>
        <developerConnection>scm:git:ssh://github.com/example/repo.git</developerConnection>
        <url>https://github.com/example/repo</url>
        <tag>v1.0.0</tag>
    </scm>
    <issueManagement>
        <system>GitHub</system>
        <url>https://github.com/example/repo/issues</url>
    </issueManagement>
    <ciManagement>
        <system>GitHub Actions</system>
        <url>https://github.com/example/repo/actions</url>
    </ciManagement>
</project>
";
    let (p1, p2) = round_trip_pom(xml)?;
    assert!(p1.scm.is_some());
    assert!(p1.issue_management.is_some());
    assert!(p1.ci_management.is_some());
    assert_eq!(p1.scm.as_ref().unwrap().tag, Some("v1.0.0".to_string()));
    assert_eq!(p1, p2);
    Ok(())
}

#[test]
fn test_round_trip_with_distribution_management() -> Result<()> {
    let xml = r"
<project>
    <modelVersion>4.0.0</modelVersion>
    <groupId>com.example</groupId>
    <artifactId>dist-app</artifactId>
    <version>1.0.0</version>
    <distributionManagement>
        <repository>
            <id>releases</id>
            <url>https://repo.example.com/releases</url>
        </repository>
        <snapshotRepository>
            <id>snapshots</id>
            <url>https://repo.example.com/snapshots</url>
        </snapshotRepository>
        <site>
            <id>site</id>
            <url>https://site.example.com</url>
        </site>
        <downloadUrl>https://download.example.com</downloadUrl>
    </distributionManagement>
</project>
";
    let (p1, p2) = round_trip_pom(xml)?;
    assert!(p1.distribution_management.is_some());
    let dm = p1.distribution_management.as_ref().unwrap();
    assert!(dm.repository.is_some());
    assert!(dm.snapshot_repository.is_some());
    assert!(dm.site.is_some());
    assert_eq!(p1, p2);
    Ok(())
}

#[test]
fn test_round_trip_with_developers() -> Result<()> {
    let xml = r"
<project>
    <modelVersion>4.0.0</modelVersion>
    <groupId>com.example</groupId>
    <artifactId>devs-app</artifactId>
    <version>1.0.0</version>
    <developers>
        <developer>
            <id>jdoe</id>
            <name>John Doe</name>
            <email>john@example.com</email>
            <url>https://johndoe.com</url>
            <organization>Example Corp</organization>
            <organizationUrl>https://example.com</organizationUrl>
            <roles>
                <role>Lead Developer</role>
                <role>Architect</role>
            </roles>
            <timezone>America/New_York</timezone>
        </developer>
        <developer>
            <id>jsmith</id>
            <name>Jane Smith</name>
            <email>jane@example.com</email>
        </developer>
    </developers>
    <contributors>
        <contributor>
            <name>Bob Wilson</name>
            <email>bob@example.com</email>
            <roles>
                <role>Tester</role>
            </roles>
        </contributor>
    </contributors>
</project>
";
    let (p1, p2) = round_trip_pom(xml)?;
    assert!(p1.developers.is_some());
    assert!(p1.contributors.is_some());
    let devs = p1.developers.as_ref().unwrap();
    assert_eq!(devs.developers.len(), 2);
    let jdoe = &devs.developers[0];
    assert!(jdoe.roles.is_some());
    assert_eq!(jdoe.roles.as_ref().unwrap().roles.len(), 2);
    assert_eq!(p1, p2);
    Ok(())
}

#[test]
fn test_round_trip_with_organization() -> Result<()> {
    let xml = r"
<project>
    <modelVersion>4.0.0</modelVersion>
    <groupId>com.example</groupId>
    <artifactId>org-app</artifactId>
    <version>1.0.0</version>
    <organization>
        <name>Example Corporation</name>
        <url>https://example.com</url>
    </organization>
    <licenses>
        <license>
            <name>Apache License, Version 2.0</name>
            <url>https://www.apache.org/licenses/LICENSE-2.0</url>
            <distribution>repo</distribution>
            <comments>A permissive license</comments>
        </license>
    </licenses>
</project>
";
    let (p1, p2) = round_trip_pom(xml)?;
    assert!(p1.organization.is_some());
    assert!(p1.licenses.is_some());
    let org = p1.organization.as_ref().unwrap();
    assert_eq!(org.name, "Example Corporation");
    assert_eq!(p1, p2);
    Ok(())
}

#[test]
fn test_round_trip_with_modules() -> Result<()> {
    let xml = r"
<project>
    <modelVersion>4.0.0</modelVersion>
    <groupId>com.example</groupId>
    <artifactId>parent-project</artifactId>
    <version>1.0.0</version>
    <packaging>pom</packaging>
    <modules>
        <module>module-core</module>
        <module>module-api</module>
        <module>module-web</module>
        <module>module-tests</module>
    </modules>
</project>
";
    let (p1, p2) = round_trip_pom(xml)?;
    assert!(p1.modules.is_some());
    let modules = p1.modules.as_ref().unwrap();
    assert_eq!(modules.modules.len(), 4);
    assert_eq!(modules.modules[0], "module-core");
    assert_eq!(p1, p2);
    Ok(())
}

#[test]
fn test_round_trip_with_reporting() -> Result<()> {
    let xml = r"
<project>
    <modelVersion>4.0.0</modelVersion>
    <groupId>com.example</groupId>
    <artifactId>reporting-app</artifactId>
    <version>1.0.0</version>
    <reporting>
        <excludeDefaults>false</excludeDefaults>
        <outputDirectory>target/site</outputDirectory>
        <plugins>
            <plugin>
                <groupId>org.apache.maven.plugins</groupId>
                <artifactId>maven-javadoc-plugin</artifactId>
                <version>3.4.0</version>
            </plugin>
            <plugin>
                <groupId>org.apache.maven.plugins</groupId>
                <artifactId>maven-project-info-reports-plugin</artifactId>
                <version>3.4.0</version>
                <reportSets>
                    <reportSet>
                        <id>default</id>
                        <reports>
                            <report>dependencies</report>
                            <report>license</report>
                        </reports>
                    </reportSet>
                </reportSets>
            </plugin>
        </plugins>
    </reporting>
</project>
";
    let (p1, p2) = round_trip_pom(xml)?;
    assert!(p1.reporting.is_some());
    let reporting = p1.reporting.as_ref().unwrap();
    assert_eq!(reporting.output_directory, Some("target/site".to_string()));
    assert!(reporting.plugins.is_some());
    assert_eq!(reporting.plugins.as_ref().unwrap().plugins.len(), 2);
    assert_eq!(p1, p2);
    Ok(())
}

#[test]
fn test_round_trip_with_repositories() -> Result<()> {
    let xml = r"
<project>
    <modelVersion>4.0.0</modelVersion>
    <groupId>com.example</groupId>
    <artifactId>repos-app</artifactId>
    <version>1.0.0</version>
    <repositories>
        <repository>
            <id>central</id>
            <name>Maven Central</name>
            <url>https://repo.maven.apache.org/maven2</url>
            <releases>
                <enabled>true</enabled>
                <updatePolicy>daily</updatePolicy>
            </releases>
            <snapshots>
                <enabled>false</enabled>
            </snapshots>
        </repository>
        <repository>
            <id>jitpack</id>
            <url>https://jitpack.io</url>
        </repository>
    </repositories>
    <pluginRepositories>
        <repository>
            <id>plugin-central</id>
            <url>https://repo.maven.apache.org/maven2</url>
        </repository>
    </pluginRepositories>
</project>
";
    let (p1, p2) = round_trip_pom(xml)?;
    assert!(p1.repositories.is_some());
    assert!(p1.plugin_repositories.is_some());
    let repos = p1.repositories.as_ref().unwrap();
    assert_eq!(repos.repositories.len(), 2);
    let central = &repos.repositories[0];
    assert!(central.releases.is_some());
    assert!(central.snapshots.is_some());
    assert_eq!(p1, p2);
    Ok(())
}

#[test]
#[expect(clippy::too_many_lines)]
fn test_round_trip_comprehensive_pom() -> Result<()> {
    let xml = r"
<project>
    <modelVersion>4.0.0</modelVersion>

    <parent>
        <groupId>org.springframework.boot</groupId>
        <artifactId>spring-boot-starter-parent</artifactId>
        <version>3.1.0</version>
        <relativePath/>
    </parent>

    <groupId>com.example</groupId>
    <artifactId>comprehensive-app</artifactId>
    <version>1.0.0-SNAPSHOT</version>
    <packaging>jar</packaging>

    <name>Comprehensive Application</name>
    <description>A comprehensive example project</description>
    <url>https://example.com/comprehensive-app</url>
    <inceptionYear>2024</inceptionYear>

    <organization>
        <name>Example Corporation</name>
        <url>https://example.com</url>
    </organization>

    <licenses>
        <license>
            <name>Apache License, Version 2.0</name>
            <url>https://www.apache.org/licenses/LICENSE-2.0</url>
            <distribution>repo</distribution>
        </license>
    </licenses>

    <developers>
        <developer>
            <id>lead</id>
            <name>Lead Developer</name>
            <email>lead@example.com</email>
            <roles>
                <role>Project Lead</role>
                <role>Architect</role>
            </roles>
        </developer>
    </developers>

    <scm>
        <connection>scm:git:git://github.com/example/comprehensive-app.git</connection>
        <developerConnection>scm:git:ssh://github.com/example/comprehensive-app.git</developerConnection>
        <url>https://github.com/example/comprehensive-app</url>
        <tag>HEAD</tag>
    </scm>

    <issueManagement>
        <system>GitHub</system>
        <url>https://github.com/example/comprehensive-app/issues</url>
    </issueManagement>

    <ciManagement>
        <system>GitHub Actions</system>
        <url>https://github.com/example/comprehensive-app/actions</url>
    </ciManagement>

    <properties>
        <java.version>17</java.version>
        <project.build.sourceEncoding>UTF-8</project.build.sourceEncoding>
        <junit.version>5.9.0</junit.version>
    </properties>

    <dependencyManagement>
        <dependencies>
            <dependency>
                <groupId>org.junit</groupId>
                <artifactId>junit-bom</artifactId>
                <version>${junit.version}</version>
                <type>pom</type>
                <scope>import</scope>
            </dependency>
        </dependencies>
    </dependencyManagement>

    <dependencies>
        <dependency>
            <groupId>org.springframework.boot</groupId>
            <artifactId>spring-boot-starter-web</artifactId>
        </dependency>
        <dependency>
            <groupId>org.springframework.boot</groupId>
            <artifactId>spring-boot-starter-test</artifactId>
            <scope>test</scope>
            <exclusions>
                <exclusion>
                    <groupId>org.junit.vintage</groupId>
                    <artifactId>junit-vintage-engine</artifactId>
                </exclusion>
            </exclusions>
        </dependency>
    </dependencies>

    <repositories>
        <repository>
            <id>spring-milestones</id>
            <name>Spring Milestones</name>
            <url>https://repo.spring.io/milestone</url>
            <snapshots>
                <enabled>false</enabled>
            </snapshots>
        </repository>
    </repositories>

    <build>
        <finalName>${project.artifactId}</finalName>
        <plugins>
            <plugin>
                <groupId>org.springframework.boot</groupId>
                <artifactId>spring-boot-maven-plugin</artifactId>
            </plugin>
            <plugin>
                <groupId>org.apache.maven.plugins</groupId>
                <artifactId>maven-compiler-plugin</artifactId>
                <version>3.11.0</version>
                <configuration>
                    <source>${java.version}</source>
                    <target>${java.version}</target>
                </configuration>
            </plugin>
        </plugins>
    </build>

    <profiles>
        <profile>
            <id>development</id>
            <activation>
                <activeByDefault>true</activeByDefault>
            </activation>
            <properties>
                <spring.profiles.active>dev</spring.profiles.active>
            </properties>
        </profile>
        <profile>
            <id>production</id>
            <properties>
                <spring.profiles.active>prod</spring.profiles.active>
            </properties>
            <build>
                <plugins>
                    <plugin>
                        <groupId>org.apache.maven.plugins</groupId>
                        <artifactId>maven-gpg-plugin</artifactId>
                        <version>3.1.0</version>
                    </plugin>
                </plugins>
            </build>
        </profile>
    </profiles>

    <distributionManagement>
        <repository>
            <id>releases</id>
            <url>https://repo.example.com/releases</url>
        </repository>
        <snapshotRepository>
            <id>snapshots</id>
            <url>https://repo.example.com/snapshots</url>
        </snapshotRepository>
    </distributionManagement>
</project>
";
    let (p1, p2) = round_trip_pom(xml)?;

    // Verify all major sections
    assert!(p1.parent.is_some());
    assert_eq!(p1.group_id, Some("com.example".to_string()));
    assert_eq!(p1.artifact_id, "comprehensive-app");
    assert_eq!(p1.packaging, Some("jar".to_string()));
    assert!(p1.organization.is_some());
    assert!(p1.licenses.is_some());
    assert!(p1.developers.is_some());
    assert!(p1.scm.is_some());
    assert!(p1.issue_management.is_some());
    assert!(p1.ci_management.is_some());
    assert!(!p1.properties.is_empty());
    assert!(p1.dependency_management.is_some());
    assert!(!p1.dependencies.dependencies.is_empty());
    assert!(p1.repositories.is_some());
    assert!(p1.build.is_some());
    assert!(p1.profiles.is_some());
    assert!(p1.distribution_management.is_some());

    // Verify round-trip equality
    assert_eq!(p1, p2);

    Ok(())
}

#[test]
fn test_round_trip_fixture_jsoup() -> Result<()> {
    let path = "tests/fixtures/jsoup.pom";
    if !std::path::Path::new(path).exists() {
        eprintln!("Skipping test: fixture file not found: {path}");
        return Ok(());
    }

    let xml = fs::read_to_string(path)?;
    let (p1, p2) = round_trip_pom(&xml)?;
    assert_projects_equal(&p1, &p2);
    assert_eq!(p1, p2);
    Ok(())
}

#[test]
fn test_round_trip_fixture_mockito() -> Result<()> {
    let path = "tests/fixtures/mockito-core.pom";
    if !std::path::Path::new(path).exists() {
        eprintln!("Skipping test: fixture file not found: {path}");
        return Ok(());
    }

    let xml = fs::read_to_string(path)?;
    let (p1, p2) = round_trip_pom(&xml)?;
    assert_projects_equal(&p1, &p2);
    assert_eq!(p1, p2);
    Ok(())
}

#[test]
fn test_round_trip_fixture_checker_qual() -> Result<()> {
    let path = "tests/fixtures/checker-qual.pom";
    if !std::path::Path::new(path).exists() {
        eprintln!("Skipping test: fixture file not found: {path}");
        return Ok(());
    }

    let xml = fs::read_to_string(path)?;
    let (p1, p2) = round_trip_pom(&xml)?;
    assert_projects_equal(&p1, &p2);
    assert_eq!(p1, p2);
    Ok(())
}

#[test]
fn test_round_trip_empty_dependencies() -> Result<()> {
    let xml = r"
<project>
    <modelVersion>4.0.0</modelVersion>
    <groupId>com.example</groupId>
    <artifactId>empty-deps</artifactId>
    <version>1.0.0</version>
    <dependencies></dependencies>
</project>
";
    let (p1, p2) = round_trip_pom(xml)?;
    assert!(p1.dependencies.dependencies.is_empty());
    assert_eq!(p1, p2);
    Ok(())
}

#[test]
fn test_round_trip_self_closing_dependencies() -> Result<()> {
    let xml = r"
<project>
    <modelVersion>4.0.0</modelVersion>
    <groupId>com.example</groupId>
    <artifactId>self-closing</artifactId>
    <version>1.0.0</version>
    <dependencies/>
</project>
";
    let (p1, p2) = round_trip_pom(xml)?;
    assert!(p1.dependencies.dependencies.is_empty());
    assert_eq!(p1, p2);
    Ok(())
}

#[test]
fn test_round_trip_empty_properties() -> Result<()> {
    let xml = r"
<project>
    <modelVersion>4.0.0</modelVersion>
    <groupId>com.example</groupId>
    <artifactId>no-props</artifactId>
    <version>1.0.0</version>
</project>
";
    let (p1, p2) = round_trip_pom(xml)?;
    assert!(p1.properties.is_empty());
    assert_eq!(p1, p2);
    Ok(())
}

#[test]
fn test_round_trip_many_properties() -> Result<()> {
    let xml = r"
<project>
    <modelVersion>4.0.0</modelVersion>
    <groupId>com.example</groupId>
    <artifactId>many-props</artifactId>
    <version>1.0.0</version>
    <properties>
        <java.version>17</java.version>
        <maven.compiler.source>17</maven.compiler.source>
        <maven.compiler.target>17</maven.compiler.target>
        <project.build.sourceEncoding>UTF-8</project.build.sourceEncoding>
        <project.reporting.outputEncoding>UTF-8</project.reporting.outputEncoding>
        <spring.version>5.3.0</spring.version>
        <junit.version>5.9.0</junit.version>
        <mockito.version>5.0.0</mockito.version>
    </properties>
</project>
";
    let (p1, p2) = round_trip_pom(xml)?;
    assert_eq!(p1.properties.len(), 8);
    assert_eq!(p1.properties.get("java.version"), Some(&"17".to_string()));
    assert_eq!(p1, p2);
    Ok(())
}
