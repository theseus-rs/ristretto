//! Serialization and deserialization tests for all types.

use ristretto_pom::Project;
use ristretto_pom::build::*;
use ristretto_pom::dependency::*;
use ristretto_pom::developer::*;
use ristretto_pom::distribution::*;
use ristretto_pom::license::*;
use ristretto_pom::organization::*;
use ristretto_pom::profile::*;
use ristretto_pom::reporting::*;
use ristretto_pom::repository::*;
use ristretto_pom::scm::*;
use ristretto_pom::types::*;

/// Helper to test XML serialization round-trip for a type
fn round_trip_xml<T>(value: &T) -> T
where
    T: serde::Serialize + serde::de::DeserializeOwned + std::fmt::Debug,
{
    let xml = quick_xml::se::to_string(value).expect("Failed to serialize");
    quick_xml::de::from_str(&xml).expect("Failed to deserialize")
}

mod dependency_tests {
    use super::*;

    #[test]
    fn test_dependency_serde() {
        let dep = Dependency::builder("org.junit", "junit")
            .version("4.13")
            .scope(DependencyScope::Test)
            .r#type("jar")
            .classifier("sources")
            .optional(true)
            .build();

        let result = round_trip_xml(&dep);
        assert_eq!(dep.group_id, result.group_id);
        assert_eq!(dep.artifact_id, result.artifact_id);
        assert_eq!(dep.version, result.version);
        assert_eq!(dep.scope, result.scope);
        assert_eq!(dep.r#type, result.r#type);
        assert_eq!(dep.classifier, result.classifier);
        assert_eq!(dep.optional, result.optional);
    }

    #[test]
    fn test_dependency_with_exclusions_serde() {
        let dep = Dependency::builder("org.springframework", "spring-core")
            .version("5.3.0")
            .exclusion(Exclusion::new("commons-logging", "commons-logging"))
            .exclusion(Exclusion::new("org.slf4j", "slf4j-api"))
            .build();

        let result = round_trip_xml(&dep);
        assert!(result.exclusions.is_some());
        assert_eq!(result.exclusions.as_ref().unwrap().exclusions.len(), 2);
    }

    #[test]
    fn test_dependencies_serde() {
        let mut deps = Dependencies::new();
        deps.add(Dependency::new("com.example", "lib-a"));
        deps.add(Dependency::new("com.example", "lib-b"));

        let result = round_trip_xml(&deps);
        assert_eq!(result.dependencies.len(), 2);
    }

    #[test]
    fn test_exclusion_serde() {
        let exclusion = Exclusion::new("org.hamcrest", "hamcrest-core");
        let result = round_trip_xml(&exclusion);
        assert_eq!(exclusion.group_id, result.group_id);
        assert_eq!(exclusion.artifact_id, result.artifact_id);
    }

    #[test]
    fn test_dependency_management_serde() {
        let dm = DependencyManagement::builder()
            .dependency(
                Dependency::builder("org.springframework", "spring-bom")
                    .version("5.3.0")
                    .scope(DependencyScope::Import)
                    .r#type("pom")
                    .build(),
            )
            .build();

        let result = round_trip_xml(&dm);
        assert_eq!(result.dependencies.dependencies.len(), 1);
    }
}

mod build_tests {
    use super::*;

    #[test]
    fn test_build_serde() {
        let build = Build::builder()
            .source_directory("src/main/java")
            .test_source_directory("src/test/java")
            .output_directory("target/classes")
            .final_name("my-app")
            .directory("target")
            .default_goal("install")
            .build();

        let result = round_trip_xml(&build);
        assert_eq!(build.source_directory, result.source_directory);
        assert_eq!(build.test_source_directory, result.test_source_directory);
        assert_eq!(build.output_directory, result.output_directory);
        assert_eq!(build.final_name, result.final_name);
        assert_eq!(build.directory, result.directory);
        assert_eq!(build.default_goal, result.default_goal);
    }

    #[test]
    fn test_build_with_plugins_serde() {
        let build = Build::builder()
            .plugin(
                Plugin::builder("maven-compiler-plugin")
                    .group_id("org.apache.maven.plugins")
                    .version("3.8.1")
                    .config("source", "17")
                    .config("target", "17")
                    .build(),
            )
            .plugin(
                Plugin::builder("maven-surefire-plugin")
                    .group_id("org.apache.maven.plugins")
                    .version("3.0.0")
                    .build(),
            )
            .build();

        let result = round_trip_xml(&build);
        assert!(result.plugins.is_some());
        assert_eq!(result.plugins.as_ref().unwrap().plugins.len(), 2);
    }

    #[test]
    fn test_plugin_serde() {
        let plugin = Plugin::builder("maven-jar-plugin")
            .group_id("org.apache.maven.plugins")
            .version("3.2.0")
            .extensions(true)
            .inherited(true)
            .config("archive.manifest.mainClass", "com.example.Main")
            .build();

        let result = round_trip_xml(&plugin);
        assert_eq!(plugin.artifact_id, result.artifact_id);
        assert_eq!(plugin.group_id, result.group_id);
        assert_eq!(plugin.version, result.version);
        assert_eq!(plugin.extensions, result.extensions);
        assert_eq!(plugin.inherited, result.inherited);
    }

    #[test]
    fn test_plugin_with_executions_serde() {
        let plugin = Plugin::builder("maven-resources-plugin")
            .execution(
                PluginExecution::builder()
                    .id("copy-resources")
                    .phase("validate")
                    .goal("copy-resources")
                    .config("outputDirectory", "${project.build.directory}")
                    .build(),
            )
            .build();

        let result = round_trip_xml(&plugin);
        assert!(result.executions.is_some());
        assert_eq!(result.executions.as_ref().unwrap().executions.len(), 1);
    }

    #[test]
    fn test_plugin_execution_serde() {
        let execution = PluginExecution::builder()
            .id("default-compile")
            .phase("compile")
            .goal("compile")
            .goal("testCompile")
            .config("source", "17")
            .build();

        let result = round_trip_xml(&execution);
        assert_eq!(execution.id, result.id);
        assert_eq!(execution.phase, result.phase);
        assert!(result.goals.is_some());
        assert_eq!(result.goals.as_ref().unwrap().goals.len(), 2);
    }

    #[test]
    fn test_resource_serde() {
        let resource = Resource::builder()
            .directory("src/main/resources")
            .target_path("META-INF")
            .filtering(true)
            .include("**/*.xml")
            .include("**/*.properties")
            .exclude("**/*.bak")
            .build();

        let result = round_trip_xml(&resource);
        assert_eq!(resource.directory, result.directory);
        assert_eq!(resource.target_path, result.target_path);
        assert_eq!(resource.filtering, result.filtering);
        assert!(result.includes.is_some());
        assert!(result.excludes.is_some());
    }

    #[test]
    fn test_extension_serde() {
        let extension = Extension::new("org.apache.maven.wagon", "wagon-ssh", "3.4.0");
        let result = round_trip_xml(&extension);
        assert_eq!(extension.group_id, result.group_id);
        assert_eq!(extension.artifact_id, result.artifact_id);
        assert_eq!(extension.version, result.version);
    }

    #[test]
    fn test_plugin_management_serde() {
        let pm = PluginManagement::new(vec![
            Plugin::builder("maven-compiler-plugin")
                .version("3.8.1")
                .build(),
            Plugin::builder("maven-surefire-plugin")
                .version("3.0.0")
                .build(),
        ]);

        let result = round_trip_xml(&pm);
        assert_eq!(result.plugins.len(), 2);
    }

    #[test]
    fn test_goals_serde() {
        let goals = Goals::from_vec(vec!["clean".to_string(), "install".to_string()]);
        let result = round_trip_xml(&goals);
        assert_eq!(result.goals.len(), 2);
    }
}

mod repository_tests {
    use super::*;

    #[test]
    fn test_repository_serde() {
        let repo = Repository::builder("central", "https://repo.maven.apache.org/maven2")
            .name("Maven Central")
            .layout(RepositoryLayout::Default)
            .releases(RepositoryPolicy::enabled())
            .snapshots(RepositoryPolicy::disabled())
            .build();

        let result = round_trip_xml(&repo);
        assert_eq!(repo.id, result.id);
        assert_eq!(repo.url, result.url);
        assert_eq!(repo.name, result.name);
        assert_eq!(repo.layout, result.layout);
        assert!(result.releases.is_some());
        assert!(result.snapshots.is_some());
    }

    #[test]
    fn test_repository_policy_serde() {
        let policy = RepositoryPolicy::builder()
            .enabled(true)
            .update_policy(UpdatePolicy::Daily)
            .checksum_policy(ChecksumPolicy::Fail)
            .build();

        let result = round_trip_xml(&policy);
        assert_eq!(policy.enabled, result.enabled);
        assert_eq!(policy.update_policy, result.update_policy);
        assert_eq!(policy.checksum_policy, result.checksum_policy);
    }

    #[test]
    fn test_repositories_serde() {
        let mut repos = Repositories::new();
        repos.add(Repository::new("repo1", "https://repo1.example.com"));
        repos.add(Repository::new("repo2", "https://repo2.example.com"));

        let result = round_trip_xml(&repos);
        assert_eq!(result.repositories.len(), 2);
    }
}

mod developer_tests {
    use super::*;

    #[test]
    fn test_developer_serde() {
        let dev = Developer::builder()
            .id("jdoe")
            .name("John Doe")
            .email("john@example.com")
            .url("https://johndoe.com")
            .organization("Example Corp")
            .organization_url("https://example.com")
            .timezone("America/New_York")
            .role("Lead Developer")
            .role("Architect")
            .property("twitter", "@johndoe")
            .build();

        let result = round_trip_xml(&dev);
        assert_eq!(dev.id, result.id);
        assert_eq!(dev.name, result.name);
        assert_eq!(dev.email, result.email);
        assert_eq!(dev.url, result.url);
        assert_eq!(dev.organization, result.organization);
        assert_eq!(dev.timezone, result.timezone);
        assert!(result.roles.is_some());
    }

    #[test]
    fn test_contributor_serde() {
        let contrib = Contributor::builder()
            .name("Jane Smith")
            .email("jane@example.com")
            .organization("Contrib Org")
            .role("Tester")
            .build();

        let result = round_trip_xml(&contrib);
        assert_eq!(contrib.name, result.name);
        assert_eq!(contrib.email, result.email);
        assert!(result.roles.is_some());
    }

    #[test]
    fn test_developers_serde() {
        let mut devs = Developers::new();
        devs.add(Developer::builder().id("dev1").name("Dev One").build());
        devs.add(Developer::builder().id("dev2").name("Dev Two").build());

        let result = round_trip_xml(&devs);
        assert_eq!(result.developers.len(), 2);
    }

    #[test]
    fn test_roles_serde() {
        let roles = Roles::from_vec(vec!["Developer".to_string(), "Maintainer".to_string()]);
        let result = round_trip_xml(&roles);
        assert_eq!(result.roles.len(), 2);
    }
}

mod license_tests {
    use super::*;

    #[test]
    fn test_license_serde() {
        let license = License::builder("Apache License 2.0")
            .url("https://www.apache.org/licenses/LICENSE-2.0")
            .distribution("repo")
            .comments("A permissive license")
            .build();

        let result = round_trip_xml(&license);
        assert_eq!(license.name, result.name);
        assert_eq!(license.url, result.url);
        assert_eq!(license.distribution, result.distribution);
        assert_eq!(license.comments, result.comments);
    }

    #[test]
    fn test_licenses_serde() {
        let mut licenses = Licenses::new();
        licenses.add(License::mit());
        licenses.add(License::apache2());

        let result = round_trip_xml(&licenses);
        assert_eq!(result.licenses.len(), 2);
    }
}

mod scm_tests {
    use super::*;

    #[test]
    fn test_scm_serde() {
        let scm = Scm::builder()
            .connection("scm:git:git://github.com/example/repo.git")
            .developer_connection("scm:git:ssh://github.com/example/repo.git")
            .url("https://github.com/example/repo")
            .tag("v1.0.0")
            .build();

        let result = round_trip_xml(&scm);
        assert_eq!(scm.connection, result.connection);
        assert_eq!(scm.developer_connection, result.developer_connection);
        assert_eq!(scm.url, result.url);
        assert_eq!(scm.tag, result.tag);
    }

    #[test]
    fn test_issue_management_serde() {
        let im = IssueManagement::new("GitHub", "https://github.com/example/repo/issues");
        let result = round_trip_xml(&im);
        assert_eq!(im.system, result.system);
        assert_eq!(im.url, result.url);
    }

    #[test]
    fn test_ci_management_serde() {
        let ci = CiManagement::builder("GitHub Actions", "https://github.com/example/repo/actions")
            .notifier(
                Notifier::builder("mail")
                    .address("ci@example.com")
                    .send_on_error(true)
                    .send_on_failure(true)
                    .build(),
            )
            .build();

        let result = round_trip_xml(&ci);
        assert_eq!(ci.system, result.system);
        assert_eq!(ci.url, result.url);
        assert!(result.notifiers.is_some());
    }

    #[test]
    fn test_notifier_serde() {
        let notifier = Notifier::builder("mail")
            .address("team@example.com")
            .send_on_error(true)
            .send_on_failure(true)
            .send_on_success(false)
            .send_on_warning(true)
            .config("smtpHost", "smtp.example.com")
            .build();

        let result = round_trip_xml(&notifier);
        assert_eq!(notifier.r#type, result.r#type);
        assert_eq!(notifier.address, result.address);
        assert_eq!(notifier.send_on_error, result.send_on_error);
        assert_eq!(notifier.send_on_failure, result.send_on_failure);
        assert_eq!(notifier.send_on_success, result.send_on_success);
        assert_eq!(notifier.send_on_warning, result.send_on_warning);
    }
}

mod distribution_tests {
    use super::*;

    #[test]
    fn test_distribution_management_serde() {
        let dm = DistributionManagement::builder()
            .repository(Repository::new(
                "releases",
                "https://repo.example.com/releases",
            ))
            .snapshot_repository(Repository::new(
                "snapshots",
                "https://repo.example.com/snapshots",
            ))
            .site(Site::new("site", "https://site.example.com"))
            .download_url("https://download.example.com")
            .status("deployed")
            .build();

        let result = round_trip_xml(&dm);
        assert!(result.repository.is_some());
        assert!(result.snapshot_repository.is_some());
        assert!(result.site.is_some());
        assert_eq!(dm.download_url, result.download_url);
        assert_eq!(dm.status, result.status);
    }

    #[test]
    fn test_site_serde() {
        let site = Site::builder("docs", "https://docs.example.com")
            .name("Project Documentation")
            .build();

        let result = round_trip_xml(&site);
        assert_eq!(site.id, result.id);
        assert_eq!(site.url, result.url);
        assert_eq!(site.name, result.name);
    }

    #[test]
    fn test_relocation_serde() {
        let relocation = Relocation::builder()
            .group_id("com.new.group")
            .artifact_id("new-artifact")
            .version("2.0.0")
            .message("This artifact has moved")
            .build();

        let result = round_trip_xml(&relocation);
        assert_eq!(relocation.group_id, result.group_id);
        assert_eq!(relocation.artifact_id, result.artifact_id);
        assert_eq!(relocation.version, result.version);
        assert_eq!(relocation.message, result.message);
    }
}

mod organization_tests {
    use super::*;

    #[test]
    fn test_parent_serde() {
        let parent = Parent::builder("com.example", "parent-pom", "1.0.0")
            .relative_path("../parent/pom.xml")
            .build();

        let result = round_trip_xml(&parent);
        assert_eq!(parent.group_id, result.group_id);
        assert_eq!(parent.artifact_id, result.artifact_id);
        assert_eq!(parent.version, result.version);
        assert_eq!(parent.relative_path, result.relative_path);
    }

    #[test]
    fn test_organization_serde() {
        let org = Organization::builder("Example Corporation")
            .url("https://example.com")
            .build();

        let result = round_trip_xml(&org);
        assert_eq!(org.name, result.name);
        assert_eq!(org.url, result.url);
    }

    #[test]
    fn test_prerequisites_serde() {
        let prereqs = Prerequisites::new("3.6.0");
        let result = round_trip_xml(&prereqs);
        assert_eq!(prereqs.maven, result.maven);
    }

    #[test]
    fn test_mailing_list_serde() {
        let ml = MailingList::builder()
            .name("Users")
            .subscribe("users-subscribe@example.com")
            .unsubscribe("users-unsubscribe@example.com")
            .post("users@example.com")
            .archive("https://lists.example.com/users")
            .other_archive("https://archive2.example.com/users")
            .build();

        let result = round_trip_xml(&ml);
        assert_eq!(ml.name, result.name);
        assert_eq!(ml.subscribe, result.subscribe);
        assert_eq!(ml.unsubscribe, result.unsubscribe);
        assert_eq!(ml.post, result.post);
        assert_eq!(ml.archive, result.archive);
        assert_eq!(ml.other_archives.len(), result.other_archives.len());
    }

    #[test]
    fn test_mailing_lists_serde() {
        let mut mls = MailingLists::new();
        mls.add(MailingList::builder().name("Users").build());
        mls.add(MailingList::builder().name("Developers").build());

        let result = round_trip_xml(&mls);
        assert_eq!(result.mailing_lists.len(), 2);
    }
}

mod profile_tests {
    use super::*;

    #[test]
    fn test_profile_serde() {
        let profile = Profile::builder("release")
            .activation(Activation::property(
                "performRelease",
                Some("true".to_string()),
            ))
            .property("maven.test.skip", "true")
            .build();

        let result = round_trip_xml(&profile);
        assert_eq!(profile.id, result.id);
        assert!(result.activation.is_some());
        assert!(result.properties.contains_key("maven.test.skip"));
    }

    #[test]
    fn test_activation_active_by_default_serde() {
        let activation = Activation::active_by_default();
        let result = round_trip_xml(&activation);
        assert!(result.active_by_default);
    }

    #[test]
    fn test_activation_jdk_serde() {
        let activation = Activation::jdk("17");
        let result = round_trip_xml(&activation);
        assert_eq!(activation.jdk, result.jdk);
    }

    #[test]
    fn test_activation_property_serde() {
        let prop = ActivationProperty::with_value("env", "production");
        let result = round_trip_xml(&prop);
        assert_eq!(prop.name, result.name);
        assert_eq!(prop.value, result.value);
    }

    #[test]
    fn test_activation_os_serde() {
        let os = ActivationOs::builder()
            .name("Windows 10")
            .family("windows")
            .arch("amd64")
            .version("10.0")
            .build();

        let result = round_trip_xml(&os);
        assert_eq!(os.name, result.name);
        assert_eq!(os.family, result.family);
        assert_eq!(os.arch, result.arch);
        assert_eq!(os.version, result.version);
    }

    #[test]
    fn test_activation_file_exists_serde() {
        let file = ActivationFile::exists("build.properties");
        let result = round_trip_xml(&file);
        assert_eq!(file.exists, result.exists);
    }

    #[test]
    fn test_activation_file_missing_serde() {
        let file = ActivationFile::missing("skip.txt");
        let result = round_trip_xml(&file);
        assert_eq!(file.missing, result.missing);
    }

    #[test]
    fn test_modules_serde() {
        let mut modules = Modules::new();
        modules.add("module-a");
        modules.add("module-b");
        modules.add("module-c");

        let result = round_trip_xml(&modules);
        assert_eq!(result.modules.len(), 3);
    }

    #[test]
    fn test_profiles_serde() {
        let mut profiles = Profiles::new();
        profiles.add(Profile::new("dev"));
        profiles.add(Profile::new("prod"));

        let result = round_trip_xml(&profiles);
        assert_eq!(result.profiles.len(), 2);
    }
}

mod reporting_tests {
    use super::*;

    #[test]
    fn test_reporting_serde() {
        let reporting = Reporting::builder()
            .exclude_defaults(true)
            .output_directory("target/site")
            .plugin(
                ReportPlugin::builder("maven-javadoc-plugin")
                    .group_id("org.apache.maven.plugins")
                    .version("3.3.1")
                    .build(),
            )
            .build();

        let result = round_trip_xml(&reporting);
        assert_eq!(reporting.exclude_defaults, result.exclude_defaults);
        assert_eq!(reporting.output_directory, result.output_directory);
        assert!(result.plugins.is_some());
    }

    #[test]
    fn test_report_plugin_serde() {
        let plugin = ReportPlugin::builder("maven-project-info-reports-plugin")
            .group_id("org.apache.maven.plugins")
            .version("3.1.0")
            .config("dependencyLocationsEnabled", "false")
            .report_set(
                ReportSet::builder()
                    .id("default")
                    .report("dependencies")
                    .report("license")
                    .build(),
            )
            .build();

        let result = round_trip_xml(&plugin);
        assert_eq!(plugin.artifact_id, result.artifact_id);
        assert!(result.report_sets.is_some());
    }

    #[test]
    fn test_report_set_serde() {
        let report_set = ReportSet::builder()
            .id("aggregate")
            .report("aggregate")
            .inherited(true)
            .build();

        let result = round_trip_xml(&report_set);
        assert_eq!(report_set.id, result.id);
        assert_eq!(report_set.inherited, result.inherited);
        assert!(result.reports.is_some());
    }

    #[test]
    fn test_reports_serde() {
        let reports = Reports::from_vec(vec![
            "dependencies".to_string(),
            "license".to_string(),
            "plugins".to_string(),
        ]);

        let result = round_trip_xml(&reports);
        assert_eq!(result.reports.len(), 3);
    }
}

mod project_tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_minimal_project_serde() {
        let project = Project::new("com.example", "my-app", "1.0.0");

        // Serialize to file and read back
        let file = NamedTempFile::new().unwrap();
        project.to_file(file.path()).unwrap();
        let result = Project::from_file(file.path()).unwrap();

        assert_eq!(project.group_id, result.group_id);
        assert_eq!(project.artifact_id, result.artifact_id);
        assert_eq!(project.version, result.version);
    }

    #[test]
    fn test_project_with_parent_serde() {
        let xml = r"
<project>
    <modelVersion>4.0.0</modelVersion>
    <parent>
        <groupId>com.example</groupId>
        <artifactId>parent-pom</artifactId>
        <version>1.0.0</version>
        <relativePath>../parent/pom.xml</relativePath>
    </parent>
    <artifactId>child-module</artifactId>
</project>
";
        let mut file = NamedTempFile::new().unwrap();
        file.write_all(xml.as_bytes()).unwrap();
        let project = Project::from_file(file.path()).unwrap();

        assert!(project.parent.is_some());
        let parent = project.parent.as_ref().unwrap();
        assert_eq!(parent.group_id, "com.example");
        assert_eq!(parent.artifact_id, "parent-pom");
        assert_eq!(parent.version, "1.0.0");
        assert_eq!(parent.relative_path, Some("../parent/pom.xml".to_string()));
    }

    #[test]
    fn test_full_project_builder_serde() {
        let project = Project::builder("comprehensive-app")
            .group_id("com.example")
            .version("2.0.0")
            .packaging("jar")
            .name("Comprehensive Application")
            .description("A test application")
            .url("https://example.com")
            .inception_year("2024")
            .organization(
                Organization::builder("Example Corp")
                    .url("https://example.com")
                    .build(),
            )
            .scm(Scm::github("example", "repo"))
            .issue_management(IssueManagement::new(
                "example",
                "https://example.com/issues",
            ))
            .ci_management(CiManagement::new("example", "https://example.com/ci"))
            .licenses(Licenses::from_vec(vec![License::mit()]))
            .property("java.version", "17")
            .property("project.build.sourceEncoding", "UTF-8")
            .build_project();

        let file = NamedTempFile::new().unwrap();
        project.to_file(file.path()).unwrap();
        let result = Project::from_file(file.path()).unwrap();

        assert_eq!(project.group_id, result.group_id);
        assert_eq!(project.artifact_id, result.artifact_id);
        assert_eq!(project.version, result.version);
        assert_eq!(project.packaging, result.packaging);
        assert_eq!(project.name, result.name);
        assert_eq!(project.description, result.description);
        assert_eq!(project.url, result.url);
        assert!(result.organization.is_some());
        assert!(result.scm.is_some());
        assert!(result.licenses.is_some());
    }
}
