use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `jdk.vm.ci.hotspot.CompilerToVM`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "jdk/vm/ci/hotspot/CompilerToVM";
    registry.register(
        class_name,
        "allocateCompileId",
        "(Ljdk/vm/ci/hotspot/HotSpotResolvedJavaMethodImpl;I)I",
        allocate_compile_id,
    );
    registry.register(
        class_name,
        "asResolvedJavaMethod",
        "(Ljava/lang/reflect/Executable;)Ljdk/vm/ci/hotspot/HotSpotResolvedJavaMethodImpl;",
        as_resolved_java_method,
    );
    registry.register(class_name, "collectCounters", "()[J", collect_counters);
    registry.register(
        class_name,
        "compileToBytecode",
        "(Ljava/lang/Object;)V",
        compile_to_bytecode,
    );
    registry.register(
        class_name,
        "constantPoolRemapInstructionOperandFromCache",
        "(Ljdk/vm/ci/hotspot/HotSpotConstantPool;I)I",
        constant_pool_remap_instruction_operand_from_cache,
    );
    registry.register(
        class_name,
        "disassembleCodeBlob",
        "(Ljdk/vm/ci/code/InstalledCode;)Ljava/lang/String;",
        disassemble_code_blob,
    );
    registry.register(
        class_name,
        "ensureLinked",
        "(Ljdk/vm/ci/hotspot/HotSpotResolvedObjectTypeImpl;)V",
        ensure_linked,
    );
    registry.register(
        class_name,
        "executeInstalledCode",
        "([Ljava/lang/Object;Ljdk/vm/ci/code/InstalledCode;)Ljava/lang/Object;",
        execute_installed_code,
    );
    registry.register(class_name, "findUniqueConcreteMethod", "(Ljdk/vm/ci/hotspot/HotSpotResolvedObjectTypeImpl;Ljdk/vm/ci/hotspot/HotSpotResolvedJavaMethodImpl;)Ljdk/vm/ci/hotspot/HotSpotResolvedJavaMethodImpl;", find_unique_concrete_method);
    registry.register(class_name, "flushDebugOutput", "()V", flush_debug_output);
    registry.register(
        class_name,
        "getBytecode",
        "(Ljdk/vm/ci/hotspot/HotSpotResolvedJavaMethodImpl;)[B",
        get_bytecode,
    );
    registry.register(class_name, "getClassInitializer", "(Ljdk/vm/ci/hotspot/HotSpotResolvedObjectTypeImpl;)Ljdk/vm/ci/hotspot/HotSpotResolvedJavaMethodImpl;", get_class_initializer);
    registry.register(
        class_name,
        "getConstantPool",
        "(Ljava/lang/Object;)Ljdk/vm/ci/hotspot/HotSpotConstantPool;",
        get_constant_pool,
    );
    registry.register(
        class_name,
        "getExceptionTableLength",
        "(Ljdk/vm/ci/hotspot/HotSpotResolvedJavaMethodImpl;)I",
        get_exception_table_length,
    );
    registry.register(
        class_name,
        "getExceptionTableStart",
        "(Ljdk/vm/ci/hotspot/HotSpotResolvedJavaMethodImpl;)J",
        get_exception_table_start,
    );
    registry.register(class_name, "getFingerprint", "(J)J", get_fingerprint);
    registry.register(
        class_name,
        "getFlagValue",
        "(Ljava/lang/String;)Ljava/lang/Object;",
        get_flag_value,
    );
    registry.register(class_name, "getHostClass", "(Ljdk/vm/ci/hotspot/HotSpotResolvedObjectTypeImpl;)Ljdk/vm/ci/hotspot/HotSpotResolvedObjectTypeImpl;", get_host_class);
    registry.register(class_name, "getImplementor", "(Ljdk/vm/ci/hotspot/HotSpotResolvedObjectTypeImpl;)Ljdk/vm/ci/hotspot/HotSpotResolvedObjectTypeImpl;", get_implementor);
    registry.register(
        class_name,
        "getLineNumberTable",
        "(Ljdk/vm/ci/hotspot/HotSpotResolvedJavaMethodImpl;)[J",
        get_line_number_table,
    );
    registry.register(
        class_name,
        "getLocalVariableTableLength",
        "(Ljdk/vm/ci/hotspot/HotSpotResolvedJavaMethodImpl;)I",
        get_local_variable_table_length,
    );
    registry.register(
        class_name,
        "getLocalVariableTableStart",
        "(Ljdk/vm/ci/hotspot/HotSpotResolvedJavaMethodImpl;)J",
        get_local_variable_table_start,
    );
    registry.register(
        class_name,
        "getMaxCallTargetOffset",
        "(J)J",
        get_max_call_target_offset,
    );
    registry.register(class_name, "getMetadata", "(Ljdk/vm/ci/code/TargetDescription;Ljdk/vm/ci/hotspot/HotSpotCompiledCode;Ljdk/vm/ci/hotspot/HotSpotMetaData;)I", get_metadata);
    registry.register(
        class_name,
        "getResolvedJavaMethod",
        "(Ljava/lang/Object;J)Ljdk/vm/ci/hotspot/HotSpotResolvedJavaMethodImpl;",
        get_resolved_java_method,
    );
    registry.register(
        class_name,
        "getResolvedJavaType",
        "(Ljava/lang/Object;JZ)Ljdk/vm/ci/hotspot/HotSpotResolvedObjectTypeImpl;",
        get_resolved_java_type,
    );
    registry.register(
        class_name,
        "getSignaturePolymorphicHolders",
        "()[Ljava/lang/String;",
        get_signature_polymorphic_holders,
    );
    registry.register(
        class_name,
        "getStackTraceElement",
        "(Ljdk/vm/ci/hotspot/HotSpotResolvedJavaMethodImpl;I)Ljava/lang/StackTraceElement;",
        get_stack_trace_element,
    );
    registry.register(class_name, "getSymbol", "(J)Ljava/lang/String;", get_symbol);
    registry.register(class_name, "getVtableIndexForInterfaceMethod", "(Ljdk/vm/ci/hotspot/HotSpotResolvedObjectTypeImpl;Ljdk/vm/ci/hotspot/HotSpotResolvedJavaMethodImpl;)I", get_vtable_index_for_interface_method);
    registry.register(
        class_name,
        "hasCompiledCodeForOSR",
        "(Ljdk/vm/ci/hotspot/HotSpotResolvedJavaMethodImpl;II)Z",
        has_compiled_code_for_osr,
    );
    registry.register(
        class_name,
        "hasFinalizableSubclass",
        "(Ljdk/vm/ci/hotspot/HotSpotResolvedObjectTypeImpl;)Z",
        has_finalizable_subclass,
    );
    registry.register(
        class_name,
        "hasNeverInlineDirective",
        "(Ljdk/vm/ci/hotspot/HotSpotResolvedJavaMethodImpl;)Z",
        has_never_inline_directive,
    );
    registry.register(class_name, "installCode", "(Ljdk/vm/ci/code/TargetDescription;Ljdk/vm/ci/hotspot/HotSpotCompiledCode;Ljdk/vm/ci/code/InstalledCode;Ljdk/vm/ci/hotspot/HotSpotSpeculationLog;)I", install_code);
    registry.register(
        class_name,
        "interpreterFrameSize",
        "(Ljdk/vm/ci/code/BytecodeFrame;)I",
        interpreter_frame_size,
    );
    registry.register(
        class_name,
        "invalidateInstalledCode",
        "(Ljdk/vm/ci/code/InstalledCode;)V",
        invalidate_installed_code,
    );
    registry.register(
        class_name,
        "isCompilable",
        "(Ljdk/vm/ci/hotspot/HotSpotResolvedJavaMethodImpl;)Z",
        is_compilable,
    );
    registry.register(class_name, "isMature", "(J)Z", is_mature);
    registry.register(
        class_name,
        "isResolvedInvokeHandleInPool",
        "(Ljdk/vm/ci/hotspot/HotSpotConstantPool;I)I",
        is_resolved_invoke_handle_in_pool,
    );
    registry.register(class_name, "iterateFrames", "([Ljdk/vm/ci/meta/ResolvedJavaMethod;[Ljdk/vm/ci/meta/ResolvedJavaMethod;ILjdk/vm/ci/code/stack/InspectedFrameVisitor;)Ljava/lang/Object;", iterate_frames);
    registry.register(
        class_name,
        "lookupAppendixInPool",
        "(Ljdk/vm/ci/hotspot/HotSpotConstantPool;I)Ljava/lang/Object;",
        lookup_appendix_in_pool,
    );
    registry.register(
        class_name,
        "lookupKlassInPool",
        "(Ljdk/vm/ci/hotspot/HotSpotConstantPool;I)Ljava/lang/Object;",
        lookup_klass_in_pool,
    );
    registry.register(
        class_name,
        "lookupKlassRefIndexInPool",
        "(Ljdk/vm/ci/hotspot/HotSpotConstantPool;I)I",
        lookup_klass_ref_index_in_pool,
    );
    registry.register(class_name, "lookupMethodInPool", "(Ljdk/vm/ci/hotspot/HotSpotConstantPool;IB)Ljdk/vm/ci/hotspot/HotSpotResolvedJavaMethodImpl;", lookup_method_in_pool);
    registry.register(
        class_name,
        "lookupNameAndTypeRefIndexInPool",
        "(Ljdk/vm/ci/hotspot/HotSpotConstantPool;I)I",
        lookup_name_and_type_ref_index_in_pool,
    );
    registry.register(
        class_name,
        "lookupNameInPool",
        "(Ljdk/vm/ci/hotspot/HotSpotConstantPool;I)Ljava/lang/String;",
        lookup_name_in_pool,
    );
    registry.register(
        class_name,
        "lookupSignatureInPool",
        "(Ljdk/vm/ci/hotspot/HotSpotConstantPool;I)Ljava/lang/String;",
        lookup_signature_in_pool,
    );
    registry.register(
        class_name,
        "lookupType",
        "(Ljava/lang/String;Ljava/lang/Class;Z)Ljdk/vm/ci/hotspot/HotSpotResolvedObjectTypeImpl;",
        lookup_type,
    );
    registry.register(
        class_name,
        "materializeVirtualObjects",
        "(Ljdk/vm/ci/hotspot/HotSpotStackFrameReference;Z)V",
        materialize_virtual_objects,
    );
    registry.register(
        class_name,
        "methodDataProfileDataSize",
        "(JI)I",
        method_data_profile_data_size,
    );
    registry.register(
        class_name,
        "methodIsIgnoredBySecurityStackWalk",
        "(Ljdk/vm/ci/hotspot/HotSpotResolvedJavaMethodImpl;)Z",
        method_is_ignored_by_security_stack_walk,
    );
    registry.register(
        class_name,
        "readConfiguration",
        "()[Ljava/lang/Object;",
        read_configuration,
    );
    registry.register(class_name, "registerNatives", "()V", register_natives);
    registry.register(
        class_name,
        "reprofile",
        "(Ljdk/vm/ci/hotspot/HotSpotResolvedJavaMethodImpl;)V",
        reprofile,
    );
    registry.register(
        class_name,
        "resetCompilationStatistics",
        "()V",
        reset_compilation_statistics,
    );
    registry.register(
        class_name,
        "resolveConstantInPool",
        "(Ljdk/vm/ci/hotspot/HotSpotConstantPool;I)Ljava/lang/Object;",
        resolve_constant_in_pool,
    );
    registry.register(class_name, "resolveFieldInPool", "(Ljdk/vm/ci/hotspot/HotSpotConstantPool;ILjdk/vm/ci/hotspot/HotSpotResolvedJavaMethodImpl;B[I)Ljdk/vm/ci/hotspot/HotSpotResolvedObjectTypeImpl;", resolve_field_in_pool);
    registry.register(
        class_name,
        "resolveInvokeDynamicInPool",
        "(Ljdk/vm/ci/hotspot/HotSpotConstantPool;I)V",
        resolve_invoke_dynamic_in_pool,
    );
    registry.register(
        class_name,
        "resolveInvokeHandleInPool",
        "(Ljdk/vm/ci/hotspot/HotSpotConstantPool;I)V",
        resolve_invoke_handle_in_pool,
    );
    registry.register(class_name, "resolveMethod", "(Ljdk/vm/ci/hotspot/HotSpotResolvedObjectTypeImpl;Ljdk/vm/ci/hotspot/HotSpotResolvedJavaMethodImpl;Ljdk/vm/ci/hotspot/HotSpotResolvedObjectTypeImpl;)Ljdk/vm/ci/hotspot/HotSpotResolvedJavaMethodImpl;", resolve_method);
    registry.register(
        class_name,
        "resolvePossiblyCachedConstantInPool",
        "(Ljdk/vm/ci/hotspot/HotSpotConstantPool;I)Ljava/lang/Object;",
        resolve_possibly_cached_constant_in_pool,
    );
    registry.register(class_name, "resolveTypeInPool", "(Ljdk/vm/ci/hotspot/HotSpotConstantPool;I)Ljdk/vm/ci/hotspot/HotSpotResolvedObjectTypeImpl;", resolve_type_in_pool);
    registry.register(
        class_name,
        "setNotInlinableOrCompilable",
        "(Ljdk/vm/ci/hotspot/HotSpotResolvedJavaMethodImpl;)V",
        set_not_inlinable_or_compilable,
    );
    registry.register(
        class_name,
        "shouldDebugNonSafepoints",
        "()Z",
        should_debug_non_safepoints,
    );
    registry.register(
        class_name,
        "shouldInlineMethod",
        "(Ljdk/vm/ci/hotspot/HotSpotResolvedJavaMethodImpl;)Z",
        should_inline_method,
    );
    registry.register(
        class_name,
        "writeDebugOutput",
        "([BII)V",
        write_debug_output,
    );
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn allocate_compile_id(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn as_resolved_java_method(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn collect_counters(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn compile_to_bytecode(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn constant_pool_remap_instruction_operand_from_cache(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn disassemble_code_blob(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn ensure_linked(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn execute_installed_code(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn find_unique_concrete_method(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn flush_debug_output(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_bytecode(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_class_initializer(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_constant_pool(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_exception_table_length(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_exception_table_start(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_fingerprint(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_flag_value(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_host_class(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_implementor(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_line_number_table(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_local_variable_table_length(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_local_variable_table_start(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_max_call_target_offset(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_metadata(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_resolved_java_method(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_resolved_java_type(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_signature_polymorphic_holders(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_stack_trace_element(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_symbol(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_vtable_index_for_interface_method(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn has_compiled_code_for_osr(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn has_finalizable_subclass(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn has_never_inline_directive(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn install_code(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn interpreter_frame_size(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn invalidate_installed_code(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn is_compilable(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn is_mature(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn is_resolved_invoke_handle_in_pool(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn iterate_frames(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn lookup_appendix_in_pool(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn lookup_klass_in_pool(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn lookup_klass_ref_index_in_pool(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn lookup_method_in_pool(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn lookup_name_and_type_ref_index_in_pool(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn lookup_name_in_pool(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn lookup_signature_in_pool(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn lookup_type(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn materialize_virtual_objects(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn method_data_profile_data_size(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn method_is_ignored_by_security_stack_walk(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn read_configuration(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn register_natives(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn reprofile(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn reset_compilation_statistics(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn resolve_constant_in_pool(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn resolve_field_in_pool(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn resolve_invoke_dynamic_in_pool(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn resolve_invoke_handle_in_pool(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn resolve_method(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn resolve_possibly_cached_constant_in_pool(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn resolve_type_in_pool(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn set_not_inlinable_or_compilable(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn should_debug_non_safepoints(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn should_inline_method(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn write_debug_output(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
