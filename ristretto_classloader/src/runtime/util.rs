use crate::runtime::models::Release;
use crate::{Error, Result};
use reqwest::Client;
use reqwest::header;
use std::env;
use std::env::consts;
use std::sync::LazyLock;
use tracing::{debug, instrument};

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
/// # Errors
/// An error will be returned if the request fails or if the version requirement is not supported.
#[instrument(level = "debug")]
pub(crate) async fn get_runtime_archive(version: &str) -> Result<(String, String, Vec<u8>)> {
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
        let (file_name, archive) = download_archive(version).await?;
        return Ok((version.to_string(), file_name, archive));
    }

    let major_version = major_version.to_string();
    let release_versions = get_release_versions(major_version.as_str()).await?;
    for release_version in release_versions {
        if release_version.starts_with(version) {
            let (file_name, archive) = download_archive(release_version.as_str()).await?;
            return Ok((release_version, file_name, archive));
        }
    }

    Err(Error::UnsupportedVersion(version.to_string()))
}

/// Download a runtime archive for the given version.
///
/// # Errors
/// An error will be returned if the request fails
#[instrument(level = "debug")]
async fn download_archive(version: &str) -> Result<(String, Vec<u8>)> {
    let client = Client::new();
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::USER_AGENT,
        header::HeaderValue::from_static(&USER_AGENT),
    );

    let (os, arch, extension) = match consts::OS {
        "macos" => {
            if consts::ARCH == "aarch64" {
                ("macosx", "aarch64", ".tar.gz")
            } else {
                ("macosx", "x64", ".tar.gz")
            }
        }
        "windows" => {
            let major_version = parse_major_version(version);
            if consts::ARCH == "x86" && major_version <= 11 {
                ("windows", "x86", "-jdk.zip")
            } else {
                ("windows", "x64", "-jdk.zip")
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
                (os, "aarch64", ".tar.gz")
            } else {
                (os, "x64", ".tar.gz")
            }
        }
    };

    let file_name = format!("amazon-corretto-{version}-{os}-{arch}{extension}");
    let url = format!("https://corretto.aws/downloads/resources/{version}/{file_name}");
    debug!("Downloading archive: {url}");
    let response = client
        .get(url)
        .headers(headers)
        .send()
        .await?
        .error_for_status()?;
    let archive = response.bytes().await?;
    Ok((file_name, archive.to_vec()))
}

/// Get the release versions for a given major version.
/// Returns a vector of release versions sorted in descending order.
///
/// # Errors
/// An error will be returned if the request fails
#[instrument(level = "debug")]
async fn get_release_versions(major_version: &str) -> Result<Vec<String>> {
    let url = format!("https://api.github.com/repos/corretto/corretto-{major_version}/releases");
    let client = Client::new();
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::USER_AGENT,
        header::HeaderValue::from_static(&USER_AGENT),
    );
    headers.insert(
        GITHUB_API_VERSION_HEADER,
        header::HeaderValue::from_static(GITHUB_API_VERSION),
    );
    if let Some(token) = &*GITHUB_TOKEN {
        headers.append(
            header::AUTHORIZATION,
            format!("Bearer {token}")
                .parse()
                .map_err(|_| Error::ParseError("Bearer token".to_string()))?,
        );
    }

    debug!("Getting release versions: {url}");
    let mut page = 1;
    let mut releases = Vec::new();

    loop {
        let response = client
            .get(&url)
            .headers(headers.clone())
            .query(&[("page", page.to_string().as_str()), ("per_page", "100")])
            .send()
            .await?
            .error_for_status()?;
        let response_releases = response.json::<Vec<Release>>().await?;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_runtime_archive_latest_exact() -> Result<()> {
        let expected_version = "11.0.26.4.1";
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
        let version = "21.0.6.7.1";
        let (_file_name, archive) = download_archive(version).await?;
        assert!(!archive.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_release_versions() -> Result<()> {
        let major_version = "21";
        let release_versions = get_release_versions(major_version).await?;
        let expected_version = "21.0.6.7.1".to_string();
        assert!(release_versions.contains(&expected_version));
        Ok(())
    }

    #[test]
    fn test_parse_major_version() {
        assert_eq!(11, parse_major_version("11"));
        assert_eq!(8, parse_major_version("8.442.06.1"));
        assert_eq!(0, parse_major_version(""));
        assert_eq!(0, parse_major_version("a"));
    }
}
