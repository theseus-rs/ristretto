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
use crate::verifiers::bytecode::cache::{CacheScope, CachedResult, MethodKey, VerificationCache};
use crate::verifiers::bytecode::config::VerifierConfig;
use crate::verifiers::bytecode::control_flow::CodeInfo;
use crate::verifiers::bytecode::diagnostics::VerificationDiagnostic;
use crate::verifiers::bytecode::fast_path::{FastPathResult, FastPathVerifier};
use crate::verifiers::bytecode::inference::InferenceVerifier;
use crate::verifiers::context::VerificationContext;
use crate::verifiers::error::{Result, VerifyError};
use crate::verifiers::methods::VerifiedMethod;
use crate::{CLASS_INITIALIZER_NAME, JAVA_6};
use std::io::Write;

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
    class_file: &ClassFile<'_>,
    method: &Method,
    context: &C,
    config: &VerifierConfig,
) -> Result<VerificationResult> {
    crate::verifiers::format::verify_method(class_file, method)?;
    verify_method_preverified(class_file, method, context, config, None)
}

/// Called only after structural checks, either for a standalone method or the whole class.
fn verify_method_preverified<C: VerificationContext>(
    class_file: &ClassFile<'_>,
    method: &Method,
    context: &C,
    config: &VerifierConfig,
    code_info: Option<CodeInfo>,
) -> Result<VerificationResult> {
    // Native and abstract methods should not have Code attribute
    if method
        .access_flags
        .intersects(MethodAccessFlags::NATIVE | MethodAccessFlags::ABSTRACT)
        && class_file.constant_pool.try_get_utf8(method.name_index)? != CLASS_INITIALIZER_NAME
    {
        // Verify they don't have Code attribute
        let has_code = method
            .attributes
            .iter()
            .any(|attribute| matches!(attribute, Attribute::Code { .. }));
        if has_code {
            let name = class_file
                .constant_pool
                .try_get_utf8(method.name_index)
                .map_err(|e| VerifyError::ClassFormatError(e.to_string()))?;
            let message = [
                "Method ",
                &name.to_str_lossy(),
                " is native or abstract but has Code attribute",
            ]
            .concat();
            return Err(VerifyError::ClassFormatError(message));
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
    // JVMS 4.10.1 mandates type checking from version 50.0 onward.
    // Configuration preferences cannot bypass this requirement.
    let use_fast_path = major_version >= JAVA_6.major();

    let (fast_path_result, code_info) = if use_fast_path {
        verify_with_fast_path(class_file, method, context, config, code_info)?
    } else {
        (None, code_info)
    };
    if let Some(result) = fast_path_result {
        return Ok(result);
    }

    // Use type inference (either directly or as fallback)
    let mut inference_verifier =
        InferenceVerifier::with_code_info(class_file, method, context, config, code_info)?;

    inference_verifier.verify()?;
    Ok(VerificationResult::success(VerificationPath::Inference))
}

fn verify_with_fast_path<C: VerificationContext>(
    class_file: &ClassFile<'_>,
    method: &Method,
    context: &C,
    config: &VerifierConfig,
    code_info: Option<CodeInfo>,
) -> Result<(Option<VerificationResult>, Option<CodeInfo>)> {
    let mut fast_verifier =
        FastPathVerifier::with_code_info(class_file, method, context, config, code_info)?;
    match fast_verifier.verify() {
        FastPathResult::Success => Ok((
            Some(VerificationResult::success(VerificationPath::FastPath)),
            None,
        )),
        FastPathResult::Failed(_) | FastPathResult::NeedsFallback(_)
            if class_file.version == JAVA_6 =>
        {
            Ok((None, Some(fast_verifier.into_code_info())))
        }
        FastPathResult::Failed(e) => Err(e),
        FastPathResult::NeedsFallback(reason) => Err(VerifyError::VerifyError(reason)),
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
    class_file: &ClassFile<'_>,
    method: &Method,
    context: &C,
    config: &VerifierConfig,
    cache: &VerificationCache,
) -> Result<VerificationResult> {
    let Some(context_token) = context.cache_token().filter(|_| cache.is_enabled()) else {
        return verify_method(class_file, method, context, config);
    };
    let mut scope = vec![0]; // Standalone method may differ from the class's method table.
    class_file.to_bytes(&mut scope)?;
    method.to_bytes(&mut scope)?;
    scope.extend_from_slice(&context_token.to_be_bytes());
    write!(scope, "{config:?}").map_err(crate::Error::from)?;
    let scope = cache.intern_scope(scope);
    verify_method_scoped(class_file, method, context, config, cache, &scope, None)
}

fn verify_method_scoped<C: VerificationContext>(
    class_file: &ClassFile<'_>,
    method: &Method,
    context: &C,
    config: &VerifierConfig,
    cache: &VerificationCache,
    scope: &CacheScope,
    verified: Option<VerifiedMethod>,
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

    let mut key = MethodKey::borrowed(class_name, method_name, descriptor);
    key.scope = std::borrow::Cow::Borrowed(scope.as_ref());

    // Check cache
    if let Some(cached) = cache.get_result_scoped(&key, scope) {
        return match cached {
            CachedResult::Success => Ok(VerificationResult::success(VerificationPath::Cached)),
            CachedResult::Failed(msg) => Err(VerifyError::VerifyError(msg)),
        };
    }

    // Perform verification
    let result = if let Some(verified) = verified {
        verify_method_preverified(class_file, method, context, config, verified.code_info)
    } else {
        verify_method(class_file, method, context, config)
    };

    // Cache result
    match &result {
        Ok(_) => cache.put_result_scoped(&key, scope, CachedResult::Success),
        Err(e) => cache.put_result_scoped(&key, scope, CachedResult::Failed(e.to_string())),
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
    class_file: &ClassFile<'_>,
    context: &C,
    config: &VerifierConfig,
) -> Result<Vec<VerificationResult>> {
    let verified = crate::verifiers::verifier::verify_with_code_info(class_file)?;
    let mut results = Vec::with_capacity(class_file.methods.len());

    for (method, verified) in class_file.methods.iter().zip(verified) {
        let result =
            verify_method_preverified(class_file, method, context, config, verified.code_info)?;
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
    class_file: &ClassFile<'_>,
    context: &C,
    config: &VerifierConfig,
    cache: &VerificationCache,
) -> Result<Vec<VerificationResult>> {
    let Some(context_token) = context.cache_token().filter(|_| cache.is_enabled()) else {
        return verify_class(class_file, context, config);
    };
    let verified = crate::verifiers::verifier::verify_with_code_info(class_file)?;
    // Method names/descriptors are unique after class verification. The complete
    // class bytes include every method, so all keys can borrow one exact scope.
    let mut scope = vec![1]; // Distinguish class-wide from standalone method scopes.
    class_file.to_bytes(&mut scope)?;
    scope.extend_from_slice(&context_token.to_be_bytes());
    write!(scope, "{config:?}").map_err(crate::Error::from)?;
    let scope = cache.intern_scope(scope);
    let mut results = Vec::with_capacity(class_file.methods.len());
    for (method, verified) in class_file.methods.iter().zip(verified) {
        results.push(verify_method_scoped(
            class_file,
            method,
            context,
            config,
            cache,
            &scope,
            Some(verified),
        )?);
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
    use crate::attributes::{ExceptionTableEntry, Instruction, StackFrame};
    use crate::constant::Constant;
    use crate::constant_pool::ConstantPool;
    use crate::verifiers::bytecode::config::FallbackStrategy;
    use crate::verifiers::bytecode::control_flow::CODE_INFO_BUILDS;
    use crate::verifiers::bytecode::handlers::test_utils::MockContext;
    use crate::verifiers::method_access_flags::METHOD_FLAG_CHECKS;
    use crate::{JAVA_5, JAVA_8, JavaStr, JavaString, Version};

    struct UnversionedContext;

    impl VerificationContext for UnversionedContext {
        fn verify_member_access(
            &self,
            _access: &crate::verifiers::context::MemberAccess<'_>,
        ) -> Result<()> {
            // This fixture models accessible members; access-policy failures have dedicated cases.
            Ok(())
        }

        fn is_subclass(&self, subclass: &JavaStr, superclass: &JavaStr) -> Result<bool> {
            MockContext::PERMISSIVE.is_subclass(subclass, superclass)
        }

        fn is_assignable(&self, target: &JavaStr, source: &JavaStr) -> Result<bool> {
            MockContext::PERMISSIVE.is_assignable(target, source)
        }

        fn common_superclass(&self, class1: &JavaStr, class2: &JavaStr) -> Result<JavaString> {
            MockContext::PERMISSIVE.common_superclass(class1, class2)
        }
    }

    fn create_mock_class_file() -> ClassFile<'static> {
        let mut constant_pool = ConstantPool::default();
        constant_pool.add(Constant::utf8("TestClass")).unwrap();
        let this_class_index = constant_pool.add(Constant::Class(1)).unwrap();
        constant_pool.add(Constant::utf8("testMethod")).unwrap();
        constant_pool.add(Constant::utf8("()V")).unwrap();
        constant_pool.add(Constant::utf8("Code")).unwrap();

        constant_pool.add_class("java/lang/Object").unwrap();
        ClassFile {
            version: Version::Java8 { minor: 0 },
            constant_pool,
            access_flags: crate::ClassAccessFlags::PUBLIC,
            this_class: this_class_index,
            super_class: 7,
            interfaces: vec![],
            fields: vec![],
            methods: vec![],
            attributes: vec![],
            code_source_url: None,
        }
    }

    #[test]
    fn class_verification_builds_boundaries_and_checks_flags_once() {
        let mut class = create_mock_class_file();
        class.methods.push(Method {
            access_flags: MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
            name_index: 3,
            descriptor_index: 4,
            attributes: vec![Attribute::Code {
                name_index: 5,
                max_stack: 1,
                max_locals: 0,
                code: vec![Instruction::Return],
                exception_table: vec![],
                attributes: vec![],
            }],
        });
        let config = VerifierConfig::default();
        let inference = config
            .clone()
            .with_fallback_strategy(FallbackStrategy::AlwaysInference);
        let assert_once = || {
            assert_eq!(CODE_INFO_BUILDS.get(), 1);
            assert_eq!(METHOD_FLAG_CHECKS.get(), 1);
        };
        for (config, expected) in [
            (&config, VerificationPath::FastPath),
            (&inference, VerificationPath::FastPath),
        ] {
            CODE_INFO_BUILDS.set(0);
            METHOD_FLAG_CHECKS.set(0);
            let results = verify_class(&class, &MockContext::PERMISSIVE, config).unwrap();
            assert_eq!(results.first().unwrap().path_used, expected);
            assert_once();
        }
        let cache = VerificationCache::new(true);
        for expected in [VerificationPath::FastPath, VerificationPath::Cached] {
            CODE_INFO_BUILDS.set(0);
            METHOD_FLAG_CHECKS.set(0);
            let results =
                verify_class_cached(&class, &MockContext::PERMISSIVE, &config, &cache).unwrap();
            assert_eq!(results.first().unwrap().path_used, expected);
            assert_once();
        }
        // An incompatible dead return exercises structural reachability analysis.
        class.version = JAVA_5;
        if let Some(Attribute::Code { code, .. }) = class
            .methods
            .first_mut()
            .and_then(|m| m.attributes.first_mut())
        {
            code.push(Instruction::Ireturn);
        }
        CODE_INFO_BUILDS.set(0);
        METHOD_FLAG_CHECKS.set(0);
        verify_class(&class, &MockContext::PERMISSIVE, &config).unwrap();
        assert_once();

        // Java 6 fallback must consume the fast path's original boundaries.
        class.version = JAVA_6;
        if let Some(Attribute::Code { code, .. }) = class
            .methods
            .first_mut()
            .and_then(|m| m.attributes.first_mut())
        {
            *code = vec![Instruction::Goto(3), Instruction::Return];
        }
        CODE_INFO_BUILDS.set(0);
        METHOD_FLAG_CHECKS.set(0);
        let results = verify_class(&class, &MockContext::PERMISSIVE, &config).unwrap();
        assert_eq!(
            results.first().unwrap().path_used,
            VerificationPath::Inference
        );
        assert_once();
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
        let context = MockContext::PERMISSIVE;

        let result = verify_method(&class_file, &method, &context, &config);
        assert!(result.is_ok());

        let result = result.unwrap();
        assert!(result.success);

        let inference_config =
            VerifierConfig::default().with_fallback_strategy(FallbackStrategy::AlwaysInference);
        let inference_result = verify_method(&class_file, &method, &context, &inference_config)
            .expect("inference preference must preserve mandatory type checking");
        assert_eq!(VerificationPath::FastPath, inference_result.path_used);
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
        let context = MockContext::PERMISSIVE;

        let result = verify_method(&class_file, &method, &context, &config);
        assert!(result.is_ok());

        let result = result.unwrap();
        assert!(result.success);
        assert_eq!(result.path_used, VerificationPath::Skipped);
    }

    #[test]
    fn cache_revalidates_changed_code_and_configuration() {
        let mut class = create_mock_class_file();
        let mut method = Method {
            access_flags: MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
            name_index: 3,
            descriptor_index: 4,
            attributes: vec![Attribute::Code {
                name_index: 5,
                max_stack: 0,
                max_locals: 0,
                code: vec![Instruction::Return],
                exception_table: vec![],
                attributes: vec![],
            }],
        };
        let cache = VerificationCache::new(true);
        let config = VerifierConfig::strict();
        let context = MockContext::PERMISSIVE;
        assert!(verify_method_cached(&class, &method, &context, &config, &cache).is_ok());
        if let Attribute::Code { code, .. } = &mut method.attributes[0] {
            *code = vec![Instruction::Pop, Instruction::Return];
        }
        assert!(verify_method_cached(&class, &method, &context, &config, &cache).is_err());
        if let Attribute::Code { code, .. } = &mut method.attributes[0] {
            *code = vec![Instruction::Return];
        }
        class.version = JAVA_5;
        let config = VerifierConfig::strict()
            .with_fallback_strategy(FallbackStrategy::AlwaysInference)
            .with_max_inference_iterations(0);
        assert!(verify_method_cached(&class, &method, &context, &config, &cache).is_err());
    }

    #[test]
    fn cache_bypasses_context_without_revision_token() {
        let class_file = create_mock_class_file();
        let method = Method {
            access_flags: MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
            name_index: 3,
            descriptor_index: 4,
            attributes: vec![Attribute::Code {
                name_index: 5,
                max_stack: 0,
                max_locals: 0,
                code: vec![Instruction::Return],
                exception_table: vec![],
                attributes: vec![],
            }],
        };
        let cache = VerificationCache::new(true);
        let config = VerifierConfig::strict();

        for _ in 0..2 {
            let result =
                verify_method_cached(&class_file, &method, &UnversionedContext, &config, &cache)
                    .expect("an unversioned context should still verify the method");
            assert!(result.success);
            assert_ne!(result.path_used, VerificationPath::Cached);
        }
        assert_eq!(cache.result_count(), 0);
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
        let context = MockContext::PERMISSIVE;
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
    fn test_unversioned_context_relationships() {
        let context = UnversionedContext;
        assert!(
            context
                .is_subclass(
                    &JavaStr::cow_from_str("Child"),
                    &JavaStr::cow_from_str("Parent")
                )
                .unwrap()
        );
        assert!(
            context
                .is_assignable(
                    &JavaStr::cow_from_str("Target"),
                    &JavaStr::cow_from_str("Source")
                )
                .unwrap()
        );
        assert_eq!(
            "java/lang/Object",
            context
                .common_superclass(
                    &JavaStr::cow_from_str("Left"),
                    &JavaStr::cow_from_str("Right")
                )
                .unwrap()
        );
    }

    #[test]
    fn test_unversioned_context_and_inference_fallbacks() {
        let mut class_file = create_mock_class_file();
        class_file.version = JAVA_6;
        let context = UnversionedContext;
        let fallback_method = Method {
            access_flags: MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
            name_index: 3,
            descriptor_index: 4,
            attributes: vec![Attribute::Code {
                name_index: 5,
                max_stack: 0,
                max_locals: 0,
                code: vec![Instruction::Goto(4), Instruction::Nop, Instruction::Return],
                exception_table: Vec::new(),
                attributes: Vec::new(),
            }],
        };
        let result = verify_method(
            &class_file,
            &fallback_method,
            &context,
            &VerifierConfig::permissive(),
        )
        .expect("permissive verifier should fall back to inference");
        assert_eq!(VerificationPath::Inference, result.path_used);
        class_file.version = JAVA_8;

        let mismatched_stackmap_method = Method {
            access_flags: MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
            name_index: 3,
            descriptor_index: 4,
            attributes: vec![Attribute::Code {
                name_index: 5,
                max_stack: 1,
                max_locals: 0,
                code: vec![Instruction::Iconst_0, Instruction::Return],
                exception_table: Vec::new(),
                attributes: vec![Attribute::StackMapTable {
                    name_index: 0,
                    frames: vec![StackFrame::SameFrame { frame_type: 1 }],
                }],
            }],
        };
        let mismatch_error = verify_method(
            &class_file,
            &mismatched_stackmap_method,
            &context,
            &VerifierConfig::permissive(),
        )
        .unwrap_err()
        .to_string();
        assert!(mismatch_error.contains("mismatch"));

        let mut class_with_methods = class_file.clone();
        class_with_methods.methods.push(Method {
            access_flags: MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
            name_index: 3,
            descriptor_index: 4,
            attributes: vec![Attribute::Code {
                name_index: 5,
                max_stack: 0,
                max_locals: 0,
                code: vec![Instruction::Return],
                exception_table: Vec::new(),
                attributes: Vec::new(),
            }],
        });
        let results = verify_class(&class_with_methods, &context, &VerifierConfig::default())
            .expect("class methods should verify");
        assert_eq!(1, results.len());

        let failed_fast_path_method = Method {
            access_flags: MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
            name_index: 3,
            descriptor_index: 4,
            attributes: vec![Attribute::Code {
                name_index: 5,
                max_stack: 1,
                max_locals: 0,
                code: vec![Instruction::Return],
                exception_table: vec![ExceptionTableEntry {
                    range_pc: 0..1,
                    handler_pc: 9,
                    catch_type: 0,
                }],
                attributes: vec![Attribute::StackMapTable {
                    name_index: 0,
                    frames: vec![StackFrame::SameFrame { frame_type: 0 }],
                }],
            }],
        };
        let error = verify_method(
            &class_file,
            &failed_fast_path_method,
            &context,
            &VerifierConfig::permissive(),
        )
        .unwrap_err()
        .to_string();
        assert_ne!(error, "");
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

    #[test]
    fn test_verification_result_failed_and_diagnostic() {
        let diagnostic = VerificationDiagnostic::new("Test", "method", "()V", 1, "bad stack");
        let result = VerificationResult::failed(VerificationPath::FastPath, "failed")
            .with_diagnostic(diagnostic);

        assert!(!result.success);
        assert_eq!(VerificationPath::FastPath, result.path_used);
        assert_eq!(Some("failed"), result.error.as_deref());
        assert!(result.diagnostic.is_some());
    }

    #[test]
    fn test_verify_method_no_code_and_native_code_error() {
        let class_file = create_mock_class_file();
        let config = VerifierConfig::default();
        let context = MockContext::PERMISSIVE;

        let no_code_method = Method {
            access_flags: MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
            name_index: 3,
            descriptor_index: 4,
            attributes: Vec::new(),
        };
        assert!(matches!(
            verify_method(&class_file, &no_code_method, &context, &config),
            Err(VerifyError::ClassFormatError(_))
        ));

        let native_with_code = Method {
            access_flags: MethodAccessFlags::PUBLIC | MethodAccessFlags::NATIVE,
            name_index: 3,
            descriptor_index: 4,
            attributes: vec![Attribute::Code {
                name_index: 5,
                max_stack: 0,
                max_locals: 0,
                code: vec![Instruction::Return],
                exception_table: Vec::new(),
                attributes: Vec::new(),
            }],
        };
        assert!(matches!(
            verify_method(&class_file, &native_with_code, &context, &config),
            Err(VerifyError::ClassFormatError(_))
        ));

        let mut bad_native_name = native_with_code;
        bad_native_name.name_index = 999;
        assert!(matches!(
            verify_method(&class_file, &bad_native_name, &context, &config),
            Err(VerifyError::InvalidConstantPoolIndex(_))
        ));
    }

    #[test]
    fn test_verify_cached_failure_and_class_cached() {
        let mut class_file = create_mock_class_file();
        let invalid_method = Method {
            access_flags: MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
            name_index: 3,
            descriptor_index: 4,
            attributes: Vec::new(),
        };
        class_file.methods = vec![invalid_method.clone()];

        let config = VerifierConfig::default();
        let context = MockContext::PERMISSIVE;
        let cache = VerificationCache::new(true);

        let first = verify_method_cached(&class_file, &invalid_method, &context, &config, &cache);
        assert!(first.is_err());
        let second = verify_method_cached(&class_file, &invalid_method, &context, &config, &cache);
        assert!(matches!(second, Err(VerifyError::VerifyError(_))));

        let mut valid_class = create_mock_class_file();
        valid_class.methods = vec![Method {
            access_flags: MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
            name_index: 3,
            descriptor_index: 4,
            attributes: vec![Attribute::Code {
                name_index: 5,
                max_stack: 1,
                max_locals: 1,
                code: vec![Instruction::Return],
                exception_table: Vec::new(),
                attributes: Vec::new(),
            }],
        }];
        let valid_cache = VerificationCache::new(true);
        let results = verify_class_cached(&valid_class, &context, &config, &valid_cache)
            .expect("class verification should succeed");
        assert_eq!(1, results.len());
    }

    #[test]
    fn test_verify_method_cached_key_errors() {
        let class_file = create_mock_class_file();
        let config = VerifierConfig::default();
        let context = MockContext::PERMISSIVE;
        let cache = VerificationCache::new(true);
        let method = Method {
            access_flags: MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
            name_index: 3,
            descriptor_index: 4,
            attributes: vec![Attribute::Code {
                name_index: 5,
                max_stack: 0,
                max_locals: 0,
                code: vec![Instruction::Return],
                exception_table: Vec::new(),
                attributes: Vec::new(),
            }],
        };

        let mut bad_class_file = class_file.clone();
        bad_class_file.this_class = 999;
        assert!(verify_method_cached(&bad_class_file, &method, &context, &config, &cache).is_err());

        let mut bad_method_name = method.clone();
        bad_method_name.name_index = 999;
        assert!(
            verify_method_cached(&class_file, &bad_method_name, &context, &config, &cache).is_err()
        );

        let mut bad_descriptor = method;
        bad_descriptor.descriptor_index = 999;
        assert!(
            verify_method_cached(&class_file, &bad_descriptor, &context, &config, &cache).is_err()
        );
    }

    #[test]
    fn test_error_classification_remaining_variants() {
        assert_eq!(
            ErrorClass::IllegalAccessError,
            ErrorClass::classify(&VerifyError::IllegalAccessError("denied".to_string()))
        );
        assert_eq!(
            ErrorClass::IncompatibleClassChangeError,
            ErrorClass::classify(&VerifyError::IncompatibleClassChangeError(
                "changed".to_string()
            ))
        );
        assert_eq!(
            ErrorClass::VerifyError,
            ErrorClass::classify(&VerifyError::VerificationError {
                context: "ctx".to_string(),
                message: "msg".to_string(),
            })
        );
        assert_eq!(
            ErrorClass::VerifyError,
            ErrorClass::classify(&VerifyError::InvalidInstruction(0))
        );
    }

    #[test]
    fn class_cache_scope_tracks_contents_and_context() {
        let mut class = create_mock_class_file();
        class.methods.push(Method {
            access_flags: MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
            name_index: 3,
            descriptor_index: 4,
            attributes: vec![Attribute::Code {
                name_index: 5,
                max_stack: 0,
                max_locals: 0,
                code: vec![Instruction::Return],
                exception_table: vec![],
                attributes: vec![],
            }],
        });
        let cache = VerificationCache::new(true);
        let config = VerifierConfig::default();
        let first = verify_class_cached(&class, &MockContext::PERMISSIVE, &config, &cache).unwrap();
        assert!(
            first
                .iter()
                .all(|r| r.path_used != VerificationPath::Cached)
        );
        let second =
            verify_class_cached(&class, &MockContext::PERMISSIVE, &config, &cache).unwrap();
        assert!(
            second
                .iter()
                .all(|r| r.path_used == VerificationPath::Cached)
        );
        let different_context =
            verify_class_cached(&class, &MockContext::STRICT, &config, &cache).unwrap();
        assert!(
            different_context
                .iter()
                .all(|r| r.path_used != VerificationPath::Cached)
        );
        class.constant_pool.add(Constant::Integer(42)).unwrap();
        let changed =
            verify_class_cached(&class, &MockContext::PERMISSIVE, &config, &cache).unwrap();
        assert!(
            changed
                .iter()
                .all(|r| r.path_used != VerificationPath::Cached)
        );
    }

    #[test]
    fn class_cache_bypasses_disabled_cache_and_unversioned_context() {
        let mut class = create_mock_class_file();
        class.methods.push(Method {
            access_flags: MethodAccessFlags::PUBLIC | MethodAccessFlags::NATIVE,
            name_index: 3,
            descriptor_index: 4,
            attributes: vec![],
        });
        let config = VerifierConfig::default();
        let cache = VerificationCache::new(true);
        let disabled = VerificationCache::disabled();
        for _ in 0..2 {
            let unversioned =
                verify_class_cached(&class, &UnversionedContext, &config, &cache).unwrap();
            let uncached =
                verify_class_cached(&class, &MockContext::PERMISSIVE, &config, &disabled).unwrap();
            for results in [unversioned, uncached] {
                assert_eq!(results.len(), 1);
                assert!(
                    results
                        .iter()
                        .all(|result| result.success
                            && result.path_used == VerificationPath::Skipped)
                );
            }
        }
        assert_eq!(cache.result_count(), 0);
        assert_eq!(disabled.result_count(), 0);
    }

    #[test]
    fn fallback_reuses_boundaries_without_changing_validation() {
        let mut class = create_mock_class_file();
        class.version = JAVA_6;
        let config = VerifierConfig::permissive();
        let mut method = Method {
            access_flags: MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
            name_index: 3,
            descriptor_index: 4,
            attributes: vec![Attribute::Code {
                name_index: 5,
                max_stack: 0,
                max_locals: 0,
                code: vec![Instruction::Goto(3), Instruction::Return],
                exception_table: vec![],
                attributes: vec![],
            }],
        };
        assert_eq!(
            verify_method(&class, &method, &MockContext::PERMISSIVE, &config)
                .unwrap()
                .path_used,
            VerificationPath::Inference
        );
        if let Some(Attribute::Code { code, .. }) = method.attributes.first_mut() {
            *code = vec![Instruction::Goto(1), Instruction::Return];
        }
        assert!(verify_method(&class, &method, &MockContext::PERMISSIVE, &config).is_err());
    }
}
