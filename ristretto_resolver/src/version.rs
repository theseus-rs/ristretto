//! Maven-compatible versions, selectors, and ranges.
//!
//! [`Version`] preserves its original repository spelling while comparing with Maven's qualifier,
//! separator, alias, and numeric rules. [`VersionSpec`] represents exact versions, `LATEST`,
//! `RELEASE`, and range unions; [`VersionRange`] exposes the bounds of one parsed interval. These
//! types are used consistently for metadata selection and dependency conflict validation.

use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt;
use std::str::FromStr;

/// A version with Maven-style comparison semantics.
#[derive(Debug, Clone, Eq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Version(String);

impl Version {
    /// Creates a version from its repository representation.
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Returns the original version string.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Version {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl FromStr for Version {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self> {
        if value.trim().is_empty() || value.trim() != value {
            return Err(Error::InvalidVersion(value.to_string()));
        }
        Ok(Self::new(value))
    }
}

impl PartialEq for Version {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        compare_lists(
            &parse_version_items(&self.0),
            &parse_version_items(&other.0),
        )
    }
}

/// A concrete version, special selector, or union of bounded version ranges.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum VersionSpec {
    /// A concrete version.
    Exact(Version),
    /// The most recently published version.
    Latest,
    /// The most recently published non-snapshot version.
    Release,
    /// One or more ranges, any of which may match.
    Ranges(Vec<VersionRange>),
}

impl VersionSpec {
    /// Returns whether a concrete version satisfies this specification.
    #[must_use]
    pub fn matches(&self, version: &Version) -> bool {
        match self {
            Self::Exact(expected) => expected == version,
            Self::Latest => true,
            Self::Release => !version.as_str().ends_with("-SNAPSHOT"),
            Self::Ranges(ranges) => ranges.iter().any(|range| range.matches(version)),
        }
    }

    /// Returns the exact version, if this is not a dynamic specification.
    #[must_use]
    pub fn exact(&self) -> Option<&Version> {
        match self {
            Self::Exact(version) => Some(version),
            Self::Latest | Self::Release | Self::Ranges(_) => None,
        }
    }
}

impl fmt::Display for VersionSpec {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Exact(version) => version.fmt(formatter),
            Self::Latest => formatter.write_str("LATEST"),
            Self::Release => formatter.write_str("RELEASE"),
            Self::Ranges(ranges) => {
                for (index, range) in ranges.iter().enumerate() {
                    if index > 0 {
                        formatter.write_str(",")?;
                    }
                    range.fmt(formatter)?;
                }
                Ok(())
            }
        }
    }
}

impl FromStr for VersionSpec {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self> {
        let value = value.trim();
        match value {
            "LATEST" => Ok(Self::Latest),
            "RELEASE" => Ok(Self::Release),
            _ if value.starts_with('[') || value.starts_with('(') => {
                Ok(Self::Ranges(parse_ranges(value)?))
            }
            _ => Ok(Self::Exact(value.parse()?)),
        }
    }
}

/// One bounded interval in a Maven version range.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct VersionRange {
    /// Lower endpoint and whether it is inclusive.
    pub lower: Option<(Version, bool)>,
    /// Upper endpoint and whether it is inclusive.
    pub upper: Option<(Version, bool)>,
}

impl VersionRange {
    /// Returns whether a version falls within this interval.
    #[must_use]
    pub fn matches(&self, version: &Version) -> bool {
        let lower_matches = self
            .lower
            .as_ref()
            .is_none_or(|(lower, inclusive)| version > lower || *inclusive && version == lower);
        let upper_matches = self
            .upper
            .as_ref()
            .is_none_or(|(upper, inclusive)| version < upper || *inclusive && version == upper);
        lower_matches && upper_matches
    }
}

impl fmt::Display for VersionRange {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(if self.lower.as_ref().is_some_and(|(_, value)| *value) {
            "["
        } else {
            "("
        })?;
        if let Some((version, _)) = &self.lower {
            version.fmt(formatter)?;
        }
        formatter.write_str(",")?;
        if let Some((version, _)) = &self.upper {
            version.fmt(formatter)?;
        }
        formatter.write_str(if self.upper.as_ref().is_some_and(|(_, value)| *value) {
            "]"
        } else {
            ")"
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum MavenItem {
    Numeric(String),
    Qualifier(String),
    List(Vec<Self>),
}

impl MavenItem {
    fn is_null(&self) -> bool {
        match self {
            Self::Numeric(value) => value == "0",
            Self::Qualifier(value) => qualifier_rank(value) == qualifier_rank(""),
            Self::List(items) => items.is_empty(),
        }
    }
}

#[derive(Debug)]
enum ArenaItem {
    Numeric(String),
    Qualifier(String),
    List(usize),
}

fn parse_version_items(value: &str) -> Vec<MavenItem> {
    let value = value.to_lowercase();
    let mut arena = vec![Vec::new()];
    let mut is_digit = false;
    let mut start = 0;
    for (index, character) in value.char_indices() {
        if character == '.' {
            if index == start {
                push_arena_item(&mut arena, ArenaItem::Numeric("0".to_string()));
            } else {
                push_parsed_item(
                    &mut arena,
                    is_digit,
                    value.get(start..index).unwrap_or_default(),
                );
            }
            start = index + character.len_utf8();
        } else if character == '-' {
            if index == start {
                push_arena_item(&mut arena, ArenaItem::Numeric("0".to_string()));
            } else {
                push_parsed_item(
                    &mut arena,
                    is_digit,
                    value.get(start..index).unwrap_or_default(),
                );
            }
            start = index + character.len_utf8();
            descend(&mut arena);
        } else if character.is_ascii_digit() {
            if !is_digit && index > start {
                if arena.last().is_some_and(|items| !items.is_empty()) {
                    descend(&mut arena);
                }
                let qualifier = value.get(start..index).unwrap_or_default();
                push_arena_item(
                    &mut arena,
                    ArenaItem::Qualifier(normalize_qualifier(qualifier, true)),
                );
                start = index;
                descend(&mut arena);
            }
            is_digit = true;
        } else {
            if is_digit && index > start {
                push_parsed_item(
                    &mut arena,
                    true,
                    value.get(start..index).unwrap_or_default(),
                );
                start = index;
                descend(&mut arena);
            }
            is_digit = false;
        }
    }
    if value.len() > start {
        if !is_digit && arena.last().is_some_and(|items| !items.is_empty()) {
            descend(&mut arena);
        }
        push_parsed_item(&mut arena, is_digit, value.get(start..).unwrap_or_default());
    }
    let mut items = materialize_list(0, &arena);
    normalize_list(&mut items);
    items
}

fn push_parsed_item(arena: &mut [Vec<ArenaItem>], is_digit: bool, value: &str) {
    let item = if is_digit {
        let trimmed = value.trim_start_matches('0');
        ArenaItem::Numeric(if trimmed.is_empty() {
            "0".to_string()
        } else {
            trimmed.to_string()
        })
    } else {
        ArenaItem::Qualifier(normalize_qualifier(value, false))
    };
    push_arena_item(arena, item);
}

fn push_arena_item(arena: &mut [Vec<ArenaItem>], item: ArenaItem) {
    if let Some(current) = arena.last_mut() {
        current.push(item);
    }
}

fn descend(arena: &mut Vec<Vec<ArenaItem>>) {
    let child = arena.len();
    push_arena_item(arena, ArenaItem::List(child));
    arena.push(Vec::new());
}

fn materialize_list(index: usize, arena: &[Vec<ArenaItem>]) -> Vec<MavenItem> {
    let Some(items) = arena.get(index) else {
        return Vec::new();
    };
    items
        .iter()
        .map(|item| match item {
            ArenaItem::Numeric(value) => MavenItem::Numeric(value.clone()),
            ArenaItem::Qualifier(value) => MavenItem::Qualifier(value.clone()),
            ArenaItem::List(child) => MavenItem::List(materialize_list(*child, arena)),
        })
        .collect()
}

fn normalize_list(items: &mut Vec<MavenItem>) {
    for item in &mut *items {
        if let MavenItem::List(children) = item {
            normalize_list(children);
        }
    }
    let mut remaining = items.len();
    while let Some(index) = remaining.checked_sub(1) {
        remaining = index;
        match items.get(index) {
            Some(item) if item.is_null() => drop(items.remove(index)),
            Some(MavenItem::List(_)) => {}
            Some(_) | None => break,
        }
    }
}

fn normalize_qualifier(value: &str, followed_by_digit: bool) -> String {
    let value = if followed_by_digit && value.len() == 1 {
        match value {
            "a" => "alpha",
            "b" => "beta",
            "m" => "milestone",
            other => other,
        }
    } else {
        value
    };
    match value {
        "cr" => "rc",
        "ga" | "final" | "release" => "",
        other => other,
    }
    .to_string()
}

fn compare_lists(left: &[MavenItem], right: &[MavenItem]) -> Ordering {
    let length = left.len().max(right.len());
    for index in 0..length {
        let ordering = if let Some(left) = left.get(index) {
            right
                .get(index)
                .map_or_else(|| compare_to_null(left), |right| compare_item(left, right))
        } else {
            right
                .get(index)
                .map_or(Ordering::Equal, |right| compare_to_null(right).reverse())
        };
        if ordering != Ordering::Equal {
            return ordering;
        }
    }
    Ordering::Equal
}

fn compare_to_null(item: &MavenItem) -> Ordering {
    match item {
        MavenItem::Numeric(value) => compare_numeric(value, "0"),
        MavenItem::Qualifier(value) => qualifier_rank(value).cmp(&qualifier_rank("")),
        MavenItem::List(items) => items
            .iter()
            .map(compare_to_null)
            .find(|ordering| *ordering != Ordering::Equal)
            .unwrap_or(Ordering::Equal),
    }
}

fn compare_item(left: &MavenItem, right: &MavenItem) -> Ordering {
    match (left, right) {
        (MavenItem::Numeric(left), MavenItem::Numeric(right)) => compare_numeric(left, right),
        (MavenItem::Numeric(_), MavenItem::Qualifier(_) | MavenItem::List(_))
        | (MavenItem::List(_), MavenItem::Qualifier(_)) => Ordering::Greater,
        (MavenItem::Qualifier(_), MavenItem::Numeric(_) | MavenItem::List(_))
        | (MavenItem::List(_), MavenItem::Numeric(_)) => Ordering::Less,
        (MavenItem::Qualifier(left), MavenItem::Qualifier(right)) => {
            qualifier_rank(left).cmp(&qualifier_rank(right))
        }
        (MavenItem::List(left), MavenItem::List(right)) => compare_lists(left, right),
    }
}

fn compare_numeric(left: &str, right: &str) -> Ordering {
    left.len().cmp(&right.len()).then_with(|| left.cmp(right))
}

fn qualifier_rank(value: &str) -> (u8, &str) {
    match value {
        "alpha" => (0, ""),
        "beta" => (1, ""),
        "milestone" => (2, ""),
        "rc" => (3, ""),
        "snapshot" => (4, ""),
        "" => (5, ""),
        "sp" => (6, ""),
        other => (7, other),
    }
}

fn parse_ranges(value: &str) -> Result<Vec<VersionRange>> {
    let mut ranges = Vec::new();
    let mut rest = value.trim();
    while !rest.is_empty() {
        // The initial range check and `range_tail` guarantee that each
        // interval starts with one of the two supported delimiters.
        let lower_inclusive = rest.starts_with('[');
        let closing_index = rest
            .find([']', ')'])
            .ok_or_else(|| Error::InvalidVersion(value.to_string()))?;
        let closing = rest
            .as_bytes()
            .get(closing_index)
            .copied()
            .ok_or_else(|| Error::InvalidVersion(value.to_string()))?;
        let body = rest
            .get(1..closing_index)
            .ok_or_else(|| Error::InvalidVersion(value.to_string()))?;
        let Some((lower, upper)) = body.split_once(',') else {
            if lower_inclusive && closing == b']' && !body.trim().is_empty() {
                let version: Version = body.trim().parse()?;
                ranges.push(VersionRange {
                    lower: Some((version.clone(), true)),
                    upper: Some((version, true)),
                });
                rest = range_tail(value, rest, closing_index)?;
                continue;
            }
            return Err(Error::InvalidVersion(value.to_string()));
        };
        if upper.contains(',') {
            return Err(Error::InvalidVersion(value.to_string()));
        }
        let lower = if lower.trim().is_empty() {
            None
        } else {
            Some((lower.trim().parse()?, lower_inclusive))
        };
        let upper = if upper.trim().is_empty() {
            None
        } else {
            Some((upper.trim().parse()?, closing == b']'))
        };
        if lower.is_none() && upper.is_none() {
            return Err(Error::InvalidVersion(value.to_string()));
        }
        ranges.push(VersionRange { lower, upper });
        rest = range_tail(value, rest, closing_index)?;
    }
    Ok(ranges)
}

fn range_tail<'a>(value: &str, rest: &'a str, closing_index: usize) -> Result<&'a str> {
    let tail = rest
        .get(closing_index + 1..)
        .ok_or_else(|| Error::InvalidVersion(value.to_string()))?
        .trim();
    if tail.is_empty() {
        return Ok(tail);
    }
    let tail = tail
        .strip_prefix(',')
        .ok_or_else(|| Error::InvalidVersion(value.to_string()))?
        .trim();
    if tail.is_empty() || !tail.starts_with(['[', '(']) {
        return Err(Error::InvalidVersion(value.to_string()));
    }
    Ok(tail)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orders_common_versions() {
        let mut versions = ["1.0", "1.0-sp", "1.0-rc1", "1.0-beta", "1.0.1"].map(Version::new);
        versions.sort();
        assert_eq!(
            versions.map(|version| version.to_string()),
            ["1.0-beta", "1.0-rc1", "1.0", "1.0-sp", "1.0.1"]
        );
    }

    #[test]
    fn follows_maven_separator_and_alias_ordering() {
        for equivalent in ["1", "1.0", "1.0.0", "1.0-ga", "1.0-final", "1.0-release"] {
            assert_eq!(Version::new("1"), Version::new(equivalent));
        }
        for (lower, higher) in [
            ("1.0.RC2", "1.0-RC3"),
            ("1.0-RC3", "1.0.1"),
            ("1.0.X2", "1.0-X3"),
            ("1.0-X3", "1.0.1"),
            ("1.0-alpha-1", "1.0-beta-1"),
            ("1.0-foo", "1.0-foo-2"),
            ("1.0-foo-2", "1.0-foo-10"),
        ] {
            assert!(
                Version::new(lower) < Version::new(higher),
                "expected {lower} < {higher}"
            );
        }
        assert_eq!(Version::new("1.0-a1"), Version::new("1.0-alpha-1"));
        assert_eq!(Version::new("1.0-cr1"), Version::new("1.0-rc1"));
        assert_eq!(Version::new("1.0.0.X1"), Version::new("1.0.0-X1"));
    }

    #[test]
    fn follows_maven_comparable_version_reference_vectors() {
        for versions in [
            &[
                "1-alpha2snapshot",
                "1-alpha2",
                "1-alpha-123",
                "1-beta-2",
                "1-beta123",
                "1-m2",
                "1-m11",
                "1-rc",
                "1-cr2",
                "1-rc123",
                "1-SNAPSHOT",
                "1",
                "1-sp",
                "1-sp2",
                "1-sp123",
                "1-abc",
                "1-def",
                "1-pom-1",
                "1-1-snapshot",
                "1-1",
                "1-2",
                "1-123",
            ][..],
            &[
                "2.0", "2.0.a", "2-1", "2.0.2", "2.0.123", "2.1.0", "2.1-a", "2.1b", "2.1-c",
                "2.1-1", "2.1.0.1", "2.2", "2.123", "11.a2", "11.a11", "11.b2", "11.b11", "11.m2",
                "11.m11", "11", "11.a", "11b", "11c", "11m",
            ][..],
        ] {
            for (left, right) in versions.iter().zip(versions.iter().skip(1)) {
                assert!(
                    Version::new(*left) < Version::new(*right),
                    "expected {left} < {right}"
                );
            }
        }

        for (left, right) in [
            ("1", "1-0"),
            ("1a", "1-a"),
            ("1a", "1.0.0-a"),
            ("1x", "1.0-x"),
            ("1ga", "1"),
            ("1release", "1"),
            ("1final", "1"),
            ("1a1", "1-alpha-1"),
            ("1b2", "1-beta-2"),
            ("1m3", "1-milestone-3"),
            ("2-abc", "2.0.0.abc"),
        ] {
            assert_eq!(
                Version::new(left),
                Version::new(right),
                "expected {left} == {right}"
            );
        }
        for (lower, higher) in [
            ("1-0.alpha", "1"),
            ("1-0.beta", "1"),
            ("6.1.0rc3", "6.1.0"),
            ("6.1.0rc3", "6.1H.5-beta"),
            ("6.1.0", "6.1H.5-beta"),
        ] {
            assert!(
                Version::new(lower) < Version::new(higher),
                "expected {lower} < {higher}"
            );
        }
    }

    #[test]
    #[expect(
        clippy::panic_in_result_fn,
        reason = "assertions provide clearer diagnostics in unit tests"
    )]
    fn parses_union_ranges() -> Result<()> {
        let range: VersionSpec = "(,1.0],[1.2,)".parse()?;
        assert!(range.matches(&Version::new("0.9")));
        assert!(!range.matches(&Version::new("1.1")));
        assert!(range.matches(&Version::new("2.0")));
        Ok(())
    }

    #[test]
    fn rejects_malformed_range_unions() {
        for value in ["[1,2][3,4]", "[1,2],", "[1,2],,[3,4]", "[1,2,3]"] {
            assert!(value.parse::<VersionSpec>().is_err(), "{value}");
        }
    }

    #[test]
    #[expect(
        clippy::panic_in_result_fn,
        reason = "assertions provide clearer diagnostics in unit tests"
    )]
    fn covers_selectors_ranges_and_version_parser_edges() -> Result<()> {
        assert!("1.0".parse::<Version>().is_ok());
        assert!("".parse::<Version>().is_err());
        assert!(" 1.0".parse::<Version>().is_err());

        let exact: VersionSpec = "1.0".parse()?;
        assert!(exact.matches(&Version::new("1.0")));
        assert!(!exact.matches(&Version::new("2.0")));
        assert_eq!(exact.exact(), Some(&Version::new("1.0")));
        assert_eq!(exact.to_string(), "1.0");
        assert!(VersionSpec::Latest.matches(&Version::new("1.0-SNAPSHOT")));
        assert!(VersionSpec::Latest.exact().is_none());
        assert_eq!(VersionSpec::Latest.to_string(), "LATEST");
        assert!(VersionSpec::Release.matches(&Version::new("1.0")));
        assert!(!VersionSpec::Release.matches(&Version::new("1.0-SNAPSHOT")));
        assert_eq!(VersionSpec::Release.to_string(), "RELEASE");

        let ranges: VersionSpec = "(,1],[2,)".parse()?;
        assert!(ranges.matches(&Version::new("1")));
        assert!(ranges.matches(&Version::new("2")));
        assert!(!ranges.matches(&Version::new("1.5")));
        assert_eq!(ranges.to_string(), "(,1],[2,)");
        assert_eq!("[1]".parse::<VersionSpec>()?.to_string(), "[1,1]");

        for value in [".1", "-1", "1..2", "1--2", "a1", "1a", "0001", "1-"] {
            let _ = Version::new(value).cmp(&Version::new("1"));
        }
        for invalid in [
            "[1,2",
            "(1)",
            "(,)",
            "[1,2,3]",
            "[1,2] trailing",
            "[1,2],",
            "[1,2],invalid",
        ] {
            assert!(invalid.parse::<VersionSpec>().is_err(), "{invalid}");
        }
        Ok(())
    }

    #[test]
    fn covers_internal_comparison_item_combinations() {
        let numeric = MavenItem::Numeric("1".to_string());
        let zero = MavenItem::Numeric("0".to_string());
        let qualifier = MavenItem::Qualifier("alpha".to_string());
        let release = MavenItem::Qualifier(String::new());
        let list = MavenItem::List(vec![numeric.clone()]);
        let empty_list = MavenItem::List(Vec::new());
        assert!(!numeric.is_null());
        assert!(zero.is_null());
        assert!(release.is_null());
        assert!(empty_list.is_null());
        assert_eq!(compare_to_null(&numeric), Ordering::Greater);
        assert_eq!(compare_to_null(&qualifier), Ordering::Less);
        assert_eq!(compare_to_null(&list), Ordering::Greater);
        assert_eq!(compare_item(&numeric, &qualifier), Ordering::Greater);
        assert_eq!(compare_item(&qualifier, &numeric), Ordering::Less);
        assert_eq!(compare_item(&list, &qualifier), Ordering::Greater);
        assert_eq!(compare_item(&qualifier, &list), Ordering::Less);
        assert_eq!(compare_item(&list, &numeric), Ordering::Less);
        assert_eq!(compare_item(&numeric, &list), Ordering::Greater);
        assert_eq!(
            compare_item(&qualifier, &MavenItem::Qualifier("beta".to_string())),
            Ordering::Less
        );
        assert_eq!(
            compare_item(&list, &MavenItem::List(vec![numeric.clone()])),
            Ordering::Equal
        );
        assert_eq!(compare_lists(&[], &[]), Ordering::Equal);
        assert_eq!(
            compare_lists(std::slice::from_ref(&numeric), &[]),
            Ordering::Greater
        );
        assert_eq!(compare_lists(&[], &[numeric]), Ordering::Less);
        assert!(materialize_list(99, &[]).is_empty());
        let mut empty: [Vec<ArenaItem>; 0] = [];
        push_arena_item(&mut empty, ArenaItem::Numeric("1".to_string()));
        assert_eq!(normalize_qualifier("z", true), "z");
        assert_eq!(normalize_qualifier("m", true), "milestone");
        assert_eq!(normalize_qualifier("final", false), "");
    }

    #[test]
    fn propagates_formatter_failures() {
        #[derive(Debug)]
        struct FailingWriter;

        impl fmt::Write for FailingWriter {
            fn write_str(&mut self, _value: &str) -> fmt::Result {
                Err(fmt::Error)
            }
        }

        let mut writer = FailingWriter;
        let range = VersionRange {
            lower: Some((Version::new("1"), false)),
            upper: Some((Version::new("2"), true)),
        };
        assert!(fmt::write(&mut writer, format_args!("{range}")).is_err());
    }
}
