use crate::Error::InvalidInstructionOffset;
use crate::attributes::bootstrap_method::BootstrapMethod;
use crate::attributes::inner_class::InnerClass;
use crate::attributes::line_number::LineNumber;
use crate::attributes::offset_utils;
use crate::attributes::parameter_annotation::ParameterAnnotation;
use crate::attributes::{
    Annotation, AnnotationElement, ExceptionTableEntry, Exports, Instruction, LocalVariableTable,
    LocalVariableTypeTable, MethodParameter, ModuleAccessFlags, Opens, Provides, Record, Requires,
    StackFrame, TypeAnnotation,
};
use crate::constant::Constant;
use crate::constant_pool::ConstantPool;
use crate::display::indent_lines;
use crate::error::Error::{InvalidAttributeLength, InvalidAttributeNameIndex};
use crate::error::Result;
use crate::version::Version;
use crate::{JAVA_5, JAVA_6, JAVA_7, JAVA_8, JAVA_9, JAVA_11, JAVA_16, JAVA_17, mutf8};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::collections::HashMap;
use std::fmt;
use std::io::{Cursor, Read};

const VERSION_45_3: Version = Version::Java1_0_2 { minor: 3 };
const VERSION_49_0: Version = JAVA_5;
const VERSION_50_0: Version = JAVA_6;
const VERSION_51_0: Version = JAVA_7;
const VERSION_52_0: Version = JAVA_8;
const VERSION_53_0: Version = JAVA_9;
const VERSION_55_0: Version = JAVA_11;
const VERSION_60_0: Version = JAVA_16;
const VERSION_61_0: Version = JAVA_17;

/// Represents a class file attribute as defined in the Java Virtual Machine Specification.
///
/// Attributes are used to provide additional information about class files, fields, methods, and code.
/// Each attribute has a name and specific data related to its purpose. The JVM specification defines
/// standard attributes, but custom attributes can also be created.
///
/// # Examples
///
/// Creating a `SourceFile` attribute:
///
/// ```
/// use ristretto_classfile::attributes::Attribute;
/// use ristretto_classfile::ConstantPool;
///
/// // Assuming we have a constant pool with a UTF8 entry for "SourceFile" at index 1
/// // and a UTF8 entry for the source file name at index 2
/// let source_file_attr = Attribute::SourceFile {
///     name_index: 1,
///     source_file_index: 2,
/// };
/// ```
///
/// Serializing an attribute to bytes:
///
/// ```
/// use ristretto_classfile::attributes::Attribute;
/// use ristretto_classfile::Result;
///
/// let source_file_attr = Attribute::SourceFile {
///     name_index: 1,
///     source_file_index: 2,
/// };
///
/// let mut bytes = Vec::new();
/// source_file_attr.to_bytes(&mut bytes)?;
/// # Ok::<(), ristretto_classfile::Error>(())
/// ```
///
/// # References
///
/// - [JVMS §4.7](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7)
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Attribute {
    /// Represents a constant value for a field.
    ///
    /// This attribute is used for fields that have a constant value. The `constant_value_index`
    /// points to the constant pool entry containing the value.
    ///
    /// # References
    ///
    /// - [JVMS §4.7.2](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.2)
    ConstantValue {
        name_index: u16,
        constant_value_index: u16,
    },

    /// Contains the bytecode and auxiliary information for a method implementation.
    ///
    /// The Code attribute contains the instructions, exception handlers, and additional attributes
    /// needed to execute a method. It also specifies the maximum stack size and local variable
    /// count.
    ///
    /// # References
    ///
    /// - [JVMS §4.7.3](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.3)
    Code {
        name_index: u16,
        max_stack: u16,
        max_locals: u16,
        code: Vec<Instruction>,
        exception_table: Vec<ExceptionTableEntry>,
        attributes: Vec<Attribute>,
    },

    /// Represents a stack map table for type checking during bytecode verification.
    ///
    /// The `StackMapTable` attribute is used by the Java Virtual Machine's bytecode verifier to
    /// type check code without requiring the loading of referenced classes. It contains information
    /// about the state of the operand stack and local variables at specific offsets in the code.
    ///
    /// # References
    ///
    /// - [JVMS §4.7.4](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.4)
    StackMapTable {
        name_index: u16,
        frames: Vec<StackFrame>,
    },

    /// Lists the checked exceptions that a method may throw.
    ///
    /// The Exceptions attribute indicates which checked exceptions a method can throw. Each entry
    /// in the `exception_indexes` list points to a `CONSTANT_Class_info` structure representing a
    /// class type that this method is declared to throw.
    ///
    /// # References
    ///
    /// - [JVMS §4.7.5](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.5)
    Exceptions {
        name_index: u16,
        exception_indexes: Vec<u16>,
    },

    /// Records the inner classes and interfaces of a class or interface.
    ///
    /// This attribute provides information about the inner classes and interfaces declared within a
    /// class. For each inner class or interface, it specifies the class name, enclosing class,
    /// inner name, and access flags.
    ///
    /// # References
    ///
    /// - [JVMS §4.7.6](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.6)
    InnerClasses {
        name_index: u16,
        classes: Vec<InnerClass>,
    },

    /// Indicates that a class is a local or anonymous class.
    ///
    /// The `EnclosingMethod` attribute provides information about the enclosing context of a local
    /// or anonymous class. It identifies the class within which the local or anonymous class is
    /// declared, and may optionally specify the method within that class.
    ///
    /// # References
    ///
    /// - [JVMS §4.7.7](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.7)
    EnclosingMethod {
        name_index: u16,
        class_index: u16,
        method_index: u16,
    },

    /// Indicates that a class, field, or method was generated by the compiler and does not appear
    /// in source code.
    ///
    /// The `Synthetic` attribute marks a class member that does not have a corresponding construct
    /// in the source code. It is used to denote members that were introduced by the compiler during
    /// compilation.
    ///
    /// # References
    ///
    /// - [JVMS §4.7.8](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.8)
    Synthetic { name_index: u16 },

    /// Stores generic signature information for a class, field, or method.
    ///
    /// The Signature attribute records generic signature information for a class, interface,
    /// constructor, method, or field declaration when that signature includes type variables or
    /// parameterized types.
    ///
    /// # References
    ///
    /// - [JVMS §4.7.9](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.9)
    Signature {
        name_index: u16,
        signature_index: u16,
    },

    /// Indicates the source file from which a class file was compiled.
    ///
    /// The `SourceFile` attribute points to a `CONSTANT_Utf8_info` structure in the constant pool
    /// that contains the name of the source file from which this class file was compiled.
    ///
    /// # References
    ///
    /// - [JVMS §4.7.10](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.10)
    SourceFile {
        name_index: u16,
        source_file_index: u16,
    },

    /// Provides additional debugging information not included in the standard source file.
    ///
    /// The `SourceDebugExtension` attribute is an optional attribute that contains additional
    /// debugging information which tools can use to implement source-level debugging. The attribute
    /// typically stores information for non-Java source files.
    ///
    /// # References
    ///
    /// - [JVMS §4.7.11](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.11)
    SourceDebugExtension {
        name_index: u16,
        debug_extension: String,
    },

    /// Maps bytecode instruction offsets to source code line numbers.
    ///
    /// The `LineNumberTable` attribute maps bytecode instruction offsets to line numbers in the
    /// original source file. This attribute is used by debuggers to determine which line of source
    /// is being executed and by exception handlers to display line numbers.
    ///
    /// # References
    ///
    /// - [JVMS §4.7.12](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.12)
    LineNumberTable {
        name_index: u16,
        line_numbers: Vec<LineNumber>,
    },

    /// Maps ranges of bytecode to information about local variables.
    ///
    /// The `LocalVariableTable` attribute records information about the local variables in a
    /// method, allowing debuggers to determine the value of a given local variable during
    /// execution. Each entry maps a range of bytecode to a specific local variable.
    ///
    /// # References
    ///
    /// - [JVMS §4.7.13](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.13)
    LocalVariableTable {
        name_index: u16,
        variables: Vec<LocalVariableTable>,
    },

    /// Provides type information for generic local variables.
    ///
    /// The `LocalVariableTypeTable` attribute records signature information for local variables in
    /// generic code, allowing debuggers to display and interact with generic types. It complements
    /// the `LocalVariableTable` by providing generic type information.
    ///
    /// # References
    ///
    /// - [JVMS §4.7.14](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.14)
    LocalVariableTypeTable {
        name_index: u16,
        variable_types: Vec<LocalVariableTypeTable>,
    },

    /// Indicates that a class, field, or method is deprecated.
    ///
    /// The Deprecated attribute indicates that a class, interface, method, or field is deprecated
    /// and should no longer be used. It corresponds to the @Deprecated annotation in Java source
    /// code.
    ///
    /// # References
    ///
    /// - [JVMS §4.7.15](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.15)
    Deprecated { name_index: u16 },

    /// Stores annotations that are visible at runtime.
    ///
    /// The `RuntimeVisibleAnnotations` attribute records the annotations on a program element that
    /// are visible to the reflection API at runtime. These correspond to annotations without a
    /// `RetentionPolicy.SOURCE` or `RetentionPolicy.CLASS`.
    ///
    /// # References
    ///
    /// - [JVMS §4.7.16](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.16)
    RuntimeVisibleAnnotations {
        name_index: u16,
        annotations: Vec<Annotation>,
    },

    /// Stores annotations that are not visible at runtime.
    ///
    /// The `RuntimeInvisibleAnnotations` attribute records the annotations on a program element
    /// that are not visible to the reflection API at runtime. These correspond to annotations with
    /// `RetentionPolicy.CLASS`.
    ///
    /// # References
    ///
    /// - [JVMS §4.7.17](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.17)
    RuntimeInvisibleAnnotations {
        name_index: u16,
        annotations: Vec<Annotation>,
    },

    /// Stores runtime-visible annotations on method parameters.
    ///
    /// The `RuntimeVisibleParameterAnnotations` attribute records annotations on method parameters
    /// that are visible to the reflection API at runtime. Each parameter can have multiple
    /// annotations.
    ///
    /// # References
    ///
    /// - [JVMS §4.7.18](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.18)
    RuntimeVisibleParameterAnnotations {
        name_index: u16,
        parameter_annotations: Vec<ParameterAnnotation>,
    },

    /// Stores runtime-invisible annotations on method parameters.
    ///
    /// The `RuntimeInvisibleParameterAnnotations` attribute records annotations on method
    /// parameters that are not visible to the reflection API at runtime. These correspond to
    /// annotations with `RetentionPolicy.CLASS`.
    ///
    /// # References
    ///
    /// - [JVMS §4.7.19](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.19)
    RuntimeInvisibleParameterAnnotations {
        name_index: u16,
        parameter_annotations: Vec<ParameterAnnotation>,
    },

    /// Stores runtime-visible type annotations.
    ///
    /// The `RuntimeVisibleTypeAnnotations` attribute records type annotations that are visible to
    /// the reflection API at runtime. Type annotations can target a wider range of program elements
    /// than traditional annotations, including type uses and type declarations.
    ///
    /// # References
    ///
    /// - [JVMS §4.7.20](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.20)
    RuntimeVisibleTypeAnnotations {
        name_index: u16,
        type_annotations: Vec<TypeAnnotation>,
    },

    /// Stores runtime-invisible type annotations.
    ///
    /// The `RuntimeInvisibleTypeAnnotations` attribute records type annotations that are not
    /// visible to the reflection API at runtime. These correspond to type annotations with
    /// `RetentionPolicy.CLASS`.
    ///
    /// # References
    ///
    /// - [JVMS §4.7.21](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.21)
    RuntimeInvisibleTypeAnnotations {
        name_index: u16,
        type_annotations: Vec<TypeAnnotation>,
    },

    /// Specifies the default value for an annotation type element.
    ///
    /// The `AnnotationDefault` attribute defines the default value for an element in an annotation
    /// type. It appears in methods of annotation interfaces that provide default values.
    ///
    /// # References
    ///
    /// - [JVMS §4.7.22](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.22)
    AnnotationDefault {
        name_index: u16,
        element: AnnotationElement,
    },

    /// Stores bootstrap method references used by invokedynamic instructions.
    ///
    /// The `BootstrapMethods` attribute records bootstrap methods referenced by invokedynamic
    /// instructions. Each method entry contains a reference to the bootstrap method and its
    /// static arguments, which are used for dynamic method invocation.
    ///
    /// # References
    ///
    /// - [JVMS §4.7.23](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.23)
    BootstrapMethods {
        name_index: u16,
        methods: Vec<BootstrapMethod>,
    },

    /// Provides information about method parameters, including names and access flags.
    ///
    /// The `MethodParameters` attribute records information about the formal parameters of a
    /// method, including names and access flags. This allows for reflection on method parameter
    /// names and modifiers (such as final or synthetic parameters).
    ///
    /// # References
    ///
    /// - [JVMS §4.7.24](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.24)
    MethodParameters {
        name_index: u16,
        parameters: Vec<MethodParameter>,
    },

    /// Defines a module, its dependencies, exports, and services.
    ///
    /// The Module attribute describes a module, including its name, requirements (dependencies),
    /// exports, opens, uses, and provides declarations. It appears in module-info class files
    /// and is part of the Java Platform Module System.
    ///
    /// # References
    ///
    /// - [JVMS §4.7.25](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.25)
    Module {
        name_index: u16,
        module_name_index: u16,
        flags: ModuleAccessFlags,
        version_index: u16,
        requires: Vec<Requires>,
        exports: Vec<Exports>,
        opens: Vec<Opens>,
        uses: Vec<u16>,
        provides: Vec<Provides>,
    },

    /// Lists all packages exported or opened by a module.
    ///
    /// The `ModulePackages` attribute records all packages that are exported or opened by a module.
    /// This information is used by various tools and APIs that need to know the complete set of
    /// packages belonging to a module.
    ///
    /// # References
    ///
    /// - [JVMS §4.7.26](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.26)
    ModulePackages {
        name_index: u16,
        package_indexes: Vec<u16>,
    },

    /// Specifies the main class of a module.
    ///
    /// The `ModuleMainClass` attribute indicates the main class of a module, which is the class
    /// containing the main method that should be executed when the module is run directly.
    ///
    /// # References
    ///
    /// - [JVMS §4.7.27](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.27)
    ModuleMainClass {
        name_index: u16,
        main_class_index: u16,
    },

    /// Identifies the host class of a nested class.
    ///
    /// The `NestHost` attribute records the top-level class that serves as the nest host for a nest
    /// member class. This attribute helps implement the new nesting-based access control in Java,
    /// which allows nested classes to access each other's private members.
    ///
    /// # References
    ///
    /// - [JVMS §4.7.28](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.28)
    NestHost {
        name_index: u16,
        host_class_index: u16,
    },

    /// Lists the member classes of a nest.
    ///
    /// The `NestMembers` attribute appears in the nest host class and records all the classes that
    /// are members of the nest. This attribute works with `NestHost` to implement nesting-based
    /// access control in Java.
    ///
    /// # References
    ///
    /// - [JVMS §4.7.29](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.29)
    NestMembers {
        name_index: u16,
        class_indexes: Vec<u16>,
    },

    /// Describes the components of a record class.
    ///
    /// The Record attribute stores information about the components of a record class, including
    /// their names, descriptors, and attributes. This attribute is used to implement the record
    /// feature introduced in Java 16.
    ///
    /// # References
    ///
    /// - [JVMS §4.7.30](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.30)
    Record {
        name_index: u16,
        records: Vec<Record>,
    },

    /// Lists the permitted direct subclasses of a sealed class.
    ///
    /// The `PermittedSubclasses` attribute records the classes that are permitted to extend a
    /// sealed class or implement a sealed interface. It is used to implement the sealed classes
    /// feature introduced in Java 17.
    ///
    /// # References
    ///
    /// - [JVMS §4.7.31](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.31)
    PermittedSubclasses {
        name_index: u16,
        class_indexes: Vec<u16>,
    },

    /// Used to support reading future classes where the structure is not known beforehand.
    ///
    /// This variant allows the parser to handle unknown attribute types gracefully by storing the
    /// raw bytes of the attribute.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::Attribute;
    ///
    /// let unknown_attr = Attribute::Unknown {
    ///     name_index: 1,  // Index in constant pool pointing to the attribute name
    ///     info: vec![0, 1, 2, 3],  // Raw attribute data
    /// };
    /// ```
    Unknown { name_index: u16, info: Vec<u8> },
}

impl Attribute {
    /// Returns the name of the Attribute as a static string.
    ///
    /// This method returns the standard name of the attribute type
    /// regardless of the actual name used in the class file.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::Attribute;
    ///
    /// let attr = Attribute::SourceFile {
    ///     name_index: 1,
    ///     source_file_index: 2,
    /// };
    ///
    /// assert_eq!(attr.name(), "SourceFile");
    /// ```
    #[must_use]
    pub fn name(&self) -> &'static str {
        match self {
            Attribute::ConstantValue { .. } => "ConstantValue",
            Attribute::Code { .. } => "Code",
            Attribute::StackMapTable { .. } => "StackMapTable",
            Attribute::Exceptions { .. } => "Exceptions",
            Attribute::InnerClasses { .. } => "InnerClasses",
            Attribute::EnclosingMethod { .. } => "EnclosingMethod",
            Attribute::Synthetic { .. } => "Synthetic",
            Attribute::Signature { .. } => "Signature",
            Attribute::SourceFile { .. } => "SourceFile",
            Attribute::SourceDebugExtension { .. } => "SourceDebugExtension",
            Attribute::LineNumberTable { .. } => "LineNumberTable",
            Attribute::LocalVariableTable { .. } => "LocalVariableTable",
            Attribute::LocalVariableTypeTable { .. } => "LocalVariableTypeTable",
            Attribute::Deprecated { .. } => "Deprecated",
            Attribute::RuntimeVisibleAnnotations { .. } => "RuntimeVisibleAnnotations",
            Attribute::RuntimeInvisibleAnnotations { .. } => "RuntimeInvisibleAnnotations",
            Attribute::RuntimeVisibleParameterAnnotations { .. } => {
                "RuntimeVisibleParameterAnnotations"
            }
            Attribute::RuntimeInvisibleParameterAnnotations { .. } => {
                "RuntimeInvisibleParameterAnnotations"
            }
            Attribute::RuntimeVisibleTypeAnnotations { .. } => "RuntimeVisibleTypeAnnotations",
            Attribute::RuntimeInvisibleTypeAnnotations { .. } => "RuntimeInvisibleTypeAnnotations",
            Attribute::AnnotationDefault { .. } => "AnnotationDefault",
            Attribute::BootstrapMethods { .. } => "BootstrapMethods",
            Attribute::MethodParameters { .. } => "MethodParameters",
            Attribute::Module { .. } => "Module",
            Attribute::ModulePackages { .. } => "ModulePackages",
            Attribute::ModuleMainClass { .. } => "ModuleMainClass",
            Attribute::NestHost { .. } => "NestHost",
            Attribute::NestMembers { .. } => "NestMembers",
            Attribute::Record { .. } => "Record",
            Attribute::PermittedSubclasses { .. } => "PermittedSubclasses",
            Attribute::Unknown { .. } => "Unknown",
        }
    }

    /// Checks if the Attribute is valid for the given Java version.
    ///
    /// Each attribute type was introduced in a specific Java version. This method checks if the
    /// attribute is supported in the specified version.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::Attribute;
    /// use ristretto_classfile::{Version, JAVA_8};
    ///
    /// let attr = Attribute::SourceFile {
    ///     name_index: 1,
    ///     source_file_index: 2,
    /// };
    ///
    /// // SourceFile was introduced early and is supported in Java 8
    /// assert!(attr.valid_for_version(&JAVA_8));
    ///
    /// // NestMembers was introduced in Java 11 and wouldn't be valid in Java 8
    /// let nest_members = Attribute::NestMembers {
    ///     name_index: 1,
    ///     class_indexes: vec![2],
    /// };
    /// assert!(!nest_members.valid_for_version(&JAVA_8));
    /// ```
    #[expect(clippy::match_same_arms)]
    #[must_use]
    pub fn valid_for_version(&self, version: &Version) -> bool {
        match self {
            Attribute::ConstantValue { .. } => *version >= VERSION_45_3,
            Attribute::Code { .. } => *version >= VERSION_45_3,
            Attribute::StackMapTable { .. } => *version >= VERSION_50_0,
            Attribute::Exceptions { .. } => *version >= VERSION_45_3,
            Attribute::InnerClasses { .. } => *version >= VERSION_45_3,
            Attribute::EnclosingMethod { .. } => *version >= VERSION_49_0,
            Attribute::Synthetic { .. } => *version >= VERSION_45_3,
            Attribute::Signature { .. } => *version >= VERSION_49_0,
            Attribute::SourceFile { .. } => *version >= VERSION_45_3,
            Attribute::SourceDebugExtension { .. } => *version >= VERSION_49_0,
            Attribute::LineNumberTable { .. } => *version >= VERSION_45_3,
            Attribute::LocalVariableTable { .. } => *version >= VERSION_49_0,
            Attribute::LocalVariableTypeTable { .. } => *version >= VERSION_45_3,
            Attribute::Deprecated { .. } => *version >= VERSION_45_3,
            Attribute::RuntimeVisibleAnnotations { .. } => *version >= VERSION_49_0,
            Attribute::RuntimeInvisibleAnnotations { .. } => *version >= VERSION_49_0,
            Attribute::RuntimeVisibleParameterAnnotations { .. } => *version >= VERSION_49_0,
            Attribute::RuntimeInvisibleParameterAnnotations { .. } => *version >= VERSION_49_0,
            Attribute::RuntimeVisibleTypeAnnotations { .. } => *version >= VERSION_52_0,
            Attribute::RuntimeInvisibleTypeAnnotations { .. } => *version >= VERSION_52_0,
            Attribute::AnnotationDefault { .. } => *version >= VERSION_49_0,
            Attribute::BootstrapMethods { .. } => *version >= VERSION_51_0,
            Attribute::MethodParameters { .. } => *version >= VERSION_52_0,
            Attribute::Module { .. } => *version >= VERSION_53_0,
            Attribute::ModulePackages { .. } => *version >= VERSION_53_0,
            Attribute::ModuleMainClass { .. } => *version >= VERSION_53_0,
            Attribute::NestHost { .. } => *version >= VERSION_55_0,
            Attribute::NestMembers { .. } => *version >= VERSION_55_0,
            Attribute::Record { .. } => *version >= VERSION_60_0,
            Attribute::PermittedSubclasses { .. } => *version >= VERSION_61_0,
            Attribute::Unknown { .. } => *version >= VERSION_45_3,
        }
    }

    /// Deserializes an Attribute from bytes.
    ///
    /// This method reads an attribute from the provided bytes cursor, using the constant pool to
    /// resolve attribute names.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::Attribute;
    /// use ristretto_classfile::ConstantPool;
    /// use std::io::Cursor;
    /// use byteorder::{BigEndian, WriteBytesExt};
    ///
    /// // Create a constant pool with the necessary entries
    /// let mut constant_pool = ConstantPool::new();
    /// let name_index = constant_pool.add_utf8("SourceFile")?;
    /// let source_file_index = constant_pool.add_utf8("MyClass.java")?;
    ///
    /// // Create bytes representing a SourceFile attribute
    /// let mut bytes = Vec::new();
    /// bytes.write_u16::<BigEndian>(name_index)?;        // name_index
    /// bytes.write_u32::<BigEndian>(2)?;                 // attribute_length
    /// bytes.write_u16::<BigEndian>(source_file_index)?; // source_file_index
    ///
    /// // Deserialize the attribute
    /// let mut cursor = Cursor::new(bytes);
    /// let attribute = Attribute::from_bytes(&constant_pool, &mut cursor)?;
    ///
    /// // Verify the deserialized attribute
    /// if let Attribute::SourceFile { name_index: attr_name_idx, source_file_index: attr_source_idx } = attribute {
    ///     assert_eq!(attr_name_idx, name_index);
    ///     assert_eq!(attr_source_idx, source_file_index);
    /// } else {
    ///     panic!("Expected SourceFile attribute");
    /// }
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    ///
    /// # Errors
    ///
    /// - Returns `InvalidAttributeNameIndex` if the name index is invalid in the constant pool.
    /// - Returns `InvalidAttributeLength` if the attribute length doesn't match the expected length.
    /// - Returns other errors if deserialization of specific attribute types fails.
    #[expect(clippy::too_many_lines)]
    pub fn from_bytes(
        constant_pool: &ConstantPool,
        bytes: &mut Cursor<Vec<u8>>,
    ) -> Result<Attribute> {
        let name_index = bytes.read_u16::<BigEndian>()?;
        let Some(Constant::Utf8(attribute_name)) = constant_pool.get(name_index) else {
            return Err(InvalidAttributeNameIndex(name_index));
        };

        let info_length = bytes.read_u32::<BigEndian>()?;

        let attribute = match attribute_name.as_str() {
            "ConstantValue" => {
                if info_length != 2 {
                    return Err(InvalidAttributeLength(info_length));
                }
                Attribute::ConstantValue {
                    name_index,
                    constant_value_index: bytes.read_u16::<BigEndian>()?,
                }
            }
            "Code" => {
                // Instruction pointers are converted from byte offsets to instruction offsets to
                // facilitate faster / easier instruction manipulation at runtime.  During runtime,
                // the instruction offset can be used directly and calculating the next instruction
                // byte offset is unnecessary. This separates the physical storage of the
                // instructions from the logical representation.
                let max_stack = bytes.read_u16::<BigEndian>()?;
                let max_locals = bytes.read_u16::<BigEndian>()?;

                let code_length = bytes.read_u32::<BigEndian>()?;
                let mut code = vec![0; code_length as usize];
                bytes.read_exact(&mut code)?;
                let (byte_to_instruction_map, instructions) =
                    offset_utils::instructions_from_bytes(&mut Cursor::new(code))?;

                let exception_length = bytes.read_u16::<BigEndian>()?;
                let mut exception_table = Vec::with_capacity(exception_length as usize);
                for _ in 0..exception_length {
                    let mut exception = ExceptionTableEntry::from_bytes(bytes)?;
                    exception.range_pc.start = *byte_to_instruction_map
                        .get(&exception.range_pc.start)
                        .ok_or(InvalidInstructionOffset(u32::from(
                            exception.range_pc.start,
                        )))?;
                    exception.range_pc.end = byte_to_instruction_map
                        .iter()
                        .filter(|&(&k, _)| k <= exception.range_pc.end)
                        .max_by_key(|&(&k, _)| k)
                        .map(|(_, &v)| v)
                        .ok_or(InvalidInstructionOffset(u32::from(exception.range_pc.end)))?;
                    exception.handler_pc = *byte_to_instruction_map
                        .get(&exception.handler_pc)
                        .ok_or(InvalidInstructionOffset(u32::from(exception.handler_pc)))?;
                    exception_table.push(exception);
                }
                let attributes = Self::from_bytes_code_attributes(
                    constant_pool,
                    bytes,
                    &byte_to_instruction_map,
                )?;
                Attribute::Code {
                    name_index,
                    max_stack,
                    max_locals,
                    code: instructions,
                    exception_table,
                    attributes,
                }
            }
            "StackMapTable" => {
                let mut info = vec![0; info_length as usize];
                bytes.clone().read_exact(&mut info)?;
                let frames_count = bytes.read_u16::<BigEndian>()?;
                let mut frames = Vec::with_capacity(frames_count as usize);
                for _ in 0..frames_count {
                    let stack_frame = StackFrame::from_bytes(bytes)?;
                    frames.push(stack_frame);
                }
                Attribute::StackMapTable { name_index, frames }
            }
            "Exceptions" => {
                let exception_indexes_count = bytes.read_u16::<BigEndian>()?;
                let mut exception_indexes = Vec::with_capacity(exception_indexes_count as usize);
                for _ in 0..exception_indexes_count {
                    exception_indexes.push(bytes.read_u16::<BigEndian>()?);
                }
                Attribute::Exceptions {
                    name_index,
                    exception_indexes,
                }
            }
            "InnerClasses" => {
                let classes_count = bytes.read_u16::<BigEndian>()?;
                let mut classes = Vec::with_capacity(classes_count as usize);
                for _ in 0..classes_count {
                    let inner_class = InnerClass::from_bytes(bytes)?;
                    classes.push(inner_class);
                }
                Attribute::InnerClasses {
                    name_index,
                    classes,
                }
            }
            "EnclosingMethod" => {
                if info_length != 4 {
                    return Err(InvalidAttributeLength(info_length));
                }
                Attribute::EnclosingMethod {
                    name_index,
                    class_index: bytes.read_u16::<BigEndian>()?,
                    method_index: bytes.read_u16::<BigEndian>()?,
                }
            }
            "Synthetic" => {
                if info_length != 0 {
                    return Err(InvalidAttributeLength(info_length));
                }
                Attribute::Synthetic { name_index }
            }
            "Signature" => {
                if info_length != 2 {
                    return Err(InvalidAttributeLength(info_length));
                }
                Attribute::Signature {
                    name_index,
                    signature_index: bytes.read_u16::<BigEndian>()?,
                }
            }
            "SourceFile" => {
                if info_length != 2 {
                    return Err(InvalidAttributeLength(info_length));
                }
                Attribute::SourceFile {
                    name_index,
                    source_file_index: bytes.read_u16::<BigEndian>()?,
                }
            }
            "SourceDebugExtension" => {
                let mut debug_extension_bytes = vec![0; info_length as usize];
                bytes.read_exact(&mut debug_extension_bytes)?;
                let debug_extension = mutf8::from_bytes(debug_extension_bytes.as_slice())?;
                Attribute::SourceDebugExtension {
                    name_index,
                    debug_extension,
                }
            }
            "LineNumberTable" => {
                let line_number_table_count = bytes.read_u16::<BigEndian>()?;
                let mut line_numbers = Vec::with_capacity(line_number_table_count as usize);
                for _ in 0..line_number_table_count {
                    line_numbers.push(LineNumber::from_bytes(bytes)?);
                }
                Attribute::LineNumberTable {
                    name_index,
                    line_numbers,
                }
            }
            "LocalVariableTable" => {
                let variables_count = bytes.read_u16::<BigEndian>()?;
                let mut variables = Vec::with_capacity(variables_count as usize);
                for _ in 0..variables_count {
                    variables.push(LocalVariableTable::from_bytes(bytes)?);
                }
                Attribute::LocalVariableTable {
                    name_index,
                    variables,
                }
            }
            "LocalVariableTypeTable" => {
                let variable_types_count = bytes.read_u16::<BigEndian>()?;
                let mut variable_types = Vec::with_capacity(variable_types_count as usize);
                for _ in 0..variable_types_count {
                    variable_types.push(LocalVariableTypeTable::from_bytes(bytes)?);
                }
                Attribute::LocalVariableTypeTable {
                    name_index,
                    variable_types,
                }
            }
            "Deprecated" => {
                if info_length != 0 {
                    return Err(InvalidAttributeLength(info_length));
                }
                Attribute::Deprecated { name_index }
            }
            "RuntimeVisibleAnnotations" => {
                let annotations_count = bytes.read_u16::<BigEndian>()?;
                let mut annotations = Vec::with_capacity(annotations_count as usize);
                for _ in 0..annotations_count {
                    let annotation = Annotation::from_bytes(bytes)?;
                    annotations.push(annotation);
                }
                Attribute::RuntimeVisibleAnnotations {
                    name_index,
                    annotations,
                }
            }
            "RuntimeInvisibleAnnotations" => {
                let annotations_count = bytes.read_u16::<BigEndian>()?;
                let mut annotations = Vec::with_capacity(annotations_count as usize);
                for _ in 0..annotations_count {
                    let annotation = Annotation::from_bytes(bytes)?;
                    annotations.push(annotation);
                }
                Attribute::RuntimeInvisibleAnnotations {
                    name_index,
                    annotations,
                }
            }
            "RuntimeVisibleParameterAnnotations" => {
                let parameter_annotations_count = bytes.read_u8()?;
                let mut parameter_annotations =
                    Vec::with_capacity(parameter_annotations_count as usize);
                for _ in 0..parameter_annotations_count {
                    let parameter_annotation = ParameterAnnotation::from_bytes(bytes)?;
                    parameter_annotations.push(parameter_annotation);
                }
                Attribute::RuntimeVisibleParameterAnnotations {
                    name_index,
                    parameter_annotations,
                }
            }
            "RuntimeInvisibleParameterAnnotations" => {
                let parameter_annotations_count = bytes.read_u8()?;
                let mut parameter_annotations =
                    Vec::with_capacity(parameter_annotations_count as usize);
                for _ in 0..parameter_annotations_count {
                    let parameter_annotation = ParameterAnnotation::from_bytes(bytes)?;
                    parameter_annotations.push(parameter_annotation);
                }
                Attribute::RuntimeInvisibleParameterAnnotations {
                    name_index,
                    parameter_annotations,
                }
            }
            "RuntimeVisibleTypeAnnotations" => {
                let type_annotations_count = bytes.read_u16::<BigEndian>()?;
                let mut type_annotations = Vec::with_capacity(type_annotations_count as usize);
                for _ in 0..type_annotations_count {
                    let type_annotation = TypeAnnotation::from_bytes(bytes)?;
                    type_annotations.push(type_annotation);
                }
                Attribute::RuntimeVisibleTypeAnnotations {
                    name_index,
                    type_annotations,
                }
            }
            "RuntimeInvisibleTypeAnnotations" => {
                let type_annotations_count = bytes.read_u16::<BigEndian>()?;
                let mut type_annotations = Vec::with_capacity(type_annotations_count as usize);
                for _ in 0..type_annotations_count {
                    let type_annotation = TypeAnnotation::from_bytes(bytes)?;
                    type_annotations.push(type_annotation);
                }
                Attribute::RuntimeInvisibleTypeAnnotations {
                    name_index,
                    type_annotations,
                }
            }
            "AnnotationDefault" => {
                let element = AnnotationElement::from_bytes(bytes)?;
                Attribute::AnnotationDefault {
                    name_index,
                    element,
                }
            }
            "BootstrapMethods" => {
                let bootstrap_methods_count = bytes.read_u16::<BigEndian>()?;
                let mut methods = Vec::with_capacity(bootstrap_methods_count as usize);
                for _ in 0..bootstrap_methods_count {
                    let bootstrap_method = BootstrapMethod::from_bytes(bytes)?;
                    methods.push(bootstrap_method);
                }
                Attribute::BootstrapMethods {
                    name_index,
                    methods,
                }
            }
            "MethodParameters" => {
                let parameters_count = bytes.read_u8()?;
                let mut parameters = Vec::with_capacity(parameters_count as usize);
                for _ in 0..parameters_count {
                    let method_parameters = MethodParameter::from_bytes(bytes)?;
                    parameters.push(method_parameters);
                }
                Attribute::MethodParameters {
                    name_index,
                    parameters,
                }
            }
            "Module" => {
                let module_name_index = bytes.read_u16::<BigEndian>()?;
                let flags = ModuleAccessFlags::from_bytes(bytes)?;
                let version_index = bytes.read_u16::<BigEndian>()?;
                let requires_count = bytes.read_u16::<BigEndian>()?;
                let mut requires = Vec::with_capacity(requires_count as usize);
                for _ in 0..requires_count {
                    let require = Requires::from_bytes(bytes)?;
                    requires.push(require);
                }
                let exports_count = bytes.read_u16::<BigEndian>()?;
                let mut exports = Vec::with_capacity(exports_count as usize);
                for _ in 0..exports_count {
                    let export = Exports::from_bytes(bytes)?;
                    exports.push(export);
                }
                let opens_count = bytes.read_u16::<BigEndian>()?;
                let mut opens = Vec::with_capacity(opens_count as usize);
                for _ in 0..opens_count {
                    let open = Opens::from_bytes(bytes)?;
                    opens.push(open);
                }
                let uses_count = bytes.read_u16::<BigEndian>()?;
                let mut uses = Vec::with_capacity(uses_count as usize);
                for _ in 0..uses_count {
                    uses.push(bytes.read_u16::<BigEndian>()?);
                }
                let provides_count = bytes.read_u16::<BigEndian>()?;
                let mut provides = Vec::with_capacity(provides_count as usize);
                for _ in 0..provides_count {
                    let provide = Provides::from_bytes(bytes)?;
                    provides.push(provide);
                }
                Attribute::Module {
                    name_index,
                    module_name_index,
                    flags,
                    version_index,
                    requires,
                    exports,
                    opens,
                    uses,
                    provides,
                }
            }
            "ModulePackages" => {
                let package_indexes_count = bytes.read_u16::<BigEndian>()?;
                let mut package_indexes = Vec::with_capacity(package_indexes_count as usize);
                for _ in 0..package_indexes_count {
                    package_indexes.push(bytes.read_u16::<BigEndian>()?);
                }
                Attribute::ModulePackages {
                    name_index,
                    package_indexes,
                }
            }
            "ModuleMainClass" => {
                if info_length != 2 {
                    return Err(InvalidAttributeLength(info_length));
                }
                Attribute::ModuleMainClass {
                    name_index,
                    main_class_index: bytes.read_u16::<BigEndian>()?,
                }
            }
            "NestHost" => {
                if info_length != 2 {
                    return Err(InvalidAttributeLength(info_length));
                }
                Attribute::NestHost {
                    name_index,
                    host_class_index: bytes.read_u16::<BigEndian>()?,
                }
            }
            "NestMembers" => {
                let class_indexes_count = bytes.read_u16::<BigEndian>()?;
                let mut class_indexes = Vec::with_capacity(class_indexes_count as usize);
                for _ in 0..class_indexes_count {
                    class_indexes.push(bytes.read_u16::<BigEndian>()?);
                }
                Attribute::NestMembers {
                    name_index,
                    class_indexes,
                }
            }
            "Record" => {
                let record_count = bytes.read_u16::<BigEndian>()?;
                let mut records = Vec::with_capacity(record_count as usize);
                for _ in 0..record_count {
                    let record = Record::from_bytes(constant_pool, bytes)?;
                    records.push(record);
                }
                Attribute::Record {
                    name_index,
                    records,
                }
            }
            "PermittedSubclasses" => {
                let class_indexes_count = bytes.read_u16::<BigEndian>()?;
                let mut class_indexes = Vec::with_capacity(class_indexes_count as usize);
                for _ in 0..class_indexes_count {
                    class_indexes.push(bytes.read_u16::<BigEndian>()?);
                }
                Attribute::PermittedSubclasses {
                    name_index,
                    class_indexes,
                }
            }
            _ => {
                let mut info = vec![0; info_length as usize];
                bytes.read_exact(&mut info)?;
                Attribute::Unknown { name_index, info }
            }
        };
        Ok(attribute)
    }

    fn from_bytes_code_attributes(
        constant_pool: &ConstantPool,
        bytes: &mut Cursor<Vec<u8>>,
        byte_to_instruction_map: &HashMap<u16, u16>,
    ) -> Result<Vec<Attribute>> {
        let attributes_count = bytes.read_u16::<BigEndian>()?;
        let mut attributes = Vec::with_capacity(attributes_count as usize);
        for _ in 0..attributes_count {
            let attribute = Attribute::from_bytes(constant_pool, bytes)?;
            match attribute {
                Attribute::LineNumberTable {
                    name_index,
                    mut line_numbers,
                } => {
                    for line_number in &mut line_numbers {
                        line_number.start_pc = *byte_to_instruction_map
                            .get(&line_number.start_pc)
                            .ok_or(InvalidInstructionOffset(u32::from(line_number.start_pc)))?;
                    }
                    let attribute = Attribute::LineNumberTable {
                        name_index,
                        line_numbers,
                    };
                    attributes.push(attribute);
                }
                Attribute::StackMapTable {
                    name_index,
                    mut frames,
                } => {
                    let mut first_frame = true;
                    let mut last_byte_offset: u16 = 0;
                    let mut last_instruction_offset: u16 = 0;
                    for frame in &mut frames {
                        let offset_delta = frame.offset_delta();
                        let byte_offset = if first_frame {
                            offset_delta
                        } else {
                            last_byte_offset
                                .saturating_add(offset_delta)
                                .saturating_add(1)
                        };

                        let instruction_offset = *byte_to_instruction_map
                            .get(&byte_offset)
                            .ok_or(InvalidInstructionOffset(u32::from(byte_offset)))?;
                        // Calculate the instruction delta offset from the last instruction offset
                        // subtracting 1 to account for the current instruction.
                        let instruction_delta_offset = if first_frame {
                            first_frame = false;
                            instruction_offset
                        } else {
                            instruction_offset
                                .saturating_sub(last_instruction_offset)
                                .saturating_sub(1)
                        };

                        match frame {
                            StackFrame::SameFrame { frame_type } => {
                                // SameFrame uses the offset as the frame type
                                *frame_type = u8::try_from(instruction_delta_offset)?;
                            }
                            StackFrame::SameLocals1StackItemFrame { frame_type, .. } => {
                                // SameLocals1StackItemFrame requires that the 64 is added to the
                                // delta offset as it is used as the frame type.
                                let instruction_delta_offset =
                                    instruction_delta_offset.saturating_add(64);
                                *frame_type = u8::try_from(instruction_delta_offset)?;
                            }
                            StackFrame::AppendFrame { offset_delta, .. }
                            | StackFrame::ChopFrame { offset_delta, .. }
                            | StackFrame::FullFrame { offset_delta, .. }
                            | StackFrame::SameFrameExtended { offset_delta, .. }
                            | StackFrame::SameLocals1StackItemFrameExtended {
                                offset_delta, ..
                            } => {
                                *offset_delta = instruction_delta_offset;
                            }
                        }
                        last_byte_offset = byte_offset;
                        last_instruction_offset = instruction_offset;
                    }
                    let attribute = Attribute::StackMapTable { name_index, frames };
                    attributes.push(attribute);
                }
                _ => attributes.push(attribute),
            }
        }
        Ok(attributes)
    }

    /// Serialize the Attribute to bytes.
    ///
    /// This method writes the attribute to the provided byte vector in the format expected by the
    /// JVM specification.
    ///
    /// # Errors
    ///
    /// - Returns an error if serialization of any part of the attribute fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::Attribute;
    /// use ristretto_classfile::Result;
    ///
    /// let source_file = Attribute::SourceFile {
    ///     name_index: 1,
    ///     source_file_index: 2,
    /// };
    ///
    /// let mut bytes = Vec::new();
    /// source_file.to_bytes(&mut bytes)?;
    ///
    /// assert_eq!(bytes, vec![0, 1, 0, 0, 0, 2, 0, 2]);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    #[expect(clippy::too_many_lines)]
    #[expect(clippy::match_same_arms)]
    pub fn to_bytes(&self, bytes: &mut Vec<u8>) -> Result<()> {
        let (name_index, info) = match self {
            Attribute::ConstantValue {
                name_index,
                constant_value_index,
            } => (name_index, constant_value_index.to_be_bytes().to_vec()),
            Attribute::Code {
                name_index,
                max_stack,
                max_locals,
                code,
                exception_table,
                attributes,
            } => {
                let mut bytes = Vec::new();
                bytes.write_u16::<BigEndian>(*max_stack)?;
                bytes.write_u16::<BigEndian>(*max_locals)?;

                let (instruction_to_byte_map, code_bytes) =
                    offset_utils::instructions_to_bytes(code)?;
                let code_length = u32::try_from(code_bytes.len())?;
                bytes.write_u32::<BigEndian>(code_length)?;
                bytes.extend_from_slice(code_bytes.as_slice());

                let exceptions_length = u16::try_from(exception_table.len())?;
                bytes.write_u16::<BigEndian>(exceptions_length)?;
                for exception in &mut exception_table.clone() {
                    // Convert the instruction offset to byte offset
                    exception.range_pc.start = *instruction_to_byte_map
                        .get(&exception.range_pc.start)
                        .ok_or(InvalidInstructionOffset(u32::from(
                            exception.range_pc.start,
                        )))?;
                    exception.range_pc.end = instruction_to_byte_map
                        .iter()
                        .filter(|&(&k, _)| k <= exception.range_pc.end)
                        .max_by_key(|&(&k, _)| k)
                        .map(|(_, &v)| v)
                        .ok_or(InvalidInstructionOffset(u32::from(exception.range_pc.end)))?;
                    exception.handler_pc = *instruction_to_byte_map
                        .get(&exception.handler_pc)
                        .ok_or(InvalidInstructionOffset(u32::from(exception.handler_pc)))?;
                    exception.to_bytes(&mut bytes)?;
                }

                Self::to_bytes_code_attributes(attributes, &mut bytes, &instruction_to_byte_map)?;
                (name_index, bytes)
            }
            Attribute::StackMapTable { name_index, frames } => {
                let mut bytes = Vec::new();
                let frames_length = u16::try_from(frames.len())?;
                bytes.write_u16::<BigEndian>(frames_length)?;
                for frame in frames {
                    frame.to_bytes(&mut bytes)?;
                }
                (name_index, bytes)
            }
            Attribute::Exceptions {
                name_index,
                exception_indexes,
            } => {
                let mut bytes = Vec::new();
                let exception_indexes_length = u16::try_from(exception_indexes.len())?;
                bytes.write_u16::<BigEndian>(exception_indexes_length)?;
                for exception_index in exception_indexes {
                    bytes.write_u16::<BigEndian>(*exception_index)?;
                }
                (name_index, bytes)
            }
            Attribute::InnerClasses {
                name_index,
                classes,
            } => {
                let mut bytes = Vec::new();
                let classes_length = u16::try_from(classes.len())?;
                bytes.write_u16::<BigEndian>(classes_length)?;
                for inner_class in classes {
                    inner_class.to_bytes(&mut bytes)?;
                }
                (name_index, bytes)
            }
            Attribute::EnclosingMethod {
                name_index,
                class_index,
                method_index,
            } => (
                name_index,
                [class_index.to_be_bytes(), method_index.to_be_bytes()].concat(),
            ),
            Attribute::Synthetic { name_index } => (name_index, Vec::new()),
            Attribute::Signature {
                name_index,
                signature_index,
            } => (name_index, signature_index.to_be_bytes().to_vec()),
            Attribute::SourceFile {
                name_index,
                source_file_index: sourcefile_index,
            } => (name_index, sourcefile_index.to_be_bytes().to_vec()),
            Attribute::SourceDebugExtension {
                name_index,
                debug_extension,
            } => {
                let bytes = mutf8::to_bytes(debug_extension)?;
                (name_index, bytes)
            }
            Attribute::LineNumberTable {
                name_index,
                line_numbers,
            } => {
                let mut bytes = Vec::new();
                let line_numbers_length = u16::try_from(line_numbers.len())?;
                bytes.write_u16::<BigEndian>(line_numbers_length)?;
                for line_number in line_numbers {
                    line_number.to_bytes(&mut bytes)?;
                }
                (name_index, bytes)
            }
            Attribute::LocalVariableTable {
                name_index,
                variables,
            } => {
                let mut bytes = Vec::new();
                let variables_length = u16::try_from(variables.len())?;
                bytes.write_u16::<BigEndian>(variables_length)?;
                for variable in variables {
                    variable.to_bytes(&mut bytes)?;
                }
                (name_index, bytes)
            }
            Attribute::LocalVariableTypeTable {
                name_index,
                variable_types,
            } => {
                let mut bytes = Vec::new();
                let variable_types_length = u16::try_from(variable_types.len())?;
                bytes.write_u16::<BigEndian>(variable_types_length)?;
                for variable_type in variable_types {
                    variable_type.to_bytes(&mut bytes)?;
                }
                (name_index, bytes)
            }
            Attribute::Deprecated { name_index } => (name_index, Vec::new()),
            Attribute::RuntimeVisibleAnnotations {
                name_index,
                annotations,
            } => {
                let mut bytes = Vec::new();
                let annotations_length = u16::try_from(annotations.len())?;
                bytes.write_u16::<BigEndian>(annotations_length)?;
                for annotation in annotations {
                    annotation.to_bytes(&mut bytes)?;
                }
                (name_index, bytes)
            }
            Attribute::RuntimeInvisibleAnnotations {
                name_index,
                annotations,
            } => {
                let mut bytes = Vec::new();
                let annotations_length = u16::try_from(annotations.len())?;
                bytes.write_u16::<BigEndian>(annotations_length)?;
                for annotation in annotations {
                    annotation.to_bytes(&mut bytes)?;
                }
                (name_index, bytes)
            }
            Attribute::RuntimeVisibleParameterAnnotations {
                name_index,
                parameter_annotations,
            } => {
                let mut bytes = Vec::new();
                let parameter_annotations_length = u8::try_from(parameter_annotations.len())?;
                bytes.write_u8(parameter_annotations_length)?;
                for parameter_annotation in parameter_annotations {
                    parameter_annotation.to_bytes(&mut bytes)?;
                }
                (name_index, bytes)
            }
            Attribute::RuntimeInvisibleParameterAnnotations {
                name_index,
                parameter_annotations,
            } => {
                let mut bytes = Vec::new();
                let parameter_annotations_length = u8::try_from(parameter_annotations.len())?;
                bytes.write_u8(parameter_annotations_length)?;
                for parameter_annotation in parameter_annotations {
                    parameter_annotation.to_bytes(&mut bytes)?;
                }
                (name_index, bytes)
            }
            Attribute::RuntimeVisibleTypeAnnotations {
                name_index,
                type_annotations,
            } => {
                let mut bytes = Vec::new();
                let type_annotations_length = u16::try_from(type_annotations.len())?;
                bytes.write_u16::<BigEndian>(type_annotations_length)?;
                for type_annotation in type_annotations {
                    type_annotation.to_bytes(&mut bytes)?;
                }
                (name_index, bytes)
            }
            Attribute::RuntimeInvisibleTypeAnnotations {
                name_index,
                type_annotations,
            } => {
                let mut bytes = Vec::new();
                let type_annotations_length = u16::try_from(type_annotations.len())?;
                bytes.write_u16::<BigEndian>(type_annotations_length)?;
                for type_annotation in type_annotations {
                    type_annotation.to_bytes(&mut bytes)?;
                }
                (name_index, bytes)
            }
            Attribute::AnnotationDefault {
                name_index,
                element,
            } => {
                let mut bytes = Vec::new();
                element.to_bytes(&mut bytes)?;
                (name_index, bytes)
            }
            Attribute::BootstrapMethods {
                name_index,
                methods,
            } => {
                let mut bytes = Vec::new();
                let methods_length = u16::try_from(methods.len())?;
                bytes.write_u16::<BigEndian>(methods_length)?;
                for method in methods {
                    method.to_bytes(&mut bytes)?;
                }
                (name_index, bytes)
            }
            Attribute::MethodParameters {
                name_index,
                parameters,
            } => {
                let mut bytes = Vec::new();
                let parameters_length = u8::try_from(parameters.len())?;
                bytes.write_u8(parameters_length)?;
                for parameter in parameters {
                    parameter.to_bytes(&mut bytes)?;
                }
                (name_index, bytes)
            }
            Attribute::Module {
                name_index,
                module_name_index,
                flags,
                version_index,
                requires,
                exports,
                opens,
                uses,
                provides,
            } => {
                let mut bytes = Vec::new();
                bytes.write_u16::<BigEndian>(*module_name_index)?;
                flags.to_bytes(&mut bytes)?;
                bytes.write_u16::<BigEndian>(*version_index)?;

                let requires_length = u16::try_from(requires.len())?;
                bytes.write_u16::<BigEndian>(requires_length)?;
                for require in requires {
                    require.to_bytes(&mut bytes)?;
                }

                let exports_length = u16::try_from(exports.len())?;
                bytes.write_u16::<BigEndian>(exports_length)?;
                for export in exports {
                    export.to_bytes(&mut bytes)?;
                }

                let opens_length = u16::try_from(opens.len())?;
                bytes.write_u16::<BigEndian>(opens_length)?;
                for open in opens {
                    open.to_bytes(&mut bytes)?;
                }

                let use_index_length = u16::try_from(uses.len())?;
                bytes.write_u16::<BigEndian>(use_index_length)?;
                for use_index in uses {
                    bytes.write_u16::<BigEndian>(*use_index)?;
                }

                let provides_length = u16::try_from(provides.len())?;
                bytes.write_u16::<BigEndian>(provides_length)?;
                for provide in provides {
                    provide.to_bytes(&mut bytes)?;
                }
                (name_index, bytes)
            }
            Attribute::ModulePackages {
                name_index,
                package_indexes,
            } => {
                let mut bytes = Vec::new();
                let package_indexes_length = u16::try_from(package_indexes.len())?;
                bytes.write_u16::<BigEndian>(package_indexes_length)?;
                for package_index in package_indexes {
                    bytes.write_u16::<BigEndian>(*package_index)?;
                }
                (name_index, bytes)
            }
            Attribute::ModuleMainClass {
                name_index,
                main_class_index,
            } => (name_index, main_class_index.to_be_bytes().to_vec()),
            Attribute::NestHost {
                name_index,
                host_class_index,
            } => (name_index, host_class_index.to_be_bytes().to_vec()),
            Attribute::NestMembers {
                name_index,
                class_indexes,
            } => {
                let mut bytes = Vec::new();
                let class_indexes_length = u16::try_from(class_indexes.len())?;
                bytes.write_u16::<BigEndian>(class_indexes_length)?;
                for class_index in class_indexes {
                    bytes.write_u16::<BigEndian>(*class_index)?;
                }
                (name_index, bytes)
            }
            Attribute::Record {
                name_index,
                records,
            } => {
                let mut bytes = Vec::new();
                let records_length = u16::try_from(records.len())?;
                bytes.write_u16::<BigEndian>(records_length)?;
                for record in records {
                    record.to_bytes(&mut bytes)?;
                }
                (name_index, bytes)
            }
            Attribute::PermittedSubclasses {
                name_index,
                class_indexes,
            } => {
                let mut bytes = Vec::new();
                let class_indexes_length = u16::try_from(class_indexes.len())?;
                bytes.write_u16::<BigEndian>(class_indexes_length)?;
                for class_index in class_indexes {
                    bytes.write_u16::<BigEndian>(*class_index)?;
                }
                (name_index, bytes)
            }
            Attribute::Unknown { name_index, info } => (name_index, info.clone()),
        };

        bytes.write_u16::<BigEndian>(*name_index)?;

        let info_length = u32::try_from(info.len())?;
        bytes.write_u32::<BigEndian>(info_length)?;
        bytes.extend_from_slice(info.as_slice());
        Ok(())
    }

    /// Serializes the attributes of a Code attribute to bytes, handling special cases.
    ///
    /// This method is used internally by the `to_bytes` method when serializing a Code attribute.
    /// It handles the conversion of instruction offsets to byte offsets for attributes that contain
    /// instruction references (like `LineNumberTable` and `StackMapTable`).
    ///
    /// # Note
    ///
    /// This method is necessary because the JVM uses byte offsets in class files, but our in-memory
    /// representation uses instruction offsets for easier manipulation.
    ///
    /// # Errors
    ///
    /// - Returns an error if any instruction offset cannot be mapped to a byte offset.
    #[expect(clippy::too_many_lines)]
    fn to_bytes_code_attributes(
        attributes: &Vec<Attribute>,
        bytes: &mut Vec<u8>,
        instruction_to_byte_map: &HashMap<u16, u16>,
    ) -> Result<()> {
        let attributes_length = u16::try_from(attributes.len())?;
        bytes.write_u16::<BigEndian>(attributes_length)?;
        for attribute in attributes {
            match attribute {
                Attribute::LineNumberTable {
                    name_index,
                    line_numbers,
                } => {
                    let mut new_line_numbers = Vec::new();
                    for line_number in line_numbers {
                        let start_pc = *instruction_to_byte_map
                            .get(&line_number.start_pc)
                            .ok_or(InvalidInstructionOffset(u32::from(line_number.start_pc)))?;
                        new_line_numbers.push(LineNumber {
                            start_pc,
                            line_number: line_number.line_number,
                        });
                    }
                    let attribute = Attribute::LineNumberTable {
                        name_index: *name_index,
                        line_numbers: new_line_numbers,
                    };
                    attribute.to_bytes(bytes)?;
                }
                Attribute::StackMapTable { name_index, frames } => {
                    let mut first_frame = true;
                    let mut last_byte_offset: u16 = 0;
                    let mut last_instruction_offset: u16 = 0;
                    let mut new_frames = Vec::new();
                    for frame in frames {
                        let offset_delta = frame.offset_delta();
                        let instruction_offset = if first_frame {
                            offset_delta
                        } else {
                            last_instruction_offset
                                .saturating_add(offset_delta)
                                .saturating_add(1)
                        };

                        let byte_offset = *instruction_to_byte_map
                            .get(&instruction_offset)
                            .ok_or(InvalidInstructionOffset(u32::from(instruction_offset)))?;
                        // Calculate the byte delta offset from the last instruction offset
                        // subtracting 1 to account for the current instruction.
                        let byte_delta_offset = if last_byte_offset == 0 {
                            first_frame = false;
                            byte_offset
                        } else {
                            byte_offset
                                .saturating_sub(last_byte_offset)
                                .saturating_sub(1)
                        };

                        match frame {
                            StackFrame::SameFrame { .. } => {
                                // SameFrame uses the offset as the frame type
                                new_frames.push(StackFrame::SameFrame {
                                    frame_type: u8::try_from(byte_delta_offset)?,
                                });
                            }
                            StackFrame::SameLocals1StackItemFrame { stack, .. } => {
                                // SameLocals1StackItemFrame requires that the 64 is added to the
                                // delta offset as it is used as the frame type.
                                let byte_delta_offset = byte_delta_offset.saturating_add(64);
                                new_frames.push(StackFrame::SameLocals1StackItemFrame {
                                    frame_type: u8::try_from(byte_delta_offset)?,
                                    stack: stack.clone(),
                                });
                            }
                            StackFrame::AppendFrame {
                                frame_type, locals, ..
                            } => {
                                new_frames.push(StackFrame::AppendFrame {
                                    frame_type: *frame_type,
                                    offset_delta: byte_delta_offset,
                                    locals: locals.clone(),
                                });
                            }
                            StackFrame::ChopFrame { frame_type, .. } => {
                                new_frames.push(StackFrame::ChopFrame {
                                    frame_type: *frame_type,
                                    offset_delta: byte_delta_offset,
                                });
                            }
                            StackFrame::FullFrame {
                                frame_type,
                                locals,
                                stack,
                                ..
                            } => {
                                new_frames.push(StackFrame::FullFrame {
                                    frame_type: *frame_type,
                                    offset_delta: byte_delta_offset,
                                    locals: locals.clone(),
                                    stack: stack.clone(),
                                });
                            }
                            StackFrame::SameFrameExtended { frame_type, .. } => {
                                new_frames.push(StackFrame::SameFrameExtended {
                                    frame_type: *frame_type,
                                    offset_delta: byte_delta_offset,
                                });
                            }
                            StackFrame::SameLocals1StackItemFrameExtended {
                                frame_type,
                                stack,
                                ..
                            } => {
                                new_frames.push(StackFrame::SameLocals1StackItemFrameExtended {
                                    frame_type: *frame_type,
                                    offset_delta: byte_delta_offset,
                                    stack: stack.clone(),
                                });
                            }
                        }
                        last_byte_offset = byte_offset;
                        last_instruction_offset = instruction_offset;
                    }
                    let attribute = Attribute::StackMapTable {
                        name_index: *name_index,
                        frames: new_frames,
                    };
                    attribute.to_bytes(bytes)?;
                }
                _ => attribute.to_bytes(bytes)?,
            }
        }
        Ok(())
    }
}

impl fmt::Display for Attribute {
    /// Implements the `Display` trait for `Attribute` to provide human-readable output.
    ///
    /// This implementation provides specialized formatting for certain attribute types:
    /// - `Code` attributes show detailed bytecode instructions with line numbers and offsets
    /// - `StackMapTable` attributes display frames in a structured format
    /// - Other attributes fall back to a Debug-like representation
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::{Attribute, LineNumber};
    /// use ristretto_classfile::attributes::Instruction;
    ///
    /// // Create a LineNumberTable attribute
    /// let line_number_table = Attribute::LineNumberTable {
    ///     name_index: 1,
    ///     line_numbers: vec![
    ///         LineNumber { start_pc: 0, line_number: 1 },
    ///         LineNumber { start_pc: 5, line_number: 2 },
    ///     ],
    /// };
    ///
    /// // Display the attribute as a string
    /// let output = line_number_table.to_string();
    /// assert!(output.contains("LineNumberTable"));
    /// assert!(output.contains("start_pc: 0"));
    /// assert!(output.contains("line_number: 1"));
    ///
    /// // Code attributes have special formatting
    /// let code_attribute = Attribute::Code {
    ///     name_index: 1,
    ///     max_stack: 2,
    ///     max_locals: 1,
    ///     code: vec![Instruction::Iconst_1, Instruction::Ireturn],
    ///     exception_table: vec![],
    ///     attributes: vec![],
    /// };
    ///
    /// let output = code_attribute.to_string();
    /// assert!(output.contains("Code:"));
    /// assert!(output.contains("stack=2, locals=1"));
    /// assert!(output.contains("iconst_1"));
    /// assert!(output.contains("ireturn"));
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Attribute::Code {
                max_stack,
                max_locals,
                code,
                exception_table,
                attributes,
                ..
            } => {
                writeln!(f, "Code:")?;
                writeln!(f, "  stack={max_stack}, locals={max_locals}")?;

                let (instruction_to_byte_map, code_bytes) =
                    offset_utils::instructions_to_bytes(code).map_err(|_| fmt::Error)?;
                let code_length = u64::try_from(code_bytes.len()).map_err(|_| fmt::Error)?;
                let mut cursor = Cursor::new(code_bytes.clone());
                while cursor.position() < code_length {
                    let index = cursor.position();
                    let mut instruction =
                        Instruction::from_bytes(&mut cursor).map_err(|_| fmt::Error)?;
                    match instruction {
                        Instruction::Tableswitch(ref mut table_switch) => {
                            let position = i32::try_from(index).map_err(|_| fmt::Error)?;
                            table_switch.default += position;
                            for offset in &mut table_switch.offsets {
                                *offset += position;
                            }
                        }
                        Instruction::Lookupswitch(ref mut lookupswitch) => {
                            let position = i32::try_from(index).map_err(|_| fmt::Error)?;
                            lookupswitch.default += position;
                            for offset in lookupswitch.pairs.values_mut() {
                                *offset += position;
                            }
                        }
                        _ => {}
                    }
                    let value = instruction.to_string();
                    let (name, value) = value.split_once(' ').unwrap_or((value.as_str(), ""));
                    let value = format!("{name:<13} {value}");
                    writeln!(f, "{index:>6}: {}", value.trim())?;
                }

                let mut exception_table = exception_table.clone();
                for exception in &mut exception_table {
                    exception.range_pc.start = *instruction_to_byte_map
                        .get(&exception.range_pc.start)
                        .ok_or(fmt::Error)?;
                    exception.range_pc.end = instruction_to_byte_map
                        .iter()
                        .filter(|&(&k, _)| k <= exception.range_pc.end)
                        .max_by_key(|&(&k, _)| k)
                        .map(|(_, &v)| v + 1)
                        .ok_or(fmt::Error)?;
                    exception.handler_pc = *instruction_to_byte_map
                        .get(&exception.handler_pc)
                        .ok_or(fmt::Error)?;
                }
                if !exception_table.is_empty() {
                    writeln!(f, "  {exception_table:?}")?;
                }

                for attribute in attributes {
                    match attribute {
                        Attribute::LineNumberTable { line_numbers, .. } => {
                            writeln!(f, "  LineNumberTable:")?;
                            for line_number in line_numbers {
                                let start_pc = instruction_to_byte_map
                                    .get(&line_number.start_pc)
                                    .ok_or(fmt::Error)?;
                                let line_number = line_number.line_number;
                                writeln!(f, "    line {line_number}: {start_pc}")?;
                            }
                        }
                        _ => writeln!(f, "{}", indent_lines(&attribute.to_string(), "  "))?,
                    }
                }
            }
            Attribute::StackMapTable { frames, .. } => {
                writeln!(f, "StackMapTable: number_of_entries = {}", frames.len())?;
                for frame in frames {
                    writeln!(f, "{}", indent_lines(&frame.to_string(), "  "))?;
                }
            }
            _ => write!(f, "{self:?}")?,
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::JAVA_1_0_2;
    use crate::attributes::annotation_value_pair::AnnotationValuePair;
    use crate::attributes::nested_class_access_flags::NestedClassAccessFlags;
    use crate::attributes::{
        AnnotationElement, ExportsFlags, OpensFlags, RequiresFlags, TargetPath, TargetType,
        VerificationType,
    };
    use crate::method_access_flags::MethodAccessFlags;
    use indoc::indoc;

    const VERSION_45_0: Version = JAVA_1_0_2;

    #[test]
    fn test_invalid_attribute_name_index_error() {
        let expected_bytes = [0, 1, 0, 0, 0, 0];

        assert_eq!(
            Err(InvalidAttributeNameIndex(1)),
            Attribute::from_bytes(
                &ConstantPool::default(),
                &mut Cursor::new(expected_bytes.to_vec())
            )
        );
    }

    fn test_invalid_attribute_from_bytes_error(attribute: &str) -> Result<()> {
        let mut constant_pool = ConstantPool::default();
        constant_pool.add_utf8(attribute)?;
        let expected_bytes = [0, 1, 0, 0, 0, 64];

        assert_eq!(
            Err(InvalidAttributeLength(64)),
            Attribute::from_bytes(&constant_pool, &mut Cursor::new(expected_bytes.to_vec()))
        );
        Ok(())
    }

    fn test_attribute(
        attribute: &Attribute,
        expected_bytes: &[u8],
        supported_versions: &Version,
    ) -> Result<()> {
        let name = attribute.name();
        let mut constant_pool = ConstantPool::default();
        constant_pool.add_utf8(name)?;

        assert!(attribute.valid_for_version(supported_versions));
        assert!(!attribute.valid_for_version(&VERSION_45_0));

        let mut bytes = Vec::new();
        attribute.to_bytes(&mut bytes)?;
        assert_eq!(expected_bytes, &bytes[..]);
        let mut bytes = Cursor::new(expected_bytes.to_vec());
        assert_eq!(
            *attribute,
            Attribute::from_bytes(&constant_pool, &mut bytes)?
        );
        Ok(())
    }

    #[test]
    fn test_constant_value_from_bytes_error() -> Result<()> {
        test_invalid_attribute_from_bytes_error("ConstantValue")
    }

    #[test]
    fn test_constant_value() -> Result<()> {
        let attribute = Attribute::ConstantValue {
            name_index: 1,
            constant_value_index: 42,
        };
        let expected_bytes = [0, 1, 0, 0, 0, 2, 0, 42];

        test_attribute(&attribute, &expected_bytes, &VERSION_45_3)
    }

    #[expect(clippy::too_many_lines)]
    #[test]
    fn test_code() -> Result<()> {
        let constant = Attribute::ConstantValue {
            name_index: 2,
            constant_value_index: 42,
        };
        let line_number_table = Attribute::LineNumberTable {
            name_index: 3,
            line_numbers: vec![LineNumber {
                start_pc: 0,
                line_number: 1,
            }],
        };
        let frames = vec![
            StackFrame::SameFrame { frame_type: 0 },
            StackFrame::SameLocals1StackItemFrame {
                frame_type: 65,
                stack: vec![VerificationType::Null],
            },
            StackFrame::SameLocals1StackItemFrameExtended {
                frame_type: 247,
                offset_delta: 0,
                stack: vec![VerificationType::Null],
            },
            StackFrame::ChopFrame {
                frame_type: 248,
                offset_delta: 0,
            },
            StackFrame::SameFrameExtended {
                frame_type: 251,
                offset_delta: 0,
            },
            StackFrame::AppendFrame {
                frame_type: 252,
                offset_delta: 0,
                locals: vec![VerificationType::Null],
            },
            StackFrame::FullFrame {
                frame_type: 255,
                offset_delta: 0,
                locals: vec![VerificationType::Null],
                stack: vec![VerificationType::Integer],
            },
        ];
        let stack_map_table = Attribute::StackMapTable {
            name_index: 4,
            frames: frames.clone(),
        };
        let exception_table_entry = ExceptionTableEntry {
            range_pc: 0..1,
            handler_pc: 0,
            catch_type: 4,
        };
        let mut attribute = Attribute::Code {
            name_index: 1,
            max_stack: 2,
            max_locals: 3,
            code: vec![
                Instruction::Nop,
                Instruction::Nop,
                Instruction::Nop,
                Instruction::Nop,
                Instruction::Nop,
                Instruction::Nop,
                Instruction::Nop,
                Instruction::Nop,
                Instruction::Return,
            ],
            exception_table: vec![exception_table_entry],
            attributes: vec![
                constant.clone(),
                line_number_table.clone(),
                stack_map_table.clone(),
            ],
        };
        let expected_bytes = [
            0, 1, 0, 0, 0, 83, 0, 2, 0, 3, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 177, 0, 1, 0, 0, 0,
            1, 0, 0, 0, 4, 0, 3, 0, 2, 0, 0, 0, 2, 0, 42, 0, 3, 0, 0, 0, 6, 0, 1, 0, 0, 0, 1, 0, 4,
            0, 0, 0, 28, 0, 7, 0, 66, 5, 247, 0, 0, 5, 248, 0, 0, 251, 0, 0, 252, 0, 0, 5, 255, 0,
            0, 0, 1, 5, 0, 1, 1,
        ];
        let expected = indoc! {"\
            Code:
              stack=2, locals=3
                 0: nop
                 1: nop
                 2: nop
                 3: nop
                 4: nop
                 5: nop
                 6: nop
                 7: nop
                 8: return
              [ExceptionTableEntry { range_pc: 0..2, handler_pc: 0, catch_type: 4 }]
              ConstantValue { name_index: 2, constant_value_index: 42 }
              LineNumberTable:
                line 1: 0
              StackMapTable: number_of_entries = 7
                frame_type = 0 /* same */
                frame_type = 65 /* same_locals_1_stack_item */
                  stack = [ null ]
                frame_type = 247 /* same_locals_1_stack_item_frame_extended */
                  offset_delta = 0
                  stack = [ null ]
                frame_type = 248 /* chop */
                  offset_delta = 0
                frame_type = 251 /* same_frame_extended */
                  offset_delta = 0
                frame_type = 252 /* append */
                  offset_delta = 0
                  locals = [ null ]
                frame_type = 255 /* full_frame */
                  offset_delta = 0
                  locals = [ null ]
                  stack = [ int ]
        "};

        assert_eq!(expected, attribute.to_string());

        let mut constant_pool = ConstantPool::default();
        constant_pool.add_utf8(attribute.name())?;
        constant_pool.add_utf8(constant.name())?;
        constant_pool.add_utf8(line_number_table.name())?;
        constant_pool.add_utf8(stack_map_table.name())?;

        assert!(attribute.valid_for_version(&VERSION_50_0)); // Update to VERSION_50_0 since StackMapTable requires it
        assert!(!attribute.valid_for_version(&VERSION_45_0));

        let mut bytes = Vec::new();
        attribute.to_bytes(&mut bytes)?;
        assert_eq!(expected_bytes, &bytes[..]);
        let mut bytes = Cursor::new(expected_bytes.to_vec());

        // Adjust the frame_type offest before comparing
        if let Attribute::Code { attributes, .. } = &mut attribute
            && let Some(Attribute::StackMapTable { frames, .. }) = attributes.get_mut(2)
            && let Some(StackFrame::SameLocals1StackItemFrame { frame_type, .. }) =
                frames.get_mut(1)
        {
            *frame_type = 66; // Update to match the expected frame type
        }
        let code_attribute = Attribute::from_bytes(&constant_pool, &mut bytes)?;
        assert_eq!(attribute, code_attribute);
        Ok(())
    }

    #[test]
    fn test_stack_map_table() -> Result<()> {
        let attribute = Attribute::StackMapTable {
            name_index: 1,
            frames: vec![StackFrame::SameFrame { frame_type: 0 }],
        };
        let expected_bytes = [0, 1, 0, 0, 0, 3, 0, 1, 0];

        assert_eq!(
            indoc! {"
                StackMapTable: number_of_entries = 1
                  frame_type = 0 /* same */
            "},
            attribute.to_string()
        );
        test_attribute(&attribute, &expected_bytes, &VERSION_50_0)
    }

    #[test]
    fn test_exceptions() -> Result<()> {
        let attribute = Attribute::Exceptions {
            name_index: 1,
            exception_indexes: vec![42],
        };
        let expected_bytes = [0, 1, 0, 0, 0, 4, 0, 1, 0, 42];

        assert_eq!(
            "Exceptions { name_index: 1, exception_indexes: [42] }",
            attribute.to_string()
        );
        test_attribute(&attribute, &expected_bytes, &VERSION_45_3)
    }

    #[test]
    fn test_inner_classes() -> Result<()> {
        let inner_class = InnerClass {
            class_info_index: 1,
            outer_class_info_index: 2,
            name_index: 3,
            access_flags: NestedClassAccessFlags::PUBLIC,
        };
        let attribute = Attribute::InnerClasses {
            name_index: 1,
            classes: vec![inner_class],
        };
        let expected_bytes = [0, 1, 0, 0, 0, 10, 0, 1, 0, 1, 0, 2, 0, 3, 0, 1];

        assert_eq!(
            "InnerClasses { name_index: 1, classes: [InnerClass { class_info_index: 1, outer_class_info_index: 2, name_index: 3, access_flags: NestedClassAccessFlags(PUBLIC) }] }",
            attribute.to_string()
        );
        test_attribute(&attribute, &expected_bytes, &VERSION_45_3)
    }

    #[test]
    fn test_enclosing_method_from_bytes_error() -> Result<()> {
        test_invalid_attribute_from_bytes_error("EnclosingMethod")
    }

    #[test]
    fn test_enclosing_method() -> Result<()> {
        let attribute = Attribute::EnclosingMethod {
            name_index: 1,
            class_index: 42,
            method_index: 3,
        };
        let expected_bytes = [0, 1, 0, 0, 0, 4, 0, 42, 0, 3];

        assert_eq!(
            "EnclosingMethod { name_index: 1, class_index: 42, method_index: 3 }",
            attribute.to_string()
        );
        test_attribute(&attribute, &expected_bytes, &VERSION_49_0)
    }

    #[test]
    fn test_synthetic_from_bytes_error() -> Result<()> {
        test_invalid_attribute_from_bytes_error("Synthetic")
    }

    #[test]
    fn test_synthetic() -> Result<()> {
        let attribute = Attribute::Synthetic { name_index: 1 };
        let expected_bytes = [0, 1, 0, 0, 0, 0];

        assert_eq!("Synthetic { name_index: 1 }", attribute.to_string());
        test_attribute(&attribute, &expected_bytes, &VERSION_45_3)
    }

    #[test]
    fn test_signature_from_bytes_error() -> Result<()> {
        test_invalid_attribute_from_bytes_error("Signature")
    }

    #[test]
    fn test_signature() -> Result<()> {
        let attribute = Attribute::Signature {
            name_index: 1,
            signature_index: 42,
        };
        let expected_bytes = [0, 1, 0, 0, 0, 2, 0, 42];

        assert_eq!(
            "Signature { name_index: 1, signature_index: 42 }",
            attribute.to_string()
        );
        test_attribute(&attribute, &expected_bytes, &VERSION_49_0)
    }

    #[test]
    fn test_source_file_from_bytes_error() -> Result<()> {
        test_invalid_attribute_from_bytes_error("SourceFile")
    }

    #[test]
    fn test_source_file() -> Result<()> {
        let attribute = Attribute::SourceFile {
            name_index: 1,
            source_file_index: 42,
        };
        let expected_bytes = [0, 1, 0, 0, 0, 2, 0, 42];

        assert_eq!(
            "SourceFile { name_index: 1, source_file_index: 42 }",
            attribute.to_string()
        );
        test_attribute(&attribute, &expected_bytes, &VERSION_45_3)
    }

    #[test]
    fn test_source_debug_extension() -> Result<()> {
        let attribute = Attribute::SourceDebugExtension {
            name_index: 1,
            debug_extension: "foo".to_string(),
        };
        let expected_bytes = [0, 1, 0, 0, 0, 3, 102, 111, 111];

        assert_eq!(
            "SourceDebugExtension { name_index: 1, debug_extension: \"foo\" }",
            attribute.to_string()
        );
        test_attribute(&attribute, &expected_bytes, &VERSION_49_0)
    }

    #[test]
    fn test_line_number_table() -> Result<()> {
        let attribute = Attribute::LineNumberTable {
            name_index: 1,
            line_numbers: vec![LineNumber {
                start_pc: 2,
                line_number: 42,
            }],
        };
        let expected_bytes = [0, 1, 0, 0, 0, 6, 0, 1, 0, 2, 0, 42];
        let expected = "LineNumberTable { name_index: 1, line_numbers: [LineNumber { start_pc: 2, line_number: 42 }] }";

        assert_eq!(expected, attribute.to_string());
        test_attribute(&attribute, &expected_bytes, &VERSION_45_3)
    }

    #[test]
    fn test_locale_variable_table() -> Result<()> {
        let variable = LocalVariableTable {
            start_pc: 1,
            length: 2,
            name_index: 3,
            descriptor_index: 4,
            index: 5,
        };
        let attribute = Attribute::LocalVariableTable {
            name_index: 1,
            variables: vec![variable],
        };
        let expected_bytes = [0, 1, 0, 0, 0, 12, 0, 1, 0, 1, 0, 2, 0, 3, 0, 4, 0, 5];

        assert_eq!(
            "LocalVariableTable { name_index: 1, variables: [LocalVariableTable { start_pc: 1, length: 2, name_index: 3, descriptor_index: 4, index: 5 }] }",
            attribute.to_string()
        );
        test_attribute(&attribute, &expected_bytes, &VERSION_49_0)
    }

    #[test]
    fn test_local_variable_type_table() -> Result<()> {
        let variable_type = LocalVariableTypeTable {
            start_pc: 1,
            length: 2,
            name_index: 3,
            signature_index: 4,
            index: 5,
        };
        let attribute = Attribute::LocalVariableTypeTable {
            name_index: 1,
            variable_types: vec![variable_type],
        };
        let expected_bytes = [0, 1, 0, 0, 0, 12, 0, 1, 0, 1, 0, 2, 0, 3, 0, 4, 0, 5];

        assert_eq!(
            "LocalVariableTypeTable { name_index: 1, variable_types: [LocalVariableTypeTable { start_pc: 1, length: 2, name_index: 3, signature_index: 4, index: 5 }] }",
            attribute.to_string()
        );
        test_attribute(&attribute, &expected_bytes, &VERSION_45_3)
    }

    #[test]
    fn test_deprecated_from_bytes_error() -> Result<()> {
        test_invalid_attribute_from_bytes_error("Deprecated")
    }

    #[test]
    fn test_deprecated() -> Result<()> {
        let attribute = Attribute::Deprecated { name_index: 1 };
        let expected_bytes = [0, 1, 0, 0, 0, 0];

        assert_eq!("Deprecated { name_index: 1 }", attribute.to_string());
        test_attribute(&attribute, &expected_bytes, &VERSION_45_3)
    }

    #[test]
    fn test_runtime_visible_annotations() -> Result<()> {
        let attribute = Attribute::RuntimeVisibleAnnotations {
            name_index: 1,
            annotations: vec![Annotation {
                type_index: 1,
                elements: vec![AnnotationValuePair {
                    name_index: 3,
                    value: AnnotationElement::Byte {
                        const_value_index: 42,
                    },
                }],
            }],
        };
        let expected_bytes = [0, 1, 0, 0, 0, 11, 0, 1, 0, 1, 0, 1, 0, 3, 66, 0, 42];

        assert_eq!(
            "RuntimeVisibleAnnotations { name_index: 1, annotations: [Annotation { type_index: 1, elements: [AnnotationValuePair { name_index: 3, value: Byte { const_value_index: 42 } }] }] }",
            attribute.to_string()
        );
        test_attribute(&attribute, &expected_bytes, &VERSION_49_0)
    }

    #[test]
    fn test_runtime_invisible_annotations() -> Result<()> {
        let attribute = Attribute::RuntimeInvisibleAnnotations {
            name_index: 1,
            annotations: vec![Annotation {
                type_index: 1,
                elements: vec![AnnotationValuePair {
                    name_index: 3,
                    value: AnnotationElement::Byte {
                        const_value_index: 42,
                    },
                }],
            }],
        };
        let expected_bytes = [0, 1, 0, 0, 0, 11, 0, 1, 0, 1, 0, 1, 0, 3, 66, 0, 42];

        assert_eq!(
            "RuntimeInvisibleAnnotations { name_index: 1, annotations: [Annotation { type_index: 1, elements: [AnnotationValuePair { name_index: 3, value: Byte { const_value_index: 42 } }] }] }",
            attribute.to_string()
        );
        test_attribute(&attribute, &expected_bytes, &VERSION_49_0)
    }

    #[test]
    fn test_runtime_visible_parameter_annotations() -> Result<()> {
        let annotation_value_pair = AnnotationValuePair {
            name_index: 1,
            value: AnnotationElement::Byte {
                const_value_index: 42,
            },
        };
        let annotation = Annotation {
            type_index: 3,
            elements: vec![annotation_value_pair],
        };
        let parameter_annotation = ParameterAnnotation {
            annotations: vec![annotation],
        };
        let attribute = Attribute::RuntimeVisibleParameterAnnotations {
            name_index: 1,
            parameter_annotations: vec![parameter_annotation],
        };
        let expected_bytes = [0, 1, 0, 0, 0, 12, 1, 0, 1, 0, 3, 0, 1, 0, 1, 66, 0, 42];

        assert_eq!(
            "RuntimeVisibleParameterAnnotations { name_index: 1, parameter_annotations: [ParameterAnnotation { annotations: [Annotation { type_index: 3, elements: [AnnotationValuePair { name_index: 1, value: Byte { const_value_index: 42 } }] }] }] }",
            attribute.to_string()
        );
        test_attribute(&attribute, &expected_bytes, &VERSION_49_0)
    }

    #[test]
    fn test_runtime_invisible_parameter_annotations() -> Result<()> {
        let annotation_value_pair = AnnotationValuePair {
            name_index: 1,
            value: AnnotationElement::Byte {
                const_value_index: 42,
            },
        };
        let annotation = Annotation {
            type_index: 3,
            elements: vec![annotation_value_pair],
        };
        let parameter_annotation = ParameterAnnotation {
            annotations: vec![annotation],
        };
        let attribute = Attribute::RuntimeInvisibleParameterAnnotations {
            name_index: 1,
            parameter_annotations: vec![parameter_annotation],
        };
        let expected_bytes = [0, 1, 0, 0, 0, 12, 1, 0, 1, 0, 3, 0, 1, 0, 1, 66, 0, 42];

        assert_eq!(
            "RuntimeInvisibleParameterAnnotations { name_index: 1, parameter_annotations: [ParameterAnnotation { annotations: [Annotation { type_index: 3, elements: [AnnotationValuePair { name_index: 1, value: Byte { const_value_index: 42 } }] }] }] }",
            attribute.to_string()
        );
        test_attribute(&attribute, &expected_bytes, &VERSION_49_0)
    }

    #[test]
    fn test_runtime_visible_type_annotations() -> Result<()> {
        let element = AnnotationValuePair {
            name_index: 1,
            value: AnnotationElement::Byte {
                const_value_index: 42,
            },
        };
        let type_annotation = TypeAnnotation {
            target_type: TargetType::Empty { target_type: 19 },
            type_path: vec![TargetPath {
                type_path_kind: 1,
                type_argument_index: 2,
            }],
            type_index: 42,
            elements: vec![element],
        };
        let attribute = Attribute::RuntimeVisibleTypeAnnotations {
            name_index: 1,
            type_annotations: vec![type_annotation],
        };
        let expected_bytes = [
            0, 1, 0, 0, 0, 15, 0, 1, 19, 1, 1, 2, 0, 42, 0, 1, 0, 1, 66, 0, 42,
        ];

        assert_eq!(
            "RuntimeVisibleTypeAnnotations { name_index: 1, type_annotations: [TypeAnnotation { target_type: Empty { target_type: 19 }, type_path: [TargetPath { type_path_kind: 1, type_argument_index: 2 }], type_index: 42, elements: [AnnotationValuePair { name_index: 1, value: Byte { const_value_index: 42 } }] }] }",
            attribute.to_string()
        );
        test_attribute(&attribute, &expected_bytes, &VERSION_52_0)
    }

    #[test]
    fn test_runtime_invisible_type_annotations() -> Result<()> {
        let element = AnnotationValuePair {
            name_index: 1,
            value: AnnotationElement::Byte {
                const_value_index: 42,
            },
        };
        let type_annotation = TypeAnnotation {
            target_type: TargetType::Empty { target_type: 19 },
            type_path: vec![TargetPath {
                type_path_kind: 1,
                type_argument_index: 2,
            }],
            type_index: 42,
            elements: vec![element],
        };
        let attribute = Attribute::RuntimeInvisibleTypeAnnotations {
            name_index: 1,
            type_annotations: vec![type_annotation],
        };
        let expected_bytes = [
            0, 1, 0, 0, 0, 15, 0, 1, 19, 1, 1, 2, 0, 42, 0, 1, 0, 1, 66, 0, 42,
        ];

        assert_eq!(
            "RuntimeInvisibleTypeAnnotations { name_index: 1, type_annotations: [TypeAnnotation { target_type: Empty { target_type: 19 }, type_path: [TargetPath { type_path_kind: 1, type_argument_index: 2 }], type_index: 42, elements: [AnnotationValuePair { name_index: 1, value: Byte { const_value_index: 42 } }] }] }",
            attribute.to_string()
        );
        test_attribute(&attribute, &expected_bytes, &VERSION_52_0)
    }

    #[test]
    fn test_annotation_default() -> Result<()> {
        let attribute = Attribute::AnnotationDefault {
            name_index: 1,
            element: AnnotationElement::Byte {
                const_value_index: 42,
            },
        };
        let expected_bytes = [0, 1, 0, 0, 0, 3, 66, 0, 42];

        assert_eq!(
            "AnnotationDefault { name_index: 1, element: Byte { const_value_index: 42 } }",
            attribute.to_string()
        );
        test_attribute(&attribute, &expected_bytes, &VERSION_49_0)
    }

    #[test]
    fn test_bootstrap_methods() -> Result<()> {
        let method = BootstrapMethod {
            bootstrap_method_ref: 3,
            arguments: vec![42],
        };
        let attribute = Attribute::BootstrapMethods {
            name_index: 1,
            methods: vec![method],
        };
        let expected_bytes = [0, 1, 0, 0, 0, 8, 0, 1, 0, 3, 0, 1, 0, 42];

        assert_eq!(
            "BootstrapMethods { name_index: 1, methods: [BootstrapMethod { bootstrap_method_ref: 3, arguments: [42] }] }",
            attribute.to_string()
        );
        test_attribute(&attribute, &expected_bytes, &VERSION_51_0)
    }

    #[test]
    fn test_method_parameters() -> Result<()> {
        let parameter = MethodParameter {
            name_index: 2,
            access_flags: MethodAccessFlags::PUBLIC,
        };
        let attribute = Attribute::MethodParameters {
            name_index: 1,
            parameters: vec![parameter],
        };
        let expected_bytes = [0, 1, 0, 0, 0, 5, 1, 0, 2, 0, 1];

        assert_eq!(
            "MethodParameters { name_index: 1, parameters: [MethodParameter { name_index: 2, access_flags: MethodAccessFlags(PUBLIC) }] }",
            attribute.to_string()
        );
        test_attribute(&attribute, &expected_bytes, &VERSION_52_0)
    }

    #[test]
    fn test_module() -> Result<()> {
        let attribute = Attribute::Module {
            name_index: 1,
            module_name_index: 2,
            flags: ModuleAccessFlags::OPEN,
            version_index: 4,
            requires: vec![Requires {
                index: 5,
                flags: RequiresFlags::MANDATED,
                version_index: 7,
            }],
            exports: vec![Exports {
                index: 8,
                flags: ExportsFlags::MANDATED,
                to_index: vec![10],
            }],
            opens: vec![Opens {
                index: 11,
                flags: OpensFlags::MANDATED,
                to_index: vec![13],
            }],
            uses: vec![14],
            provides: vec![Provides {
                index: 15,
                with_index: vec![16],
            }],
        };
        let expected_bytes = [
            0, 1, 0, 0, 0, 46, 0, 2, 0, 32, 0, 4, 0, 1, 0, 5, 128, 0, 0, 7, 0, 1, 0, 8, 128, 0, 0,
            1, 0, 10, 0, 1, 0, 11, 128, 0, 0, 1, 0, 13, 0, 1, 0, 14, 0, 1, 0, 15, 0, 1, 0, 16,
        ];

        assert_eq!(
            "Module { name_index: 1, module_name_index: 2, flags: ModuleAccessFlags(OPEN), version_index: 4, requires: [Requires { index: 5, flags: RequiresFlags(MANDATED), version_index: 7 }], exports: [Exports { index: 8, flags: ExportsFlags(MANDATED), to_index: [10] }], opens: [Opens { index: 11, flags: OpensFlags(MANDATED), to_index: [13] }], uses: [14], provides: [Provides { index: 15, with_index: [16] }] }",
            attribute.to_string()
        );
        test_attribute(&attribute, &expected_bytes, &VERSION_53_0)
    }

    #[test]
    fn test_module_packages() -> Result<()> {
        let attribute = Attribute::ModulePackages {
            name_index: 1,
            package_indexes: vec![42],
        };
        let expected_bytes = [0, 1, 0, 0, 0, 4, 0, 1, 0, 42];

        assert_eq!(
            "ModulePackages { name_index: 1, package_indexes: [42] }",
            attribute.to_string()
        );
        test_attribute(&attribute, &expected_bytes, &VERSION_53_0)
    }

    #[test]
    fn test_module_main_class_from_bytes_error() -> Result<()> {
        test_invalid_attribute_from_bytes_error("ModuleMainClass")
    }

    #[test]
    fn test_module_main_class() -> Result<()> {
        let attribute = Attribute::ModuleMainClass {
            name_index: 1,
            main_class_index: 42,
        };
        let expected_bytes = [0, 1, 0, 0, 0, 2, 0, 42];

        assert_eq!(
            "ModuleMainClass { name_index: 1, main_class_index: 42 }",
            attribute.to_string()
        );
        test_attribute(&attribute, &expected_bytes, &VERSION_53_0)
    }

    #[test]
    fn test_nest_host_from_bytes_error() -> Result<()> {
        test_invalid_attribute_from_bytes_error("NestHost")
    }

    #[test]
    fn test_nest_host() -> Result<()> {
        let attribute = Attribute::NestHost {
            name_index: 1,
            host_class_index: 42,
        };
        let expected_bytes = [0, 1, 0, 0, 0, 2, 0, 42];

        assert_eq!(
            "NestHost { name_index: 1, host_class_index: 42 }",
            attribute.to_string()
        );
        test_attribute(&attribute, &expected_bytes, &VERSION_55_0)
    }

    #[test]
    fn test_nest_members() -> Result<()> {
        let attribute = Attribute::NestMembers {
            name_index: 1,
            class_indexes: vec![42],
        };
        let expected_bytes = [0, 1, 0, 0, 0, 4, 0, 1, 0, 42];

        assert_eq!(
            "NestMembers { name_index: 1, class_indexes: [42] }",
            attribute.to_string()
        );
        test_attribute(&attribute, &expected_bytes, &VERSION_55_0)
    }

    #[test]
    fn test_record() -> Result<()> {
        let constant = Attribute::ConstantValue {
            name_index: 1,
            constant_value_index: 42,
        };
        let record = Record {
            name_index: 2,
            descriptor_index: 3,
            attributes: vec![constant.clone()],
        };
        let attribute = Attribute::Record {
            name_index: 4,
            records: vec![record],
        };
        let expected_bytes = [
            0, 4, 0, 0, 0, 16, 0, 1, 0, 2, 0, 3, 0, 1, 0, 1, 0, 0, 0, 2, 0, 42,
        ];

        let mut constant_pool = ConstantPool::default();
        constant_pool.add_utf8(constant.name())?;
        constant_pool.add_utf8("bar")?;
        constant_pool.add_utf8("test")?;
        constant_pool.add_utf8(attribute.name())?;

        assert!(attribute.valid_for_version(&VERSION_60_0));
        assert!(!attribute.valid_for_version(&VERSION_45_0));

        assert_eq!(
            "Record { name_index: 4, records: [Record { name_index: 2, descriptor_index: 3, attributes: [ConstantValue { name_index: 1, constant_value_index: 42 }] }] }",
            attribute.to_string()
        );

        let mut bytes = Vec::new();
        attribute.to_bytes(&mut bytes)?;
        assert_eq!(expected_bytes, &bytes[..]);
        let mut bytes = Cursor::new(expected_bytes.to_vec());
        assert_eq!(
            attribute,
            Attribute::from_bytes(&constant_pool, &mut bytes)?
        );
        Ok(())
    }

    #[test]
    fn test_permitted_subclasses() -> Result<()> {
        let attribute = Attribute::PermittedSubclasses {
            name_index: 1,
            class_indexes: vec![42],
        };
        let expected_bytes = [0, 1, 0, 0, 0, 4, 0, 1, 0, 42];

        assert_eq!(
            "PermittedSubclasses { name_index: 1, class_indexes: [42] }",
            attribute.to_string()
        );
        test_attribute(&attribute, &expected_bytes, &VERSION_61_0)
    }

    #[test]
    fn test_unknown() -> Result<()> {
        let attribute = Attribute::Unknown {
            name_index: 1,
            info: vec![0, 42],
        };
        let expected_bytes = [0, 1, 0, 0, 0, 2, 0, 42];

        assert_eq!(
            "Unknown { name_index: 1, info: [0, 42] }",
            attribute.to_string()
        );
        test_attribute(&attribute, &expected_bytes, &VERSION_45_3)
    }
}
