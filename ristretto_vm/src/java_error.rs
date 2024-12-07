/// Errors that can occur when loading classes
#[derive(Debug, thiserror::Error)]
pub enum JavaError {
    /// `ArithmeticException`
    /// See: <https://docs.oracle.com/en/java/javase/23/docs/api/java.base/java/lang/ArithmeticException.html>
    #[error("{0}")]
    ArithmeticException(String),
    /// `ArrayIndexOutOfBoundsException`
    /// See: <https://docs.oracle.com/en/java/javase/23/docs/api/java.base/java/lang/ArrayIndexOutOfBoundsException.html>
    #[error("Index {index} out of bounds for length {length}")]
    ArrayIndexOutOfBoundsException { index: usize, length: usize },
    /// `ClassCastException`
    /// See: <https://docs.oracle.com/en/java/javase/23/docs/api/java.base/java/lang/ClassCastException.html>
    #[error("class {source_class_name} cannot be cast to class {target_class_name}")]
    ClassCastException {
        source_class_name: String,
        target_class_name: String,
    },
    /// `ClassFormatError`
    /// See: <https://docs.oracle.com/en/java/javase/23/docs/api/java.base/java/lang/ClassFormatError.html>
    #[error("{0}")]
    ClassFormatError(String),
    /// `IllegalArgumentException`
    /// See: <https://docs.oracle.com/en/java/javase/23/docs/api/java.base/java/lang/IllegalArgumentException.html>
    #[error("{0}")]
    IllegalArgumentException(String),
    /// `IndexOutOfBoundsException`
    /// See: <https://docs.oracle.com/en/java/javase/23/docs/api/java.base/java/lang/IndexOutOfBoundsException.html>
    #[error("Index: {index}, Size {size}")]
    IndexOutOfBoundsException { index: i32, size: i32 },
    /// `NoClassDefFoundError`
    /// See: <https://docs.oracle.com/en/java/javase/23/docs/api/java.base/java/lang/NoClassDefFoundError.html>
    #[error("{0}")]
    NoClassDefFoundError(String),
    /// `NullPointerException`
    /// See: <https://docs.oracle.com/en/java/javase/23/docs/api/java.base/java/lang/NullPointerException.html>
    #[error("{0}")]
    NullPointerException(String),
}

impl JavaError {
    /// Get the Java class name for the error
    #[must_use]
    pub fn class_name(&self) -> &str {
        match self {
            JavaError::ArrayIndexOutOfBoundsException { .. } => {
                "java/lang/ArrayIndexOutOfBoundsException"
            }
            JavaError::ArithmeticException(_) => "java/lang/ArithmeticException",
            JavaError::ClassCastException { .. } => "java/lang/ClassCastException",
            JavaError::ClassFormatError(_) => "java/lang/ClassFormatError",
            JavaError::IllegalArgumentException(_) => "java/lang/IllegalArgumentException",
            JavaError::IndexOutOfBoundsException { .. } => "java/lang/IndexOutOfBoundsException",
            JavaError::NoClassDefFoundError(_) => "java/lang/NoClassDefFoundError",
            JavaError::NullPointerException(_) => "java/lang/NullPointerException",
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
    fn test_arithmetic_exception() {
        let error = JavaError::ArithmeticException("division by zero".to_string());
        assert_eq!(error.class_name(), "java/lang/ArithmeticException");
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
            "java/lang/ArrayIndexOutOfBoundsException"
        );
        assert_eq!(error.message(), "Index 5 out of bounds for length 3");
    }

    #[test]
    fn test_class_cast_exception() {
        let error = JavaError::ClassCastException {
            source_class_name: "java.lang.String".to_string(),
            target_class_name: "java.lang.Integer".to_string(),
        };
        assert_eq!(error.class_name(), "java/lang/ClassCastException");
        assert_eq!(
            error.message(),
            "class java.lang.String cannot be cast to class java.lang.Integer"
        );
    }

    #[test]
    fn test_class_format_error() {
        let error = JavaError::ClassFormatError("invalid class format".to_string());
        assert_eq!(error.class_name(), "java/lang/ClassFormatError");
        assert_eq!(error.message(), "invalid class format");
    }

    #[test]
    fn test_illegal_argument_exception() {
        let error = JavaError::IllegalArgumentException("invalid argument".to_string());
        assert_eq!(error.class_name(), "java/lang/IllegalArgumentException");
        assert_eq!(error.message(), "invalid argument");
    }

    #[test]
    fn test_index_out_of_bounds_exception() {
        let error = JavaError::IndexOutOfBoundsException { index: 5, size: 3 };
        assert_eq!(error.class_name(), "java/lang/IndexOutOfBoundsException");
        assert_eq!(error.message(), "Index: 5, Size 3");
    }

    #[test]
    fn test_no_class_def_found_error() {
        let error = JavaError::NoClassDefFoundError("java/lang/String".to_string());
        assert_eq!(error.class_name(), "java/lang/NoClassDefFoundError");
        assert_eq!(error.message(), "java/lang/String");
    }

    #[test]
    fn test_null_pointer_exception() {
        let error = JavaError::NullPointerException("null".to_string());
        assert_eq!(error.class_name(), "java/lang/NullPointerException");
        assert_eq!(error.message(), "null");
    }
}
