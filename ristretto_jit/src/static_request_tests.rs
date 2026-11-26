//! Static initialization is driven by the embedding while native activations park.
use super::*;
use crate::{FieldAccess, FieldRequest};
use ristretto_classfile::attributes::{MaxLocals, MaxStack};
use ristretto_classfile::{ClassAccessFlags, Constant, ConstantPool, Field, FieldAccessFlags};

fn fixture(
    field_type: &str,
    descriptor: &str,
    name: &str,
    code: impl FnOnce(u16) -> Vec<Instruction>,
) -> Result<(ClassFile<'static>, u16)> {
    let mut pool = ConstantPool::new();
    let this_class = pool.add_class("StaticRequestFixture")?;
    let super_class = pool.add_class("java/lang/Object")?;
    let index = pool.add_field_ref(this_class, "slot", field_type)?;
    let field = Field {
        access_flags: FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC,
        name_index: pool.add_utf8("slot")?,
        descriptor_index: pool.add_utf8(field_type)?,
        field_type: FieldType::parse(field_type)?,
        attributes: Vec::new(),
    };
    let mut method = Method {
        access_flags: MethodAccessFlags::STATIC
            | if name == "<clinit>" {
                MethodAccessFlags::empty()
            } else {
                MethodAccessFlags::PUBLIC
            },
        name_index: pool.add_utf8(name)?,
        descriptor_index: pool.add_utf8(descriptor)?,
        attributes: Vec::new(),
    };
    let code = code(index);
    method.attributes.push(Attribute::Code {
        name_index: pool.add_utf8("Code")?,
        max_stack: code.max_stack(&pool)?,
        max_locals: code.max_locals(&pool, &method)?,
        code,
        exception_table: Vec::new(),
        attributes: Vec::new(),
    });
    let class = ClassFile {
        version: ristretto_classfile::JAVA_5,
        constant_pool: pool,
        this_class,
        super_class,
        access_flags: ClassAccessFlags::PUBLIC | ClassAccessFlags::SUPER,
        fields: vec![field],
        methods: vec![method],
        ..Default::default()
    };
    class.verify()?;
    Ok((class, index))
}

fn method<'a>(class: &'a ClassFile<'_>) -> Result<&'a Method> {
    class
        .methods
        .first()
        .ok_or_else(|| InternalError("static fixture method".to_owned()))
}

fn replace_index(class: &mut ClassFile<'_>, index: u16) {
    for method in &mut class.methods {
        for attribute in &mut method.attributes {
            if let Attribute::Code { code, .. } = attribute {
                for instruction in code {
                    match instruction {
                        Instruction::Getstatic(current) | Instruction::Putstatic(current) => {
                            *current = index;
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

fn request(kind: FieldAccess, index: u16, bci: usize) -> ExecutionStep {
    ExecutionStep::Field(FieldRequest { kind, index, bci })
}

fn roots(context: &Context) -> Result<Vec<i64>> {
    Ok(context
        .roots
        .lock()
        .map_err(|error| InternalError(error.to_string()))?
        .clone())
}

fn reference_store(field_type: &str) -> Result<(ClassFile<'static>, u16)> {
    fixture(
        field_type,
        &format!("({field_type}){field_type}"),
        "run",
        |index| {
            vec![
                Instruction::Aload_0,
                Instruction::Aconst_null,
                Instruction::Astore_0,
                Instruction::Putstatic(index),
                Instruction::Getstatic(index),
                Instruction::Areturn,
            ]
        },
    )
}

#[test]
fn static_integer_requests_have_exact_metadata_arity_and_signed_cells() -> Result<()> {
    let (class, index) = fixture("I", "(I)I", "run", |index| {
        vec![
            Instruction::Iload_0,
            Instruction::Putstatic(index),
            Instruction::Getstatic(index),
            Instruction::Ireturn,
        ]
    })?;
    assert!(!Compiler::can_compile(method(&class)?));
    assert_eq!(
        Compiler::experimental_rejection_reason(&class, method(&class)?),
        None
    );
    assert_eq!(
        Compiler::request_rejection_reason(&class, method(&class)?),
        None
    );
    let function = compile(&class, "run")?;
    for value in [i32::MIN, -1, 0, i32::MAX] {
        let context = Context::new();
        let mut activation = function.activation(&[Value::I32(value)])?;
        assert_eq!(
            activation.resume_with_context(NonZeroU32::MIN, context.pointer())?,
            request(FieldAccess::PutStatic, index, 1)
        );
        assert_eq!(activation.call_arguments()?, &[i64::from(value)]);
        assert_eq!(activation.suspended_bci(), Some(1));
        assert_eq!(activation.remaining_budget()?, None);
        assert!(
            activation
                .resume_with_context(NonZeroU32::MIN, context.pointer())
                .is_err()
        );
        activation.complete_call(0)?;
        assert!(activation.complete_call(0).is_err());
        assert_eq!(
            activation.resume_with_context(NonZeroU32::MIN, context.pointer())?,
            request(FieldAccess::GetStatic, index, 4)
        );
        assert_eq!(activation.call_arguments()?, []);
        activation.complete_call(i64::from(value))?;
        assert_eq!(
            activation.resume_with_context(NonZeroU32::MIN, context.pointer())?,
            ExecutionStep::Returned(Some(Value::I32(value)))
        );
    }
    Ok(())
}

#[test]
fn static_reference_requests_keep_outgoing_tokens_after_clearing_locals() -> Result<()> {
    for descriptor in [
        "Ljava/lang/Object;",
        "LClosedNode;",
        "[I",
        "[Ljava/lang/Object;",
    ] {
        let (class, index) = reference_store(descriptor)?;
        let function = compile(&class, "run")?;
        for token in [0, 91, i64::MAX] {
            let context = Context::new();
            let mut activation = function.activation(&[Value::Ptr(token)])?;
            assert_eq!(
                activation.resume_with_context(NonZeroU32::MIN, context.pointer())?,
                request(FieldAccess::PutStatic, index, 3)
            );
            assert_eq!(activation.call_arguments()?, &[token]);
            if token != 0 {
                assert!(roots(&context)?.contains(&token));
            }
            let old_pointer = context.pointer();
            let context = Box::new(context);
            assert_ne!(old_pointer, context.pointer());
            activation.complete_call(0)?;
            assert_eq!(
                activation.resume_with_context(NonZeroU32::MIN, context.pointer())?,
                request(FieldAccess::GetStatic, index, 6)
            );
            assert_eq!(activation.call_arguments()?, []);
            activation.complete_call(token)?;
            assert_eq!(
                activation.resume_with_context(NonZeroU32::MIN, context.pointer())?,
                ExecutionStep::Returned(Some(Value::Ptr(token)))
            );
            if token != 0 {
                assert!(roots(&context)?.contains(&token));
            }
        }
    }
    Ok(())
}

#[test]
fn a_compiled_initializer_runs_while_the_requesting_activation_stays_parked() -> Result<()> {
    let (outer, outer_index) = reference_store("Ljava/lang/Object;")?;
    let (initializer, index) = fixture("I", "()V", "<clinit>", |index| {
        vec![
            Instruction::Iconst_5,
            Instruction::Putstatic(index),
            Instruction::Return,
        ]
    })?;
    let function = compile(&outer, "run")?;
    let mut waiting = function.activation(&[Value::Ptr(91)])?;
    drop(function);
    let outer_context = Context::new();
    assert_eq!(
        waiting.resume_with_context(NonZeroU32::MIN, outer_context.pointer())?,
        request(FieldAccess::PutStatic, outer_index, 3)
    );
    let original_roots = roots(&outer_context)?;
    let initializer = compile(&initializer, "<clinit>")?;
    let mut initializing = initializer.activation(&[])?;
    drop(initializer);
    let inner_context = Context::new();
    assert_eq!(
        initializing.resume_with_context(NonZeroU32::MIN, inner_context.pointer())?,
        request(FieldAccess::PutStatic, index, 1)
    );
    assert_eq!(initializing.call_arguments()?, &[5]);
    assert!(
        waiting
            .resume_with_context(NonZeroU32::MIN, outer_context.pointer())
            .is_err()
    );
    assert_eq!(waiting.call_arguments()?, &[91]);
    assert_eq!(roots(&outer_context)?, original_roots);
    initializing.complete_call(0)?;
    assert_eq!(
        initializing.resume_with_context(NonZeroU32::MIN, inner_context.pointer())?,
        ExecutionStep::Returned(None)
    );
    waiting.complete_call(0)?;
    assert_eq!(
        waiting.resume_with_context(NonZeroU32::MIN, outer_context.pointer())?,
        request(FieldAccess::GetStatic, outer_index, 6)
    );
    waiting.complete_call(91)?;
    assert_eq!(
        waiting.resume_with_context(NonZeroU32::MIN, outer_context.pointer())?,
        ExecutionStep::Returned(Some(Value::Ptr(91)))
    );
    Ok(())
}

#[test]
fn static_request_errors_and_cancellation_do_not_run_following_bytecodes() -> Result<()> {
    let (class, index) = reference_store("Ljava/lang/Object;")?;
    let function = compile(&class, "run")?;
    for cancel in [true, false] {
        let context = Context::new();
        let mut activation = function.activation(&[Value::Ptr(91)])?;
        assert_eq!(
            activation.resume_with_context(NonZeroU32::MIN, context.pointer())?,
            request(FieldAccess::PutStatic, index, 3)
        );
        assert!(roots(&context)?.contains(&91));
        if cancel {
            // The embedding still owns its published roots after cancellation.
            drop(activation);
            assert!(roots(&context)?.contains(&91));
        } else {
            context.failed.store(true, Ordering::Relaxed);
            activation.complete_call(0)?;
            assert_eq!(
                activation.resume_with_context(NonZeroU32::MIN, context.pointer())?,
                ExecutionStep::Returned(None)
            );
            assert!(activation.call_arguments().is_err());
        }
    }
    Ok(())
}

#[test]
fn static_reference_results_must_be_published_before_return() -> Result<()> {
    let (class, index) = fixture(
        "Ljava/lang/Object;",
        "()Ljava/lang/Object;",
        "run",
        |index| vec![Instruction::Getstatic(index), Instruction::Areturn],
    )?;
    let function = compile(&class, "run")?;
    let context = Context::new();
    let mut activation = function.activation(&[])?;
    assert_eq!(
        activation.resume_with_context(NonZeroU32::MIN, context.pointer())?,
        request(FieldAccess::GetStatic, index, 0)
    );
    let original_roots = roots(&context)?;
    context.rejected_token.store(91, Ordering::Relaxed);
    activation.complete_call(91)?;
    assert_eq!(
        activation.resume_with_context(NonZeroU32::MIN, context.pointer())?,
        ExecutionStep::Returned(None)
    );
    assert!(context.failed.load(Ordering::Relaxed));
    assert_eq!(roots(&context)?, original_roots);
    Ok(())
}

#[test]
fn malformed_static_metadata_rejects_before_native_symbol_linking() -> Result<()> {
    let (valid, _) = fixture("I", "()I", "run", |index| {
        vec![Instruction::Getstatic(index), Instruction::Ireturn]
    })?;
    for case in 0..7 {
        let mut class = valid.clone();
        let owner = class.this_class;
        let pool = &mut class.constant_pool;
        let index = match case {
            0 => pool.add_integer(7)?,
            1 => {
                let owner = pool.add_utf8("not-a-Class-constant")?;
                pool.add_field_ref(owner, "slot", "I")?
            }
            2 => {
                let member = pool.add_integer(1)?;
                pool.add(Constant::FieldRef {
                    class_index: owner,
                    name_and_type_index: member,
                })?
            }
            3 => pool.add_field_ref(owner, "slot", "Iextra")?,
            4 => pool.add_field_ref(owner, "slot", "V")?,
            5 => pool.add_field_ref(owner, "", "I")?,
            _ => {
                let name_index = pool.add_class("NotUtf8")?;
                let descriptor_index = pool.add_utf8("I")?;
                let member = pool.add(Constant::NameAndType {
                    name_index,
                    descriptor_index,
                })?;
                pool.add(Constant::FieldRef {
                    class_index: owner,
                    name_and_type_index: member,
                })?
            }
        };
        replace_index(&mut class, index);
        assert_eq!(
            Compiler::request_rejection_reason(&class, method(&class)?),
            Some("experimental_static_field_metadata")
        );
        assert!(
            matches!(Compiler::new()?.compile_requests(&class, method(&class)?, &[]),
            Err(crate::Error::UnsupportedMethod(reason)) if reason == "experimental_static_field_metadata")
        );
    }
    Ok(())
}

#[test]
fn well_formed_generic_static_representations_remain_admitted() -> Result<()> {
    // This compiler ABI is broader than canonical static storage. The VM must
    // separately reject its unsupported scalar/layout/final/volatile cases.
    for (descriptor, ret) in [
        ("B", Instruction::Ireturn),
        ("C", Instruction::Ireturn),
        ("S", Instruction::Ireturn),
        ("Z", Instruction::Ireturn),
        ("I", Instruction::Ireturn),
        ("J", Instruction::Lreturn),
        ("F", Instruction::Freturn),
        ("D", Instruction::Dreturn),
        ("Ljava/lang/Object;", Instruction::Areturn),
        ("[I", Instruction::Areturn),
        ("[[I", Instruction::Areturn),
    ] {
        let (class, _) = fixture(descriptor, &format!("(){descriptor}"), "run", |index| {
            vec![Instruction::Getstatic(index), ret]
        })?;
        assert_eq!(
            Compiler::request_rejection_reason(&class, method(&class)?),
            None
        );
        let _ = compile(&class, "run")?;
    }
    Ok(())
}

fn frozen_classes() -> Result<Vec<ClassFile<'static>>> {
    let bytes: [&[u8]; 12] = [
        include_bytes!("../tests/fixtures/canonical_static/CanonicalStaticRootsSmall.class"),
        include_bytes!("../tests/fixtures/canonical_static/CanonicalStaticRootsSmall$Node.class"),
        include_bytes!("../tests/fixtures/canonical_static/CanonicalStaticRootsFull.class"),
        include_bytes!("../tests/fixtures/canonical_static/CanonicalStaticRootsFull$Node.class"),
        include_bytes!("../tests/fixtures/canonical_static/CanonicalStaticProtocol.class"),
        include_bytes!("../tests/fixtures/canonical_static/CanonicalStaticProtocol$Journal.class"),
        include_bytes!("../tests/fixtures/canonical_static/CanonicalStaticProtocol$A.class"),
        include_bytes!("../tests/fixtures/canonical_static/CanonicalStaticProtocol$B.class"),
        include_bytes!("../tests/fixtures/canonical_static/CanonicalStaticProtocol$Lazy.class"),
        include_bytes!(
            "../tests/fixtures/canonical_static/CanonicalStaticProtocol$Fieldless.class"
        ),
        include_bytes!("../tests/fixtures/canonical_static/CanonicalStaticProtocol$Bad.class"),
        include_bytes!(
            "../tests/fixtures/canonical_static/CanonicalStaticProtocol$NestedBad.class"
        ),
    ];
    Ok(bytes
        .into_iter()
        .map(ClassFile::from_bytes)
        .collect::<std::result::Result<_, _>>()?)
}

#[test]
fn every_frozen_vm_static_method_compiles_without_legacy_helper_symbols() -> Result<()> {
    let mut methods = 0;
    let mut initializers = 0;
    for class in frozen_classes()? {
        class.verify()?;
        for method in &class.methods {
            let name = class.constant_pool.try_get_utf8(method.name_index)?;
            assert_eq!(
                Compiler::request_rejection_reason(&class, method),
                None,
                "{}::{name}",
                class.class_name()?
            );
            let function = compile(&class, &name.to_str_lossy())?;
            assert!(function.requests_only());
            methods += 1;
            initializers += usize::from(name == "<clinit>");
        }
    }
    assert!(methods > 30);
    assert_eq!(initializers, 8);
    println!("Compiled {methods} frozen Java methods, including {initializers} initializers");
    Ok(())
}

#[test]
fn frozen_static_root_initializers_emit_array_and_owning_static_requests() -> Result<()> {
    for (class, slots) in [
        (
            ClassFile::from_bytes(include_bytes!(
                "../tests/fixtures/canonical_static/CanonicalStaticRootsSmall.class"
            ))?,
            9,
        ),
        (
            ClassFile::from_bytes(include_bytes!(
                "../tests/fixtures/canonical_static/CanonicalStaticRootsFull.class"
            ))?,
            2048,
        ),
    ] {
        let function = compile(&class, "<clinit>")?;
        script(
            &function,
            &[],
            &[
                ("allocation", &[slots], 91),
                ("field", &[91], 0),
                ("allocation", &[slots], 92),
                ("field", &[92], 0),
                ("field", &[1], 0),
            ],
            None,
        )?;
    }
    Ok(())
}
