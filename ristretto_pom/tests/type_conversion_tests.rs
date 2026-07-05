#![cfg(not(target_family = "wasm"))]
#![expect(
    clippy::panic_in_result_fn,
    reason = "integration tests use assertions with Result-returning tests"
)]
//! Tests for standalone POM type conversions.

use ristretto_pom::{
    ChecksumPolicy, DependencyScope, Error, Packaging, RepositoryLayout, Result, UpdatePolicy,
};
use serde::Serialize;
use serde::de::DeserializeOwned;

#[derive(Debug, serde::Deserialize, Serialize)]
struct XmlValue<T> {
    value: T,
}

fn round_trip_xml<T>(value: T) -> Result<T>
where
    T: DeserializeOwned + Serialize,
{
    let xml = quick_xml::se::to_string(&XmlValue { value })?;
    let value: XmlValue<T> = quick_xml::de::from_str(&xml)?;
    Ok(value.value)
}

#[test]
fn dependency_scope_and_packaging_all_conversions() -> Result<()> {
    for (scope, text) in [
        (DependencyScope::Compile, "compile"),
        (DependencyScope::Provided, "provided"),
        (DependencyScope::Runtime, "runtime"),
        (DependencyScope::Test, "test"),
        (DependencyScope::System, "system"),
        (DependencyScope::Import, "import"),
    ] {
        assert_eq!(text.parse::<DependencyScope>()?, scope);
    }
    assert!(matches!(
        "unknown".parse::<DependencyScope>(),
        Err(Error::InvalidValue(message)) if message.contains("Unknown dependency scope")
    ));

    for (packaging, text) in [
        (Packaging::Jar, "jar"),
        (Packaging::War, "war"),
        (Packaging::Ear, "ear"),
        (Packaging::MavenPlugin, "maven-plugin"),
        (Packaging::Ejb, "ejb"),
        (Packaging::Rar, "rar"),
        (Packaging::Other("bundle".to_string()), "bundle"),
    ] {
        assert_eq!(packaging.to_string(), text);
        assert_eq!(round_trip_xml(packaging.clone())?, packaging);
        assert_eq!(text.parse::<Packaging>()?, packaging);
    }
    Ok(())
}

#[test]
fn repository_policy_types_all_conversions() -> Result<()> {
    for (layout, text) in [
        (RepositoryLayout::Default, "default"),
        (RepositoryLayout::Legacy, "legacy"),
        (RepositoryLayout::Other("flat".to_string()), "flat"),
    ] {
        assert_eq!(layout.to_string(), text);
        assert_eq!(round_trip_xml(layout.clone())?, layout);
    }

    for (policy, text) in [
        (UpdatePolicy::Always, "always"),
        (UpdatePolicy::Daily, "daily"),
        (UpdatePolicy::Never, "never"),
        (
            UpdatePolicy::Interval("interval:60".to_string()),
            "interval:60",
        ),
    ] {
        assert_eq!(policy.to_string(), text);
        assert_eq!(round_trip_xml(policy.clone())?, policy);
    }

    for (policy, text) in [
        (ChecksumPolicy::Ignore, "ignore"),
        (ChecksumPolicy::Fail, "fail"),
        (ChecksumPolicy::Warn, "warn"),
    ] {
        assert_eq!(policy.to_string(), text);
        assert_eq!(round_trip_xml(policy)?, policy);
    }
    Ok(())
}
