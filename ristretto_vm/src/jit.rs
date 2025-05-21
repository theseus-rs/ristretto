use crate::Error::{InternalError, JitError};
use crate::{Result, VM};
use dashmap::DashMap;
use ristretto_classfile::MethodAccessFlags;
use ristretto_classloader::{Class, Method, Value};
use ristretto_jit::Error::{
    UnsupportedInstruction, UnsupportedMethod, UnsupportedTargetISA, UnsupportedType,
};
use ristretto_jit::Function;
use std::sync::{Arc, LazyLock};
use tracing::{debug, error, info};

/// The function cache is a thread-safe map that stores compiled functions.
static FUNCTION_CACHE: LazyLock<DashMap<String, Option<Arc<Function>>>> =
    LazyLock::new(DashMap::new);

/// Attempt to JIT compile a method.
///
/// This function will only compile the method if the VM is not in interpreted mode.
pub(crate) async fn compile(
    vm: &Arc<VM>,
    class: &Arc<Class>,
    method: &Method,
) -> Result<Option<Arc<Function>>> {
    if vm.configuration().interpreted() {
        return Ok(None);
    }

    let definition = method.definition();
    let class_name = class.name();
    let method_name = method.name();
    let method_descriptor = method.descriptor();
    let fully_qualified_method_name = format!("{class_name}.{method_name}{method_descriptor}");

    if let Some(function) = FUNCTION_CACHE.get(&fully_qualified_method_name) {
        debug!("Using cached function for {fully_qualified_method_name}");
        return Ok(function.clone());
    }
    let Some(compiler) = vm.compiler() else {
        return Ok(None);
    };

    let class_file = class.class_file();

    match compiler.compile(class_file, definition) {
        Ok(function) => {
            let function = Arc::new(function);
            info!("compiled method {fully_qualified_method_name}");
            FUNCTION_CACHE.insert(fully_qualified_method_name, Some(function.clone()));
            Ok(Some(function))
        }
        Err(UnsupportedInstruction(instruction)) => {
            debug!("Unsupported instruction: {instruction:?}");
            FUNCTION_CACHE.insert(fully_qualified_method_name, None);
            Ok(None)
        }
        Err(UnsupportedMethod(message)) => {
            debug!("Unsupported method: {message}");
            FUNCTION_CACHE.insert(fully_qualified_method_name, None);
            Ok(None)
        }
        Err(UnsupportedTargetISA(message)) => {
            debug!("Unsupported target ISA: {message}");
            FUNCTION_CACHE.insert(fully_qualified_method_name, None);
            Ok(None)
        }
        Err(UnsupportedType(vm_type)) => {
            debug!("Unsupported type: {vm_type}");
            FUNCTION_CACHE.insert(fully_qualified_method_name, None);
            Ok(None)
        }
        Err(error) => {
            let constant_pool = class.constant_pool();
            error!(
                "Error compiling instructions for {fully_qualified_method_name}:\n\
                Error:\n\
                {error:?}\n\
                Constant Pool:\n\
                {constant_pool}\n\
                Method:\n\
                {method:?}"
            );
            Err(JitError(error))
        }
    }
}

pub(crate) fn execute(
    function: Arc<Function>,
    method: &Method,
    mut parameters: Vec<Value>,
) -> Result<Option<Value>> {
    if !method.access_flags().contains(MethodAccessFlags::STATIC) {
        // Remove the first parameter (the `this` reference) for non-static methods
        parameters.remove(0);
    };

    let arguments = convert_parameters(parameters)?;
    let result = if let Some(value) = function.execute(arguments)? {
        let value = convert_to_vm(&value)?;
        Some(value)
    } else {
        None
    };
    Ok(result)
}

/// Convert parameters to JIT values.
fn convert_parameters(parameters: Vec<Value>) -> Result<Vec<ristretto_jit::Value>> {
    let mut values = Vec::with_capacity(parameters.len());
    for value in &parameters {
        let value = convert_to_jit(value)?;
        values.push(value);
    }
    Ok(values)
}

/// Convert a VM value to a JIT value.
fn convert_to_jit(value: &Value) -> Result<ristretto_jit::Value> {
    let jit_value = match value {
        Value::Int(value) => ristretto_jit::Value::I32(*value),
        Value::Long(value) => ristretto_jit::Value::I64(*value),
        Value::Float(value) => ristretto_jit::Value::F32(*value),
        Value::Double(value) => ristretto_jit::Value::F64(*value),
        _ => {
            eprintln!("Unsupported JIT value type: {value:?}");
            return Err(InternalError(format!(
                "Unsupported JIT value type: {value:?}"
            )));
        }
    };
    Ok(jit_value)
}

/// Convert a JIT value to a VM value.
fn convert_to_vm(jit_value: &ristretto_jit::Value) -> Result<Value> {
    let value = match jit_value {
        ristretto_jit::Value::I32(value) => Value::from(*value),
        ristretto_jit::Value::I64(value) => Value::from(*value),
        ristretto_jit::Value::F32(value) => Value::from(*value),
        ristretto_jit::Value::F64(value) => Value::from(*value),
    };
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_parameters() -> Result<()> {
        let parameters = vec![
            Value::Int(1),
            Value::Long(2),
            Value::Float(3.1),
            Value::Double(4.2),
        ];
        let values = convert_parameters(parameters)?;
        assert_eq!(values.len(), 4);
        assert_eq!(values[0], ristretto_jit::Value::I32(1));
        assert_eq!(values[1], ristretto_jit::Value::I64(2));
        assert_eq!(values[2], ristretto_jit::Value::F32(3.1));
        assert_eq!(values[3], ristretto_jit::Value::F64(4.2));
        Ok(())
    }

    #[test]
    fn test_convert_to_jit_i32() -> Result<()> {
        let value = Value::Int(42);
        let result = convert_to_jit(&value)?;
        assert_eq!(result, ristretto_jit::Value::I32(42));
        Ok(())
    }

    #[test]
    fn test_convert_to_jit_i64() -> Result<()> {
        let value = Value::Long(42);
        let result = convert_to_jit(&value)?;
        assert_eq!(result, ristretto_jit::Value::I64(42));
        Ok(())
    }

    #[test]
    fn test_convert_to_jit_f32() -> Result<()> {
        let value = Value::Float(42.1);
        let result = convert_to_jit(&value)?;
        assert_eq!(result, ristretto_jit::Value::F32(42.1));
        Ok(())
    }

    #[test]
    fn test_convert_to_jit_f64() -> Result<()> {
        let value = Value::Double(42.1);
        let result = convert_to_jit(&value)?;
        assert_eq!(result, ristretto_jit::Value::F64(42.1));
        Ok(())
    }

    #[test]
    fn test_convert_to_jit_unsupported() {
        let value = Value::Object(None);
        let result = convert_to_jit(&value);
        assert!(result.is_err());
        if let Err(error) = result {
            assert_eq!(
                error.to_string(),
                "Internal error: Unsupported JIT value type: Object(None)"
            );
        }
    }

    #[test]
    fn test_convert_to_vm_i32() -> Result<()> {
        let value = ristretto_jit::Value::I32(42);
        let result = convert_to_vm(&value)?;
        assert_eq!(result, Value::Int(42));
        Ok(())
    }

    #[test]
    fn test_convert_to_vm_i64() -> Result<()> {
        let value = ristretto_jit::Value::I64(42);
        let result = convert_to_vm(&value)?;
        assert_eq!(result, Value::Long(42));
        Ok(())
    }

    #[test]
    fn test_convert_to_vm_f32() -> Result<()> {
        let value = ristretto_jit::Value::F32(42.1);
        let result = convert_to_vm(&value)?;
        assert_eq!(result, Value::Float(42.1));
        Ok(())
    }

    #[test]
    fn test_convert_to_vm_f64() -> Result<()> {
        let value = ristretto_jit::Value::F64(42.1);
        let result = convert_to_vm(&value)?;
        assert_eq!(result, Value::Double(42.1));
        Ok(())
    }
}
