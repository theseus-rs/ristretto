use crate::ClassLoader;
use ristretto_classfile::ClassFile;
use std::fmt::Debug;
use std::sync::Arc;

/// A class loaded by the class loader.
pub struct Class {
    class_loader: Arc<ClassLoader>,
    class_file: Arc<ClassFile>,
}

impl Class {
    /// Create a new class with the given class file.
    #[must_use]
    pub fn new(class_loader: Arc<ClassLoader>, class_file: Arc<ClassFile>) -> Self {
        Self {
            class_loader,
            class_file,
        }
    }

    /// Get the class loader.
    #[must_use]
    fn get_class_loader(&self) -> Arc<ClassLoader> {
        Arc::clone(&self.class_loader)
    }

    /// Get the class file.
    #[must_use]
    pub fn get_class_file(&self) -> Arc<ClassFile> {
        Arc::clone(&self.class_file)
    }
}

/// Implement the `Debug` trait for `Class`.
impl Debug for Class {
    /// Formats the class for debugging.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Class")
            .field("class_loader", &self.class_loader.get_name())
            .field("class_file", &self.class_file)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;
    use indoc::indoc;
    use std::io::Cursor;

    #[test]
    fn test_new() -> Result<()> {
        let class_loader = ClassLoader::default();
        let bytes = include_bytes!("../../classes/Simple.class").to_vec();
        let mut cursor = Cursor::new(bytes);
        let class_file = ClassFile::from_bytes(&mut cursor)?;
        let class = Class::new(Arc::new(class_loader), Arc::new(class_file));
        assert_eq!("bootstrap", class.get_class_loader().get_name());
        assert_eq!("Simple", class.get_class_file().class_name()?);
        Ok(())
    }

    #[test]
    fn test_debug() -> Result<()> {
        let class_loader = ClassLoader::default();
        let bytes = include_bytes!("../../classes/Minimum.class").to_vec();
        let mut cursor = Cursor::new(bytes);
        let class_file = ClassFile::from_bytes(&mut cursor)?;
        let class = Class::new(Arc::new(class_loader), Arc::new(class_file));
        let debug = format!("{class:?}");
        assert_eq!(
            debug,
            indoc! {r#"Class { class_loader: "bootstrap", class_file: ClassFile { version: Java21 { minor: 0 }, constant_pool: ConstantPool { constants: [Placeholder, Constant(MethodRef { class_index: 2, name_and_type_index: 3 }), Constant(Class(4)), Constant(NameAndType { name_index: 5, descriptor_index: 6 }), Constant(Utf8("java/lang/Object")), Constant(Utf8("<init>")), Constant(Utf8("()V")), Constant(Class(8)), Constant(Utf8("Minimum")), Constant(Utf8("Code")), Constant(Utf8("LineNumberTable")), Constant(Utf8("SourceFile")), Constant(Utf8("Minimum.java"))] }, access_flags: ClassAccessFlags(PUBLIC | SUPER), this_class: 7, super_class: 2, interfaces: [], fields: [], methods: [Method { access_flags: MethodAccessFlags(PUBLIC), name_index: 5, descriptor_index: 6, attributes: [Code { name_index: 9, max_stack: 1, max_locals: 1, code: [Aload_0, Invokespecial(1), Return], exceptions: [], attributes: [LineNumberTable { name_index: 10, line_numbers: [LineNumber { start_pc: 0, line_number: 1 }] }] }] }], attributes: [SourceFile { name_index: 11, source_file_index: 12 }] } }"#}
        );
        Ok(())
    }
}
