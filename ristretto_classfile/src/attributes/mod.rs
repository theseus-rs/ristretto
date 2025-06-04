//! # Attributes Module
//!
//! This module contains definitions and implementations for the various attributes that can
//! appear in Java class files according to the JVM specification. Attributes provide additional
//! information about classes, fields, methods, and code within class files.
//!
//! Attributes are used by the JVM for execution, verification, debugging, and other purposes.
//! Each attribute has a specific structure and semantic meaning defined by the JVM specification.

/// Represents a Java annotation structure used in runtime and class file annotations.
mod annotation;

/// Defines elements that can appear within Java annotations.
mod annotation_element;

/// Represents name-value pairs within Java annotations.
mod annotation_value_pair;

/// Defines the structure for array type information.
mod array_type;

/// Core attribute structure that serves as the base for all class file attributes.
mod attribute;

/// Represents bootstrap method entries used for invokedynamic instructions.
mod bootstrap_method;

/// Defines exception table entries for try-catch blocks in method code.
mod exception_table_entry;

/// Represents module exports declarations for Java modules.
mod exports;

/// Defines access flags for module exports.
mod exports_flags;

/// Contains information about inner classes and their relationship to the outer class.
mod inner_class;

/// Represents JVM bytecode instructions within method code.
mod instruction;

/// Maps bytecode offsets to source code line numbers for debugging.
mod line_number;

/// Provides information about local variables within method code.
mod local_variable_table;

/// Represents targeting information for local variables in type annotations.
mod local_variable_target;

/// Provides type information for local variables within method code.
mod local_variable_type_table;

/// Defines the maximum number of local variables for a method.
mod max_locals;

/// Defines the maximum operand stack size for a method.
mod max_stack;

/// Contains information about method parameters.
mod method_parameter;

/// Defines access flags for Java modules.
mod module_access_flags;

/// Defines access flags for nested classes.
mod nested_class_access_flags;

/// Utilities for working with bytecode offsets.
mod offset_utils;

/// Represents module opens declarations for Java modules.
mod opens;

/// Defines access flags for module opens.
mod opens_flags;

/// Contains annotation information for method parameters.
mod parameter_annotation;

/// Represents module provides declarations for Java modules.
mod provides;

/// Contains information about record components for Java records.
mod record;

/// Represents module requires declarations for Java modules.
mod requires;

/// Defines access flags for module requires.
mod requires_flags;

/// Represents stack frame information for verification and debugging.
mod stack_frame;

/// Defines path information for type annotations.
mod target_path;

/// Defines the target type for type annotations.
mod target_type;

/// Represents type annotations that can appear in class files.
mod type_annotation;

/// Defines verification types used during bytecode verification.
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
pub use instruction::Instruction;
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
