#![forbid(unsafe_code)]
#![deny(clippy::pedantic)]

use ristretto_classfile::attributes::{Attribute, Instruction, LineNumber};
use ristretto_classfile::{
    ClassAccessFlags, ClassFile, Constant, ConstantPool, MethodAccessFlags, Result, Version,
};
use std::fs;
use std::io::Cursor;

/// Creates a simple "Hello, World!" class file equivalent to the following Java code:
///
/// ```java
/// public class HelloWorld {
//     public static void main(String[] args) {
//         System.out.println("Hello, World!");
//     }
// }
/// ```
#[allow(clippy::too_many_lines)]
fn main() -> Result<()> {
    let mut constant_pool = ConstantPool::default();
    constant_pool.add(Constant::MethodRef {
        class_index: 2,
        name_and_type_index: 3,
    });
    constant_pool.add(Constant::Class { name_index: 4 });
    let super_class = u16::try_from(constant_pool.len())?;
    constant_pool.add(Constant::NameAndType {
        name_index: 5,
        descriptor_index: 6,
    });
    constant_pool.add(Constant::Utf8("java/lang/Object".to_string()));
    constant_pool.add(Constant::Utf8("<init>".to_string()));
    constant_pool.add(Constant::Utf8("()V".to_string()));
    constant_pool.add(Constant::FieldRef {
        class_index: 8,
        name_and_type_index: 9,
    });
    constant_pool.add(Constant::Class { name_index: 10 });
    constant_pool.add(Constant::NameAndType {
        name_index: 11,
        descriptor_index: 12,
    });
    constant_pool.add(Constant::Utf8("java/lang/System".to_string()));
    constant_pool.add(Constant::Utf8("out".to_string()));
    constant_pool.add(Constant::Utf8("Ljava/io/PrintStream;".to_string()));
    constant_pool.add(Constant::String { string_index: 14 });
    constant_pool.add(Constant::Utf8("Hello, World!".to_string()));
    constant_pool.add(Constant::MethodRef {
        class_index: 16,
        name_and_type_index: 17,
    });
    constant_pool.add(Constant::Class { name_index: 18 });
    constant_pool.add(Constant::NameAndType {
        name_index: 19,
        descriptor_index: 20,
    });
    constant_pool.add(Constant::Utf8("java/io/PrintStream".to_string()));
    constant_pool.add(Constant::Utf8("println".to_string()));
    constant_pool.add(Constant::Utf8("(Ljava/lang/String;)V".to_string()));
    constant_pool.add(Constant::Class { name_index: 22 });
    let this_class = u16::try_from(constant_pool.len())?;
    constant_pool.add(Constant::Utf8("HelloWorld".to_string()));
    constant_pool.add(Constant::Utf8("Code".to_string()));
    constant_pool.add(Constant::Utf8("LineNumberTable".to_string()));
    constant_pool.add(Constant::Utf8("main".to_string()));
    constant_pool.add(Constant::Utf8("([Ljava/lang/String;)V".to_string()));

    let mut methods = Vec::new();
    let mut init_method = ristretto_classfile::Method {
        access_flags: MethodAccessFlags::PUBLIC,
        name_index: 5,
        descriptor_index: 6,
        attributes: Vec::new(),
    };
    init_method.attributes.push(Attribute::Code {
        name_index: 23,
        max_stack: 1,
        max_locals: 1,
        code: instructions_as_bytes(&vec![
            Instruction::Aload_0,
            Instruction::Invokespecial(1),
            Instruction::Return,
        ])?,
        exceptions: Vec::new(),
        attributes: Vec::new(),
    });
    methods.push(init_method);

    let mut main_method = ristretto_classfile::Method {
        access_flags: MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
        name_index: 25,
        descriptor_index: 26,
        attributes: Vec::new(),
    };
    main_method.attributes.push(Attribute::Code {
        name_index: 23,
        max_stack: 2,
        max_locals: 1,
        code: instructions_as_bytes(&vec![
            Instruction::Getstatic(7),
            Instruction::Ldc(13),
            Instruction::Invokevirtual(15),
            Instruction::Return,
        ])?,
        exceptions: Vec::new(),
        attributes: Vec::new(),
    });
    main_method.attributes.push(Attribute::LineNumberTable {
        name_index: 24,
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

fn instructions_as_bytes(instructions: &Vec<Instruction>) -> Result<Vec<u8>> {
    let mut bytes = Cursor::new(Vec::new());
    for instruction in instructions {
        instruction.to_bytes(&mut bytes)?;
    }
    Ok(bytes.into_inner())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_main() -> Result<()> {
        main()
    }
}
