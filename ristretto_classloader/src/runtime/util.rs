#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
use crate::runtime::http;
#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
use crate::runtime::models::Release;
#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
use crate::{Error, Result};
#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
use ristretto_classfile::{JAVA_8, JAVA_11};
use std::env;
#[cfg(not(target_family = "wasm"))]
use std::env::consts;
use std::sync::LazyLock;
use tracing::debug;
#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
use tracing::warn;

const DEFAULT_MAJOR_VERSION: u16 = 21;
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
    if major_version == JAVA_8.java() && version_parts == 4 || version_parts == 5 {
        let (file_name, archive) = download_archive_for(version, os, arch).await?;
        return Ok((version.to_string(), file_name, archive));
    }

    let major_version = major_version.to_string();
    let release_versions = get_release_versions(major_version.as_str(), os, arch).await?;
    for release_version in release_versions {
        if matches_version(&release_version, version) {
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
    let (file_name, url) = archive_location(version, os, arch);
    debug!("Downloading archive: {url}");
    let archive = http::get_bytes(&url, &headers, &[]).await?;
    Ok((file_name, archive))
}

/// Construct the archive name and URL for a Corretto version and platform.
#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
fn archive_location(version: &str, os: &str, arch: &str) -> (String, String) {
    let (corretto_os, corretto_arch, extension) = match os {
        "macos" => {
            let arch = if arch == "x64" { "x64" } else { "aarch64" };
            ("macosx", arch, ".tar.gz")
        }
        "windows" => {
            let major_version = parse_major_version(version);
            let arch = if arch == "x86" && major_version <= JAVA_11.java() {
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
    (file_name, url)
}

/// Match a complete version or a prefix ending at a version-component boundary.
#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
fn matches_version(release_version: &str, version: &str) -> bool {
    release_version == version
        || release_version
            .strip_prefix(version)
            .is_some_and(|suffix| suffix.starts_with('.'))
}

/// Get the published release versions for a given major version and platform.
/// Returns versions in descending numeric order.
///
/// # Errors
///
/// An error will be returned if the request fails
#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
async fn get_release_versions(major_version: &str, os: &str, arch: &str) -> Result<Vec<String>> {
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
        releases.extend(response_releases);
        page += 1;
    }

    let versions = available_release_versions(&releases, os, arch);
    debug!("Release versions for {os}-{arch}: {versions:?}");

    Ok(versions)
}

/// Only consider archives published for this platform. Corretto normally links downloads in the
/// release body rather than attaching GitHub assets. A release may list older builds for other
/// platforms, so require the exact version's URL, not merely the platform name or archive filename.
#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
fn available_release_versions(releases: &[Release], os: &str, arch: &str) -> Vec<String> {
    let mut versions = releases
        .iter()
        .filter_map(|release| {
            let version = &release.tag_name;
            // Corretto uses four or five numeric components and sometimes a numeric packaging
            // revision (e.g. 11.0.6.10.1-2), which is not a semantic-version prerelease suffix.
            let components = version
                .split(['.', '-'])
                .map(str::parse::<u32>)
                .collect::<std::result::Result<Vec<_>, _>>()
                .ok()?;
            let (_, url) = archive_location(version, os, arch);
            let published = release
                .assets
                .iter()
                .any(|asset| asset.browser_download_url == url)
                || release.body.as_deref().is_some_and(|body| {
                    body.split(|character: char| {
                        character.is_whitespace() || matches!(character, '(' | ')' | '<' | '>')
                    })
                    .any(|link| link == url)
                });
            published.then(|| (components, version.clone()))
        })
        .collect::<Vec<_>>();
    versions.sort_unstable_by(|a, b| b.0.cmp(&a.0));
    versions.into_iter().map(|(_, version)| version).collect()
}

/// Parse the major version from a version string. Returns 0 if the major version cannot be parsed
/// as a `u16`.
pub(crate) fn parse_major_version(version: &str) -> u16 {
    let major_part = version.split('.').next().unwrap_or("0");
    major_part.parse::<u16>().unwrap_or(0)
}

#[cfg(all(test, not(all(target_family = "wasm", target_os = "unknown"))))]
mod tests {
    use super::*;
    use crate::runtime::bootstrap::{JAVA_8_VERSION, JAVA_11_VERSION, JAVA_21_VERSION};
    use crate::runtime::models::Asset;

    fn release(version: &str, body: Option<&str>) -> Release {
        Release {
            url: String::new(),
            assets_url: String::new(),
            upload_url: String::new(),
            html_url: String::new(),
            id: 0,
            tag_name: version.to_string(),
            name: format!("Amazon Corretto {version}"),
            draft: false,
            prerelease: false,
            assets: Vec::new(),
            body: body.map(str::to_string),
        }
    }

    #[test]
    fn test_release_versions_numeric_order() {
        let releases = [
            "11.0.9.12.1",
            "11.0.32.12.1",
            "11.0.10.9.1",
            "11.0.10.10.1",
            "11.0.6.10.1-2",
            "11.0.6.10.1-10",
            "11.0.6.10.1",
            "21.0.9.11.1",
            "21.0.13.0.11",
            "8.99.01.1",
            "8.100.08.1",
            "8.100.10.1",
        ]
        .map(|version| {
            let body = format!(
                "[Download](https://corretto.aws/downloads/resources/{version}/amazon-corretto-{version}-linux-x64.tar.gz)"
            );
            release(version, Some(&body))
        });
        assert_eq!(
            available_release_versions(&releases, "linux", "x64"),
            [
                "21.0.13.0.11",
                "21.0.9.11.1",
                "11.0.32.12.1",
                "11.0.10.10.1",
                "11.0.10.9.1",
                "11.0.9.12.1",
                "11.0.6.10.1-10",
                "11.0.6.10.1-2",
                "11.0.6.10.1",
                "8.100.10.1",
                "8.100.08.1",
                "8.99.01.1",
            ]
        );
    }

    #[test]
    fn test_release_versions_skip_unpublished_macos_archives() {
        let releases = [
            release(
                "11.0.9.12.1",
                Some(indoc::indoc! {"
                    | macOS x64 | [Download](https://corretto.aws/downloads/resources/11.0.9.12.1/amazon-corretto-11.0.9.12.1-macosx-x64.tar.gz) |
                    | Linux aarch64 | [Download](https://corretto.aws/downloads/resources/11.0.9.12.1/amazon-corretto-11.0.9.12.1-linux-aarch64.tar.gz) |
                "}),
            ),
            release(
                "11.0.32.12.1",
                Some(
                    "[Download](https://corretto.aws/downloads/resources/11.0.32.12.1/amazon-corretto-11.0.32.12.1-macosx-aarch64.tar.gz)",
                ),
            ),
        ];
        assert_eq!(
            available_release_versions(&releases, "macos", "aarch64"),
            ["11.0.32.12.1"]
        );
        assert_eq!(
            available_release_versions(&releases, "macos", "x64"),
            ["11.0.9.12.1"]
        );
    }

    #[test]
    fn test_release_versions_skip_linux_only_update_on_macos() {
        let releases = [
            release(
                "21.0.9.11.1",
                Some(indoc::indoc! {"
                    Linux only release; for other platforms, please use 21.0.9.10.1.
                    | Linux x64 | [Download](https://corretto.aws/downloads/resources/21.0.9.11.1/amazon-corretto-21.0.9.11.1-linux-x64.tar.gz) |
                    | macOS aarch64 | [Download](https://corretto.aws/downloads/resources/21.0.9.10.1/amazon-corretto-21.0.9.10.1-macosx-aarch64.tar.gz) |
                "}),
            ),
            release(
                "21.0.9.10.1",
                Some(indoc::indoc! {"
                    | Linux x64 | [Download](https://corretto.aws/downloads/resources/21.0.9.10.1/amazon-corretto-21.0.9.10.1-linux-x64.tar.gz) |
                    | macOS aarch64 | [Download](https://corretto.aws/downloads/resources/21.0.9.10.1/amazon-corretto-21.0.9.10.1-macosx-aarch64.tar.gz) |
                "}),
            ),
        ];
        assert_eq!(
            available_release_versions(&releases, "macos", "aarch64"),
            ["21.0.9.10.1"]
        );
        assert_eq!(
            available_release_versions(&releases, "linux", "x64"),
            ["21.0.9.11.1", "21.0.9.10.1"]
        );
    }

    #[test]
    fn test_release_versions_require_archive_download_link() {
        let releases = [
            release("11.0.9.12.1", None),
            release(
                "11.0.9.12.1",
                Some("amazon-corretto-11.0.9.12.1-macosx-aarch64.tar.gz"),
            ),
            release(
                "11.0.9.12.1",
                Some(
                    "[Signature](https://corretto.aws/downloads/resources/11.0.9.12.1/amazon-corretto-11.0.9.12.1-macosx-aarch64.tar.gz.sig)",
                ),
            ),
            release("not-a-version", None),
        ];
        assert_eq!(
            available_release_versions(&releases, "macos", "aarch64"),
            [] as [String; 0]
        );
    }

    #[test]
    fn test_release_versions_from_assets() {
        let mut release = release("21.0.9.10.1", None);
        release.assets.push(Asset {
            url: String::new(),
            id: 0,
            node_id: String::new(),
            name: "amazon-corretto-21.0.9.10.1-windows-x64-jdk.zip".to_string(),
            label: String::new(),
            content_type: "application/zip".to_string(),
            state: "uploaded".to_string(),
            size: 0,
            browser_download_url: "https://corretto.aws/downloads/resources/21.0.9.10.1/amazon-corretto-21.0.9.10.1-windows-x64-jdk.zip".to_string(),
        });
        assert_eq!(
            available_release_versions(&[release], "windows", "x64"),
            ["21.0.9.10.1"]
        );
    }

    #[test]
    fn test_matches_version_component_boundaries() {
        assert!(matches_version("11.0.32.12.1", "11"));
        assert!(matches_version("11.0.32.12.1", "11.0.32"));
        assert!(matches_version("11.0.32.12.1", "11.0.32.12.1"));
        assert!(!matches_version("11.0.32.12.1", "11.0.3"));
        assert!(!matches_version("11.0.32.12.1", "1"));
        assert!(!matches_version("11.0.32.12.1", "11.0.32.12.10"));
    }

    #[tokio::test]
    #[ignore = "requires network access"]
    async fn test_get_runtime_archive_latest_exact() -> Result<()> {
        let expected_version = JAVA_11_VERSION;
        let (version, file_name, archive) = get_runtime_archive(expected_version).await?;
        assert_eq!(expected_version, version);
        assert!(file_name.contains(expected_version));
        assert_ne!(archive, [] as [u8; 0]);
        Ok(())
    }

    #[tokio::test]
    #[ignore = "requires network access"]
    async fn test_get_runtime_archive_partial_version() -> Result<()> {
        let partial_version = JAVA_8_VERSION
            .split('.')
            .take(2)
            .collect::<Vec<_>>()
            .join(".");
        let (version, file_name, archive) = get_runtime_archive(&partial_version).await?;
        assert!(version.starts_with(&partial_version));
        assert!(file_name.contains(&partial_version));
        assert_ne!(archive, [] as [u8; 0]);
        Ok(())
    }

    #[tokio::test]
    #[ignore = "requires network access"]
    async fn test_get_runtime_archive_latest_major_version() -> Result<()> {
        let major_version = "17";
        let (version, file_name, archive) = get_runtime_archive(major_version).await?;
        assert!(version.starts_with(major_version));
        assert!(file_name.contains(major_version));
        assert_ne!(archive, [] as [u8; 0]);
        Ok(())
    }

    #[tokio::test]
    #[ignore = "requires network access"]
    async fn test_get_runtime_archive_latest_lts() -> Result<()> {
        let (version, _file_name, archive) = get_runtime_archive("*").await?;
        let expected_major_version = DEFAULT_MAJOR_VERSION.to_string();
        assert!(version.starts_with(expected_major_version.as_str()));
        assert_ne!(archive, [] as [u8; 0]);
        Ok(())
    }

    #[tokio::test]
    #[ignore = "requires network access"]
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
    #[ignore = "requires network access"]
    async fn test_download_archive() -> Result<()> {
        let version = JAVA_21_VERSION;
        let (_file_name, archive) = download_archive(version).await?;
        assert_ne!(archive, [] as [u8; 0]);
        Ok(())
    }

    #[tokio::test]
    #[ignore = "requires network access"]
    async fn test_get_release_versions() -> Result<()> {
        let (os, arch) = host_os_arch();
        for (major_version, expected_version) in [("11", JAVA_11_VERSION), ("21", JAVA_21_VERSION)]
        {
            let release_versions = get_release_versions(major_version, os, arch).await?;
            assert!(
                release_versions
                    .iter()
                    .any(|version| version == expected_version)
            );
        }
        Ok(())
    }

    #[test]
    fn test_parse_major_version() {
        assert_eq!(11, parse_major_version("11"));
        assert_eq!(8, parse_major_version("8.472.08.1"));
        assert_eq!(u16::MAX, parse_major_version("65535.0.1"));
        assert_eq!(0, parse_major_version("65536.0.1"));
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
