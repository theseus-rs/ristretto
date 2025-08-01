/// Errors that can occur when loading classes
#[derive(Debug, thiserror::Error)]
pub enum JavaError {
    /// `AbstractMethodError`
    /// See: <https://docs.oracle.com/en/java/javase/24/docs/api/java.base/java/lang/AbstractMethodError.html>
    #[error("{0}")]
    AbstractMethodError(String),
    /// `AccessControlException`
    /// See: <https://docs.oracle.com/en/java/javase/24/docs/api/java.base/java/security/AccessControlException.html>
    #[error("{0}")]
    AccessControlException(String),
    /// `ArithmeticException`
    /// See: <https://docs.oracle.com/en/java/javase/24/docs/api/java.base/java/lang/ArithmeticException.html>
    #[error("{0}")]
    ArithmeticException(String),
    /// `ArrayIndexOutOfBoundsException`
    /// See: <https://docs.oracle.com/en/java/javase/24/docs/api/java.base/java/lang/ArrayIndexOutOfBoundsException.html>
    #[error("Index {index} out of bounds for length {length}")]
    ArrayIndexOutOfBoundsException { index: i32, length: usize },
    /// `BootstrapMethodError`
    /// See: <https://docs.oracle.com/en/java/javase/24/docs/api/java.base/java/lang/BootstrapMethodError.html>
    #[error("{0}")]
    BootstrapMethodError(String),
    /// `ClassCastException`
    /// See: <https://docs.oracle.com/en/java/javase/24/docs/api/java.base/java/lang/ClassCastException.html>
    #[error("class {source_class_name} cannot be cast to class {target_class_name}")]
    ClassCastException {
        source_class_name: String,
        target_class_name: String,
    },
    /// `ClassFormatError`
    /// See: <https://docs.oracle.com/en/java/javase/24/docs/api/java.base/java/lang/ClassFormatError.html>
    #[error("{0}")]
    ClassFormatError(String),
    /// `ClassNotFoundException`
    /// See: <https://docs.oracle.com/en/java/javase/24/docs/api/java.base/java/lang/ClassNotFoundException.html>
    #[error("{0}")]
    ClassNotFoundException(String),
    /// `FileNotFoundException`
    /// See: <https://docs.oracle.com/en/java/javase/24/docs/api/java.base/java/io/FileNotFoundException.html>
    #[error("{0}")]
    FileNotFoundException(String),
    /// `IllegalAccessError`
    /// See: <https://docs.oracle.com/en/java/javase/24/docs/api/java.base/java/lang/IllegalAccessError.html>
    #[error("{0}")]
    IllegalAccessError(String),
    /// `IllegalArgumentException`
    /// See: <https://docs.oracle.com/en/java/javase/24/docs/api/java.base/java/lang/IllegalArgumentException.html>
    #[error("{0}")]
    IllegalArgumentException(String),
    /// `IncompatibleClassChangeError`
    /// See: <https://docs.oracle.com/en/java/javase/24/docs/api/java.base/java/lang/IncompatibleClassChangeError.html>
    #[error("{0}")]
    IncompatibleClassChangeError(String),
    /// `IndexOutOfBoundsException`
    /// See: <https://docs.oracle.com/en/java/javase/24/docs/api/java.base/java/lang/IndexOutOfBoundsException.html>
    #[error("Index: {index}, Size {size}")]
    IndexOutOfBoundsException { index: i32, size: i32 },
    /// `IOException`
    /// See: <https://docs.oracle.com/en/java/javase/24/docs/api/java.base/java/io/IOException.html>
    #[error("{0}")]
    IoException(String),
    /// `NoClassDefFoundError`
    /// See: <https://docs.oracle.com/en/java/javase/24/docs/api/java.base/java/lang/NoClassDefFoundError.html>
    #[error("{0}")]
    NoClassDefFoundError(String),
    /// `NoSuchMethodError`
    /// See: <https://docs.oracle.com/en/java/javase/24/docs/api/java.base/java/lang/NoSuchMethodError.html>
    #[error("{0}")]
    NoSuchMethodError(String),
    /// `NullPointerException`
    /// See: <https://docs.oracle.com/en/java/javase/24/docs/api/java.base/java/lang/NullPointerException.html>
    #[error("{0}")]
    NullPointerException(String),
    /// `RuntimeException`
    /// See: <https://docs.oracle.com/en/java/javase/24/docs/api/java.base/java/lang/RuntimeException.html>
    #[error("{0}")]
    RuntimeException(String),
    /// `UnsupportedOperationException`
    /// See: <https://docs.oracle.com/en/java/javase/24/docs/api/java.base/java/lang/UnsupportedOperationException.html>
    #[error("{0}")]
    UnsupportedOperationException(String),
}

impl JavaError {
    /// Get the Java class name for the error
    #[must_use]
    pub fn class_name(&self) -> &str {
        match self {
            JavaError::AbstractMethodError(_) => "java.lang.AbstractMethodError",
            JavaError::AccessControlException { .. } => "java.security.AccessControlException",
            JavaError::ArrayIndexOutOfBoundsException { .. } => {
                "java.lang.ArrayIndexOutOfBoundsException"
            }
            JavaError::ArithmeticException(_) => "java.lang.ArithmeticException",
            JavaError::BootstrapMethodError(_) => "java.lang.BootstrapMethodError",
            JavaError::ClassCastException { .. } => "java.lang.ClassCastException",
            JavaError::ClassFormatError(_) => "java.lang.ClassFormatError",
            JavaError::ClassNotFoundException(_) => "java.lang.ClassNotFoundException",
            JavaError::FileNotFoundException(_) => "java.io.FileNotFoundException",
            JavaError::IllegalAccessError(_) => "java.lang.IllegalAccessError",
            JavaError::IllegalArgumentException(_) => "java.lang.IllegalArgumentException",
            JavaError::IncompatibleClassChangeError(_) => "java.lang.IncompatibleClassChangeError",
            JavaError::IndexOutOfBoundsException { .. } => "java.lang.IndexOutOfBoundsException",
            JavaError::IoException(_) => "java.io.IOException",
            JavaError::NoClassDefFoundError(_) => "java.lang.NoClassDefFoundError",
            JavaError::NoSuchMethodError(_) => "java.lang.NoSuchMethodError",
            JavaError::NullPointerException(_) => "java.lang.NullPointerException",
            JavaError::RuntimeException(_) => "java.lang.RuntimeException",
            JavaError::UnsupportedOperationException(_) => {
                "java.lang.UnsupportedOperationException"
            }
        }
    }

    /// Get the error message
    #[must_use]
    pub fn message(&self) -> String {
        format!("{self}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abstract_method_error() {
        let error = JavaError::AbstractMethodError("foo".to_string());
        assert_eq!(error.class_name(), "java.lang.AbstractMethodError");
        assert_eq!(error.message(), "foo");
    }

    #[test]
    fn test_access_control_exception() {
        let error = JavaError::AccessControlException("foo".to_string());
        assert_eq!(error.class_name(), "java.security.AccessControlException");
        assert_eq!(error.message(), "foo");
    }

    #[test]
    fn test_arithmetic_exception() {
        let error = JavaError::ArithmeticException("division by zero".to_string());
        assert_eq!(error.class_name(), "java.lang.ArithmeticException");
        assert_eq!(error.message(), "division by zero");
    }

    #[test]
    fn test_array_index_out_of_bounds_exception() {
        let error = JavaError::ArrayIndexOutOfBoundsException {
            index: 5,
            length: 3,
        };
        assert_eq!(
            error.class_name(),
            "java.lang.ArrayIndexOutOfBoundsException"
        );
        assert_eq!(error.message(), "Index 5 out of bounds for length 3");
    }

    #[test]
    fn test_bootstrap_method_error() {
        let error = JavaError::BootstrapMethodError("foo".to_string());
        assert_eq!(error.class_name(), "java.lang.BootstrapMethodError");
        assert_eq!(error.message(), "foo");
    }

    #[test]
    fn test_class_cast_exception() {
        let error = JavaError::ClassCastException {
            source_class_name: "java.lang.String".to_string(),
            target_class_name: "java.lang.Integer".to_string(),
        };
        assert_eq!(error.class_name(), "java.lang.ClassCastException");
        assert_eq!(
            error.message(),
            "class java.lang.String cannot be cast to class java.lang.Integer"
        );
    }

    #[test]
    fn test_class_format_error() {
        let error = JavaError::ClassFormatError("invalid class format".to_string());
        assert_eq!(error.class_name(), "java.lang.ClassFormatError");
        assert_eq!(error.message(), "invalid class format");
    }

    #[test]
    fn test_class_not_found_error() {
        let error = JavaError::ClassNotFoundException("foo".to_string());
        assert_eq!(error.class_name(), "java.lang.ClassNotFoundException");
        assert_eq!(error.message(), "foo");
    }

    #[test]
    fn test_file_not_found_exception() {
        let error = JavaError::FileNotFoundException("foo".to_string());
        assert_eq!(error.class_name(), "java.io.FileNotFoundException");
        assert_eq!(error.message(), "foo");
    }

    #[test]
    fn test_illegal_access_error() {
        let error = JavaError::IllegalAccessError("foo".to_string());
        assert_eq!(error.class_name(), "java.lang.IllegalAccessError");
        assert_eq!(error.message(), "foo");
    }

    #[test]
    fn test_illegal_argument_exception() {
        let error = JavaError::IllegalArgumentException("invalid argument".to_string());
        assert_eq!(error.class_name(), "java.lang.IllegalArgumentException");
        assert_eq!(error.message(), "invalid argument");
    }

    #[test]
    fn test_index_out_of_bounds_exception() {
        let error = JavaError::IndexOutOfBoundsException { index: 5, size: 3 };
        assert_eq!(error.class_name(), "java.lang.IndexOutOfBoundsException");
        assert_eq!(error.message(), "Index: 5, Size 3");
    }

    #[test]
    fn test_incompatible_class_change_error() {
        let error = JavaError::IncompatibleClassChangeError("foo".to_string());
        assert_eq!(error.class_name(), "java.lang.IncompatibleClassChangeError");
        assert_eq!(error.message(), "foo");
    }

    #[test]
    fn test_io_exception() {
        let error = JavaError::IoException("foo".to_string());
        assert_eq!(error.class_name(), "java.io.IOException");
        assert_eq!(error.message(), "foo");
    }

    #[test]
    fn test_no_class_def_found_error() {
        let error = JavaError::NoClassDefFoundError("java.lang.String".to_string());
        assert_eq!(error.class_name(), "java.lang.NoClassDefFoundError");
        assert_eq!(error.message(), "java.lang.String");
    }

    #[test]
    fn test_no_such_method_error() {
        let error = JavaError::NoSuchMethodError("foo".to_string());
        assert_eq!(error.class_name(), "java.lang.NoSuchMethodError");
        assert_eq!(error.message(), "foo");
    }

    #[test]
    fn test_null_pointer_exception() {
        let error = JavaError::NullPointerException("null".to_string());
        assert_eq!(error.class_name(), "java.lang.NullPointerException");
        assert_eq!(error.message(), "null");
    }

    #[test]
    fn test_runtime_exception() {
        let error = JavaError::RuntimeException("foo".to_string());
        assert_eq!(error.class_name(), "java.lang.RuntimeException");
        assert_eq!(error.message(), "foo");
    }

    #[test]
    fn test_unsupported_operation_exception() {
        let error = JavaError::UnsupportedOperationException("foo".to_string());
        assert_eq!(
            error.class_name(),
            "java.lang.UnsupportedOperationException"
        );
        assert_eq!(error.message(), "foo");
    }
}
