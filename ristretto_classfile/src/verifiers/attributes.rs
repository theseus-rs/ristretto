use crate::attributes::{
    Annotation, AnnotationElement, Attribute, BootstrapMethod, ExceptionTableEntry, Exports,
    InnerClass, Instruction, LineNumber, LocalVariableTable, LocalVariableTypeTable,
    MethodParameter, Opens, ParameterAnnotation, Provides, Record, Requires, TypeAnnotation,
};
use crate::class_file::ClassFile;
use crate::constant::Constant;
use crate::field::Field;
use crate::method::Method;
use crate::verifiers::code;
use crate::verifiers::error::Result;
use crate::verifiers::error::VerifyError::{
    InvalidConstantPoolIndex, InvalidConstantPoolIndexType, VerificationError,
};

/// The context in which attributes are being verified.
#[derive(Clone, Copy, Debug)]
pub(crate) enum AttributeContext<'a> {
    Class,
    Field(&'a Field),
    Method(&'a Method),
    Code(&'a Method, u16),
    RecordComponent,
}

/// Verify the attributes.
///
/// # Errors
/// Returns `VerificationError` if the attributes are invalid.
#[expect(clippy::too_many_lines)]
pub(crate) fn verify(
    class_file: &ClassFile,
    attributes: &[Attribute],
    context: AttributeContext,
) -> Result<()> {
    for attribute in attributes {
        match attribute {
            Attribute::ConstantValue {
                constant_value_index,
                ..
            } => verify_constant_value(class_file, context, *constant_value_index)?,
            Attribute::Code {
                max_stack,
                max_locals,
                code,
                attributes,
                exception_table,
                ..
            } => verify_code(
                class_file,
                context,
                *max_stack,
                *max_locals,
                code,
                exception_table,
                attributes,
            )?,
            Attribute::StackMapTable { .. } => verify_stack_map_table(context)?,
            Attribute::Exceptions {
                exception_indexes, ..
            } => verify_exceptions(class_file, context, exception_indexes)?,
            Attribute::InnerClasses { classes, .. } => {
                verify_inner_classes(class_file, context, classes)?;
            }
            Attribute::EnclosingMethod {
                class_index,
                method_index,
                ..
            } => verify_enclosing_method(class_file, context, *class_index, *method_index)?,
            Attribute::Synthetic { .. } => verify_synthetic(context)?,
            Attribute::Signature {
                signature_index, ..
            } => verify_signature(class_file, context, *signature_index)?,
            Attribute::SourceFile {
                source_file_index, ..
            } => verify_source_file(class_file, context, *source_file_index)?,
            Attribute::SourceDebugExtension { .. } => verify_source_debug_extension(context)?,
            Attribute::LineNumberTable { line_numbers, .. } => {
                verify_line_number_table(context, line_numbers)?;
            }
            Attribute::LocalVariableTable { variables, .. } => {
                verify_local_variable_table(class_file, context, variables)?;
            }
            Attribute::LocalVariableTypeTable { variable_types, .. } => {
                verify_local_variable_type_table(class_file, context, variable_types)?;
            }
            Attribute::Deprecated { .. } => verify_deprecated(context)?,
            Attribute::RuntimeVisibleAnnotations { annotations, .. }
            | Attribute::RuntimeInvisibleAnnotations { annotations, .. } => {
                verify_annotations(class_file, annotations)?;
            }
            Attribute::RuntimeVisibleParameterAnnotations {
                parameter_annotations,
                ..
            }
            | Attribute::RuntimeInvisibleParameterAnnotations {
                parameter_annotations,
                ..
            } => verify_parameter_annotations(class_file, context, parameter_annotations)?,
            Attribute::RuntimeVisibleTypeAnnotations {
                type_annotations, ..
            }
            | Attribute::RuntimeInvisibleTypeAnnotations {
                type_annotations, ..
            } => verify_type_annotations(class_file, context, type_annotations)?,
            Attribute::AnnotationDefault { element, .. } => {
                verify_annotation_default(class_file, context, element)?;
            }
            Attribute::BootstrapMethods { methods, .. } => {
                verify_bootstrap_methods(class_file, context, methods)?;
            }
            Attribute::MethodParameters { parameters, .. } => {
                verify_method_parameters(class_file, context, parameters)?;
            }
            Attribute::Module {
                module_name_index,
                version_index,
                requires,
                exports,
                opens,
                uses,
                provides,
                ..
            } => verify_module(
                class_file,
                context,
                *module_name_index,
                *version_index,
                requires,
                exports,
                opens,
                uses,
                provides,
            )?,
            Attribute::ModulePackages {
                package_indexes, ..
            } => verify_module_packages(class_file, context, package_indexes)?,
            Attribute::ModuleMainClass {
                main_class_index, ..
            } => verify_module_main_class(class_file, context, *main_class_index)?,
            Attribute::NestHost {
                host_class_index, ..
            } => verify_nest_host(class_file, context, *host_class_index)?,
            Attribute::NestMembers { class_indexes, .. } => {
                verify_nest_members(class_file, context, class_indexes)?;
            }
            Attribute::Record { records, .. } => verify_record(class_file, context, records)?,
            Attribute::PermittedSubclasses { class_indexes, .. } => {
                verify_permitted_subclasses(class_file, context, class_indexes)?;
            }
            Attribute::Unknown { .. } => {}
        }
    }
    Ok(())
}

fn verify_constant_value(
    class_file: &ClassFile,
    context: AttributeContext,
    constant_value_index: u16,
) -> Result<()> {
    if !matches!(context, AttributeContext::Field(_)) {
        return Err(VerificationError {
            context: "ConstantValue Attribute".to_string(),
            message: "ConstantValue attribute only allowed in Field context".to_string(),
        });
    }
    match class_file.constant_pool.get(constant_value_index) {
        Some(
            Constant::Long(_)
            | Constant::Float(_)
            | Constant::Double(_)
            | Constant::Integer(_)
            | Constant::String(_),
        ) => Ok(()),
        Some(_) => Err(InvalidConstantPoolIndexType(constant_value_index)),
        None => Err(InvalidConstantPoolIndex(constant_value_index)),
    }
}

fn verify_code(
    class_file: &ClassFile,
    context: AttributeContext,
    max_stack: u16,
    max_locals: u16,
    code: &[Instruction],
    exception_table: &[ExceptionTableEntry],
    attributes: &[Attribute],
) -> Result<()> {
    let AttributeContext::Method(method) = context else {
        return Err(VerificationError {
            context: "Code Attribute".to_string(),
            message: "Code attribute only allowed in Method context".to_string(),
        });
    };
    // code::verify returns the code_length which is needed for context
    let code_length = code::verify(
        class_file,
        method,
        max_stack,
        max_locals,
        code,
        exception_table,
        attributes,
    )?;
    // Recursively verify attributes of Code
    verify(
        class_file,
        attributes,
        AttributeContext::Code(method, code_length),
    )
}

fn verify_stack_map_table(context: AttributeContext) -> Result<()> {
    if !matches!(context, AttributeContext::Code(..)) {
        return Err(VerificationError {
            context: "StackMapTable Attribute".to_string(),
            message: "StackMapTable attribute only allowed in Code context".to_string(),
        });
    }
    // Structural verification is handled in code::verify
    Ok(())
}

fn verify_exceptions(
    class_file: &ClassFile,
    context: AttributeContext,
    exception_indexes: &[u16],
) -> Result<()> {
    if !matches!(context, AttributeContext::Method(_)) {
        return Err(VerificationError {
            context: "Exceptions Attribute".to_string(),
            message: "Exceptions attribute only allowed in Method context".to_string(),
        });
    }
    for index in exception_indexes {
        match class_file.constant_pool.get(*index) {
            Some(Constant::Class(_)) => {}
            Some(_) => return Err(InvalidConstantPoolIndexType(*index)),
            None => return Err(InvalidConstantPoolIndex(*index)),
        }
    }
    Ok(())
}

fn verify_inner_classes(
    class_file: &ClassFile,
    context: AttributeContext,
    classes: &[InnerClass],
) -> Result<()> {
    if !matches!(context, AttributeContext::Class) {
        return Err(VerificationError {
            context: "InnerClasses Attribute".to_string(),
            message: "InnerClasses attribute only allowed in ClassFile context".to_string(),
        });
    }
    for class in classes {
        // class_info_index
        match class_file.constant_pool.get(class.class_info_index) {
            Some(Constant::Class(_)) => {}
            Some(_) => {
                return Err(InvalidConstantPoolIndexType(class.class_info_index));
            }
            None => return Err(InvalidConstantPoolIndex(class.class_info_index)),
        }
        // outer_class_info_index (can be 0)
        if class.outer_class_info_index != 0 {
            match class_file.constant_pool.get(class.outer_class_info_index) {
                Some(Constant::Class(_)) => {}
                Some(_) => {
                    return Err(InvalidConstantPoolIndexType(class.outer_class_info_index));
                }
                None => {
                    return Err(InvalidConstantPoolIndex(class.outer_class_info_index));
                }
            }
        }
        // inner_name_index (can be 0)
        if class.name_index != 0 {
            match class_file.constant_pool.get(class.name_index) {
                Some(Constant::Utf8(_)) => {}
                Some(_) => return Err(InvalidConstantPoolIndexType(class.name_index)),
                None => return Err(InvalidConstantPoolIndex(class.name_index)),
            }
        }
    }
    Ok(())
}

fn verify_enclosing_method(
    class_file: &ClassFile,
    context: AttributeContext,
    class_index: u16,
    method_index: u16,
) -> Result<()> {
    if !matches!(context, AttributeContext::Class) {
        return Err(VerificationError {
            context: "EnclosingMethod Attribute".to_string(),
            message: "EnclosingMethod attribute only allowed in ClassFile context".to_string(),
        });
    }
    match class_file.constant_pool.get(class_index) {
        Some(Constant::Class(_)) => {}
        Some(_) => return Err(InvalidConstantPoolIndexType(class_index)),
        None => return Err(InvalidConstantPoolIndex(class_index)),
    }
    if method_index != 0 {
        match class_file.constant_pool.get(method_index) {
            Some(Constant::NameAndType { .. }) => {}
            Some(_) => return Err(InvalidConstantPoolIndexType(method_index)),
            None => return Err(InvalidConstantPoolIndex(method_index)),
        }
    }
    Ok(())
}

fn verify_synthetic(context: AttributeContext) -> Result<()> {
    if matches!(context, AttributeContext::Code(..)) {
        return Err(VerificationError {
            context: "Synthetic Attribute".to_string(),
            message: "Synthetic attribute not allowed in Code context".to_string(),
        });
    }
    Ok(())
}

fn verify_signature(
    class_file: &ClassFile,
    context: AttributeContext,
    signature_index: u16,
) -> Result<()> {
    if matches!(context, AttributeContext::Code(..)) {
        return Err(VerificationError {
            context: "Signature Attribute".to_string(),
            message: "Signature attribute not allowed in Code context".to_string(),
        });
    }
    match class_file.constant_pool.get(signature_index) {
        Some(Constant::Utf8(_)) => {}
        Some(_) => return Err(InvalidConstantPoolIndexType(signature_index)),
        None => return Err(InvalidConstantPoolIndex(signature_index)),
    }
    Ok(())
}

fn verify_source_file(
    class_file: &ClassFile,
    context: AttributeContext,
    source_file_index: u16,
) -> Result<()> {
    if !matches!(context, AttributeContext::Class) {
        return Err(VerificationError {
            context: "SourceFile Attribute".to_string(),
            message: "SourceFile attribute only allowed in ClassFile context".to_string(),
        });
    }
    match class_file.constant_pool.get(source_file_index) {
        Some(Constant::Utf8(_)) => {}
        Some(_) => return Err(InvalidConstantPoolIndexType(source_file_index)),
        None => return Err(InvalidConstantPoolIndex(source_file_index)),
    }
    Ok(())
}

fn verify_source_debug_extension(context: AttributeContext) -> Result<()> {
    if !matches!(context, AttributeContext::Class) {
        return Err(VerificationError {
            context: "SourceDebugExtension Attribute".to_string(),
            message: "SourceDebugExtension attribute only allowed in ClassFile context".to_string(),
        });
    }
    Ok(())
}

fn verify_line_number_table(context: AttributeContext, line_numbers: &[LineNumber]) -> Result<()> {
    // LineNumberTable is typically inside Code attribute, but some compilers (e.g., javac 21+)
    // place it as a sibling attribute at the Method level. Both are accepted by the JVM.
    match context {
        AttributeContext::Code(_, code_length) => {
            // When inside Code, validate start_pc against code_length
            for line_number in line_numbers {
                if line_number.start_pc >= code_length {
                    return Err(VerificationError {
                        context: "LineNumberTable Attribute".to_string(),
                        message: format!(
                            "Invalid start_pc {} in LineNumberTable (code length {})",
                            line_number.start_pc, code_length
                        ),
                    });
                }
            }
        }
        AttributeContext::Method(_) => {
            // When at Method level (sibling of Code), we cannot validate start_pc
            // as we don't have code_length. The JVM accepts this placement.
        }
        _ => {
            return Err(VerificationError {
                context: "LineNumberTable Attribute".to_string(),
                message: "LineNumberTable attribute only allowed in Code or Method context"
                    .to_string(),
            });
        }
    }
    Ok(())
}

fn verify_local_variable_table(
    class_file: &ClassFile,
    context: AttributeContext,
    variables: &[LocalVariableTable],
) -> Result<()> {
    // LocalVariableTable is typically inside Code attribute, but some compilers
    // place it as a sibling attribute at the Method level. Both are accepted by the JVM.
    let code_length = match context {
        AttributeContext::Code(_, len) => Some(len),
        AttributeContext::Method(_) => None, // Cannot validate ranges without code_length
        _ => {
            return Err(VerificationError {
                context: "LocalVariableTable Attribute".to_string(),
                message: "LocalVariableTable attribute only allowed in Code or Method context"
                    .to_string(),
            });
        }
    };

    for local_variable in variables {
        // Only validate ranges if we have code_length (i.e., in Code context)
        if let Some(code_length) = code_length
            && u32::from(local_variable.start_pc) + u32::from(local_variable.length)
                > u32::from(code_length)
        {
            return Err(VerificationError {
                context: "LocalVariableTable Attribute".to_string(),
                message: format!(
                    "Invalid range start_pc {} + length {} in LocalVariableTable (code length {})",
                    local_variable.start_pc, local_variable.length, code_length
                ),
            });
        }
        // Verify name_index and descriptor_index
        match class_file.constant_pool.get(local_variable.name_index) {
            Some(Constant::Utf8(_)) => {}
            Some(_) => {
                return Err(InvalidConstantPoolIndexType(local_variable.name_index));
            }
            None => return Err(InvalidConstantPoolIndex(local_variable.name_index)),
        }
        match class_file
            .constant_pool
            .get(local_variable.descriptor_index)
        {
            Some(Constant::Utf8(_)) => {}
            Some(_) => {
                return Err(InvalidConstantPoolIndexType(
                    local_variable.descriptor_index,
                ));
            }
            None => {
                return Err(InvalidConstantPoolIndex(local_variable.descriptor_index));
            }
        }
    }
    Ok(())
}

fn verify_local_variable_type_table(
    class_file: &ClassFile,
    context: AttributeContext,
    variable_types: &[LocalVariableTypeTable],
) -> Result<()> {
    // LocalVariableTypeTable is typically inside Code attribute, but some compilers
    // place it as a sibling attribute at the Method level. Both are accepted by the JVM.
    let code_length = match context {
        AttributeContext::Code(_, len) => Some(len),
        AttributeContext::Method(_) => None, // Cannot validate ranges without code_length
        _ => {
            return Err(VerificationError {
                context: "LocalVariableTypeTable Attribute".to_string(),
                message: "LocalVariableTypeTable attribute only allowed in Code or Method context"
                    .to_string(),
            });
        }
    };

    for local_variable in variable_types {
        // Only validate ranges if we have code_length (i.e., in Code context)
        if let Some(code_length) = code_length
            && u32::from(local_variable.start_pc) + u32::from(local_variable.length)
                > u32::from(code_length)
        {
            return Err(VerificationError {
                context: "LocalVariableTypeTable Attribute".to_string(),
                message: format!(
                    "Invalid range start_pc {} + length {} in LocalVariableTypeTable (code length {})",
                    local_variable.start_pc, local_variable.length, code_length
                ),
            });
        }
        // Verify name_index and signature_index
        match class_file.constant_pool.get(local_variable.name_index) {
            Some(Constant::Utf8(_)) => {}
            Some(_) => {
                return Err(InvalidConstantPoolIndexType(local_variable.name_index));
            }
            None => return Err(InvalidConstantPoolIndex(local_variable.name_index)),
        }
        match class_file.constant_pool.get(local_variable.signature_index) {
            Some(Constant::Utf8(_)) => {}
            Some(_) => {
                return Err(InvalidConstantPoolIndexType(local_variable.signature_index));
            }
            None => {
                return Err(InvalidConstantPoolIndex(local_variable.signature_index));
            }
        }
    }
    Ok(())
}

fn verify_deprecated(context: AttributeContext) -> Result<()> {
    if matches!(context, AttributeContext::Code(..)) {
        return Err(VerificationError {
            context: "Deprecated Attribute".to_string(),
            message: "Deprecated attribute not allowed in Code context".to_string(),
        });
    }
    Ok(())
}

fn verify_annotations(class_file: &ClassFile, annotations: &[Annotation]) -> Result<()> {
    for annotation in annotations {
        verify_annotation(class_file, annotation)?;
    }
    Ok(())
}

fn verify_annotation(class_file: &ClassFile, annotation: &Annotation) -> Result<()> {
    match class_file.constant_pool.get(annotation.type_index) {
        Some(Constant::Utf8(_)) => {}
        Some(_) => return Err(InvalidConstantPoolIndexType(annotation.type_index)),
        None => return Err(InvalidConstantPoolIndex(annotation.type_index)),
    }
    for pair in &annotation.elements {
        match class_file.constant_pool.get(pair.name_index) {
            Some(Constant::Utf8(_)) => {}
            Some(_) => return Err(InvalidConstantPoolIndexType(pair.name_index)),
            None => return Err(InvalidConstantPoolIndex(pair.name_index)),
        }
        verify_annotation_element(class_file, &pair.value)?;
    }
    Ok(())
}

fn verify_annotation_element(class_file: &ClassFile, element: &AnnotationElement) -> Result<()> {
    match element {
        AnnotationElement::Byte { const_value_index }
        | AnnotationElement::Char { const_value_index }
        | AnnotationElement::Int { const_value_index }
        | AnnotationElement::Short { const_value_index }
        | AnnotationElement::Boolean { const_value_index } => {
            match class_file.constant_pool.get(*const_value_index) {
                Some(Constant::Integer(_)) => {}
                Some(_) => return Err(InvalidConstantPoolIndexType(*const_value_index)),
                None => return Err(InvalidConstantPoolIndex(*const_value_index)),
            }
        }
        AnnotationElement::Double { const_value_index } => {
            match class_file.constant_pool.get(*const_value_index) {
                Some(Constant::Double(_)) => {}
                Some(_) => return Err(InvalidConstantPoolIndexType(*const_value_index)),
                None => return Err(InvalidConstantPoolIndex(*const_value_index)),
            }
        }
        AnnotationElement::Float { const_value_index } => {
            match class_file.constant_pool.get(*const_value_index) {
                Some(Constant::Float(_)) => {}
                Some(_) => return Err(InvalidConstantPoolIndexType(*const_value_index)),
                None => return Err(InvalidConstantPoolIndex(*const_value_index)),
            }
        }
        AnnotationElement::Long { const_value_index } => {
            match class_file.constant_pool.get(*const_value_index) {
                Some(Constant::Long(_)) => {}
                Some(_) => return Err(InvalidConstantPoolIndexType(*const_value_index)),
                None => return Err(InvalidConstantPoolIndex(*const_value_index)),
            }
        }
        AnnotationElement::String { const_value_index } => {
            match class_file.constant_pool.get(*const_value_index) {
                Some(Constant::Utf8(_)) => {}
                Some(_) => return Err(InvalidConstantPoolIndexType(*const_value_index)),
                None => return Err(InvalidConstantPoolIndex(*const_value_index)),
            }
        }
        AnnotationElement::Enum {
            type_name_index,
            const_name_index,
        } => {
            match class_file.constant_pool.get(*type_name_index) {
                Some(Constant::Utf8(_)) => {}
                Some(_) => return Err(InvalidConstantPoolIndexType(*type_name_index)),
                None => return Err(InvalidConstantPoolIndex(*type_name_index)),
            }
            match class_file.constant_pool.get(*const_name_index) {
                Some(Constant::Utf8(_)) => {}
                Some(_) => return Err(InvalidConstantPoolIndexType(*const_name_index)),
                None => return Err(InvalidConstantPoolIndex(*const_name_index)),
            }
        }
        AnnotationElement::Class { class_info_index } => {
            match class_file.constant_pool.get(*class_info_index) {
                Some(Constant::Utf8(_)) => {}
                Some(_) => return Err(InvalidConstantPoolIndexType(*class_info_index)),
                None => return Err(InvalidConstantPoolIndex(*class_info_index)),
            }
        }
        AnnotationElement::Annotation { annotation } => {
            verify_annotation(class_file, annotation)?;
        }
        AnnotationElement::Array { values } => {
            for element in values {
                verify_annotation_element(class_file, element)?;
            }
        }
    }
    Ok(())
}

fn verify_parameter_annotations(
    class_file: &ClassFile,
    context: AttributeContext,
    parameter_annotations: &[ParameterAnnotation],
) -> Result<()> {
    if !matches!(context, AttributeContext::Method(_)) {
        return Err(VerificationError {
            context: "ParameterAnnotations Attribute".to_string(),
            message: "ParameterAnnotations attribute only allowed in Method context".to_string(),
        });
    }
    for parameter_annotation in parameter_annotations {
        verify_annotations(class_file, &parameter_annotation.annotations)?;
    }
    Ok(())
}

fn verify_type_annotations(
    class_file: &ClassFile,
    context: AttributeContext,
    type_annotations: &[TypeAnnotation],
) -> Result<()> {
    if matches!(context, AttributeContext::Code(..)) {
        // Type annotations in Code attribute target instructions, local variables, etc.
        // We should verify target_type here if we want to be strict.
    }
    for type_annotation in type_annotations {
        // Verify target_info based on target_type
        // For now, just verify the annotation part
        match class_file.constant_pool.get(type_annotation.type_index) {
            Some(Constant::Utf8(_)) => {}
            Some(_) => return Err(InvalidConstantPoolIndexType(type_annotation.type_index)),
            None => return Err(InvalidConstantPoolIndex(type_annotation.type_index)),
        }
        for pair in &type_annotation.elements {
            match class_file.constant_pool.get(pair.name_index) {
                Some(Constant::Utf8(_)) => {}
                Some(_) => return Err(InvalidConstantPoolIndexType(pair.name_index)),
                None => return Err(InvalidConstantPoolIndex(pair.name_index)),
            }
            verify_annotation_element(class_file, &pair.value)?;
        }
    }
    Ok(())
}

fn verify_annotation_default(
    class_file: &ClassFile,
    context: AttributeContext,
    element: &AnnotationElement,
) -> Result<()> {
    if !matches!(context, AttributeContext::Method(_)) {
        return Err(VerificationError {
            context: "AnnotationDefault Attribute".to_string(),
            message: "AnnotationDefault attribute only allowed in Method context".to_string(),
        });
    }
    verify_annotation_element(class_file, element)
}

fn verify_bootstrap_methods(
    class_file: &ClassFile,
    context: AttributeContext,
    methods: &[BootstrapMethod],
) -> Result<()> {
    if !matches!(context, AttributeContext::Class) {
        return Err(VerificationError {
            context: "BootstrapMethods Attribute".to_string(),
            message: "BootstrapMethods attribute only allowed in ClassFile context".to_string(),
        });
    }
    for method in methods {
        match class_file.constant_pool.get(method.bootstrap_method_ref) {
            Some(Constant::MethodHandle { .. }) => {}
            Some(_) => return Err(InvalidConstantPoolIndexType(method.bootstrap_method_ref)),
            None => return Err(InvalidConstantPoolIndex(method.bootstrap_method_ref)),
        }
        for argument in &method.arguments {
            match class_file.constant_pool.get(*argument) {
                Some(_) => {} // Can be various constants
                None => return Err(InvalidConstantPoolIndex(*argument)),
            }
        }
    }
    Ok(())
}

fn verify_method_parameters(
    class_file: &ClassFile,
    context: AttributeContext,
    parameters: &[MethodParameter],
) -> Result<()> {
    if !matches!(context, AttributeContext::Method(_)) {
        return Err(VerificationError {
            context: "MethodParameters Attribute".to_string(),
            message: "MethodParameters attribute only allowed in Method context".to_string(),
        });
    }
    for parameter in parameters {
        if parameter.name_index != 0 {
            match class_file.constant_pool.get(parameter.name_index) {
                Some(Constant::Utf8(_)) => {}
                Some(_) => return Err(InvalidConstantPoolIndexType(parameter.name_index)),
                None => return Err(InvalidConstantPoolIndex(parameter.name_index)),
            }
        }
    }
    Ok(())
}

#[expect(clippy::too_many_arguments)]
fn verify_module(
    class_file: &ClassFile,
    context: AttributeContext,
    module_name_index: u16,
    version_index: u16,
    requires: &[Requires],
    exports: &[Exports],
    opens: &[Opens],
    uses: &[u16],
    provides: &[Provides],
) -> Result<()> {
    if !matches!(context, AttributeContext::Class) {
        return Err(VerificationError {
            context: "Module Attribute".to_string(),
            message: "Module attribute only allowed in ClassFile context".to_string(),
        });
    }
    match class_file.constant_pool.get(module_name_index) {
        Some(Constant::Module(_)) => {}
        Some(_) => return Err(InvalidConstantPoolIndexType(module_name_index)),
        None => return Err(InvalidConstantPoolIndex(module_name_index)),
    }
    if version_index != 0 {
        match class_file.constant_pool.get(version_index) {
            Some(Constant::Utf8(_)) => {}
            Some(_) => return Err(InvalidConstantPoolIndexType(version_index)),
            None => return Err(InvalidConstantPoolIndex(version_index)),
        }
    }
    for require in requires {
        match class_file.constant_pool.get(require.index) {
            Some(Constant::Module(_)) => {}
            Some(_) => return Err(InvalidConstantPoolIndexType(require.index)),
            None => return Err(InvalidConstantPoolIndex(require.index)),
        }
        if require.version_index != 0 {
            match class_file.constant_pool.get(require.version_index) {
                Some(Constant::Utf8(_)) => {}
                Some(_) => {
                    return Err(InvalidConstantPoolIndexType(require.version_index));
                }
                None => return Err(InvalidConstantPoolIndex(require.version_index)),
            }
        }
    }
    for export in exports {
        match class_file.constant_pool.get(export.index) {
            Some(Constant::Package(_)) => {}
            Some(_) => return Err(InvalidConstantPoolIndexType(export.index)),
            None => return Err(InvalidConstantPoolIndex(export.index)),
        }
        for index in &export.to_index {
            match class_file.constant_pool.get(*index) {
                Some(Constant::Module(_)) => {}
                Some(_) => return Err(InvalidConstantPoolIndexType(*index)),
                None => return Err(InvalidConstantPoolIndex(*index)),
            }
        }
    }
    for open in opens {
        match class_file.constant_pool.get(open.index) {
            Some(Constant::Package(_)) => {}
            Some(_) => return Err(InvalidConstantPoolIndexType(open.index)),
            None => return Err(InvalidConstantPoolIndex(open.index)),
        }
        for index in &open.to_index {
            match class_file.constant_pool.get(*index) {
                Some(Constant::Module(_)) => {}
                Some(_) => return Err(InvalidConstantPoolIndexType(*index)),
                None => return Err(InvalidConstantPoolIndex(*index)),
            }
        }
    }
    for use_index in uses {
        match class_file.constant_pool.get(*use_index) {
            Some(Constant::Class(_)) => {}
            Some(_) => return Err(InvalidConstantPoolIndexType(*use_index)),
            None => return Err(InvalidConstantPoolIndex(*use_index)),
        }
    }
    for provide in provides {
        match class_file.constant_pool.get(provide.index) {
            Some(Constant::Class(_)) => {}
            Some(_) => return Err(InvalidConstantPoolIndexType(provide.index)),
            None => return Err(InvalidConstantPoolIndex(provide.index)),
        }
        for index in &provide.with_index {
            match class_file.constant_pool.get(*index) {
                Some(Constant::Class(_)) => {}
                Some(_) => return Err(InvalidConstantPoolIndexType(*index)),
                None => return Err(InvalidConstantPoolIndex(*index)),
            }
        }
    }
    Ok(())
}

fn verify_module_packages(
    class_file: &ClassFile,
    context: AttributeContext,
    package_indexes: &[u16],
) -> Result<()> {
    if !matches!(context, AttributeContext::Class) {
        return Err(VerificationError {
            context: "ModulePackages Attribute".to_string(),
            message: "ModulePackages attribute only allowed in ClassFile context".to_string(),
        });
    }
    for index in package_indexes {
        match class_file.constant_pool.get(*index) {
            Some(Constant::Package(_)) => {}
            Some(_) => return Err(InvalidConstantPoolIndexType(*index)),
            None => return Err(InvalidConstantPoolIndex(*index)),
        }
    }
    Ok(())
}

fn verify_module_main_class(
    class_file: &ClassFile,
    context: AttributeContext,
    main_class_index: u16,
) -> Result<()> {
    if !matches!(context, AttributeContext::Class) {
        return Err(VerificationError {
            context: "ModuleMainClass Attribute".to_string(),
            message: "ModuleMainClass attribute only allowed in ClassFile context".to_string(),
        });
    }
    match class_file.constant_pool.get(main_class_index) {
        Some(Constant::Class(_)) => {}
        Some(_) => return Err(InvalidConstantPoolIndexType(main_class_index)),
        None => return Err(InvalidConstantPoolIndex(main_class_index)),
    }
    Ok(())
}

fn verify_nest_host(
    class_file: &ClassFile,
    context: AttributeContext,
    host_class_index: u16,
) -> Result<()> {
    if !matches!(context, AttributeContext::Class) {
        return Err(VerificationError {
            context: "NestHost Attribute".to_string(),
            message: "NestHost attribute only allowed in ClassFile context".to_string(),
        });
    }
    match class_file.constant_pool.get(host_class_index) {
        Some(Constant::Class(_)) => {}
        Some(_) => return Err(InvalidConstantPoolIndexType(host_class_index)),
        None => return Err(InvalidConstantPoolIndex(host_class_index)),
    }
    Ok(())
}

fn verify_nest_members(
    class_file: &ClassFile,
    context: AttributeContext,
    class_indexes: &[u16],
) -> Result<()> {
    if !matches!(context, AttributeContext::Class) {
        return Err(VerificationError {
            context: "NestMembers Attribute".to_string(),
            message: "NestMembers attribute only allowed in ClassFile context".to_string(),
        });
    }
    for index in class_indexes {
        match class_file.constant_pool.get(*index) {
            Some(Constant::Class(_)) => {}
            Some(_) => return Err(InvalidConstantPoolIndexType(*index)),
            None => return Err(InvalidConstantPoolIndex(*index)),
        }
    }
    Ok(())
}

fn verify_record(
    class_file: &ClassFile,
    context: AttributeContext,
    records: &[Record],
) -> Result<()> {
    if !matches!(context, AttributeContext::Class) {
        return Err(VerificationError {
            context: "Record Attribute".to_string(),
            message: "Record attribute only allowed in ClassFile context".to_string(),
        });
    }
    for record in records {
        match class_file.constant_pool.get(record.name_index) {
            Some(Constant::Utf8(_)) => {}
            Some(_) => return Err(InvalidConstantPoolIndexType(record.name_index)),
            None => return Err(InvalidConstantPoolIndex(record.name_index)),
        }
        match class_file.constant_pool.get(record.descriptor_index) {
            Some(Constant::Utf8(_)) => {}
            Some(_) => return Err(InvalidConstantPoolIndexType(record.descriptor_index)),
            None => return Err(InvalidConstantPoolIndex(record.descriptor_index)),
        }
        verify(
            class_file,
            &record.attributes,
            AttributeContext::RecordComponent,
        )?;
    }
    Ok(())
}

fn verify_permitted_subclasses(
    class_file: &ClassFile,
    context: AttributeContext,
    class_indexes: &[u16],
) -> Result<()> {
    if !matches!(context, AttributeContext::Class) {
        return Err(VerificationError {
            context: "PermittedSubclasses Attribute".to_string(),
            message: "PermittedSubclasses attribute only allowed in ClassFile context".to_string(),
        });
    }
    for index in class_indexes {
        match class_file.constant_pool.get(*index) {
            Some(Constant::Class(_)) => {}
            Some(_) => return Err(InvalidConstantPoolIndexType(*index)),
            None => return Err(InvalidConstantPoolIndex(*index)),
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::attributes::{
        AnnotationValuePair, ExportsFlags, ModuleAccessFlags, NestedClassAccessFlags, OpensFlags,
        RequiresFlags,
    };
    use crate::constant::Constant;
    use crate::constant_pool::ConstantPool;
    use crate::field::Field;
    use crate::method::Method;
    use crate::verifiers::error::VerifyError;
    use crate::{BaseType, FieldAccessFlags, FieldType};
    use {Attribute, InnerClass, LineNumber, LocalVariableTable, LocalVariableTypeTable};

    fn create_class_file(constants: Vec<Constant>) -> ClassFile {
        let mut constant_pool = ConstantPool::default();
        for constant in constants {
            let _ = constant_pool.add(constant);
        }
        ClassFile {
            constant_pool,
            ..Default::default()
        }
    }

    #[test]
    fn test_verify_constant_value() {
        let constants = vec![Constant::Integer(42)];
        let class_file = create_class_file(constants);
        let attribute = Attribute::ConstantValue {
            name_index: 0,
            constant_value_index: 1,
        };
        let field = Field {
            access_flags: FieldAccessFlags::empty(),
            name_index: 0,
            descriptor_index: 0,
            field_type: FieldType::Base(BaseType::Int),
            attributes: vec![],
        };
        let context = AttributeContext::Field(&field);

        assert!(verify(&class_file, std::slice::from_ref(&attribute), context).is_ok());

        assert!(matches!(
            verify(
                &class_file,
                std::slice::from_ref(&attribute),
                AttributeContext::Class
            ),
            Err(VerifyError::VerificationError { .. })
        ));

        let attribute_invalid_index = Attribute::ConstantValue {
            name_index: 0,
            constant_value_index: 2,
        };
        assert!(matches!(
            verify(&class_file, &[attribute_invalid_index], context),
            Err(VerifyError::InvalidConstantPoolIndex(2))
        ));

        let constants_invalid = vec![Constant::Class(1)];
        let class_file_invalid = create_class_file(constants_invalid);
        assert!(matches!(
            verify(&class_file_invalid, &[attribute], context),
            Err(VerifyError::InvalidConstantPoolIndexType(1))
        ));
    }

    #[test]
    fn test_verify_code() {
        let constants = vec![
            Constant::Utf8("()V".to_string()),
            Constant::Utf8("Code".to_string()),
        ];
        let class_file = create_class_file(constants);
        let method = Method {
            descriptor_index: 1,
            ..Default::default()
        };
        let attribute = Attribute::Code {
            name_index: 2,
            max_stack: 0,
            max_locals: 1,
            code: vec![],
            exception_table: vec![],
            attributes: vec![],
        };
        let context = AttributeContext::Method(&method);

        assert!(verify(&class_file, std::slice::from_ref(&attribute), context).is_ok());

        assert!(matches!(
            verify(
                &class_file,
                std::slice::from_ref(&attribute),
                AttributeContext::Class
            ),
            Err(VerifyError::VerificationError { .. })
        ));

        let line_number_table = Attribute::LineNumberTable {
            name_index: 0,
            line_numbers: vec![],
        };
        let attribute_recursive = Attribute::Code {
            name_index: 2,
            max_stack: 0,
            max_locals: 1,
            code: vec![],
            exception_table: vec![],
            attributes: vec![line_number_table],
        };
        assert!(verify(&class_file, &[attribute_recursive], context).is_ok());
    }

    #[test]
    fn test_verify_stack_map_table() {
        let attribute = Attribute::StackMapTable {
            name_index: 0,
            frames: vec![],
        };
        let method = Method::default();
        let context = AttributeContext::Code(&method, 0);

        assert!(
            verify(
                &ClassFile::default(),
                std::slice::from_ref(&attribute),
                context
            )
            .is_ok()
        );

        assert!(matches!(
            verify(
                &ClassFile::default(),
                &[attribute],
                AttributeContext::Method(&method)
            ),
            Err(VerifyError::VerificationError { .. })
        ));
    }

    #[test]
    fn test_verify_exceptions() {
        let constants = vec![Constant::Class(2), Constant::Utf8("Exception".to_string())];
        let class_file = create_class_file(constants);
        let attribute = Attribute::Exceptions {
            name_index: 0,
            exception_indexes: vec![1],
        };
        let method = Method::default();
        let context = AttributeContext::Method(&method);

        assert!(verify(&class_file, std::slice::from_ref(&attribute), context).is_ok());

        assert!(matches!(
            verify(
                &class_file,
                std::slice::from_ref(&attribute),
                AttributeContext::Class
            ),
            Err(VerifyError::VerificationError { .. })
        ));

        let attribute_invalid_index = Attribute::Exceptions {
            name_index: 0,
            exception_indexes: vec![3],
        };
        assert!(matches!(
            verify(&class_file, &[attribute_invalid_index], context),
            Err(VerifyError::InvalidConstantPoolIndex(3))
        ));

        let attribute_invalid_type = Attribute::Exceptions {
            name_index: 0,
            exception_indexes: vec![2],
        };
        assert!(matches!(
            verify(&class_file, &[attribute_invalid_type], context),
            Err(VerifyError::InvalidConstantPoolIndexType(2))
        ));
    }

    #[test]
    fn test_verify_inner_classes() {
        let constants = vec![
            Constant::Class(2),
            Constant::Utf8("Inner".to_string()),
            Constant::Class(4),
            Constant::Utf8("Outer".to_string()),
            Constant::Utf8("Name".to_string()),
        ];
        let class_file = create_class_file(constants);
        let inner_class = InnerClass {
            class_info_index: 1,
            outer_class_info_index: 3,
            name_index: 5,
            access_flags: NestedClassAccessFlags::empty(),
        };
        let attribute = Attribute::InnerClasses {
            name_index: 0,
            classes: vec![inner_class.clone()],
        };
        let context = AttributeContext::Class;

        assert!(verify(&class_file, std::slice::from_ref(&attribute), context).is_ok());

        assert!(matches!(
            verify(
                &class_file,
                std::slice::from_ref(&attribute),
                AttributeContext::Method(&Method::default())
            ),
            Err(VerifyError::VerificationError { .. })
        ));

        // Invalid class_info_index
        let inner_class_invalid_class = InnerClass {
            class_info_index: 5,
            ..inner_class.clone()
        };
        let attribute_invalid_class = Attribute::InnerClasses {
            name_index: 0,
            classes: vec![inner_class_invalid_class],
        };
        assert!(matches!(
            verify(&class_file, &[attribute_invalid_class], context),
            Err(VerifyError::InvalidConstantPoolIndexType(5))
        ));

        // Invalid outer_class_info_index
        let inner_class_invalid_outer = InnerClass {
            outer_class_info_index: 5,
            ..inner_class.clone()
        };
        let attribute_invalid_outer = Attribute::InnerClasses {
            name_index: 0,
            classes: vec![inner_class_invalid_outer],
        };
        assert!(matches!(
            verify(&class_file, &[attribute_invalid_outer], context),
            Err(VerifyError::InvalidConstantPoolIndexType(5))
        ));

        // Invalid name_index
        let inner_class_invalid_name = InnerClass {
            name_index: 1,
            ..inner_class.clone()
        };
        let attribute_invalid_name = Attribute::InnerClasses {
            name_index: 0,
            classes: vec![inner_class_invalid_name],
        };
        assert!(matches!(
            verify(&class_file, &[attribute_invalid_name], context),
            Err(VerifyError::InvalidConstantPoolIndexType(1))
        ));
    }

    #[test]
    fn test_verify_enclosing_method() {
        let constants = vec![
            Constant::Class(2),
            Constant::Utf8("Class".to_string()),
            Constant::NameAndType {
                name_index: 4,
                descriptor_index: 5,
            },
            Constant::Utf8("method".to_string()),
            Constant::Utf8("()V".to_string()),
        ];
        let class_file = create_class_file(constants);
        let attribute = Attribute::EnclosingMethod {
            name_index: 0,
            class_index: 1,
            method_index: 3,
        };
        let context = AttributeContext::Class;

        assert!(verify(&class_file, std::slice::from_ref(&attribute), context).is_ok());

        assert!(matches!(
            verify(
                &class_file,
                std::slice::from_ref(&attribute),
                AttributeContext::Method(&Method::default())
            ),
            Err(VerifyError::VerificationError { .. })
        ));

        let attribute_invalid_class = Attribute::EnclosingMethod {
            name_index: 0,
            class_index: 3,
            method_index: 3,
        };
        assert!(matches!(
            verify(&class_file, &[attribute_invalid_class], context),
            Err(VerifyError::InvalidConstantPoolIndexType(3))
        ));

        let attribute_invalid_method = Attribute::EnclosingMethod {
            name_index: 0,
            class_index: 1,
            method_index: 1,
        };
        assert!(matches!(
            verify(&class_file, &[attribute_invalid_method], context),
            Err(VerifyError::InvalidConstantPoolIndexType(1))
        ));
    }

    #[test]
    fn test_verify_synthetic() {
        let attribute = Attribute::Synthetic { name_index: 0 };
        let context = AttributeContext::Class;

        assert!(
            verify(
                &ClassFile::default(),
                std::slice::from_ref(&attribute),
                context
            )
            .is_ok()
        );

        let method = Method::default();
        assert!(matches!(
            verify(
                &ClassFile::default(),
                &[attribute],
                AttributeContext::Code(&method, 0)
            ),
            Err(VerifyError::VerificationError { .. })
        ));
    }

    #[test]
    fn test_verify_signature() {
        let constants = vec![Constant::Utf8("Signature".to_string())];
        let class_file = create_class_file(constants);
        let attribute = Attribute::Signature {
            name_index: 0,
            signature_index: 1,
        };
        let context = AttributeContext::Class;

        assert!(verify(&class_file, std::slice::from_ref(&attribute), context).is_ok());

        let method = Method::default();
        assert!(matches!(
            verify(
                &class_file,
                std::slice::from_ref(&attribute),
                AttributeContext::Code(&method, 0)
            ),
            Err(VerifyError::VerificationError { .. })
        ));

        let attribute_invalid = Attribute::Signature {
            name_index: 0,
            signature_index: 2,
        };
        assert!(matches!(
            verify(&class_file, &[attribute_invalid], context),
            Err(VerifyError::InvalidConstantPoolIndex(2))
        ));
    }

    #[test]
    fn test_verify_source_file() {
        let constants = vec![Constant::Utf8("Source.java".to_string())];
        let class_file = create_class_file(constants);
        let attribute = Attribute::SourceFile {
            name_index: 0,
            source_file_index: 1,
        };
        let context = AttributeContext::Class;

        assert!(verify(&class_file, std::slice::from_ref(&attribute), context).is_ok());

        assert!(matches!(
            verify(
                &class_file,
                std::slice::from_ref(&attribute),
                AttributeContext::Method(&Method::default())
            ),
            Err(VerifyError::VerificationError { .. })
        ));

        let attribute_invalid = Attribute::SourceFile {
            name_index: 0,
            source_file_index: 2,
        };
        assert!(matches!(
            verify(&class_file, &[attribute_invalid], context),
            Err(VerifyError::InvalidConstantPoolIndex(2))
        ));
    }

    #[test]
    fn test_verify_source_debug_extension() {
        let attribute = Attribute::SourceDebugExtension {
            name_index: 0,
            debug_extension: "debug".to_string(),
        };
        let context = AttributeContext::Class;

        assert!(
            verify(
                &ClassFile::default(),
                std::slice::from_ref(&attribute),
                context
            )
            .is_ok()
        );

        assert!(matches!(
            verify(
                &ClassFile::default(),
                &[attribute],
                AttributeContext::Method(&Method::default())
            ),
            Err(VerifyError::VerificationError { .. })
        ));
    }

    #[test]
    fn test_verify_line_number_table() {
        let attribute = Attribute::LineNumberTable {
            name_index: 0,
            line_numbers: vec![LineNumber {
                start_pc: 0,
                line_number: 1,
            }],
        };
        let method = Method::default();
        let context = AttributeContext::Code(&method, 10);

        assert!(
            verify(
                &ClassFile::default(),
                std::slice::from_ref(&attribute),
                context
            )
            .is_ok()
        );

        // Method context is now allowed (some compilers place LineNumberTable at method level)
        assert!(
            verify(
                &ClassFile::default(),
                std::slice::from_ref(&attribute),
                AttributeContext::Method(&method)
            )
            .is_ok()
        );

        // Class context should still fail
        assert!(matches!(
            verify(
                &ClassFile::default(),
                std::slice::from_ref(&attribute),
                AttributeContext::Class
            ),
            Err(VerifyError::VerificationError { .. })
        ));

        let attribute_invalid = Attribute::LineNumberTable {
            name_index: 0,
            line_numbers: vec![LineNumber {
                start_pc: 10,
                line_number: 1,
            }],
        };
        assert!(matches!(
            verify(&ClassFile::default(), &[attribute_invalid], context),
            Err(VerifyError::VerificationError { .. })
        ));
    }

    #[test]
    fn test_verify_local_variable_table() {
        let constants = vec![
            Constant::Utf8("name".to_string()),
            Constant::Utf8("descriptor".to_string()),
        ];
        let class_file = create_class_file(constants);
        let attribute = Attribute::LocalVariableTable {
            name_index: 0,
            variables: vec![LocalVariableTable {
                start_pc: 0,
                length: 1,
                name_index: 1,
                descriptor_index: 2,
                index: 0,
            }],
        };
        let method = Method::default();
        let context = AttributeContext::Code(&method, 10);

        assert!(verify(&class_file, std::slice::from_ref(&attribute), context).is_ok());

        // Method context is now allowed (some compilers place LocalVariableTable at method level)
        assert!(
            verify(
                &class_file,
                std::slice::from_ref(&attribute),
                AttributeContext::Method(&method)
            )
            .is_ok()
        );

        // Class context should still fail
        assert!(matches!(
            verify(
                &class_file,
                std::slice::from_ref(&attribute),
                AttributeContext::Class
            ),
            Err(VerifyError::VerificationError { .. })
        ));

        let attribute_range = Attribute::LocalVariableTable {
            name_index: 0,
            variables: vec![LocalVariableTable {
                start_pc: 9,
                length: 2,
                name_index: 1,
                descriptor_index: 2,
                index: 0,
            }],
        };
        assert!(matches!(
            verify(&class_file, &[attribute_range], context),
            Err(VerifyError::VerificationError { .. })
        ));

        let attribute_name = Attribute::LocalVariableTable {
            name_index: 0,
            variables: vec![LocalVariableTable {
                start_pc: 0,
                length: 1,
                name_index: 3,
                descriptor_index: 2,
                index: 0,
            }],
        };
        assert!(matches!(
            verify(&class_file, &[attribute_name], context),
            Err(VerifyError::InvalidConstantPoolIndex(3))
        ));

        let attribute_desc = Attribute::LocalVariableTable {
            name_index: 0,
            variables: vec![LocalVariableTable {
                start_pc: 0,
                length: 1,
                name_index: 1,
                descriptor_index: 3,
                index: 0,
            }],
        };
        assert!(matches!(
            verify(&class_file, &[attribute_desc], context),
            Err(VerifyError::InvalidConstantPoolIndex(3))
        ));
    }

    #[test]
    fn test_verify_local_variable_type_table() {
        let constants = vec![
            Constant::Utf8("name".to_string()),
            Constant::Utf8("signature".to_string()),
        ];
        let class_file = create_class_file(constants);
        let attribute = Attribute::LocalVariableTypeTable {
            name_index: 0,
            variable_types: vec![LocalVariableTypeTable {
                start_pc: 0,
                length: 1,
                name_index: 1,
                signature_index: 2,
                index: 0,
            }],
        };
        let method = Method::default();
        let context = AttributeContext::Code(&method, 10);

        assert!(verify(&class_file, std::slice::from_ref(&attribute), context).is_ok());

        // Method context is now allowed (some compilers place LocalVariableTypeTable at method level)
        assert!(
            verify(
                &class_file,
                std::slice::from_ref(&attribute),
                AttributeContext::Method(&method)
            )
            .is_ok()
        );

        // Class context should still fail
        assert!(matches!(
            verify(
                &class_file,
                std::slice::from_ref(&attribute),
                AttributeContext::Class
            ),
            Err(VerifyError::VerificationError { .. })
        ));

        let attribute_range = Attribute::LocalVariableTypeTable {
            name_index: 0,
            variable_types: vec![LocalVariableTypeTable {
                start_pc: 9,
                length: 2,
                name_index: 1,
                signature_index: 2,
                index: 0,
            }],
        };
        assert!(matches!(
            verify(&class_file, &[attribute_range], context),
            Err(VerifyError::VerificationError { .. })
        ));

        let attribute_name = Attribute::LocalVariableTypeTable {
            name_index: 0,
            variable_types: vec![LocalVariableTypeTable {
                start_pc: 0,
                length: 1,
                name_index: 3,
                signature_index: 2,
                index: 0,
            }],
        };
        assert!(matches!(
            verify(&class_file, &[attribute_name], context),
            Err(VerifyError::InvalidConstantPoolIndex(3))
        ));

        let attribute_sig = Attribute::LocalVariableTypeTable {
            name_index: 0,
            variable_types: vec![LocalVariableTypeTable {
                start_pc: 0,
                length: 1,
                name_index: 1,
                signature_index: 3,
                index: 0,
            }],
        };
        assert!(matches!(
            verify(&class_file, &[attribute_sig], context),
            Err(VerifyError::InvalidConstantPoolIndex(3))
        ));
    }

    #[test]
    fn test_verify_deprecated() {
        let attribute = Attribute::Deprecated { name_index: 0 };
        let context = AttributeContext::Class;

        assert!(
            verify(
                &ClassFile::default(),
                std::slice::from_ref(&attribute),
                context
            )
            .is_ok()
        );

        let method = Method::default();
        assert!(matches!(
            verify(
                &ClassFile::default(),
                &[attribute],
                AttributeContext::Code(&method, 0)
            ),
            Err(VerifyError::VerificationError { .. })
        ));
    }

    #[test]
    fn test_verify_unknown_attribute() {
        let attribute = Attribute::Unknown {
            name_index: 0,
            info: vec![],
        };
        let context = AttributeContext::Class;
        assert!(verify(&ClassFile::default(), &[attribute], context).is_ok());
    }

    #[test]
    fn test_verify_annotations() {
        let constants = vec![
            Constant::Utf8("LAnnotation;".to_string()),
            Constant::Utf8("name".to_string()),
            Constant::Integer(42),
        ];
        let class_file = create_class_file(constants);
        let annotation = Annotation {
            type_index: 1,
            elements: vec![AnnotationValuePair {
                name_index: 2,
                value: AnnotationElement::Int {
                    const_value_index: 3,
                },
            }],
        };
        let attribute = Attribute::RuntimeVisibleAnnotations {
            name_index: 0,
            annotations: vec![annotation.clone()],
        };
        let context = AttributeContext::Class;

        assert!(verify(&class_file, std::slice::from_ref(&attribute), context).is_ok());

        let annotation_invalid_type = Annotation {
            type_index: 3,
            ..annotation.clone()
        };
        let attribute_invalid_type = Attribute::RuntimeVisibleAnnotations {
            name_index: 0,
            annotations: vec![annotation_invalid_type],
        };
        assert!(matches!(
            verify(&class_file, &[attribute_invalid_type], context),
            Err(VerifyError::InvalidConstantPoolIndexType(3))
        ));

        let annotation_invalid_name = Annotation {
            elements: vec![AnnotationValuePair {
                name_index: 3,
                value: AnnotationElement::Int {
                    const_value_index: 3,
                },
            }],
            ..annotation.clone()
        };
        let attribute_invalid_name = Attribute::RuntimeVisibleAnnotations {
            name_index: 0,
            annotations: vec![annotation_invalid_name],
        };
        assert!(matches!(
            verify(&class_file, &[attribute_invalid_name], context),
            Err(VerifyError::InvalidConstantPoolIndexType(3))
        ));

        let annotation_invalid_value = Annotation {
            elements: vec![AnnotationValuePair {
                name_index: 2,
                value: AnnotationElement::Int {
                    const_value_index: 1,
                },
            }],
            ..annotation.clone()
        };
        let attribute_invalid_value = Attribute::RuntimeVisibleAnnotations {
            name_index: 0,
            annotations: vec![annotation_invalid_value],
        };
        assert!(matches!(
            verify(&class_file, &[attribute_invalid_value], context),
            Err(VerifyError::InvalidConstantPoolIndexType(1))
        ));
    }

    #[test]
    fn test_verify_parameter_annotations() {
        let attribute = Attribute::RuntimeVisibleParameterAnnotations {
            name_index: 0,
            parameter_annotations: vec![],
        };
        let method = Method::default();
        let context = AttributeContext::Method(&method);

        assert!(
            verify(
                &ClassFile::default(),
                std::slice::from_ref(&attribute),
                context
            )
            .is_ok()
        );

        assert!(matches!(
            verify(&ClassFile::default(), &[attribute], AttributeContext::Class),
            Err(VerifyError::VerificationError { .. })
        ));
    }

    #[test]
    fn test_verify_annotation_default() {
        let constants = vec![Constant::Integer(42)];
        let class_file = create_class_file(constants);
        let attribute = Attribute::AnnotationDefault {
            name_index: 0,
            element: AnnotationElement::Int {
                const_value_index: 1,
            },
        };
        let method = Method::default();
        let context = AttributeContext::Method(&method);

        assert!(verify(&class_file, std::slice::from_ref(&attribute), context).is_ok());

        assert!(matches!(
            verify(&class_file, &[attribute], AttributeContext::Class),
            Err(VerifyError::VerificationError { .. })
        ));
    }

    #[test]
    fn test_verify_bootstrap_methods() {
        let constants = vec![
            Constant::MethodHandle {
                reference_kind: crate::ReferenceKind::InvokeStatic,
                reference_index: 2,
            },
            Constant::Integer(42),
        ];
        let class_file = create_class_file(constants);
        let attribute = Attribute::BootstrapMethods {
            name_index: 0,
            methods: vec![BootstrapMethod {
                bootstrap_method_ref: 1,
                arguments: vec![2],
            }],
        };
        let context = AttributeContext::Class;

        assert!(verify(&class_file, std::slice::from_ref(&attribute), context).is_ok());

        assert!(matches!(
            verify(
                &class_file,
                std::slice::from_ref(&attribute),
                AttributeContext::Method(&Method::default())
            ),
            Err(VerifyError::VerificationError { .. })
        ));

        let attribute_invalid_ref = Attribute::BootstrapMethods {
            name_index: 0,
            methods: vec![BootstrapMethod {
                bootstrap_method_ref: 2,
                arguments: vec![2],
            }],
        };
        assert!(matches!(
            verify(&class_file, &[attribute_invalid_ref], context),
            Err(VerifyError::InvalidConstantPoolIndexType(2))
        ));

        let attribute_invalid_arg = Attribute::BootstrapMethods {
            name_index: 0,
            methods: vec![BootstrapMethod {
                bootstrap_method_ref: 1,
                arguments: vec![3],
            }],
        };
        assert!(matches!(
            verify(&class_file, &[attribute_invalid_arg], context),
            Err(VerifyError::InvalidConstantPoolIndex(3))
        ));
    }

    #[test]
    fn test_verify_method_parameters() {
        let constants = vec![Constant::Utf8("param".to_string())];
        let class_file = create_class_file(constants);
        let attribute = Attribute::MethodParameters {
            name_index: 0,
            parameters: vec![MethodParameter {
                name_index: 1,
                access_flags: crate::MethodAccessFlags::empty(),
            }],
        };
        let method = Method::default();
        let context = AttributeContext::Method(&method);

        assert!(verify(&class_file, std::slice::from_ref(&attribute), context).is_ok());

        assert!(matches!(
            verify(
                &class_file,
                std::slice::from_ref(&attribute),
                AttributeContext::Class
            ),
            Err(VerifyError::VerificationError { .. })
        ));

        let attribute_invalid = Attribute::MethodParameters {
            name_index: 0,
            parameters: vec![MethodParameter {
                name_index: 2,
                access_flags: crate::MethodAccessFlags::empty(),
            }],
        };
        assert!(matches!(
            verify(&class_file, &[attribute_invalid], context),
            Err(VerifyError::InvalidConstantPoolIndex(2))
        ));
    }

    #[test]
    fn test_verify_module() {
        let constants = vec![
            Constant::Module(2),
            Constant::Utf8("module".to_string()),
            Constant::Utf8("1.0".to_string()),
            Constant::Package(5),
            Constant::Utf8("package".to_string()),
            Constant::Class(7),
            Constant::Utf8("Class".to_string()),
        ];
        let class_file = create_class_file(constants);
        let attribute = Attribute::Module {
            name_index: 0,
            module_name_index: 1,
            flags: ModuleAccessFlags::empty(),
            version_index: 3,
            requires: vec![Requires {
                index: 1,
                flags: RequiresFlags::empty(),
                version_index: 3,
            }],
            exports: vec![Exports {
                index: 4,
                flags: ExportsFlags::empty(),
                to_index: vec![1],
            }],
            opens: vec![Opens {
                index: 4,
                flags: OpensFlags::empty(),
                to_index: vec![1],
            }],
            uses: vec![6],
            provides: vec![Provides {
                index: 6,
                with_index: vec![6],
            }],
        };
        let context = AttributeContext::Class;

        assert!(verify(&class_file, std::slice::from_ref(&attribute), context).is_ok());

        assert!(matches!(
            verify(
                &class_file,
                &[attribute],
                AttributeContext::Method(&Method::default())
            ),
            Err(VerifyError::VerificationError { .. })
        ));
    }

    #[test]
    fn test_verify_module_packages() {
        let constants = vec![Constant::Package(2), Constant::Utf8("package".to_string())];
        let class_file = create_class_file(constants);
        let attribute = Attribute::ModulePackages {
            name_index: 0,
            package_indexes: vec![1],
        };
        let context = AttributeContext::Class;

        assert!(verify(&class_file, std::slice::from_ref(&attribute), context).is_ok());

        assert!(matches!(
            verify(
                &class_file,
                &[attribute],
                AttributeContext::Method(&Method::default())
            ),
            Err(VerifyError::VerificationError { .. })
        ));
    }

    #[test]
    fn test_verify_module_main_class() {
        let constants = vec![Constant::Class(2), Constant::Utf8("Main".to_string())];
        let class_file = create_class_file(constants);
        let attribute = Attribute::ModuleMainClass {
            name_index: 0,
            main_class_index: 1,
        };
        let context = AttributeContext::Class;

        assert!(verify(&class_file, std::slice::from_ref(&attribute), context).is_ok());

        assert!(matches!(
            verify(
                &class_file,
                &[attribute],
                AttributeContext::Method(&Method::default())
            ),
            Err(VerifyError::VerificationError { .. })
        ));
    }

    #[test]
    fn test_verify_nest_host() {
        let constants = vec![Constant::Class(2), Constant::Utf8("Host".to_string())];
        let class_file = create_class_file(constants);
        let attribute = Attribute::NestHost {
            name_index: 0,
            host_class_index: 1,
        };
        let context = AttributeContext::Class;

        assert!(verify(&class_file, std::slice::from_ref(&attribute), context).is_ok());

        assert!(matches!(
            verify(
                &class_file,
                &[attribute],
                AttributeContext::Method(&Method::default())
            ),
            Err(VerifyError::VerificationError { .. })
        ));
    }

    #[test]
    fn test_verify_nest_members() {
        let constants = vec![Constant::Class(2), Constant::Utf8("Member".to_string())];
        let class_file = create_class_file(constants);
        let attribute = Attribute::NestMembers {
            name_index: 0,
            class_indexes: vec![1],
        };
        let context = AttributeContext::Class;

        assert!(verify(&class_file, std::slice::from_ref(&attribute), context).is_ok());

        assert!(matches!(
            verify(
                &class_file,
                &[attribute],
                AttributeContext::Method(&Method::default())
            ),
            Err(VerifyError::VerificationError { .. })
        ));
    }

    #[test]
    fn test_verify_record() {
        let constants = vec![
            Constant::Utf8("name".to_string()),
            Constant::Utf8("descriptor".to_string()),
        ];
        let class_file = create_class_file(constants);
        let attribute = Attribute::Record {
            name_index: 0,
            records: vec![Record {
                name_index: 1,
                descriptor_index: 2,
                attributes: vec![],
            }],
        };
        let context = AttributeContext::Class;

        assert!(verify(&class_file, std::slice::from_ref(&attribute), context).is_ok());

        assert!(matches!(
            verify(
                &class_file,
                &[attribute],
                AttributeContext::Method(&Method::default())
            ),
            Err(VerifyError::VerificationError { .. })
        ));
    }

    #[test]
    fn test_verify_permitted_subclasses() {
        let constants = vec![Constant::Class(2), Constant::Utf8("Sub".to_string())];
        let class_file = create_class_file(constants);
        let attribute = Attribute::PermittedSubclasses {
            name_index: 0,
            class_indexes: vec![1],
        };
        let context = AttributeContext::Class;

        assert!(verify(&class_file, std::slice::from_ref(&attribute), context).is_ok());

        assert!(matches!(
            verify(
                &class_file,
                &[attribute],
                AttributeContext::Method(&Method::default())
            ),
            Err(VerifyError::VerificationError { .. })
        ));
    }
}
