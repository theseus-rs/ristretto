use crate::Error::{FieldNotFound, MethodNotFound, PoisonedLock};
use crate::{Field, Method, Result};
use indexmap::IndexMap;
use ristretto_classfile::attributes::Attribute;
use ristretto_classfile::{BaseType, ClassFile, ConstantPool, FieldAccessFlags};
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::str::FromStr;
use std::sync::{Arc, RwLock};

/// A representation of a Java class.
#[derive(Debug)]
pub struct Class {
    name: String,
    source_file: Option<String>,
    class_file: ClassFile,
    parent: Arc<RwLock<Option<Arc<Class>>>>,
    interfaces: Arc<RwLock<Vec<Arc<Class>>>>,
    fields: IndexMap<String, Arc<Field>>,
    methods: HashMap<String, Arc<Method>>,
}

impl Class {
    /// Create a new class.
    #[must_use]
    pub fn new(
        name: String,
        source_file: Option<String>,
        class_file: ClassFile,
        parent: Option<Arc<Class>>,
        interfaces: Vec<Arc<Class>>,
        fields: IndexMap<String, Arc<Field>>,
        methods: HashMap<String, Arc<Method>>,
    ) -> Self {
        Self {
            name,
            source_file,
            class_file,
            parent: Arc::new(RwLock::new(parent)),
            interfaces: Arc::new(RwLock::new(interfaces)),
            fields,
            methods,
        }
    }

    /// Create a new array class.
    ///
    /// # Errors
    /// if the class name cannot be added to the constant pool
    pub fn new_array<S: AsRef<str>>(name: S) -> Result<Self> {
        let name = name.as_ref().to_string();
        let mut constant_pool = ConstantPool::new();
        let class_index = constant_pool.add_class(name.clone())?;
        let class_file = ClassFile {
            constant_pool,
            this_class: class_index,
            ..Default::default()
        };
        let methods = HashMap::new();
        Ok(Self {
            name,
            source_file: None,
            class_file,
            parent: Arc::new(RwLock::new(None)),
            interfaces: Arc::new(RwLock::new(Vec::new())),
            fields: IndexMap::new(),
            methods,
        })
    }

    /// Create a new class from the given class file.
    ///
    /// # Errors
    /// if the class file cannot be read.
    pub fn from(class_file: ClassFile) -> Result<Self> {
        let name = class_file.class_name()?.clone();
        let mut source_file = None;

        for attribute in &class_file.attributes {
            if let Attribute::SourceFile {
                source_file_index, ..
            } = attribute
            {
                let constant_pool = &class_file.constant_pool;
                let source_file_name = constant_pool.try_get_utf8(*source_file_index)?;
                source_file = Some(source_file_name.to_string());
                break;
            }
        }

        let mut fields = IndexMap::new();
        for class_field in &class_file.fields {
            let field = Field::from(&class_file, class_field)?;
            let field_name = field.name().to_string();
            fields.insert(field_name, Arc::new(field));
        }

        let mut methods = HashMap::new();
        for class_file_method in &class_file.methods {
            let method = Method::from(&class_file, class_file_method)?;
            let method_identifier = method.identifier();
            methods.insert(method_identifier, Arc::new(method));
        }

        Ok(Self {
            name,
            source_file,
            class_file,
            parent: Arc::new(RwLock::new(None)),
            interfaces: Arc::new(RwLock::new(Vec::new())),
            fields,
            methods,
        })
    }

    /// Get the class name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the raw component name for an array class.
    fn array_component_type(&self) -> &str {
        let mut component_type = self.name.trim_start_matches('[');
        if component_type.ends_with(';') {
            component_type = component_type
                .strip_prefix('L')
                .unwrap_or_default()
                .strip_suffix(';')
                .unwrap_or_default();
        }
        component_type
    }

    /// Get the component name for an array class.
    #[must_use]
    pub fn component_type(&self) -> Option<&str> {
        if !self.is_array() {
            return None;
        }
        let component_type = self.array_component_type();
        let component_type = match component_type {
            "B" => "byte",
            "C" => "char",
            "D" => "double",
            "F" => "float",
            "I" => "int",
            "J" => "long",
            "S" => "short",
            "Z" => "boolean",
            _ => component_type,
        };
        Some(component_type)
    }

    /// Get the class source file name.
    #[must_use]
    pub fn source_file(&self) -> Option<&str> {
        self.source_file.as_deref()
    }

    /// Determine if this class is an array
    #[must_use]
    pub fn is_array(&self) -> bool {
        self.name.starts_with('[')
    }

    /// Get the number of array dimensions
    #[must_use]
    pub fn array_dimensions(&self) -> usize {
        self.name.chars().filter(|&c| c == '[').count()
    }

    /// Determine if this class is a primitive
    #[must_use]
    pub fn is_primitive(&self) -> bool {
        if !self.is_array() {
            return false;
        }
        let component_type = self.array_component_type();
        let Ok(component_type) = char::from_str(component_type) else {
            return false;
        };
        BaseType::parse(component_type).is_ok()
    }

    /// Get the class file.
    #[must_use]
    pub fn class_file(&self) -> &ClassFile {
        &self.class_file
    }

    /// Get the parent class.
    ///
    /// # Errors
    pub fn parent(&self) -> Result<Option<Arc<Class>>> {
        let parent_guard = self
            .parent
            .read()
            .map_err(|error| PoisonedLock(error.to_string()))?;
        match parent_guard.as_ref() {
            Some(parent) => Ok(Some(parent.clone())),
            None => Ok(None),
        }
    }

    /// Set the parent class.
    ///
    /// # Errors
    pub fn set_parent(&self, parent: Option<Arc<Class>>) -> Result<()> {
        let mut parent_guard = self
            .parent
            .write()
            .map_err(|error| PoisonedLock(error.to_string()))?;
        *parent_guard = parent;
        Ok(())
    }

    /// Get the class interfaces.
    ///
    /// # Errors
    pub fn interfaces(&self) -> Result<Vec<Arc<Class>>> {
        let parent_guard = self
            .interfaces
            .read()
            .map_err(|error| PoisonedLock(error.to_string()))?;
        let interfaces = parent_guard.clone();
        Ok(interfaces)
    }

    /// Set the class interfaces.
    ///
    /// # Errors
    pub fn set_interfaces(&self, interfaces: Vec<Arc<Class>>) -> Result<()> {
        let mut interfaces_guard = self
            .interfaces
            .write()
            .map_err(|error| PoisonedLock(error.to_string()))?;
        *interfaces_guard = interfaces;
        Ok(())
    }

    /// Get the constant pool
    #[must_use]
    pub fn constant_pool(&self) -> &ConstantPool {
        &self.class_file.constant_pool
    }

    /// Get a mutable constant pool
    pub fn constant_pool_mut(&mut self) -> &mut ConstantPool {
        &mut self.class_file.constant_pool
    }

    /// Get a static field by name.
    ///
    /// # Errors
    /// if the field is not found.
    pub fn static_field<S: AsRef<str>>(&self, name: S) -> Result<Arc<Field>> {
        let name = name.as_ref();
        if let Some(field) = self.fields.get(name) {
            if !field.access_flags().contains(FieldAccessFlags::STATIC) {
                return Err(FieldNotFound {
                    class_name: self.name.to_string(),
                    field_name: name.to_string(),
                });
            }
            return Ok(field.clone());
        }

        let Some(parent) = &self.parent()? else {
            return Err(FieldNotFound {
                class_name: self.name.to_string(),
                field_name: name.to_string(),
            });
        };

        let field = parent.static_field(name)?;
        Ok(field)
    }

    /// Get a list of field names in the class hierarchy.
    ///
    /// # Errors
    /// if there is an issue accessing the parent class.
    fn field_names(&self) -> Result<Vec<String>> {
        let mut field_names = Vec::new();
        let mut parent = self.parent()?;
        while let Some(class) = parent {
            for field_name in class.fields.keys().rev() {
                field_names.insert(0, field_name.clone());
            }
            parent = class.parent()?;
        }

        for field_name in self.fields.keys() {
            field_names.push(field_name.clone());
        }
        Ok(field_names)
    }

    /// Field offset by name.  This is primarily used by the Unsafe class that references fields by
    /// offset.
    ///
    /// # Errors
    /// if the field is not found.
    pub fn field_offset<S: AsRef<str>>(&self, name: S) -> Result<usize> {
        let name = name.as_ref().to_string();
        let field_names = self.field_names()?;
        for (offset, field_name) in field_names.iter().enumerate() {
            if field_name == &name {
                return Ok(offset);
            }
        }
        Err(FieldNotFound {
            class_name: self.name().to_string(),
            field_name: name,
        })
    }

    /// Returns the field name for an offset.  This is primarily used by the Unsafe class that
    /// references fields by offset.
    ///
    /// # Errors
    /// if the field is not found.
    pub fn field_name(&self, offset: usize) -> Result<String> {
        let field_names = self.field_names()?;
        let Some(key) = field_names.get(offset) else {
            return Err(FieldNotFound {
                class_name: self.name().to_string(),
                field_name: offset.to_string(),
            });
        };

        Ok(key.to_string())
    }

    /// Get the class initializer method.
    #[must_use]
    pub fn class_initializer(&self) -> Option<Arc<Method>> {
        self.method("<clinit>", "()V")
    }

    /// Get an object initializer method.
    #[must_use]
    pub fn object_initializer(&self, descriptor: &str) -> Option<Arc<Method>> {
        self.method("<init>", descriptor)
    }

    /// Get the main method.
    #[must_use]
    pub fn main_method(&self) -> Option<Arc<Method>> {
        self.method("main", "([Ljava/lang/String;)V")
    }

    /// Get a method by name and descriptor.
    #[must_use]
    pub fn method<S: AsRef<str>>(&self, name: S, descriptor: S) -> Option<Arc<Method>> {
        let name = name.as_ref();
        let descriptor = descriptor.as_ref();
        let method_identifier = format!("{name}:{descriptor}");
        let method = self.methods.get(&method_identifier);
        method.cloned()
    }

    /// Get a method by name and descriptor.
    ///
    /// # Errors
    /// if the method is not found.
    pub fn try_get_method<S: AsRef<str>>(&self, name: S, descriptor: S) -> Result<Arc<Method>> {
        let name = name.as_ref();
        let descriptor = descriptor.as_ref();
        let Some(method) = self.method(name, descriptor) else {
            return Err(MethodNotFound {
                class_name: self.name().to_string(),
                method_name: name.to_string(),
                method_descriptor: descriptor.to_string(),
            });
        };
        Ok(method)
    }

    /// Determine if this class is assignable from the given class.
    ///
    /// # Errors
    /// if classes or interfaces cannot be accessed.
    pub fn is_assignable_from<S: AsRef<str>>(&self, class_name: S) -> Result<bool> {
        let class_name = class_name.as_ref();
        if self.name() == class_name {
            return Ok(true);
        }
        if let Some(parent) = self.parent()? {
            if parent.is_assignable_from(class_name)? {
                return Ok(true);
            }
        }
        for interface in self.interfaces()? {
            if interface.is_assignable_from(class_name)? {
                return Ok(true);
            }
        }
        Ok(false)
    }
}

impl PartialEq for Class {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.class_file == other.class_file
            && *self.parent.read().expect("parent") == *other.parent.read().expect("parent")
            && self.fields == other.fields
            && self.methods == other.methods
    }
}

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{runtime, Result};
    use std::io::Cursor;

    const JAVA_VERSION: &str = "21.0.4.7.1";

    async fn object_class() -> Result<Arc<Class>> {
        let (_version, class_loader) = runtime::class_loader(JAVA_VERSION).await?;
        class_loader.load("java/lang/Object").await
    }

    async fn string_class() -> Result<Arc<Class>> {
        let (_version, class_loader) = runtime::class_loader(JAVA_VERSION).await?;
        let string_class = class_loader.load("java/lang/String").await?;

        let object_class = object_class().await?;
        string_class.set_parent(Some(object_class))?;

        let serializable_class = serializable_class().await?;
        string_class.set_interfaces(vec![serializable_class])?;

        Ok(string_class)
    }

    async fn serializable_class() -> Result<Arc<Class>> {
        let (_version, class_loader) = runtime::class_loader(JAVA_VERSION).await?;
        class_loader.load("java/io/Serializable").await
    }

    fn simple_class() -> Result<Class> {
        let bytes = include_bytes!("../../classes/Simple.class").to_vec();
        let mut cursor = Cursor::new(bytes);
        let class_file = ClassFile::from_bytes(&mut cursor)?;
        Class::from(class_file)
    }

    #[test]
    fn test_new() -> Result<()> {
        let class = simple_class()?;
        assert_eq!("Simple", class.name());
        assert_eq!(Some("Simple.java"), class.source_file());
        assert_eq!("Simple", class.class_file().class_name()?);
        assert_eq!("Simple", format!("{class}"));
        assert_eq!(0, class.array_dimensions());
        assert!(!class.is_array());
        assert!(!class.is_primitive());
        Ok(())
    }

    #[test]
    fn test_new_array_boolean() -> Result<()> {
        let class = Class::new_array("[Z")?;
        assert_eq!("[Z", class.name());
        assert_eq!(Some("boolean"), class.component_type());
        assert_eq!(1, class.array_dimensions());
        assert!(class.is_array());
        assert!(class.is_primitive());
        Ok(())
    }

    #[test]
    fn test_new_array_byte() -> Result<()> {
        let class = Class::new_array("[B")?;
        assert_eq!("[B", class.name());
        assert_eq!(Some("byte"), class.component_type());
        assert_eq!(1, class.array_dimensions());
        assert!(class.is_array());
        assert!(class.is_primitive());
        Ok(())
    }

    #[test]
    fn test_new_array_char() -> Result<()> {
        let class = Class::new_array("[C")?;
        assert_eq!("[C", class.name());
        assert_eq!(Some("char"), class.component_type());
        assert_eq!(1, class.array_dimensions());
        assert!(class.is_array());
        assert!(class.is_primitive());
        Ok(())
    }

    #[test]
    fn test_new_array_double() -> Result<()> {
        let class = Class::new_array("[D")?;
        assert_eq!("[D", class.name());
        assert_eq!(Some("double"), class.component_type());
        assert_eq!(1, class.array_dimensions());
        assert!(class.is_array());
        assert!(class.is_primitive());
        Ok(())
    }

    #[test]
    fn test_new_array_float() -> Result<()> {
        let class = Class::new_array("[F")?;
        assert_eq!("[F", class.name());
        assert_eq!(Some("float"), class.component_type());
        assert_eq!(1, class.array_dimensions());
        assert!(class.is_array());
        assert!(class.is_primitive());
        Ok(())
    }

    #[test]
    fn test_new_array_int() -> Result<()> {
        let class = Class::new_array("[I")?;
        assert_eq!("[I", class.name());
        assert_eq!(Some("int"), class.component_type());
        assert_eq!(1, class.array_dimensions());
        assert!(class.is_array());
        assert!(class.is_primitive());
        Ok(())
    }

    #[test]
    fn test_new_array_long() -> Result<()> {
        let class = Class::new_array("[J")?;
        assert_eq!("[J", class.name());
        assert_eq!(Some("long"), class.component_type());
        assert_eq!(1, class.array_dimensions());
        assert!(class.is_array());
        assert!(class.is_primitive());
        Ok(())
    }

    #[test]
    fn test_new_array_short() -> Result<()> {
        let class = Class::new_array("[S")?;
        assert_eq!("[S", class.name());
        assert_eq!(Some("short"), class.component_type());
        assert_eq!(1, class.array_dimensions());
        assert!(class.is_array());
        assert!(class.is_primitive());
        Ok(())
    }

    #[test]
    fn test_new_array_string() -> Result<()> {
        let class = Class::new_array("[Ljava/lang/String;")?;
        assert_eq!("[Ljava/lang/String;", class.name());
        assert_eq!(Some("java/lang/String"), class.component_type());
        assert_eq!(1, class.array_dimensions());
        assert!(class.is_array());
        assert!(!class.is_primitive());
        Ok(())
    }

    #[test]
    fn test_new_array_multiple_dimensions() -> Result<()> {
        let class = Class::new_array("[[[[[B")?;
        assert_eq!("[[[[[B", class.name());
        assert_eq!(Some("byte"), class.component_type());
        assert_eq!(5, class.array_dimensions());
        assert!(class.is_array());
        assert!(class.is_primitive());
        Ok(())
    }

    #[test]
    fn test_class_initializer() -> Result<()> {
        let class = simple_class()?;
        let method = class.class_initializer().expect("class initializer");
        assert_eq!("<clinit>", method.name());
        assert_eq!("()V", method.descriptor());
        Ok(())
    }

    #[test]
    fn test_object_initializer() -> Result<()> {
        let class = simple_class()?;
        let method = class.object_initializer("()V").expect("class initializer");
        assert_eq!("<init>", method.name());
        assert_eq!("()V", method.descriptor());
        Ok(())
    }

    #[test]
    fn test_main_method() -> Result<()> {
        let class = simple_class()?;
        let method = class.main_method().expect("class initializer");
        assert_eq!("main", method.name());
        assert_eq!("([Ljava/lang/String;)V", method.descriptor());
        Ok(())
    }

    #[test]
    fn test_method() -> Result<()> {
        let class = simple_class()?;
        let name = "getPublicValue";
        let descriptor = "()I";
        let method = class.method(name, descriptor).expect("class initializer");
        assert_eq!(name, method.name());
        assert_eq!(descriptor, method.descriptor());
        Ok(())
    }

    #[tokio::test]
    async fn test_string_instanceof_object() -> Result<()> {
        let string_class = string_class().await?;
        let object_class = object_class().await?;
        assert!(string_class.is_assignable_from(&object_class.name)?);
        Ok(())
    }

    #[tokio::test]
    async fn test_object_instanceof_string() -> Result<()> {
        let object_class = object_class().await?;
        let string_class = string_class().await?;
        assert!(!object_class.is_assignable_from(&string_class.name)?);
        Ok(())
    }

    #[tokio::test]
    async fn test_string_instanceof_serializable() -> Result<()> {
        let string_class = string_class().await?;
        let serializable_class = serializable_class().await?;
        assert!(string_class.is_assignable_from(&serializable_class.name)?);
        Ok(())
    }

    #[tokio::test]
    async fn test_object_instanceof_serializable_class() -> Result<()> {
        let object_class = object_class().await?;
        let serializable_class = serializable_class().await?;
        assert!(!object_class.is_assignable_from(&serializable_class.name)?);
        Ok(())
    }
}
