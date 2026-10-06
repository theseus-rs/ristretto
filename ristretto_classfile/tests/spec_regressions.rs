//! Regression cases discovered by differential class-file verification against `OpenJDK`.
#![expect(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    reason = "small generated class-file test fixtures"
)]
use ristretto_classfile::attributes::*;
use ristretto_classfile::*;

use ristretto_classfile::verifiers::bytecode::{VerifierConfig, verify_class};
use ristretto_classfile::verifiers::context::VerificationContext;
use ristretto_classfile::verifiers::error::Result as VResult;

struct Context;
impl VerificationContext for Context {
    fn is_subclass(&self, source: &str, target: &str) -> VResult<bool> {
        self.is_assignable(target, source)
    }
    fn is_assignable(&self, target: &str, source: &str) -> VResult<bool> {
        Ok(target == source
            || target == "java/lang/Object"
            || (source == "Audit" && target == "Parent"))
    }
    fn common_superclass(&self, a: &str, b: &str) -> VResult<String> {
        Ok(if a == b { a } else { "java/lang/Object" }.to_string())
    }
}
fn base() -> ClassFile<'static> {
    let mut c = ClassFile {
        version: JAVA_25,
        access_flags: ClassAccessFlags::PUBLIC,
        ..Default::default()
    };
    c.this_class = c.constant_pool.add_class("Audit").unwrap();
    c.super_class = c.constant_pool.add_class("java/lang/Object").unwrap();
    c
}
fn method(
    c: &mut ClassFile<'static>,
    name: &str,
    desc: &str,
    stack: u16,
    locals: u16,
    code: Vec<Instruction>,
    frames: Option<Vec<StackFrame>>,
) -> Method {
    let name_index = c.constant_pool.add_utf8(name).unwrap();
    let descriptor_index = c.constant_pool.add_utf8(desc).unwrap();
    let code_name = c.constant_pool.add_utf8("Code").unwrap();
    let attributes = frames
        .map(|frames| {
            vec![Attribute::StackMapTable {
                name_index: c.constant_pool.add_utf8("StackMapTable").unwrap(),
                frames,
            }]
        })
        .unwrap_or_default();
    Method {
        access_flags: MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
        name_index,
        descriptor_index,
        attributes: vec![Attribute::Code {
            name_index: code_name,
            max_stack: stack,
            max_locals: locals,
            code,
            exception_table: vec![],
            attributes,
        }],
    }
}
fn full(
    offset_delta: u16,
    locals: Vec<VerificationType>,
    stack: Vec<VerificationType>,
) -> StackFrame {
    StackFrame::FullFrame {
        frame_type: 255,
        offset_delta,
        locals,
        stack,
    }
}

use Instruction::*;
use VerificationType as T;

fn check(mut class: ClassFile<'static>, method: Option<Method>, valid: bool) {
    if let Some(method) = method {
        class.methods.push(method);
    }
    let result = verify_class(&class, &Context, &VerifierConfig::strict());
    assert_eq!(result.is_ok(), valid, "{result:?}");
    if valid {
        let mut bytes = Vec::new();
        class.to_bytes(&mut bytes).unwrap();
        let parsed = ClassFile::from_bytes(&bytes).unwrap();
        verify_class(&parsed, &Context, &VerifierConfig::strict()).unwrap();
        let mut encoded = Vec::new();
        parsed.to_bytes(&mut encoded).unwrap();
        assert_eq!(bytes, encoded);
    }
}
#[test]
fn baseline() {
    check(base(), None, true);
}

#[test]
fn missing_super() {
    let mut c = base();
    c.super_class = 0;
    check(c, None, false);
}

#[test]
fn self_super() {
    let mut c = base();
    c.super_class = c.this_class;
    check(c, None, false);
}

#[test]
fn array_this() {
    let mut c = base();
    c.this_class = c.constant_pool.add_class("[I").unwrap();
    check(c, None, false);
}

#[test]
fn duplicate_interfaces() {
    let mut c = base();
    c.interfaces = vec![c.super_class, c.super_class];
    check(c, None, false);
}

#[test]
fn invalid_module() {
    let mut c = base();
    c.access_flags = ClassAccessFlags::MODULE | ClassAccessFlags::PUBLIC;
    check(c, None, false);
}

#[test]
fn invalid_method_name_descriptor() {
    let mut c = base();
    let m = method(&mut c, "bad/name", "invalid", 0, 0, vec![], None);
    check(c, Some(m), false);
}

#[test]
fn missing_code() {
    let mut c = base();
    let mut m = method(&mut c, "f", "()V", 0, 0, vec![Return], None);
    m.attributes.clear();
    check(c, Some(m), false);
}

#[test]
fn duplicate_methods() {
    let mut c = base();
    let m = method(&mut c, "f", "()V", 0, 0, vec![Return], None);
    c.methods.push(m.clone());
    check(c, Some(m), false);
}

#[test]
fn duplicate_code() {
    let mut c = base();
    let mut m = method(&mut c, "f", "()V", 0, 0, vec![Return], None);
    m.attributes.push(m.attributes[0].clone());
    check(c, Some(m), false);
}

#[test]
fn native_code() {
    let mut c = base();
    let mut m = method(&mut c, "f", "()V", 0, 0, vec![Return], None);
    m.access_flags |= MethodAccessFlags::NATIVE;
    check(c, Some(m), false);
}

#[test]
fn empty_code() {
    let mut c = base();
    let m = method(&mut c, "f", "()V", 0, 0, vec![], None);
    check(c, Some(m), false);
}

#[test]
fn stack_underflow() {
    let mut c = base();
    let m = method(&mut c, "f", "()V", 0, 0, vec![Pop, Return], None);
    check(c, Some(m), false);
}

#[test]
fn wrong_void_return() {
    let mut c = base();
    let m = method(&mut c, "f", "()I", 0, 0, vec![Return], None);
    check(c, Some(m), false);
}

#[test]
fn wrong_typed_return() {
    let mut c = base();
    let m = method(&mut c, "f", "()V", 1, 0, vec![Iconst_0, Ireturn], None);
    check(c, Some(m), false);
}

#[test]
fn uninitialized_constructor() {
    let mut c = base();
    let mut m = method(&mut c, "<init>", "()V", 0, 1, vec![Return], None);
    m.access_flags = MethodAccessFlags::PUBLIC;
    check(c, Some(m), false);
}

#[test]
fn byte_array_as_int_array() {
    let mut c = base();
    let m = method(&mut c, "f", "([B)[I", 1, 1, vec![Aload_0, Areturn], None);
    check(c, Some(m), false);
}

#[test]
fn iaload_byte_array() {
    let mut c = base();
    let m = method(
        &mut c,
        "f",
        "([B)I",
        2,
        1,
        vec![Aload_0, Iconst_0, Iaload, Ireturn],
        None,
    );
    check(c, Some(m), false);
}

#[test]
fn backedge_type_mismatch() {
    let mut c = base();
    let m = method(
        &mut c,
        "f",
        "(I)V",
        1,
        1,
        vec![Iload_0, Pop, Fconst_0, Fstore_0, Goto(0)],
        Some(vec![StackFrame::SameFrame { frame_type: 0 }]),
    );
    check(c, Some(m), false);
}

#[test]
fn initial_frame_overridden() {
    let mut c = base();
    let m = method(
        &mut c,
        "f",
        "(I)V",
        1,
        1,
        vec![Fload_0, Pop, Return],
        Some(vec![full(0, vec![T::Float], vec![])]),
    );
    check(c, Some(m), false);
}

#[test]
fn top_local_promoted() {
    let mut c = base();
    let m = method(
        &mut c,
        "f",
        "()V",
        1,
        1,
        vec![Nop, Nop, Iload_0, Pop, Return],
        Some(vec![full(2, vec![T::Integer], vec![])]),
    );
    check(c, Some(m), false);
}

#[test]
fn unchecked_dead_code() {
    let mut c = base();
    let m = method(&mut c, "f", "()V", 0, 0, vec![Return, Pop, Return], None);
    check(c, Some(m), false);
}

#[test]
fn valid_branch_stackmap() {
    let mut c = base();
    let m = method(
        &mut c,
        "f",
        "()V",
        0,
        0,
        vec![Goto(1), Return],
        Some(vec![StackFrame::SameFrame { frame_type: 1 }]),
    );
    check(c, Some(m), true);
}

#[test]
fn frames_at_zero_and_one() {
    let mut c = base();
    let m = method(
        &mut c,
        "f",
        "()V",
        0,
        0,
        vec![Nop, Return],
        Some(vec![
            StackFrame::SameFrame { frame_type: 0 },
            StackFrame::SameFrame { frame_type: 0 },
        ]),
    );
    check(c, Some(m), true);
}

#[test]
fn oversized_frame_truncated() {
    let mut c = base();
    let m = method(
        &mut c,
        "f",
        "()V",
        0,
        0,
        vec![Return],
        Some(vec![full(0, vec![T::Integer], vec![T::Integer])]),
    );
    check(c, Some(m), false);
}

#[test]
fn valid_long_stackmap() {
    let mut c = base();
    let m = method(
        &mut c,
        "f",
        "()V",
        2,
        0,
        vec![Lconst_0, Nop, Pop2, Return],
        Some(vec![StackFrame::SameLocals1StackItemFrame {
            frame_type: 65,
            stack: vec![T::Long],
        }]),
    );
    check(c, Some(m), true);
}

#[test]
fn valid_append_frame() {
    let mut c = base();
    let m = method(
        &mut c,
        "f",
        "()V",
        1,
        1,
        vec![Iconst_0, Istore_0, Nop, Iload_0, Pop, Return],
        Some(vec![StackFrame::AppendFrame {
            frame_type: 252,
            offset_delta: 2,
            locals: vec![T::Integer],
        }]),
    );
    check(c, Some(m), true);
}

#[test]
fn valid_array_stackmap() {
    let mut c = base();
    let array = c.constant_pool.add_class("[I").unwrap();
    let m = method(
        &mut c,
        "f",
        "([I)I",
        2,
        1,
        vec![Aload_0, Iconst_0, Iaload, Ireturn],
        Some(vec![full(
            0,
            vec![T::Object { cpool_index: array }],
            vec![],
        )]),
    );
    check(c, Some(m), true);
}

#[test]
fn wrong_constructor_owner() {
    let mut c = base();
    let a = c.constant_pool.add_class("java/lang/Object").unwrap();
    let b = c.constant_pool.add_class("java/lang/String").unwrap();
    let nat = c.constant_pool.add_name_and_type("<init>", "()V").unwrap();
    let init = c
        .constant_pool
        .add(Constant::MethodRef {
            class_index: b,
            name_and_type_index: nat,
        })
        .unwrap();
    let m = method(
        &mut c,
        "f",
        "()V",
        1,
        0,
        vec![New(a), Invokespecial(init), Return],
        None,
    );
    check(c, Some(m), false);
}

#[test]
fn checkcast_initializes_object() {
    let mut c = base();
    let a = c.constant_pool.add_class("java/lang/Object").unwrap();
    let m = method(
        &mut c,
        "f",
        "()Ljava/lang/Object;",
        1,
        0,
        vec![New(a), Checkcast(a), Areturn],
        None,
    );
    check(c, Some(m), false);
}

#[test]
fn invalid_instanceof_index() {
    let mut c = base();
    let m = method(
        &mut c,
        "f",
        "()V",
        1,
        0,
        vec![Aconst_null, Instanceof(65535), Pop, Return],
        None,
    );
    check(c, Some(m), false);
}

#[test]
fn zero_multianewarray_dimensions() {
    let mut c = base();
    let a = c.constant_pool.add_class("[I").unwrap();
    let m = method(
        &mut c,
        "f",
        "()V",
        1,
        0,
        vec![Multianewarray(a, 0), Pop, Return],
        None,
    );
    check(c, Some(m), false);
}

#[test]
fn new_array_class() {
    let mut c = base();
    let a = c.constant_pool.add_class("[I").unwrap();
    let m = method(&mut c, "f", "()V", 1, 0, vec![New(a), Pop, Return], None);
    check(c, Some(m), false);
}

#[test]
fn int_dynamic_constant() {
    let mut c = base();
    let nat = c.constant_pool.add_name_and_type("x", "I").unwrap();
    let d = c
        .constant_pool
        .add(Constant::Dynamic {
            bootstrap_method_attr_index: 0,
            name_and_type_index: nat,
        })
        .unwrap();
    let m = method(&mut c, "f", "()I", 1, 0, vec![Ldc_w(d), Ireturn], None);
    check(c, Some(m), false);
}

#[test]
fn valid_long_legacy_method() {
    let mut c = base();
    c.version = JAVA_5;
    let mut code = vec![Nop; 1001];
    code.push(Return);
    let m = method(&mut c, "f", "()V", 0, 0, code, None);
    check(c, Some(m), true);
}

#[test]
fn valid_legacy_jsr() {
    let mut c = base();
    c.version = JAVA_5;
    let m = method(
        &mut c,
        "f",
        "()V",
        1,
        1,
        vec![Jsr(2), Return, Astore_0, Ret(0)],
        None,
    );
    check(c, Some(m), true);
}

#[test]
fn stackmap_overflow_discarded() {
    let mut c = base();
    let m = method(
        &mut c,
        "f",
        "()V",
        0,
        0,
        vec![Nop, Return],
        Some(vec![full(1, vec![], vec![T::Integer])]),
    );
    check(c, Some(m), false);
}

#[test]
fn handler_integer_stack() {
    let mut c = base();
    let mut m = method(
        &mut c,
        "f",
        "()V",
        1,
        0,
        vec![Nop, Return, Pop, Return],
        Some(vec![full(2, vec![], vec![T::Integer])]),
    );
    if let Attribute::Code {
        exception_table, ..
    } = &mut m.attributes[0]
    {
        exception_table.push(ExceptionTableEntry {
            range_pc: 0..1,
            handler_pc: 2,
            catch_type: 0,
        });
    }
    check(c, Some(m), false);
}

#[test]
fn catch_non_throwable() {
    let mut c = base();
    let object = c.super_class;
    let mut m = method(
        &mut c,
        "f",
        "()V",
        1,
        0,
        vec![Nop, Return, Pop, Return],
        Some(vec![full(
            2,
            vec![],
            vec![T::Object {
                cpool_index: object,
            }],
        )]),
    );
    if let Attribute::Code {
        exception_table, ..
    } = &mut m.attributes[0]
    {
        exception_table.push(ExceptionTableEntry {
            range_pc: 0..1,
            handler_pc: 2,
            catch_type: c.super_class,
        });
    }
    check(c, Some(m), false);
}

#[test]
fn valid_empty_permitted() {
    let mut c = base();
    let idx = c.constant_pool.add_utf8("PermittedSubclasses").unwrap();
    c.attributes.push(Attribute::PermittedSubclasses {
        name_index: idx,
        class_indexes: vec![],
    });
    check(c, None, true);
}

#[test]
fn duplicate_source_file() {
    let mut c = base();
    let idx = c.constant_pool.add_utf8("SourceFile").unwrap();
    let val = c.constant_pool.add_utf8("Audit.java").unwrap();
    let a = Attribute::SourceFile {
        name_index: idx,
        source_file_index: val,
    };
    c.attributes = vec![a.clone(), a];
    check(c, None, false);
}

#[test]
fn duplicate_nest_members() {
    let mut c = base();
    let idx = c.constant_pool.add_utf8("NestMembers").unwrap();
    c.attributes.push(Attribute::NestMembers {
        name_index: idx,
        class_indexes: vec![c.super_class, c.super_class],
    });
    check(c, None, true);
}

#[test]
fn duplicate_permitted_members() {
    let mut c = base();
    let idx = c.constant_pool.add_utf8("PermittedSubclasses").unwrap();
    c.attributes.push(Attribute::PermittedSubclasses {
        name_index: idx,
        class_indexes: vec![c.super_class, c.super_class],
    });
    check(c, None, true);
}

#[test]
fn valid_record_unusual_name() {
    let mut c = base();
    let idx = c.constant_pool.add_utf8("Record").unwrap();
    let n = c.constant_pool.add_utf8("x").unwrap();
    let d = c.constant_pool.add_utf8("Lfoo-bar;").unwrap();
    c.attributes.push(Attribute::Record {
        name_index: idx,
        records: vec![Record {
            name_index: n,
            descriptor_index: d,
            attributes: vec![],
        }],
    });
    check(c, None, true);
}

#[test]
fn wrong_constant_value_type() {
    let mut class = base();
    let name_index = class.constant_pool.add_utf8("x").unwrap();
    let descriptor_index = class.constant_pool.add_utf8("I").unwrap();
    let attribute_name = class.constant_pool.add_utf8("ConstantValue").unwrap();
    let value_index = class.constant_pool.add_long(42).unwrap();
    class.fields.push(Field {
        access_flags: FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC,
        name_index,
        descriptor_index,
        field_type: FieldType::Base(BaseType::Int),
        attributes: vec![Attribute::ConstantValue {
            name_index: attribute_name,
            constant_value_index: value_index,
        }],
    });
    check(class, None, false);
}

#[test]
fn invalid_methodtype_descriptor() {
    let mut c = base();
    c.constant_pool.add_method_type("not-a-method").unwrap();
    check(c, None, false);
}

#[test]
fn class_initializer_name_and_type_in_enclosing_method() {
    let mut c = base();
    let method_index = c
        .constant_pool
        .add_name_and_type("<clinit>", "()V")
        .unwrap();
    c.attributes.push(Attribute::EnclosingMethod {
        name_index: c.constant_pool.add_utf8("EnclosingMethod").unwrap(),
        class_index: c.super_class,
        method_index,
    });
    check(c, None, true);
}

#[test]
fn class_initializer_method_reference_is_rejected() {
    let mut c = base();
    c.constant_pool
        .add_method_ref(c.super_class, "<clinit>", "()V")
        .unwrap();
    check(c, None, false);
}

#[test]
fn invalid_newinvokespecial_handle() {
    let mut c = base();
    let r = c
        .constant_pool
        .add_method_ref(c.super_class, "toString", "()Ljava/lang/String;")
        .unwrap();
    c.constant_pool
        .add(Constant::MethodHandle {
            reference_kind: ReferenceKind::NewInvokeSpecial,
            reference_index: r,
        })
        .unwrap();
    check(c, None, false);
}

#[test]
fn duplicate_real_interfaces() {
    let mut c = base();
    let serial = c.constant_pool.add_class("java/io/Serializable").unwrap();
    c.interfaces = vec![serial, serial];
    check(c, None, false);
}

#[test]
fn null_throw_control() {
    let mut c = base();
    let m = method(&mut c, "f", "()V", 1, 0, vec![Aconst_null, Athrow], None);
    check(c, Some(m), true);
}

#[test]
fn throw_non_throwable() {
    let mut c = base();
    let m = method(
        &mut c,
        "f",
        "(Ljava/lang/Object;)V",
        1,
        1,
        vec![Aload_0, Athrow],
        None,
    );
    check(c, Some(m), false);
}

#[test]
fn monitor_uninitialized() {
    let mut c = base();
    let a = c.constant_pool.add_class("java/lang/Object").unwrap();
    let m = method(
        &mut c,
        "f",
        "()V",
        1,
        0,
        vec![New(a), Monitorenter, Return],
        None,
    );
    check(c, Some(m), true);
}

#[test]
fn putfield_uninitialized_new() {
    let mut c = base();
    let r = c
        .constant_pool
        .add_field_ref(c.super_class, "x", "I")
        .unwrap();
    let m = method(
        &mut c,
        "f",
        "()V",
        2,
        0,
        vec![New(4), Iconst_0, Putfield(r), Return],
        None,
    );
    check(c, Some(m), false);
}

#[test]
fn invokeinterface_wrong_tag_count() {
    let mut c = base();
    let r = c
        .constant_pool
        .add_method_ref(c.super_class, "toString", "()Ljava/lang/String;")
        .unwrap();
    let m = method(
        &mut c,
        "f",
        "()V",
        1,
        0,
        vec![Aconst_null, Invokeinterface(r, 0), Pop, Return],
        None,
    );
    check(c, Some(m), false);
}

#[test]
fn invoke_clinit() {
    let mut c = base();
    let r = c
        .constant_pool
        .add_method_ref(c.super_class, "<clinit>", "()V")
        .unwrap();
    let m = method(
        &mut c,
        "f",
        "()V",
        0,
        0,
        vec![Invokestatic(r), Return],
        None,
    );
    check(c, Some(m), false);
}

#[test]
fn valid_int_condy() {
    let mut class = base();
    let owner_index = class.constant_pool.add_class("java/lang/Integer").unwrap();
    let reference_index = class
        .constant_pool
        .add_method_ref(owner_index, "toString", "(I)Ljava/lang/String;")
        .unwrap();
    let handle_index = class
        .constant_pool
        .add(Constant::MethodHandle {
            reference_kind: ReferenceKind::InvokeStatic,
            reference_index,
        })
        .unwrap();
    let bn = class.constant_pool.add_utf8("BootstrapMethods").unwrap();
    class.attributes.push(Attribute::BootstrapMethods {
        name_index: bn,
        methods: vec![BootstrapMethod {
            bootstrap_method_ref: handle_index,
            arguments: vec![],
        }],
    });
    let nat = class.constant_pool.add_name_and_type("x", "I").unwrap();
    let dynamic_index = class
        .constant_pool
        .add(Constant::Dynamic {
            bootstrap_method_attr_index: 0,
            name_and_type_index: nat,
        })
        .unwrap();
    let test_method = method(
        &mut class,
        "f",
        "()I",
        1,
        0,
        vec![Ldc_w(dynamic_index), Ireturn],
        None,
    );
    check(class, Some(test_method), true);
}

#[test]
fn valid_handler_byte_index_mismatch() {
    let mut c = base();
    let mut m = method(
        &mut c,
        "f",
        "()V",
        1,
        0,
        vec![Bipush(0), Pop, Return, Pop, Return],
        Some(vec![full(3, vec![], vec![T::Object { cpool_index: 4 }])]),
    );
    if let Attribute::Code {
        exception_table, ..
    } = &mut m.attributes[0]
    {
        exception_table.push(ExceptionTableEntry {
            range_pc: 0..2,
            handler_pc: 3,
            catch_type: 0,
        });
    }
    check(c, Some(m), true);
}

#[test]
fn valid_dead_return_legacy() {
    let mut c = base();
    c.version = JAVA_5;
    let m = method(&mut c, "f", "()V", 0, 0, vec![Return, Ireturn], None);
    check(c, Some(m), true);
}

#[test]
fn valid_v50_inference_fallback() {
    let mut c = base();
    c.version = JAVA_6;
    let m = method(&mut c, "f", "()V", 0, 0, vec![Goto(1), Return], None);
    check(c, Some(m), true);
}

#[test]
fn static_final_constructor() {
    let mut c = base();
    let mut m = method(&mut c, "<init>", "()V", 0, 0, vec![Return], None);
    m.access_flags |= MethodAccessFlags::FINAL;
    check(c, Some(m), false);
}

#[test]
fn invalid_clinit() {
    let mut c = base();
    let mut m = method(&mut c, "<clinit>", "(I)V", 0, 2, vec![Return], None);
    m.access_flags = MethodAccessFlags::PUBLIC;
    check(c, Some(m), false);
}

#[test]
fn static_interface_pre_java8() {
    let mut c = base();
    c.version = JAVA_7;
    c.access_flags =
        ClassAccessFlags::PUBLIC | ClassAccessFlags::INTERFACE | ClassAccessFlags::ABSTRACT;
    let m = method(&mut c, "f", "()V", 0, 0, vec![Return], None);
    check(c, Some(m), false);
}

#[test]
fn instance_256_parameter_slots() {
    let mut c = base();
    let desc = format!("({})V", "I".repeat(255));
    let mut m = method(&mut c, "f", &desc, 0, 256, vec![Return], None);
    m.access_flags = MethodAccessFlags::PUBLIC;
    check(c, Some(m), false);
}

#[test]
fn duplicate_fields() {
    let mut c = base();
    let n = c.constant_pool.add_utf8("x").unwrap();
    let d = c.constant_pool.add_utf8("I").unwrap();
    let f = Field {
        access_flags: FieldAccessFlags::PUBLIC,
        name_index: n,
        descriptor_index: d,
        field_type: FieldType::Base(BaseType::Int),
        attributes: vec![],
    };
    c.fields = vec![f.clone(), f];
    check(c, None, false);
}

#[test]
fn constructed_invalid_field_descriptor() {
    let mut c = base();
    let n = c.constant_pool.add_utf8("x").unwrap();
    let d = c.constant_pool.add_utf8("garbage").unwrap();
    c.fields.push(Field {
        access_flags: FieldAccessFlags::PUBLIC,
        name_index: n,
        descriptor_index: d,
        field_type: FieldType::Base(BaseType::Int),
        attributes: vec![],
    });
    check(c, None, false);
}

#[test]
fn module_constant_in_ordinary_class() {
    let mut c = base();
    let idx = c.constant_pool.add_utf8("x").unwrap();
    c.constant_pool.add(Constant::Module(idx)).unwrap();
    check(c, None, false);
}

#[test]
fn valid_ignored_misplaced_constantvalue() {
    let mut c = base();
    let n = c.constant_pool.add_utf8("ConstantValue").unwrap();
    let v = c.constant_pool.add_integer(0).unwrap();
    c.attributes.push(Attribute::ConstantValue {
        name_index: n,
        constant_value_index: v,
    });
    check(c, None, true);
}

#[test]
fn unvalidated_attribute_name_index() {
    let mut c = base();
    let mut m = method(&mut c, "f", "()V", 0, 0, vec![Return], None);
    if let Attribute::Code { name_index, .. } = &mut m.attributes[0] {
        *name_index = 0;
    }
    check(c, Some(m), false);
}

#[test]
fn bad_local_variable_index() {
    let mut class = base();
    let mut test_method = method(&mut class, "f", "()V", 0, 0, vec![Return], None);
    let table_name = class.constant_pool.add_utf8("LocalVariableTable").unwrap();
    let variable_name = class.constant_pool.add_utf8("x").unwrap();
    let descriptor_index = class.constant_pool.add_utf8("J").unwrap();
    if let Attribute::Code { attributes, .. } = &mut test_method.attributes[0] {
        attributes.push(Attribute::LocalVariableTable {
            name_index: table_name,
            variables: vec![LocalVariableTable {
                start_pc: 0,
                length: 1,
                name_index: variable_name,
                descriptor_index,
                index: 65535,
            }],
        });
    }
    check(class, Some(test_method), false);
}

#[test]
fn invalid_type_annotation_path_context() {
    let mut c = base();
    let n = c
        .constant_pool
        .add_utf8("RuntimeVisibleTypeAnnotations")
        .unwrap();
    let t = c.constant_pool.add_utf8("LDeprecated;").unwrap();
    c.attributes.push(Attribute::RuntimeVisibleTypeAnnotations {
        name_index: n,
        type_annotations: vec![TypeAnnotation {
            target_type: TargetType::Empty { target_type: 0x13 },
            type_path: vec![TargetPath {
                type_path_kind: 255,
                type_argument_index: 5,
            }],
            type_index: t,
            elements: vec![],
        }],
    });
    check(c, None, false);
}

#[test]
fn large_logical_branch_sizing() {
    let mut c = base();
    let mut code = vec![Iinc(0, 0); 18000];
    code.push(Goto(18000));
    let m = method(
        &mut c,
        "f",
        "(I)V",
        0,
        1,
        code,
        Some(vec![StackFrame::SameFrameExtended {
            frame_type: 251,
            offset_delta: 18000,
        }]),
    );
    check(c, Some(m), true);
}

#[test]
fn overwritten_long_local() {
    let mut c = base();
    c.version = JAVA_5;
    let m = method(
        &mut c,
        "f",
        "(I)V",
        2,
        3,
        vec![
            Lconst_0,
            Lstore_1,
            Iload_0,
            Ifeq(7),
            Iconst_0,
            Istore_2,
            Goto(9),
            Fconst_0,
            Fstore_2,
            Lload_1,
            Pop2,
            Return,
        ],
        None,
    );
    check(c, Some(m), false);
}

#[test]
fn compact_frame_requires_promotion() {
    let mut c = base();
    let mut code = vec![Iinc(0, 0); 63];
    code.push(Return);
    let m = method(
        &mut c,
        "f",
        "(I)V",
        0,
        1,
        code,
        Some(vec![StackFrame::SameFrame { frame_type: 63 }]),
    );
    check(c, Some(m), true);
}

#[test]
fn duplicate_record_components() {
    let mut c = base();
    let n = c.constant_pool.add_utf8("Record").unwrap();
    let name = c.constant_pool.add_utf8("x").unwrap();
    let desc = c.constant_pool.add_utf8("I").unwrap();
    let rec = Record {
        name_index: name,
        descriptor_index: desc,
        attributes: vec![],
    };
    c.attributes.push(Attribute::Record {
        name_index: n,
        records: vec![rec.clone(), rec],
    });
    check(c, None, true);
}

#[test]
fn handler_uses_post_instruction_locals() {
    let mut c = base();
    c.version = JAVA_5;
    let mut m = method(
        &mut c,
        "f",
        "()V",
        1,
        1,
        vec![Iconst_0, Istore_0, Return, Pop, Iload_0, Pop, Return],
        None,
    );
    if let Attribute::Code {
        exception_table, ..
    } = &mut m.attributes[0]
    {
        exception_table.push(ExceptionTableEntry {
            range_pc: 1..2,
            handler_pc: 3,
            catch_type: 0,
        });
    }
    check(c, Some(m), false);
}

#[test]
fn valid_constructor_calls_own_method() {
    let mut c = base();
    let init = c
        .constant_pool
        .add_method_ref(c.super_class, "<init>", "()V")
        .unwrap();
    let call = c
        .constant_pool
        .add_method_ref(c.this_class, "f", "()V")
        .unwrap();
    let mut f = method(&mut c, "f", "()V", 0, 1, vec![Return], None);
    f.access_flags = MethodAccessFlags::PUBLIC;
    c.methods.push(f);
    let mut m = method(
        &mut c,
        "<init>",
        "()V",
        1,
        1,
        vec![
            Aload_0,
            Invokespecial(init),
            Aload_0,
            Invokevirtual(call),
            Return,
        ],
        None,
    );
    m.access_flags = MethodAccessFlags::PUBLIC;
    check(c, Some(m), true);
}

#[test]
fn overwritten_long_slot_cannot_be_restored_by_stackmap() {
    let mut class = base();
    let method = method(
        &mut class,
        "f",
        "()V",
        2,
        2,
        vec![
            Lconst_0, Lstore_0, Iconst_0, Istore_1, Nop, Lload_0, Pop2, Return,
        ],
        Some(vec![full(4, vec![T::Long], vec![])]),
    );
    check(class, Some(method), false);
}

#[test]
fn constructor_cannot_hide_uninitialized_this_by_overwriting_local_zero() {
    let mut class = base();
    let mut method = method(
        &mut class,
        "<init>",
        "()V",
        1,
        1,
        vec![Aconst_null, Astore_0, Return],
        None,
    );
    method.access_flags = MethodAccessFlags::PUBLIC;
    check(class, Some(method), false);
}

#[test]
fn legacy_subroutine_multiple_call_sites() {
    let mut class = base();
    class.version = JAVA_5;
    let method = method(
        &mut class,
        "f",
        "()V",
        1,
        1,
        vec![Jsr(3), Jsr(3), Return, Astore_0, Ret(0)],
        None,
    );
    check(class, Some(method), true);
}

#[test]
fn legacy_subroutine_recursive_call_is_rejected() {
    let mut class = base();
    class.version = JAVA_5;
    let method = method(
        &mut class,
        "f",
        "()V",
        1,
        1,
        vec![Jsr(2), Return, Astore_0, Jsr(2), Ret(0)],
        None,
    );
    check(class, Some(method), false);
}

#[test]
fn java8_private_interface_method_is_valid() {
    let mut class = base();
    class.version = JAVA_8;
    class.access_flags =
        ClassAccessFlags::PUBLIC | ClassAccessFlags::INTERFACE | ClassAccessFlags::ABSTRACT;
    let mut method = method(&mut class, "f", "()V", 0, 1, vec![Return], None);
    method.access_flags = MethodAccessFlags::PRIVATE;
    check(class, Some(method), true);
}

#[test]
fn chop_frame_removes_a_category_two_local() {
    let mut class = base();
    let method = method(
        &mut class,
        "f",
        "(IJ)V",
        1,
        3,
        vec![Nop, Iload_0, Pop, Return],
        Some(vec![StackFrame::ChopFrame {
            frame_type: 250,
            offset_delta: 1,
        }]),
    );
    check(class, Some(method), true);
}

#[test]
fn module_tables_require_java_base_and_unique_dependencies() {
    let mut class = base();
    class.this_class = class.constant_pool.add_class("module-info").unwrap();
    class.super_class = 0;
    class.access_flags = ClassAccessFlags::MODULE;
    let name_index = class.constant_pool.add_utf8("Module").unwrap();
    let module_name_index = class.constant_pool.add_module("example").unwrap();
    let base_index = class.constant_pool.add_module("java.base").unwrap();
    class.attributes.push(Attribute::Module {
        name_index,
        module_name_index,
        flags: ModuleAccessFlags::empty(),
        version_index: 0,
        requires: vec![],
        exports: vec![],
        opens: vec![],
        uses: vec![],
        provides: vec![],
    });
    assert!(class.verify().is_err());
    let base = Requires {
        index: base_index,
        flags: RequiresFlags::MANDATED,
        version_index: 0,
    };
    if let Attribute::Module { requires, .. } = &mut class.attributes[0] {
        requires.push(base.clone());
    }
    assert!(class.verify().is_ok());
    if let Attribute::Module { requires, .. } = &mut class.attributes[0] {
        requires.push(base);
    }
    assert!(class.verify().is_err());
}

#[test]
fn member_access_context_can_reject_protected_receiver() {
    struct Deny;
    impl VerificationContext for Deny {
        fn is_subclass(&self, _: &str, _: &str) -> VResult<bool> {
            Ok(true)
        }
        fn is_assignable(&self, _: &str, _: &str) -> VResult<bool> {
            Ok(true)
        }
        fn common_superclass(&self, _: &str, _: &str) -> VResult<String> {
            Ok("java/lang/Object".into())
        }
        #[expect(
            clippy::panic_in_result_fn,
            reason = "assert the symbolic member supplied to this test context"
        )]
        fn verify_member_access(
            &self,
            access: &verifiers::context::MemberAccess<'_>,
        ) -> VResult<()> {
            assert_eq!(access.name, "x");
            Err(verifiers::VerifyError::VerifyError(
                "inaccessible member".into(),
            ))
        }
    }
    let mut class = base();
    let index = class
        .constant_pool
        .add_field_ref(class.this_class, "x", "I")
        .unwrap();
    let method = method(
        &mut class,
        "f",
        "(LAudit;)I",
        1,
        1,
        vec![Aload_0, Getfield(index), Ireturn],
        None,
    );
    let result =
        verifiers::bytecode::verify_method(&class, &method, &Deny, &VerifierConfig::strict());
    assert!(result.is_err());
}

#[test]
fn class_initializer_ignores_ordinary_method_flags() {
    let mut class = base();
    let mut method = method(&mut class, "<clinit>", "()V", 0, 0, vec![Return], None);
    method.access_flags = MethodAccessFlags::all();
    check(class.clone(), Some(method.clone()), true);
    method.attributes.clear();
    check(class, Some(method), false);
}

#[test]
fn legacy_class_initializer_has_no_receiver_and_can_have_parameters() {
    for descriptor in ["()V", "(I)V"] {
        let mut class = base();
        class.version = JAVA_5;
        let (locals, code) = if descriptor == "()V" {
            (0, vec![Return])
        } else {
            (1, vec![Iload_0, Pop, Return])
        };
        let mut method = method(&mut class, "<clinit>", descriptor, 1, locals, code, None);
        method.access_flags = MethodAccessFlags::empty();
        check(class.clone(), Some(method.clone()), true);
        class.version = JAVA_7;
        check(class, Some(method), false);
    }
}

#[test]
fn jsr_rejects_uninitialized_references_in_locals_and_stack() {
    for keep_on_stack in [false, true] {
        let mut class = base();
        class.version = JAVA_5;
        let code = if keep_on_stack {
            vec![
                New(class.super_class),
                Jsr(4),
                Pop,
                Return,
                Astore_1,
                Ret(1),
            ]
        } else {
            vec![
                New(class.super_class),
                Astore_0,
                Jsr(4),
                Return,
                Astore_1,
                Ret(1),
            ]
        };
        let method = method(&mut class, "f", "()V", 2, 2, code, None);
        check(class, Some(method), false);
    }
}

#[test]
fn invokespecial_receiver_must_be_assignable_to_current_class() {
    for (descriptor, valid) in [("(Ljava/lang/Object;)V", false), ("(LAudit;)V", true)] {
        let mut class = base();
        let index = class
            .constant_pool
            .add_method_ref(class.super_class, "toString", "()Ljava/lang/String;")
            .unwrap();
        let method = method(
            &mut class,
            "f",
            descriptor,
            1,
            1,
            vec![Aload_0, Invokespecial(index), Pop, Return],
            None,
        );
        check(class, Some(method), valid);
    }
}

#[test]
fn invokespecial_owner_must_be_above_current_class() {
    let mut class = base();
    let other = class.constant_pool.add_class("Other").unwrap();
    let index = class
        .constant_pool
        .add_method_ref(other, "f", "()V")
        .unwrap();
    // null is assignable to both Audit and Other, but Other is not a valid owner.
    let method = method(
        &mut class,
        "f",
        "()V",
        1,
        0,
        vec![Aconst_null, Invokespecial(index), Return],
        None,
    );
    check(class, Some(method), false);
}

#[test]
fn class_literals_require_version_49() {
    let mut class = base();
    let index = class.this_class;
    let method = method(
        &mut class,
        "f",
        "()V",
        1,
        0,
        vec![Ldc_w(index), Pop, Return],
        None,
    );
    class.version = JAVA_1_4;
    check(class.clone(), Some(method.clone()), false);
    class.version = JAVA_5;
    check(class, Some(method), true);
}

#[test]
fn instance_invocation_parameter_limit_includes_receiver() {
    let mut class = base();
    let descriptor = format!("({})V", "I".repeat(255));
    let index = class
        .constant_pool
        .add_method_ref(class.this_class, "target", descriptor.as_str())
        .unwrap();
    // Static constraints apply even to unreachable instructions.
    let method = method(
        &mut class,
        "f",
        "()V",
        0,
        0,
        vec![Return, Invokevirtual(index)],
        None,
    );
    check(class, Some(method), false);
}
