//! # Attributes Module
//!
//! This module provides definitions and parsing capabilities for the various attributes
//! that can appear in Java class files, as specified by the Java Virtual Machine (JVM)
//! Specification. Attributes are a fundamental part of the class file format, offering
//! extensible metadata about classes, fields, methods, and code.
//!
//! ## Overview
//!
//! Attributes are used to store information that is not part of the basic structure of
//! a class file element but is essential for its correct interpretation, execution,
//! or for tools like debuggers and compilers. Each attribute has a name, which is an
//! index into the constant pool (pointing to a `CONSTANT_Utf8_info` structure), and
//! a sequence of bytes representing its value. The structure and meaning of these bytes
//! are specific to the attribute type.
//!
//! This module defines Rust structs corresponding to these attributes, facilitating
//! their representation and manipulation.
//!
//! ## Common Attributes
//!
//! Some of the well-known attributes include:
//! - `Code`: Contains the JVM bytecode for a method.
//! - `LineNumberTable`: Maps bytecode offsets to source file line numbers for debugging.
//! - `SourceFile`: Specifies the original source file name.
//! - `ConstantValue`: Indicates the constant value for a static final field.
//! - `Exceptions`: Lists the checked exceptions a method may throw.
//! - `InnerClasses`: Describes nested classes.
//! - `RuntimeVisibleAnnotations`: Stores annotations that are visible at runtime.
//! - `BootstrapMethods`: Used by `invokedynamic` instructions.
//!
//! For a comprehensive list and detailed descriptions, refer to the
//! [JVM Specification, Chapter 4: The class File Format](https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7).

/// Represents a Java annotation, a form of metadata that can be added to Java code elements.
/// Annotations are used by the compiler, runtime, or other tools.
mod annotation;

/// Defines the various kinds of elements that can appear within a Java annotation's value.
mod annotation_element;

/// Represents a name-value pair within a Java annotation, forming its members.
mod annotation_value_pair;

/// Defines the structure for representing array type information, often used in signatures or annotations.
mod array_type;

/// The core `Attribute` structure, serving as a base for all specific class file attributes.
/// It typically holds the attribute name index and its raw byte data.
mod attribute;

/// Represents entries in the `BootstrapMethods` attribute, used for `invokedynamic` instructions.
mod bootstrap_method;

/// Defines entries in an exception table, part of the `Code` attribute, for try-catch blocks.
mod exception_table_entry;

/// Represents `exports` declarations in a `Module` attribute, specifying packages exported by a Java module.
mod exports;

/// Defines access flags (e.g., `ACC_SYNTHETIC`, `ACC_MANDATED`) for module exports.
mod exports_flags;

/// Contains information about inner classes and their relationship to any outer class, part of the `InnerClasses` attribute.
mod inner_class;

/// Represents individual JVM bytecode instructions within the `Code` attribute of a method.
mod instruction;

/// Maps bytecode offsets to source code line numbers, part of the `LineNumberTable` attribute, crucial for debugging.
mod line_number;

/// Provides information about local variables within a method's scope, part of the `LocalVariableTable` attribute.
mod local_variable_table;

/// Represents targeting information for local variables in type annotations (e.g., `RuntimeVisibleTypeAnnotations`).
mod local_variable_target;

/// Provides type information (signatures) for local variables, part of the `LocalVariableTypeTable` attribute.
mod local_variable_type_table;

/// Defines the maximum number of local variables (including parameters) for a method, part of the `Code` attribute.
mod max_locals;

/// Defines the maximum operand stack size required by a method at any point during its execution, part of the `Code` attribute.
mod max_stack;

/// Contains information about method parameters, part of the `MethodParameters` attribute.
mod method_parameter;

/// Defines access flags for Java modules (e.g., `ACC_OPEN`, `ACC_SYNTHETIC`, `ACC_MANDATED`).
mod module_access_flags;

/// Defines access flags for nested (inner) classes (e.g., `ACC_PUBLIC`, `ACC_STATIC`).
mod nested_class_access_flags;

/// Utilities for working with bytecode offsets, often used in parsing or manipulating code attributes.
mod offset_utils;

/// Represents `opens` declarations in a `Module` attribute, specifying packages opened by a Java module.
mod opens;

/// Defines access flags (e.g., `ACC_SYNTHETIC`, `ACC_MANDATED`) for module opens.
mod opens_flags;

/// Contains annotation information for method parameters, part of `RuntimeVisibleParameterAnnotations` or `RuntimeInvisibleParameterAnnotations`.
mod parameter_annotation;

/// Represents `provides` declarations in a `Module` attribute, specifying services provided by a Java module.
mod provides;

/// Contains information about record components for Java records, part of the `Record` attribute.
mod record;

/// Represents `requires` declarations in a `Module` attribute, specifying dependencies of a Java module.
mod requires;

/// Defines access flags (e.g., `ACC_TRANSITIVE`, `ACC_STATIC_PHASE`, `ACC_SYNTHETIC`, `ACC_MANDATED`) for module requires.
mod requires_flags;

/// Represents stack frame information used for bytecode verification and debugging, part of the `StackMapTable` attribute.
mod stack_frame;

/// Defines the path to a type argument or a wildcard bound within a `TypeAnnotation`.
mod target_path;

/// Defines the target type (e.g., field, method return) for a `TypeAnnotation`.
mod target_type;

/// Represents type annotations that can appear on various program elements, enhancing Java's type system.
mod type_annotation;

/// Defines verification types used during the bytecode verification process, often found in `StackMapTable` attributes.
mod verification_type;

pub use annotation::Annotation;
pub use annotation_element::AnnotationElement;
pub use annotation_value_pair::AnnotationValuePair;
pub use array_type::ArrayType;
pub use attribute::Attribute;
pub use bootstrap_method::BootstrapMethod;
pub use exception_table_entry::ExceptionTableEntry;
pub use exports::Exports;
pub use exports_flags::ExportsFlags;
pub use inner_class::InnerClass;
pub use instruction::{Instruction, LookupSwitch, TableSwitch};
pub use line_number::LineNumber;
pub use local_variable_table::LocalVariableTable;
pub use local_variable_target::LocalVariableTarget;
pub use local_variable_type_table::LocalVariableTypeTable;
pub use max_locals::MaxLocals;
pub use max_stack::MaxStack;
pub use method_parameter::MethodParameter;
pub use module_access_flags::ModuleAccessFlags;
pub use nested_class_access_flags::NestedClassAccessFlags;
pub use opens::Opens;
pub use opens_flags::OpensFlags;
pub use parameter_annotation::ParameterAnnotation;
pub use provides::Provides;
pub use record::Record;
pub use requires::Requires;
pub use requires_flags::RequiresFlags;
pub use stack_frame::StackFrame;
pub use target_path::TargetPath;
pub use target_type::TargetType;
pub use type_annotation::TypeAnnotation;
pub use verification_type::VerificationType;

// Example for Annotation (assuming AnnotationValuePair and AnnotationElement are defined and accessible)
// To make this example runnable, you'd need to define dummy versions or import actual ones.
// For demonstration, we'll assume they exist.
//
// /// ```
// /// use ristretto_classfile::attributes::{Annotation, AnnotationValuePair, AnnotationElement};
// ///
// /// // Assuming AnnotationElement::String { const_value_index: u16 } exists
// /// // and AnnotationValuePair { name_index: u16, value: AnnotationElement } exists
// ///
// /// let annotation = Annotation {
// ///     type_index: 10, // Index in constant pool for annotation type
// ///     num_element_value_pairs: 1,
// ///     element_value_pairs: vec![
// ///         AnnotationValuePair {
// ///             name_index: 11, // Index for "value"
// ///             value: AnnotationElement::String { const_value_index: 12 } // Index for "Hello"
// ///         }
// ///     ]
// /// };
// ///
// /// assert_eq!(annotation.type_index, 10);
// /// assert_eq!(annotation.element_value_pairs.len(), 1);
// /// ```

// Example for LineNumber
//
// /// ```
// /// use ristretto_classfile::attributes::LineNumber;
// ///
// /// let line_info = LineNumber {
// ///     start_pc: 100,    // Bytecode offset
// ///     line_number: 42,  // Source line number
// /// };
// ///
// /// assert_eq!(line_info.start_pc, 100);
// /// assert_eq!(line_info.line_number, 42);
// /// ```

// Example for MaxStack
//
// /// ```
// /// use ristretto_classfile::attributes::MaxStack;
// ///
// /// let max_stack_attr = MaxStack {
// ///     max_stack: 16, // Maximum operand stack size
// /// };
// ///
// /// assert_eq!(max_stack_attr.max_stack, 16);
// /// ```

// Example for Attribute (generic structure)
//
// /// ```
// /// use ristretto_classfile::attributes::Attribute;
// ///
// /// // A generic attribute, e.g., a custom one or one whose content is opaque here.
// /// let generic_attribute = Attribute {
// ///     attribute_name_index: 5, // Index in constant pool for attribute name
// ///     attribute_length: 4,
// ///     info: vec![0xDE, 0xAD, 0xBE, 0xEF], // Raw byte data
// /// };
// ///
// /// assert_eq!(generic_attribute.attribute_name_index, 5);
// /// assert_eq!(generic_attribute.info.len(), 4);
// /// ```
