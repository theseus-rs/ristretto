use crate::java::lang::class;
use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classfile::attributes::Attribute;
use ristretto_classloader::{Class, Value};
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Frame;
use ristretto_types::JavaObject;
use ristretto_types::ModuleAccess;
use ristretto_types::Thread;
use ristretto_types::VM;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

struct BootModuleReferenceState {
    java_base: AtomicBool,
}

#[intrinsic_method(
    "jdk/internal/reflect/Reflection.areNestMates(Ljava/lang/Class;Ljava/lang/Class;)Z",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn are_nest_mates<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let member_class = parameters.pop()?;
    let current_class = parameters.pop()?;
    if member_class.is_null() || current_class.is_null() {
        return Ok(Some(Value::from(false)));
    }

    let member_class = class::get_class(&thread, &member_class).await?;
    let current_class = class::get_class(&thread, &current_class).await?;
    if member_class == current_class {
        return Ok(Some(Value::from(true)));
    }

    let member_nest_host = get_nest_host(&thread, member_class).await?;
    let current_nest_host = get_nest_host(&thread, current_class).await?;
    let are_nest_mates = member_nest_host == current_nest_host;
    Ok(Some(Value::from(are_nest_mates)))
}

/// Returns the nest host class if it exists, otherwise returns the class.
async fn get_nest_host<T: Thread + 'static>(
    thread: &Arc<T>,
    class: Arc<Class>,
) -> Result<Arc<Class>> {
    let class_file = class.class_file();
    for attribute in &class_file.attributes {
        let Attribute::NestHost {
            name_index: _name_index,
            host_class_index,
        } = attribute
        else {
            continue;
        };

        let constant_pool = &class_file.constant_pool;
        let host_class = constant_pool.try_get_class(*host_class_index)?;
        let host_class = thread.class_java_str(host_class).await?;
        return Ok(host_class);
    }
    Ok(class)
}

#[intrinsic_method(
    "jdk/internal/reflect/Reflection.getCallerClass()Ljava/lang/Class;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_caller_class<T: Thread + 'static>(
    thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    let frames = thread.frames().await?;
    // Skip frames:
    // - Frame 0: The current intrinsic (getCallerClass)
    // - Frame 1: The immediate caller (typically Reflection.getCallerClass bytecode frame)
    // - Any additional reflection/method-handle infrastructure frames that shouldn't be visible
    //
    // The key is to find the "real" caller; the first class that is not part of the
    // reflection or method handle implementation machinery.
    //
    // We skip:
    // - jdk/internal/reflect/* (reflection implementation)
    // - sun/reflect/* (legacy reflection)
    //
    // We do NOT skip java/lang/invoke/* because those classes may legitimately call
    // getCallerClass to get their own class (e.g., StringConcatFactory calling
    // MethodHandles.lookup() should get a lookup for StringConcatFactory, not the user class).
    if frames.iter().rev().any(|frame| {
        frame.class().name() == "java/lang/Class" && frame.method().name() == "getResourceAsStream"
    }) {
        ensure_java_base_module_reference(&thread).await?;
    }

    for frame in frames.iter().rev().skip(1) {
        let class = frame.class();
        let class_name = class.name();
        // Skip only reflection implementation frames
        if class_name.starts_with("jdk/internal/reflect/") || class_name.starts_with("sun/reflect/")
        {
            continue;
        }
        if is_caller_sensitive_frame(frame.as_ref())? {
            continue;
        }
        let class_object = class.to_object(&thread).await?;
        return Ok(Some(class_object));
    }
    // No caller found
    Ok(Some(Value::Object(None)))
}

async fn ensure_java_base_module_reference<T: Thread + 'static>(thread: &Arc<T>) -> Result<()> {
    let vm = thread.vm()?;
    if !vm.module_system().resolved_configuration().is_empty() {
        return Ok(());
    }

    let state = vm
        .resource_manager()
        .get_or_init(|| BootModuleReferenceState {
            java_base: AtomicBool::new(false),
        })?;
    if state.java_base.swap(true, Ordering::AcqRel) {
        return Ok(());
    }

    if let Err(error) = register_java_base_module_reference(thread).await {
        state.java_base.store(false, Ordering::Release);
        return Err(error);
    }

    Ok(())
}

async fn register_java_base_module_reference<T: Thread + 'static>(thread: &Arc<T>) -> Result<()> {
    let boot_loader = thread
        .try_invoke(
            "jdk.internal.loader.ClassLoaders",
            "bootLoader()Ljdk/internal/loader/BuiltinClassLoader;",
            &[] as &[Value],
        )
        .await?;
    if boot_loader.is_null() {
        return Ok(());
    }

    let system_finder = thread
        .try_invoke(
            "java.lang.module.ModuleFinder",
            "ofSystem()Ljava/lang/module/ModuleFinder;",
            &[] as &[Value],
        )
        .await?;
    if system_finder.is_null() {
        return Ok(());
    }

    let module_name = "java.base".to_object(thread.as_ref()).await?;
    let finder_class = system_finder
        .as_object_ref()?
        .class()
        .name()
        .replace('/', ".");
    let module_ref = thread
        .try_invoke(
            &finder_class,
            "find(Ljava/lang/String;)Ljava/util/Optional;",
            &[system_finder, module_name],
        )
        .await?;
    if module_ref.is_null() {
        return Ok(());
    }

    let module_ref = thread
        .try_invoke(
            "java.util.Optional",
            "orElse(Ljava/lang/Object;)Ljava/lang/Object;",
            &[module_ref, Value::Object(None)],
        )
        .await?;
    if module_ref.is_null() {
        return Ok(());
    }

    thread
        .invoke(
            "jdk.internal.loader.BuiltinClassLoader",
            "loadModule(Ljava/lang/module/ModuleReference;)V",
            &[boot_loader, module_ref],
        )
        .await?;
    Ok(())
}

fn is_caller_sensitive_frame<F: Frame>(frame: &F) -> Result<bool> {
    let class_file = frame.class().class_file();
    let constant_pool = &class_file.constant_pool;
    let method = frame.method();
    for attribute in &method.definition().attributes {
        let (Attribute::RuntimeVisibleAnnotations { annotations, .. }
        | Attribute::RuntimeInvisibleAnnotations { annotations, .. }) = attribute
        else {
            continue;
        };
        for annotation in annotations {
            let annotation_type = constant_pool.try_get_utf8(annotation.type_index)?;
            if annotation_type == "Ljdk/internal/reflect/CallerSensitive;"
                || annotation_type == "Lsun/reflect/CallerSensitive;"
            {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

#[intrinsic_method(
    "jdk/internal/reflect/Reflection.getClassAccessFlags(Ljava/lang/Class;)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_class_access_flags<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let object = parameters.pop()?;
    let class_name = {
        let object = object.as_object_ref()?;
        object.value("name")?.as_string()?
    };
    let class = thread.class(&class_name).await?;
    let class_file = class.class_file();
    let access_flags = &class_file.access_flags;
    #[expect(clippy::cast_lossless)]
    let class_access_flags = access_flags.bits() as i32;
    Ok(Some(Value::Int(class_access_flags)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_are_nest_mates_current_class_null_is_false() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let current_class = Value::Object(None);
        let member_class = thread
            .class("java.lang.String")
            .await?
            .to_object(&thread)
            .await?;
        let mut parameters = Parameters::default();
        parameters.push(current_class);
        parameters.push(member_class);
        let value = are_nest_mates(thread, parameters)
            .await?
            .expect("nest_mates");
        let are_nest_mates = value.as_bool()?;
        assert!(!are_nest_mates);
        Ok(())
    }

    #[tokio::test]
    async fn test_are_nest_mates_member_class_null_is_false() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let current_class = thread
            .class("java.lang.String")
            .await?
            .to_object(&thread)
            .await?;
        let member_class = Value::Object(None);
        let mut parameters = Parameters::default();
        parameters.push(current_class);
        parameters.push(member_class);
        let value = are_nest_mates(thread, parameters)
            .await?
            .expect("nest_mates");
        let are_nest_mates = value.as_bool()?;
        assert!(!are_nest_mates);
        Ok(())
    }

    #[tokio::test]
    async fn test_are_nest_mates_same_class_is_true() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let string_class = thread
            .class("java.lang.String")
            .await?
            .to_object(&thread)
            .await?;
        let current_class = string_class.clone();
        let member_class = string_class;
        let mut parameters = Parameters::default();
        parameters.push(current_class);
        parameters.push(member_class);
        let value = are_nest_mates(thread, parameters)
            .await?
            .expect("nest_mates");
        let are_nest_mates = value.as_bool()?;
        assert!(are_nest_mates);
        Ok(())
    }

    #[tokio::test]
    async fn test_are_nest_mates_same_nest_host_is_true() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let current_class = thread
            .class("java.lang.Integer")
            .await?
            .to_object(&thread)
            .await?;
        let member_class = thread
            .class("java.lang.Integer$IntegerCache")
            .await?
            .to_object(&thread)
            .await?;
        let mut parameters = Parameters::default();
        parameters.push(current_class);
        parameters.push(member_class);
        let value = are_nest_mates(thread, parameters)
            .await?
            .expect("nest_mates");
        let are_nest_mates = value.as_bool()?;
        assert!(are_nest_mates);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_class_access_flags() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let class = thread.class("java.lang.String").await?;
        let class_object = class.to_object(&thread).await?;
        let parameters = Parameters::new(vec![class_object]);
        let result = get_class_access_flags(thread, parameters).await?;
        let access_flags = result.expect("access_flags").as_i32()?;
        assert_eq!(access_flags, 49);
        Ok(())
    }
}
