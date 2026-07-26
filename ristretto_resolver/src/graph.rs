//! Annotated dependency graphs, lock snapshots, and graph inspection.
//!
//! A [`Resolution`] retains every declared occurrence, including conflicts, exclusions, optional
//! edges, overrides, and cycles, while its artifact list contains only materializable selections.
//! Use [`Resolution::selected_dependencies`], [`Resolution::transitive_dependencies`],
//! [`Resolution::paths_to`], and [`Resolution::explain`] for higher-level inspection.
//! [`ResolutionLock`] provides a credential-free graph snapshot whose repository configuration is
//! validated and reattached before download.

use crate::{ArtifactCoordinate, ArtifactKey, Error, RemoteRepository, Result};
use ristretto_pom::DependencyScope;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fmt::Write;

/// Stable index of a node within one [`Resolution`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
#[serde(transparent)]
pub struct NodeId(pub usize);

/// Why a node is or is not present in the selected classpath.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum NodeStatus {
    /// Selected by mediation.
    Selected,
    /// Not selected because another node won the conflict.
    Conflict {
        /// Winning node.
        winner: NodeId,
    },
    /// Removed by a POM exclusion.
    Excluded,
    /// Removed because it is optional on a transitive edge.
    Optional,
    /// Removed by a caller override rule.
    OverriddenOut,
    /// Repeated edge forming a dependency cycle.
    Cycle,
}

/// A transformation recorded while resolving a dependency.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum ResolutionEvent {
    /// A dependency-management entry supplied or changed the version.
    Managed {
        /// Version before management.
        requested: Option<String>,
        /// Version after management.
        selected: String,
    },
    /// A version range or special selector chose a concrete version.
    VersionSelected {
        /// Original selector.
        specification: String,
        /// Concrete result.
        selected: String,
    },
    /// A POM relocation changed coordinates.
    Relocated {
        /// Coordinate before relocation.
        from: ArtifactCoordinate,
        /// Coordinate after relocation.
        to: ArtifactCoordinate,
    },
    /// A caller rule changed the dependency.
    Override {
        /// Human-readable action.
        action: String,
    },
    /// Conflict mediation selected another node.
    Mediated {
        /// Winning node.
        winner: NodeId,
    },
}

/// One declared dependency occurrence in the annotated graph.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct DependencyNode {
    /// Graph-local identifier.
    pub id: NodeId,
    /// Coordinate after management, dynamic version selection, and overrides.
    pub coordinate: ArtifactCoordinate,
    /// Coordinate as first declared on this edge, when it was complete.
    pub requested: Option<ArtifactCoordinate>,
    /// Effective scope at this point in the graph.
    pub scope: DependencyScope,
    /// Whether the declaration was optional.
    pub optional: bool,
    /// Shortest distance from a root.
    pub depth: usize,
    /// Selection status.
    pub status: NodeStatus,
    /// Ordered transformations applied to this occurrence.
    pub events: Vec<ResolutionEvent>,
    /// Repository that supplied the descriptor.
    pub repository_id: Option<String>,
}

/// A directed declaration edge.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct DependencyEdge {
    /// Declaring parent.
    pub from: NodeId,
    /// Declared child.
    pub to: NodeId,
}

/// A downloadable artifact selected by resolution.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ResolvedArtifact {
    /// Node represented by this artifact.
    pub node: NodeId,
    /// Logical artifact coordinate.
    pub coordinate: ArtifactCoordinate,
    /// Version used in the remote filename, which differs for timestamped snapshots.
    pub file_version: String,
    /// Repository identifier.
    pub repository_id: String,
    /// Fully resolved source URL.
    pub url: String,
}

/// Complete successful dependency resolution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Resolution {
    /// Root nodes in request order.
    pub roots: Vec<NodeId>,
    /// All selected and omitted dependency occurrences.
    pub nodes: Vec<DependencyNode>,
    /// Declaration edges.
    pub edges: Vec<DependencyEdge>,
    /// Selected main artifacts in deterministic classpath order.
    pub artifacts: Vec<ResolvedArtifact>,
    /// Repositories considered, after mirror selection.
    #[serde(skip)]
    pub repositories: Vec<RemoteRepository>,
    /// Non-fatal compatibility and repository diagnostics.
    pub diagnostics: Vec<String>,
    /// Whether materialization may retry equivalent coordinates in later repositories.
    ///
    /// Live resolutions enable fallback. Reattached lockfiles disable it so a
    /// timestamped snapshot or other locked artifact cannot silently change.
    #[serde(skip)]
    pub repository_fallback: bool,
}

/// Credential-free, serializable snapshot of a resolved dependency graph.
///
/// Repository credentials and proxy state are intentionally excluded. Call
/// [`ResolutionLock::attach_repositories`] with runtime repository configuration
/// before materializing artifacts.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ResolutionLock {
    /// Lockfile format version.
    pub format_version: u32,
    /// Root nodes in request order.
    pub roots: Vec<NodeId>,
    /// All selected and omitted dependency occurrences.
    pub nodes: Vec<DependencyNode>,
    /// Declaration edges.
    pub edges: Vec<DependencyEdge>,
    /// Selected artifacts and their repository provenance.
    pub artifacts: Vec<ResolvedArtifact>,
    /// Non-fatal compatibility and repository diagnostics.
    pub diagnostics: Vec<String>,
}

impl Resolution {
    /// Creates a credential-free serializable lock snapshot.
    ///
    /// # Errors
    ///
    /// Returns an error when the resolution contains a local `systemPath`
    /// artifact, because an arbitrary machine-local file cannot be represented
    /// as a portable repository lock entry.
    pub fn to_lock(&self) -> Result<ResolutionLock> {
        if self
            .artifacts
            .iter()
            .any(|artifact| artifact.repository_id == "__system")
        {
            return Err(Error::InvalidConfiguration(
                "resolution locks cannot contain local systemPath artifacts".to_string(),
            ));
        }
        Ok(ResolutionLock {
            format_version: 1,
            roots: self.roots.clone(),
            nodes: self.nodes.clone(),
            edges: self.edges.clone(),
            artifacts: self.artifacts.clone(),
            diagnostics: self.diagnostics.clone(),
        })
    }

    /// Returns selected nodes in deterministic classpath order without duplicates.
    #[must_use]
    pub fn selected_dependencies(&self) -> Vec<&DependencyNode> {
        let mut seen = BTreeSet::new();
        self.nodes
            .iter()
            .filter(|node| node.status == NodeStatus::Selected)
            .filter(|node| seen.insert(node.coordinate.key.clone()))
            .collect()
    }

    /// Returns selected transitive nodes, excluding roots.
    #[must_use]
    pub fn transitive_dependencies(&self) -> Vec<&DependencyNode> {
        let roots = self.roots.iter().copied().collect::<BTreeSet<_>>();
        self.selected_dependencies()
            .into_iter()
            .filter(|node| !roots.contains(&node.id))
            .collect()
    }

    /// Returns every root-to-node path for a selected artifact identity.
    #[must_use]
    pub fn paths_to(&self, key: &ArtifactKey) -> Vec<Vec<NodeId>> {
        let targets = self
            .nodes
            .iter()
            .filter(|node| &node.coordinate.key == key)
            .map(|node| node.id)
            .collect::<BTreeSet<_>>();
        let mut outgoing: BTreeMap<NodeId, Vec<NodeId>> = BTreeMap::new();
        for edge in &self.edges {
            outgoing.entry(edge.from).or_default().push(edge.to);
        }
        let mut paths = Vec::new();
        let mut queue = self
            .roots
            .iter()
            .map(|root| vec![*root])
            .collect::<VecDeque<_>>();
        while let Some(path) = queue.pop_front() {
            // Every queued path is initialized with a root and only extended.
            let last = path.last().copied().unwrap_or(NodeId(usize::MAX));
            if targets.contains(&last) {
                paths.push(path.clone());
            }
            if let Some(children) = outgoing.get(&last) {
                for child in children {
                    if !path.contains(child) {
                        let mut child_path = path.clone();
                        child_path.push(*child);
                        queue.push_back(child_path);
                    }
                }
            }
        }
        paths
    }

    /// Returns a concise explanation of a node's transformations and selection state.
    #[must_use]
    pub fn explain(&self, node: NodeId) -> Option<String> {
        let node = self.nodes.get(node.0)?;
        let mut explanation = format!("{}: {:?}", node.coordinate, node.status);
        for event in &node.events {
            let _ = write!(explanation, "\n- {event:?}");
        }
        Some(explanation)
    }
}

impl ResolutionLock {
    /// Reattaches runtime repository configuration and validates all locked provenance.
    ///
    /// # Errors
    ///
    /// Returns an error when the format is unsupported, graph indexes are invalid,
    /// a repository is missing, or an artifact URL does not match its coordinate.
    #[expect(
        clippy::too_many_lines,
        reason = "lock attachment validates graph structure, coordinates, provenance, and URLs as one trust boundary"
    )]
    pub fn attach_repositories(self, repositories: Vec<RemoteRepository>) -> Result<Resolution> {
        if self.format_version != 1 {
            return Err(Error::InvalidConfiguration(format!(
                "unsupported resolution lock format {}",
                self.format_version
            )));
        }
        let mut repository_ids = BTreeSet::new();
        for repository in &repositories {
            repository.validate()?;
            if !repository_ids.insert(repository.id.as_str()) {
                return Err(Error::InvalidConfiguration(
                    "resolution lock repositories contain duplicate ids".to_string(),
                ));
            }
        }
        let node_count = self.nodes.len();
        if self
            .nodes
            .iter()
            .enumerate()
            .any(|(index, node)| node.id.0 != index)
            || self.roots.iter().any(|node| node.0 >= node_count)
            || self
                .edges
                .iter()
                .any(|edge| edge.from.0 >= node_count || edge.to.0 >= node_count)
            || self
                .artifacts
                .iter()
                .any(|artifact| artifact.node.0 >= node_count)
        {
            return Err(Error::InvalidConfiguration(
                "resolution lock contains invalid graph indexes".to_string(),
            ));
        }
        for node in &self.nodes {
            validate_coordinate(&node.coordinate)?;
            validate_optional_coordinate(node.requested.as_ref())?;
            if node
                .repository_id
                .as_ref()
                .is_some_and(|id| !repository_ids.contains(id.as_str()))
            {
                return Err(Error::InvalidConfiguration(format!(
                    "resolution lock requires repository '{}'",
                    node.repository_id.as_deref().unwrap_or_default()
                )));
            }
            if let NodeStatus::Conflict { winner } = &node.status {
                let winner_node = self.nodes.get(winner.0).ok_or_else(|| {
                    Error::InvalidConfiguration(
                        "resolution lock contains an invalid conflict winner".to_string(),
                    )
                })?;
                if winner_node.status != NodeStatus::Selected
                    || winner_node.coordinate.key != node.coordinate.key
                {
                    return Err(Error::InvalidConfiguration(
                        "resolution lock conflict winner does not select the same artifact"
                            .to_string(),
                    ));
                }
            }
            for event in &node.events {
                match event {
                    ResolutionEvent::Mediated { winner } => {
                        if self
                            .nodes
                            .get(winner.0)
                            .is_none_or(|winner| winner.status != NodeStatus::Selected)
                        {
                            return Err(Error::InvalidConfiguration(
                                "resolution lock contains an invalid mediation winner".to_string(),
                            ));
                        }
                    }
                    ResolutionEvent::Relocated { from, to } => {
                        validate_coordinate(from)?;
                        validate_coordinate(to)?;
                    }
                    ResolutionEvent::Managed { .. }
                    | ResolutionEvent::VersionSelected { .. }
                    | ResolutionEvent::Override { .. } => {}
                }
            }
        }
        let mut artifact_identities = BTreeSet::new();
        for artifact in &self.artifacts {
            if !artifact_identities.insert((
                artifact.node,
                artifact.coordinate.clone(),
                artifact.file_version.clone(),
                artifact.repository_id.clone(),
                artifact.url.clone(),
            )) {
                return Err(Error::InvalidConfiguration(format!(
                    "resolution lock contains duplicate artifact {}",
                    artifact.coordinate
                )));
            }
            drop(ArtifactCoordinate::from_key(
                artifact.coordinate.key.clone(),
                artifact.coordinate.version.clone(),
            )?);
            drop(ArtifactCoordinate::from_key(
                artifact.coordinate.key.clone(),
                artifact.file_version.clone(),
            )?);
            let repository = repositories
                .iter()
                .find(|repository| repository.id == artifact.repository_id)
                .ok_or_else(|| {
                    Error::InvalidConfiguration(format!(
                        "resolution lock requires repository '{}'",
                        artifact.repository_id
                    ))
                })?;
            // Artifact node indexes were validated before this loop.
            #[expect(
                clippy::indexing_slicing,
                reason = "all artifact indexes were bounds-checked before this loop"
            )]
            let node = &self.nodes[artifact.node.0];
            if node.status != NodeStatus::Selected
                || node.coordinate.key.group_id != artifact.coordinate.key.group_id
                || node.coordinate.key.artifact_id != artifact.coordinate.key.artifact_id
                || node.coordinate.version != artifact.coordinate.version
                || node.repository_id.as_deref() != Some(&artifact.repository_id)
            {
                return Err(Error::InvalidConfiguration(format!(
                    "locked artifact {} does not match its selected graph node",
                    artifact.coordinate
                )));
            }
            let expected = artifact_resource_url(repository, artifact)?;
            if artifact.url != expected {
                return Err(Error::InvalidConfiguration(format!(
                    "locked artifact URL does not match {} in repository '{}'",
                    artifact.coordinate, repository.id
                )));
            }
        }
        Ok(Resolution {
            roots: self.roots,
            nodes: self.nodes,
            edges: self.edges,
            artifacts: self.artifacts,
            repositories,
            diagnostics: self.diagnostics,
            repository_fallback: false,
        })
    }
}

fn validate_coordinate(coordinate: &ArtifactCoordinate) -> Result<()> {
    ArtifactCoordinate::from_key(coordinate.key.clone(), coordinate.version.clone()).map(drop)
}

fn validate_optional_coordinate(coordinate: Option<&ArtifactCoordinate>) -> Result<()> {
    coordinate.map_or(Ok(()), validate_coordinate)
}

fn artifact_resource_url(
    repository: &RemoteRepository,
    artifact: &ResolvedArtifact,
) -> Result<String> {
    repository.resource_url(
        &artifact
            .coordinate
            .artifact_path_with_version(&artifact.file_version),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use ristretto_pom::DependencyScope;

    fn sample() -> Result<Resolution> {
        let repository = RemoteRepository::new("repo", "https://repo.example/")?;
        let root = ArtifactCoordinate::new("org.example", "root", "1.0")?;
        let child = ArtifactCoordinate::new("org.example", "child", "1.0")?;
        let artifact = ResolvedArtifact {
            node: NodeId(1),
            coordinate: child.clone(),
            file_version: "1.0".to_string(),
            repository_id: "repo".to_string(),
            url: repository.resource_url(&child.artifact_path())?,
        };
        Ok(Resolution {
            roots: vec![NodeId(0)],
            nodes: vec![
                DependencyNode {
                    id: NodeId(0),
                    coordinate: root.clone(),
                    requested: Some(root),
                    scope: DependencyScope::Compile,
                    optional: false,
                    depth: 0,
                    status: NodeStatus::Selected,
                    events: Vec::new(),
                    repository_id: Some("repo".to_string()),
                },
                DependencyNode {
                    id: NodeId(1),
                    coordinate: child.clone(),
                    requested: Some(child),
                    scope: DependencyScope::Runtime,
                    optional: false,
                    depth: 1,
                    status: NodeStatus::Selected,
                    events: vec![ResolutionEvent::VersionSelected {
                        specification: "[1,2)".to_string(),
                        selected: "1.0".to_string(),
                    }],
                    repository_id: Some("repo".to_string()),
                },
            ],
            edges: vec![DependencyEdge {
                from: NodeId(0),
                to: NodeId(1),
            }],
            artifacts: vec![artifact],
            repositories: vec![repository],
            diagnostics: vec!["diagnostic".to_string()],
            repository_fallback: true,
        })
    }

    #[test]
    #[expect(
        clippy::panic_in_result_fn,
        reason = "assertions provide clearer diagnostics in unit tests"
    )]
    fn inspects_and_reattaches_resolution_graphs() -> Result<()> {
        let resolution = sample()?;
        assert_eq!(resolution.selected_dependencies().len(), 2);
        assert_eq!(resolution.transitive_dependencies().len(), 1);
        assert_eq!(
            resolution.paths_to(&ArtifactKey::new("org.example", "child")?),
            vec![vec![NodeId(0), NodeId(1)]]
        );
        assert!(
            resolution
                .explain(NodeId(1))
                .is_some_and(|explanation| explanation.contains("VersionSelected"))
        );
        assert!(resolution.explain(NodeId(99)).is_none());
        assert!(
            resolution
                .paths_to(&ArtifactKey::new("org.example", "missing")?)
                .is_empty()
        );

        let repositories = resolution.repositories.clone();
        let restored = resolution.to_lock()?.attach_repositories(repositories)?;
        assert!(!restored.repository_fallback);
        assert_eq!(restored.nodes, resolution.nodes);
        Ok(())
    }

    #[test]
    #[expect(
        clippy::panic_in_result_fn,
        clippy::too_many_lines,
        reason = "one table-style test enumerates every independent lock invariant"
    )]
    fn rejects_tampered_resolution_locks() -> Result<()> {
        let resolution = sample()?;
        let repositories = resolution.repositories.clone();
        validate_optional_coordinate(None)?;

        let mut lock = resolution.to_lock()?;
        lock.format_version = 99;
        assert!(lock.attach_repositories(repositories.clone()).is_err());

        let mut lock = resolution.to_lock()?;
        lock.roots = vec![NodeId(99)];
        assert!(lock.attach_repositories(repositories.clone()).is_err());

        let mut lock = resolution.to_lock()?;
        lock.nodes.get_mut(1).expect("child").id = NodeId(9);
        assert!(lock.attach_repositories(repositories.clone()).is_err());

        let mut lock = resolution.to_lock()?;
        lock.artifacts.get_mut(0).expect("artifact").repository_id = "missing".to_string();
        assert!(lock.attach_repositories(repositories.clone()).is_err());

        let mut lock = resolution.to_lock()?;
        lock.artifacts.get_mut(0).expect("artifact").url =
            "https://repo.example/other.jar".to_string();
        assert!(lock.attach_repositories(repositories.clone()).is_err());

        let mut lock = resolution.to_lock()?;
        lock.artifacts
            .get_mut(0)
            .expect("artifact")
            .coordinate
            .version = "..".to_string();
        assert!(lock.attach_repositories(repositories.clone()).is_err());

        let mut lock = resolution.to_lock()?;
        lock.nodes.get_mut(1).expect("child").status = NodeStatus::Excluded;
        assert!(lock.attach_repositories(repositories.clone()).is_err());

        let mut lock = resolution.to_lock()?;
        lock.edges.get_mut(0).expect("edge").to = NodeId(99);
        assert!(lock.attach_repositories(repositories.clone()).is_err());

        let mut lock = resolution.to_lock()?;
        lock.artifacts.get_mut(0).expect("artifact").node = NodeId(99);
        assert!(lock.attach_repositories(repositories.clone()).is_err());

        let mut lock = resolution.to_lock()?;
        lock.nodes.get_mut(0).expect("root").coordinate.version = "..".to_string();
        assert!(lock.attach_repositories(repositories.clone()).is_err());

        let mut lock = resolution.to_lock()?;
        lock.nodes
            .get_mut(1)
            .expect("child")
            .requested
            .as_mut()
            .expect("requested")
            .version = "..".to_string();
        assert!(lock.attach_repositories(repositories.clone()).is_err());

        let mut lock = resolution.to_lock()?;
        lock.artifacts.get_mut(0).expect("artifact").file_version = "..".to_string();
        assert!(lock.attach_repositories(repositories.clone()).is_err());

        let mut lock = resolution.to_lock()?;
        lock.artifacts
            .push(lock.artifacts.first().expect("artifact").clone());
        assert!(lock.attach_repositories(repositories.clone()).is_err());

        let mut lock = resolution.to_lock()?;
        lock.artifacts
            .get_mut(0)
            .expect("artifact")
            .coordinate
            .key
            .artifact_id = "other".to_string();
        assert!(lock.attach_repositories(repositories.clone()).is_err());

        let repository = repositories.first().expect("repository").clone();
        assert!(
            resolution
                .to_lock()?
                .attach_repositories(vec![repository.clone(), repository])
                .is_err()
        );

        let mut invalid_repository = repositories.first().expect("repository").clone();
        invalid_repository.url = "https://repo.example".to_string();
        assert!(
            resolution
                .to_lock()?
                .attach_repositories(vec![invalid_repository])
                .is_err()
        );

        let mut lock = resolution.to_lock()?;
        lock.nodes.get_mut(0).expect("root").repository_id = Some("missing".to_string());
        assert!(lock.attach_repositories(repositories.clone()).is_err());

        let mut lock = resolution.to_lock()?;
        lock.nodes.get_mut(1).expect("child").status = NodeStatus::Conflict { winner: NodeId(99) };
        assert!(lock.attach_repositories(repositories.clone()).is_err());

        let mut lock = resolution.to_lock()?;
        lock.nodes.get_mut(1).expect("child").status = NodeStatus::Conflict { winner: NodeId(0) };
        assert!(lock.attach_repositories(repositories.clone()).is_err());

        let mut lock = resolution.to_lock()?;
        lock.nodes
            .get_mut(1)
            .expect("child")
            .events
            .push(ResolutionEvent::Mediated { winner: NodeId(99) });
        assert!(lock.attach_repositories(repositories.clone()).is_err());

        let mut lock = resolution.to_lock()?;
        lock.nodes
            .get_mut(1)
            .expect("child")
            .events
            .push(ResolutionEvent::Relocated {
                from: ArtifactCoordinate::new("org.example", "old", "1")?,
                to: ArtifactCoordinate::new("org.example", "new", "1")?,
            });
        if let Some(ResolutionEvent::Relocated { to, .. }) =
            lock.nodes.get_mut(1).expect("child").events.last_mut()
        {
            to.version = "..".to_string();
        }
        assert!(lock.attach_repositories(repositories.clone()).is_err());

        let mut lock = resolution.to_lock()?;
        lock.nodes
            .get_mut(1)
            .expect("child")
            .events
            .push(ResolutionEvent::Relocated {
                from: ArtifactCoordinate::new("org.example", "old", "1")?,
                to: ArtifactCoordinate::new("org.example", "new", "1")?,
            });
        if let Some(ResolutionEvent::Relocated { from, .. }) =
            lock.nodes.get_mut(1).expect("child").events.last_mut()
        {
            from.version = "..".to_string();
        }
        assert!(lock.attach_repositories(repositories).is_err());
        Ok(())
    }
}
