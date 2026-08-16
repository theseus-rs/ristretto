use crate::{Configuration, Error};
use ristretto_classloader::runtime;
use std::path::PathBuf;

pub(super) async fn java8_tools_jar(
    configuration: &Configuration,
) -> Result<Option<PathBuf>, Error> {
    let (java_home, java_version) = if let Some(java_home) = configuration.java_home() {
        let (_, java_version, _) = runtime::home_class_loader(java_home).await?;
        (java_home.clone(), java_version)
    } else if let Some(java_version) = configuration.java_version() {
        if java_major_version(java_version) != Some(8) {
            return Ok(None);
        }
        let (java_home, java_version, _) = runtime::version_class_loader(java_version).await?;
        (java_home, java_version)
    } else {
        return Ok(None);
    };

    if java_major_version(&java_version) != Some(8) {
        return Ok(None);
    }
    let tools_jar = java_home.join("lib").join("tools.jar");
    if tools_jar.is_file() {
        Ok(Some(tools_jar))
    } else {
        Err(Error::InternalError(format!(
            "Java 8 compiler tools not found at {}",
            tools_jar.display()
        )))
    }
}

fn java_major_version(version: &str) -> Option<u16> {
    let mut components = version.split(['.', 'u']);
    let first = components.next()?.parse().ok()?;
    if first == 1 {
        components.next()?.parse().ok()
    } else {
        Some(first)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_java_major_versions() {
        assert_eq!(Some(8), java_major_version("8"));
        assert_eq!(Some(8), java_major_version("1.8.0_402"));
        assert_eq!(Some(8), java_major_version("8u402"));
        assert_eq!(Some(25), java_major_version("25.0.1"));
        assert_eq!(None, java_major_version("*"));
    }
}
