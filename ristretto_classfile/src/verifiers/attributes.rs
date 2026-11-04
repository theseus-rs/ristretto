use crate::JAVA_7;
use crate::attributes::{
    Annotation, AnnotationElement, Attribute, BootstrapMethod, Exports, InnerClass, LineNumber,
    LocalVariableTable, LocalVariableTypeTable, MethodParameter, Opens, ParameterAnnotation,
    Provides, Requires, TargetType, TypeAnnotation,
};
use crate::class_access_flags::ClassAccessFlags;
use crate::class_file::ClassFile;
use crate::constant::Constant;
use crate::field::Field;
use crate::method::Method;
use crate::verifiers::bytecode::control_flow::CodeInfo;
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

impl AttributeContext<'_> {
    fn location(self) -> crate::attributes::attribute::AttributeLocation {
        match self {
            AttributeContext::Class => crate::attributes::attribute::AttributeLocation::Class,
            AttributeContext::Field(_) => crate::attributes::attribute::AttributeLocation::Field,
            AttributeContext::Method(_) => crate::attributes::attribute::AttributeLocation::Method,
            AttributeContext::Code(..) => crate::attributes::attribute::AttributeLocation::Code,
            AttributeContext::RecordComponent => {
                crate::attributes::attribute::AttributeLocation::RecordComponent
            }
        }
    }
}

/// Verify the attributes.
///
/// # Errors
/// Returns `VerificationError` if the attributes are invalid.
pub(crate) fn verify(
    class_file: &ClassFile<'_>,
    attributes: &[Attribute],
    context: AttributeContext,
) -> Result<()> {
    verify_with_code_info(class_file, attributes, context, None)
}

pub(crate) fn verify_with_code_info(
    class_file: &ClassFile<'_>,
    attributes: &[Attribute],
    context: AttributeContext,
    code_info: Option<&CodeInfo>,
) -> Result<()> {
    let location = context.location();
    for (index, attribute) in attributes.iter().enumerate() {
        let previous = attributes.get(..index).unwrap_or_default();
        if super::format::verify_attribute(class_file, attribute, previous, location)? {
            verify_attribute(class_file, attribute, previous, context, code_info)?;
        }
    }
    Ok(())
}

#[expect(clippy::too_many_lines)]
fn verify_attribute(
    class_file: &ClassFile<'_>,
    attribute: &Attribute,
    previous: &[Attribute],
    context: AttributeContext,
    code_info: Option<&CodeInfo>,
) -> Result<()> {
    match (attribute, context) {
        (
            Attribute::ConstantValue {
                constant_value_index,
                ..
            },
            _,
        ) => verify_constant_value(class_file, *constant_value_index)?,
        (
            Attribute::Code {
                max_stack,
                max_locals,
                code,
                attributes,
                exception_table,
                ..
            },
            AttributeContext::Method(method),
        ) => {
            let owned_info;
            let info = if let Some(info) = code_info {
                info
            } else {
                owned_info = CodeInfo::from_code(code)?;
                &owned_info
            };
            let code_length = code::verify(
                class_file,
                method,
                *max_stack,
                *max_locals,
                code,
                exception_table,
                attributes,
                info,
            )?;
            super::format::verify_code_metadata(class_file, attribute, info)?;
            verify(
                class_file,
                attributes,
                AttributeContext::Code(method, code_length),
            )?;
        }
        (
            Attribute::Exceptions {
                exception_indexes, ..
            },
            _,
        ) => verify_exceptions(class_file, exception_indexes)?,
        (Attribute::InnerClasses { classes, .. }, _) => {
            verify_inner_classes(class_file, classes)?;
        }
        (
            Attribute::EnclosingMethod {
                class_index,
                method_index,
                ..
            },
            _,
        ) => verify_enclosing_method(class_file, *class_index, *method_index)?,
        (
            Attribute::Signature {
                signature_index, ..
            },
            _,
        ) => verify_signature(class_file, *signature_index)?,
        (
            Attribute::SourceFile {
                source_file_index, ..
            },
            _,
        ) => verify_source_file(class_file, *source_file_index)?,
        (
            Attribute::LineNumberTable { line_numbers, .. },
            AttributeContext::Code(_, code_length),
        ) => {
            verify_line_number_table(code_length, line_numbers)?;
        }
        (
            Attribute::LocalVariableTable { variables, .. },
            AttributeContext::Code(_, code_length),
        ) => {
            verify_local_variable_table(class_file, code_length, variables)?;
        }
        (
            Attribute::LocalVariableTypeTable { variable_types, .. },
            AttributeContext::Code(_, code_length),
        ) => {
            verify_local_variable_type_table(class_file, code_length, variable_types)?;
        }
        (
            Attribute::RuntimeVisibleAnnotations { annotations, .. }
            | Attribute::RuntimeInvisibleAnnotations { annotations, .. },
            _,
        ) => {
            verify_annotations(class_file, annotations)?;
        }
        (
            Attribute::RuntimeVisibleParameterAnnotations {
                parameter_annotations,
                ..
            }
            | Attribute::RuntimeInvisibleParameterAnnotations {
                parameter_annotations,
                ..
            },
            _,
        ) => verify_parameter_annotations(class_file, parameter_annotations)?,
        (
            Attribute::RuntimeVisibleTypeAnnotations {
                type_annotations, ..
            }
            | Attribute::RuntimeInvisibleTypeAnnotations {
                type_annotations, ..
            },
            _,
        ) => verify_type_annotations(class_file, context, type_annotations)?,
        (Attribute::AnnotationDefault { element, .. }, _) => {
            verify_annotation_default(class_file, element)?;
        }
        (Attribute::BootstrapMethods { methods, .. }, _) => {
            verify_bootstrap_methods(class_file, methods)?;
        }
        (Attribute::MethodParameters { parameters, .. }, _) => {
            verify_method_parameters(class_file, parameters)?;
        }
        (
            Attribute::Module {
                module_name_index,
                version_index,
                requires,
                exports,
                opens,
                uses,
                provides,
                ..
            },
            _,
        ) => verify_module(
            class_file,
            *module_name_index,
            *version_index,
            requires,
            exports,
            opens,
            uses,
            provides,
        )?,
        (
            Attribute::ModulePackages {
                package_indexes, ..
            },
            _,
        ) => verify_module_packages(class_file, package_indexes)?,
        (
            Attribute::ModuleMainClass {
                main_class_index, ..
            },
            _,
        ) => verify_module_main_class(class_file, *main_class_index)?,
        (Attribute::NestHost { .. } | Attribute::NestMembers { .. }, _) => {
            super::nest::verify(class_file, attribute, previous)?;
        }
        (Attribute::Record { .. }, _) => super::record::verify(class_file, attribute, previous)?,
        (Attribute::PermittedSubclasses { .. }, _) => {
            super::permitted_subclasses::verify(class_file, attribute, previous)?;
        }
        _ => {}
    }
    Ok(())
}

fn verify_constant_value(class_file: &ClassFile<'_>, constant_value_index: u16) -> Result<()> {
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

fn verify_exceptions(class_file: &ClassFile<'_>, exception_indexes: &[u16]) -> Result<()> {
    for index in exception_indexes {
        super::format::verify_non_array_class(class_file, *index)?;
    }
    Ok(())
}

fn verify_inner_classes(class_file: &ClassFile<'_>, classes: &[InnerClass]) -> Result<()> {
    for class in classes {
        // class_info_index
        super::format::verify_non_array_class(class_file, class.class_info_index)?;
        // outer_class_info_index (can be 0)
        if class.outer_class_info_index != 0 {
            super::format::verify_non_array_class(class_file, class.outer_class_info_index)?;
        }
        // inner_name_index (can be 0)
        if class.name_index == 0 {
            if class_file.version.major() >= JAVA_7.major() && class.outer_class_info_index != 0 {
                return Err(VerificationError {
                    context: "InnerClasses Attribute".to_string(),
                    message: "Anonymous inner class must have zero outer class index".to_string(),
                });
            }
            continue;
        }
        match class_file.constant_pool.get(class.name_index) {
            Some(Constant::Utf8(_)) => {}
            Some(_) => return Err(InvalidConstantPoolIndexType(class.name_index)),
            None => return Err(InvalidConstantPoolIndex(class.name_index)),
        }
    }
    Ok(())
}

fn verify_enclosing_method(
    class_file: &ClassFile<'_>,
    class_index: u16,
    method_index: u16,
) -> Result<()> {
    super::format::verify_non_array_class(class_file, class_index)?;
    if method_index != 0 {
        match class_file.constant_pool.get(method_index) {
            Some(Constant::NameAndType { .. }) => {}
            Some(_) => return Err(InvalidConstantPoolIndexType(method_index)),
            None => return Err(InvalidConstantPoolIndex(method_index)),
        }
    }
    Ok(())
}

fn verify_signature(class_file: &ClassFile<'_>, signature_index: u16) -> Result<()> {
    match class_file.constant_pool.get(signature_index) {
        Some(Constant::Utf8(_)) => {}
        Some(_) => return Err(InvalidConstantPoolIndexType(signature_index)),
        None => return Err(InvalidConstantPoolIndex(signature_index)),
    }
    Ok(())
}

fn verify_source_file(class_file: &ClassFile<'_>, source_file_index: u16) -> Result<()> {
    match class_file.constant_pool.get(source_file_index) {
        Some(Constant::Utf8(_)) => {}
        Some(_) => return Err(InvalidConstantPoolIndexType(source_file_index)),
        None => return Err(InvalidConstantPoolIndex(source_file_index)),
    }
    Ok(())
}

fn verify_line_number_table(code_length: u16, line_numbers: &[LineNumber]) -> Result<()> {
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
    Ok(())
}

fn verify_local_variable_table(
    class_file: &ClassFile<'_>,
    code_length: u16,
    variables: &[LocalVariableTable],
) -> Result<()> {
    for local_variable in variables {
        if u32::from(local_variable.start_pc) + u32::from(local_variable.length)
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
    class_file: &ClassFile<'_>,
    code_length: u16,
    variable_types: &[LocalVariableTypeTable],
) -> Result<()> {
    for local_variable in variable_types {
        if u32::from(local_variable.start_pc) + u32::from(local_variable.length)
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

fn verify_annotations(class_file: &ClassFile<'_>, annotations: &[Annotation]) -> Result<()> {
    for annotation in annotations {
        verify_annotation(class_file, annotation)?;
    }
    Ok(())
}

fn verify_annotation(class_file: &ClassFile<'_>, annotation: &Annotation) -> Result<()> {
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

fn verify_annotation_element(
    class_file: &ClassFile<'_>,
    element: &AnnotationElement,
) -> Result<()> {
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
    class_file: &ClassFile<'_>,
    parameter_annotations: &[ParameterAnnotation],
) -> Result<()> {
    for parameter_annotation in parameter_annotations {
        verify_annotations(class_file, &parameter_annotation.annotations)?;
    }
    Ok(())
}

fn verify_annotation_target(
    class_file: &ClassFile<'_>,
    context: AttributeContext,
    target: &TargetType,
) -> Result<()> {
    let tag = target.target_type();
    let valid_location = match context {
        AttributeContext::Class => matches!(tag, 0x00 | 0x10 | 0x11),
        AttributeContext::Field(_) | AttributeContext::RecordComponent => tag == 0x13,
        AttributeContext::Method(_) => matches!(tag, 0x01 | 0x12 | 0x14..=0x17),
        AttributeContext::Code(..) => matches!(tag, 0x40..=0x4B),
    };
    let invalid = || VerificationError {
        context: "Type annotation target".to_string(),
        message: "Invalid target location or index".to_string(),
    };
    if !valid_location {
        return Err(invalid());
    }
    match (context, target) {
        (
            AttributeContext::Class,
            TargetType::SuperType {
                supertype_index, ..
            },
        ) => {
            if *supertype_index == u16::MAX {
                if class_file.super_class == 0
                    || class_file
                        .access_flags
                        .contains(ClassAccessFlags::INTERFACE)
                {
                    return Err(invalid());
                }
            } else if usize::from(*supertype_index) >= class_file.interfaces.len() {
                return Err(invalid());
            }
        }
        (
            AttributeContext::Code(method, _),
            TargetType::Catch {
                exception_table_index,
                ..
            },
        ) => {
            let count = method
                .attributes
                .iter()
                .find_map(|a| match a {
                    Attribute::Code {
                        exception_table, ..
                    } => Some(exception_table.len()),
                    _ => None,
                })
                .unwrap_or(0);
            if usize::from(*exception_table_index) >= count {
                return Err(invalid());
            }
        }
        (
            AttributeContext::Method(method),
            TargetType::Throws {
                throws_type_index, ..
            },
        ) => {
            let count = method
                .attributes
                .iter()
                .find_map(|a| match a {
                    Attribute::Exceptions {
                        exception_indexes, ..
                    } => Some(exception_indexes.len()),
                    _ => None,
                })
                .unwrap_or(0);
            if usize::from(*throws_type_index) >= count {
                return Err(invalid());
            }
        }
        _ => {}
    }
    Ok(())
}

fn verify_type_annotations(
    class_file: &ClassFile<'_>,
    context: AttributeContext,
    type_annotations: &[TypeAnnotation],
) -> Result<()> {
    for type_annotation in type_annotations {
        type_annotation.target_type.validate()?;
        verify_annotation_target(class_file, context, &type_annotation.target_type)?;
        for path in &type_annotation.type_path {
            path.validate()?;
        }
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
    class_file: &ClassFile<'_>,
    element: &AnnotationElement,
) -> Result<()> {
    verify_annotation_element(class_file, element)
}

fn verify_bootstrap_methods(class_file: &ClassFile<'_>, methods: &[BootstrapMethod]) -> Result<()> {
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
    class_file: &ClassFile<'_>,
    parameters: &[MethodParameter],
) -> Result<()> {
    for parameter in parameters {
        if parameter.name_index == 0 {
            continue;
        }
        match class_file.constant_pool.get(parameter.name_index) {
            Some(Constant::Utf8(_)) => {}
            Some(_) => return Err(InvalidConstantPoolIndexType(parameter.name_index)),
            None => return Err(InvalidConstantPoolIndex(parameter.name_index)),
        }
    }
    Ok(())
}

#[expect(clippy::too_many_arguments)]
fn verify_module(
    class_file: &ClassFile<'_>,
    module_name_index: u16,
    version_index: u16,
    requires: &[Requires],
    exports: &[Exports],
    opens: &[Opens],
    uses: &[u16],
    provides: &[Provides],
) -> Result<()> {
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
        if require.version_index == 0 {
            continue;
        }
        match class_file.constant_pool.get(require.version_index) {
            Some(Constant::Utf8(_)) => {}
            Some(_) => {
                return Err(InvalidConstantPoolIndexType(require.version_index));
            }
            None => return Err(InvalidConstantPoolIndex(require.version_index)),
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

fn verify_module_packages(class_file: &ClassFile<'_>, package_indexes: &[u16]) -> Result<()> {
    for index in package_indexes {
        match class_file.constant_pool.get(*index) {
            Some(Constant::Package(_)) => {}
            Some(_) => return Err(InvalidConstantPoolIndexType(*index)),
            None => return Err(InvalidConstantPoolIndex(*index)),
        }
    }
    Ok(())
}

fn verify_module_main_class(class_file: &ClassFile<'_>, main_class_index: u16) -> Result<()> {
    super::format::verify_non_array_class(class_file, main_class_index)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::attributes::ExceptionTableEntry;
    use crate::attributes::{
        AnnotationValuePair, ExportsFlags, Instruction, MethodParameterAccessFlags,
        ModuleAccessFlags, NestedClassAccessFlags, OpensFlags, Record, RequiresFlags, TargetType,
    };
    use crate::constant::Constant;
    use crate::constant_pool::ConstantPool;
    use crate::field::Field;
    use crate::method::Method;
    use crate::{BaseType, FieldAccessFlags, FieldType};
    use {Attribute, InnerClass, LineNumber, LocalVariableTable, LocalVariableTypeTable};

    fn verify(
        class_file: &ClassFile<'_>,
        attributes: &[Attribute],
        context: AttributeContext,
    ) -> Result<()> {
        let location = context.location();
        for (index, attribute) in attributes.iter().enumerate() {
            if location.recognizes(attribute.name().as_bytes()) {
                verify_attribute(class_file, attribute, &attributes[..index], context, None)?;
            }
        }
        Ok(())
    }

    fn create_class_file(constants: Vec<Constant<'static>>) -> ClassFile<'static> {
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

        assert_eq!(
            Ok(()),
            verify(
                &class_file,
                std::slice::from_ref(&attribute),
                AttributeContext::Class,
            )
        );

        let attribute_invalid_index = Attribute::ConstantValue {
            name_index: 0,
            constant_value_index: 2,
        };
        assert_eq!(
            Err(InvalidConstantPoolIndex(2)),
            verify(&class_file, &[attribute_invalid_index], context)
        );

        let constants_invalid = vec![Constant::Class(1)];
        let class_file_invalid = create_class_file(constants_invalid);
        assert_eq!(
            Err(InvalidConstantPoolIndexType(1)),
            verify(&class_file_invalid, &[attribute], context)
        );
    }

    #[test]
    fn test_verify_code() {
        let constants = vec![
            Constant::utf8("()V"),
            Constant::utf8("Code"),
            Constant::utf8("(J)V"),
            Constant::utf8("LineNumberTable"),
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
            code: vec![Instruction::Return],
            exception_table: vec![],
            attributes: vec![],
        };
        let context = AttributeContext::Method(&method);

        assert!(verify(&class_file, std::slice::from_ref(&attribute), context).is_ok());

        let invalid_locals_method = Method {
            descriptor_index: 3,
            ..Default::default()
        };
        assert!(
            verify(
                &class_file,
                std::slice::from_ref(&attribute),
                AttributeContext::Method(&invalid_locals_method),
            )
            .is_err()
        );

        assert!(matches!(
            verify(
                &class_file,
                std::slice::from_ref(&attribute),
                AttributeContext::Class
            ),
            Ok(())
        ));

        let line_number_table = Attribute::LineNumberTable {
            name_index: 4,
            line_numbers: vec![],
        };
        let attribute_recursive = Attribute::Code {
            name_index: 2,
            max_stack: 0,
            max_locals: 1,
            code: vec![Instruction::Return],
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
            Ok(())
        ));
    }

    #[test]
    fn test_verify_exceptions() {
        let constants = vec![Constant::Class(2), Constant::utf8("Exception")];
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
            Ok(())
        ));

        let attribute_invalid_index = Attribute::Exceptions {
            name_index: 0,
            exception_indexes: vec![3],
        };
        assert!(matches!(
            verify(&class_file, &[attribute_invalid_index], context),
            Err(InvalidConstantPoolIndex(3))
        ));

        let attribute_invalid_type = Attribute::Exceptions {
            name_index: 0,
            exception_indexes: vec![2],
        };
        assert!(matches!(
            verify(&class_file, &[attribute_invalid_type], context),
            Err(InvalidConstantPoolIndexType(2))
        ));
    }

    #[test]
    fn test_verify_inner_classes() {
        let constants = vec![
            Constant::Class(2),
            Constant::utf8("Inner"),
            Constant::Class(4),
            Constant::utf8("Outer"),
            Constant::utf8("Name"),
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
            Ok(())
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
            Err(InvalidConstantPoolIndexType(5))
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
            Err(InvalidConstantPoolIndexType(5))
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
            Err(InvalidConstantPoolIndexType(1))
        ));
    }

    #[test]
    fn test_verify_enclosing_method() {
        let constants = vec![
            Constant::Class(2),
            Constant::utf8("Class"),
            Constant::NameAndType {
                name_index: 4,
                descriptor_index: 5,
            },
            Constant::utf8("method"),
            Constant::utf8("()V"),
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
            Ok(())
        ));

        let attribute_invalid_class = Attribute::EnclosingMethod {
            name_index: 0,
            class_index: 3,
            method_index: 3,
        };
        assert!(matches!(
            verify(&class_file, &[attribute_invalid_class], context),
            Err(InvalidConstantPoolIndexType(3))
        ));

        let attribute_invalid_method = Attribute::EnclosingMethod {
            name_index: 0,
            class_index: 1,
            method_index: 1,
        };
        assert!(matches!(
            verify(&class_file, &[attribute_invalid_method], context),
            Err(InvalidConstantPoolIndexType(1))
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
            Ok(())
        ));
    }

    #[test]
    fn test_verify_signature() {
        let constants = vec![Constant::utf8("Signature")];
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
            Ok(())
        ));

        let attribute_invalid = Attribute::Signature {
            name_index: 0,
            signature_index: 2,
        };
        assert!(matches!(
            verify(&class_file, &[attribute_invalid], context),
            Err(InvalidConstantPoolIndex(2))
        ));
    }

    #[test]
    fn test_verify_source_file() {
        let constants = vec![Constant::utf8("Source.java")];
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
            Ok(())
        ));

        let attribute_invalid = Attribute::SourceFile {
            name_index: 0,
            source_file_index: 2,
        };
        assert!(matches!(
            verify(&class_file, &[attribute_invalid], context),
            Err(InvalidConstantPoolIndex(2))
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
            Ok(())
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

        // A misplaced LineNumberTable is ignored at method level.
        assert!(
            verify(
                &ClassFile::default(),
                std::slice::from_ref(&attribute),
                AttributeContext::Method(&method)
            )
            .is_ok()
        );

        // A misplaced debug attribute is also ignored at class level.
        assert!(matches!(
            verify(
                &ClassFile::default(),
                std::slice::from_ref(&attribute),
                AttributeContext::Class
            ),
            Ok(())
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
            Err(VerificationError { .. })
        ));
    }

    #[test]
    fn test_verify_local_variable_table() {
        let constants = vec![Constant::utf8("name"), Constant::utf8("descriptor")];
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

        // A misplaced LocalVariableTable is ignored at method level.
        assert!(
            verify(
                &class_file,
                std::slice::from_ref(&attribute),
                AttributeContext::Method(&method)
            )
            .is_ok()
        );

        // A misplaced debug attribute is also ignored at class level.
        assert!(matches!(
            verify(
                &class_file,
                std::slice::from_ref(&attribute),
                AttributeContext::Class
            ),
            Ok(())
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
            Err(VerificationError { .. })
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
            Err(InvalidConstantPoolIndex(3))
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
            Err(InvalidConstantPoolIndex(3))
        ));
    }

    #[test]
    fn test_verify_local_variable_type_table() {
        let constants = vec![Constant::utf8("name"), Constant::utf8("signature")];
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

        // A misplaced LocalVariableTypeTable is ignored at method level.
        assert!(
            verify(
                &class_file,
                std::slice::from_ref(&attribute),
                AttributeContext::Method(&method)
            )
            .is_ok()
        );

        // A misplaced debug attribute is also ignored at class level.
        assert!(matches!(
            verify(
                &class_file,
                std::slice::from_ref(&attribute),
                AttributeContext::Class
            ),
            Ok(())
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
            Err(VerificationError { .. })
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
            Err(InvalidConstantPoolIndex(3))
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
            Err(InvalidConstantPoolIndex(3))
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
            Ok(())
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
            Constant::utf8("LAnnotation;"),
            Constant::utf8("name"),
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
            Err(InvalidConstantPoolIndexType(3))
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
            Err(InvalidConstantPoolIndexType(3))
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
            Err(InvalidConstantPoolIndexType(1))
        ));
    }

    #[test]
    fn test_verify_annotation_nested_invalid_values() {
        let class_file = create_class_file(vec![
            Constant::utf8("LAnnotation;"),
            Constant::utf8("name"),
            Constant::Integer(42),
        ]);
        let annotation = Annotation {
            type_index: 1,
            elements: Vec::new(),
        };
        let context = AttributeContext::Class;
        let nested_invalid_value = Annotation {
            elements: vec![AnnotationValuePair {
                name_index: 2,
                value: AnnotationElement::Annotation {
                    annotation: Annotation {
                        type_index: 3,
                        elements: Vec::new(),
                    },
                },
            }],
            ..annotation.clone()
        };
        assert!(matches!(
            verify(
                &class_file,
                &[Attribute::RuntimeVisibleAnnotations {
                    name_index: 0,
                    annotations: vec![nested_invalid_value],
                }],
                context,
            ),
            Err(InvalidConstantPoolIndexType(3))
        ));

        let array_invalid_value = Annotation {
            elements: vec![AnnotationValuePair {
                name_index: 2,
                value: AnnotationElement::Array {
                    values: vec![AnnotationElement::Int {
                        const_value_index: 1,
                    }],
                },
            }],
            ..annotation
        };
        assert!(matches!(
            verify(
                &class_file,
                &[Attribute::RuntimeVisibleAnnotations {
                    name_index: 0,
                    annotations: vec![array_invalid_value],
                }],
                context,
            ),
            Err(InvalidConstantPoolIndexType(1))
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
            Ok(())
        ));

        let class_file = create_class_file(vec![
            Constant::utf8("LAnnotation;"),
            Constant::utf8("name"),
            Constant::Integer(42),
        ]);
        let invalid_parameter_value = Attribute::RuntimeVisibleParameterAnnotations {
            name_index: 0,
            parameter_annotations: vec![ParameterAnnotation {
                annotations: vec![Annotation {
                    type_index: 1,
                    elements: vec![AnnotationValuePair {
                        name_index: 2,
                        value: AnnotationElement::Int {
                            const_value_index: 1,
                        },
                    }],
                }],
            }],
        };
        assert!(matches!(
            verify(&class_file, &[invalid_parameter_value], context),
            Err(InvalidConstantPoolIndexType(1))
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
            Ok(())
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
            Ok(())
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
            Err(InvalidConstantPoolIndexType(2))
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
            Err(InvalidConstantPoolIndex(3))
        ));
    }

    #[test]
    fn test_verify_method_parameters() {
        let constants = vec![Constant::utf8("param")];
        let class_file = create_class_file(constants);
        let attribute = Attribute::MethodParameters {
            name_index: 0,
            parameters: vec![MethodParameter {
                name_index: 1,
                access_flags: MethodParameterAccessFlags::empty(),
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
            Ok(())
        ));

        let attribute_invalid = Attribute::MethodParameters {
            name_index: 0,
            parameters: vec![MethodParameter {
                name_index: 2,
                access_flags: MethodParameterAccessFlags::empty(),
            }],
        };
        assert!(matches!(
            verify(&class_file, &[attribute_invalid], context),
            Err(InvalidConstantPoolIndex(2))
        ));
    }

    #[test]
    fn test_verify_optional_zero_indexes() {
        let class_file = create_class_file(vec![
            Constant::Class(2),
            Constant::utf8("Inner"),
            Constant::Module(4),
            Constant::utf8("module"),
        ]);
        let class_context = AttributeContext::Class;

        assert!(
            verify(
                &class_file,
                &[Attribute::InnerClasses {
                    name_index: 0,
                    classes: vec![InnerClass {
                        class_info_index: 1,
                        outer_class_info_index: 0,
                        name_index: 0,
                        access_flags: NestedClassAccessFlags::empty(),
                    }],
                }],
                class_context,
            )
            .is_ok()
        );

        let method = Method::default();
        assert!(
            verify(
                &class_file,
                &[Attribute::MethodParameters {
                    name_index: 0,
                    parameters: vec![MethodParameter {
                        name_index: 0,
                        access_flags: MethodParameterAccessFlags::empty(),
                    }],
                }],
                AttributeContext::Method(&method),
            )
            .is_ok()
        );

        assert!(
            verify(
                &class_file,
                &[Attribute::Module {
                    name_index: 0,
                    module_name_index: 3,
                    flags: ModuleAccessFlags::empty(),
                    version_index: 0,
                    requires: vec![Requires {
                        index: 3,
                        flags: RequiresFlags::empty(),
                        version_index: 0,
                    }],
                    exports: Vec::new(),
                    opens: Vec::new(),
                    uses: Vec::new(),
                    provides: Vec::new(),
                }],
                class_context,
            )
            .is_ok()
        );
    }

    #[test]
    fn test_verify_module() {
        let constants = vec![
            Constant::Module(2),
            Constant::utf8("module"),
            Constant::utf8("1.0"),
            Constant::Package(5),
            Constant::utf8("package"),
            Constant::Class(7),
            Constant::utf8("Class"),
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
            Ok(())
        ));
    }

    #[test]
    fn test_verify_module_packages() {
        let constants = vec![Constant::Package(2), Constant::utf8("package")];
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
            Ok(())
        ));
    }

    #[test]
    fn test_verify_module_main_class() {
        let constants = vec![Constant::Class(2), Constant::utf8("Main")];
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
            Ok(())
        ));
    }

    #[test]
    fn test_verify_nest_host() {
        let constants = vec![Constant::Class(2), Constant::utf8("Host")];
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
            Ok(())
        ));
    }

    #[test]
    fn test_verify_nest_members() {
        let constants = vec![Constant::Class(2), Constant::utf8("Member")];
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
            Ok(())
        ));
    }

    #[test]
    fn test_verify_record() {
        let constants = vec![Constant::utf8("name"), Constant::utf8("I")];
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
            Ok(())
        ));
    }

    #[test]
    fn test_verify_permitted_subclasses() {
        let constants = vec![Constant::Class(2), Constant::utf8("Sub")];
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
            Ok(())
        ));
    }

    #[test]
    fn test_verify_inner_class_enclosing_invalid_paths() {
        let class_context = AttributeContext::Class;

        let empty_class = create_class_file(Vec::new());
        let class_file = create_class_file(vec![
            Constant::Class(2),
            Constant::utf8("Class"),
            Constant::NameAndType {
                name_index: 2,
                descriptor_index: 2,
            },
        ]);

        let missing_inner_class = Attribute::InnerClasses {
            name_index: 0,
            classes: vec![InnerClass {
                class_info_index: 1,
                outer_class_info_index: 0,
                name_index: 0,
                access_flags: NestedClassAccessFlags::empty(),
            }],
        };
        assert!(matches!(
            verify(&empty_class, &[missing_inner_class], class_context),
            Err(InvalidConstantPoolIndex(1))
        ));

        let missing_outer_class = Attribute::InnerClasses {
            name_index: 0,
            classes: vec![InnerClass {
                class_info_index: 1,
                outer_class_info_index: 4,
                name_index: 0,
                access_flags: NestedClassAccessFlags::empty(),
            }],
        };
        assert!(matches!(
            verify(&class_file, &[missing_outer_class], class_context),
            Err(InvalidConstantPoolIndex(4))
        ));

        let missing_inner_name = Attribute::InnerClasses {
            name_index: 0,
            classes: vec![InnerClass {
                class_info_index: 1,
                outer_class_info_index: 0,
                name_index: 4,
                access_flags: NestedClassAccessFlags::empty(),
            }],
        };
        assert!(matches!(
            verify(&class_file, &[missing_inner_name], class_context),
            Err(InvalidConstantPoolIndex(4))
        ));

        assert!(matches!(
            verify(
                &empty_class,
                &[Attribute::EnclosingMethod {
                    name_index: 0,
                    class_index: 1,
                    method_index: 0,
                }],
                class_context,
            ),
            Err(InvalidConstantPoolIndex(1))
        ));
        assert!(matches!(
            verify(
                &class_file,
                &[Attribute::EnclosingMethod {
                    name_index: 0,
                    class_index: 1,
                    method_index: 4,
                }],
                class_context,
            ),
            Err(InvalidConstantPoolIndex(4))
        ));
    }

    #[test]
    fn test_verify_signature_source_local_var_invalid_paths() {
        let method = Method::default();
        let class_context = AttributeContext::Class;
        let code_context = AttributeContext::Code(&method, 10);
        let int_class = create_class_file(vec![Constant::Integer(1), Constant::utf8("ok")]);

        assert!(matches!(
            verify(
                &int_class,
                &[Attribute::Signature {
                    name_index: 0,
                    signature_index: 1,
                }],
                class_context,
            ),
            Err(InvalidConstantPoolIndexType(1))
        ));
        assert!(matches!(
            verify(
                &int_class,
                &[Attribute::SourceFile {
                    name_index: 0,
                    source_file_index: 1,
                }],
                class_context,
            ),
            Err(InvalidConstantPoolIndexType(1))
        ));

        assert!(matches!(
            verify(
                &int_class,
                &[Attribute::LocalVariableTable {
                    name_index: 0,
                    variables: vec![LocalVariableTable {
                        start_pc: 0,
                        length: 1,
                        name_index: 1,
                        descriptor_index: 2,
                        index: 0,
                    }],
                }],
                code_context,
            ),
            Err(InvalidConstantPoolIndexType(1))
        ));
        assert!(matches!(
            verify(
                &int_class,
                &[Attribute::LocalVariableTable {
                    name_index: 0,
                    variables: vec![LocalVariableTable {
                        start_pc: 0,
                        length: 1,
                        name_index: 2,
                        descriptor_index: 1,
                        index: 0,
                    }],
                }],
                code_context,
            ),
            Err(InvalidConstantPoolIndexType(1))
        ));

        assert!(matches!(
            verify(
                &int_class,
                &[Attribute::LocalVariableTypeTable {
                    name_index: 0,
                    variable_types: vec![LocalVariableTypeTable {
                        start_pc: 0,
                        length: 1,
                        name_index: 1,
                        signature_index: 2,
                        index: 0,
                    }],
                }],
                code_context,
            ),
            Err(InvalidConstantPoolIndexType(1))
        ));
        assert!(matches!(
            verify(
                &int_class,
                &[Attribute::LocalVariableTypeTable {
                    name_index: 0,
                    variable_types: vec![LocalVariableTypeTable {
                        start_pc: 0,
                        length: 1,
                        name_index: 2,
                        signature_index: 1,
                        index: 0,
                    }],
                }],
                code_context,
            ),
            Err(InvalidConstantPoolIndexType(1))
        ));
    }

    #[test]
    fn test_verify_annotation_element_scalar_invalid_paths() {
        let empty_class = create_class_file(Vec::new());
        let int_class = create_class_file(vec![Constant::Integer(1), Constant::utf8("ok")]);

        assert!(matches!(
            verify_annotation_element(
                &empty_class,
                &AnnotationElement::Int {
                    const_value_index: 1
                }
            ),
            Err(InvalidConstantPoolIndex(1))
        ));
        assert!(matches!(
            verify_annotation_element(
                &int_class,
                &AnnotationElement::Double {
                    const_value_index: 2
                }
            ),
            Err(InvalidConstantPoolIndexType(2))
        ));
        assert!(matches!(
            verify_annotation_element(
                &empty_class,
                &AnnotationElement::Double {
                    const_value_index: 1
                }
            ),
            Err(InvalidConstantPoolIndex(1))
        ));
        assert!(matches!(
            verify_annotation_element(
                &int_class,
                &AnnotationElement::Float {
                    const_value_index: 2
                }
            ),
            Err(InvalidConstantPoolIndexType(2))
        ));
        assert!(matches!(
            verify_annotation_element(
                &empty_class,
                &AnnotationElement::Float {
                    const_value_index: 1
                }
            ),
            Err(InvalidConstantPoolIndex(1))
        ));
        assert!(matches!(
            verify_annotation_element(
                &int_class,
                &AnnotationElement::Long {
                    const_value_index: 2
                }
            ),
            Err(InvalidConstantPoolIndexType(2))
        ));
        assert!(matches!(
            verify_annotation_element(
                &empty_class,
                &AnnotationElement::Long {
                    const_value_index: 1
                }
            ),
            Err(InvalidConstantPoolIndex(1))
        ));
        assert!(matches!(
            verify_annotation_element(
                &int_class,
                &AnnotationElement::String {
                    const_value_index: 1
                }
            ),
            Err(InvalidConstantPoolIndexType(1))
        ));
        assert!(matches!(
            verify_annotation_element(
                &empty_class,
                &AnnotationElement::String {
                    const_value_index: 1
                }
            ),
            Err(InvalidConstantPoolIndex(1))
        ));
    }

    #[test]
    fn test_verify_annotation_element_enum_class_invalid_paths() {
        let empty_class = create_class_file(Vec::new());
        let int_class = create_class_file(vec![Constant::Integer(1), Constant::utf8("ok")]);

        assert!(matches!(
            verify_annotation_element(
                &int_class,
                &AnnotationElement::Enum {
                    type_name_index: 1,
                    const_name_index: 2,
                },
            ),
            Err(InvalidConstantPoolIndexType(1))
        ));
        assert!(matches!(
            verify_annotation_element(
                &empty_class,
                &AnnotationElement::Enum {
                    type_name_index: 1,
                    const_name_index: 1,
                },
            ),
            Err(InvalidConstantPoolIndex(1))
        ));
        assert!(matches!(
            verify_annotation_element(
                &int_class,
                &AnnotationElement::Enum {
                    type_name_index: 2,
                    const_name_index: 1,
                },
            ),
            Err(InvalidConstantPoolIndexType(1))
        ));
        assert!(matches!(
            verify_annotation_element(
                &create_class_file(vec![Constant::utf8("Enum")]),
                &AnnotationElement::Enum {
                    type_name_index: 1,
                    const_name_index: 2,
                },
            ),
            Err(InvalidConstantPoolIndex(2))
        ));
        assert!(matches!(
            verify_annotation_element(
                &int_class,
                &AnnotationElement::Class {
                    class_info_index: 1
                }
            ),
            Err(InvalidConstantPoolIndexType(1))
        ));
        assert!(matches!(
            verify_annotation_element(
                &empty_class,
                &AnnotationElement::Class {
                    class_info_index: 1
                }
            ),
            Err(InvalidConstantPoolIndex(1))
        ));
    }

    #[test]
    #[expect(
        clippy::too_many_lines,
        reason = "covers invalid annotation payloads in a valid field context"
    )]
    fn test_verify_type_annotation_invalid_paths() {
        let field = Field {
            access_flags: FieldAccessFlags::empty(),
            name_index: 0,
            descriptor_index: 0,
            field_type: FieldType::Base(BaseType::Int),
            attributes: vec![],
        };
        let class_context = AttributeContext::Field(&field);
        let empty_class = create_class_file(Vec::new());
        let int_class = create_class_file(vec![Constant::Integer(1), Constant::utf8("ok")]);

        let bad_type_annotation = TypeAnnotation {
            target_type: TargetType::Empty { target_type: 0x13 },
            type_path: Vec::new(),
            type_index: 1,
            elements: Vec::new(),
        };
        assert!(matches!(
            verify(
                &int_class,
                &[Attribute::RuntimeVisibleTypeAnnotations {
                    name_index: 0,
                    type_annotations: vec![bad_type_annotation.clone()],
                }],
                class_context,
            ),
            Err(InvalidConstantPoolIndexType(1))
        ));
        assert!(matches!(
            verify(
                &empty_class,
                &[Attribute::RuntimeVisibleTypeAnnotations {
                    name_index: 0,
                    type_annotations: vec![bad_type_annotation.clone()],
                }],
                class_context,
            ),
            Err(InvalidConstantPoolIndex(1))
        ));

        let bad_pair_name = TypeAnnotation {
            type_index: 2,
            elements: vec![AnnotationValuePair {
                name_index: 1,
                value: AnnotationElement::Int {
                    const_value_index: 1,
                },
            }],
            ..bad_type_annotation
        };
        assert!(matches!(
            verify(
                &int_class,
                &[Attribute::RuntimeVisibleTypeAnnotations {
                    name_index: 0,
                    type_annotations: vec![bad_pair_name],
                }],
                class_context,
            ),
            Err(InvalidConstantPoolIndexType(1))
        ));

        let missing_pair_name = TypeAnnotation {
            target_type: TargetType::Empty { target_type: 0x13 },
            type_path: Vec::new(),
            type_index: 1,
            elements: vec![AnnotationValuePair {
                name_index: 2,
                value: AnnotationElement::Int {
                    const_value_index: 1,
                },
            }],
        };
        assert!(matches!(
            verify(
                &create_class_file(vec![Constant::utf8("Type")]),
                &[Attribute::RuntimeVisibleTypeAnnotations {
                    name_index: 0,
                    type_annotations: vec![missing_pair_name],
                }],
                class_context,
            ),
            Err(InvalidConstantPoolIndex(2))
        ));

        let bad_pair_value = TypeAnnotation {
            target_type: TargetType::Empty { target_type: 0x13 },
            type_path: Vec::new(),
            type_index: 2,
            elements: vec![AnnotationValuePair {
                name_index: 2,
                value: AnnotationElement::Int {
                    const_value_index: 2,
                },
            }],
        };
        assert!(matches!(
            verify(
                &int_class,
                &[Attribute::RuntimeVisibleTypeAnnotations {
                    name_index: 0,
                    type_annotations: vec![bad_pair_value],
                }],
                class_context,
            ),
            Err(InvalidConstantPoolIndexType(2))
        ));
    }

    struct AnnotationTestFixture {
        class_file: ClassFile<'static>,
        method: Method,
        annotation: Annotation,
        element_name: u16,
        int_value: u16,
        class_name: u16,
        module_name: u16,
        module_version: u16,
        package_name: u16,
        nested: Annotation,
    }

    fn annotation_value(name_index: u16, value: AnnotationElement) -> AnnotationValuePair {
        AnnotationValuePair { name_index, value }
    }

    fn annotation_elements(
        element_name: u16,
        int_value: u16,
        double_value: u16,
        float_value: u16,
        long_value: u16,
        string_value: u16,
        nested: &Annotation,
    ) -> Vec<AnnotationValuePair> {
        vec![
            annotation_value(
                element_name,
                AnnotationElement::Byte {
                    const_value_index: int_value,
                },
            ),
            annotation_value(
                element_name,
                AnnotationElement::Char {
                    const_value_index: int_value,
                },
            ),
            annotation_value(
                element_name,
                AnnotationElement::Short {
                    const_value_index: int_value,
                },
            ),
            annotation_value(
                element_name,
                AnnotationElement::Boolean {
                    const_value_index: int_value,
                },
            ),
            annotation_value(
                element_name,
                AnnotationElement::Double {
                    const_value_index: double_value,
                },
            ),
            annotation_value(
                element_name,
                AnnotationElement::Float {
                    const_value_index: float_value,
                },
            ),
            annotation_value(
                element_name,
                AnnotationElement::Long {
                    const_value_index: long_value,
                },
            ),
            annotation_value(
                element_name,
                AnnotationElement::String {
                    const_value_index: string_value,
                },
            ),
            annotation_value(
                element_name,
                AnnotationElement::Enum {
                    type_name_index: string_value,
                    const_name_index: element_name,
                },
            ),
            annotation_value(
                element_name,
                AnnotationElement::Class {
                    class_info_index: string_value,
                },
            ),
            annotation_value(
                element_name,
                AnnotationElement::Annotation {
                    annotation: nested.clone(),
                },
            ),
            annotation_value(
                element_name,
                AnnotationElement::Array {
                    values: vec![AnnotationElement::Int {
                        const_value_index: int_value,
                    }],
                },
            ),
        ]
    }

    fn create_annotation_test_fixture() -> AnnotationTestFixture {
        let mut constant_pool = ConstantPool::default();
        let annotation_type = constant_pool.add_utf8("LAnnotation;").unwrap();
        let element_name = constant_pool.add_utf8("value").unwrap();
        let int_value = constant_pool.add(Constant::Integer(1)).unwrap();
        let double_value = constant_pool.add(Constant::Double(2.0)).unwrap();
        let float_value = constant_pool.add(Constant::Float(3.0)).unwrap();
        let long_value = constant_pool.add(Constant::Long(4)).unwrap();
        let string_value = constant_pool.add_utf8("text").unwrap();
        let module_name = constant_pool.add_module("module").unwrap();
        let module_version = constant_pool.add_utf8("1.0").unwrap();
        let package_name = constant_pool.add_package("package").unwrap();
        let class_name = constant_pool.add_class("Outer$Inner").unwrap();
        let method_name = constant_pool.add_utf8("method").unwrap();
        let method_descriptor = constant_pool.add_utf8("()V").unwrap();
        let class_file = ClassFile {
            constant_pool,
            ..Default::default()
        };
        let method = Method {
            name_index: method_name,
            descriptor_index: method_descriptor,
            ..Default::default()
        };
        let nested = Annotation {
            type_index: annotation_type,
            elements: Vec::new(),
        };
        let annotation = Annotation {
            type_index: annotation_type,
            elements: annotation_elements(
                element_name,
                int_value,
                double_value,
                float_value,
                long_value,
                string_value,
                &nested,
            ),
        };
        AnnotationTestFixture {
            class_file,
            method,
            annotation,
            element_name,
            int_value,
            class_name,
            module_name,
            module_version,
            package_name,
            nested,
        }
    }

    #[test]
    fn test_verify_valid_annotation_attribute_paths() {
        let AnnotationTestFixture {
            class_file,
            method,
            annotation,
            nested,
            element_name,
            int_value,
            ..
        } = create_annotation_test_fixture();

        assert!(
            verify(
                &class_file,
                &[Attribute::RuntimeInvisibleAnnotations {
                    name_index: 0,
                    annotations: vec![annotation.clone()],
                }],
                AttributeContext::Class,
            )
            .is_ok()
        );
        assert!(
            verify(
                &class_file,
                &[Attribute::RuntimeInvisibleParameterAnnotations {
                    name_index: 0,
                    parameter_annotations: vec![ParameterAnnotation {
                        annotations: vec![annotation.clone()],
                    }],
                }],
                AttributeContext::Method(&method),
            )
            .is_ok()
        );
        assert!(
            verify(
                &class_file,
                &[Attribute::RuntimeInvisibleTypeAnnotations {
                    name_index: 0,
                    type_annotations: vec![TypeAnnotation {
                        target_type: TargetType::Offset {
                            target_type: 0x43,
                            offset: 0
                        },
                        type_path: Vec::new(),
                        type_index: annotation.type_index,
                        elements: vec![AnnotationValuePair {
                            name_index: element_name,
                            value: AnnotationElement::Int {
                                const_value_index: int_value,
                            },
                        }],
                    }],
                }],
                AttributeContext::Code(&method, 1),
            )
            .is_ok()
        );
        assert!(
            verify(
                &class_file,
                &[Attribute::AnnotationDefault {
                    name_index: 0,
                    element: AnnotationElement::Annotation { annotation: nested },
                }],
                AttributeContext::Method(&method),
            )
            .is_ok()
        );
    }

    #[test]
    fn test_verify_valid_method_class_module_paths() {
        let AnnotationTestFixture {
            class_file,
            method,
            element_name,
            class_name,
            module_name,
            module_version,
            package_name,
            ..
        } = create_annotation_test_fixture();

        assert!(
            verify(
                &class_file,
                &[Attribute::MethodParameters {
                    name_index: 0,
                    parameters: vec![MethodParameter {
                        name_index: element_name,
                        access_flags: MethodParameterAccessFlags::empty(),
                    }],
                }],
                AttributeContext::Method(&method),
            )
            .is_ok()
        );
        assert!(
            verify(
                &class_file,
                &[Attribute::InnerClasses {
                    name_index: 0,
                    classes: vec![InnerClass {
                        class_info_index: class_name,
                        outer_class_info_index: class_name,
                        name_index: element_name,
                        access_flags: NestedClassAccessFlags::empty(),
                    }],
                }],
                AttributeContext::Class,
            )
            .is_ok()
        );
        assert!(
            verify(
                &class_file,
                &[Attribute::EnclosingMethod {
                    name_index: 0,
                    class_index: class_name,
                    method_index: 0,
                }],
                AttributeContext::Class,
            )
            .is_ok()
        );
        assert!(
            verify(
                &class_file,
                &[Attribute::Module {
                    name_index: 0,
                    module_name_index: module_name,
                    flags: ModuleAccessFlags::empty(),
                    version_index: module_version,
                    requires: vec![Requires {
                        index: module_name,
                        flags: RequiresFlags::empty(),
                        version_index: module_version,
                    }],
                    exports: vec![Exports {
                        index: package_name,
                        flags: ExportsFlags::empty(),
                        to_index: Vec::new(),
                    }],
                    opens: Vec::new(),
                    uses: Vec::new(),
                    provides: Vec::new(),
                }],
                AttributeContext::Class,
            )
            .is_ok()
        );
    }

    fn module_attribute(
        module_name_index: u16,
        version_index: u16,
        requires: Vec<Requires>,
        exports: Vec<Exports>,
        opens: Vec<Opens>,
        uses: Vec<u16>,
        provides: Vec<Provides>,
    ) -> Attribute {
        Attribute::Module {
            name_index: 0,
            module_name_index,
            flags: ModuleAccessFlags::empty(),
            version_index,
            requires,
            exports,
            opens,
            uses,
            provides,
        }
    }

    fn module_edge_class_file() -> ClassFile<'static> {
        create_class_file(vec![
            Constant::Module(2),
            Constant::utf8("module"),
            Constant::utf8("1.0"),
            Constant::Package(5),
            Constant::utf8("package"),
            Constant::Class(7),
            Constant::utf8("Class"),
        ])
    }

    fn attribute_edge_class_file() -> ClassFile<'static> {
        create_class_file(vec![
            Constant::Module(2),
            Constant::utf8("module"),
            Constant::utf8("1.0"),
            Constant::Package(5),
            Constant::utf8("package"),
            Constant::Class(7),
            Constant::utf8("Class"),
            Constant::Integer(42),
        ])
    }

    fn module_with_edges(opens: Vec<Opens>, uses: Vec<u16>, provides: Vec<Provides>) -> Attribute {
        module_attribute(1, 0, Vec::new(), Vec::new(), opens, uses, provides)
    }

    #[test]
    fn test_verify_module_name_version_invalid_paths() {
        let class_context = AttributeContext::Class;
        let class_file = module_edge_class_file();

        assert!(matches!(
            verify(
                &class_file,
                &[module_attribute(
                    2,
                    0,
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    Vec::new()
                )],
                class_context,
            ),
            Err(InvalidConstantPoolIndexType(2))
        ));
        assert!(matches!(
            verify(
                &class_file,
                &[module_attribute(
                    8,
                    0,
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    Vec::new()
                )],
                class_context,
            ),
            Err(InvalidConstantPoolIndex(8))
        ));
        assert!(matches!(
            verify(
                &class_file,
                &[module_attribute(
                    1,
                    1,
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    Vec::new()
                )],
                class_context,
            ),
            Err(InvalidConstantPoolIndexType(1))
        ));
        assert!(matches!(
            verify(
                &class_file,
                &[module_attribute(
                    1,
                    8,
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    Vec::new()
                )],
                class_context,
            ),
            Err(InvalidConstantPoolIndex(8))
        ));
    }

    #[test]
    fn test_verify_module_requires_invalid_paths() {
        let class_context = AttributeContext::Class;
        let class_file = create_class_file(vec![
            Constant::Module(2),
            Constant::utf8("module"),
            Constant::utf8("1.0"),
            Constant::Package(5),
            Constant::utf8("package"),
            Constant::Class(7),
            Constant::utf8("Class"),
        ]);

        assert!(matches!(
            verify(
                &class_file,
                &[module_attribute(
                    1,
                    0,
                    vec![Requires {
                        index: 2,
                        flags: RequiresFlags::empty(),
                        version_index: 0,
                    }],
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                )],
                class_context,
            ),
            Err(InvalidConstantPoolIndexType(2))
        ));
        assert!(matches!(
            verify(
                &class_file,
                &[module_attribute(
                    1,
                    0,
                    vec![Requires {
                        index: 8,
                        flags: RequiresFlags::empty(),
                        version_index: 0,
                    }],
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                )],
                class_context,
            ),
            Err(InvalidConstantPoolIndex(8))
        ));
        assert!(matches!(
            verify(
                &class_file,
                &[module_attribute(
                    1,
                    0,
                    vec![Requires {
                        index: 1,
                        flags: RequiresFlags::empty(),
                        version_index: 1,
                    }],
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                )],
                class_context,
            ),
            Err(InvalidConstantPoolIndexType(1))
        ));
        assert!(matches!(
            verify(
                &class_file,
                &[module_attribute(
                    1,
                    0,
                    vec![Requires {
                        index: 1,
                        flags: RequiresFlags::empty(),
                        version_index: 8,
                    }],
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                )],
                class_context,
            ),
            Err(InvalidConstantPoolIndex(8))
        ));
    }

    #[test]
    fn test_verify_module_exports_invalid_paths() {
        let class_context = AttributeContext::Class;
        let class_file = module_edge_class_file();

        assert!(matches!(
            verify(
                &class_file,
                &[module_attribute(
                    1,
                    0,
                    Vec::new(),
                    vec![Exports {
                        index: 1,
                        flags: ExportsFlags::empty(),
                        to_index: Vec::new(),
                    }],
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                )],
                class_context,
            ),
            Err(InvalidConstantPoolIndexType(1))
        ));
        assert!(matches!(
            verify(
                &class_file,
                &[module_attribute(
                    1,
                    0,
                    Vec::new(),
                    vec![Exports {
                        index: 8,
                        flags: ExportsFlags::empty(),
                        to_index: Vec::new(),
                    }],
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                )],
                class_context,
            ),
            Err(InvalidConstantPoolIndex(8))
        ));
        assert!(matches!(
            verify(
                &class_file,
                &[module_attribute(
                    1,
                    0,
                    Vec::new(),
                    vec![Exports {
                        index: 4,
                        flags: ExportsFlags::empty(),
                        to_index: vec![2],
                    }],
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                )],
                class_context,
            ),
            Err(InvalidConstantPoolIndexType(2))
        ));
        assert!(matches!(
            verify(
                &class_file,
                &[module_attribute(
                    1,
                    0,
                    Vec::new(),
                    vec![Exports {
                        index: 4,
                        flags: ExportsFlags::empty(),
                        to_index: vec![8],
                    }],
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                )],
                class_context,
            ),
            Err(InvalidConstantPoolIndex(8))
        ));
    }

    #[test]
    fn test_verify_module_uses_packages_main_class_invalid_paths() {
        let class_context = AttributeContext::Class;
        let class_file = module_edge_class_file();
        assert!(matches!(
            verify(
                &class_file,
                &[module_attribute(
                    1,
                    0,
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    vec![2],
                    Vec::new()
                )],
                class_context,
            ),
            Err(InvalidConstantPoolIndexType(2))
        ));
        assert!(matches!(
            verify(
                &class_file,
                &[module_attribute(
                    1,
                    0,
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    vec![8],
                    Vec::new()
                )],
                class_context,
            ),
            Err(InvalidConstantPoolIndex(8))
        ));

        assert!(matches!(
            verify(
                &class_file,
                &[Attribute::ModulePackages {
                    name_index: 0,
                    package_indexes: vec![2],
                }],
                class_context,
            ),
            Err(InvalidConstantPoolIndexType(2))
        ));
        assert!(matches!(
            verify(
                &class_file,
                &[Attribute::ModulePackages {
                    name_index: 0,
                    package_indexes: vec![8],
                }],
                class_context,
            ),
            Err(InvalidConstantPoolIndex(8))
        ));
        assert!(matches!(
            verify(
                &class_file,
                &[Attribute::ModuleMainClass {
                    name_index: 0,
                    main_class_index: 2,
                }],
                class_context,
            ),
            Err(InvalidConstantPoolIndexType(2))
        ));
        assert!(matches!(
            verify(
                &class_file,
                &[Attribute::ModuleMainClass {
                    name_index: 0,
                    main_class_index: 8,
                }],
                class_context,
            ),
            Err(InvalidConstantPoolIndex(8))
        ));
    }

    #[test]
    fn test_verify_annotation_and_bootstrap_index_edges() {
        let class_context = AttributeContext::Class;
        let class_file = attribute_edge_class_file();

        let annotation = |type_index, name_index| Annotation {
            type_index,
            elements: vec![AnnotationValuePair {
                name_index,
                value: AnnotationElement::Int {
                    const_value_index: 8,
                },
            }],
        };
        assert!(matches!(
            verify_annotations(&class_file, &[annotation(9, 2)]),
            Err(InvalidConstantPoolIndex(9))
        ));
        assert!(matches!(
            verify_annotations(&class_file, &[annotation(2, 9)]),
            Err(InvalidConstantPoolIndex(9))
        ));

        let missing_bootstrap = Attribute::BootstrapMethods {
            name_index: 0,
            methods: vec![BootstrapMethod {
                bootstrap_method_ref: 9,
                arguments: Vec::new(),
            }],
        };
        assert!(matches!(
            verify(&class_file, &[missing_bootstrap], class_context),
            Err(InvalidConstantPoolIndex(9))
        ));

        let method = Method::default();
        let bad_parameter = Attribute::MethodParameters {
            name_index: 0,
            parameters: vec![MethodParameter {
                name_index: 8,
                access_flags: MethodParameterAccessFlags::empty(),
            }],
        };
        assert!(matches!(
            verify(
                &class_file,
                &[bad_parameter],
                AttributeContext::Method(&method),
            ),
            Err(InvalidConstantPoolIndexType(8))
        ));
    }

    #[test]
    fn test_verify_module_opens_invalid_paths() {
        let class_context = AttributeContext::Class;
        let class_file = attribute_edge_class_file();

        assert!(matches!(
            verify(
                &class_file,
                &[module_with_edges(
                    vec![Opens {
                        index: 8,
                        flags: OpensFlags::empty(),
                        to_index: Vec::new(),
                    }],
                    Vec::new(),
                    Vec::new()
                )],
                class_context,
            ),
            Err(InvalidConstantPoolIndexType(8))
        ));
        assert!(matches!(
            verify(
                &class_file,
                &[module_with_edges(
                    vec![Opens {
                        index: 9,
                        flags: OpensFlags::empty(),
                        to_index: Vec::new(),
                    }],
                    Vec::new(),
                    Vec::new()
                )],
                class_context,
            ),
            Err(InvalidConstantPoolIndex(9))
        ));
        assert!(matches!(
            verify(
                &class_file,
                &[module_with_edges(
                    vec![Opens {
                        index: 4,
                        flags: OpensFlags::empty(),
                        to_index: vec![8],
                    }],
                    Vec::new(),
                    Vec::new()
                )],
                class_context,
            ),
            Err(InvalidConstantPoolIndexType(8))
        ));
        assert!(matches!(
            verify(
                &class_file,
                &[module_with_edges(
                    vec![Opens {
                        index: 4,
                        flags: OpensFlags::empty(),
                        to_index: vec![9],
                    }],
                    Vec::new(),
                    Vec::new()
                )],
                class_context,
            ),
            Err(InvalidConstantPoolIndex(9))
        ));
    }

    #[test]
    fn test_verify_module_provides_invalid_paths() {
        let class_context = AttributeContext::Class;
        let class_file = attribute_edge_class_file();

        assert!(matches!(
            verify(
                &class_file,
                &[module_with_edges(
                    Vec::new(),
                    Vec::new(),
                    vec![Provides {
                        index: 8,
                        with_index: Vec::new(),
                    }]
                )],
                class_context,
            ),
            Err(InvalidConstantPoolIndexType(8))
        ));
        assert!(matches!(
            verify(
                &class_file,
                &[module_with_edges(
                    Vec::new(),
                    Vec::new(),
                    vec![Provides {
                        index: 9,
                        with_index: Vec::new(),
                    }]
                )],
                class_context,
            ),
            Err(InvalidConstantPoolIndex(9))
        ));
        assert!(matches!(
            verify(
                &class_file,
                &[module_with_edges(
                    Vec::new(),
                    Vec::new(),
                    vec![Provides {
                        index: 6,
                        with_index: vec![8],
                    }]
                )],
                class_context,
            ),
            Err(InvalidConstantPoolIndexType(8))
        ));
        assert!(matches!(
            verify(
                &class_file,
                &[module_with_edges(
                    Vec::new(),
                    Vec::new(),
                    vec![Provides {
                        index: 6,
                        with_index: vec![9],
                    }]
                )],
                class_context,
            ),
            Err(InvalidConstantPoolIndex(9))
        ));
    }

    #[test]
    fn test_verify_nest_invalid_paths() {
        let class_context = AttributeContext::Class;
        let class_file = attribute_edge_class_file();

        assert!(matches!(
            verify(
                &class_file,
                &[Attribute::NestHost {
                    name_index: 0,
                    host_class_index: 8,
                }],
                class_context,
            ),
            Err(InvalidConstantPoolIndexType(8))
        ));
        assert!(matches!(
            verify(
                &class_file,
                &[Attribute::NestHost {
                    name_index: 0,
                    host_class_index: 9,
                }],
                class_context,
            ),
            Err(InvalidConstantPoolIndex(9))
        ));
        assert!(matches!(
            verify(
                &class_file,
                &[Attribute::NestMembers {
                    name_index: 0,
                    class_indexes: vec![8],
                }],
                class_context,
            ),
            Err(InvalidConstantPoolIndexType(8))
        ));
        assert!(matches!(
            verify(
                &class_file,
                &[Attribute::NestMembers {
                    name_index: 0,
                    class_indexes: vec![9],
                }],
                class_context,
            ),
            Err(InvalidConstantPoolIndex(9))
        ));
    }

    #[test]
    fn test_verify_record_invalid_paths() {
        let class_context = AttributeContext::Class;
        let mut class_file = attribute_edge_class_file();

        let record_attribute = |record: Record| Attribute::Record {
            name_index: 0,
            records: vec![record],
        };
        assert!(matches!(
            verify(
                &class_file,
                &[record_attribute(Record {
                    name_index: 8,
                    descriptor_index: 2,
                    attributes: Vec::new(),
                })],
                class_context,
            ),
            Err(InvalidConstantPoolIndexType(8))
        ));
        assert!(matches!(
            verify(
                &class_file,
                &[record_attribute(Record {
                    name_index: 9,
                    descriptor_index: 2,
                    attributes: Vec::new(),
                })],
                class_context,
            ),
            Err(InvalidConstantPoolIndex(9))
        ));
        assert!(matches!(
            verify(
                &class_file,
                &[record_attribute(Record {
                    name_index: 2,
                    descriptor_index: 8,
                    attributes: Vec::new(),
                })],
                class_context,
            ),
            Err(InvalidConstantPoolIndexType(8))
        ));
        assert!(matches!(
            verify(
                &class_file,
                &[record_attribute(Record {
                    name_index: 2,
                    descriptor_index: 9,
                    attributes: Vec::new(),
                })],
                class_context,
            ),
            Err(InvalidConstantPoolIndex(9))
        ));
        let descriptor_index = class_file.constant_pool.add_utf8("I").unwrap();
        assert!(matches!(
            verify(
                &class_file,
                &[record_attribute(Record {
                    name_index: 2,
                    descriptor_index,
                    attributes: vec![Attribute::Code {
                        name_index: 0,
                        max_stack: 0,
                        max_locals: 0,
                        code: Vec::new(),
                        exception_table: Vec::new(),
                        attributes: Vec::new(),
                    }],
                })],
                class_context,
            ),
            Err(VerificationError { .. })
        ));
    }

    #[test]
    fn test_verify_permitted_subclasses_invalid_paths() {
        let class_context = AttributeContext::Class;
        let class_file = attribute_edge_class_file();

        assert!(matches!(
            verify(
                &class_file,
                &[Attribute::PermittedSubclasses {
                    name_index: 0,
                    class_indexes: vec![8],
                }],
                class_context,
            ),
            Err(InvalidConstantPoolIndexType(8))
        ));
        assert!(matches!(
            verify(
                &class_file,
                &[Attribute::PermittedSubclasses {
                    name_index: 0,
                    class_indexes: vec![9],
                }],
                class_context,
            ),
            Err(InvalidConstantPoolIndex(9))
        ));
    }

    #[test]
    fn anonymous_inner_class_outer_index_is_version_sensitive() {
        let mut class_file = create_class_file(vec![
            Constant::utf8("Inner"),
            Constant::Class(1),
            Constant::utf8("Outer"),
            Constant::Class(3),
        ]);
        let inner = InnerClass {
            class_info_index: 2,
            outer_class_info_index: 4,
            name_index: 0,
            access_flags: NestedClassAccessFlags::empty(),
        };
        class_file.version = JAVA_7;
        assert!(verify_inner_classes(&class_file, std::slice::from_ref(&inner)).is_err());
        class_file.version = crate::JAVA_6;
        assert!(verify_inner_classes(&class_file, &[inner]).is_ok());
    }

    #[test]
    fn type_annotation_targets_match_location_and_table_bounds() {
        let class = ClassFile {
            super_class: 1,
            interfaces: vec![2],
            ..ClassFile::default()
        };
        let method = Method {
            attributes: vec![Attribute::Code {
                name_index: 0,
                max_stack: 1,
                max_locals: 0,
                code: vec![],
                attributes: vec![],
                exception_table: vec![ExceptionTableEntry {
                    range_pc: 0..1,
                    handler_pc: 0,
                    catch_type: 0,
                }],
            }],
            ..Method::default()
        };
        let field = Field {
            access_flags: FieldAccessFlags::empty(),
            name_index: 0,
            descriptor_index: 0,
            field_type: FieldType::Base(BaseType::Int),
            attributes: vec![],
        };
        for (context, target) in [
            (
                AttributeContext::Class,
                TargetType::SuperType {
                    target_type: 0x10,
                    supertype_index: u16::MAX,
                },
            ),
            (
                AttributeContext::Field(&field),
                TargetType::Empty { target_type: 0x13 },
            ),
            (
                AttributeContext::Method(&method),
                TargetType::Empty { target_type: 0x14 },
            ),
            (
                AttributeContext::Code(&method, 1),
                TargetType::Offset {
                    target_type: 0x43,
                    offset: 0,
                },
            ),
        ] {
            assert!(verify_annotation_target(&class, context, &target).is_ok());
            assert!(
                verify_annotation_target(&class, context, &TargetType::Empty { target_type: 0xFF })
                    .is_err()
            );
        }
        for (index, valid) in [(0, true), (1, false), (u16::MAX, true)] {
            assert_eq!(
                verify_annotation_target(
                    &class,
                    AttributeContext::Class,
                    &TargetType::SuperType {
                        target_type: 0x10,
                        supertype_index: index
                    }
                )
                .is_ok(),
                valid
            );
        }
        for (index, valid) in [(0, true), (1, false)] {
            assert_eq!(
                verify_annotation_target(
                    &class,
                    AttributeContext::Code(&method, 1),
                    &TargetType::Catch {
                        target_type: 0x42,
                        exception_table_index: index
                    }
                )
                .is_ok(),
                valid
            );
        }
        assert!(
            verify_annotation_target(
                &class,
                AttributeContext::Class,
                &TargetType::Catch {
                    target_type: 0x42,
                    exception_table_index: 0
                }
            )
            .is_err()
        );
    }

    #[test]
    fn annotation_superclass_and_throws_targets_have_real_entries() {
        let target = TargetType::SuperType {
            target_type: 0x10,
            supertype_index: u16::MAX,
        };
        let root = ClassFile::default();
        assert!(verify_annotation_target(&root, AttributeContext::Class, &target).is_err());
        let interface = ClassFile {
            super_class: 1,
            access_flags: ClassAccessFlags::INTERFACE,
            ..ClassFile::default()
        };
        assert!(verify_annotation_target(&interface, AttributeContext::Class, &target).is_err());
        let method = Method {
            attributes: vec![Attribute::Exceptions {
                name_index: 0,
                exception_indexes: vec![1],
            }],
            ..Method::default()
        };
        for (index, valid) in [(0, true), (1, false)] {
            assert_eq!(
                verify_annotation_target(
                    &root,
                    AttributeContext::Method(&method),
                    &TargetType::Throws {
                        target_type: 0x17,
                        throws_type_index: index
                    }
                )
                .is_ok(),
                valid
            );
        }
    }
}
