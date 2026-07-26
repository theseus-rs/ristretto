//! Resolve Spring Boot's web starter and print its complete annotated dependency tree.
//!
//! Resolve from Maven Central:
//!
//! ```text
//! cargo run -p resolver
//! ```
//!
//! Prefer artifacts already present in the standard local Maven repository, falling back to
//! Maven Central for anything missing:
//!
//! ```text
//! cargo run -p resolver -- --local-m2
//! ```
//!
//! Add `--local-m2-only` for an offline, local-only resolution, or `--download-to <DIRECTORY>` to
//! materialize every selected JAR after printing the tree.

#![cfg_attr(
    test,
    expect(
        clippy::panic_in_result_fn,
        reason = "the example test uses assertions while propagating resolver errors"
    )
)]

#[cfg(target_family = "wasm")]
fn main() {}

#[cfg(not(target_family = "wasm"))]
use ristretto_resolver::{
    ArtifactCoordinate, Error, FileDestination, NodeId, RemoteRepository, Resolution,
    ResolutionRequest, Resolver, Result,
};
#[cfg(not(target_family = "wasm"))]
use std::collections::{BTreeMap, BTreeSet, VecDeque};
#[cfg(not(target_family = "wasm"))]
use std::env;
#[cfg(not(target_family = "wasm"))]
use std::path::PathBuf;
#[cfg(not(target_family = "wasm"))]
use url::Url;

#[cfg(not(target_family = "wasm"))]
const DEFAULT_SPRING_BOOT_VERSION: &str = "3.5.4";

#[cfg(not(target_family = "wasm"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LocalM2Mode {
    Disabled,
    Preferred,
    Only,
}

#[cfg(not(target_family = "wasm"))]
#[derive(Debug)]
struct Options {
    spring_boot_version: String,
    local_m2: LocalM2Mode,
    download_to: Option<PathBuf>,
}

#[cfg(not(target_family = "wasm"))]
impl Options {
    fn parse() -> Result<Option<Self>> {
        let mut options = Self {
            spring_boot_version: DEFAULT_SPRING_BOOT_VERSION.to_string(),
            local_m2: LocalM2Mode::Disabled,
            download_to: None,
        };
        let mut arguments = env::args().skip(1).collect::<VecDeque<_>>();
        while let Some(argument) = arguments.pop_front() {
            match argument.as_str() {
                "--spring-boot-version" => {
                    let Some(version) = arguments.pop_front() else {
                        return Err(Error::InvalidConfiguration(
                            "--spring-boot-version requires a value".to_string(),
                        ));
                    };
                    options.spring_boot_version = version;
                }
                "--local-m2" => options.local_m2 = LocalM2Mode::Preferred,
                "--local-m2-only" => options.local_m2 = LocalM2Mode::Only,
                "--download-to" => {
                    let Some(directory) = arguments.pop_front() else {
                        return Err(Error::InvalidConfiguration(
                            "--download-to requires a directory".to_string(),
                        ));
                    };
                    options.download_to = Some(PathBuf::from(directory));
                }
                "-h" | "--help" => {
                    print_usage();
                    return Ok(None);
                }
                _ => {
                    return Err(Error::InvalidConfiguration(format!(
                        "unknown argument '{argument}'; use --help for usage"
                    )));
                }
            }
        }
        Ok(Some(options))
    }
}

#[cfg(not(target_family = "wasm"))]
#[tokio::main]
async fn main() -> Result<()> {
    let Some(options) = Options::parse()? else {
        return Ok(());
    };
    let (resolver, resolution) = resolve(&options).await?;

    println!(
        "Spring Boot {} dependency tree",
        options.spring_boot_version
    );
    print_tree(&resolution);
    for diagnostic in &resolution.diagnostics {
        eprintln!("resolution warning: {diagnostic}");
    }

    if let Some(directory) = options.download_to {
        let destination = FileDestination::new(&directory);
        let report = resolver.download(&resolution, &destination).await?;
        println!(
            "\nDownloaded {} selected artifacts to {}",
            report.artifacts.len(),
            directory.display()
        );
        for diagnostic in report.diagnostics {
            eprintln!("download warning: {diagnostic}");
        }
    }
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
async fn resolve(options: &Options) -> Result<(Resolver, Resolution)> {
    let resolver = build_resolver(options.local_m2)?;
    let root = ArtifactCoordinate::new(
        "org.springframework.boot",
        "spring-boot-starter-web",
        &options.spring_boot_version,
    )?;
    let resolution = resolver.resolve(&ResolutionRequest::new(root)).await?;
    Ok((resolver, resolution))
}

#[cfg(not(target_family = "wasm"))]
fn build_resolver(local_m2: LocalM2Mode) -> Result<Resolver> {
    let builder = Resolver::builder();
    match local_m2 {
        LocalM2Mode::Disabled => builder.build(),
        LocalM2Mode::Preferred => builder
            .repositories(vec![local_maven_repository()?, RemoteRepository::central()])
            .build(),
        LocalM2Mode::Only => builder
            .repositories(vec![local_maven_repository()?])
            .offline(true)
            .build(),
    }
}

#[cfg(not(target_family = "wasm"))]
fn local_maven_repository() -> Result<RemoteRepository> {
    let home = env::var_os("HOME").ok_or_else(|| {
        Error::InvalidConfiguration("HOME is not set; cannot locate ~/.m2/repository".to_string())
    })?;
    let path = PathBuf::from(home).join(".m2").join("repository");
    if !path.is_dir() {
        return Err(Error::InvalidConfiguration(format!(
            "local Maven repository does not exist: {}",
            path.display()
        )));
    }
    let url = Url::from_directory_path(&path).map_err(|()| {
        Error::InvalidConfiguration(format!(
            "cannot convert local Maven repository to a file URL: {}",
            path.display()
        ))
    })?;
    RemoteRepository::new("maven-local", url.as_str())
}

#[cfg(not(target_family = "wasm"))]
fn print_tree(resolution: &Resolution) {
    let mut children = BTreeMap::<NodeId, Vec<NodeId>>::new();
    for edge in &resolution.edges {
        children.entry(edge.from).or_default().push(edge.to);
    }
    let mut ancestors = BTreeSet::new();
    for (index, root) in resolution.roots.iter().enumerate() {
        let connector = if index + 1 == resolution.roots.len() {
            "└── "
        } else {
            "├── "
        };
        print_node(resolution, *root, "", connector, &children, &mut ancestors);
    }
}

#[cfg(not(target_family = "wasm"))]
fn print_node(
    resolution: &Resolution,
    id: NodeId,
    prefix: &str,
    connector: &str,
    children: &BTreeMap<NodeId, Vec<NodeId>>,
    ancestors: &mut BTreeSet<NodeId>,
) {
    let Some(node) = resolution.nodes.get(id.0) else {
        return;
    };
    println!(
        "{prefix}{connector}{} [{:?}, {:?}]",
        node.coordinate, node.scope, node.status
    );

    if !ancestors.insert(id) {
        return;
    }
    let child_prefix = if connector == "└── " {
        format!("{prefix}    ")
    } else {
        format!("{prefix}│   ")
    };
    if let Some(child_nodes) = children.get(&id) {
        for (index, child) in child_nodes.iter().enumerate() {
            let child_connector = if index + 1 == child_nodes.len() {
                "└── "
            } else {
                "├── "
            };
            print_node(
                resolution,
                *child,
                &child_prefix,
                child_connector,
                children,
                ancestors,
            );
        }
    }
    ancestors.remove(&id);
}

#[cfg(not(target_family = "wasm"))]
fn print_usage() {
    println!(
        "Usage: resolver [OPTIONS]\n\
         \n\
         Options:\n\
           --spring-boot-version <VERSION>  Spring Boot version (default: \
         {DEFAULT_SPRING_BOOT_VERSION})\n\
           --local-m2                      Prefer ~/.m2/repository, then Maven Central\n\
           --local-m2-only                 Resolve only from ~/.m2/repository in offline mode\n\
           --download-to <DIRECTORY>       Download every selected artifact\n\
           -h, --help                      Print this help"
    );
}

#[cfg(all(test, not(target_family = "wasm")))]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_resolve() -> Result<()> {
        let options = Options {
            spring_boot_version: DEFAULT_SPRING_BOOT_VERSION.to_string(),
            local_m2: LocalM2Mode::Disabled,
            download_to: None,
        };

        let (_resolver, resolution) = resolve(&options).await?;

        assert_eq!(1, resolution.roots.len());
        let root = resolution
            .roots
            .first()
            .and_then(|root_id| resolution.nodes.get(root_id.0))
            .ok_or_else(|| {
                Error::InvalidConfiguration("resolution did not contain a root node".to_string())
            })?;
        assert_eq!("org.springframework.boot", root.coordinate.key.group_id);
        assert_eq!("spring-boot-starter-web", root.coordinate.key.artifact_id);
        assert_eq!(DEFAULT_SPRING_BOOT_VERSION, root.coordinate.version);
        assert!(resolution.nodes.len() > 1);
        assert!(resolution.artifacts.iter().any(|artifact| {
            artifact.coordinate.key.group_id == "org.springframework"
                && artifact.coordinate.key.artifact_id == "spring-webmvc"
        }));
        Ok(())
    }
}
