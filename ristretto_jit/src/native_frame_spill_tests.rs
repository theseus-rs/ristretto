//! Native sizes reflect optimization and values preserved across opaque calls.
use super::*;
use ristretto_classfile::attributes::{Attribute, Instruction};
use ristretto_classfile::{
    ClassAccessFlags, ClassFile, ConstantPool, Field, FieldAccessFlags, Method, MethodAccessFlags,
};

use std::sync::atomic::{AtomicI64, Ordering};

fn spill_fixture(count: u16, opaque: bool) -> Result<ClassFile<'static>> {
    let mut pool = ConstantPool::new();
    let this_class = pool.add_class("NativeFrameSpills")?;
    let super_class = pool.add_class("java/lang/Object")?;
    let name_index = pool.add_utf8("sum")?;
    let descriptor_index = pool.add_utf8(if opaque { "([J)J" } else { "(J)J" })?;
    let code_name = pool.add_utf8("Code")?;
    let field = Field {
        access_flags: FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC,
        name_index: pool.add_utf8("tick")?,
        descriptor_index: pool.add_utf8("J")?,
        field_type: ristretto_classfile::FieldType::parse("J")?,
        attributes: vec![],
    };
    let mut code = Vec::new();
    for index in 0..count {
        if opaque {
            code.extend([
                Instruction::Aload_0,
                Instruction::Iconst_0,
                Instruction::Laload,
            ]);
        } else {
            let constant = pool.add_long(i64::from(index).wrapping_mul(0x1_0000_0001))?;
            code.extend([
                Instruction::Lload_0,
                Instruction::Ldc2_w(constant),
                Instruction::Lxor,
            ]);
        }
        code.push(Instruction::Lstore_w(2 * (index + 1)));
    }
    code.push(Instruction::Lconst_0);
    for index in (0..count).rev() {
        code.extend([Instruction::Lload_w(2 * (index + 1)), Instruction::Ladd]);
    }
    code.push(Instruction::Lreturn);
    Ok(ClassFile {
        version: ristretto_classfile::JAVA_5,
        constant_pool: pool,
        access_flags: ClassAccessFlags::PUBLIC | ClassAccessFlags::SUPER,
        this_class,
        super_class,
        fields: vec![field],
        methods: vec![Method {
            access_flags: MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
            name_index,
            descriptor_index,
            attributes: vec![Attribute::Code {
                name_index: code_name,
                max_stack: 4,
                max_locals: 2 * (count + 1),
                code,
                exception_table: vec![],
                attributes: vec![],
            }],
        }],
        ..Default::default()
    })
}

#[test]
fn primitive_local_count_does_not_define_native_frame_size() -> Result<()> {
    let class = spill_fixture(1024, false)?;
    class.verify()?;
    let method = class.methods.first().expect("spill method");
    assert!(crate::Compiler::can_compile(method));
    let function = crate::Compiler::new()?.compile(&class, method, &[])?;
    assert!(function.native_stack_bytes() >= 16, "{function:?}");
    for argument in [0_i64, 0x1234_5678_9876_5432, -1] {
        let expected = (0..1024).fold(0_i64, |sum, index| {
            sum.wrapping_add(argument ^ (i64::from(index) * 0x1_0000_0001))
        });
        assert_eq!(
            function.execute(&[crate::Value::I64(argument)], std::ptr::null())?,
            Some(crate::Value::I64(expected))
        );
    }
    #[cfg(feature = "execution-profile")]
    assert_eq!(function.execution_count(), 3);
    println!(
        "native primitive frame with 2050 declared locals: {} bytes",
        function.native_stack_bytes()
    );
    Ok(())
}

extern "C" fn read_counter(context: *const AtomicI64, _bci: i32, _array: i64, _index: i32) -> i64 {
    // Safety: this fixture passes an AtomicI64 owner for the whole synchronous call.
    unsafe { &*context }.fetch_add(1, Ordering::Relaxed)
}

extern "C" fn publish(_context: *const u8, _roots: *const i64, _count: i32) -> i32 {
    1
}

extern "C" fn pending(_context: *const u8) -> i64 {
    0
}

extern "C" fn null_receiver(context: *const AtomicI64, _bci: i32) {
    // SAFETY: the fixture owns this counter for the synchronous invocation.
    unsafe { &*context }.store(i64::MIN, Ordering::Relaxed);
}

#[test]
fn metadata_includes_values_spilled_across_opaque_calls() -> Result<()> {
    // Each array read now owns continuation state. Keep the live-value fixture
    // below the state-transfer limit, which the separate rejection test covers.
    const COUNT: u16 = 248;
    let class = spill_fixture(COUNT, true)?;
    class.verify()?;
    let method = class.methods.first().expect("spill method");
    assert!(!crate::Compiler::can_compile(method));
    let symbols = [
        ("jit_laload", read_counter as *const u8),
        ("jit_publish_frame_roots", publish as *const u8),
        ("jit_pending_exception", pending as *const u8),
        ("jit_throw_npe", null_receiver as *const u8),
    ];
    let function = crate::Compiler::new()?.compile(&class, method, &symbols)?;
    // Probe/root slots are small. Live opaque return values cannot be
    // rematerialized; their spills must appear in backend metadata.
    assert!(function.native_stack_bytes() > 4096, "{function:?}");
    let counter = AtomicI64::new(0);
    let context = std::ptr::from_ref(&counter).cast::<u8>();
    assert_eq!(
        function.execute(&[crate::Value::Ptr(42)], context)?,
        Some(crate::Value::I64(
            i64::from(COUNT) * i64::from(COUNT - 1) / 2
        ))
    );
    assert_eq!(counter.load(Ordering::Relaxed), i64::from(COUNT));
    #[cfg(feature = "execution-profile")]
    assert_eq!(function.execution_count(), 1);
    println!(
        "native frame with opaque call spills: {} bytes",
        function.native_stack_bytes()
    );
    Ok(())
}

#[test]
fn excessive_request_state_is_rejected_before_codegen() -> Result<()> {
    let mut class = spill_fixture(1024, true)?;
    let field = class
        .constant_pool
        .add_field_ref(class.this_class, "tick", "J")?;
    let method = class.methods.first_mut().expect("spill method");
    for attribute in &mut method.attributes {
        if let Attribute::Code { code, .. } = attribute {
            let mut replaced = Vec::new();
            let mut index = 0;
            while let Some(instruction) = code.get(index) {
                if instruction == &Instruction::Aload_0 {
                    replaced.push(Instruction::Getstatic(field));
                    index += 3;
                } else {
                    replaced.push(instruction.clone());
                    index += 1;
                }
            }
            *code = replaced;
        }
    }
    class.verify()?;
    let result =
        crate::Compiler::new()?.compile(&class, class.methods.first().expect("spill method"), &[]);
    assert!(
        matches!(result, Err(Error::UnsupportedMethod(ref message)) if message == "continuation state-transfer limit exceeded"),
        "{result:?}"
    );
    Ok(())
}
