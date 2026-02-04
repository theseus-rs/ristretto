use ristretto_pom::{Project, Result};
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
#[expect(clippy::too_many_lines)]
fn test_comprehensive_pom() -> Result<()> {
    let xml = r#"
<project xmlns="http://maven.apache.org/POM/4.0.0">
    <modelVersion>4.0.0</modelVersion>
    <parent>
        <groupId>com.example.parent</groupId>
        <artifactId>parent-pom</artifactId>
        <version>1.0.0</version>
        <relativePath>../parent/pom.xml</relativePath>
    </parent>
    <groupId>com.example</groupId>
    <artifactId>comprehensive-project</artifactId>
    <version>1.2.3</version>
    <packaging>jar</packaging>
    <name>Comprehensive Project</name>
    <description>A project verifying all fields</description>
    <url>https://example.com</url>
    <inceptionYear>2024</inceptionYear>

    <organization>
        <name>Example Org</name>
        <url>https://example.org</url>
    </organization>

    <licenses>
        <license>
            <name>MIT</name>
            <url>https://opensource.org/licenses/MIT</url>
            <distribution>repo</distribution>
            <comments>A permissive license</comments>
        </license>
    </licenses>

    <developers>
        <developer>
            <id>dev1</id>
            <name>Developer One</name>
            <email>dev1@example.com</email>
            <url>https://dev1.com</url>
            <organization>Org 1</organization>
            <organizationUrl>https://org1.com</organizationUrl>
            <roles>
                <role>Architect</role>
                <role>Developer</role>
            </roles>
            <timezone>UTC</timezone>
            <properties>
                <prop1>val1</prop1>
            </properties>
        </developer>
    </developers>

    <contributors>
        <contributor>
            <name>Contributor One</name>
            <roles><role>Tester</role></roles>
        </contributor>
    </contributors>

    <mailingLists>
        <mailingList>
            <name>Users</name>
            <subscribe>users-subscribe@example.com</subscribe>
            <unsubscribe>users-unsubscribe@example.com</unsubscribe>
            <post>users@example.com</post>
            <archive>https://lists.example.com/users</archive>
        </mailingList>
    </mailingLists>

    <prerequisites>
        <maven>3.6.0</maven>
    </prerequisites>

    <modules>
        <module>module-a</module>
        <module>module-b</module>
    </modules>

    <scm>
        <connection>scm:git:git://github.com/example/repo.git</connection>
        <developerConnection>scm:git:ssh://github.com/example/repo.git</developerConnection>
        <tag>HEAD</tag>
        <url>https://github.com/example/repo</url>
    </scm>

    <issueManagement>
        <system>GitHub</system>
        <url>https://github.com/example/repo/issues</url>
    </issueManagement>

    <ciManagement>
        <system>GitHub Actions</system>
        <url>https://github.com/example/repo/actions</url>
        <notifiers>
            <notifier>
                <type>mail</type>
                <sendOnError>true</sendOnError>
                <sendOnFailure>true</sendOnFailure>
                <sendOnSuccess>false</sendOnSuccess>
                <sendOnWarning>false</sendOnWarning>
                <address>ci@example.com</address>
                <configuration>
                    <key>val</key>
                </configuration>
            </notifier>
        </notifiers>
    </ciManagement>

    <distributionManagement>
        <repository>
            <id>releases</id>
            <name>Release Repo</name>
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
        <downloadUrl>https://dl.example.com</downloadUrl>
        <relocation>
            <groupId>com.new</groupId>
        </relocation>
        <status>deployed</status>
    </distributionManagement>

    <properties>
        <java.version>17</java.version>
        <project.build.sourceEncoding>UTF-8</project.build.sourceEncoding>
    </properties>

    <dependencyManagement>
        <dependencies>
            <dependency>
                <groupId>org.springframework</groupId>
                <artifactId>spring-framework-bom</artifactId>
                <version>6.0.0</version>
                <type>pom</type>
                <scope>import</scope>
            </dependency>
        </dependencies>
    </dependencyManagement>

    <dependencies>
        <dependency>
            <groupId>org.slf4j</groupId>
            <artifactId>slf4j-api</artifactId>
            <version>1.7.32</version>
        </dependency>
        <dependency>
            <groupId>org.junit.jupiter</groupId>
            <artifactId>junit-jupiter</artifactId>
            <version>5.8.1</version>
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
            <id>central</id>
            <url>https://repo.maven.apache.org/maven2</url>
            <snapshots>
                <enabled>false</enabled>
            </snapshots>
        </repository>
    </repositories>

    <pluginRepositories>
        <repository>
            <id>plugins</id>
            <url>https://plugins.example.com</url>
        </repository>
    </pluginRepositories>

    <build>
        <sourceDirectory>src/main/java</sourceDirectory>
        <testSourceDirectory>src/test/java</testSourceDirectory>
        <finalName>comprehensive-app</finalName>
        <plugins>
            <plugin>
                <groupId>org.apache.maven.plugins</groupId>
                <artifactId>maven-compiler-plugin</artifactId>
                <version>3.8.1</version>
                <configuration>
                    <source>17</source>
                    <target>17</target>
                </configuration>
            </plugin>
        </plugins>
    </build>

    <reporting>
        <excludeDefaults>true</excludeDefaults>
        <outputDirectory>target/site</outputDirectory>
    </reporting>
    
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
            <build>
                <plugins>
                    <plugin>
                        <groupId>org.apache.maven.plugins</groupId>
                        <artifactId>maven-gpg-plugin</artifactId>
                        <version>1.6</version>
                    </plugin>
                </plugins>
            </build>
        </profile>
    </profiles>
</project>
"#;

    let mut file = NamedTempFile::new()?;
    file.write_all(xml.as_bytes())?;
    let project = Project::from_file(file.path())?;

    assert_eq!(project.group_id.as_deref(), Some("com.example"));
    assert_eq!(project.artifact_id, "comprehensive-project");
    assert_eq!(project.version.as_deref(), Some("1.2.3"));
    assert_eq!(project.packaging.as_deref(), Some("jar"));
    assert_eq!(project.name.as_deref(), Some("Comprehensive Project"));
    assert_eq!(
        project.properties.get("java.version").map(String::as_str),
        Some("17")
    );

    assert!(project.parent.is_some());
    assert_eq!(project.parent.unwrap().group_id, "com.example.parent");

    // Organization, Licenses
    assert!(project.organization.is_some());
    let licenses = project.licenses.unwrap();
    assert_eq!(licenses.licenses.len(), 1);

    // Developers with Roles
    assert!(project.developers.is_some());
    let devs = project.developers.unwrap();
    assert_eq!(devs.developers.len(), 1);
    let dev = &devs.developers[0];
    assert!(dev.roles.is_some());
    assert_eq!(dev.roles.as_ref().unwrap().roles[0], "Architect");

    // SCM, CI, Distribution
    assert!(project.scm.is_some());
    assert!(project.ci_management.is_some());
    assert!(project.distribution_management.is_some());

    // Modules
    assert!(project.modules.is_some());
    assert_eq!(project.modules.unwrap().modules.len(), 2);

    // Dependency Management
    assert!(project.dependency_management.is_some());
    assert_eq!(project.dependencies.dependencies.len(), 2);

    // Check exclusions
    let junit_dep = &project.dependencies.dependencies[1];
    assert!(junit_dep.exclusions.is_some());
    assert_eq!(
        junit_dep.exclusions.as_ref().unwrap().exclusions[0].group_id,
        "org.junit.vintage"
    );

    // Build with Plugins
    assert!(project.build.is_some());
    let build = project.build.unwrap();
    assert!(build.plugins.is_some());
    let plugins = build.plugins.unwrap();
    assert_eq!(plugins.plugins.len(), 1);

    // Reporting
    assert!(project.reporting.is_some());

    // Profiles
    assert!(project.profiles.is_some());
    let profiles = project.profiles.unwrap();
    assert_eq!(profiles.profiles.len(), 1);
    assert_eq!(profiles.profiles[0].id, "release");
    assert!(profiles.profiles[0].activation.is_some());

    Ok(())
}
