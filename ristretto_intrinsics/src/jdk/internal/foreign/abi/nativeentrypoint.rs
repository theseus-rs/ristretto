//! Native downcall-stub ownership.
//!
//! `HotSpot` returns an address in its code cache from `makeDowncallStub`. Ristretto does not expose
//! its executable-code allocator to Java code, so the `long` returned here is an opaque VM-owned
//! token instead. Keeping the token and its decoded configuration in a per-VM registry gives it
//! the same lifetime semantics as a `HotSpot` runtime stub without relying on pointer width, host
//! endianness, or an architecture-specific executable-memory implementation.

use parking_lot::RwLock as ParkingRwLock;
use portable_atomic::{AtomicI64, Ordering};
use ristretto_classfile::JAVA_21;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::{Reference, Value};
use ristretto_gc::sync::RwLock as GcRwLock;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Error::InternalError;
use ristretto_types::Thread;
use ristretto_types::VM;
use ristretto_types::{Parameters, Result};
use std::collections::HashMap;
use std::sync::Arc;

/// Keep synthetic code-cache tokens disjoint from the other opaque handle ranges used by the VM.
const FIRST_DOWNCALL_STUB: i64 = 0x6300_0000_0000_0000;

/// Handles are process-unique, just like real code addresses. Stub storage remains per-VM so that
/// dropping a VM releases all of its outstanding stubs.
static NEXT_DOWNCALL_STUB: AtomicI64 = AtomicI64::new(FIRST_DOWNCALL_STUB);

#[derive(Clone, Debug, Eq, PartialEq)]
struct MethodTypeDescriptor {
    parameters: Vec<String>,
    return_type: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct VMStorageDescriptor {
    storage_type: i8,
    segment_mask_or_size: i16,
    index_or_offset: i32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct AbiDescriptor {
    architecture: String,
    input_storage: Vec<Vec<VMStorageDescriptor>>,
    output_storage: Vec<Vec<VMStorageDescriptor>>,
    volatile_storage: Vec<Vec<VMStorageDescriptor>>,
    stack_alignment: i32,
    shadow_space: i32,
    scratch1: VMStorageDescriptor,
    scratch2: VMStorageDescriptor,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct DowncallStub {
    method_type: MethodTypeDescriptor,
    abi: AbiDescriptor,
    argument_moves: Vec<Option<VMStorageDescriptor>>,
    return_moves: Vec<Option<VMStorageDescriptor>>,
    needs_return_buffer: bool,
    captured_state_mask: i32,
    needs_transition: bool,
}

#[derive(Debug, Default)]
struct DowncallStubs {
    stubs: ParkingRwLock<HashMap<i64, DowncallStub>>,
}

impl DowncallStubs {
    /// Inserts a stub and returns its process-unique token. A zero result has the same meaning as
    /// `HotSpot`'s null `RuntimeStub`: allocation failed.
    fn insert(&self, stub: DowncallStub) -> i64 {
        let Ok(handle) =
            NEXT_DOWNCALL_STUB.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |current| {
                current.checked_add(1)
            })
        else {
            return 0;
        };

        let mut stubs = self.stubs.write();
        if stubs.try_reserve(1).is_err() {
            return 0;
        }
        stubs.insert(handle, stub);
        handle
    }

    fn remove(&self, handle: i64) -> bool {
        handle != 0 && self.stubs.write().remove(&handle).is_some()
    }
}

fn downcall_stubs<V: VM + ?Sized>(vm: &V) -> Result<Arc<DowncallStubs>> {
    vm.resource_manager().get_or_init(DowncallStubs::default)
}

fn class_name(value: &Value) -> Result<String> {
    let class = value.as_object_ref()?;
    if class.class().name() != "java/lang/Class" {
        return Err(InternalError(format!(
            "expected java/lang/Class, found {}",
            class.class().name()
        )));
    }
    Ok(class.value("name")?.as_string()?)
}

fn decode_method_type(
    reference: &ristretto_gc::Gc<GcRwLock<Reference>>,
) -> Result<MethodTypeDescriptor> {
    let method_type = reference.read();
    let method_type = method_type.as_object_ref()?;
    if method_type.class().name() != "java/lang/invoke/MethodType" {
        return Err(InternalError(format!(
            "expected java/lang/invoke/MethodType, found {}",
            method_type.class().name()
        )));
    }

    let parameter_types = method_type.value("ptypes")?;
    let (parameter_array_class, parameter_types) = parameter_types.as_class_vec_ref()?;
    if parameter_array_class.name() != "[Ljava/lang/Class;" {
        return Err(InternalError(format!(
            "expected [Ljava/lang/Class;, found {}",
            parameter_array_class.name()
        )));
    }
    let parameters = parameter_types
        .iter()
        .map(class_name)
        .collect::<Result<Vec<_>>>()?;
    let return_type = class_name(&method_type.value("rtype")?)?;
    Ok(MethodTypeDescriptor {
        parameters,
        return_type,
    })
}

fn decode_vm_storage(value: &Value) -> Result<Option<VMStorageDescriptor>> {
    if value.is_null() {
        // Since JDK 25 an Object heap base has no native move; its paired offset does.
        return Ok(None);
    }
    let storage = value.as_object_ref()?;
    if storage.class().name() != "jdk/internal/foreign/abi/VMStorage" {
        return Err(InternalError(format!(
            "expected jdk/internal/foreign/abi/VMStorage, found {}",
            storage.class().name()
        )));
    }
    Ok(Some(VMStorageDescriptor {
        storage_type: storage.value("type")?.as_i8()?,
        segment_mask_or_size: storage.value("segmentMaskOrSize")?.as_i16()?,
        index_or_offset: storage.value("indexOrOffset")?.as_i32()?,
    }))
}

fn decode_vm_storages(
    reference: &ristretto_gc::Gc<GcRwLock<Reference>>,
) -> Result<Vec<Option<VMStorageDescriptor>>> {
    let storages = reference.read();
    let (storage_array_class, storages) = storages.as_class_vec_ref()?;
    if storage_array_class.name() != "[Ljdk/internal/foreign/abi/VMStorage;" {
        return Err(InternalError(format!(
            "expected [Ljdk/internal/foreign/abi/VMStorage;, found {}",
            storage_array_class.name()
        )));
    }
    storages
        .iter()
        .map(decode_vm_storage)
        .collect::<Result<Vec<_>>>()
}

fn decode_required_vm_storage(value: &Value) -> Result<VMStorageDescriptor> {
    decode_vm_storage(value)?.ok_or_else(|| {
        InternalError("ABI descriptor cannot contain null VMStorage values".to_string())
    })
}

fn decode_vm_storage_matrix(value: &Value) -> Result<Vec<Vec<VMStorageDescriptor>>> {
    let (matrix_class, storage_arrays) = value.as_class_vec_ref()?;
    if matrix_class.name() != "[[Ljdk/internal/foreign/abi/VMStorage;" {
        return Err(InternalError(format!(
            "expected [[Ljdk/internal/foreign/abi/VMStorage;, found {}",
            matrix_class.name()
        )));
    }
    storage_arrays
        .iter()
        .map(|storage_array| {
            let (storage_array_class, storages) = storage_array.as_class_vec_ref()?;
            if storage_array_class.name() != "[Ljdk/internal/foreign/abi/VMStorage;" {
                return Err(InternalError(format!(
                    "expected [Ljdk/internal/foreign/abi/VMStorage;, found {}",
                    storage_array_class.name()
                )));
            }
            storages
                .iter()
                .map(decode_required_vm_storage)
                .collect::<Result<Vec<_>>>()
        })
        .collect()
}

fn decode_abi(reference: &ristretto_gc::Gc<GcRwLock<Reference>>) -> Result<AbiDescriptor> {
    let (
        architecture,
        input_storage,
        output_storage,
        volatile_storage,
        stack_alignment,
        shadow_space,
        scratch1,
        scratch2,
    ) = {
        let abi = reference.read();
        let abi = abi.as_object_ref()?;
        if abi.class().name() != "jdk/internal/foreign/abi/ABIDescriptor" {
            return Err(InternalError(format!(
                "expected jdk/internal/foreign/abi/ABIDescriptor, found {}",
                abi.class().name()
            )));
        }
        (
            abi.value("arch")?,
            abi.value("inputStorage")?,
            abi.value("outputStorage")?,
            abi.value("volatileStorage")?,
            abi.value("stackAlignment")?.as_i32()?,
            abi.value("shadowSpace")?.as_i32()?,
            abi.value("scratch1")?,
            abi.value("scratch2")?,
        )
    };

    Ok(AbiDescriptor {
        architecture: architecture.as_object_ref()?.class().name().to_string(),
        input_storage: decode_vm_storage_matrix(&input_storage)?,
        output_storage: decode_vm_storage_matrix(&output_storage)?,
        volatile_storage: decode_vm_storage_matrix(&volatile_storage)?,
        stack_alignment,
        shadow_space,
        scratch1: decode_required_vm_storage(&scratch1)?,
        scratch2: decode_required_vm_storage(&scratch2)?,
    })
}

fn validate_stub(stub: &DowncallStub, java_major_version: u16) -> Result<()> {
    if stub.method_type.parameters.len() != stub.argument_moves.len() {
        return Err(InternalError(format!(
            "downcall method type has {} parameters but {} argument moves",
            stub.method_type.parameters.len(),
            stub.argument_moves.len()
        )));
    }

    for (index, (parameter, argument_move)) in stub
        .method_type
        .parameters
        .iter()
        .zip(&stub.argument_moves)
        .enumerate()
    {
        if argument_move.is_none() && (java_major_version < 25 || parameter != "java.lang.Object") {
            return Err(InternalError(format!(
                "missing VMStorage for downcall parameter {index} of type {parameter}"
            )));
        }
    }

    if stub.return_moves.iter().any(Option::is_none) {
        return Err(InternalError(
            "downcall return moves cannot contain null VMStorage values".to_string(),
        ));
    }

    // NativeEntryPoint.make performs this assertion in Java. Keep the invariant at the VM
    // boundary as well so a directly-invoked intrinsic cannot create an unusable stub.
    if (stub.return_moves.len() > 1) != stub.needs_return_buffer {
        return Err(InternalError(
            "multiple-register return and return-buffer flag disagree".to_string(),
        ));
    }
    Ok(())
}

#[intrinsic_method(
    "jdk/internal/foreign/abi/NativeEntryPoint.freeDowncallStub0(J)Z",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn free_downcall_stub_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let downcall_stub = parameters.pop_long()?;
    let vm = thread.vm()?;
    let removed = downcall_stubs(vm.as_ref())?.remove(downcall_stub);
    Ok(Some(Value::from(removed)))
}

#[intrinsic_method(
    "jdk/internal/foreign/abi/NativeEntryPoint.makeDowncallStub(Ljava/lang/invoke/MethodType;Ljdk/internal/foreign/abi/ABIDescriptor;[Ljdk/internal/foreign/abi/VMStorage;[Ljdk/internal/foreign/abi/VMStorage;ZIZ)J",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn make_downcall_stub<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let needs_transition = parameters.pop_bool()?;
    let captured_state_mask = parameters.pop_int()?;
    let needs_return_buffer = parameters.pop_bool()?;
    let encoded_return_moves = parameters.pop_reference()?;
    let encoded_argument_moves = parameters.pop_reference()?;
    let abi = parameters.pop_reference()?;
    let method_type = parameters.pop_reference()?;

    let method_type = method_type
        .as_ref()
        .ok_or_else(|| InternalError("downcall method type cannot be null".to_string()))?;
    let abi = abi
        .as_ref()
        .ok_or_else(|| InternalError("downcall ABI descriptor cannot be null".to_string()))?;
    let encoded_argument_moves = encoded_argument_moves
        .as_ref()
        .ok_or_else(|| InternalError("downcall argument moves cannot be null".to_string()))?;
    let encoded_return_moves = encoded_return_moves
        .as_ref()
        .ok_or_else(|| InternalError("downcall return moves cannot be null".to_string()))?;

    let stub = DowncallStub {
        method_type: decode_method_type(method_type)?,
        abi: decode_abi(abi)?,
        argument_moves: decode_vm_storages(encoded_argument_moves)?,
        return_moves: decode_vm_storages(encoded_return_moves)?,
        needs_return_buffer,
        captured_state_mask,
        needs_transition,
    };
    let vm = thread.vm()?;
    validate_stub(&stub, vm.java_major_version())?;
    let handle = downcall_stubs(vm.as_ref())?.insert(stub);
    Ok(Some(Value::Long(handle)))
}

#[intrinsic_method(
    "jdk/internal/foreign/abi/NativeEntryPoint.registerNatives()V",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn register_natives<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    // Intrinsics are registered in Ristretto's method registry before class initialization.
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ristretto_classloader::Object;
    use ristretto_types::JavaObject;

    async fn method_type<T: Thread + 'static>(
        thread: &Arc<T>,
        parameters: &[&str],
        return_type: &str,
    ) -> Result<Value> {
        let vm = thread.vm()?;
        let mut decoded_parameters = Vec::with_capacity(parameters.len());
        for parameter in parameters {
            let parameter = thread.class(parameter).await?.to_object(thread).await?;
            decoded_parameters.push(parameter);
        }
        let return_type = thread.class(return_type).await?.to_object(thread).await?;

        let class_array = thread.class("[Ljava/lang/Class;").await?;
        let parameter_types = Value::new_object(
            vm.garbage_collector(),
            Reference::try_from((class_array, decoded_parameters))?,
        );
        let mut method_type = Object::new(thread.class("java/lang/invoke/MethodType").await?)?;
        method_type.set_value("ptypes", parameter_types)?;
        method_type.set_value("rtype", return_type)?;
        Ok(Value::from_object(vm.garbage_collector(), method_type))
    }

    async fn vm_storage<T: Thread + 'static>(thread: &Arc<T>) -> Result<Value> {
        let vm = thread.vm()?;
        let mut storage = Object::new(thread.class("jdk/internal/foreign/abi/VMStorage").await?)?;
        storage.set_value("type", Value::Int(0))?;
        storage.set_value("segmentMaskOrSize", Value::Int(15))?;
        storage.set_value("indexOrOffset", Value::Int(7))?;
        Ok(Value::from_object(vm.garbage_collector(), storage))
    }

    async fn vm_storage_array<T: Thread + 'static>(
        thread: &Arc<T>,
        storages: Vec<Value>,
    ) -> Result<Value> {
        let vm = thread.vm()?;
        let array_class = thread
            .class("[Ljdk/internal/foreign/abi/VMStorage;")
            .await?;
        Ok(Value::new_object(
            vm.garbage_collector(),
            Reference::try_from((array_class, storages))?,
        ))
    }

    async fn vm_storage_matrix<T: Thread + 'static>(thread: &Arc<T>) -> Result<Value> {
        let vm = thread.vm()?;
        let empty = vm_storage_array(thread, Vec::new()).await?;
        let matrix_class = thread
            .class("[[Ljdk/internal/foreign/abi/VMStorage;")
            .await?;
        Ok(Value::new_object(
            vm.garbage_collector(),
            Reference::try_from((matrix_class, vec![empty.clone(), empty.clone(), empty]))?,
        ))
    }

    async fn abi_descriptor<T: Thread + 'static>(thread: &Arc<T>) -> Result<Value> {
        let vm = thread.vm()?;
        let architecture = Object::new(thread.class("java/lang/Object").await?)?;
        let architecture = Value::from_object(vm.garbage_collector(), architecture);
        let mut abi = Object::new(
            thread
                .class("jdk/internal/foreign/abi/ABIDescriptor")
                .await?,
        )?;
        abi.set_value("arch", architecture)?;
        abi.set_value("inputStorage", vm_storage_matrix(thread).await?)?;
        abi.set_value("outputStorage", vm_storage_matrix(thread).await?)?;
        abi.set_value("volatileStorage", vm_storage_matrix(thread).await?)?;
        abi.set_value("stackAlignment", Value::Int(16))?;
        abi.set_value("shadowSpace", Value::Int(0))?;
        abi.set_value("scratch1", vm_storage(thread).await?)?;
        abi.set_value("scratch2", vm_storage(thread).await?)?;
        Ok(Value::from_object(vm.garbage_collector(), abi))
    }

    fn decoded_abi_descriptor() -> AbiDescriptor {
        let storage = VMStorageDescriptor {
            storage_type: 0,
            segment_mask_or_size: 15,
            index_or_offset: 7,
        };
        AbiDescriptor {
            architecture: "test/Architecture".to_string(),
            input_storage: Vec::new(),
            output_storage: Vec::new(),
            volatile_storage: Vec::new(),
            stack_alignment: 16,
            shadow_space: 0,
            scratch1: storage,
            scratch2: storage,
        }
    }

    async fn minimal_stub_parameters<T: Thread + 'static>(thread: &Arc<T>) -> Result<Parameters> {
        Ok(Parameters::new(vec![
            method_type(thread, &["long"], "void").await?,
            abi_descriptor(thread).await?,
            vm_storage_array(thread, vec![vm_storage(thread).await?]).await?,
            vm_storage_array(thread, Vec::new()).await?,
            Value::from(false),
            Value::Int(0),
            Value::from(false),
        ]))
    }

    #[tokio::test]
    async fn test_make_and_free_downcall_stub() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = minimal_stub_parameters(&thread).await?;
        let handle = make_downcall_stub(Arc::clone(&thread), parameters)
            .await?
            .expect("handle")
            .as_i64()?;
        assert_ne!(handle, 0);

        let result = free_downcall_stub_0(
            Arc::clone(&thread),
            Parameters::new(vec![Value::Long(handle)]),
        )
        .await?;
        assert_eq!(result, Some(Value::from(true)));

        let result =
            free_downcall_stub_0(thread, Parameters::new(vec![Value::Long(handle)])).await?;
        assert_eq!(result, Some(Value::from(false)));
        Ok(())
    }

    #[tokio::test]
    async fn test_free_zero_or_unknown_downcall_stub() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        for handle in [0, i64::MIN, i64::MAX] {
            let result = free_downcall_stub_0(
                Arc::clone(&thread),
                Parameters::new(vec![Value::Long(handle)]),
            )
            .await?;
            assert_eq!(result, Some(Value::from(false)));
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_downcall_stub_handles_are_unique() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let first_parameters = minimal_stub_parameters(&thread).await?;
        let second_parameters = minimal_stub_parameters(&thread).await?;
        let first = make_downcall_stub(Arc::clone(&thread), first_parameters)
            .await?
            .expect("first")
            .as_i64()?;
        let second = make_downcall_stub(thread, second_parameters)
            .await?
            .expect("second")
            .as_i64()?;
        assert_ne!(first, second);
        assert_ne!(first, 0);
        assert_ne!(second, 0);
        Ok(())
    }

    #[tokio::test]
    async fn test_downcall_stub_ownership_is_per_vm() -> Result<()> {
        let (_first_vm, first_thread) = crate::test::thread().await?;
        let (_second_vm, second_thread) = crate::test::thread().await?;
        let parameters = minimal_stub_parameters(&first_thread).await?;
        let handle = make_downcall_stub(Arc::clone(&first_thread), parameters)
            .await?
            .expect("handle")
            .as_i64()?;

        let result =
            free_downcall_stub_0(second_thread, Parameters::new(vec![Value::Long(handle)])).await?;
        assert_eq!(result, Some(Value::from(false)));

        let result =
            free_downcall_stub_0(first_thread, Parameters::new(vec![Value::Long(handle)])).await?;
        assert_eq!(result, Some(Value::from(true)));
        Ok(())
    }

    #[tokio::test]
    async fn test_make_downcall_stub_decodes_configuration() -> Result<()> {
        let (vm, thread) = crate::test::thread().await?;
        let method_type = method_type(&thread, &["long"], "void").await?;
        let argument_moves = vm_storage_array(&thread, vec![vm_storage(&thread).await?]).await?;
        let return_moves = vm_storage_array(&thread, Vec::new()).await?;

        let parameters = Parameters::new(vec![
            method_type,
            abi_descriptor(&thread).await?,
            argument_moves,
            return_moves,
            Value::from(false),
            Value::Int(4),
            Value::from(true),
        ]);
        let handle = make_downcall_stub(Arc::clone(&thread), parameters)
            .await?
            .expect("handle")
            .as_i64()?;

        let registry = downcall_stubs(vm.as_ref())?;
        let stubs = registry.stubs.read();
        let stub = stubs
            .get(&handle)
            .ok_or_else(|| InternalError("missing downcall stub".to_string()))?;
        assert_eq!(
            stub.method_type,
            MethodTypeDescriptor {
                parameters: vec!["long".to_string()],
                return_type: "void".to_string(),
            }
        );
        assert_eq!(
            stub.argument_moves,
            vec![Some(VMStorageDescriptor {
                storage_type: 0,
                segment_mask_or_size: 15,
                index_or_offset: 7,
            })]
        );
        assert_eq!(stub.return_moves, Vec::new());
        assert_eq!(stub.abi.stack_alignment, 16);
        assert_eq!(stub.abi.shadow_space, 0);
        assert_eq!(stub.abi.input_storage, vec![Vec::new(); 3]);
        assert_eq!(stub.abi.output_storage, vec![Vec::new(); 3]);
        assert_eq!(stub.abi.volatile_storage, vec![Vec::new(); 3]);
        assert_eq!(stub.captured_state_mask, 4);
        assert!(stub.needs_transition);
        Ok(())
    }

    #[test]
    fn test_validate_stub_return_buffer_invariant() {
        let storage = VMStorageDescriptor {
            storage_type: 0,
            segment_mask_or_size: 15,
            index_or_offset: 7,
        };
        let stub = DowncallStub {
            method_type: MethodTypeDescriptor {
                parameters: Vec::new(),
                return_type: "void".to_string(),
            },
            abi: decoded_abi_descriptor(),
            argument_moves: Vec::new(),
            return_moves: vec![Some(storage), Some(storage)],
            needs_return_buffer: false,
            captured_state_mask: 0,
            needs_transition: false,
        };
        assert!(validate_stub(&stub, 25).is_err());

        let stub = DowncallStub {
            needs_return_buffer: true,
            ..stub
        };
        assert!(validate_stub(&stub, 25).is_ok());
    }

    #[test]
    fn test_validate_stub_null_storage_rules() {
        let stub = DowncallStub {
            method_type: MethodTypeDescriptor {
                parameters: vec!["java.lang.Object".to_string()],
                return_type: "void".to_string(),
            },
            abi: decoded_abi_descriptor(),
            argument_moves: vec![None],
            return_moves: Vec::new(),
            needs_return_buffer: false,
            captured_state_mask: 0,
            needs_transition: false,
        };
        assert!(validate_stub(&stub, 21).is_err());
        assert!(validate_stub(&stub, 25).is_ok());

        let stub = DowncallStub {
            method_type: MethodTypeDescriptor {
                parameters: Vec::new(),
                return_type: "long".to_string(),
            },
            argument_moves: Vec::new(),
            return_moves: vec![None],
            ..stub
        };
        assert!(validate_stub(&stub, 25).is_err());
    }

    #[tokio::test]
    async fn test_make_downcall_stub_rejects_null_metadata() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![
            Value::Object(None),
            Value::Object(None),
            Value::Object(None),
            Value::Object(None),
            Value::from(false),
            Value::Int(0),
            Value::from(false),
        ]);
        let error = make_downcall_stub(thread, parameters)
            .await
            .expect_err("null metadata must be rejected");
        assert!(error.to_string().contains("method type cannot be null"));
        Ok(())
    }

    #[tokio::test]
    async fn test_nullable_argument_moves_are_jdk_25_only() -> Result<()> {
        let (_java21_vm, java21_thread) = crate::test::java21_thread().await?;
        let java21_parameters = Parameters::new(vec![
            method_type(&java21_thread, &["java/lang/Object"], "void").await?,
            abi_descriptor(&java21_thread).await?,
            vm_storage_array(&java21_thread, vec![Value::Object(None)]).await?,
            vm_storage_array(&java21_thread, Vec::new()).await?,
            Value::from(false),
            Value::Int(0),
            Value::from(false),
        ]);
        assert!(
            make_downcall_stub(java21_thread, java21_parameters)
                .await
                .is_err()
        );

        let (_java25_vm, java25_thread) = crate::test::java25_thread().await?;
        let java25_parameters = Parameters::new(vec![
            method_type(&java25_thread, &["java/lang/Object"], "void").await?,
            abi_descriptor(&java25_thread).await?,
            vm_storage_array(&java25_thread, vec![Value::Object(None)]).await?,
            vm_storage_array(&java25_thread, Vec::new()).await?,
            Value::from(false),
            Value::Int(0),
            Value::from(false),
        ]);
        let handle = make_downcall_stub(java25_thread, java25_parameters)
            .await?
            .expect("handle")
            .as_i64()?;
        assert_ne!(handle, 0);
        Ok(())
    }

    #[tokio::test]
    async fn test_register_natives() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = register_natives(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }
}
