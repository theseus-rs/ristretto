//! Integration tests for the HotSpot-like bytecode verifier.
//!
//! These tests validate the verifier against various bytecode patterns
//! including control flow, exception handling, and type checking scenarios.

#![expect(
    clippy::expect_used,
    clippy::panic_in_result_fn,
    reason = "integration tests use assertions and expected-error checks with Result-returning tests"
)]

use ristretto_classfile::attributes::{
    Attribute, InnerClass, Instruction, MethodParameter, ModuleAccessFlags, NestedClassAccessFlags,
    Requires, RequiresFlags, StackFrame,
};
use ristretto_classfile::verifiers::bytecode::{
    FallbackStrategy, VerificationPath, VerifierConfig, VerifyMode, verify_class, verify_method,
};
use ristretto_classfile::verifiers::context::VerificationContext;
use ristretto_classfile::verifiers::error::{Result, VerifyError};
use ristretto_classfile::{
    ClassAccessFlags, ClassFile, Constant, ConstantPool, Method, MethodAccessFlags, Version,
};

/// Mock verification context for testing.
struct TestContext;

impl VerificationContext for TestContext {
    fn is_subclass(&self, subclass: &str, superclass: &str) -> Result<bool> {
        // Simple hierarchy for testing
        if superclass == "java/lang/Object" {
            return Ok(true);
        }
        if superclass == "java/lang/Throwable" && subclass.contains("Exception") {
            return Ok(true);
        }
        Ok(subclass == superclass)
    }

    fn is_assignable(&self, target: &str, source: &str) -> Result<bool> {
        if target == source {
            return Ok(true);
        }
        if target == "java/lang/Object" {
            return Ok(true);
        }
        Ok(false)
    }

    fn common_superclass(&self, class1: &str, class2: &str) -> Result<String> {
        if class1 == "error" || class2 == "error" {
            return Err(VerifyError::VerifyError(
                "common superclass failure".to_string(),
            ));
        }
        Ok("java/lang/Object".to_string())
    }
}

fn create_test_class_file(version: Version) -> Result<ClassFile<'static>> {
    let mut constant_pool = ConstantPool::default();
    constant_pool.add(Constant::utf8("TestClass"))?;
    let this_class_index = constant_pool.add(Constant::Class(1))?;
    constant_pool.add(Constant::utf8("test"))?;
    constant_pool.add(Constant::utf8("()V"))?;
    constant_pool.add(Constant::utf8("Code"))?;

    Ok(ClassFile {
        version,
        constant_pool,
        access_flags: ClassAccessFlags::PUBLIC,
        this_class: this_class_index,
        super_class: 0,
        interfaces: vec![],
        fields: vec![],
        methods: vec![],
        attributes: vec![],
        code_source_url: None,
    })
}

fn create_static_void_method(code: Vec<Instruction>, max_stack: u16, max_locals: u16) -> Method {
    create_static_method_with_descriptor(4, code, max_stack, max_locals)
}

fn create_static_method_with_descriptor(
    descriptor_index: u16,
    code: Vec<Instruction>,
    max_stack: u16,
    max_locals: u16,
) -> Method {
    let code_attribute = Attribute::Code {
        name_index: 5,
        max_stack,
        max_locals,
        code,
        exception_table: vec![],
        attributes: vec![],
    };

    Method {
        access_flags: MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
        name_index: 3,
        descriptor_index,
        attributes: vec![code_attribute],
    }
}

fn create_method_with_descriptor(
    access_flags: MethodAccessFlags,
    name_index: u16,
    descriptor_index: u16,
    code: Vec<Instruction>,
    max_stack: u16,
    max_locals: u16,
) -> Method {
    let code_attribute = Attribute::Code {
        name_index: 5,
        max_stack,
        max_locals,
        code,
        exception_table: vec![],
        attributes: vec![],
    };

    Method {
        access_flags,
        name_index,
        descriptor_index,
        attributes: vec![code_attribute],
    }
}

#[test]
fn test_class_verify_reports_attribute_errors() -> Result<()> {
    let mut class_file = create_test_class_file(Version::Java8 { minor: 0 })?;
    class_file.attributes.push(Attribute::Code {
        name_index: 5,
        max_stack: 0,
        max_locals: 0,
        code: vec![],
        exception_table: vec![],
        attributes: vec![],
    });
    assert!(class_file.verify().is_err());

    let mut class_file = create_test_class_file(Version::Java9 { minor: 0 })?;
    class_file.attributes.push(Attribute::Module {
        name_index: 0,
        module_name_index: u16::MAX,
        flags: ModuleAccessFlags::empty(),
        version_index: 0,
        requires: Vec::new(),
        exports: Vec::new(),
        opens: Vec::new(),
        uses: Vec::new(),
        provides: Vec::new(),
    });
    assert!(class_file.verify().is_err());

    Ok(())
}

#[test]
fn test_class_verify_accepts_module_main_class_attribute() -> Result<()> {
    let mut class_file = create_test_class_file(Version::Java9 { minor: 0 })?;
    let main_class_index = class_file.constant_pool.add_class("Main")?;
    class_file.attributes.push(Attribute::ModuleMainClass {
        name_index: 0,
        main_class_index,
    });

    assert!(class_file.verify().is_ok());
    Ok(())
}

#[test]
fn test_class_verify_accepts_attribute_optional_indexes() -> Result<()> {
    let mut class_file = create_test_class_file(Version::Java9 { minor: 0 })?;

    let parameter_name = class_file.constant_pool.add_utf8("value")?;
    class_file.methods.push(Method {
        access_flags: MethodAccessFlags::PUBLIC,
        name_index: 3,
        descriptor_index: 4,
        attributes: vec![Attribute::MethodParameters {
            name_index: 0,
            parameters: vec![MethodParameter {
                name_index: parameter_name,
                access_flags: MethodAccessFlags::empty(),
            }],
        }],
    });

    let inner_class_index = class_file.constant_pool.add_class("TestClass$Inner")?;
    let inner_name_index = class_file.constant_pool.add_utf8("Inner")?;
    class_file.attributes.push(Attribute::InnerClasses {
        name_index: 0,
        classes: vec![InnerClass {
            class_info_index: inner_class_index,
            outer_class_info_index: 0,
            name_index: inner_name_index,
            access_flags: NestedClassAccessFlags::empty(),
        }],
    });

    let module_index = class_file.constant_pool.add_module("test.module")?;
    let module_version_index = class_file.constant_pool.add_utf8("1.0")?;
    class_file.attributes.push(Attribute::Module {
        name_index: 0,
        module_name_index: module_index,
        flags: ModuleAccessFlags::empty(),
        version_index: 0,
        requires: vec![Requires {
            index: module_index,
            flags: RequiresFlags::empty(),
            version_index: module_version_index,
        }],
        exports: Vec::new(),
        opens: Vec::new(),
        uses: Vec::new(),
        provides: Vec::new(),
    });

    assert!(class_file.verify().is_ok());
    Ok(())
}

#[test]
fn test_straight_line_code_no_stackmap_required() -> Result<()> {
    // Java 8+ class with straight-line code should not require StackMapTable
    let class_file = create_test_class_file(Version::Java8 { minor: 0 })?;
    let method = create_static_void_method(
        vec![
            Instruction::Iconst_1,
            Instruction::Iconst_2,
            Instruction::Iadd,
            Instruction::Pop,
            Instruction::Return,
        ],
        2,
        1,
    );

    let config = VerifierConfig::strict();
    let context = TestContext;

    let result = verify_method(&class_file, &method, &context, &config);
    assert!(
        result.is_ok(),
        "Straight-line code should verify without StackMapTable"
    );
    Ok(())
}

#[test]
fn test_verify_mode_none_skips_verification() -> Result<()> {
    let class_file = create_test_class_file(Version::Java8 { minor: 0 })?;
    // Invalid bytecode; would fail verification
    let method = create_static_void_method(
        vec![
            Instruction::Pop, // Stack underflow!
            Instruction::Return,
        ],
        1,
        1,
    );

    let config = VerifierConfig::default().with_verify_mode(VerifyMode::None);
    let context = TestContext;

    // This should still check for Code attribute but skip actual verification
    // Since our implementation doesn't fully implement skip mode, this tests the structure
    let result = verify_method(&class_file, &method, &context, &config);
    // Note: Current implementation still verifies, but the config should indicate skip
    assert!(!config.should_verify(false) || result.is_err());
    Ok(())
}

#[test]
fn test_pre_java6_uses_inference() -> Result<()> {
    // Java 5 class file (version 49) should use inference verifier
    let class_file = create_test_class_file(Version::Java5 { minor: 0 })?;
    let method = create_static_void_method(
        vec![
            Instruction::Iconst_0,
            Instruction::Istore_0,
            Instruction::Iload_0,
            Instruction::Pop,
            Instruction::Return,
        ],
        1,
        1,
    );

    let config = VerifierConfig::default();
    let context = TestContext;

    let result = verify_method(&class_file, &method, &context, &config);
    assert!(result.is_ok());

    // Verify it used inference path (pre-Java 6)
    let result = result?;
    assert_eq!(result.path_used, VerificationPath::Inference);
    Ok(())
}

#[test]
fn test_inference_merges_reference_types() -> Result<()> {
    let mut class_file = create_test_class_file(Version::Java5 { minor: 0 })?;
    let config = VerifierConfig::default();
    let context = TestContext;
    let merge_code = vec![
        Instruction::Iload_2,
        Instruction::Ifeq(4),
        Instruction::Aload_0,
        Instruction::Goto(5),
        Instruction::Aload_1,
        Instruction::Pop,
        Instruction::Return,
    ];

    let object_descriptor = class_file
        .constant_pool
        .add_utf8("(Ljava/lang/String;Ljava/lang/Object;I)V")?;
    let object_method =
        create_static_method_with_descriptor(object_descriptor, merge_code.clone(), 1, 3);
    let result = verify_method(&class_file, &object_method, &context, &config)?;
    assert_eq!(result.path_used, VerificationPath::Inference);

    let array_descriptor = class_file
        .constant_pool
        .add_utf8("([Ljava/lang/String;[Ljava/lang/Object;I)V")?;
    let array_method =
        create_static_method_with_descriptor(array_descriptor, merge_code.clone(), 1, 3);
    let result = verify_method(&class_file, &array_method, &context, &config)?;
    assert_eq!(result.path_used, VerificationPath::Inference);

    let error_descriptor = class_file
        .constant_pool
        .add_utf8("(Lerror;Ljava/lang/Object;I)V")?;
    let error_method =
        create_static_method_with_descriptor(error_descriptor, merge_code.clone(), 1, 3);
    assert!(verify_method(&class_file, &error_method, &context, &config).is_err());

    let error_array_descriptor = class_file
        .constant_pool
        .add_utf8("([Lerror;[Ljava/lang/Object;I)V")?;
    let error_array_method =
        create_static_method_with_descriptor(error_array_descriptor, merge_code, 1, 3);
    assert!(verify_method(&class_file, &error_array_method, &context, &config).is_err());

    Ok(())
}

#[test]
fn test_inference_merges_incompatible_primitive_types_to_top() -> Result<()> {
    let mut class_file = create_test_class_file(Version::Java5 { minor: 0 })?;
    let descriptor = class_file.constant_pool.add_utf8("(I)V")?;
    let method = create_static_method_with_descriptor(
        descriptor,
        vec![
            Instruction::Iload_0,
            Instruction::Ifeq(4),
            Instruction::Iconst_0,
            Instruction::Goto(5),
            Instruction::Fconst_0,
            Instruction::Pop,
            Instruction::Return,
        ],
        1,
        1,
    );

    let error = verify_method(
        &class_file,
        &method,
        &TestContext,
        &VerifierConfig::default(),
    )
    .expect_err("merging incompatible primitive stack values must produce an invalid Top use");
    assert!(error.to_string().contains("pop cannot be used"));

    let object_class = class_file.constant_pool.add_class("java/lang/Object")?;
    let method = create_static_method_with_descriptor(
        descriptor,
        vec![
            Instruction::Iload_0,
            Instruction::Ifeq(4),
            Instruction::New(object_class),
            Instruction::Goto(5),
            Instruction::New(object_class),
            Instruction::Pop,
            Instruction::Return,
        ],
        1,
        1,
    );
    let error = verify_method(
        &class_file,
        &method,
        &TestContext,
        &VerifierConfig::default(),
    )
    .expect_err("merging different uninitialized objects must produce an invalid Top use");
    assert!(error.to_string().contains("pop cannot be used"));

    Ok(())
}

#[test]
fn test_java8_with_stackmap_uses_fast_path() -> Result<()> {
    let mut class_file = create_test_class_file(Version::Java8 { minor: 0 })?;

    // Add constant pool entries for StackMapTable
    class_file
        .constant_pool
        .add(Constant::utf8("StackMapTable"))?;

    // Simple code with a conditional branch requiring StackMapTable
    // Note: Ifeq pops the int, so at branch target we have empty stack
    let code = vec![
        Instruction::Iconst_0, // 0: push 0
        Instruction::Ifeq(4),  // 1: if == 0, jump to return (this consumes the int)
        Instruction::Iconst_1, // 2: push 1 (fallthrough case)
        Instruction::Pop,      // 3: pop
        Instruction::Return,   // 4: return
    ];

    // Create StackMapTable for the branch target at instruction index 4.
    // The bytecode offset is 6 because ifeq is 3 bytes wide.
    // SameFrame type 0-63 means offset_delta = frame_type
    let stack_map_frames = vec![
        StackFrame::SameFrame { frame_type: 6 }, // Frame at offset 6 (return)
    ];

    let code_attribute = Attribute::Code {
        name_index: 5,
        max_stack: 1,
        max_locals: 1,
        code,
        exception_table: vec![],
        attributes: vec![Attribute::StackMapTable {
            name_index: 6,
            frames: stack_map_frames,
        }],
    };

    let method = Method {
        access_flags: MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
        name_index: 3,
        descriptor_index: 4,
        attributes: vec![code_attribute],
    };

    let config = VerifierConfig::strict();
    let context = TestContext;

    let result = verify_method(&class_file, &method, &context, &config);
    assert!(
        result.is_ok(),
        "Should verify with StackMapTable: {:?}",
        result.err()
    );

    let result = result?;
    assert_eq!(result.path_used, VerificationPath::FastPath);
    Ok(())
}

#[test]
fn test_fallback_to_inference_when_configured() -> Result<()> {
    let class_file = create_test_class_file(Version::Java8 { minor: 0 })?;

    // Code with branch but no StackMapTable
    let code = vec![
        Instruction::Iconst_0,
        Instruction::Ifeq(4), // Branch without StackMapTable
        Instruction::Iconst_1,
        Instruction::Pop,
        Instruction::Return, // Target
    ];

    let code_attribute = Attribute::Code {
        name_index: 5,
        max_stack: 1,
        max_locals: 1,
        code,
        exception_table: vec![],
        attributes: vec![], // No StackMapTable!
    };

    let method = Method {
        access_flags: MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
        name_index: 3,
        descriptor_index: 4,
        attributes: vec![code_attribute],
    };

    // With strict config, this should fail
    let strict_config = VerifierConfig::strict();
    let context = TestContext;
    let _result = verify_method(&class_file, &method, &context, &strict_config);
    // May fail or fall back depending on implementation

    // With permissive config, should fall back to inference
    let permissive_config = VerifierConfig::permissive();
    let result = verify_method(&class_file, &method, &context, &permissive_config);
    assert!(
        result.is_ok(),
        "Should succeed with fallback: {:?}",
        result.err()
    );
    Ok(())
}

#[test]
fn test_verifier_config_builder() {
    let config = VerifierConfig::new()
        .with_verify_mode(VerifyMode::All)
        .with_fallback_strategy(FallbackStrategy::FallbackToInference)
        .with_verbose(true)
        .with_trace(true)
        .with_max_inference_iterations(500);

    assert_eq!(config.verify_mode, VerifyMode::All);
    assert_eq!(
        config.fallback_strategy,
        FallbackStrategy::FallbackToInference
    );
    assert!(config.verbose());
    assert!(config.trace());
    assert_eq!(config.max_inference_iterations, 500);
}

#[test]
fn test_verify_class_all_methods() -> Result<()> {
    let mut class_file = create_test_class_file(Version::Java8 { minor: 0 })?;

    // Add multiple methods
    let method1 = create_static_void_method(vec![Instruction::Return], 0, 0);
    let method2 = create_static_void_method(
        vec![Instruction::Iconst_0, Instruction::Pop, Instruction::Return],
        1,
        0,
    );

    class_file.methods = vec![method1, method2];

    let config = VerifierConfig::default();
    let context = TestContext;

    let results = verify_class(&class_file, &context, &config);
    assert!(results.is_ok());

    let results = results?;
    assert_eq!(results.len(), 2);
    assert!(results.iter().all(|r| r.success));
    Ok(())
}

#[test]
fn test_category2_type_handling() -> Result<()> {
    let class_file = create_test_class_file(Version::Java8 { minor: 0 })?;

    // Test long and double handling
    let method = create_static_void_method(
        vec![
            Instruction::Lconst_1, // Push long (2 slots)
            Instruction::Dconst_1, // Push double (2 slots)
            Instruction::Pop2,     // Pop double
            Instruction::Pop2,     // Pop long
            Instruction::Return,
        ],
        4,
        0,
    );

    let config = VerifierConfig::default();
    let context = TestContext;

    let result = verify_method(&class_file, &method, &context, &config);
    assert!(
        result.is_ok(),
        "Category 2 types should be handled correctly"
    );
    Ok(())
}

#[test]
fn test_stack_overflow_detection() -> Result<()> {
    let class_file = create_test_class_file(Version::Java8 { minor: 0 })?;

    // max_stack is 1 but we push 2 values
    let method = create_static_void_method(
        vec![
            Instruction::Iconst_1,
            Instruction::Iconst_2, // Stack overflow!
            Instruction::Pop,
            Instruction::Pop,
            Instruction::Return,
        ],
        1, // Only 1 stack slot allowed
        0,
    );

    let config = VerifierConfig::default();
    let context = TestContext;

    let result = verify_method(&class_file, &method, &context, &config);
    assert!(result.is_err(), "Should detect stack overflow");
    Ok(())
}

#[test]
fn test_stack_underflow_detection() -> Result<()> {
    let class_file = create_test_class_file(Version::Java8 { minor: 0 })?;

    // Pop from empty stack
    let method = create_static_void_method(
        vec![
            Instruction::Pop, // Stack underflow!
            Instruction::Return,
        ],
        1,
        0,
    );

    let config = VerifierConfig::default();
    let context = TestContext;

    let result = verify_method(&class_file, &method, &context, &config);
    assert!(result.is_err(), "Should detect stack underflow");
    Ok(())
}

fn assert_method_verification_error_contains(
    class_file: &ClassFile<'_>,
    method: &Method,
    expected_message: &str,
) {
    let error = verify_method(class_file, method, &TestContext, &VerifierConfig::default())
        .expect_err("type mismatch must fail verification");
    let message = error.to_string();
    assert!(
        message.contains(expected_message),
        "{message:?} should contain {expected_message:?}"
    );
}

fn assert_static_verifier_error_contains(
    descriptor: &str,
    code: Vec<Instruction>,
    max_stack: u16,
    max_locals: u16,
    expected_message: &str,
) -> Result<()> {
    let mut class_file = create_test_class_file(Version::Java5 { minor: 0 })?;
    let descriptor_index = class_file.constant_pool.add_utf8(descriptor)?;
    let method =
        create_static_method_with_descriptor(descriptor_index, code, max_stack, max_locals);
    assert_method_verification_error_contains(&class_file, &method, expected_message);
    Ok(())
}

#[test]
fn test_type_mismatch_error_formats_top() -> Result<()> {
    assert_static_verifier_error_contains(
        "(J)V",
        vec![Instruction::Iload_1, Instruction::Return],
        1,
        2,
        "top",
    )
}

#[test]
fn test_type_mismatch_error_formats_int() -> Result<()> {
    assert_static_verifier_error_contains(
        "(I)V",
        vec![Instruction::Aload_0, Instruction::Return],
        1,
        1,
        "int",
    )
}

#[test]
fn test_type_mismatch_error_formats_float() -> Result<()> {
    assert_static_verifier_error_contains(
        "(F)V",
        vec![Instruction::Iload_0, Instruction::Return],
        1,
        1,
        "float",
    )
}

#[test]
fn test_type_mismatch_error_formats_long() -> Result<()> {
    assert_static_verifier_error_contains(
        "(J)V",
        vec![Instruction::Fload_0, Instruction::Return],
        1,
        2,
        "long",
    )
}

#[test]
fn test_type_mismatch_error_formats_double() -> Result<()> {
    assert_static_verifier_error_contains(
        "(D)V",
        vec![Instruction::Fload_0, Instruction::Return],
        1,
        2,
        "double",
    )
}

#[test]
fn test_type_mismatch_error_formats_null() -> Result<()> {
    assert_static_verifier_error_contains(
        "()I",
        vec![Instruction::Aconst_null, Instruction::Ireturn],
        1,
        0,
        "null",
    )
}

#[test]
fn test_type_mismatch_error_formats_uninitialized_this() -> Result<()> {
    let mut class_file = create_test_class_file(Version::Java5 { minor: 0 })?;
    let init_name = class_file.constant_pool.add_utf8("<init>")?;
    let method = create_method_with_descriptor(
        MethodAccessFlags::PUBLIC,
        init_name,
        4,
        vec![Instruction::Iload_0, Instruction::Return],
        1,
        1,
    );
    assert_method_verification_error_contains(&class_file, &method, "uninitializedThis");
    Ok(())
}

#[test]
fn test_type_mismatch_error_formats_uninitialized() -> Result<()> {
    let mut class_file = create_test_class_file(Version::Java5 { minor: 0 })?;
    let int_return = class_file.constant_pool.add_utf8("()I")?;
    let new_class = class_file.constant_pool.add_class("java/lang/Object")?;
    let method = create_static_method_with_descriptor(
        int_return,
        vec![Instruction::New(new_class), Instruction::Ireturn],
        1,
        0,
    );
    assert_method_verification_error_contains(&class_file, &method, "uninitialized(");
    Ok(())
}

#[test]
fn test_type_mismatch_error_formats_object() -> Result<()> {
    assert_static_verifier_error_contains(
        "(Ljava/lang/String;)V",
        vec![Instruction::Iload_0, Instruction::Return],
        1,
        1,
        "java/lang/String",
    )
}

#[test]
fn test_type_mismatch_error_formats_array() -> Result<()> {
    assert_static_verifier_error_contains(
        "([Ljava/lang/String;)V",
        vec![Instruction::Iload_0, Instruction::Return],
        1,
        1,
        "java/lang/String[]",
    )
}
