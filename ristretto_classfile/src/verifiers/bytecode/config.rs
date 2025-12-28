//! # Verifier Configuration
//!
//! This module provides configuration options for the bytecode verifier.
//!
//! # Configuration Options
//!
//! - [`VerifyMode`]: Controls which classes are verified
//! - [`VerifierConfig`]: Main configuration struct with all options
//!
//! # Example
//!
//! ```rust,ignore
//! use ristretto_classfile::verifiers::bytecode::config::{VerifierConfig, VerifyMode};
//!
//! let config = VerifierConfig::default()
//!     .with_verify_mode(VerifyMode::All)
//!     .with_verbose(true);
//! ```
//!
//! # References
//!
//! - [JVM Tool Interface: Verification](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.10)

use std::fmt;

use bitflags::bitflags;

bitflags! {
    /// Boolean options for the verifier configuration.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub struct VerifierFlags: u8 {
        /// Enable verbose verification output.
        const VERBOSE = 0b0000_0001;
        /// Enable tracing of all verification steps.
        const TRACE = 0b0000_0010;
        /// Cache verification results.
        const CACHE_RESULTS = 0b0000_0100;
        /// Allow JSR/RET instructions.
        const ALLOW_JSR_RET = 0b0000_1000;
        /// Strict exception handler type checking.
        const STRICT_EXCEPTION_HANDLERS = 0b0001_0000;
    }
}

/// Verification mode controlling which classes are verified.
///
/// This corresponds to the `-Xverify` JVM options:
/// - `-Xverify:all` - Verify all classes
/// - `-Xverify:remote` - Verify only classes loaded from network (default)
/// - `-Xverify:none` - Skip verification entirely
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VerifyMode {
    /// Verify all classes.
    ///
    /// This is the most strict mode and should be used for development
    /// and when loading untrusted code.
    All,

    /// Verify only remote/untrusted classes (default).
    ///
    /// Classes from the bootstrap classpath are not verified.
    /// This is the default JVM behavior.
    #[default]
    Remote,

    /// Skip verification entirely.
    ///
    /// **WARNING**: This is dangerous and should only be used for
    /// performance testing or with fully trusted code.
    None,
}

impl fmt::Display for VerifyMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::All => write!(f, "all"),
            Self::Remote => write!(f, "remote"),
            Self::None => write!(f, "none"),
        }
    }
}

/// Strategy for handling classes without `StackMapTable`.
///
/// For class files version 50+ (Java 6+), `StackMapTable` is required.
/// This enum controls behavior when it's missing or invalid.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FallbackStrategy {
    /// Fail verification if `StackMapTable` is missing/invalid (strict mode).
    ///
    /// This is the recommended mode for production.
    #[default]
    Strict,

    /// Fall back to type inference verifier if `StackMapTable` is missing/invalid.
    ///
    /// This provides compatibility with bytecode instrumentation tools
    /// that may not properly update `StackMapTable`.
    FallbackToInference,

    /// Always use type inference, even if `StackMapTable` is present.
    ///
    /// This is useful for debugging or comparing verification results.
    AlwaysInference,
}

impl fmt::Display for FallbackStrategy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Strict => write!(f, "strict"),
            Self::FallbackToInference => write!(f, "fallback"),
            Self::AlwaysInference => write!(f, "inference"),
        }
    }
}

/// Main verifier configuration.
///
/// This struct controls all aspects of bytecode verification behavior.
///
/// # Example
///
/// ```rust,ignore
/// use ristretto_classfile::verifiers::bytecode::config::VerifierConfig;
///
/// // Default configuration (recommended)
/// let config = VerifierConfig::default();
///
/// // Strict configuration for untrusted code
/// let strict = VerifierConfig::strict();
///
/// // Permissive configuration for compatibility
/// let permissive = VerifierConfig::permissive();
/// ```
#[derive(Debug, Clone)]
pub struct VerifierConfig {
    /// Which classes to verify.
    pub verify_mode: VerifyMode,

    /// How to handle missing/invalid `StackMapTable`.
    pub fallback_strategy: FallbackStrategy,

    /// Boolean flags for verification options.
    pub flags: VerifierFlags,

    /// Maximum iterations for type inference (slow path).
    ///
    /// Prevents infinite loops in malformed bytecode.
    /// Default is 1000.
    pub max_inference_iterations: usize,

    /// Minimum class file version for `StackMapTable` requirement.
    ///
    /// Classes with major version >= this value must have `StackMapTable`.
    /// Default is 50 (Java 6).
    pub stackmap_required_version: u16,
}

impl Default for VerifierConfig {
    fn default() -> Self {
        Self {
            verify_mode: VerifyMode::default(),
            fallback_strategy: FallbackStrategy::default(),
            flags: VerifierFlags::STRICT_EXCEPTION_HANDLERS,
            max_inference_iterations: 1000,
            stackmap_required_version: 50, // Java 6
        }
    }
}

impl VerifierConfig {
    /// Creates a new configuration with default settings.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a strict configuration suitable for untrusted code.
    ///
    /// - Verifies all classes
    /// - No fallback to inference
    /// - Strict exception handler checking
    #[must_use]
    pub fn strict() -> Self {
        Self {
            verify_mode: VerifyMode::All,
            fallback_strategy: FallbackStrategy::Strict,
            flags: VerifierFlags::STRICT_EXCEPTION_HANDLERS,
            ..Default::default()
        }
    }

    /// Creates a permissive configuration for maximum compatibility.
    ///
    /// - Falls back to inference if `StackMapTable` is invalid
    /// - Relaxed exception handler checking
    /// - Allows JSR/RET for old bytecode
    #[must_use]
    pub fn permissive() -> Self {
        Self {
            fallback_strategy: FallbackStrategy::FallbackToInference,
            flags: VerifierFlags::ALLOW_JSR_RET,
            ..Default::default()
        }
    }

    /// Sets the verification mode.
    #[must_use]
    pub const fn with_verify_mode(mut self, mode: VerifyMode) -> Self {
        self.verify_mode = mode;
        self
    }

    /// Sets the fallback strategy.
    #[must_use]
    pub const fn with_fallback_strategy(mut self, strategy: FallbackStrategy) -> Self {
        self.fallback_strategy = strategy;
        self
    }

    /// Enables or disables verbose output.
    #[must_use]
    pub fn with_verbose(mut self, verbose: bool) -> Self {
        self.flags.set(VerifierFlags::VERBOSE, verbose);
        self
    }

    /// Enables or disables detailed tracing.
    #[must_use]
    pub fn with_trace(mut self, trace: bool) -> Self {
        self.flags.set(VerifierFlags::TRACE, trace);
        self
    }

    /// Sets the maximum inference iterations.
    #[must_use]
    pub const fn with_max_inference_iterations(mut self, max: usize) -> Self {
        self.max_inference_iterations = max;
        self
    }

    /// Enables or disables result caching.
    #[must_use]
    pub fn with_cache_results(mut self, cache: bool) -> Self {
        self.flags.set(VerifierFlags::CACHE_RESULTS, cache);
        self
    }

    /// Returns whether verbose output is enabled.
    #[must_use]
    pub const fn verbose(&self) -> bool {
        self.flags.contains(VerifierFlags::VERBOSE)
    }

    /// Returns whether tracing is enabled.
    #[must_use]
    pub const fn trace(&self) -> bool {
        self.flags.contains(VerifierFlags::TRACE)
    }

    /// Checks if verification should be performed for the given context.
    ///
    /// # Arguments
    ///
    /// * `is_trusted` - Whether the class is from a trusted source
    #[must_use]
    pub const fn should_verify(&self, is_trusted: bool) -> bool {
        match self.verify_mode {
            VerifyMode::All => true,
            VerifyMode::Remote => !is_trusted,
            VerifyMode::None => false,
        }
    }

    /// Checks if `StackMapTable` is required for the given class file version.
    #[must_use]
    pub const fn requires_stackmap(&self, major_version: u16) -> bool {
        major_version >= self.stackmap_required_version
    }

    /// Checks if type inference should be used.
    #[must_use]
    pub const fn use_inference(&self) -> bool {
        matches!(self.fallback_strategy, FallbackStrategy::AlwaysInference)
    }

    /// Checks if fallback to inference is allowed.
    #[must_use]
    pub const fn allows_inference_fallback(&self) -> bool {
        matches!(
            self.fallback_strategy,
            FallbackStrategy::FallbackToInference | FallbackStrategy::AlwaysInference
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = VerifierConfig::default();
        assert_eq!(config.verify_mode, VerifyMode::Remote);
        assert_eq!(config.fallback_strategy, FallbackStrategy::Strict);
        assert!(!config.verbose());
        assert!(!config.trace());
    }

    #[test]
    fn test_strict_config() {
        let config = VerifierConfig::strict();
        assert_eq!(config.verify_mode, VerifyMode::All);
        assert_eq!(config.fallback_strategy, FallbackStrategy::Strict);
        assert!(
            config
                .flags
                .contains(VerifierFlags::STRICT_EXCEPTION_HANDLERS)
        );
    }

    #[test]
    fn test_permissive_config() {
        let config = VerifierConfig::permissive();
        assert_eq!(
            config.fallback_strategy,
            FallbackStrategy::FallbackToInference
        );
        assert!(
            !config
                .flags
                .contains(VerifierFlags::STRICT_EXCEPTION_HANDLERS)
        );
        assert!(config.flags.contains(VerifierFlags::ALLOW_JSR_RET));
    }

    #[test]
    fn test_should_verify() {
        let all = VerifierConfig::new().with_verify_mode(VerifyMode::All);
        assert!(all.should_verify(true));
        assert!(all.should_verify(false));

        let remote = VerifierConfig::new().with_verify_mode(VerifyMode::Remote);
        assert!(!remote.should_verify(true));
        assert!(remote.should_verify(false));

        let none = VerifierConfig::new().with_verify_mode(VerifyMode::None);
        assert!(!none.should_verify(true));
        assert!(!none.should_verify(false));
    }

    #[test]
    fn test_requires_stackmap() {
        let config = VerifierConfig::default();
        assert!(!config.requires_stackmap(49)); // Java 5
        assert!(config.requires_stackmap(50)); // Java 6
        assert!(config.requires_stackmap(52)); // Java 8
        assert!(config.requires_stackmap(65)); // Java 21
    }
}
