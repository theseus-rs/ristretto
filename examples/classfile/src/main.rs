#![forbid(unsafe_code)]
#![deny(clippy::pedantic)]

use ristretto_classfile::{ClassFile, Constant, ConstantPool, Result, Version};

fn main() -> Result<()> {
    let mut constant_pool = ConstantPool::default();
    constant_pool.add(Constant::Utf8("Foo".to_string()));
    let utf8_index = u16::try_from(constant_pool.len())?;
    constant_pool.add(Constant::Class {
        name_index: utf8_index,
    });
    let class_index = u16::try_from(constant_pool.len())?;
    let class_file = ClassFile {
        version: Version::Java21 { minor: 0 },
        constant_pool,
        this_class: class_index,
        ..Default::default()
    };
    class_file.verify()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_main() -> Result<()> {
        main()
    }
}
