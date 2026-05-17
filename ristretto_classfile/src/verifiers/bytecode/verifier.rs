use crate::class_file::ClassFile;
use crate::method::Method;
use crate::verifiers::bytecode::config::VerifierConfig;
use crate::verifiers::bytecode::unified;
use crate::verifiers::context::VerificationContext;
use crate::verifiers::error::Result;

/// Verifies a method's bytecode using the unified bytecode verifier.
///
/// This compatibility entry point keeps the historical `bytecode::verify`
/// API while delegating to the active verifier implementation.
///
/// # Errors
///
/// Returns a `VerifyError` if the bytecode is invalid.
pub fn verify<C: VerificationContext>(
    class_file: &ClassFile<'_>,
    method: &Method,
    context: &C,
) -> Result<()> {
    let config = VerifierConfig::default();
    unified::verify_method(class_file, method, context, &config).map(|_| ())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::attributes::{Attribute, Instruction};
    use crate::constant::Constant;
    use crate::constant_pool::ConstantPool;
    use crate::method_access_flags::MethodAccessFlags;
    use crate::verifiers::bytecode::handlers::test_utils::MockContext;
    use crate::verifiers::error::VerifyError;
    use crate::{ClassAccessFlags, Version};

    fn create_mock_class_file() -> ClassFile<'static> {
        let mut constant_pool = ConstantPool::default();
        constant_pool.add(Constant::utf8("TestClass")).unwrap();
        let this_class_index = constant_pool.add(Constant::Class(1)).unwrap();
        constant_pool.add(Constant::utf8("testMethod")).unwrap();
        constant_pool.add(Constant::utf8("()V")).unwrap();
        constant_pool.add(Constant::utf8("Code")).unwrap();

        ClassFile {
            version: Version::Java8 { minor: 0 },
            constant_pool,
            access_flags: ClassAccessFlags::PUBLIC,
            this_class: this_class_index,
            super_class: 0,
            interfaces: Vec::new(),
            fields: Vec::new(),
            methods: Vec::new(),
            attributes: Vec::new(),
            code_source_url: None,
        }
    }

    fn method_with_code(code: Vec<Instruction>, max_stack: u16, max_locals: u16) -> Method {
        Method {
            access_flags: MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
            name_index: 3,
            descriptor_index: 4,
            attributes: vec![Attribute::Code {
                name_index: 5,
                max_stack,
                max_locals,
                code,
                exception_table: Vec::new(),
                attributes: Vec::new(),
            }],
        }
    }

    #[test]
    fn test_mock_context_methods() {
        let context = MockContext::PERMISSIVE;
        assert!(context.is_subclass("A", "B").unwrap());
        assert!(context.is_assignable("A", "B").unwrap());
        assert_eq!(
            "java/lang/Object",
            context.common_superclass("A", "B").unwrap()
        );
    }

    #[test]
    fn test_verify_simple_method() {
        let class_file = create_mock_class_file();
        let method = method_with_code(vec![Instruction::Return], 0, 0);

        assert_eq!(
            Ok(()),
            verify(&class_file, &method, &MockContext::PERMISSIVE)
        );
    }

    #[test]
    fn test_verify_native_method() {
        let class_file = create_mock_class_file();
        let method = Method {
            access_flags: MethodAccessFlags::PUBLIC | MethodAccessFlags::NATIVE,
            name_index: 3,
            descriptor_index: 4,
            attributes: Vec::new(),
        };

        assert_eq!(
            Ok(()),
            verify(&class_file, &method, &MockContext::PERMISSIVE)
        );
    }

    #[test]
    fn test_verify_native_method_with_code_fails() {
        let class_file = create_mock_class_file();
        let mut method = method_with_code(vec![Instruction::Return], 0, 0);
        method.access_flags |= MethodAccessFlags::NATIVE;

        assert!(matches!(
            verify(&class_file, &method, &MockContext::PERMISSIVE),
            Err(VerifyError::ClassFormatError(_))
        ));
    }

    #[test]
    fn test_verify_method_without_code_fails() {
        let class_file = create_mock_class_file();
        let method = Method {
            access_flags: MethodAccessFlags::PUBLIC,
            name_index: 3,
            descriptor_index: 4,
            attributes: Vec::new(),
        };

        assert!(matches!(
            verify(&class_file, &method, &MockContext::PERMISSIVE),
            Err(VerifyError::ClassFormatError(_))
        ));
    }

    #[test]
    fn test_verify_arithmetic() {
        let class_file = create_mock_class_file();
        let method = method_with_code(
            vec![
                Instruction::Iconst_1,
                Instruction::Iconst_2,
                Instruction::Iadd,
                Instruction::Pop,
                Instruction::Return,
            ],
            2,
            0,
        );

        assert_eq!(
            Ok(()),
            verify(&class_file, &method, &MockContext::PERMISSIVE)
        );
    }

    #[test]
    fn test_verify_local_variables() {
        let class_file = create_mock_class_file();
        let method = method_with_code(
            vec![
                Instruction::Iconst_5,
                Instruction::Istore_0,
                Instruction::Iload_0,
                Instruction::Pop,
                Instruction::Return,
            ],
            1,
            1,
        );

        assert_eq!(
            Ok(()),
            verify(&class_file, &method, &MockContext::PERMISSIVE)
        );
    }

    #[test]
    fn test_verify_rejects_missing_stackmap_control_flow() {
        let class_file = create_mock_class_file();
        let method = method_with_code(
            vec![Instruction::Goto(2), Instruction::Nop, Instruction::Return],
            0,
            0,
        );

        let error = verify(&class_file, &method, &MockContext::PERMISSIVE)
            .expect_err("control flow without StackMapTable must be rejected");
        assert!(matches!(error, VerifyError::VerifyError(_)));
        assert!(
            error
                .to_string()
                .contains("StackMapTable required for class file version 50+ with control flow")
        );
    }
}
