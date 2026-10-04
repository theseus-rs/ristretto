//! Class-file constraints that relate names, descriptors, flags, and attributes.
use super::error::{Result, VerifyError};
use crate::attributes::Attribute;
use crate::{
    BaseType, ClassAccessFlags, ClassFile, Constant, FieldType, JavaStr, Method, MethodAccessFlags,
    ReferenceKind, Version,
};
use std::collections::HashSet;

fn invalid(message: &str) -> VerifyError {
    VerifyError::ClassFormatError(message.to_string())
}

fn member_name(name: &JavaStr, method: bool) -> Result<()> {
    let bytes = name.as_bytes();
    if bytes.is_empty()
        || bytes
            .iter()
            .any(|c| b".;[/".contains(c) || (method && b"<>".contains(c)))
    {
        return Err(invalid("Invalid member name"));
    }
    Ok(())
}

fn class_name(name: &JavaStr, arrays: bool) -> Result<()> {
    if arrays && name.as_bytes().starts_with(b"[") {
        FieldType::parse_java_str(name)?;
        return Ok(());
    }
    for part in name.as_bytes().split(|b| *b == b'/') {
        if part.is_empty() || part.iter().any(|b| b".;[".contains(b)) {
            return Err(invalid("Invalid internal class name"));
        }
    }
    Ok(())
}

pub(super) fn verify(class: &ClassFile<'_>) -> Result<()> {
    Version::from(class.version.major(), class.version.minor())?;
    if class.constant_pool.len() > 65534 {
        return Err(invalid("Constant pool exceeds u2 count"));
    }
    let pool = &class.constant_pool;
    let name = class.class_name()?;
    class_name(name, false)?;
    let module = class.access_flags.contains(ClassAccessFlags::MODULE);
    if module {
        if class.version.major() < 53
            || class.access_flags != ClassAccessFlags::MODULE
            || name != "module-info"
            || class.super_class != 0
            || !class.interfaces.is_empty()
            || !class.fields.is_empty()
            || !class.methods.is_empty()
            || class
                .attributes
                .iter()
                .filter(|a| matches!(a, Attribute::Module { .. }))
                .count()
                != 1
        {
            return Err(invalid("Invalid module-info class"));
        }
    } else {
        if (class.super_class == 0) != (name == "java/lang/Object") {
            return Err(invalid("Only java/lang/Object may have no superclass"));
        }
        if class.super_class != 0 {
            let parent = pool.try_get_class(class.super_class)?;
            class_name(parent, false)?;
            if parent == name {
                return Err(invalid("Class cannot be its own superclass"));
            }
            if class.access_flags.contains(ClassAccessFlags::INTERFACE)
                && parent != "java/lang/Object"
            {
                return Err(invalid("Interface superclass must be java/lang/Object"));
            }
        }
    }
    let mut interfaces = HashSet::new();
    for index in &class.interfaces {
        let interface = pool.try_get_class(*index)?;
        class_name(interface, false)?;
        if !interfaces.insert(interface) {
            return Err(invalid("Duplicate interface"));
        }
    }
    verify_attributes(
        class,
        &class.attributes,
        crate::attributes::attribute::AttributeLocation::Class,
    )?;
    verify_fields(class)?;
    let mut methods = HashSet::new();
    for method in &class.methods {
        verify_attributes(
            class,
            &method.attributes,
            crate::attributes::attribute::AttributeLocation::Method,
        )?;
        verify_method(class, method)?;
        if !methods.insert((
            pool.try_get_utf8(method.name_index)?,
            pool.try_get_utf8(method.descriptor_index)?,
        )) {
            return Err(invalid("Duplicate method name and descriptor"));
        }
    }
    verify_constants(class)?;
    verify_module_tables(class)?;
    Ok(())
}

pub(super) fn verify_method(class: &ClassFile<'_>, method: &Method) -> Result<()> {
    let name = class.constant_pool.try_get_utf8(method.name_index)?;
    let descriptor = class.constant_pool.try_get_utf8(method.descriptor_index)?;
    let (parameters, ret) = FieldType::parse_method_descriptor(descriptor)?;
    let flags = method.access_flags;
    if name == "<init>" {
        let disallowed = MethodAccessFlags::STATIC
            | MethodAccessFlags::FINAL
            | MethodAccessFlags::SYNCHRONIZED
            | MethodAccessFlags::BRIDGE
            | MethodAccessFlags::NATIVE
            | MethodAccessFlags::ABSTRACT;
        if ret.is_some()
            || flags.intersects(disallowed)
            || class.access_flags.contains(ClassAccessFlags::INTERFACE)
        {
            return Err(invalid("Invalid constructor flags or descriptor"));
        }
    } else if name == "<clinit>" {
        if ret.is_some()
            || (class.version.major() >= 51
                && (!parameters.is_empty() || !flags.contains(MethodAccessFlags::STATIC)))
        {
            return Err(invalid("Invalid class initializer flags or descriptor"));
        }
    } else {
        member_name(name, true)?;
        if class.access_flags.contains(ClassAccessFlags::INTERFACE) {
            if class.version.major() < 52
                && !flags.contains(MethodAccessFlags::PUBLIC | MethodAccessFlags::ABSTRACT)
            {
                return Err(invalid(
                    "Legacy interface method must be public and abstract",
                ));
            }
            if class.version.major() >= 52
                && !flags.intersects(MethodAccessFlags::PUBLIC | MethodAccessFlags::PRIVATE)
            {
                return Err(invalid("Interface method must be public or private"));
            }
        }
    }
    let receiver = usize::from(!flags.contains(MethodAccessFlags::STATIC) && name != "<clinit>");
    let slots = receiver
        + parameters
            .iter()
            .map(|p| {
                if matches!(p, FieldType::Base(BaseType::Long | BaseType::Double)) {
                    2
                } else {
                    1
                }
            })
            .sum::<usize>();
    if slots > 255 {
        return Err(invalid(
            "Method parameters including receiver exceed 255 slots",
        ));
    }
    let mut code_count = 0;
    for attr in &method.attributes {
        if let Attribute::Code { max_locals, .. } = attr {
            code_count += 1;
            if usize::from(*max_locals) < slots {
                return Err(invalid("Arguments exceed max_locals"));
            }
        }
    }
    if code_count
        != usize::from(
            name == "<clinit>"
                || !flags.intersects(MethodAccessFlags::ABSTRACT | MethodAccessFlags::NATIVE),
        )
    {
        return Err(invalid("Invalid number of Code attributes"));
    }
    super::bytecode::constraints::verify_static(class, method)?;
    Ok(())
}

fn verify_attributes(
    class: &ClassFile<'_>,
    attributes: &[Attribute],
    location: crate::attributes::attribute::AttributeLocation,
) -> Result<()> {
    use crate::attributes::attribute::AttributeLocation as L;
    let pool = &class.constant_pool;
    let mut names = HashSet::new();
    for attribute in attributes {
        let name = pool.try_get_utf8(attribute.name_index())?;
        if matches!(attribute, Attribute::Unknown { .. }) {
            continue;
        }
        if name != attribute.name() {
            return Err(invalid("Attribute name index disagrees with its contents"));
        }
        if !location.recognizes(name.as_bytes()) {
            continue;
        }
        if !matches!(
            attribute,
            Attribute::LineNumberTable { .. }
                | Attribute::LocalVariableTable { .. }
                | Attribute::LocalVariableTypeTable { .. }
        ) && !names.insert(name)
        {
            return Err(invalid("Duplicate attribute"));
        }
        match attribute {
            Attribute::Code { .. } => verify_code_metadata(class, attribute)?,
            Attribute::Record { records, .. } => {
                for record in records {
                    member_name(pool.try_get_utf8(record.name_index)?, false)?;
                    verify_attributes(class, &record.attributes, L::RecordComponent)?;
                }
            }
            Attribute::BootstrapMethods { methods, .. } => {
                for bootstrap in methods {
                    if !matches!(
                        pool.try_get(bootstrap.bootstrap_method_ref)?,
                        Constant::MethodHandle {
                            reference_kind: ReferenceKind::InvokeStatic
                                | ReferenceKind::NewInvokeSpecial,
                            ..
                        }
                    ) {
                        return Err(invalid(
                            "Bootstrap method must be an invokestatic or newinvokespecial method handle",
                        ));
                    }
                    for argument in &bootstrap.arguments {
                        if !matches!(
                            pool.try_get(*argument)?,
                            Constant::Integer(_)
                                | Constant::Float(_)
                                | Constant::Long(_)
                                | Constant::Double(_)
                                | Constant::String(_)
                                | Constant::Class(_)
                                | Constant::MethodHandle { .. }
                                | Constant::MethodType(_)
                                | Constant::Dynamic { .. }
                        ) {
                            return Err(invalid("Bootstrap argument must be a loadable constant"));
                        }
                    }
                }
            }
            Attribute::RuntimeVisibleTypeAnnotations {
                type_annotations, ..
            }
            | Attribute::RuntimeInvisibleTypeAnnotations {
                type_annotations, ..
            } => {
                for annotation in type_annotations {
                    let target = annotation.target_type.target_type();
                    let allowed = match location {
                        L::Class => matches!(target, 0x00 | 0x10 | 0x11),
                        L::Field | L::RecordComponent => target == 0x13,
                        L::Method => matches!(target, 0x01 | 0x12 | 0x14..=0x17),
                        L::Code => (0x40..=0x4b).contains(&target),
                        L::Any => true,
                    };
                    if !allowed
                        || annotation.type_path.iter().any(|p| {
                            p.type_path_kind > 3
                                || (p.type_path_kind != 3 && p.type_argument_index != 0)
                        })
                    {
                        return Err(invalid("Invalid type annotation target or path"));
                    }
                }
            }
            _ => {}
        }
    }
    Ok(())
}

fn verify_fields(class: &ClassFile<'_>) -> Result<()> {
    let pool = &class.constant_pool;
    let mut fields = HashSet::new();
    for field in &class.fields {
        verify_attributes(
            class,
            &field.attributes,
            crate::attributes::attribute::AttributeLocation::Field,
        )?;
        let name = pool.try_get_utf8(field.name_index)?;
        member_name(name, false)?;
        let descriptor = pool.try_get_utf8(field.descriptor_index)?;
        let ty = FieldType::parse_java_str(descriptor)?;
        if ty != field.field_type {
            return Err(invalid("Field type disagrees with descriptor"));
        }
        if !fields.insert((name, descriptor)) {
            return Err(invalid("Duplicate field name and descriptor"));
        }
        for attribute in &field.attributes {
            if let Attribute::ConstantValue {
                constant_value_index,
                ..
            } = attribute
            {
                let valid = match (&ty, pool.try_get(*constant_value_index)?) {
                    (
                        FieldType::Base(
                            BaseType::Boolean
                            | BaseType::Byte
                            | BaseType::Char
                            | BaseType::Short
                            | BaseType::Int,
                        ),
                        Constant::Integer(_),
                    )
                    | (FieldType::Base(BaseType::Long), Constant::Long(_))
                    | (FieldType::Base(BaseType::Double), Constant::Double(_))
                    | (FieldType::Base(BaseType::Float), Constant::Float(_)) => true,
                    (FieldType::Object(name), Constant::String(_)) => name == "java/lang/String",
                    _ => false,
                };
                if !valid {
                    return Err(invalid(
                        "ConstantValue type disagrees with field descriptor",
                    ));
                }
            }
        }
    }
    Ok(())
}

fn verify_constants(class: &ClassFile<'_>) -> Result<()> {
    let pool = &class.constant_pool;
    let module = class.access_flags.contains(ClassAccessFlags::MODULE);
    for constant in pool {
        match constant {
            Constant::Class(index) => class_name(pool.try_get_utf8(*index)?, true)?,
            Constant::Module(_) | Constant::Package(_) if !module => {
                return Err(invalid("Module and Package constants require ACC_MODULE"));
            }
            Constant::MethodType(index) => {
                FieldType::parse_method_descriptor(pool.try_get_utf8(*index)?)?;
            }
            Constant::NameAndType {
                name_index,
                descriptor_index,
            } => {
                let name = pool.try_get_utf8(*name_index)?;
                let descriptor = pool.try_get_utf8(*descriptor_index)?;
                if descriptor.as_bytes().starts_with(b"(") {
                    let (_, ret) = FieldType::parse_method_descriptor(descriptor)?;
                    if name == "<init>" {
                        if ret.is_some() {
                            return Err(invalid("Constructor descriptor must return void"));
                        }
                    } else if name != "<clinit>" {
                        // Kotlin uses <clinit> in EnclosingMethod metadata. A NameAndType
                        // is not itself an invocation; method references are checked below.
                        member_name(name, true)?;
                    }
                } else {
                    member_name(name, false)?;
                    FieldType::parse_java_str(descriptor)?;
                }
            }
            Constant::FieldRef {
                name_and_type_index,
                ..
            }
            | Constant::Dynamic {
                name_and_type_index,
                ..
            } => {
                let (name, descriptor) = pool.try_get_name_and_type(*name_and_type_index)?;
                member_name(
                    pool.try_get_utf8(*name)?,
                    matches!(constant, Constant::Dynamic { .. }),
                )?;
                FieldType::parse_java_str(pool.try_get_utf8(*descriptor)?)?;
            }
            Constant::MethodRef {
                name_and_type_index,
                ..
            }
            | Constant::InterfaceMethodRef {
                name_and_type_index,
                ..
            }
            | Constant::InvokeDynamic {
                name_and_type_index,
                ..
            } => {
                let (name, descriptor) = pool.try_get_name_and_type(*name_and_type_index)?;
                let name = pool.try_get_utf8(*name)?;
                if name != "<init>" || !matches!(constant, Constant::MethodRef { .. }) {
                    member_name(name, true)?;
                }
                let (_, ret) = FieldType::parse_method_descriptor(pool.try_get_utf8(*descriptor)?)?;
                if name == "<init>" && ret.is_some() {
                    return Err(invalid("Constructor reference must return void"));
                }
            }
            Constant::MethodHandle {
                reference_kind,
                reference_index,
            } => {
                if let Constant::MethodRef {
                    name_and_type_index,
                    ..
                }
                | Constant::InterfaceMethodRef {
                    name_and_type_index,
                    ..
                } = pool.try_get(*reference_index)?
                {
                    let (name, _) = pool.try_get_name_and_type(*name_and_type_index)?;
                    let name = pool.try_get_utf8(*name)?;
                    if (*reference_kind == ReferenceKind::NewInvokeSpecial) != (name == "<init>")
                        || name == "<clinit>"
                    {
                        return Err(invalid("Illegal MethodHandle target name"));
                    }
                }
            }
            _ => {}
        }
    }
    Ok(())
}

fn verify_code_metadata(class: &ClassFile<'_>, attribute: &Attribute) -> Result<()> {
    use crate::attributes::attribute::AttributeLocation as L;
    let pool = &class.constant_pool;
    let Attribute::Code {
        code,
        max_locals,
        attributes,
        exception_table,
        ..
    } = attribute
    else {
        return Ok(());
    };
    verify_attributes(class, attributes, L::Code)?;
    let count = u16::try_from(code.len())?;
    for handler in exception_table {
        if handler.range_pc.start >= handler.range_pc.end
            || handler.range_pc.end > count
            || handler.handler_pc >= count
        {
            return Err(invalid("Invalid exception handler range"));
        }
    }
    for nested in attributes {
        match nested {
            Attribute::LocalVariableTable { variables, .. } => {
                for local in variables {
                    let ty = FieldType::parse_java_str(pool.try_get_utf8(local.descriptor_index)?)?;
                    let width = if matches!(ty, FieldType::Base(BaseType::Long | BaseType::Double))
                    {
                        2
                    } else {
                        1
                    };
                    if u32::from(local.index) + width > u32::from(*max_locals)
                        || local.start_pc >= count
                        || u32::from(local.start_pc) + u32::from(local.length) > u32::from(count)
                    {
                        return Err(invalid("Invalid LocalVariableTable range or local index"));
                    }
                }
            }
            Attribute::LocalVariableTypeTable { variable_types, .. } => {
                for local in variable_types {
                    if local.index >= *max_locals
                        || local.start_pc >= count
                        || u32::from(local.start_pc) + u32::from(local.length) > u32::from(count)
                    {
                        return Err(invalid(
                            "Invalid LocalVariableTypeTable range or local index",
                        ));
                    }
                }
            }
            Attribute::StackMapTable { frames, .. } => {
                for frame in frames {
                    use crate::attributes::StackFrame as F;
                    let (locals, stack): (&[_], &[_]) = match frame {
                        F::FullFrame { locals, stack, .. } => (locals, stack),
                        F::AppendFrame { locals, .. } => (locals, &[]),
                        F::SameLocals1StackItemFrame { stack, .. }
                        | F::SameLocals1StackItemFrameExtended { stack, .. } => (&[], stack),
                        _ => (&[], &[]),
                    };
                    for ty in locals.iter().chain(stack) {
                        if let crate::attributes::VerificationType::Uninitialized { offset } = ty
                            && !matches!(
                                code.get(usize::from(*offset)),
                                Some(crate::attributes::Instruction::New(_))
                            )
                        {
                            return Err(invalid("Stack map uninitialized type must refer to new"));
                        }
                    }
                }
            }
            _ => {}
        }
    }

    Ok(())
}

fn unique_names<'a>(
    indexes: impl IntoIterator<Item = u16>,
    resolve: impl Fn(u16) -> crate::Result<&'a JavaStr>,
) -> Result<()> {
    let mut names = HashSet::new();
    for index in indexes {
        if !names.insert(resolve(index)?) {
            return Err(invalid("Duplicate module table entry"));
        }
    }
    Ok(())
}

fn verify_module_tables(class: &ClassFile<'_>) -> Result<()> {
    use crate::attributes::{ModuleAccessFlags, RequiresFlags};
    let pool = &class.constant_pool;
    for attribute in &class.attributes {
        if let Attribute::Module {
            module_name_index,
            flags,
            requires,
            exports,
            opens,
            uses,
            provides,
            ..
        } = attribute
        {
            let name = pool.try_get_module(*module_name_index)?;
            unique_names(requires.iter().map(|r| r.index), |i| pool.try_get_module(i))?;
            if name == "java.base" {
                if !requires.is_empty() {
                    return Err(invalid("java.base must have no requires entries"));
                }
            } else {
                let base = requires
                    .iter()
                    .find(|r| pool.try_get_module(r.index).is_ok_and(|n| n == "java.base"));
                let Some(base) = base else {
                    return Err(invalid("Module must require java.base"));
                };
                if base.flags.contains(RequiresFlags::SYNTHETIC)
                    || (class.version.major() >= 54
                        && base.flags.contains(RequiresFlags::STATIC_PHASE))
                {
                    return Err(invalid("Invalid flags on java.base dependency"));
                }
            }
            if flags.contains(ModuleAccessFlags::OPEN) && !opens.is_empty() {
                return Err(invalid("Open module must have an empty opens table"));
            }
            unique_names(exports.iter().map(|e| e.index), |i| pool.try_get_package(i))?;
            for export in exports {
                unique_names(export.to_index.iter().copied(), |i| pool.try_get_module(i))?;
            }
            unique_names(opens.iter().map(|e| e.index), |i| pool.try_get_package(i))?;
            for open in opens {
                unique_names(open.to_index.iter().copied(), |i| pool.try_get_module(i))?;
            }
            unique_names(uses.iter().copied(), |i| pool.try_get_class(i))?;
            unique_names(provides.iter().map(|p| p.index), |i| pool.try_get_class(i))?;
            for provider in provides {
                if provider.with_index.is_empty() {
                    return Err(invalid("Module provider must name an implementation"));
                }
                unique_names(provider.with_index.iter().copied(), |i| {
                    pool.try_get_class(i)
                })?;
            }
        }
        if let Attribute::ModulePackages {
            package_indexes, ..
        } = attribute
        {
            unique_names(package_indexes.iter().copied(), |i| pool.try_get_package(i))?;
        }
    }
    Ok(())
}
