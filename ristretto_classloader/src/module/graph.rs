//! Module graph for efficient readability lookups.

use crate::module::resolution::ResolvedConfiguration;
use std::collections::{BTreeSet, HashMap, HashSet};

/// A graph representation of module readability for efficient lookups.
#[derive(Clone, Debug)]
pub struct ModuleGraph {
    /// Adjacency list: module -> set of modules it reads.
    reads: HashMap<String, BTreeSet<String>>,
    /// Reverse adjacency: module -> set of modules that read it.
    read_by: HashMap<String, BTreeSet<String>>,
}

impl ModuleGraph {
    /// Creates a new module graph from a resolved configuration.
    #[must_use]
    pub fn from_configuration(config: &ResolvedConfiguration) -> Self {
        let mut reads: HashMap<String, BTreeSet<String>> = HashMap::new();
        let mut read_by: HashMap<String, BTreeSet<String>> = HashMap::new();

        for module in config.modules() {
            let name = module.name().to_string();
            let module_reads = module.reads().clone();

            // Build reverse mapping
            for read in &module_reads {
                read_by
                    .entry(read.clone())
                    .or_default()
                    .insert(name.clone());
            }

            reads.insert(name, module_reads);
        }

        Self { reads, read_by }
    }

    /// Returns true if `from` reads `to`.
    #[must_use]
    pub fn reads(&self, from: &str, to: &str) -> bool {
        self.reads.get(from).is_some_and(|r| r.contains(to))
    }

    /// Returns all modules that `from` reads.
    #[must_use]
    pub fn reads_of(&self, from: &str) -> Option<&BTreeSet<String>> {
        self.reads.get(from)
    }

    /// Returns all modules that read `to`.
    #[must_use]
    pub fn readers_of(&self, to: &str) -> Option<&BTreeSet<String>> {
        self.read_by.get(to)
    }

    /// Returns the number of modules in the graph.
    #[must_use]
    pub fn len(&self) -> usize {
        self.reads.len()
    }

    /// Returns true if the graph is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.reads.is_empty()
    }

    /// Computes the transitive closure of readability from a starting module.
    #[must_use]
    pub fn transitive_reads(&self, from: &str) -> BTreeSet<String> {
        let mut result = BTreeSet::new();
        let mut visited = HashSet::new();
        self.collect_transitive_reads(from, &mut result, &mut visited);
        result
    }

    fn collect_transitive_reads(
        &self,
        from: &str,
        result: &mut BTreeSet<String>,
        visited: &mut HashSet<String>,
    ) {
        if visited.contains(from) {
            return;
        }
        visited.insert(from.to_string());

        if let Some(reads) = self.reads.get(from) {
            for read in reads {
                result.insert(read.clone());
                self.collect_transitive_reads(read, result, visited);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_graph_empty() {
        let graph = ModuleGraph {
            reads: HashMap::new(),
            read_by: HashMap::new(),
        };
        assert!(graph.is_empty());
        assert_eq!(graph.len(), 0);
    }

    #[test]
    fn test_module_graph_reads() {
        let mut reads = HashMap::new();
        reads.insert(
            "my.module".to_string(),
            BTreeSet::from(["java.base".to_string()]),
        );
        let graph = ModuleGraph {
            reads,
            read_by: HashMap::new(),
        };
        assert!(graph.reads("my.module", "java.base"));
        assert!(!graph.reads("my.module", "java.sql"));
    }
}
