//! # Reference Instruction Handlers
//!
//! Handles verification of reference-related instructions:
//! - Object creation: `new`, `newarray`, `anewarray`, `multianewarray`
//! - Field access: `getfield`, `putfield`, `getstatic`, `putstatic`
//! - Method invocation: `invokevirtual`, `invokespecial`, `invokestatic`, `invokeinterface`, `invokedynamic`
//! - Type checks: `checkcast`, `instanceof`
//! - Array operations: `arraylength`
//!
//! # References
//!
//! - [JVMS §6.5 - Instructions](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5)

use std::sync::Arc;

use crate::FieldType;
use crate::attributes::ArrayType;
use crate::class_file::ClassFile;
use crate::verifiers::bytecode::frame::Frame;
use crate::verifiers::bytecode::type_system::VerificationType;
use crate::verifiers::context::VerificationContext;
use crate::verifiers::error::{Result, VerifyError};

/// Context for resolving constant pool entries.
#[derive(Debug)]
pub struct ConstantPoolResolver<'a> {
    class_file: &'a ClassFile,
}

impl<'a> ConstantPoolResolver<'a> {
    /// Creates a new resolver.
    #[must_use]
    pub fn new(class_file: &'a ClassFile) -> Self {
        Self { class_file }
    }

    /// Resolves a field reference.
    ///
    /// Returns (`class_name`, `field_name`, `field_descriptor`).
    ///
    /// # Errors
    ///
    /// Returns an error if the constant pool reference is invalid.
    pub fn resolve_field_ref(&self, index: u16) -> Result<(String, String, String)> {
        let (class_index, name_and_type_index) = self
            .class_file
            .constant_pool
            .try_get_field_ref(index)
            .map_err(|e| VerifyError::ClassFormatError(e.to_string()))?;

        let class_name = self
            .class_file
            .constant_pool
            .try_get_class(*class_index)
            .map_err(|e| VerifyError::ClassFormatError(e.to_string()))?;

        let (name_index, descriptor_index) = self
            .class_file
            .constant_pool
            .try_get_name_and_type(*name_and_type_index)
            .map_err(|e| VerifyError::ClassFormatError(e.to_string()))?;

        let name = self
            .class_file
            .constant_pool
            .try_get_utf8(*name_index)
            .map_err(|e| VerifyError::ClassFormatError(e.to_string()))?;

        let descriptor = self
            .class_file
            .constant_pool
            .try_get_utf8(*descriptor_index)
            .map_err(|e| VerifyError::ClassFormatError(e.to_string()))?;

        Ok((
            class_name.to_string(),
            name.to_string(),
            descriptor.to_string(),
        ))
    }

    /// Resolves a method reference.
    ///
    /// Returns (`class_name`, `method_name`, `method_descriptor`).
    ///
    /// # Errors
    ///
    /// Returns an error if the constant pool reference is invalid.
    pub fn resolve_method_ref(&self, index: u16) -> Result<(String, String, String)> {
        // Try Methodref first, then InterfaceMethodref
        let (class_index, name_and_type_index) = self
            .class_file
            .constant_pool
            .try_get_method_ref(index)
            .or_else(|_| {
                self.class_file
                    .constant_pool
                    .try_get_interface_method_ref(index)
            })
            .map_err(|e| VerifyError::ClassFormatError(e.to_string()))?;

        let class_name = self
            .class_file
            .constant_pool
            .try_get_class(*class_index)
            .map_err(|e| VerifyError::ClassFormatError(e.to_string()))?;

        let (name_index, descriptor_index) = self
            .class_file
            .constant_pool
            .try_get_name_and_type(*name_and_type_index)
            .map_err(|e| VerifyError::ClassFormatError(e.to_string()))?;

        let name = self
            .class_file
            .constant_pool
            .try_get_utf8(*name_index)
            .map_err(|e| VerifyError::ClassFormatError(e.to_string()))?;

        let descriptor = self
            .class_file
            .constant_pool
            .try_get_utf8(*descriptor_index)
            .map_err(|e| VerifyError::ClassFormatError(e.to_string()))?;

        Ok((
            class_name.to_string(),
            name.to_string(),
            descriptor.to_string(),
        ))
    }

    /// Resolves a class reference.
    ///
    /// # Errors
    ///
    /// Returns an error if the constant pool reference is invalid.
    pub fn resolve_class(&self, index: u16) -> Result<String> {
        self.class_file
            .constant_pool
            .try_get_class(index)
            .map(std::string::ToString::to_string)
            .map_err(|e| VerifyError::ClassFormatError(e.to_string()))
    }

    /// Resolves an `InvokeDynamic` reference.
    ///
    /// Returns the method descriptor.
    ///
    /// # Errors
    ///
    /// Returns an error if the constant pool reference is invalid.
    pub fn resolve_invoke_dynamic(&self, index: u16) -> Result<String> {
        let (_bootstrap_index, name_and_type_index) = self
            .class_file
            .constant_pool
            .try_get_invoke_dynamic(index)
            .map_err(|e| VerifyError::ClassFormatError(e.to_string()))?;

        let (_, descriptor_index) = self
            .class_file
            .constant_pool
            .try_get_name_and_type(*name_and_type_index)
            .map_err(|e| VerifyError::ClassFormatError(e.to_string()))?;

        let descriptor = self
            .class_file
            .constant_pool
            .try_get_utf8(*descriptor_index)
            .map_err(|e| VerifyError::ClassFormatError(e.to_string()))?;

        Ok(descriptor.to_string())
    }
}

/// Handles `new` - create new object.
///
/// Stack: ... → ..., objectref
///
/// # Errors
///
/// Returns an error if stack overflow occurs.
///
/// # References
///
/// - [JVMS §6.5.new](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.new)
pub fn handle_new(frame: &mut Frame, offset: u16, _class_name: &str) -> Result<()> {
    // Push an uninitialized reference
    frame.push(VerificationType::Uninitialized(offset))
}

/// Handles `newarray` - create new primitive array.
///
/// Stack: ..., count → ..., arrayref
///
/// # Errors
///
/// Returns an error if the count is not an `int`.
///
/// # References
///
/// - [JVMS §6.5.newarray](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.newarray)
pub fn handle_newarray(frame: &mut Frame, atype: &ArrayType) -> Result<()> {
    // Pop count (must be int)
    let count = frame.pop()?;
    if count != VerificationType::Integer {
        return Err(VerifyError::VerifyError(format!(
            "newarray: expected int count, got {count}"
        )));
    }

    // Determine component type
    let component = match atype {
        ArrayType::Boolean
        | ArrayType::Byte
        | ArrayType::Char
        | ArrayType::Short
        | ArrayType::Int => VerificationType::Integer,
        ArrayType::Float => VerificationType::Float,
        ArrayType::Long => VerificationType::Long,
        ArrayType::Double => VerificationType::Double,
    };

    frame.push(VerificationType::Array(Box::new(component)))
}

/// Handles `anewarray` - create new reference array.
///
/// Stack: ..., count → ..., arrayref
///
/// # Errors
///
/// Returns an error if the count is not an `int`.
///
/// # References
///
/// - [JVMS §6.5.anewarray](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.anewarray)
pub fn handle_anewarray(frame: &mut Frame, class_name: &str) -> Result<()> {
    let count = frame.pop()?;
    if count != VerificationType::Integer {
        return Err(VerifyError::VerifyError(format!(
            "anewarray: expected int count, got {count}"
        )));
    }

    // The component type is the referenced class
    let component = if class_name.starts_with('[') {
        // Creating an array of arrays
        parse_type_descriptor(class_name)?
    } else {
        VerificationType::Object(Arc::from(class_name))
    };

    frame.push(VerificationType::Array(Box::new(component)))
}

/// Handles `multianewarray` - create new multidimensional array.
///
/// Stack: ..., count1, [count2, ...] → ..., arrayref
///
/// # Errors
///
/// Returns an error if the counts are not integers.
///
/// # References
///
/// - [JVMS §6.5.multianewarray](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.multianewarray)
pub fn handle_multianewarray(frame: &mut Frame, class_name: &str, dimensions: u8) -> Result<()> {
    // Pop 'dimensions' int values
    for _ in 0..dimensions {
        let count = frame.pop()?;
        if count != VerificationType::Integer {
            return Err(VerifyError::VerifyError(format!(
                "multianewarray: expected int count, got {count}"
            )));
        }
    }

    // Parse the array type
    let array_type = parse_type_descriptor(class_name)?;
    frame.push(array_type)
}

/// Handles `getfield` - get instance field.
///
/// Stack: ..., objectref → ..., value
///
/// # Errors
///
/// Returns an error if the object type is incorrect.
///
/// # References
///
/// - [JVMS §6.5.getfield](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.getfield)
pub fn handle_getfield<C: VerificationContext>(
    frame: &mut Frame,
    class_name: &str,
    field_descriptor: &str,
    context: &C,
) -> Result<()> {
    let objectref = frame.pop()?;

    // Verify objectref is assignable to the field's class
    if !objectref.is_null() {
        let expected = VerificationType::Object(Arc::from(class_name));
        if !objectref.is_assignable_to(&expected, context)? {
            return Err(VerifyError::VerifyError(format!(
                "getfield: {objectref} is not assignable to {class_name}"
            )));
        }
    }

    // Push the field type
    let field_type = parse_type_descriptor(field_descriptor)?;
    if field_type.is_category2() {
        frame.push(field_type)?;
        frame.push(VerificationType::Top)
    } else {
        frame.push(field_type)
    }
}

/// Handles `putfield` - set instance field.
///
/// Stack: ..., objectref, value → ...
///
/// # Errors
///
/// Returns an error if the types are incorrect.
///
/// # References
///
/// - [JVMS §6.5.putfield](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.putfield)
pub fn handle_putfield<C: VerificationContext>(
    frame: &mut Frame,
    class_name: &str,
    field_descriptor: &str,
    context: &C,
) -> Result<()> {
    let field_type = parse_type_descriptor(field_descriptor)?;

    // Pop value
    let value = if field_type.is_category2() {
        frame.pop_category2()?
    } else {
        frame.pop()?
    };

    // Verify value is assignable to field type
    if !value.is_assignable_to(&field_type, context)? {
        return Err(VerifyError::VerifyError(format!(
            "putfield: {value} is not assignable to {field_type}"
        )));
    }

    // Pop objectref
    let objectref = frame.pop()?;

    // For putfield on uninitialized 'this', the objectref can be UninitializedThis
    // when setting fields in a constructor before calling super()
    if !matches!(
        objectref,
        VerificationType::UninitializedThis | VerificationType::Uninitialized(_)
    ) && !objectref.is_null()
    {
        let expected = VerificationType::Object(Arc::from(class_name));
        if !objectref.is_assignable_to(&expected, context)? {
            return Err(VerifyError::VerifyError(format!(
                "putfield: {objectref} is not assignable to {class_name}"
            )));
        }
    }

    Ok(())
}

/// Handles `getstatic` - get static field.
///
/// Stack: ... → ..., value
///
/// # Errors
///
/// Returns an error if the descriptor is invalid.
///
/// # References
///
/// - [JVMS §6.5.getstatic](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.getstatic)
pub fn handle_getstatic(frame: &mut Frame, field_descriptor: &str) -> Result<()> {
    let field_type = parse_type_descriptor(field_descriptor)?;
    if field_type.is_category2() {
        frame.push(field_type)?;
        frame.push(VerificationType::Top)
    } else {
        frame.push(field_type)
    }
}

/// Handles `putstatic` - set static field.
///
/// Stack: ..., value → ...
///
/// # Errors
///
/// Returns an error if the value type is incorrect.
///
/// # References
///
/// - [JVMS §6.5.putstatic](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.putstatic)
pub fn handle_putstatic<C: VerificationContext>(
    frame: &mut Frame,
    field_descriptor: &str,
    context: &C,
) -> Result<()> {
    let field_type = parse_type_descriptor(field_descriptor)?;

    let value = if field_type.is_category2() {
        frame.pop_category2()?
    } else {
        frame.pop()?
    };

    if !value.is_assignable_to(&field_type, context)? {
        return Err(VerifyError::VerifyError(format!(
            "putstatic: {value} is not assignable to {field_type}"
        )));
    }

    Ok(())
}

/// Handles method invocation instructions.
///
/// Pops arguments and objectref (for non-static), pushes return value.
///
/// # Errors
///
/// Returns an error if the argument types are incorrect.
pub fn handle_invoke<C: VerificationContext>(
    frame: &mut Frame,
    class_name: &str,
    method_name: &str,
    descriptor: &str,
    is_static: bool,
    context: &C,
) -> Result<Option<VerificationType>> {
    let (params, return_type) = FieldType::parse_method_descriptor(descriptor)
        .map_err(|e| VerifyError::ClassFormatError(e.to_string()))?;

    // Pop arguments in reverse order
    for param in params.iter().rev() {
        let param_type = VerificationType::from_field_type(param);
        let value = if param_type.is_category2() {
            frame.pop_category2()?
        } else {
            frame.pop()?
        };

        if !value.is_assignable_to(&param_type, context)? {
            return Err(VerifyError::VerifyError(format!(
                "invoke: argument {value} is not assignable to {param_type}"
            )));
        }
    }

    // For non-static methods, pop objectref
    if !is_static {
        let objectref = frame.pop()?;

        if method_name != "<init>" {
            // Regular method call
            let expected = VerificationType::Object(Arc::from(class_name));
            if !objectref.is_null() && !objectref.is_assignable_to(&expected, context)? {
                return Err(VerifyError::VerifyError(format!(
                    "invoke: {objectref} is not assignable to {class_name}"
                )));
            }
        }
        // For <init>, objectref handling is done by the caller (handle_invokespecial)
    }

    // Push return type
    if let Some(ret) = return_type {
        let ret_type = VerificationType::from_field_type(&ret);
        if ret_type.is_category2() {
            frame.push(ret_type.clone())?;
            frame.push(VerificationType::Top)?;
        } else {
            frame.push(ret_type.clone())?;
        }
        Ok(Some(ret_type))
    } else {
        Ok(None)
    }
}

/// Handles `invokespecial` including constructor calls.
///
/// # Errors
///
/// Returns an error if the argument types are incorrect.
pub fn handle_invokespecial<C: VerificationContext>(
    frame: &mut Frame,
    class_name: &str,
    method_name: &str,
    descriptor: &str,
    context: &C,
) -> Result<()> {
    let (params, return_type) = FieldType::parse_method_descriptor(descriptor)
        .map_err(|e| VerifyError::ClassFormatError(e.to_string()))?;

    // Pop arguments in reverse order
    for param in params.iter().rev() {
        let param_type = VerificationType::from_field_type(param);
        let value = if param_type.is_category2() {
            frame.pop_category2()?
        } else {
            frame.pop()?
        };

        if !value.is_assignable_to(&param_type, context)? {
            return Err(VerifyError::VerifyError(format!(
                "invokespecial: argument {value} is not assignable to {param_type}"
            )));
        }
    }

    // Pop objectref
    let objectref = frame.pop()?;

    if method_name == "<init>" {
        // Constructor call - initialize the object
        match &objectref {
            VerificationType::Uninitialized(offset) => {
                let initialized = VerificationType::Object(Arc::from(class_name));
                frame.initialize_object(&VerificationType::Uninitialized(*offset), &initialized);
            }
            VerificationType::UninitializedThis => {
                let initialized = VerificationType::Object(Arc::from(class_name));
                frame.initialize_object(&VerificationType::UninitializedThis, &initialized);
            }
            _ => {
                return Err(VerifyError::VerifyError(format!(
                    "invokespecial <init>: expected uninitialized object, got {objectref}"
                )));
            }
        }
    } else {
        // Non-constructor special method
        let expected = VerificationType::Object(Arc::from(class_name));
        if !objectref.is_null() && !objectref.is_assignable_to(&expected, context)? {
            return Err(VerifyError::VerifyError(format!(
                "invokespecial: {objectref} is not assignable to {class_name}"
            )));
        }
    }

    // Push return type
    if let Some(ret) = return_type {
        let ret_type = VerificationType::from_field_type(&ret);
        if ret_type.is_category2() {
            frame.push(ret_type)?;
            frame.push(VerificationType::Top)?;
        } else {
            frame.push(ret_type)?;
        }
    }

    Ok(())
}

/// Handles `invokedynamic`.
///
/// # Errors
///
/// Returns an error if the argument types are incorrect.
///
/// # References
///
/// - [JVMS §6.5.invokedynamic](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.invokedynamic)
pub fn handle_invokedynamic<C: VerificationContext>(
    frame: &mut Frame,
    descriptor: &str,
    context: &C,
) -> Result<()> {
    let (params, return_type) = FieldType::parse_method_descriptor(descriptor)
        .map_err(|e| VerifyError::ClassFormatError(e.to_string()))?;

    // Pop arguments in reverse order
    for param in params.iter().rev() {
        let param_type = VerificationType::from_field_type(param);
        let value = if param_type.is_category2() {
            frame.pop_category2()?
        } else {
            frame.pop()?
        };

        if !value.is_assignable_to(&param_type, context)? {
            return Err(VerifyError::VerifyError(format!(
                "invokedynamic: argument {value} is not assignable to {param_type}"
            )));
        }
    }

    // Push return type
    if let Some(ret) = return_type {
        let ret_type = VerificationType::from_field_type(&ret);
        if ret_type.is_category2() {
            frame.push(ret_type)?;
            frame.push(VerificationType::Top)?;
        } else {
            frame.push(ret_type)?;
        }
    }

    Ok(())
}

/// Handles `checkcast` - check object type.
///
/// Stack: ..., objectref → ..., objectref
///
/// # Errors
///
/// Returns an error if the operand is not a reference type.
///
/// # References
///
/// - [JVMS §6.5.checkcast](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.checkcast)
pub fn handle_checkcast(frame: &mut Frame, class_name: &str) -> Result<()> {
    let objectref = frame.pop()?;

    // Must be a reference type
    if !objectref.is_reference() {
        return Err(VerifyError::VerifyError(format!(
            "checkcast: expected reference, got {objectref}"
        )));
    }

    // Push the cast type (the reference is now known to be this type or null)
    let cast_type = if class_name.starts_with('[') {
        parse_type_descriptor(class_name)?
    } else {
        VerificationType::Object(Arc::from(class_name))
    };

    frame.push(cast_type)
}

/// Handles `instanceof` - check object type.
///
/// Stack: ..., objectref → ..., result (int)
///
/// # Errors
///
/// Returns an error if the operand is not a reference type.
///
/// # References
///
/// - [JVMS §6.5.instanceof](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.instanceof)
pub fn handle_instanceof(frame: &mut Frame) -> Result<()> {
    let objectref = frame.pop()?;

    if !objectref.is_reference() {
        return Err(VerifyError::VerifyError(format!(
            "instanceof: expected reference, got {objectref}"
        )));
    }

    // Result is int (0 or 1)
    frame.push(VerificationType::Integer)
}

/// Handles `arraylength` - get array length.
///
/// Stack: ..., arrayref → ..., length (int)
///
/// # Errors
///
/// Returns an error if the operand is not an array type.
///
/// # References
///
/// - [JVMS §6.5.arraylength](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.arraylength)
pub fn handle_arraylength(frame: &mut Frame) -> Result<()> {
    let arrayref = frame.pop()?;

    // Must be an array type or null
    if !matches!(
        arrayref,
        VerificationType::Array(_) | VerificationType::Null
    ) {
        // Also accept Object that looks like an array (legacy representation)
        if let VerificationType::Object(name) = &arrayref {
            if !name.starts_with('[') {
                return Err(VerifyError::VerifyError(format!(
                    "arraylength: expected array, got {arrayref}"
                )));
            }
        } else {
            return Err(VerifyError::VerifyError(format!(
                "arraylength: expected array, got {arrayref}"
            )));
        }
    }

    frame.push(VerificationType::Integer)
}

/// Parses a type descriptor into a `VerificationType`.
fn parse_type_descriptor(descriptor: &str) -> Result<VerificationType> {
    let field_type =
        FieldType::parse(descriptor).map_err(|e| VerifyError::ClassFormatError(e.to_string()))?;
    Ok(VerificationType::from_field_type(&field_type))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Version;
    use crate::constant::Constant;
    use crate::constant_pool::ConstantPool;
    use crate::verifiers::bytecode::handlers::test_utils::{MockContext, StrictMockContext};

    // Helper function to create a class file with a complete constant pool for testing
    fn create_test_class_file_with_refs() -> ClassFile {
        let mut constant_pool = ConstantPool::default();

        // Index 1: Utf8 "TestClass"
        constant_pool
            .add(Constant::Utf8("TestClass".to_string()))
            .unwrap();
        // Index 2: Class(1) - TestClass
        constant_pool.add(Constant::Class(1)).unwrap();

        // Index 3: Utf8 "testField"
        constant_pool
            .add(Constant::Utf8("testField".to_string()))
            .unwrap();
        // Index 4: Utf8 "I" (int descriptor)
        constant_pool.add(Constant::Utf8("I".to_string())).unwrap();
        // Index 5: NameAndType(3, 4) - testField:I
        constant_pool
            .add(Constant::NameAndType {
                name_index: 3,
                descriptor_index: 4,
            })
            .unwrap();
        // Index 6: Fieldref(2, 5) - TestClass.testField:I
        constant_pool
            .add(Constant::FieldRef {
                class_index: 2,
                name_and_type_index: 5,
            })
            .unwrap();

        // Index 7: Utf8 "testMethod"
        constant_pool
            .add(Constant::Utf8("testMethod".to_string()))
            .unwrap();
        // Index 8: Utf8 "(I)V" (method descriptor)
        constant_pool
            .add(Constant::Utf8("(I)V".to_string()))
            .unwrap();
        // Index 9: NameAndType(7, 8) - testMethod:(I)V
        constant_pool
            .add(Constant::NameAndType {
                name_index: 7,
                descriptor_index: 8,
            })
            .unwrap();
        // Index 10: Methodref(2, 9) - TestClass.testMethod:(I)V
        constant_pool
            .add(Constant::MethodRef {
                class_index: 2,
                name_and_type_index: 9,
            })
            .unwrap();

        // Index 11: Utf8 "InterfaceClass"
        constant_pool
            .add(Constant::Utf8("InterfaceClass".to_string()))
            .unwrap();
        // Index 12: Class(11) - InterfaceClass
        constant_pool.add(Constant::Class(11)).unwrap();
        // Index 13: InterfaceMethodref(12, 9) - InterfaceClass.testMethod:(I)V
        constant_pool
            .add(Constant::InterfaceMethodRef {
                class_index: 12,
                name_and_type_index: 9,
            })
            .unwrap();

        // Index 14: InvokeDynamic(0, 9) - bootstrap method 0, testMethod:(I)V
        constant_pool
            .add(Constant::InvokeDynamic {
                bootstrap_method_attr_index: 0,
                name_and_type_index: 9,
            })
            .unwrap();

        ClassFile {
            version: Version::Java8 { minor: 0 },
            constant_pool,
            access_flags: crate::ClassAccessFlags::PUBLIC,
            this_class: 2,
            super_class: 0,
            interfaces: vec![],
            fields: vec![],
            methods: vec![],
            attributes: vec![],
        }
    }

    #[test]
    fn test_constant_pool_resolver_new() {
        let class_file = create_test_class_file_with_refs();
        let resolver = ConstantPoolResolver::new(&class_file);
        assert_eq!(resolver.class_file, &class_file);
    }

    #[test]
    fn test_resolve_field_ref_success() {
        let class_file = create_test_class_file_with_refs();
        let resolver = ConstantPoolResolver::new(&class_file);

        let (class_name, field_name, descriptor) = resolver.resolve_field_ref(6).unwrap();

        assert_eq!(class_name, "TestClass");
        assert_eq!(field_name, "testField");
        assert_eq!(descriptor, "I");
    }

    #[test]
    fn test_resolve_field_ref_invalid_index_fails() {
        let class_file = create_test_class_file_with_refs();
        let resolver = ConstantPoolResolver::new(&class_file);

        let result = resolver.resolve_field_ref(999);
        assert!(result.is_err());
    }

    #[test]
    fn test_resolve_field_ref_wrong_constant_type_fails() {
        let class_file = create_test_class_file_with_refs();
        let resolver = ConstantPoolResolver::new(&class_file);

        // Index 10 is a Methodref, not a Fieldref
        let result = resolver.resolve_field_ref(10);
        assert!(result.is_err());
    }

    #[test]
    fn test_resolve_method_ref_success() {
        let class_file = create_test_class_file_with_refs();
        let resolver = ConstantPoolResolver::new(&class_file);

        let (class_name, method_name, descriptor) = resolver.resolve_method_ref(10).unwrap();

        assert_eq!(class_name, "TestClass");
        assert_eq!(method_name, "testMethod");
        assert_eq!(descriptor, "(I)V");
    }

    #[test]
    fn test_resolve_method_ref_interface_method_success() {
        let class_file = create_test_class_file_with_refs();
        let resolver = ConstantPoolResolver::new(&class_file);

        let (class_name, method_name, descriptor) = resolver.resolve_method_ref(13).unwrap();

        assert_eq!(class_name, "InterfaceClass");
        assert_eq!(method_name, "testMethod");
        assert_eq!(descriptor, "(I)V");
    }

    #[test]
    fn test_resolve_method_ref_invalid_index_fails() {
        let class_file = create_test_class_file_with_refs();
        let resolver = ConstantPoolResolver::new(&class_file);

        let result = resolver.resolve_method_ref(999);
        assert!(result.is_err());
    }

    #[test]
    fn test_resolve_method_ref_wrong_constant_type_fails() {
        let class_file = create_test_class_file_with_refs();
        let resolver = ConstantPoolResolver::new(&class_file);

        // Index 6 is a Fieldref, not a Methodref
        let result = resolver.resolve_method_ref(6);
        assert!(result.is_err());
    }

    #[test]
    fn test_resolve_class_success() {
        let class_file = create_test_class_file_with_refs();
        let resolver = ConstantPoolResolver::new(&class_file);

        let class_name = resolver.resolve_class(2).unwrap();
        assert_eq!(class_name, "TestClass");
    }

    #[test]
    fn test_resolve_class_interface_success() {
        let class_file = create_test_class_file_with_refs();
        let resolver = ConstantPoolResolver::new(&class_file);

        let class_name = resolver.resolve_class(12).unwrap();
        assert_eq!(class_name, "InterfaceClass");
    }

    #[test]
    fn test_resolve_class_invalid_index_fails() {
        let class_file = create_test_class_file_with_refs();
        let resolver = ConstantPoolResolver::new(&class_file);

        let result = resolver.resolve_class(999);
        assert!(result.is_err());
    }

    #[test]
    fn test_resolve_class_wrong_constant_type_fails() {
        let class_file = create_test_class_file_with_refs();
        let resolver = ConstantPoolResolver::new(&class_file);

        // Index 1 is a Utf8, not a Class
        let result = resolver.resolve_class(1);
        assert!(result.is_err());
    }

    #[test]
    fn test_resolve_invoke_dynamic_success() {
        let class_file = create_test_class_file_with_refs();
        let resolver = ConstantPoolResolver::new(&class_file);

        let descriptor = resolver.resolve_invoke_dynamic(14).unwrap();
        assert_eq!(descriptor, "(I)V");
    }

    #[test]
    fn test_resolve_invoke_dynamic_invalid_index_fails() {
        let class_file = create_test_class_file_with_refs();
        let resolver = ConstantPoolResolver::new(&class_file);

        let result = resolver.resolve_invoke_dynamic(999);
        assert!(result.is_err());
    }

    #[test]
    fn test_resolve_invoke_dynamic_wrong_constant_type_fails() {
        let class_file = create_test_class_file_with_refs();
        let resolver = ConstantPoolResolver::new(&class_file);

        // Index 10 is a Methodref, not an InvokeDynamic
        let result = resolver.resolve_invoke_dynamic(10);
        assert!(result.is_err());
    }

    #[test]
    fn test_handle_new_success() {
        let mut frame = Frame::new(5, 10);

        handle_new(&mut frame, 0, "java/lang/Object").unwrap();

        assert_eq!(*frame.peek().unwrap(), VerificationType::Uninitialized(0));
    }

    #[test]
    fn test_handle_new_with_offset() {
        let mut frame = Frame::new(5, 10);

        handle_new(&mut frame, 42, "java/lang/String").unwrap();

        assert_eq!(*frame.peek().unwrap(), VerificationType::Uninitialized(42));
    }

    #[test]
    fn test_handle_newarray_int_success() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();

        handle_newarray(&mut frame, &ArrayType::Int).unwrap();

        assert_eq!(
            *frame.peek().unwrap(),
            VerificationType::Array(Box::new(VerificationType::Integer))
        );
    }

    #[test]
    fn test_handle_newarray_long_success() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();

        handle_newarray(&mut frame, &ArrayType::Long).unwrap();

        assert_eq!(
            *frame.peek().unwrap(),
            VerificationType::Array(Box::new(VerificationType::Long))
        );
    }

    #[test]
    fn test_handle_newarray_wrong_count_type_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Float).unwrap();

        let result = handle_newarray(&mut frame, &ArrayType::Int);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("expected int count")
        );
    }

    #[test]
    fn test_handle_anewarray_success() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();

        handle_anewarray(&mut frame, "java/lang/String").unwrap();

        match frame.peek().unwrap() {
            VerificationType::Array(component) => {
                assert_eq!(
                    **component,
                    VerificationType::Object(Arc::from("java/lang/String"))
                );
            }
            _ => panic!("Expected array type"),
        }
    }

    #[test]
    fn test_handle_anewarray_wrong_count_type_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Float).unwrap();

        let result = handle_anewarray(&mut frame, "java/lang/String");
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("expected int count")
        );
    }

    #[test]
    fn test_handle_multianewarray_success() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();
        frame.push(VerificationType::Integer).unwrap();

        handle_multianewarray(&mut frame, "[[I", 2).unwrap();

        // Should have an array on the stack
        assert!(matches!(frame.peek().unwrap(), VerificationType::Array(_)));
    }

    #[test]
    fn test_handle_multianewarray_wrong_count_type_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();
        frame.push(VerificationType::Float).unwrap();

        let result = handle_multianewarray(&mut frame, "[[I", 2);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("expected int count")
        );
    }

    #[test]
    fn test_handle_getstatic_int() {
        let mut frame = Frame::new(5, 10);

        handle_getstatic(&mut frame, "I").unwrap();
        assert_eq!(*frame.peek().unwrap(), VerificationType::Integer);
    }

    #[test]
    fn test_handle_getstatic_long() {
        let mut frame = Frame::new(5, 10);

        handle_getstatic(&mut frame, "J").unwrap();
        assert_eq!(frame.stack_depth(), 2); // Long takes 2 slots
    }

    #[test]
    fn test_handle_getstatic_object() {
        let mut frame = Frame::new(5, 10);

        handle_getstatic(&mut frame, "Ljava/lang/String;").unwrap();
        assert_eq!(
            *frame.peek().unwrap(),
            VerificationType::Object(Arc::from("java/lang/String"))
        );
    }

    #[test]
    fn test_handle_putstatic_int_success() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();

        handle_putstatic(&mut frame, "I", &ctx).unwrap();
        assert!(frame.is_stack_empty());
    }

    #[test]
    fn test_handle_putstatic_wrong_type_fails() {
        let ctx = StrictMockContext;
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Float).unwrap();

        let result = handle_putstatic(&mut frame, "I", &ctx);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not assignable"));
    }

    #[test]
    fn test_handle_getfield_success() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::java_lang_object()).unwrap();

        handle_getfield(&mut frame, "java/lang/Object", "I", &ctx).unwrap();
        assert_eq!(*frame.peek().unwrap(), VerificationType::Integer);
    }

    #[test]
    fn test_handle_getfield_null_success() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Null).unwrap();

        handle_getfield(&mut frame, "java/lang/Object", "I", &ctx).unwrap();
        assert_eq!(*frame.peek().unwrap(), VerificationType::Integer);
    }

    #[test]
    fn test_handle_getfield_wrong_object_type_fails() {
        let ctx = StrictMockContext;
        let mut frame = Frame::new(5, 10);
        frame
            .push(VerificationType::Object(Arc::from("some/Other")))
            .unwrap();

        let result = handle_getfield(&mut frame, "java/lang/String", "I", &ctx);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not assignable"));
    }

    #[test]
    fn test_handle_putfield_success() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::java_lang_object()).unwrap();
        frame.push(VerificationType::Integer).unwrap();

        handle_putfield(&mut frame, "java/lang/Object", "I", &ctx).unwrap();
        assert!(frame.is_stack_empty());
    }

    #[test]
    fn test_handle_putfield_wrong_value_type_fails() {
        let ctx = StrictMockContext;
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::java_lang_object()).unwrap();
        frame.push(VerificationType::Float).unwrap();

        let result = handle_putfield(&mut frame, "java/lang/Object", "I", &ctx);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not assignable"));
    }

    #[test]
    fn test_handle_checkcast_success() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::java_lang_object()).unwrap();

        handle_checkcast(&mut frame, "java/lang/String").unwrap();

        assert_eq!(
            *frame.peek().unwrap(),
            VerificationType::Object(Arc::from("java/lang/String"))
        );
    }

    #[test]
    fn test_handle_checkcast_null_success() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Null).unwrap();

        handle_checkcast(&mut frame, "java/lang/String").unwrap();

        // After checkcast, the type is known to be the cast type
        assert_eq!(
            *frame.peek().unwrap(),
            VerificationType::Object(Arc::from("java/lang/String"))
        );
    }

    #[test]
    fn test_handle_checkcast_non_reference_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();

        let result = handle_checkcast(&mut frame, "java/lang/String");
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("expected reference")
        );
    }

    #[test]
    fn test_handle_instanceof_success() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::java_lang_object()).unwrap();

        handle_instanceof(&mut frame).unwrap();

        assert_eq!(*frame.peek().unwrap(), VerificationType::Integer);
    }

    #[test]
    fn test_handle_instanceof_null_success() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Null).unwrap();

        handle_instanceof(&mut frame).unwrap();

        assert_eq!(*frame.peek().unwrap(), VerificationType::Integer);
    }

    #[test]
    fn test_handle_instanceof_non_reference_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();

        let result = handle_instanceof(&mut frame);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("expected reference")
        );
    }

    #[test]
    fn test_handle_arraylength_success() {
        let mut frame = Frame::new(5, 10);
        frame
            .push(VerificationType::Array(Box::new(VerificationType::Integer)))
            .unwrap();

        handle_arraylength(&mut frame).unwrap();

        assert_eq!(*frame.peek().unwrap(), VerificationType::Integer);
    }

    #[test]
    fn test_handle_arraylength_null_success() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Null).unwrap();

        handle_arraylength(&mut frame).unwrap();

        assert_eq!(*frame.peek().unwrap(), VerificationType::Integer);
    }

    #[test]
    fn test_handle_arraylength_non_array_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::java_lang_object()).unwrap();

        let result = handle_arraylength(&mut frame);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("expected array"));
    }

    #[test]
    fn test_parse_type_descriptor_int() {
        assert_eq!(
            parse_type_descriptor("I").unwrap(),
            VerificationType::Integer
        );
    }

    #[test]
    fn test_parse_type_descriptor_long() {
        assert_eq!(parse_type_descriptor("J").unwrap(), VerificationType::Long);
    }

    #[test]
    fn test_parse_type_descriptor_float() {
        assert_eq!(parse_type_descriptor("F").unwrap(), VerificationType::Float);
    }

    #[test]
    fn test_parse_type_descriptor_double() {
        assert_eq!(
            parse_type_descriptor("D").unwrap(),
            VerificationType::Double
        );
    }

    #[test]
    fn test_parse_type_descriptor_object() {
        assert_eq!(
            parse_type_descriptor("Ljava/lang/String;").unwrap(),
            VerificationType::Object(Arc::from("java/lang/String"))
        );
    }

    #[test]
    fn test_parse_type_descriptor_array() {
        assert_eq!(
            parse_type_descriptor("[I").unwrap(),
            VerificationType::Array(Box::new(VerificationType::Integer))
        );
    }

    #[test]
    fn test_parse_type_descriptor_nested_array() {
        assert_eq!(
            parse_type_descriptor("[[I").unwrap(),
            VerificationType::Array(Box::new(VerificationType::Array(Box::new(
                VerificationType::Integer
            ))))
        );
    }

    #[test]
    fn test_handle_invoke_static_no_args_void_return() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        let result = handle_invoke(&mut frame, "Test", "test", "()V", true, &ctx).unwrap();
        assert!(result.is_none());
        assert!(frame.is_stack_empty());
    }

    #[test]
    fn test_handle_invoke_static_with_int_arg() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();

        let result = handle_invoke(&mut frame, "Test", "test", "(I)V", true, &ctx).unwrap();
        assert!(result.is_none());
        assert!(frame.is_stack_empty());
    }

    #[test]
    fn test_handle_invoke_static_with_return_value() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        let result = handle_invoke(&mut frame, "Test", "test", "()I", true, &ctx).unwrap();
        assert!(result.is_some());
        assert_eq!(*frame.peek().unwrap(), VerificationType::Integer);
    }

    #[test]
    fn test_handle_invoke_virtual_success() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::java_lang_object()).unwrap();

        let result = handle_invoke(
            &mut frame,
            "java/lang/Object",
            "toString",
            "()Ljava/lang/String;",
            false,
            &ctx,
        )
        .unwrap();
        assert!(result.is_some());
    }

    #[test]
    fn test_handle_invoke_wrong_arg_type_fails() {
        let ctx = StrictMockContext;
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Float).unwrap();

        let result = handle_invoke(&mut frame, "Test", "test", "(I)V", true, &ctx);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not assignable"));
    }

    #[test]
    fn test_handle_invoke_wrong_objectref_type_fails() {
        let ctx = StrictMockContext;
        let mut frame = Frame::new(5, 10);
        frame
            .push(VerificationType::Object(Arc::from("other/Class")))
            .unwrap();

        let result = handle_invoke(
            &mut frame,
            "java/lang/String",
            "toString",
            "()Ljava/lang/String;",
            false,
            &ctx,
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not assignable"));
    }

    #[test]
    fn test_handle_invoke_with_long_arg() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Long).unwrap();

        let result = handle_invoke(&mut frame, "Test", "test", "(J)V", true, &ctx).unwrap();
        assert!(result.is_none());
        assert!(frame.is_stack_empty());
    }

    #[test]
    fn test_handle_invoke_with_long_return() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        let result = handle_invoke(&mut frame, "Test", "test", "()J", true, &ctx).unwrap();
        assert!(result.is_some());
        assert_eq!(frame.stack_depth(), 2); // Long takes 2 slots
    }

    #[test]
    fn test_handle_invokespecial_constructor_uninitialized() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Uninitialized(0)).unwrap();

        handle_invokespecial(&mut frame, "java/lang/Object", "<init>", "()V", &ctx).unwrap();
        // After constructor, the object should be initialized
        assert!(frame.is_stack_empty());
    }

    #[test]
    fn test_handle_invokespecial_constructor_uninitialized_this() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::UninitializedThis).unwrap();

        handle_invokespecial(&mut frame, "java/lang/Object", "<init>", "()V", &ctx).unwrap();
        assert!(frame.is_stack_empty());
    }

    #[test]
    fn test_handle_invokespecial_constructor_with_args() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Uninitialized(0)).unwrap();
        frame.push(VerificationType::Integer).unwrap();

        handle_invokespecial(&mut frame, "Test", "<init>", "(I)V", &ctx).unwrap();
        assert!(frame.is_stack_empty());
    }

    #[test]
    fn test_handle_invokespecial_constructor_wrong_objectref_fails() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::java_lang_object()).unwrap();

        let result = handle_invokespecial(&mut frame, "java/lang/Object", "<init>", "()V", &ctx);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("expected uninitialized")
        );
    }

    #[test]
    fn test_handle_invokespecial_non_constructor() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::java_lang_object()).unwrap();

        handle_invokespecial(&mut frame, "java/lang/Object", "privateMethod", "()V", &ctx).unwrap();
        assert!(frame.is_stack_empty());
    }

    #[test]
    fn test_handle_invokespecial_non_constructor_with_return() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::java_lang_object()).unwrap();

        handle_invokespecial(&mut frame, "java/lang/Object", "privateMethod", "()I", &ctx).unwrap();
        assert_eq!(*frame.peek().unwrap(), VerificationType::Integer);
    }

    #[test]
    fn test_handle_invokespecial_wrong_arg_type_fails() {
        let ctx = StrictMockContext;
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Uninitialized(0)).unwrap();
        frame.push(VerificationType::Float).unwrap();

        let result = handle_invokespecial(&mut frame, "Test", "<init>", "(I)V", &ctx);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not assignable"));
    }

    #[test]
    fn test_handle_invokespecial_non_constructor_wrong_objectref_fails() {
        let ctx = StrictMockContext;
        let mut frame = Frame::new(5, 10);
        frame
            .push(VerificationType::Object(Arc::from("other/Class")))
            .unwrap();

        let result =
            handle_invokespecial(&mut frame, "java/lang/String", "privateMethod", "()V", &ctx);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not assignable"));
    }

    #[test]
    fn test_handle_invokedynamic_no_args_void_return() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        handle_invokedynamic(&mut frame, "()V", &ctx).unwrap();
        assert!(frame.is_stack_empty());
    }

    #[test]
    fn test_handle_invokedynamic_with_args() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();
        frame.push(VerificationType::Integer).unwrap();

        handle_invokedynamic(&mut frame, "(II)V", &ctx).unwrap();
        assert!(frame.is_stack_empty());
    }

    #[test]
    fn test_handle_invokedynamic_with_return() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();

        handle_invokedynamic(&mut frame, "(I)I", &ctx).unwrap();
        assert_eq!(*frame.peek().unwrap(), VerificationType::Integer);
    }

    #[test]
    fn test_handle_invokedynamic_with_long_return() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        handle_invokedynamic(&mut frame, "()J", &ctx).unwrap();
        assert_eq!(frame.stack_depth(), 2); // Long takes 2 slots
    }

    #[test]
    fn test_handle_invokedynamic_wrong_arg_type_fails() {
        let ctx = StrictMockContext;
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Float).unwrap();

        let result = handle_invokedynamic(&mut frame, "(I)V", &ctx);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not assignable"));
    }

    #[test]
    fn test_handle_invokedynamic_with_object_arg() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::java_lang_string()).unwrap();

        handle_invokedynamic(&mut frame, "(Ljava/lang/String;)V", &ctx).unwrap();
        assert!(frame.is_stack_empty());
    }

    #[test]
    fn test_handle_newarray_boolean() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();

        handle_newarray(&mut frame, &ArrayType::Boolean).unwrap();
        assert!(matches!(frame.peek().unwrap(), VerificationType::Array(_)));
    }

    #[test]
    fn test_handle_newarray_byte() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();

        handle_newarray(&mut frame, &ArrayType::Byte).unwrap();
        assert!(matches!(frame.peek().unwrap(), VerificationType::Array(_)));
    }

    #[test]
    fn test_handle_newarray_char() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();

        handle_newarray(&mut frame, &ArrayType::Char).unwrap();
        assert!(matches!(frame.peek().unwrap(), VerificationType::Array(_)));
    }

    #[test]
    fn test_handle_newarray_short() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();

        handle_newarray(&mut frame, &ArrayType::Short).unwrap();
        assert!(matches!(frame.peek().unwrap(), VerificationType::Array(_)));
    }

    #[test]
    fn test_handle_newarray_float() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();

        handle_newarray(&mut frame, &ArrayType::Float).unwrap();
        assert_eq!(
            *frame.peek().unwrap(),
            VerificationType::Array(Box::new(VerificationType::Float))
        );
    }

    #[test]
    fn test_handle_newarray_double() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();

        handle_newarray(&mut frame, &ArrayType::Double).unwrap();
        assert_eq!(
            *frame.peek().unwrap(),
            VerificationType::Array(Box::new(VerificationType::Double))
        );
    }

    #[test]
    fn test_handle_anewarray_nested_array() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();

        handle_anewarray(&mut frame, "[I").unwrap();

        // Should be array of int arrays
        match frame.peek().unwrap() {
            VerificationType::Array(component) => {
                assert!(matches!(**component, VerificationType::Array(_)));
            }
            _ => panic!("Expected array type"),
        }
    }

    #[test]
    fn test_handle_arraylength_legacy_array_representation() {
        let mut frame = Frame::new(5, 10);
        // Legacy representation: Object with name starting with '['
        frame
            .push(VerificationType::Object(Arc::from("[I")))
            .unwrap();

        handle_arraylength(&mut frame).unwrap();
        assert_eq!(*frame.peek().unwrap(), VerificationType::Integer);
    }

    #[test]
    fn test_handle_arraylength_primitive_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();

        let result = handle_arraylength(&mut frame);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("expected array"));
    }

    #[test]
    fn test_parse_type_descriptor_invalid_fails() {
        let result = parse_type_descriptor("X");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_type_descriptor_byte() {
        assert_eq!(
            parse_type_descriptor("B").unwrap(),
            VerificationType::Integer
        );
    }

    #[test]
    fn test_parse_type_descriptor_char() {
        assert_eq!(
            parse_type_descriptor("C").unwrap(),
            VerificationType::Integer
        );
    }

    #[test]
    fn test_parse_type_descriptor_short() {
        assert_eq!(
            parse_type_descriptor("S").unwrap(),
            VerificationType::Integer
        );
    }

    #[test]
    fn test_parse_type_descriptor_boolean() {
        assert_eq!(
            parse_type_descriptor("Z").unwrap(),
            VerificationType::Integer
        );
    }
}
