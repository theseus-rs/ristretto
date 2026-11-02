#![forbid(unsafe_code)]

use ristretto_classfile::attributes::{Attribute, Instruction, LineNumber, MaxLocals, MaxStack};
use ristretto_classfile::{
    CONSTRUCTOR_NAME, ClassAccessFlags, ClassFile, ConstantPool, JAVA_21, MethodAccessFlags, Result,
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
    let bytes = generate_class()?;
    fs::write("HelloWorld.class", bytes)?;
    Ok(())
}

fn generate_class() -> Result<Vec<u8>> {
    let mut constant_pool = ConstantPool::default();
    let super_class = constant_pool.add_class("java/lang/Object")?;
    let object_init = constant_pool.add_method_ref(super_class, CONSTRUCTOR_NAME, "()V")?;
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
        attributes: vec![Attribute::LineNumberTable {
            name_index: line_number_table_index,
            line_numbers: vec![
                LineNumber {
                    start_pc: 0, // Byte offset of Getstatic
                    line_number: 3,
                },
                LineNumber {
                    start_pc: 8, // Byte offset of Return
                    line_number: 4,
                },
            ],
        }],
    });
    methods.push(main_method);

    let class_file = ClassFile {
        version: JAVA_21,
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
    Ok(bytes)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_generate_class() -> Result<()> {
        let bytes = generate_class()?;
        let class_file = ClassFile::from_bytes(&bytes)?;

        class_file.verify()?;
        assert_eq!(JAVA_21, class_file.version);
        assert_eq!("HelloWorld", class_file.class_name()?);
        assert!(class_file.to_string().contains("Hello, World!"));
        let method_names = class_file
            .methods
            .iter()
            .map(|method| class_file.constant_pool.try_get_utf8(method.name_index))
            .collect::<Result<Vec<_>>>()?;
        assert_eq!(vec![CONSTRUCTOR_NAME, "main"], method_names);
        let main = class_file.methods.last().expect("main method");
        let Attribute::Code {
            code, attributes, ..
        } = &main.attributes[0]
        else {
            panic!("main must have code");
        };
        let Attribute::LineNumberTable { line_numbers, .. } = &attributes[0] else {
            panic!("main must have line numbers");
        };
        let source_lines = Instruction::with_offsets(code)
            .map(|(offset, _)| {
                let line = line_numbers
                    .iter()
                    .rev()
                    .find(|line| usize::from(line.start_pc) <= offset)
                    .expect("source line for every instruction");
                (offset, line.line_number)
            })
            .collect::<Vec<_>>();
        assert_eq!(vec![(0, 3), (3, 3), (5, 3), (8, 4)], source_lines);
        Ok(())
    }
}
