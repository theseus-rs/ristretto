//! Declarative rules for changing transitive dependencies.
//!
//! An [`OverrideMatcher`] selects an artifact identity and can narrow the match by extension or
//! classifier. [`OverrideRule`] then excludes it, forces a version, or replaces it with another
//! coordinate. Applied transformations are retained as resolution events so graph diagnostics can
//! explain the resulting classpath.

use crate::{ArtifactCoordinate, ArtifactKey, Version};
use serde::{Deserialize, Serialize};

/// Matches dependency identities while permitting optional type/classifier narrowing.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct OverrideMatcher {
    /// Required group identifier.
    pub group_id: String,
    /// Required artifact identifier.
    pub artifact_id: String,
    /// Optional extension constraint.
    pub extension: Option<String>,
    /// Optional classifier constraint. `Some(None)` requires no classifier.
    pub classifier: Option<Option<String>>,
}

impl OverrideMatcher {
    /// Matches every type/classifier of one group and artifact.
    #[must_use]
    pub fn new(group_id: impl Into<String>, artifact_id: impl Into<String>) -> Self {
        Self {
            group_id: group_id.into(),
            artifact_id: artifact_id.into(),
            extension: None,
            classifier: None,
        }
    }

    /// Returns whether this matcher accepts a key.
    #[must_use]
    pub fn matches(&self, key: &ArtifactKey) -> bool {
        self.group_id == key.group_id
            && self.artifact_id == key.artifact_id
            && self
                .extension
                .as_ref()
                .is_none_or(|extension| extension == &key.extension)
            && self
                .classifier
                .as_ref()
                .is_none_or(|classifier| classifier == &key.classifier)
    }

    pub(crate) fn specificity(&self) -> u8 {
        u8::from(self.extension.is_some()) + u8::from(self.classifier.is_some())
    }
}

/// An action applied to matching transitive dependencies.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum OverrideAction {
    /// Forces an exact version while retaining artifact identity.
    ForceVersion(Version),
    /// Substitutes a complete artifact coordinate.
    Replace(ArtifactCoordinate),
    /// Removes the matching dependency edge.
    Exclude,
}

/// One transitive dependency override.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct OverrideRule {
    /// Dependency identity matcher.
    pub matcher: OverrideMatcher,
    /// Action applied to matching edges.
    pub action: OverrideAction,
}

impl OverrideRule {
    /// Creates an override rule.
    #[must_use]
    pub fn new(matcher: OverrideMatcher, action: OverrideAction) -> Self {
        Self { matcher, action }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn narrows_override_matchers_by_extension_and_classifier() {
        let key = ArtifactKey::new("org.example", "demo")
            .and_then(|key| key.with_extension("zip"))
            .and_then(|key| key.with_classifier("tests"))
            .expect("artifact key");
        let broad = OverrideMatcher::new("org.example", "demo");
        assert!(broad.matches(&key));
        assert_eq!(broad.specificity(), 0);

        let mut exact = broad.clone();
        exact.extension = Some("zip".to_string());
        exact.classifier = Some(Some("tests".to_string()));
        assert!(exact.matches(&key));
        assert_eq!(exact.specificity(), 2);
        exact.classifier = Some(None);
        assert!(!exact.matches(&key));
        exact.group_id = "other".to_string();
        assert!(!exact.matches(&key));
    }
}
