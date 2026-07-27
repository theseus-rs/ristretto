#![cfg(not(target_family = "wasm"))]
#![expect(
    clippy::panic_in_result_fn,
    reason = "integration test assertions provide useful database output diagnostics"
)]

use ristretto_resolver::{ArtifactCoordinate, FileDestination, ResolutionRequest, Resolver};
use ristretto_vm::{ClassPath, ConfigurationBuilder, VM};
use std::error::Error;
use std::io::Cursor;
use std::path::PathBuf;
use std::sync::Arc;
use tempfile::TempDir;
use tokio::sync::Mutex;

const H2_VERSION: &str = "2.3.232";

fn expected_rows() -> Vec<(i32, String)> {
    vec![
        (1, "Alan Turing".to_string()),
        (2, "John von Neumann".to_string()),
    ]
}

fn selected_rows(output: &str) -> Option<Vec<(i32, String)>> {
    output
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let (id, name) = line.trim().split_once('|')?;
            Some((id.parse().ok()?, name.to_string()))
        })
        .collect()
}

#[tokio::test]
async fn test_jdbc_h2() -> Result<(), Box<dyn Error>> {
    let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let classes_directory = cargo_manifest.join("..").join("classes");
    let dependencies_directory = TempDir::new()?;
    let resolver = Resolver::builder().build()?;
    let request =
        ResolutionRequest::new(ArtifactCoordinate::new("com.h2database", "h2", H2_VERSION)?);
    let (_, download) = resolver
        .resolve_and_download(
            &request,
            &FileDestination::new(dependencies_directory.path()),
        )
        .await?;

    let mut class_path_entries = vec![classes_directory];
    class_path_entries.extend(
        download
            .artifacts
            .into_iter()
            .map(|artifact| artifact.output),
    );
    let class_path = ClassPath::from(&class_path_entries);
    let stdout = Arc::new(Mutex::new(Cursor::new(Vec::<u8>::new())));
    let configuration = ConfigurationBuilder::new()
        .class_path(class_path)
        .main_class("JDBC")
        .stdout(stdout.clone())
        .build()?;
    let vm = VM::new(configuration).await?;

    let result = vm.invoke_main(&[] as &[&str]).await?;
    assert!(result.is_none());
    vm.wait_for_non_daemon_threads().await?;

    let output = stdout.lock().await;
    let output = String::from_utf8(output.get_ref().clone())?;
    assert_eq!(selected_rows(&output), Some(expected_rows()));
    Ok(())
}

#[test]
fn test_selected_rows_are_valid() {
    let output = "1|Alan Turing\n2|John von Neumann\n";
    assert_eq!(selected_rows(output), Some(expected_rows()));
    assert_eq!(selected_rows("1|Alan Turing\ninvalid\n"), None);
}
