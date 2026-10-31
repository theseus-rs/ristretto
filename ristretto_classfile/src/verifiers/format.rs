//! Class-file constraints that relate names, descriptors, flags, and attributes.
use super::error::{Result, VerifyError};
use crate::attributes::Attribute;
use crate::{
    BaseType, ClassAccessFlags, ClassFile, FieldType, JavaStr, Method, MethodAccessFlags, Version,
};

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

pub(super) fn verify(class: &ClassFile<'_>) -> Result<()> {
    Version::from(class.version.major(), class.version.minor())?;
    if class.constant_pool.len() > 65534 {
        return Err(invalid("Constant pool exceeds u2 count"));
    }
    for method in &class.methods {
        verify_method(class, method)?;
    }
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
        if descriptor != "()V"
            || (class.version.major() >= 51 && !flags.contains(MethodAccessFlags::STATIC))
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
        != usize::from(!flags.intersects(MethodAccessFlags::ABSTRACT | MethodAccessFlags::NATIVE))
    {
        return Err(invalid("Invalid number of Code attributes"));
    }
    super::bytecode::constraints::verify_static(class, method)?;
    Ok(())
}
