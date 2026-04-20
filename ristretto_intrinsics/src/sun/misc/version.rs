use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaObject;
use ristretto_types::Thread;
use ristretto_types::VM;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

const VERSION_CLASS: &str = "sun.misc.Version";

/// Parse a Java version string of the form `"<feature>.<update>.<build>[.<revision>]"`
/// (e.g. `"8.482.08.1"`) into the components used by `sun.misc.Version`:
/// `(major, minor, micro, update, build)`.
///
/// For Java 8 the reported triple is always `1.8.0`; the second component of the version string
/// represents the update number and the third component represents the build number. Any
/// component that cannot be parsed defaults to `0`.
fn parse_version(version: &str) -> (i32, i32, i32, i32, i32) {
    let mut parts = version.split('.');
    let feature: i32 = parts.next().and_then(|p| p.parse().ok()).unwrap_or(0);
    let update: i32 = parts.next().and_then(|p| p.parse().ok()).unwrap_or(0);
    let build: i32 = parts.next().and_then(|p| p.parse().ok()).unwrap_or(0);

    if feature <= 8 {
        (1, feature, 0, update, build)
    } else {
        (feature, 0, update, 0, build)
    }
}

async fn set_version_fields<T: Thread + 'static>(thread: &Arc<T>, prefix: &str) -> Result<()> {
    let vm = thread.vm()?;
    let (major, minor, micro, update, build) = parse_version(vm.java_version());
    let class = thread.class(VERSION_CLASS).await?;
    class.set_static_value(&format!("{prefix}_major_version"), Value::Int(major))?;
    class.set_static_value(&format!("{prefix}_minor_version"), Value::Int(minor))?;
    class.set_static_value(&format!("{prefix}_micro_version"), Value::Int(micro))?;
    class.set_static_value(&format!("{prefix}_update_version"), Value::Int(update))?;
    class.set_static_value(&format!("{prefix}_build_number"), Value::Int(build))?;
    Ok(())
}

#[intrinsic_method(
    "sun/misc/Version.getJdkSpecialVersion()Ljava/lang/String;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn get_jdk_special_version<T: Thread + 'static>(
    thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    let value = "".to_object(&thread).await?;
    Ok(Some(value))
}

#[intrinsic_method("sun/misc/Version.getJdkVersionInfo()V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn get_jdk_version_info<T: Thread + 'static>(
    thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    set_version_fields(&thread, "jdk").await?;
    Ok(None)
}

#[intrinsic_method(
    "sun/misc/Version.getJvmSpecialVersion()Ljava/lang/String;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn get_jvm_special_version<T: Thread + 'static>(
    thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    let value = "".to_object(&thread).await?;
    Ok(Some(value))
}

#[intrinsic_method("sun/misc/Version.getJvmVersionInfo()Z", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn get_jvm_version_info<T: Thread + 'static>(
    thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    set_version_fields(&thread, "jvm").await?;
    Ok(Some(Value::from(true)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_version_java8() {
        assert_eq!(parse_version("8.482.08.1"), (1, 8, 0, 482, 8));
    }

    #[test]
    fn test_parse_version_java11() {
        assert_eq!(parse_version("11.0.30.7.1"), (11, 0, 0, 0, 30));
    }

    #[test]
    fn test_parse_version_empty() {
        assert_eq!(parse_version(""), (1, 0, 0, 0, 0));
    }

    #[test]
    fn test_parse_version_partial() {
        assert_eq!(parse_version("8"), (1, 8, 0, 0, 0));
    }

    #[tokio::test]
    async fn test_get_jdk_special_version() -> Result<()> {
        let (_vm, thread) = crate::test::java8_thread().await?;
        let result = get_jdk_special_version(thread.clone(), Parameters::default()).await?;
        let expected = "".to_object(&thread).await?;
        assert_eq!(result, Some(expected));
        Ok(())
    }

    #[tokio::test]
    async fn test_get_jdk_version_info() -> Result<()> {
        let (_vm, thread) = crate::test::java8_thread().await?;
        let result = get_jdk_version_info(thread.clone(), Parameters::default()).await?;
        assert_eq!(result, None);
        let class = thread.class(VERSION_CLASS).await?;
        assert_eq!(class.static_value("jdk_major_version")?.as_i32()?, 1);
        assert_eq!(class.static_value("jdk_minor_version")?.as_i32()?, 8);
        assert_eq!(class.static_value("jdk_micro_version")?.as_i32()?, 0);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_jvm_special_version() -> Result<()> {
        let (_vm, thread) = crate::test::java8_thread().await?;
        let result = get_jvm_special_version(thread.clone(), Parameters::default()).await?;
        let expected = "".to_object(&thread).await?;
        assert_eq!(result, Some(expected));
        Ok(())
    }

    #[tokio::test]
    async fn test_get_jvm_version_info() -> Result<()> {
        let (_vm, thread) = crate::test::java8_thread().await?;
        let result = get_jvm_version_info(thread.clone(), Parameters::default()).await?;
        assert_eq!(result, Some(Value::from(true)));
        let class = thread.class(VERSION_CLASS).await?;
        assert_eq!(class.static_value("jvm_major_version")?.as_i32()?, 1);
        assert_eq!(class.static_value("jvm_minor_version")?.as_i32()?, 8);
        assert_eq!(class.static_value("jvm_micro_version")?.as_i32()?, 0);
        Ok(())
    }
}
