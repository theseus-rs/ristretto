#![forbid(unsafe_code)]
#![deny(clippy::pedantic)]

use ristretto_classfile::attributes::{Attribute, Instruction, LineNumber, MaxLocals, MaxStack};
use ristretto_classfile::{
    ClassAccessFlags, ClassFile, ConstantPool, MethodAccessFlags, Result, Version,
};
use std::fs;

/// Creates a class file equivalent to the following Java code:
///
/// ```java
/// public class HelloWorld {
///     public static void main(String[] args) {
///         System.out.println("Hello, World!");
///     }
/// }
/// ```
fn main() -> Result<()> {
    let mut constant_pool = ConstantPool::default();
    let super_class = constant_pool.add_class("java/lang/Object")?;
    let object_init = constant_pool.add_method_ref(super_class, "<init>", "()V")?;
    let system_class = constant_pool.add_class("java/lang/System")?;
    let println_field =
        constant_pool.add_field_ref(system_class, "out", "Ljava/io/PrintStream;")?;
    let hello_world_string = constant_pool.add_string("Hello, World!")?;
    let print_stream_class = constant_pool.add_class("java/io/PrintStream")?;
    let println_method =
        constant_pool.add_method_ref(print_stream_class, "println", "(Ljava/lang/String;)V")?;
    let this_class = constant_pool.add_class("HelloWorld")?;
    let code_index = constant_pool.add_utf8("Code")?;
    let line_number_table_index = constant_pool.add_utf8("LineNumberTable")?;
    let main_name_index = constant_pool.add_utf8("main")?;
    let main_descriptor_index = constant_pool.add_utf8("([Ljava/lang/String;)V")?;

    let mut methods = Vec::new();
    let (_class_index, name_and_type_index) = constant_pool.try_get_method_ref(object_init)?;
    let (name_index, descriptor_index) =
        constant_pool.try_get_name_and_type(*name_and_type_index)?;
    let mut init_method = ristretto_classfile::Method {
        access_flags: MethodAccessFlags::PUBLIC,
        name_index: *name_index,
        descriptor_index: *descriptor_index,
        attributes: Vec::new(),
    };
    let init_method_code = vec![
        Instruction::Aload_0,
        Instruction::Invokespecial(object_init),
        Instruction::Return,
    ];
    let init_max_stack = init_method_code.max_stack(&constant_pool)?;
    let init_max_locals = init_method_code.max_locals(&constant_pool, &init_method)?;
    init_method.attributes.push(Attribute::Code {
        name_index: code_index,
        max_stack: init_max_stack,
        max_locals: init_max_locals,
        code: init_method_code,
        exception_table: Vec::new(),
        attributes: Vec::new(),
    });
    methods.push(init_method);

    let mut main_method = ristretto_classfile::Method {
        access_flags: MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
        name_index: main_name_index,
        descriptor_index: main_descriptor_index,
        attributes: Vec::new(),
    };
    let main_method_code = vec![
        Instruction::Getstatic(println_field),
        Instruction::Ldc(u8::try_from(hello_world_string)?),
        Instruction::Invokevirtual(println_method),
        Instruction::Return,
    ];
    let main_max_stack = main_method_code.max_stack(&constant_pool)?;
    let main_max_locals = main_method_code.max_locals(&constant_pool, &main_method)?;
    main_method.attributes.push(Attribute::Code {
        name_index: code_index,
        max_stack: main_max_stack,
        max_locals: main_max_locals,
        code: main_method_code,
        exception_table: Vec::new(),
        attributes: Vec::new(),
    });
    main_method.attributes.push(Attribute::LineNumberTable {
        name_index: line_number_table_index,
        line_numbers: vec![
            LineNumber {
                start_pc: 0,
                line_number: 3,
            },
            LineNumber {
                start_pc: 8,
                line_number: 4,
            },
        ],
    });
    methods.push(main_method);

    let class_file = ClassFile {
        version: Version::Java21 { minor: 0 },
        access_flags: ClassAccessFlags::PUBLIC | ClassAccessFlags::SUPER,
        constant_pool,
        this_class,
        super_class,
        methods,
        ..Default::default()
    };
    class_file.verify()?;

    let mut bytes = Vec::new();
    class_file.to_bytes(&mut bytes)?;
    fs::write("HelloWorld.class", bytes)?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_main() -> Result<()> {
        main()
    }
}
