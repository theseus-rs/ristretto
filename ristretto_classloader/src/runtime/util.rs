#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
use crate::runtime::http;
#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
use crate::runtime::models::Release;
#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
use crate::{Error, Result};
use std::env;
#[cfg(not(target_family = "wasm"))]
use std::env::consts;
use std::sync::LazyLock;
use tracing::debug;
#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
use tracing::warn;

const DEFAULT_MAJOR_VERSION: u64 = 21;
const GITHUB_API_VERSION_HEADER: &str = "X-GitHub-Api-Version";
const GITHUB_API_VERSION: &str = "2022-11-28";

static GITHUB_TOKEN: LazyLock<Option<String>> = LazyLock::new(|| match env::var("GITHUB_TOKEN") {
    Ok(token) => {
        debug!("GITHUB_TOKEN environment variable found");
        Some(token)
    }
    Err(_) => None,
});

static USER_AGENT: LazyLock<String> = LazyLock::new(|| {
    format!(
        "{PACKAGE}/{VERSION}",
        PACKAGE = env!("CARGO_PKG_NAME"),
        VERSION = env!("CARGO_PKG_VERSION")
    )
});

/// Get a runtime archive for the given version requirement. If the version requirement is a partial
/// version, the latest release matching the partial version will be returned. If the version
/// requirement is a major version, the latest release for that major version will be returned. If
/// the version requirement is a `*`, the latest LTS release supported by the runtime will be
/// returned.
///
/// The archive is downloaded for the host OS / architecture.
///
/// # Errors
///
/// An error will be returned if the request fails or if the version requirement is not supported.
#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
pub(crate) async fn get_runtime_archive(version: &str) -> Result<(String, String, Vec<u8>)> {
    let (os, arch) = host_os_arch();
    get_runtime_archive_for(version, os, arch).await
}

/// Get a runtime archive for the given version requirement and explicit OS / architecture. This is
/// the cross-OS variant of [`get_runtime_archive`].
///
/// # Errors
///
/// An error will be returned if the request fails or if the version requirement is not supported.
#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
pub(crate) async fn get_runtime_archive_for(
    version: &str,
    os: &str,
    arch: &str,
) -> Result<(String, String, Vec<u8>)> {
    let version = if version == "*" {
        DEFAULT_MAJOR_VERSION.to_string()
    } else {
        version.to_string()
    };
    let version = version.as_str();

    let major_version = parse_major_version(version);
    if major_version == 0 {
        return Err(Error::UnsupportedVersion(version.to_string()));
    }

    let version_parts = version.chars().filter(|&c| c == '.').count() + 1;
    if major_version == 8 && version_parts == 4 || version_parts == 5 {
        let (file_name, archive) = download_archive_for(version, os, arch).await?;
        return Ok((version.to_string(), file_name, archive));
    }

    let major_version = major_version.to_string();
    let release_versions = get_release_versions(major_version.as_str()).await?;
    for release_version in release_versions {
        if release_version.starts_with(version) {
            match download_archive_for(release_version.as_str(), os, arch).await {
                Ok((file_name, archive)) => {
                    return Ok((release_version, file_name, archive));
                }
                Err(error) => {
                    warn!("Unable to download archive: {error}");
                }
            }
        }
    }

    Err(Error::UnsupportedVersion(version.to_string()))
}

/// Determine the corretto OS/arch tuple for the host platform.
#[cfg(not(target_family = "wasm"))]
pub(crate) fn host_os_arch() -> (&'static str, &'static str) {
    match consts::OS {
        "macos" => {
            if consts::ARCH == "x86_64" {
                ("macos", "x64")
            } else {
                ("macos", "aarch64")
            }
        }
        "windows" => {
            if consts::ARCH == "x86" {
                ("windows", "x86")
            } else {
                ("windows", "x64")
            }
        }
        _ => {
            let target = env::var("TARGET").unwrap_or_default();
            let os = if target.contains("musl") {
                "alpine-linux"
            } else {
                "linux"
            };
            if consts::ARCH == "aarch64" {
                (os, "aarch64")
            } else {
                (os, "x64")
            }
        }
    }
}

/// Determine the corretto OS/arch tuple to use when running under wasi. wasi has no host OS, so
/// pick a sensible default that the JDK is published for. `x64` Linux is always available.
#[cfg(target_os = "wasi")]
pub(crate) fn host_os_arch() -> (&'static str, &'static str) {
    ("linux", "x64")
}

/// Download a runtime archive for the given version targeting the host OS / architecture.
///
/// # Errors
///
/// An error will be returned if the request fails
#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
async fn download_archive(version: &str) -> Result<(String, Vec<u8>)> {
    let (os, arch) = host_os_arch();
    download_archive_for(version, os, arch).await
}

/// Download a runtime archive for the given version targeting the specified OS / architecture.
///
/// `os` is one of `"macos"`, `"linux"`, `"alpine-linux"`, or `"windows"`. `arch` is one of
/// `"x64"`, `"x86"`, or `"aarch64"`. Note that internally `"macos"` is mapped to corretto's
/// `"macosx"` directory name.
///
/// # Errors
///
/// An error will be returned if the request fails
#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
pub(crate) async fn download_archive_for(
    version: &str,
    os: &str,
    arch: &str,
) -> Result<(String, Vec<u8>)> {
    let headers = vec![("user-agent".to_string(), USER_AGENT.clone())];

    let (corretto_os, corretto_arch, extension) = match os {
        "macos" => {
            let arch = if arch == "x64" { "x64" } else { "aarch64" };
            ("macosx", arch, ".tar.gz")
        }
        "windows" => {
            let major_version = parse_major_version(version);
            let arch = if arch == "x86" && major_version <= 11 {
                "x86"
            } else {
                "x64"
            };
            ("windows", arch, "-jdk.zip")
        }
        _ => {
            let arch = if arch == "aarch64" { "aarch64" } else { "x64" };
            (os, arch, ".tar.gz")
        }
    };

    let file_name = format!("amazon-corretto-{version}-{corretto_os}-{corretto_arch}{extension}");
    let url = format!("https://corretto.aws/downloads/resources/{version}/{file_name}");
    debug!("Downloading archive: {url}");
    let archive = http::get_bytes(&url, &headers, &[]).await?;
    Ok((file_name, archive))
}

/// Get the release versions for a given major version.
/// Returns a vector of release versions sorted in descending order.
///
/// # Errors
///
/// An error will be returned if the request fails
#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
async fn get_release_versions(major_version: &str) -> Result<Vec<String>> {
    let url = format!("https://api.github.com/repos/corretto/corretto-{major_version}/releases");
    let mut headers = vec![
        ("user-agent".to_string(), USER_AGENT.clone()),
        (
            GITHUB_API_VERSION_HEADER.to_string(),
            GITHUB_API_VERSION.to_string(),
        ),
    ];
    if let Some(token) = &*GITHUB_TOKEN {
        headers.push(("authorization".to_string(), format!("Bearer {token}")));
    }

    debug!("Getting release versions: {url}");
    let mut page = 1u32;
    let mut releases = Vec::new();

    loop {
        let page_str = page.to_string();
        let query = [("page", page_str.as_str()), ("per_page", "100")];
        let response_releases: Vec<Release> = http::get_json(&url, &headers, &query).await?;
        if response_releases.is_empty() {
            break;
        }
        for release in response_releases {
            releases.push(release.name);
        }
        page += 1;
    }

    // Sort the releases in descending order
    releases.sort_by(|a, b| b.cmp(a));
    debug!("Release versions: {releases:?}");

    Ok(releases)
}

/// Parse the major version from a version string. Returns 0 if the major version cannot be parsed.
pub(crate) fn parse_major_version(version: &str) -> u64 {
    let major_part = version.split('.').next().unwrap_or("0");
    major_part.parse::<u64>().unwrap_or(0)
}

#[cfg(all(test, not(all(target_family = "wasm", target_os = "unknown"))))]
mod tests {
    use super::*;
    use crate::runtime::bootstrap::{JAVA_11_VERSION, JAVA_21_VERSION};

    #[tokio::test]
    async fn test_get_runtime_archive_latest_exact() -> Result<()> {
        let expected_version = JAVA_11_VERSION;
        let (version, file_name, archive) = get_runtime_archive(expected_version).await?;
        assert_eq!(expected_version, version);
        assert!(file_name.contains(expected_version));
        assert!(!archive.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_runtime_archive_partial_version() -> Result<()> {
        let partial_version = "8.422";
        let (version, file_name, archive) = get_runtime_archive(partial_version).await?;
        assert!(version.starts_with(partial_version));
        assert!(file_name.contains(partial_version));
        assert!(!archive.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_runtime_archive_latest_major_version() -> Result<()> {
        let major_version = "17";
        let (version, file_name, archive) = get_runtime_archive(major_version).await?;
        assert!(version.starts_with(major_version));
        assert!(file_name.contains(major_version));
        assert!(!archive.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_runtime_archive_latest_lts() -> Result<()> {
        let (version, _file_name, archive) = get_runtime_archive("*").await?;
        let expected_major_version = DEFAULT_MAJOR_VERSION.to_string();
        assert!(version.starts_with(expected_major_version.as_str()));
        assert!(!archive.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_runtime_archive_unsupported_version() {
        let result = get_runtime_archive("21.0.0.0.0").await;
        assert!(matches!(result, Err(Error::RequestError(_))));
    }

    #[tokio::test]
    async fn test_get_runtime_archive_invalid() {
        let result = get_runtime_archive("0").await;
        assert!(matches!(result, Err(Error::UnsupportedVersion(_))));
    }

    #[tokio::test]
    async fn test_download_archive() -> Result<()> {
        let version = JAVA_21_VERSION;
        let (_file_name, archive) = download_archive(version).await?;
        assert!(!archive.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_release_versions() -> Result<()> {
        let major_version = "21";
        let release_versions = get_release_versions(major_version).await?;
        let expected_version = JAVA_21_VERSION.to_string();
        assert!(release_versions.contains(&expected_version));
        Ok(())
    }

    #[test]
    fn test_parse_major_version() {
        assert_eq!(11, parse_major_version("11"));
        assert_eq!(8, parse_major_version("8.472.08.1"));
        assert_eq!(0, parse_major_version(""));
        assert_eq!(0, parse_major_version("a"));
    }

    #[cfg(not(target_family = "wasm"))]
    #[test]
    fn test_host_os_arch_returns_supported_tuple() {
        let (os, arch) = host_os_arch();
        assert!(
            matches!(os, "macos" | "windows" | "linux" | "alpine-linux"),
            "unexpected os {os}"
        );
        assert!(
            matches!(arch, "x64" | "x86" | "aarch64"),
            "unexpected arch {arch}"
        );
        if os == "macos" && consts::ARCH == "x86_64" {
            assert_eq!(arch, "x64", "Intel Mac must map to x64");
        }
    }
}
