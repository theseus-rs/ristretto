use crate::runtime::util;
use crate::{ClassLoader, ClassPath, Error, Result};
use flate2::bufread::GzDecoder;
use std::path::{Path, PathBuf};
use std::{env, fs, io};
use tar::Archive;
use tracing::{debug, instrument};

pub const DEFAULT_RUNTIME_VERSION: &str = "21.0.4.7.1";

/// Get a class loader for the default Java runtime version. If the version is not installed, the
/// archive will be downloaded and extracted.
///
/// # Errors
/// An error will be returned if the class loader cannot be created.
pub async fn default_class_loader() -> Result<(String, ClassLoader)> {
    class_loader(DEFAULT_RUNTIME_VERSION).await
}

/// Get a class loader for the given Java runtime version. If the version is not installed, the
/// archive will be downloaded and extracted. A version can be a partial version, a major version,
/// or a `*` to get the latest LTS release supported by the runtime.
///
/// # Errors
/// An error will be returned if the class loader cannot be created.
#[instrument(level = "debug")]
pub async fn class_loader(version: &str) -> Result<(String, ClassLoader)> {
    let mut archive_version = version.to_string();
    let current_dir = env::current_dir().unwrap_or_default();
    #[cfg(target_arch = "wasm32")]
    let home_dir = current_dir;
    #[cfg(not(target_arch = "wasm32"))]
    let home_dir = home::home_dir().unwrap_or(current_dir);
    let base_path = home_dir.join(".ristretto");
    let mut installation_dir = base_path.join(version);

    if !installation_dir.exists() {
        let (version, file_name, archive) = util::get_runtime_archive(version).await?;
        installation_dir =
            extract_archive(version.as_str(), file_name.as_str(), &archive, &base_path)?;
        archive_version = version;
    }

    let class_path = get_class_path(&archive_version, &installation_dir)?;
    let class_loader = ClassLoader::new("bootstrap", class_path);
    Ok((archive_version, class_loader))
}

/// Get the class path for the given version.
///
/// # Errors
/// An error will be returned if the class path cannot be determined.
fn get_class_path(version: &str, installation_dir: &Path) -> Result<ClassPath> {
    #[cfg(target_os = "macos")]
    let installation_dir = installation_dir.join("Contents").join("Home");

    let class_path = if util::parse_major_version(version) <= 8 {
        let rt_jar_path = installation_dir.join("jre").join("lib").join("rt.jar");
        let class_path = rt_jar_path.to_string_lossy();
        class_path.to_string()
    } else {
        let jmods_path = installation_dir.join("jmods");
        let jmod_files = std::fs::read_dir(jmods_path)?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();
                if path.extension()?.to_string_lossy() == "jmod" {
                    Some(path)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        let mut class_paths = jmod_files
            .iter()
            .map(|path| path.to_string_lossy().to_string())
            .collect::<Vec<_>>();
        class_paths.sort_by(Ord::cmp);
        class_paths.join(":")
    };
    Ok(ClassPath::from(class_path))
}

/// Extract the archive to the installation directory.
///
/// # Errors
/// An error will be returned if the archive cannot be extracted.
#[instrument(level = "debug", skip(archive))]
fn extract_archive(
    version: &str,
    file_name: &str,
    archive: &Vec<u8>,
    out_dir: &PathBuf,
) -> Result<PathBuf> {
    fs::create_dir_all(out_dir)?;

    let Some(extension) = file_name.split('.').last() else {
        return Err(Error::ArchiveError(
            "No extension found in file name".to_string(),
        ));
    };

    let archive = io::Cursor::new(archive);
    let extract_dir = tempfile::tempdir_in(out_dir)?.into_path();
    debug!(
        "Extracting {file_name} to {}",
        extract_dir.to_string_lossy()
    );

    if extension == "zip" {
        let mut zip = zip::ZipArchive::new(archive)?;
        zip.extract(&extract_dir)?;
    } else {
        let decoder = GzDecoder::new(archive);
        let mut tar = Archive::new(decoder);
        tar.unpack(extract_dir.clone())?;
    };

    let runtime_dir = {
        let mut entries = std::fs::read_dir(&extract_dir)?;
        let Some(runtime_dir) = entries.next() else {
            return Err(Error::ArchiveError(
                "No directory found in archive".to_string(),
            ));
        };
        runtime_dir?
    };
    let runtime_dir = runtime_dir.path();
    let installation_dir = out_dir.join(version);

    // Rename the runtime directory to the installation directory. Another process may have
    // already installed the runtime, so we need to check if the installation directory exists.
    // If it does, we can ignore the error.
    let rename_result = fs::rename(runtime_dir.clone(), installation_dir.clone());
    if let Err(error) = rename_result {
        debug!(
            "Failed to rename {} to {}",
            runtime_dir.to_string_lossy(),
            installation_dir.to_string_lossy(),
        );
        if !installation_dir.exists() {
            return Err(Error::ArchiveError(error.to_string()));
        }
    }
    fs::remove_dir_all(&extract_dir)?;
    debug!(
        "Installed {version} to: {}",
        installation_dir.to_string_lossy()
    );

    Ok(installation_dir)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_class_loader_v8() -> Result<()> {
        let version = "8.422.05.1";
        let (archive_version, class_loader) = class_loader(version).await?;
        assert_eq!(version, archive_version);
        assert_eq!("bootstrap", class_loader.name());
        Ok(())
    }

    #[tokio::test]
    async fn test_class_loader_v21() -> Result<()> {
        let version = "21.0.4.7.1";
        let (archive_version, class_loader) = class_loader(version).await?;
        assert_eq!(version, archive_version);
        assert_eq!("bootstrap", class_loader.name());
        Ok(())
    }
}
