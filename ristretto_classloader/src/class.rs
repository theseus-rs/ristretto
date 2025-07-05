use crate::Error::{FieldNotFound, MethodNotFound, PoisonedLock};
use crate::field::FieldKey;
use crate::{Field, Method, Object, Result, Value};
use ristretto_classfile::attributes::Attribute;
use ristretto_classfile::{
    ClassAccessFlags, ClassFile, ConstantPool, FieldAccessFlags, MethodAccessFlags,
};
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::sync::{Arc, LazyLock, RwLock};

/// A list of methods that are designated as polymorphic in the Java Virtual Machine.
///
/// Polymorphic methods can accept different argument types and return types at different call
/// sites, even though they have a single method descriptor in the class file. The JVM uses a
/// special invokedynamic mechanism to dispatch these methods.
///
/// Each entry is a tuple of (`class_name`, `method_name`, `method_decsriptor`) that identifies a
/// polymorphic method. The method will be matched regardless of the descriptor at the call site.
///
/// These methods are primarily used with method handles and the invokedynamic instruction
/// in Java's reflection and lambda implementations.
///
/// TODO: This implementation should likely be refactored to use a more dynamic approach that looks
///       for the `PolymorphicSignature` annotation in the method attributes, rather than hardcoding
///       the method names and classes.
pub static POLYMORPHIC_METHODS: LazyLock<HashMap<(&'static str, &'static str), &'static str>> =
    LazyLock::new(|| {
        let mut map = HashMap::new();
        map.insert(
            ("java/lang/invoke/MethodHandle", "invoke"),
            "([Ljava/lang/Object;)Ljava/lang/Object;",
        );
        map.insert(
            ("java/lang/invoke/MethodHandle", "invokeBasic"),
            "([Ljava/lang/Object;)Ljava/lang/Object;",
        );
        map.insert(
            ("java/lang/invoke/MethodHandle", "invokeExact"),
            "([Ljava/lang/Object;)Ljava/lang/Object;",
        );
        map.insert(
            ("java/lang/invoke/MethodHandle", "linkToInterface"),
            "([Ljava/lang/Object;)Ljava/lang/Object;",
        );
        map.insert(
            ("java/lang/invoke/MethodHandle", "linkToNative"),
            "([Ljava/lang/Object;)Ljava/lang/Object;",
        );
        map.insert(
            ("java/lang/invoke/MethodHandle", "linkToSpecial"),
            "([Ljava/lang/Object;)Ljava/lang/Object;",
        );
        map.insert(
            ("java/lang/invoke/MethodHandle", "linkToStatic"),
            "([Ljava/lang/Object;)Ljava/lang/Object;",
        );
        map.insert(
            ("java/lang/invoke/MethodHandle", "linkToVirtual"),
            "([Ljava/lang/Object;)Ljava/lang/Object;",
        );
        map
    });

/// A representation of a Java class.
#[expect(clippy::struct_field_names)]
#[derive(Debug)]
pub struct Class {
    name: String,
    source_file: Option<String>,
    class_file: ClassFile,
    parent: Arc<RwLock<Option<Arc<Class>>>>,
    interfaces: Arc<RwLock<Vec<Arc<Class>>>>,
    static_fields: Vec<Arc<Field>>,
    static_values: Vec<Arc<RwLock<Value>>>,
    object_fields: Vec<Arc<Field>>,
    methods: HashMap<String, Arc<Method>>,
    object: Arc<RwLock<Option<Object>>>,
}

impl Class {
    /// Create a new named class.
    ///
    /// # Errors
    ///
    /// if the class name cannot be added to the constant pool
    pub fn new_named<S: AsRef<str>>(name: S) -> Result<Arc<Self>> {
        let name = name.as_ref().to_string();
        let mut constant_pool = ConstantPool::new();
        let class_index = constant_pool.add_class(name.clone())?;
        let class_file = ClassFile {
            constant_pool,
            this_class: class_index,
            ..Default::default()
        };
        let class = Arc::new(Self {
            name,
            source_file: None,
            class_file,
            parent: Arc::new(RwLock::new(None)),
            interfaces: Arc::new(RwLock::new(Vec::new())),
            static_fields: Vec::new(),
            static_values: Vec::new(),
            object_fields: Vec::new(),
            methods: HashMap::new(),
            object: Arc::new(RwLock::new(None)),
        });
        Ok(class)
    }

    /// Create a new class from the given class file.
    ///
    /// # Errors
    ///
    /// if the class file cannot be read.
    pub fn from(mut class_file: ClassFile) -> Result<Arc<Self>> {
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

        let mut static_fields = Vec::new();
        let mut static_values = Vec::new();
        let mut object_fields = Vec::new();
        let constant_pool = &class_file.constant_pool;
        for (index, class_field) in class_file.fields.iter().enumerate() {
            let index = u16::try_from(index)?;
            let field = Arc::new(Field::from(&class_file, index, class_field)?);
            if field.access_flags().contains(FieldAccessFlags::STATIC) {
                static_fields.push(field.clone());
                let value = field.default_static_value(constant_pool)?;
                static_values.push(Arc::new(RwLock::new(value)));
            } else {
                object_fields.push(field);
            }
        }

        let mut methods = HashMap::new();
        for class_file_method in &class_file.methods {
            let method = Method::from(&class_file, class_file_method)?;
            let signature = method.signature();
            methods.insert(signature, Arc::new(method));
        }

        if class_file.class_name()? == "java/lang/invoke/DirectMethodHandle$Holder" {
            Self::add_trampoline_methods(&mut class_file, &mut methods)?;
        }

        let class = Arc::new(Self {
            name: class_file.class_name()?.to_string(),
            source_file,
            class_file,
            parent: Arc::new(RwLock::new(None)),
            interfaces: Arc::new(RwLock::new(Vec::new())),
            static_fields,
            static_values,
            object_fields,
            methods,
            object: Arc::new(RwLock::new(None)),
        });
        Ok(class)
    }

    /// Get the class name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the package.
    #[must_use]
    pub fn package(&self) -> &str {
        let index = self.name.rfind('/').unwrap_or(self.name.len());
        &self.name[..index]
    }

    /// Transform the class name to a descriptor.
    #[must_use]
    pub fn to_descriptor(class_name: &str) -> String {
        match class_name {
            "boolean" => "Z".to_string(),
            "byte" => "B".to_string(),
            "char" => "C".to_string(),
            "double" => "D".to_string(),
            "float" => "F".to_string(),
            "int" => "I".to_string(),
            "long" => "J".to_string(),
            "short" => "S".to_string(),
            "void" => "V".to_string(),
            name => {
                let name = name.replace('.', "/");
                if name.starts_with('[') {
                    return name;
                }
                format!("L{name};")
            }
        }
    }

    /// Get the raw component name for an array class.
    #[must_use]
    pub fn array_component_type(&self) -> &str {
        let mut component_type = self.name().split('[').next_back().unwrap_or_default();
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
        self.name().starts_with('[')
    }

    /// Get the number of array dimensions
    #[must_use]
    pub fn array_dimensions(&self) -> usize {
        self.name().chars().filter(|&c| c == '[').count()
    }

    /// Determine if this class is an array
    #[must_use]
    pub fn is_interface(&self) -> bool {
        self.class_file
            .access_flags
            .contains(ClassAccessFlags::INTERFACE)
    }

    /// Determine if this class is a primitive
    #[must_use]
    pub fn is_primitive(&self) -> bool {
        matches!(
            self.name(),
            "boolean" | "byte" | "char" | "double" | "float" | "int" | "long" | "short" | "void"
        )
    }

    /// Get the class file.
    #[must_use]
    pub fn class_file(&self) -> &ClassFile {
        &self.class_file
    }

    /// Get the parent class.
    ///
    /// # Errors
    ///
    /// if the parent class cannot be accessed due to a poisoned lock.
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
    ///
    /// if the parent class cannot be set due to a poisoned lock.
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
    ///
    /// if the interfaces cannot be accessed due to a poisoned lock.
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
    ///
    /// if the interfaces cannot be set due to a poisoned lock.
    pub fn set_interfaces(&self, interfaces: Vec<Arc<Class>>) -> Result<()> {
        let mut interfaces_guard = self
            .interfaces
            .write()
            .map_err(|error| PoisonedLock(error.to_string()))?;
        *interfaces_guard = interfaces;
        Ok(())
    }

    /// Add trampoline methods for `java.lang.invoke.DirectMethodHandle$Holder`.  Method signatures
    /// are typically expected to be added dynamically at runtime during
    /// `java/lang/invoke/MethodHandleNatives.resolve(...)`, but for the sake of simplicity, and to
    /// avoid needing put `methods` in an `Arc<RwLock<>>`, we add them here for now.
    fn add_trampoline_methods(
        class_file: &mut ClassFile,
        methods: &mut HashMap<String, Arc<Method>>,
    ) -> Result<()> {
        let method_signatures = [
            ("invokeInterface", "([Ljava/lang/Object;)Ljava/lang/Object;"),
            ("invokeSpecial", "([Ljava/lang/Object;)Ljava/lang/Object;"),
            ("invokeStatic", "([Ljava/lang/Object;)Ljava/lang/Object;"),
            ("invokeVirtual", "([Ljava/lang/Object;)Ljava/lang/Object;"),
        ];
        for (name, descriptor) in method_signatures {
            let constant_pool = &mut class_file.constant_pool;
            let method_index =
                constant_pool.add_method_ref(class_file.this_class, name, descriptor)?;
            let (_class_index, name_and_type_index) =
                constant_pool.try_get_method_ref(method_index)?;
            let (name_index, descriptor_index) =
                constant_pool.try_get_name_and_type(*name_and_type_index)?;
            let access_flags =
                MethodAccessFlags::PRIVATE | MethodAccessFlags::STATIC | MethodAccessFlags::NATIVE;
            let method = ristretto_classfile::Method {
                name_index: *name_index,
                descriptor_index: *descriptor_index,
                access_flags,
                attributes: vec![],
            };
            let method = Method::from(class_file, &method)?;
            methods.insert(method.signature(), Arc::new(method));
        }
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

    /// Get the declared fields for the class. The fields are returned in the order they are defined
    /// in the class file.
    #[must_use]
    pub fn declared_fields(&self) -> Vec<Arc<Field>> {
        let mut fields = self
            .static_fields
            .iter()
            .chain(self.object_fields.iter())
            .cloned()
            .collect::<Vec<_>>();
        // Sort the fields by field index to ensure the order is consistent with the order they are
        // defined in the class file.
        fields.sort_by_key(|field| field.offset());
        fields
    }

    /// Get a declared field by key.
    ///
    /// # Errors
    ///
    /// if the field is not found.
    pub fn declared_field<K: FieldKey>(&self, key: K) -> Result<Arc<Field>> {
        let declared_fields = self.declared_fields();
        if let Some((_index, field)) = key.get_field(&declared_fields) {
            return Ok(field.clone());
        }
        let Some(parent) = &self.parent()? else {
            return Err(FieldNotFound {
                class_name: self.name().to_string(),
                field_name: key.to_string(),
            });
        };
        let Ok(field) = parent.declared_field(key) else {
            return Err(FieldNotFound {
                class_name: self.name().to_string(),
                field_name: key.to_string(),
            });
        };
        Ok(field)
    }

    /// Get a static field by key.
    ///
    /// # Errors
    ///
    /// if the field is not found.
    pub fn static_field<K: FieldKey>(&self, key: K) -> Result<Arc<Field>> {
        if let Some((_index, field)) = key.get_field(&self.static_fields) {
            return Ok(field.clone());
        }

        if let Some(parent) = &self.parent()?
            && let Ok(field) = parent.static_field(key)
        {
            return Ok(field);
        }

        Err(FieldNotFound {
            class_name: self.name().to_string(),
            field_name: key.to_string(),
        })
    }

    /// Get static field and value lock by key.
    ///
    /// # Errors
    ///
    /// if the field cannot be found.
    fn static_field_value<K: FieldKey>(&self, key: K) -> Result<(Arc<Field>, Arc<RwLock<Value>>)> {
        if let Some((index, field)) = key.get_field(&self.static_fields)
            && let Some(value_lock) = self.static_values.get(index)
        {
            return Ok((field.clone(), value_lock.clone()));
        }

        if let Some(parent) = &self.parent()?
            && let Ok((field, value)) = parent.static_field_value(key)
        {
            return Ok((field, value));
        }

        Err(FieldNotFound {
            class_name: self.name().to_string(),
            field_name: key.to_string(),
        })
    }

    /// Get a static field value by key.
    ///
    /// # Errors
    ///
    /// if the field is not found.
    pub fn static_value<K: FieldKey>(&self, key: K) -> Result<Value> {
        if let Ok((_field, value_lock)) = self.static_field_value(key) {
            let value_guard = value_lock
                .read()
                .map_err(|error| PoisonedLock(error.to_string()))?;
            return Ok(value_guard.clone());
        }

        if let Some(parent) = &self.parent()?
            && let Ok(value) = parent.static_value(key)
        {
            return Ok(value);
        }

        Err(FieldNotFound {
            class_name: self.name().to_string(),
            field_name: key.to_string(),
        })
    }

    /// Set a static field value.
    ///
    /// # Errors
    ///
    /// if the field is not found.
    pub fn set_static_value<K: FieldKey>(&self, key: K, value: Value) -> Result<()> {
        let (field, value_lock) = self.static_field_value(key)?;
        field.check_value(&value)?;
        let mut value_guard = value_lock
            .write()
            .map_err(|error| PoisonedLock(error.to_string()))?;
        *value_guard = value;
        Ok(())
    }

    /// Set a static field value without checking field constraints.
    ///
    /// # Errors
    ///
    /// if the field is not found.
    pub fn set_static_value_unchecked<K: FieldKey>(&self, key: K, value: Value) -> Result<()> {
        let (_field, value_lock) = self.static_field_value(key)?;
        let mut value_guard = value_lock
            .write()
            .map_err(|error| PoisonedLock(error.to_string()))?;
        *value_guard = value;
        Ok(())
    }

    /// Get the object fields for this class.
    ///
    /// # Errors
    ///
    /// if there is an issue accessing the parent class due to a poisoned lock.
    #[must_use]
    pub fn object_fields(&self) -> &[Arc<Field>] {
        &self.object_fields
    }

    /// Get all object fields in the class hierarchy, including those from parent classes.
    ///
    /// # Errors
    ///
    /// if there is an issue accessing the parent class due to a poisoned lock.
    pub fn all_object_fields(&self) -> Result<Vec<Arc<Field>>> {
        let mut fields = Vec::with_capacity(self.object_fields.len());

        // Collect all classes in hierarchy from root to current class
        let mut class_hierarchy = Vec::new();
        let mut current_class = self.parent()?;
        while let Some(class) = current_class {
            class_hierarchy.push(class.clone());
            current_class = class.parent()?;
        }

        // Reverse to go from root (Object) to current class
        class_hierarchy.reverse();

        // Process fields from parent to child to maintain correct ordering
        for class in class_hierarchy {
            let object_fields = class.object_fields();
            for field in object_fields {
                fields.push(field.clone());
            }
        }

        // Add the fields from the current class
        for field in &self.object_fields {
            fields.push(field.clone());
        }

        Ok(fields)
    }

    /// Get a list of field names in the class hierarchy.
    ///
    /// # Errors
    ///
    /// if there is an issue accessing the parent class.
    fn field_names(&self) -> Result<Vec<String>> {
        let mut field_names = Vec::new();
        let mut parent = self.parent()?;
        while let Some(class) = parent {
            for field in class.declared_fields().iter().rev() {
                let field_name = field.name().to_string();
                field_names.insert(0, field_name);
            }
            parent = class.parent()?;
        }

        for field in self.declared_fields() {
            let field_name = field.name().to_string();
            field_names.push(field_name);
        }
        Ok(field_names)
    }

    /// Field offset by name.  This is primarily used by the Unsafe class that references fields by
    /// offset.
    ///
    /// # Errors
    ///
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
    ///
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

    /// Get all methods.
    #[must_use]
    pub fn methods(&self) -> Vec<Arc<Method>> {
        self.methods.values().cloned().collect()
    }

    /// Get the main method.
    #[must_use]
    pub fn main_method(&self) -> Option<Arc<Method>> {
        let method = self.method("main", "([Ljava/lang/String;)V")?;
        if !method
            .access_flags()
            .contains(MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC)
        {
            return None;
        }
        Some(method)
    }

    /// Get a method by name and descriptor.
    #[must_use]
    pub fn method<N, D>(&self, name: N, descriptor: D) -> Option<Arc<Method>>
    where
        N: AsRef<str>,
        D: AsRef<str>,
    {
        let name = name.as_ref();
        let descriptor = descriptor.as_ref();
        let signature = format!("{name}{descriptor}");
        if let Some(method) = self.methods.get(&signature) {
            return Some(method.clone());
        }

        let Ok(class_name) = self.class_file.class_name() else {
            return None;
        };

        if let Some(polymorphic_descriptor) = POLYMORPHIC_METHODS.get(&(class_name, name)) {
            // If the class name and method name match a polymorphic method, we can return it
            // regardless of the descriptor.
            let signature = format!("{name}{polymorphic_descriptor}");
            if let Some(method) = self.methods.get(&signature) {
                return Some(method.clone());
            }
        }

        None
    }

    /// Get a method by name and descriptor.
    ///
    /// # Errors
    ///
    /// if the method is not found.
    pub fn try_get_method<N, D>(&self, name: N, descriptor: D) -> Result<Arc<Method>>
    where
        N: AsRef<str>,
        D: AsRef<str>,
    {
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

    /// Get the object for the class.
    ///
    /// # Errors
    ///
    /// if the object cannot be accessed due to a poisoned lock.
    pub fn object(&self) -> Result<Option<Object>> {
        let object_guard = self
            .object
            .read()
            .map_err(|error| PoisonedLock(error.to_string()))?;
        match object_guard.as_ref() {
            Some(object) => Ok(Some(object.clone())),
            None => Ok(None),
        }
    }

    /// Set the object for the class.
    ///
    /// # Errors
    ///
    /// if the object cannot be set due to a poisoned lock.
    pub fn set_object(&self, object: Option<Object>) -> Result<()> {
        let mut object_guard = self
            .object
            .write()
            .map_err(|error| PoisonedLock(error.to_string()))?;
        *object_guard = object;
        Ok(())
    }

    /// Determine if this class is a subclass of the given class.
    ///
    /// # Errors
    ///
    /// if the parent class cannot be accessed due to a poisoned lock.
    pub fn is_subclass_of(&self, class: &Arc<Class>) -> Result<bool> {
        let mut current: Option<Arc<Class>> = self.parent()?;
        while let Some(ref parent) = current {
            if parent.name() == class.name() {
                return Ok(true);
            }
            current = parent.parent()?;
        }
        Ok(false)
    }

    /// Determine if this class is assignable from the given class.
    ///
    /// # Errors
    ///
    /// if classes or interfaces cannot be accessed.
    pub fn is_assignable_from(&self, class: &Arc<Class>) -> Result<bool> {
        if self.name() == class.name() || self.name() == "java/lang/Object" {
            return Ok(true);
        }

        if self.is_array() {
            if !class.is_array() {
                return Ok(false);
            }
            // If the array is an array of primitives, then the two arrays are assignable if they
            // have the same number of dimensions and the component types are the same.
            if !self.name().ends_with(';') || !class.name().ends_with(';') {
                return Ok(false);
            }
            if self.array_dimensions() > class.array_dimensions() {
                return Ok(false);
            }
            let component_name = self.component_type();
            if component_name == Some("java/lang/Object")
                || component_name == class.component_type()
            {
                return Ok(true);
            }
            return Ok(false);
        }

        if let Some(parent) = class.parent()? {
            if self.is_assignable_from(&parent)? {
                return Ok(true);
            }
        }

        for interface in class.interfaces()? {
            if self.is_assignable_from(&interface)? {
                return Ok(true);
            }
        }

        Ok(false)
    }
}

impl Eq for Class {}

impl PartialEq for Class {
    fn eq(&self, other: &Self) -> bool {
        // Optimization for the case where the two classes are the same instance.
        if std::ptr::eq(self, other) {
            return true;
        }

        // TODO: This is a very basic equality check. It should be extended to include static_values
        self.name() == other.name()
            && self.source_file == other.source_file
            && self.class_file == other.class_file
            && *self.parent.read().expect("parent") == *other.parent.read().expect("parent")
            && *self.interfaces.read().expect("parent") == *other.interfaces.read().expect("parent")
            && self.static_fields == other.static_fields
            && self.object_fields == other.object_fields
            && self.methods == other.methods
            && *self.object.read().expect("parent") == *other.object.read().expect("parent")
    }
}

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error::IllegalAccessError;
    use crate::{Error, Result, runtime};
    use ristretto_classfile::{BaseType, FieldType};
    use std::io::Cursor;

    async fn object_class() -> Result<Arc<Class>> {
        let (_java_home, _java_version, class_loader) = runtime::default_class_loader().await?;
        class_loader.load("java.lang.Object").await
    }

    async fn string_class() -> Result<Arc<Class>> {
        let (_java_home, _java_version, class_loader) = runtime::default_class_loader().await?;
        let string_class = class_loader.load("java.lang.String").await?;

        let object_class = object_class().await?;
        string_class.set_parent(Some(object_class))?;

        let serializable_class = serializable_class().await?;
        string_class.set_interfaces(vec![serializable_class])?;

        Ok(string_class)
    }

    async fn static_class() -> Result<Arc<Class>> {
        let mut constant_pool = ConstantPool::new();
        let class_name_index = constant_pool.add_class("StaticTest")?;

        let static_field_index =
            constant_pool.add_field_ref(class_name_index, "staticField", "I")?;
        let (_class_index, name_and_type_index) =
            constant_pool.try_get_field_ref(static_field_index)?;
        let (name_index, descriptor_index) =
            constant_pool.try_get_name_and_type(*name_and_type_index)?;
        let static_field = ristretto_classfile::Field {
            name_index: *name_index,
            descriptor_index: *descriptor_index,
            access_flags: FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC,
            field_type: FieldType::Base(BaseType::Int),
            attributes: vec![],
        };

        let static_final_field_index =
            constant_pool.add_field_ref(class_name_index, "staticFinalField", "I")?;
        let (_class_index, name_and_type_index) =
            constant_pool.try_get_field_ref(static_final_field_index)?;
        let (name_index, descriptor_index) =
            constant_pool.try_get_name_and_type(*name_and_type_index)?;
        let static_final_field = ristretto_classfile::Field {
            name_index: *name_index,
            descriptor_index: *descriptor_index,
            access_flags: FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC,
            field_type: FieldType::Base(BaseType::Int),
            attributes: vec![],
        };

        let class_file = ClassFile {
            constant_pool,
            this_class: class_name_index,
            access_flags: ClassAccessFlags::PUBLIC,
            fields: vec![static_field, static_final_field],
            ..Default::default()
        };
        let class = Class::from(class_file)?;
        let (_java_home, _java_version, class_loader) = runtime::default_class_loader().await?;
        class_loader.register(class.clone()).await?;
        Ok(class)
    }

    async fn serializable_class() -> Result<Arc<Class>> {
        let (_java_home, _java_version, class_loader) = runtime::default_class_loader().await?;
        class_loader.load("java.io.Serializable").await
    }

    fn simple_class() -> Result<Arc<Class>> {
        let bytes = include_bytes!("../../classes/Simple.class").to_vec();
        let mut cursor = Cursor::new(bytes);
        let class_file = ClassFile::from_bytes(&mut cursor)?;
        Class::from(class_file)
    }

    #[tokio::test]
    async fn test_new() -> Result<()> {
        let class = string_class().await?;
        assert_eq!("java/lang/String", class.name());
        assert_eq!("java/lang", class.package());
        assert_eq!(None, class.component_type());
        assert_eq!(0, class.array_dimensions());
        assert!(!class.is_array());
        Ok(())
    }

    #[test]
    fn test_to_descriptor() {
        assert_eq!("Z", Class::to_descriptor("boolean"));
        assert_eq!("B", Class::to_descriptor("byte"));
        assert_eq!("C", Class::to_descriptor("char"));
        assert_eq!("D", Class::to_descriptor("double"));
        assert_eq!("F", Class::to_descriptor("float"));
        assert_eq!("I", Class::to_descriptor("int"));
        assert_eq!("J", Class::to_descriptor("long"));
        assert_eq!("S", Class::to_descriptor("short"));
        assert_eq!("V", Class::to_descriptor("void"));
        assert_eq!(
            "Ljava/lang/String;",
            Class::to_descriptor("java.lang.String")
        );
        assert_eq!(
            "Ljava/lang/String;",
            Class::to_descriptor("java/lang/String")
        );
        assert_eq!(
            "[Ljava/lang/String;",
            Class::to_descriptor("[Ljava/lang/String;")
        );
        assert_eq!("[Z", Class::to_descriptor("[Z"));
        assert_eq!("[B", Class::to_descriptor("[B"));
        assert_eq!("[C", Class::to_descriptor("[C"));
        assert_eq!("[D", Class::to_descriptor("[D"));
        assert_eq!("[F", Class::to_descriptor("[F"));
        assert_eq!("[I", Class::to_descriptor("[I"));
        assert_eq!("[J", Class::to_descriptor("[J"));
        assert_eq!("[S", Class::to_descriptor("[S"));
        assert_eq!("[V", Class::to_descriptor("[V"));
    }

    #[test]
    fn test_new_array_boolean() -> Result<()> {
        let class = Class::new_named("[Z")?;
        assert_eq!("[Z", class.name());
        assert_eq!(Some("boolean"), class.component_type());
        assert_eq!(1, class.array_dimensions());
        assert!(class.is_array());
        Ok(())
    }

    #[test]
    fn test_new_array_byte() -> Result<()> {
        let class = Class::new_named("[B")?;
        assert_eq!("[B", class.name());
        assert_eq!(Some("byte"), class.component_type());
        assert_eq!(1, class.array_dimensions());
        assert!(class.is_array());
        Ok(())
    }

    #[test]
    fn test_new_array_char() -> Result<()> {
        let class = Class::new_named("[C")?;
        assert_eq!("[C", class.name());
        assert_eq!(Some("char"), class.component_type());
        assert_eq!(1, class.array_dimensions());
        assert!(class.is_array());
        Ok(())
    }

    #[test]
    fn test_new_array_double() -> Result<()> {
        let class = Class::new_named("[D")?;
        assert_eq!("[D", class.name());
        assert_eq!(Some("double"), class.component_type());
        assert_eq!(1, class.array_dimensions());
        assert!(class.is_array());
        Ok(())
    }

    #[test]
    fn test_new_array_float() -> Result<()> {
        let class = Class::new_named("[F")?;
        assert_eq!("[F", class.name());
        assert_eq!(Some("float"), class.component_type());
        assert_eq!(1, class.array_dimensions());
        assert!(class.is_array());
        Ok(())
    }

    #[test]
    fn test_new_array_int() -> Result<()> {
        let class = Class::new_named("[I")?;
        assert_eq!("[I", class.name());
        assert_eq!(Some("int"), class.component_type());
        assert_eq!(1, class.array_dimensions());
        assert!(class.is_array());
        Ok(())
    }

    #[test]
    fn test_new_array_long() -> Result<()> {
        let class = Class::new_named("[J")?;
        assert_eq!("[J", class.name());
        assert_eq!(Some("long"), class.component_type());
        assert_eq!(1, class.array_dimensions());
        assert!(class.is_array());
        Ok(())
    }

    #[test]
    fn test_new_array_short() -> Result<()> {
        let class = Class::new_named("[S")?;
        assert_eq!("[S", class.name());
        assert_eq!(Some("short"), class.component_type());
        assert_eq!(1, class.array_dimensions());
        assert!(class.is_array());
        Ok(())
    }

    #[test]
    fn test_new_array_string() -> Result<()> {
        let class = Class::new_named("[Ljava/lang/String;")?;
        assert_eq!("[Ljava/lang/String;", class.name());
        assert_eq!(Some("java/lang/String"), class.component_type());
        assert_eq!(1, class.array_dimensions());
        assert!(class.is_array());
        Ok(())
    }

    #[test]
    fn test_new_array_multiple_dimensions() -> Result<()> {
        let class = Class::new_named("[[[[[B")?;
        assert_eq!("[[[[[B", class.name());
        assert_eq!(Some("byte"), class.component_type());
        assert_eq!(5, class.array_dimensions());
        assert!(class.is_array());
        Ok(())
    }

    #[tokio::test]
    async fn test_source_file() -> Result<()> {
        let class = string_class().await?;
        assert_eq!(Some("String.java"), class.source_file());
        Ok(())
    }

    #[tokio::test]
    async fn test_is_array() -> Result<()> {
        let string_class = string_class().await?;
        assert!(!string_class.is_array());
        let int_array_class = Class::new_named("[I")?;
        assert!(int_array_class.is_array());
        Ok(())
    }

    #[tokio::test]
    async fn test_is_interface() -> Result<()> {
        let string_class = string_class().await?;
        assert!(!string_class.is_interface());
        let serializable_class = serializable_class().await?;
        assert!(serializable_class.is_interface());
        Ok(())
    }

    #[tokio::test]
    async fn test_is_primitive() -> Result<()> {
        let string_class = string_class().await?;
        assert!(!string_class.is_primitive());
        let int_class = Class::new_named("int")?;
        assert!(int_class.is_primitive());
        Ok(())
    }

    #[tokio::test]
    async fn test_class_file() -> Result<()> {
        let class = string_class().await?;
        let class_file = class.class_file();
        assert_eq!("java/lang/String", class_file.class_name()?);
        Ok(())
    }

    #[tokio::test]
    async fn test_parent() -> Result<()> {
        let string_class = string_class().await?;
        let parent = string_class
            .parent()?
            .ok_or(Error::ClassNotFound("java.lang.Object".to_string()))?;
        assert_eq!("java/lang/Object", parent.name());
        Ok(())
    }

    #[tokio::test]
    async fn test_interfaces() -> Result<()> {
        let string_class = string_class().await?;
        let serializable_class = serializable_class().await?;
        let interfaces = string_class.interfaces()?;
        assert!(interfaces.contains(&serializable_class));
        Ok(())
    }

    #[test]
    fn test_constant_pool() -> Result<()> {
        let class = Class::new_named("[Z")?;
        let constant_pool = class.constant_pool();
        assert!(!constant_pool.is_empty());
        Ok(())
    }

    #[test]
    fn test_constant_pool_mut() -> Result<()> {
        let mut class = Class::new_named("[Z")?;
        let class = Arc::get_mut(&mut class).expect("class");
        let constant_pool = class.constant_pool_mut();
        let index = constant_pool.add_string("foo")?;
        assert!(index > 0);
        Ok(())
    }

    #[tokio::test]
    async fn test_fields() -> Result<()> {
        let class = string_class().await?;
        let fields = class.declared_fields();
        assert_eq!(11, fields.len());
        Ok(())
    }

    #[tokio::test]
    async fn test_field() -> Result<()> {
        let class = string_class().await?;
        let result = class.declared_field("value");
        assert!(result.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn test_field_not_found() -> Result<()> {
        let class = string_class().await?;
        let result = class.declared_field("foo");
        assert!(matches!(
            result,
            Err(FieldNotFound { class_name, field_name })
            if class.name() == class_name && field_name == "foo"
        ));
        Ok(())
    }

    #[tokio::test]
    async fn test_static_field() -> Result<()> {
        let class = string_class().await?;
        let result = class.static_field("serialVersionUID");
        assert!(result.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn test_static_field_not_found() -> Result<()> {
        let class = string_class().await?;
        let result = class.static_field("foo");
        assert!(matches!(
            result,
            Err(FieldNotFound { class_name, field_name })
            if class.name() == class_name && field_name == "foo"
        ));
        Ok(())
    }

    #[tokio::test]
    async fn test_static_field_found_but_not_static() -> Result<()> {
        let class = string_class().await?;
        let result = class.static_field("value");
        assert!(matches!(
            result,
            Err(FieldNotFound { class_name, field_name })
            if class.name() == class_name && field_name == "value"
        ));
        Ok(())
    }

    #[tokio::test]
    async fn test_static_value() -> Result<()> {
        let class = string_class().await?;
        let result: i64 = class.static_value("serialVersionUID")?.try_into()?;
        assert_eq!(result, -6_849_794_470_754_667_710);
        Ok(())
    }

    #[tokio::test]
    async fn test_static_value_not_found() -> Result<()> {
        let class = string_class().await?;
        let result = class.static_value("foo");
        assert!(matches!(
            result,
            Err(FieldNotFound { class_name, field_name })
            if class.name() == class_name && field_name == "foo"
        ));
        Ok(())
    }

    #[tokio::test]
    async fn test_static_value_found_but_not_static() -> Result<()> {
        let class = string_class().await?;
        let result = class.static_value("value");
        assert!(matches!(
            result,
            Err(FieldNotFound { class_name, field_name })
            if class.name() == class_name && field_name == "value"
        ));
        Ok(())
    }

    #[tokio::test]
    async fn test_set_static_value() -> Result<()> {
        let class = static_class().await?;
        class.set_static_value("staticField", Value::Int(42))?;
        let value: i32 = class.static_value("staticField")?.try_into()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_set_static_value_final() -> Result<()> {
        let class = string_class().await?;
        let _result = class.set_static_value("staticFinalField", Value::Int(42));
        // TODO: Check that final fields that are already set throw an error
        // assert!(matches!(
        //     result,
        //     Err(IllegalAccessError(message))
        //     if message == "staticFinalField"
        // ));
        Ok(())
    }

    #[tokio::test]
    async fn test_set_static_value_invalid_type() -> Result<()> {
        let class = static_class().await?;
        let result = class.set_static_value("staticField", Value::Object(None));
        assert!(matches!(
            result,
            Err(IllegalAccessError(message))
            if message == "Invalid value for int field"
        ));
        Ok(())
    }

    #[tokio::test]
    async fn test_set_static_value_not_found() -> Result<()> {
        let class = static_class().await?;
        let result = class.set_static_value("foo", Value::Object(None));
        assert!(matches!(
            result,
            Err(FieldNotFound { class_name, field_name })
            if class.name() == class_name && field_name == "foo"
        ));
        Ok(())
    }

    #[tokio::test]
    async fn test_set_static_value_found_but_not_static() -> Result<()> {
        let class = string_class().await?;
        let result = class.set_static_value("value", Value::Object(None));
        assert!(matches!(
            result,
            Err(FieldNotFound { class_name, field_name })
            if class.name() == class_name && field_name == "value"
        ));
        Ok(())
    }

    #[tokio::test]
    async fn test_set_static_value_unchecked() -> Result<()> {
        let class = static_class().await?;
        class.set_static_value_unchecked("staticField", Value::Int(42))?;
        let value: i32 = class.static_value("staticField")?.try_into()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_set_static_value_unchecked_final() -> Result<()> {
        let class = static_class().await?;
        class.set_static_value_unchecked("staticFinalField", Value::Int(42))?;
        let value: i32 = class.static_value("staticFinalField")?.try_into()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_set_static_value_unchecked_invalid_type() -> Result<()> {
        let class = static_class().await?;
        class.set_static_value_unchecked("staticField", Value::Int(42))?;
        let value: i32 = class.static_value("staticField")?.try_into()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_set_static_value_unchecked_not_found() -> Result<()> {
        let class = static_class().await?;
        let result = class.set_static_value_unchecked("foo", Value::Object(None));
        assert!(matches!(
            result,
            Err(FieldNotFound { class_name, field_name })
            if class.name() == class_name && field_name == "foo"
        ));
        Ok(())
    }

    #[tokio::test]
    async fn test_set_static_value_unchecked_found_but_not_static() -> Result<()> {
        let class = string_class().await?;
        let result = class.set_static_value_unchecked("value", Value::Object(None));
        assert!(matches!(
            result,
            Err(FieldNotFound { class_name, field_name })
            if class.name() == class_name && field_name == "value"
        ));
        Ok(())
    }

    #[tokio::test]
    async fn test_object_fields() -> Result<()> {
        let class = string_class().await?;
        let fields = class.object_fields();
        let expected_names = ["value", "coder", "hash", "hashIsZero"];
        assert_eq!(fields.len(), expected_names.len());
        for (i, field) in fields.iter().enumerate() {
            assert_eq!(expected_names[i], field.name());
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_all_object_fields() -> Result<()> {
        let (_java_home, _java_version, class_loader) = runtime::default_class_loader().await?;
        let abstract_string_builder_class =
            class_loader.load("java.lang.AbstractStringBuilder").await?;
        let class = class_loader.load("java.lang.StringBuilder").await?;
        class.set_parent(Some(abstract_string_builder_class))?;
        let fields = class.all_object_fields()?;
        let expected_names = ["value", "coder", "maybeLatin1", "count"];
        assert_eq!(fields.len(), expected_names.len());
        for (i, field) in fields.iter().enumerate() {
            assert_eq!(expected_names[i], field.name());
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_field_names() -> Result<()> {
        let class = string_class().await?;
        let field_names = class.field_names()?;
        assert!(field_names.contains(&"value".to_string()));
        Ok(())
    }

    #[tokio::test]
    async fn test_field_offset() -> Result<()> {
        let class = string_class().await?;
        let offset = class.field_offset("value")?;
        assert_eq!(0, offset);
        Ok(())
    }

    #[tokio::test]
    async fn test_field_offset_not_found() -> Result<()> {
        let class = string_class().await?;
        let result = class.field_offset("foo");
        assert!(matches!(
            result,
            Err(FieldNotFound { class_name, field_name })
            if class.name() == class_name && field_name == "foo"
        ));
        Ok(())
    }

    #[tokio::test]
    async fn test_field_name() -> Result<()> {
        let class = string_class().await?;
        let name = class.field_name(0)?;
        assert_eq!("value", name);
        Ok(())
    }

    #[tokio::test]
    async fn test_field_name_not_found() -> Result<()> {
        let class = string_class().await?;
        let result = class.field_name(usize::MAX);
        assert!(matches!(
            result,
            Err(FieldNotFound { class_name, field_name })
            if class.name() == class_name && field_name == usize::MAX.to_string()
        ));
        Ok(())
    }

    #[tokio::test]
    async fn test_class_initializer() -> Result<()> {
        let class = string_class().await?;
        let method = class.class_initializer().expect("class initializer");
        assert_eq!("<clinit>", method.name());
        assert_eq!("()V", method.descriptor());
        Ok(())
    }

    #[tokio::test]
    async fn test_object_initializer() -> Result<()> {
        let class = string_class().await?;
        let method = class
            .object_initializer("()V")
            .expect("instant initializer");
        assert_eq!("<init>", method.name());
        assert_eq!("()V", method.descriptor());
        Ok(())
    }

    #[test]
    fn test_main_method() -> Result<()> {
        let class = simple_class()?;
        let method = class.main_method().expect("method");
        assert_eq!("main", method.name());
        assert_eq!("([Ljava/lang/String;)V", method.descriptor());
        Ok(())
    }

    #[tokio::test]
    async fn test_method() -> Result<()> {
        let class = string_class().await?;
        let name = "isEmpty";
        let descriptor = "()Z";
        let method = class.method(name, descriptor).expect("method");
        assert_eq!(name, method.name());
        assert_eq!(descriptor, method.descriptor());
        Ok(())
    }

    #[tokio::test]
    async fn test_method_polymorphic() -> Result<()> {
        let (_java_home, _java_version, class_loader) = runtime::default_class_loader().await?;
        let class = class_loader.load("java.lang.invoke.MethodHandle").await?;
        let name = "linkToStatic";
        let descriptor = "(Ljava/lang/invoke/MemberName;)Ljava/lang/Object;";
        let method = class.method(name, descriptor).expect("method");
        assert_eq!(name, method.name());
        assert_eq!(
            "([Ljava/lang/Object;)Ljava/lang/Object;",
            method.descriptor()
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_method_not_found() -> Result<()> {
        let class = string_class().await?;
        let method = class.method("foo", "()V");
        assert!(method.is_none());
        Ok(())
    }

    #[tokio::test]
    async fn test_try_get_method() -> Result<()> {
        let class = string_class().await?;
        let name = "isEmpty";
        let descriptor = "()Z";
        let method = class.try_get_method(name, descriptor)?;
        assert_eq!(name, method.name());
        assert_eq!(descriptor, method.descriptor());
        Ok(())
    }

    #[tokio::test]
    async fn test_try_get_method_not_found() -> Result<()> {
        let class = string_class().await?;
        let name = "foo";
        let descriptor = "()V";
        let result = class.try_get_method(name, descriptor);
        assert!(matches!(
            result,
            Err(MethodNotFound {
                class_name,
                method_name,
                method_descriptor,
            }) if class.name() == class_name
                && method_name == "foo"
                && method_descriptor == "()V"
        ));
        Ok(())
    }

    #[tokio::test]
    async fn test_object_not_is_subclass_of_object() -> Result<()> {
        let object_class = object_class().await?;
        assert!(!object_class.is_subclass_of(&object_class)?);
        Ok(())
    }

    #[tokio::test]
    async fn test_string_is_subclass_of_object() -> Result<()> {
        let string_class = string_class().await?;
        let object_class = object_class().await?;
        assert!(string_class.is_subclass_of(&object_class)?);
        Ok(())
    }

    #[tokio::test]
    async fn test_object_not_subclass_of_string() -> Result<()> {
        let string_class = string_class().await?;
        let object_class = object_class().await?;
        assert!(!object_class.is_subclass_of(&string_class)?);
        Ok(())
    }

    #[tokio::test]
    async fn test_object_is_assignable_from_object() -> Result<()> {
        let object_class = object_class().await?;
        assert!(object_class.is_assignable_from(&object_class)?);
        Ok(())
    }

    #[tokio::test]
    async fn test_object_is_assignable_from_string() -> Result<()> {
        let string_class = string_class().await?;
        let object_class = object_class().await?;
        assert!(object_class.is_assignable_from(&string_class)?);
        Ok(())
    }

    #[tokio::test]
    async fn test_string_not_is_assignable_from_object() -> Result<()> {
        let string_class = string_class().await?;
        let object_class = object_class().await?;
        assert!(!string_class.is_assignable_from(&object_class)?);
        Ok(())
    }

    #[tokio::test]
    async fn test_string_not_is_assignable_from_serializable() -> Result<()> {
        let string_class = string_class().await?;
        let serializable_class = serializable_class().await?;
        assert!(!string_class.is_assignable_from(&serializable_class)?);
        Ok(())
    }

    #[tokio::test]
    async fn test_object_is_assignable_from_serializable_class() -> Result<()> {
        let object_class = object_class().await?;
        let serializable_class = serializable_class().await?;
        assert!(object_class.is_assignable_from(&serializable_class)?);
        Ok(())
    }

    #[tokio::test]
    async fn test_object_array_not_is_assignable_from_object() -> Result<()> {
        let object_array_class = Class::new_named("[Ljava/lang/Object;")?;
        let object_class = object_class().await?;
        assert!(!object_array_class.is_assignable_from(&object_class)?);
        Ok(())
    }

    #[tokio::test]
    async fn test_object_is_assignable_from_int_array() -> Result<()> {
        let object_class = object_class().await?;
        let int_array_class = Class::new_named("[I")?;
        assert!(object_class.is_assignable_from(&int_array_class)?);
        Ok(())
    }

    #[tokio::test]
    async fn test_int_array_is_assignable_from_int_array() -> Result<()> {
        let int_array_class = Class::new_named("[I")?;
        assert!(int_array_class.is_assignable_from(&int_array_class)?);
        Ok(())
    }

    #[tokio::test]
    async fn test_object_is_assignable_from_object_array() -> Result<()> {
        let object_class = object_class().await?;
        let object_array_class = Class::new_named("[Ljava/lang/Object;")?;
        assert!(object_class.is_assignable_from(&object_array_class)?);
        Ok(())
    }

    #[tokio::test]
    async fn test_object_array_is_assignable_from_object_array() -> Result<()> {
        let object_array_class = Class::new_named("[Ljava/lang/Object;")?;
        assert!(object_array_class.is_assignable_from(&object_array_class)?);
        Ok(())
    }

    #[tokio::test]
    async fn test_object_array_not_is_assignable_from_int_array() -> Result<()> {
        let object_array_class = Class::new_named("[Ljava/lang/Object;")?;
        let int_array_class = Class::new_named("[I")?;
        assert!(!object_array_class.is_assignable_from(&int_array_class)?);
        Ok(())
    }

    #[tokio::test]
    async fn test_int_array_not_is_assignable_from_object_array() -> Result<()> {
        let int_array_class = Class::new_named("[I")?;
        let object_array_class = Class::new_named("[Ljava/lang/Object;")?;
        assert!(!int_array_class.is_assignable_from(&object_array_class)?);
        Ok(())
    }

    #[tokio::test]
    async fn test_object_array_is_assignable_from_string_array() -> Result<()> {
        let object_array_class = Class::new_named("[Ljava/lang/Object;")?;
        let string_array_class = Class::new_named("[Ljava/lang/String;")?;
        assert!(object_array_class.is_assignable_from(&string_array_class)?);
        Ok(())
    }

    #[tokio::test]
    async fn test_string_array_not_is_assignable_from_object_array() -> Result<()> {
        let string_array_class = Class::new_named("[Ljava/lang/String;")?;
        let object_array_class = Class::new_named("[Ljava/lang/Object;")?;
        assert!(!string_array_class.is_assignable_from(&object_array_class)?);
        Ok(())
    }

    #[tokio::test]
    async fn test_object_array_is_assignable_from_multiple_dimension_object_array() -> Result<()> {
        let object_array_class = Class::new_named("[Ljava/lang/Object;")?;
        let two_dimension_object_array_class = Class::new_named("[[Ljava/lang/Object;")?;
        assert!(object_array_class.is_assignable_from(&two_dimension_object_array_class)?);
        Ok(())
    }

    #[tokio::test]
    async fn test_multiple_dimension_object_array_not_is_assignable_from_object_array() -> Result<()>
    {
        let two_dimension_object_array_class = Class::new_named("[[Ljava/lang/Object;")?;
        let object_array_class = Class::new_named("[Ljava/lang/Object;")?;
        assert!(!two_dimension_object_array_class.is_assignable_from(&object_array_class)?);
        Ok(())
    }

    #[tokio::test]
    async fn test_to_string() -> Result<()> {
        let class = string_class().await?;
        assert_eq!("java/lang/String", class.to_string());
        Ok(())
    }
}
