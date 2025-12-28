//! # Unified Bytecode Verifier
//!
//! This module provides the main entry point for bytecode verification, implementing a fast optimization strategy:
//!
//! 1. **Fast Path**: StackMapTable-driven verification (single pass)
//! 2. **Slow Path**: Type-inference verification (iterative dataflow)
//!
//! # Usage
//!
//! ```rust,ignore
//! use ristretto_classfile::verifiers::bytecode::unified::{UnifiedVerifier, verify_method};
//! use ristretto_classfile::verifiers::bytecode::config::VerifierConfig;
//!
//! let config = VerifierConfig::default();
//! let result = verify_method(&class_file, &method, &context, &config);
//! ```
//!
//! # References
//!
//! - [JVMS §4.10 - Verification of class Files](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.10)

use crate::attributes::Attribute;
use crate::class_file::ClassFile;
use crate::method::Method;
use crate::method_access_flags::MethodAccessFlags;
use crate::verifiers::bytecode::cache::{CachedResult, MethodKey, VerificationCache};
use crate::verifiers::bytecode::config::VerifierConfig;
use crate::verifiers::bytecode::diagnostics::VerificationDiagnostic;
use crate::verifiers::bytecode::fast_path::{FastPathResult, FastPathVerifier};
use crate::verifiers::bytecode::inference::InferenceVerifier;
use crate::verifiers::context::VerificationContext;
use crate::verifiers::error::{Result, VerifyError};

/// Verification result with additional metadata.
#[derive(Debug)]
pub struct VerificationResult {
    /// Whether verification succeeded.
    pub success: bool,
    /// The verification path used.
    pub path_used: VerificationPath,
    /// Error message if failed.
    pub error: Option<String>,
    /// Diagnostic information if available.
    pub diagnostic: Option<VerificationDiagnostic>,
}

/// Which verification path was used.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerificationPath {
    /// StackMapTable-driven fast path.
    FastPath,
    /// Type-inference slow path.
    Inference,
    /// Skipped (native/abstract methods).
    Skipped,
    /// Cached result.
    Cached,
}

impl VerificationResult {
    /// Creates a successful result.
    #[must_use]
    pub fn success(path: VerificationPath) -> Self {
        Self {
            success: true,
            path_used: path,
            error: None,
            diagnostic: None,
        }
    }

    /// Creates a failed result.
    #[must_use]
    pub fn failed(path: VerificationPath, error: impl Into<String>) -> Self {
        Self {
            success: false,
            path_used: path,
            error: Some(error.into()),
            diagnostic: None,
        }
    }

    /// Adds diagnostic information.
    #[must_use]
    pub fn with_diagnostic(mut self, diagnostic: VerificationDiagnostic) -> Self {
        self.diagnostic = Some(diagnostic);
        self
    }
}

/// Verifies a method's bytecode.
///
/// This is the main entry point for bytecode verification. It automatically
/// selects the appropriate verification strategy based on class file version
/// and configuration.
///
/// # Arguments
///
/// * `class_file` - The class file containing the method
/// * `method` - The method to verify
/// * `context` - The verification context for type hierarchy checks
/// * `config` - Verifier configuration
///
/// # Returns
///
/// A `VerificationResult` with details about the verification.
///
/// # Errors
///
/// Returns an error if verification fails and fallback is not possible.
pub fn verify_method<C: VerificationContext>(
    class_file: &ClassFile,
    method: &Method,
    context: &C,
    config: &VerifierConfig,
) -> Result<VerificationResult> {
    // Native and abstract methods should not have Code attribute
    if method
        .access_flags
        .intersects(MethodAccessFlags::NATIVE | MethodAccessFlags::ABSTRACT)
    {
        // Verify they don't have Code attribute
        for attribute in &method.attributes {
            if matches!(attribute, Attribute::Code { .. }) {
                let name = class_file
                    .constant_pool
                    .try_get_utf8(method.name_index)
                    .map_err(|e| VerifyError::ClassFormatError(e.to_string()))?;
                return Err(VerifyError::ClassFormatError(format!(
                    "Method {name} is native or abstract but has Code attribute"
                )));
            }
        }
        return Ok(VerificationResult::success(VerificationPath::Skipped));
    }

    // Check if method has Code attribute
    let has_code = method
        .attributes
        .iter()
        .any(|a| matches!(a, Attribute::Code { .. }));

    if !has_code {
        return Err(VerifyError::ClassFormatError(
            "Method has no Code attribute".to_string(),
        ));
    }

    // Determine verification strategy
    let major_version = class_file.version.major();
    let use_fast_path = !config.use_inference() && config.requires_stackmap(major_version);

    if use_fast_path {
        // Try fast path first
        let mut fast_verifier = FastPathVerifier::new(class_file, method, context, config)?;

        match fast_verifier.verify() {
            FastPathResult::Success => {
                return Ok(VerificationResult::success(VerificationPath::FastPath));
            }
            FastPathResult::Failed(e) => {
                if config.allows_inference_fallback() {
                    // Fall through to inference
                } else {
                    return Err(e);
                }
            }
            FastPathResult::NeedsFallback(reason) => {
                if !config.allows_inference_fallback() {
                    return Err(VerifyError::VerifyError(format!(
                        "Fast path verification failed and fallback disabled: {reason}"
                    )));
                }
                // Fall through to inference
            }
        }
    }

    // Use type inference (either directly or as fallback)
    let mut inference_verifier = InferenceVerifier::new(class_file, method, context, config)?;

    match inference_verifier.verify() {
        Ok(()) => Ok(VerificationResult::success(VerificationPath::Inference)),
        Err(e) => Err(e),
    }
}

/// Verifies a method with caching support.
///
/// This function checks the cache before performing verification and
/// stores results in the cache for future lookups.
///
/// # Errors
///
/// Returns an error if verification fails.
pub fn verify_method_cached<C: VerificationContext>(
    class_file: &ClassFile,
    method: &Method,
    context: &C,
    config: &VerifierConfig,
    cache: &VerificationCache,
) -> Result<VerificationResult> {
    // Build cache key
    let class_name = class_file
        .constant_pool
        .try_get_class(class_file.this_class)
        .map_err(|e| VerifyError::ClassFormatError(e.to_string()))?;

    let method_name = class_file
        .constant_pool
        .try_get_utf8(method.name_index)
        .map_err(|e| VerifyError::ClassFormatError(e.to_string()))?;

    let descriptor = class_file
        .constant_pool
        .try_get_utf8(method.descriptor_index)
        .map_err(|e| VerifyError::ClassFormatError(e.to_string()))?;

    let key = MethodKey::new(class_name, method_name, descriptor);

    // Check cache
    if let Some(cached) = cache.get_result(&key) {
        return match cached {
            CachedResult::Success => Ok(VerificationResult::success(VerificationPath::Cached)),
            CachedResult::Failed(msg) => Err(VerifyError::VerifyError(msg)),
        };
    }

    // Perform verification
    let result = verify_method(class_file, method, context, config);

    // Cache result
    match &result {
        Ok(_) => cache.put_result(key, CachedResult::Success),
        Err(e) => cache.put_result(key, CachedResult::Failed(e.to_string())),
    }

    result
}

/// Verifies all methods in a class file.
///
/// # Arguments
///
/// * `class_file` - The class file to verify
/// * `context` - The verification context for type hierarchy checks
/// * `config` - Verifier configuration
///
/// # Errors
///
/// Returns the first verification error encountered.
pub fn verify_class<C: VerificationContext>(
    class_file: &ClassFile,
    context: &C,
    config: &VerifierConfig,
) -> Result<Vec<VerificationResult>> {
    let mut results = Vec::with_capacity(class_file.methods.len());

    for method in &class_file.methods {
        let result = verify_method(class_file, method, context, config)?;
        results.push(result);
    }

    Ok(results)
}

/// Verifies all methods in a class file with caching.
///
/// # Errors
///
/// Returns an error if verification of any method fails.
pub fn verify_class_cached<C: VerificationContext>(
    class_file: &ClassFile,
    context: &C,
    config: &VerifierConfig,
    cache: &VerificationCache,
) -> Result<Vec<VerificationResult>> {
    let mut results = Vec::with_capacity(class_file.methods.len());

    for method in &class_file.methods {
        let result = verify_method_cached(class_file, method, context, config, cache)?;
        results.push(result);
    }

    Ok(results)
}

/// Error classification for proper JVM error types.
///
/// This helps map verification failures to the correct JVM exception type.
#[expect(clippy::enum_variant_names)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorClass {
    /// `ClassFormatError` - malformed class file structure
    ClassFormatError,
    /// `VerifyError` - bytecode verification failure
    VerifyError,
    /// `NoClassDefFoundError` - missing class reference
    NoClassDefFoundError,
    /// `IllegalAccessError` - access control violation
    IllegalAccessError,
    /// `IncompatibleClassChangeError` - class structure changed incompatibly
    IncompatibleClassChangeError,
}

impl ErrorClass {
    /// Classifies a verification error.
    #[must_use]
    pub fn classify(error: &VerifyError) -> Self {
        match error {
            VerifyError::ClassFormatError(_) => Self::ClassFormatError,
            VerifyError::NoClassDefFoundError(_) => Self::NoClassDefFoundError,
            VerifyError::IllegalAccessError(_) => Self::IllegalAccessError,
            VerifyError::IncompatibleClassChangeError(_) => Self::IncompatibleClassChangeError,
            VerifyError::VerifyError(_) | VerifyError::VerificationError { .. } => {
                Self::VerifyError
            }
            _ => Self::VerifyError,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Version;
    use crate::attributes::Instruction;
    use crate::constant::Constant;
    use crate::constant_pool::ConstantPool;

    struct MockContext;

    impl VerificationContext for MockContext {
        fn is_subclass(&self, _subclass: &str, _superclass: &str) -> Result<bool> {
            Ok(false)
        }
        fn is_assignable(&self, _target: &str, _source: &str) -> Result<bool> {
            Ok(true)
        }
        fn common_superclass(&self, _class1: &str, _class2: &str) -> Result<String> {
            Ok("java/lang/Object".to_string())
        }
    }

    fn create_mock_class_file() -> ClassFile {
        let mut constant_pool = ConstantPool::default();
        constant_pool
            .add(Constant::Utf8("TestClass".to_string()))
            .unwrap();
        let this_class_index = constant_pool.add(Constant::Class(1)).unwrap();
        constant_pool
            .add(Constant::Utf8("testMethod".to_string()))
            .unwrap();
        constant_pool
            .add(Constant::Utf8("()V".to_string()))
            .unwrap();
        constant_pool
            .add(Constant::Utf8("Code".to_string()))
            .unwrap();

        ClassFile {
            version: Version::Java8 { minor: 0 },
            constant_pool,
            access_flags: crate::ClassAccessFlags::PUBLIC,
            this_class: this_class_index,
            super_class: 0,
            interfaces: vec![],
            fields: vec![],
            methods: vec![],
            attributes: vec![],
        }
    }

    #[test]
    fn test_verify_simple_method() {
        let class_file = create_mock_class_file();
        let code = vec![Instruction::Return];
        let code_attribute = Attribute::Code {
            name_index: 5,
            max_stack: 1,
            max_locals: 1,
            code,
            exception_table: vec![],
            attributes: vec![],
        };

        let method = Method {
            access_flags: MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
            name_index: 3,
            descriptor_index: 4,
            attributes: vec![code_attribute],
        };

        let config = VerifierConfig::default();
        let context = MockContext;

        let result = verify_method(&class_file, &method, &context, &config);
        assert!(result.is_ok());

        let result = result.unwrap();
        assert!(result.success);
    }

    #[test]
    fn test_verify_native_method() {
        let class_file = create_mock_class_file();
        let method = Method {
            access_flags: MethodAccessFlags::PUBLIC | MethodAccessFlags::NATIVE,
            name_index: 3,
            descriptor_index: 4,
            attributes: vec![],
        };

        let config = VerifierConfig::default();
        let context = MockContext;

        let result = verify_method(&class_file, &method, &context, &config);
        assert!(result.is_ok());

        let result = result.unwrap();
        assert!(result.success);
        assert_eq!(result.path_used, VerificationPath::Skipped);
    }

    #[test]
    fn test_verify_with_cache() {
        let class_file = create_mock_class_file();
        let code = vec![Instruction::Return];
        let code_attribute = Attribute::Code {
            name_index: 5,
            max_stack: 1,
            max_locals: 1,
            code,
            exception_table: vec![],
            attributes: vec![],
        };

        let method = Method {
            access_flags: MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
            name_index: 3,
            descriptor_index: 4,
            attributes: vec![code_attribute],
        };

        let config = VerifierConfig::default();
        let context = MockContext;
        let cache = VerificationCache::new(true);

        // First verification
        let result1 = verify_method_cached(&class_file, &method, &context, &config, &cache);
        assert!(result1.is_ok());
        assert_ne!(result1.unwrap().path_used, VerificationPath::Cached);

        // Second verification should use cache
        let result2 = verify_method_cached(&class_file, &method, &context, &config, &cache);
        assert!(result2.is_ok());
        assert_eq!(result2.unwrap().path_used, VerificationPath::Cached);
    }

    #[test]
    fn test_error_classification() {
        let cf_error = VerifyError::ClassFormatError("test".to_string());
        assert_eq!(
            ErrorClass::classify(&cf_error),
            ErrorClass::ClassFormatError
        );

        let v_error = VerifyError::VerifyError("test".to_string());
        assert_eq!(ErrorClass::classify(&v_error), ErrorClass::VerifyError);

        let ncdf_error = VerifyError::NoClassDefFoundError("test".to_string());
        assert_eq!(
            ErrorClass::classify(&ncdf_error),
            ErrorClass::NoClassDefFoundError
        );
    }
}
