//! Evaluation of Maven 4 profile activation conditions.
//!
//! Conditions are evaluated while constructing an effective POM. The expression language
//! supports Maven property lookups, boolean and comparison operators, version predicates, and
//! the file and string helpers used by Maven 4 profiles. Inputs come exclusively from
//! [`ResolutionContext`], which keeps activation deterministic and prevents condition evaluation
//! from reading ambient process state unexpectedly.

use crate::{ResolutionContext, Version, VersionSpec};
use fancy_regex::RegexBuilder;
use std::cmp::Ordering;
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq)]
enum Value {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
}

impl Value {
    fn as_bool(&self) -> bool {
        match self {
            Self::Null => false,
            Self::Bool(value) => *value,
            Self::Number(value) => number_as_i32(*value) != 0,
            Self::String(value) => !value.trim().is_empty(),
        }
    }

    fn as_number(&self) -> Option<f64> {
        match self {
            Self::Number(value) => Some(*value),
            Self::Bool(value) => Some(if *value { 1.0 } else { 0.0 }),
            Self::String(value) => value.parse().ok(),
            Self::Null => None,
        }
    }

    fn as_index(&self) -> Option<usize> {
        let value = number_as_i32(self.as_number()?);
        usize::try_from(value).ok()
    }

    fn as_string(&self) -> String {
        match self {
            Self::Null => "null".to_string(),
            Self::Bool(value) => value.to_string(),
            Self::Number(value) if value.fract() == 0.0 => format!("{value:.0}"),
            Self::Number(value) => value.to_string(),
            Self::String(value) => value.clone(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operator {
    Or,
    And,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Add,
    Subtract,
    Multiply,
    Divide,
    Not,
}

#[derive(Debug, Clone, PartialEq)]
enum Token {
    LeftParenthesis,
    RightParenthesis,
    Comma,
    Operator(Operator),
    Property(String),
    String(String),
    Number(f64),
    Identifier(String),
}

pub(crate) fn matches(
    expression: &str,
    context: &ResolutionContext,
    properties: &BTreeMap<String, String>,
) -> bool {
    Parser::new(expression, context, properties)
        .and_then(Parser::evaluate)
        .is_some_and(|value| value.as_bool())
}

struct Parser<'a> {
    tokens: Vec<Token>,
    current: usize,
    context: &'a ResolutionContext,
    properties: &'a BTreeMap<String, String>,
}

impl<'a> Parser<'a> {
    fn new(
        expression: &str,
        context: &'a ResolutionContext,
        properties: &'a BTreeMap<String, String>,
    ) -> Option<Self> {
        Some(Self {
            tokens: tokenize(expression)?,
            current: 0,
            context,
            properties,
        })
    }

    fn evaluate(mut self) -> Option<Value> {
        let value = self.parse_or()?;
        (self.current == self.tokens.len()).then_some(value)
    }

    fn parse_or(&mut self) -> Option<Value> {
        let mut left = self.parse_and()?;
        while self.consume_operator(Operator::Or) {
            let right = self.parse_and()?;
            left = Value::Bool(logical_bool(&left)? || logical_bool(&right)?);
        }
        Some(left)
    }

    fn parse_and(&mut self) -> Option<Value> {
        let mut left = self.parse_comparison()?;
        while self.consume_operator(Operator::And) {
            let right = self.parse_comparison()?;
            left = Value::Bool(logical_bool(&left)? && logical_bool(&right)?);
        }
        Some(left)
    }

    fn parse_comparison(&mut self) -> Option<Value> {
        let mut left = self.parse_additive()?;
        while let Some(operator) = self.consume_comparison() {
            let right = self.parse_additive()?;
            left = Value::Bool(compare(&left, operator, &right)?);
        }
        Some(left)
    }

    fn parse_additive(&mut self) -> Option<Value> {
        let mut left = self.parse_multiplicative()?;
        loop {
            if self.consume_operator(Operator::Add) {
                let right = self.parse_multiplicative()?;
                left = if matches!(left, Value::String(_)) || matches!(right, Value::String(_)) {
                    Value::String(left.as_string() + &right.as_string())
                } else {
                    Value::Number(left.as_number()? + right.as_number()?)
                };
            } else if self.consume_operator(Operator::Subtract) {
                let right = self.parse_multiplicative()?;
                left = Value::Number(left.as_number()? - right.as_number()?);
            } else {
                return Some(left);
            }
        }
    }

    fn parse_multiplicative(&mut self) -> Option<Value> {
        let mut left = self.parse_unary()?;
        loop {
            if self.consume_operator(Operator::Multiply) {
                let right = self.parse_unary()?;
                left = Value::Number(left.as_number()? * right.as_number()?);
            } else if self.consume_operator(Operator::Divide) {
                let right = self.parse_unary()?.as_number()?;
                if right == 0.0 {
                    return None;
                }
                left = Value::Number(left.as_number()? / right);
            } else {
                return Some(left);
            }
        }
    }

    fn parse_unary(&mut self) -> Option<Value> {
        if self.consume_operator(Operator::Not) {
            return Some(Value::Bool(!self.parse_unary()?.as_bool()));
        }
        if self.consume_operator(Operator::Subtract) {
            return Some(Value::Number(-self.parse_unary()?.as_number()?));
        }
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Option<Value> {
        let token = self.tokens.get(self.current)?.clone();
        self.current += 1;
        match token {
            Token::LeftParenthesis => {
                let value = self.parse_or()?;
                self.consume(&Token::RightParenthesis).then_some(value)
            }
            Token::Property(name) => Some(self.property(&name)),
            Token::String(value) => Some(Value::String(value)),
            Token::Number(value) => Some(Value::Number(value)),
            Token::Identifier(name) if name.eq_ignore_ascii_case("true") => Some(Value::Bool(true)),
            Token::Identifier(name) if name.eq_ignore_ascii_case("false") => {
                Some(Value::Bool(false))
            }
            Token::Identifier(name) if self.consume(&Token::LeftParenthesis) => {
                let arguments = self.parse_arguments()?;
                self.call(&name, &arguments)
            }
            Token::Identifier(name) => Some(Value::String(name)),
            Token::RightParenthesis | Token::Comma | Token::Operator(_) => None,
        }
    }

    fn parse_arguments(&mut self) -> Option<Vec<Value>> {
        let mut arguments = Vec::new();
        if self.consume(&Token::RightParenthesis) {
            return Some(arguments);
        }
        loop {
            arguments.push(self.parse_or()?);
            if self.consume(&Token::RightParenthesis) {
                return Some(arguments);
            }
            if !self.consume(&Token::Comma) {
                return None;
            }
        }
    }

    fn property(&self, name: &str) -> Value {
        let value = match name {
            "project.basedir" | "project.rootDirectory" => self
                .context
                .base_directory
                .as_ref()
                .map(|path| path.to_string_lossy().into_owned()),
            "java.version" => self.context.java_version.clone(),
            "os.name" => self.context.os_name.clone(),
            "os.arch" => self.context.os_arch.clone(),
            "os.version" => self.context.os_version.clone(),
            _ => name
                .strip_prefix("env.")
                .and_then(|name| self.context.environment.get(name))
                .or_else(|| self.context.properties.get(name))
                .or_else(|| self.properties.get(name))
                .cloned(),
        };
        value.map_or(Value::Null, Value::String)
    }

    fn call(&self, name: &str, arguments: &[Value]) -> Option<Value> {
        match (name, arguments) {
            ("length", [value]) => Some(Value::Number(count_as_number(
                value.as_string().encode_utf16().count(),
            ))),
            ("upper", [value]) => Some(Value::String(value.as_string().to_uppercase())),
            ("lower", [value]) => Some(Value::String(value.as_string().to_lowercase())),
            ("substring", [value, start]) => {
                substring(&value.as_string(), start.as_index()?, None).map(Value::String)
            }
            ("substring", [value, start, end]) => {
                substring(&value.as_string(), start.as_index()?, Some(end.as_index()?))
                    .map(Value::String)
            }
            ("indexOf", [value, needle]) => {
                let value = value.as_string();
                let needle = needle.as_string();
                let index = value.find(&needle).map_or(-1.0, |index| {
                    count_as_number(
                        value
                            .get(..index)
                            .unwrap_or_default()
                            .encode_utf16()
                            .count(),
                    )
                });
                Some(Value::Number(index))
            }
            ("contains", [value, needle]) => {
                Some(Value::Bool(value.as_string().contains(&needle.as_string())))
            }
            ("matches", [value, pattern]) => {
                let pattern = pattern.as_string();
                let mut builder = RegexBuilder::new(&pattern);
                builder.backtrack_limit(1_000_000);
                let regex = builder.build().ok()?;
                let value = value.as_string();
                Some(Value::Bool(regex.find(&value).ok()?.is_some_and(|found| {
                    found.start() == 0 && found.end() == value.len()
                })))
            }
            ("not", [value]) => Some(Value::Bool(!value.as_bool())),
            ("if", [condition, when_true, when_false]) => Some(if condition.as_bool() {
                when_true.clone()
            } else {
                when_false.clone()
            }),
            ("exists", [path]) => Some(Value::Bool(self.path_exists(&path.as_string()))),
            ("missing", [path]) => Some(Value::Bool(!self.path_exists(&path.as_string()))),
            ("inrange", [version, range]) => {
                let range: VersionSpec = range.as_string().parse().ok()?;
                Some(Value::Bool(
                    range.matches(&Version::new(version.as_string())),
                ))
            }
            _ => None,
        }
    }

    fn path_exists(&self, path: &str) -> bool {
        #[cfg(target_family = "wasm")]
        {
            let _ = self;
            let _ = path;
            false
        }
        #[cfg(not(target_family = "wasm"))]
        {
            let path = interpolate_path(path, self.context, self.properties);
            let path = std::path::Path::new(&path);
            let pattern = if path.is_absolute() {
                path.to_path_buf()
            } else {
                let Some(base) = &self.context.base_directory else {
                    return false;
                };
                base.join(path)
            };
            glob::glob(&pattern.to_string_lossy())
                .is_ok_and(|mut entries| entries.any(|entry| entry.is_ok()))
        }
    }

    fn consume(&mut self, expected: &Token) -> bool {
        if self.tokens.get(self.current) == Some(expected) {
            self.current += 1;
            true
        } else {
            false
        }
    }

    fn consume_operator(&mut self, expected: Operator) -> bool {
        self.consume(&Token::Operator(expected))
    }

    fn consume_comparison(&mut self) -> Option<Operator> {
        let Token::Operator(
            operator @ (Operator::Equal
            | Operator::NotEqual
            | Operator::Less
            | Operator::LessEqual
            | Operator::Greater
            | Operator::GreaterEqual),
        ) = self.tokens.get(self.current)?
        else {
            return None;
        };
        self.current += 1;
        Some(*operator)
    }
}

#[expect(
    clippy::cast_possible_truncation,
    reason = "Maven condition semantics use Java Number.intValue"
)]
fn number_as_i32(value: f64) -> i32 {
    value as i32
}

#[expect(
    clippy::cast_precision_loss,
    reason = "Maven expression numbers are represented as double precision values"
)]
fn count_as_number(value: usize) -> f64 {
    value as f64
}

fn compare(left: &Value, operator: Operator, right: &Value) -> Option<bool> {
    if let (Value::Bool(left), Value::Bool(right)) = (left, right) {
        return match operator {
            Operator::Equal => Some(left == right),
            Operator::NotEqual => Some(left != right),
            _ => None,
        };
    }
    if matches!((left, right), (Value::Null, Value::Null)) {
        return match operator {
            Operator::Equal => Some(true),
            Operator::NotEqual => Some(false),
            _ => None,
        };
    }
    if let (Value::Number(left), Value::Number(right)) = (left, right) {
        return Some(match operator {
            Operator::Equal => (left - right).abs() < 1e-9,
            Operator::NotEqual => (left - right).abs() >= 1e-9,
            Operator::Less => left < right,
            Operator::LessEqual => left <= right,
            Operator::Greater => left > right,
            Operator::GreaterEqual => left >= right,
            _ => return None,
        });
    }
    let ordering = match (left, right) {
        (Value::Null, _) | (_, Value::Null) => {
            return match operator {
                Operator::Equal => Some(false),
                Operator::NotEqual => Some(true),
                _ => None,
            };
        }
        (Value::String(left), Value::String(right)) => left.cmp(right),
        _ => return None,
    };
    Some(match operator {
        Operator::Equal => ordering == Ordering::Equal,
        Operator::NotEqual => ordering != Ordering::Equal,
        Operator::Less => ordering == Ordering::Less,
        Operator::LessEqual => ordering != Ordering::Greater,
        Operator::Greater => ordering == Ordering::Greater,
        Operator::GreaterEqual => ordering != Ordering::Less,
        _ => return None,
    })
}

fn logical_bool(value: &Value) -> Option<bool> {
    if let Value::Bool(value) = value {
        Some(*value)
    } else {
        None
    }
}

fn substring(value: &str, start: usize, end: Option<usize>) -> Option<String> {
    let code_units = value.encode_utf16().collect::<Vec<_>>();
    let end = end.unwrap_or(code_units.len());
    (start <= end && end <= code_units.len())
        .then(|| String::from_utf16(code_units.get(start..end)?).ok())
        .flatten()
}

fn interpolate_path(
    value: &str,
    context: &ResolutionContext,
    properties: &BTreeMap<String, String>,
) -> String {
    let mut result = value.to_string();
    for _ in 0..64 {
        let Some(start) = result.find("${") else {
            break;
        };
        let Some(relative_end) = result.get(start + 2..).and_then(|tail| tail.find('}')) else {
            break;
        };
        let end = start + 2 + relative_end;
        // `end` is derived from a closing brace found in this UTF-8 string.
        let key = &result[start + 2..end];
        let base_directory;
        let replacement = match key {
            "basedir" | "project.basedir" | "project.rootDirectory" => {
                base_directory = context
                    .base_directory
                    .as_ref()
                    .map(|path| path.to_string_lossy().into_owned());
                base_directory.as_ref()
            }
            _ => key
                .strip_prefix("env.")
                .and_then(|name| context.environment.get(name))
                .or_else(|| context.properties.get(key))
                .or_else(|| properties.get(key)),
        };
        let Some(replacement) = replacement else {
            break;
        };
        result.replace_range(start..=end, replacement);
    }
    result
}

fn tokenize(expression: &str) -> Option<Vec<Token>> {
    let mut tokens = Vec::new();
    let mut characters = expression.char_indices().peekable();
    while let Some((index, character)) = characters.next() {
        if character.is_whitespace() {
            continue;
        }
        if character == '$' && characters.peek().is_some_and(|(_, next)| *next == '{') {
            let _ = characters.next();
            let start = index + 2;
            let mut end = None;
            for (candidate, value) in characters.by_ref() {
                if value == '}' {
                    end = Some(candidate);
                    break;
                }
            }
            tokens.push(Token::Property(expression.get(start..end?)?.to_string()));
            continue;
        }
        if matches!(character, '\'' | '"') {
            let quote = character;
            let mut value = String::new();
            let mut closed = false;
            for (_, candidate) in characters.by_ref() {
                if candidate == quote {
                    closed = true;
                    break;
                }
                value.push(candidate);
            }
            if !closed {
                return None;
            }
            tokens.push(Token::String(value));
            continue;
        }
        let single = match character {
            '(' => Some(Token::LeftParenthesis),
            ')' => Some(Token::RightParenthesis),
            ',' => Some(Token::Comma),
            '+' => Some(Token::Operator(Operator::Add)),
            '-' => Some(Token::Operator(Operator::Subtract)),
            '*' => Some(Token::Operator(Operator::Multiply)),
            '/' => Some(Token::Operator(Operator::Divide)),
            _ => None,
        };
        if let Some(token) = single {
            tokens.push(token);
            continue;
        }
        if matches!(character, '&' | '|' | '=' | '!' | '<' | '>') {
            let next = characters.peek().map(|(_, value)| *value);
            let operator = match (character, next) {
                ('&', Some('&')) => Operator::And,
                ('|', Some('|')) => Operator::Or,
                ('=', Some('=')) => Operator::Equal,
                ('!', Some('=')) => Operator::NotEqual,
                ('<', Some('=')) => Operator::LessEqual,
                ('>', Some('=')) => Operator::GreaterEqual,
                ('!', _) => Operator::Not,
                ('<', _) => Operator::Less,
                ('>', _) => Operator::Greater,
                _ => return None,
            };
            if matches!(
                (character, next),
                ('&', Some('&')) | ('|', Some('|')) | ('=' | '!' | '<' | '>', Some('='))
            ) {
                let _ = characters.next();
            }
            tokens.push(Token::Operator(operator));
            continue;
        }
        let start = index;
        let mut end = expression.len();
        while let Some((candidate, value)) = characters.peek().copied() {
            if value.is_whitespace()
                || matches!(
                    value,
                    '(' | ')' | ',' | '+' | '-' | '*' | '/' | '&' | '|' | '=' | '!' | '<' | '>'
                )
            {
                end = candidate;
                break;
            }
            let _ = characters.next();
        }
        let value = expression.get(start..end)?;
        if let Ok(number) = value.parse() {
            tokens.push(Token::Number(number));
        } else {
            tokens.push(Token::Identifier(value.to_string()));
        }
    }
    (!tokens.is_empty()).then_some(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluates_maven_condition_functions_and_comparisons() {
        let mut context = ResolutionContext::default()
            .with_property("feature", "resolver-enabled")
            .with_property("user.name", "Ristretto");
        context.java_version = Some("21.0.2".to_string());
        let properties = BTreeMap::new();
        assert!(matches(
            "contains(${feature}, 'enabled') && length(${user.name}) > 5",
            &context,
            &properties
        ));
        assert!(matches(
            "matches(${feature}, 'resolver-.*') && inrange(${java.version}, '[21,22)')",
            &context,
            &properties
        ));
        assert!(matches("matches('12345', '\\d+')", &context, &properties));
        assert!(matches("matches('abab', '(ab)\\1')", &context, &properties));
        assert!(matches(
            "matches('foobar', 'foo(?=bar)bar')",
            &context,
            &properties
        ));
        assert!(!matches(
            "matches(${feature}, 'disabled-.*') || length(${user.name}) < 5",
            &context,
            &properties
        ));
        assert!(matches("0.1 + 0.2 == 0.3", &context, &properties));
        assert!(matches(
            "'release-' + 2 == 'release-2'",
            &context,
            &properties
        ));
        assert!(!matches("0.5", &context, &properties));
        assert!(matches("true == true", &context, &properties));
        assert!(matches("true != false", &context, &properties));
        assert!(!matches("1 && true", &context, &properties));
        assert!(matches("!false", &context, &properties));
        assert!(matches(
            "${missing} == ${alsoMissing}",
            &context,
            &properties
        ));
        assert!(!matches(
            "${missing} != ${alsoMissing}",
            &context,
            &properties
        ));
    }

    #[test]
    fn evaluates_exists_missing_and_rejects_invalid_expressions() {
        #[cfg(not(target_family = "wasm"))]
        let directory = tempfile::TempDir::new().expect("temporary directory");
        #[cfg(not(target_family = "wasm"))]
        std::fs::write(directory.path().join("present.txt"), b"fixture").expect("write fixture");
        #[cfg(not(target_family = "wasm"))]
        let context = ResolutionContext {
            base_directory: Some(directory.path().to_path_buf()),
            ..ResolutionContext::default()
        };
        #[cfg(target_family = "wasm")]
        let context = ResolutionContext::default();
        let properties = BTreeMap::new();
        #[cfg(not(target_family = "wasm"))]
        assert!(matches(
            "exists('present.txt') && missing('absent.txt')",
            &context,
            &properties
        ));
        #[cfg(target_family = "wasm")]
        {
            assert!(!matches("exists('present.txt')", &context, &properties));
            assert!(matches("missing('absent.txt')", &context, &properties));
        }
        assert!(!matches("unsupported(${unknown})", &context, &properties));
        assert!(!matches("length(", &context, &properties));
        #[cfg(not(target_family = "wasm"))]
        assert!(matches("exists('*.txt')", &context, &properties));
    }

    #[test]
    #[expect(
        clippy::too_many_lines,
        reason = "table-like coverage test enumerates the Maven expression grammar"
    )]
    fn covers_maven_condition_value_function_and_parser_edge_paths() {
        assert_eq!(
            compare(&Value::Bool(true), Operator::Greater, &Value::Bool(false)),
            None
        );
        assert_eq!(compare(&Value::Null, Operator::Less, &Value::Null), None);
        assert_eq!(
            compare(
                &Value::String("1".to_string()),
                Operator::Equal,
                &Value::Number(1.0)
            ),
            None
        );
        assert!(!Value::Null.as_bool());
        assert!(Value::String("value".to_string()).as_bool());
        assert!(!Value::String(" ".to_string()).as_bool());
        assert_eq!(Value::Bool(true).as_number(), Some(1.0));
        assert_eq!(Value::Bool(false).as_number(), Some(0.0));
        assert_eq!(Value::String("2.5".to_string()).as_number(), Some(2.5));
        assert_eq!(Value::String("no".to_string()).as_number(), None);
        assert_eq!(Value::Null.as_number(), None);
        assert_eq!(Value::Number(-1.0).as_index(), None);
        assert_eq!(Value::String("2".to_string()).as_index(), Some(2));
        assert_eq!(Value::Null.as_string(), "null");
        assert_eq!(Value::Bool(true).as_string(), "true");
        assert_eq!(Value::Number(1.25).as_string(), "1.25");

        let mut context = ResolutionContext::default()
            .with_property("user", "context")
            .with_property("numeric", "2");
        context
            .environment
            .insert("CI".to_string(), "true".to_string());
        context.java_version = Some("21".to_string());
        context.os_name = Some("Linux".to_string());
        context.os_arch = Some("x86_64".to_string());
        context.os_version = Some("6.1".to_string());
        let properties = BTreeMap::from([
            ("project".to_string(), "model".to_string()),
            ("path".to_string(), "present.txt".to_string()),
        ]);

        for expression in [
            "upper('release') == 'RELEASE'",
            "lower('RELEASE') == 'release'",
            "substring('release', 3) == 'ease'",
            "substring('release', 1, 3) == 'el'",
            "indexOf('a😀b', 'b') == 3",
            "indexOf('abc', 'z') == -1",
            "not(false)",
            "if(true, 'yes', 'no') == 'yes'",
            "if(false, 'yes', 'no') == 'no'",
            "(2 + 3) * 4 / 2 - 1 == 9",
            "-2 < -1",
            "'a' < 'b'",
            "'a' <= 'a'",
            "'b' > 'a'",
            "'b' >= 'b'",
            "'a' != 'b'",
            "${missing} == ${also.missing}",
            "${missing} != 'value'",
            "${java.version} == '21'",
            "${os.name} == 'Linux'",
            "${os.arch} == 'x86_64'",
            "${os.version} == '6.1'",
            "${env.CI} == 'true'",
            "${user} == 'context'",
            "${project} == 'model'",
            "true || false",
            "true && true",
            "2 >= 2",
            "2 <= 2",
            "2 != 3",
        ] {
            assert!(
                matches(expression, &context, &properties),
                "expected true: {expression}"
            );
        }

        for expression in [
            "",
            "1 / 0",
            "substring('abc', -1)",
            "substring('abc', 2, 1)",
            "substring('😀', 1, 2)",
            "matches('value', '[')",
            "inrange('1', 'invalid')",
            "length()",
            "length('a', 'b')",
            "unknown()",
            "true false",
            "(true",
            ")",
            ",",
            "true == false",
            "${missing} < 'value'",
            "true + ${missing}",
            "true * 'no'",
            "true / 'no'",
            "&",
            "|",
            "=",
            "${unclosed",
            "'unclosed",
        ] {
            assert!(
                !matches(expression, &context, &properties),
                "expected false: {expression}"
            );
        }

        assert_eq!(
            compare(&Value::Null, Operator::Equal, &Value::Null),
            Some(true)
        );
        assert_eq!(
            compare(&Value::Null, Operator::NotEqual, &Value::String("x".into())),
            Some(true)
        );
        assert_eq!(
            compare(&Value::String("x".into()), Operator::Equal, &Value::Null),
            Some(false)
        );
        assert_eq!(
            compare(&Value::Null, Operator::Less, &Value::String("x".into())),
            None
        );
        assert_eq!(
            compare(&Value::Bool(true), Operator::Equal, &Value::Bool(true)),
            Some(true)
        );
        assert_eq!(
            compare(&Value::Null, Operator::NotEqual, &Value::Null),
            Some(false)
        );
        assert_eq!(
            compare(&Value::Number(1.0), Operator::Add, &Value::Number(2.0)),
            None
        );
        assert_eq!(
            compare(
                &Value::String("a".into()),
                Operator::Add,
                &Value::String("b".into())
            ),
            None
        );
        assert_eq!(logical_bool(&Value::Bool(false)), Some(false));
        assert_eq!(logical_bool(&Value::Number(1.0)), None);
        assert_eq!(substring("abc", 4, None), None);
    }

    #[test]
    fn interpolates_condition_paths_with_all_property_sources() {
        let base_directory = std::path::PathBuf::from("/workspace/project");
        let mut context = ResolutionContext {
            base_directory: Some(base_directory.clone()),
            ..ResolutionContext::default()
        };
        context
            .environment
            .insert("ROOT".to_string(), "environment".to_string());
        context
            .properties
            .insert("root".to_string(), "context".to_string());
        let properties = BTreeMap::from([("model".to_string(), "project".to_string())]);
        assert_eq!(
            interpolate_path(
                "${basedir}/${env.ROOT}/${root}/${model}",
                &context,
                &properties
            ),
            format!(
                "{}/environment/context/project",
                base_directory.to_string_lossy()
            )
        );
        assert_eq!(
            interpolate_path("${unknown}/file", &context, &properties),
            "${unknown}/file"
        );
        assert_eq!(
            interpolate_path("${unclosed", &context, &properties),
            "${unclosed"
        );
        assert!(!matches(
            "exists('relative.txt')",
            &ResolutionContext::default(),
            &properties
        ));
        assert!(!matches("exists('[')", &context, &properties));
        #[cfg(not(target_family = "wasm"))]
        assert!(matches(
            &format!(
                "exists('{}')",
                std::env::current_dir()
                    .expect("current directory")
                    .to_string_lossy()
            ),
            &context,
            &properties
        ));
        assert!(matches(
            "${project.basedir} == ${project.rootDirectory}",
            &context,
            &properties
        ));
        assert!(!matches("contains('a' 'a')", &context, &properties));
    }
}
