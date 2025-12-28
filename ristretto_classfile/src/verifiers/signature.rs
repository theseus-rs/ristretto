//! Generic signature grammar validation according to [JVMS §4.7.9.1](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.9.1).
//!
//! This module validates generic signatures for classes, methods, and fields.
//! It parses and validates signatures according to the grammar defined in the
//! Java Virtual Machine Specification.

use crate::verifiers::error::Result;
use crate::verifiers::error::VerifyError::VerificationError;
use std::collections::HashSet;

/// Represents a parsed type parameter with its name and bounds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TypeParameter {
    pub name: String,
    pub class_bound: Option<String>,
    pub interface_bounds: Vec<String>,
}

/// Context for signature validation, tracking declared type variables.
#[derive(Debug, Default)]
pub(crate) struct SignatureContext {
    /// Type variables declared in this context (class or method type parameters)
    declared_type_variables: HashSet<String>,
}

impl SignatureContext {
    /// Create a new empty signature context.
    #[must_use]
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Add a declared type variable to the context.
    pub fn add_type_variable(&mut self, name: &str) {
        self.declared_type_variables.insert(name.to_string());
    }

    /// Check if a type variable is declared in this context.
    #[must_use]
    pub fn has_type_variable(&self, name: &str) -> bool {
        self.declared_type_variables.contains(name)
    }
}

/// Parser for generic signatures.
struct SignatureParser<'a> {
    input: &'a str,
    pos: usize,
    context: SignatureContext,
}

impl<'a> SignatureParser<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            input,
            pos: 0,
            context: SignatureContext::new(),
        }
    }

    fn peek(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.peek()?;
        self.pos += c.len_utf8();
        Some(c)
    }

    fn expect(&mut self, expected: char) -> Result<()> {
        match self.advance() {
            Some(c) if c == expected => Ok(()),
            Some(c) => Err(VerificationError {
                context: "Signature".to_string(),
                message: format!(
                    "Expected '{expected}' but found '{c}' at position {}",
                    self.pos
                ),
            }),
            None => Err(VerificationError {
                context: "Signature".to_string(),
                message: format!("Expected '{expected}' but found end of input"),
            }),
        }
    }

    fn is_eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    /// Parse an identifier (used for type variable names, class names, etc.)
    fn parse_identifier(&mut self) -> Result<String> {
        let start = self.pos;
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' || c == '$' {
                self.advance();
            } else {
                break;
            }
        }
        if start == self.pos {
            return Err(VerificationError {
                context: "Signature".to_string(),
                message: format!("Expected identifier at position {start}"),
            });
        }
        Ok(self.input[start..self.pos].to_string())
    }

    /// Parse a class name (internal form with '/')
    fn parse_class_name(&mut self) -> Result<String> {
        let start = self.pos;
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' || c == '$' || c == '/' {
                self.advance();
            } else {
                break;
            }
        }
        if start == self.pos {
            return Err(VerificationError {
                context: "Signature".to_string(),
                message: format!("Expected class name at position {start}"),
            });
        }
        Ok(self.input[start..self.pos].to_string())
    }

    /// Parse type parameters: `<TypeParameter+>`
    fn parse_type_parameters(&mut self) -> Result<Vec<TypeParameter>> {
        self.expect('<')?;
        let mut params = Vec::new();
        while self.peek() != Some('>') {
            let param = self.parse_type_parameter()?;
            self.context.add_type_variable(&param.name);
            params.push(param);
        }
        self.expect('>')?;
        if params.is_empty() {
            return Err(VerificationError {
                context: "Signature".to_string(),
                message: "Type parameters cannot be empty".to_string(),
            });
        }
        Ok(params)
    }

    /// Parse a single type parameter: `Identifier ClassBound InterfaceBound*`
    fn parse_type_parameter(&mut self) -> Result<TypeParameter> {
        let name = self.parse_identifier()?;
        self.expect(':')?;

        // ClassBound is optional but the colon is required
        let class_bound = if self.peek() != Some(':') && self.peek() != Some('>') {
            Some(self.parse_field_type_signature()?)
        } else {
            None
        };

        let mut interface_bounds = Vec::new();
        while self.peek() == Some(':') {
            self.advance(); // consume ':'
            interface_bounds.push(self.parse_field_type_signature()?);
        }

        Ok(TypeParameter {
            name,
            class_bound,
            interface_bounds,
        })
    }

    /// Parse a field type signature
    fn parse_field_type_signature(&mut self) -> Result<String> {
        let start = self.pos;
        match self.peek() {
            Some('L') => self.parse_class_type_signature()?,
            Some('[') => self.parse_array_type_signature()?,
            Some('T') => self.parse_type_variable_signature()?,
            Some(c) => {
                return Err(VerificationError {
                    context: "Signature".to_string(),
                    message: format!("Invalid field type signature starting with '{c}'"),
                });
            }
            None => {
                return Err(VerificationError {
                    context: "Signature".to_string(),
                    message: "Unexpected end of signature".to_string(),
                });
            }
        }
        Ok(self.input[start..self.pos].to_string())
    }

    /// Parse a class type signature: `L ClassName TypeArguments? (. Identifier TypeArguments?)* ;`
    fn parse_class_type_signature(&mut self) -> Result<()> {
        self.expect('L')?;
        self.parse_class_name()?;

        // Optional type arguments
        if self.peek() == Some('<') {
            self.parse_type_arguments()?;
        }

        // Inner class suffixes
        while self.peek() == Some('.') {
            self.advance();
            self.parse_identifier()?;
            if self.peek() == Some('<') {
                self.parse_type_arguments()?;
            }
        }

        self.expect(';')?;
        Ok(())
    }

    /// Parse type arguments: `< TypeArgument+ >`
    fn parse_type_arguments(&mut self) -> Result<()> {
        self.expect('<')?;
        let mut count = 0;
        while self.peek() != Some('>') {
            self.parse_type_argument()?;
            count += 1;
        }
        self.expect('>')?;
        if count == 0 {
            return Err(VerificationError {
                context: "Signature".to_string(),
                message: "Type arguments cannot be empty".to_string(),
            });
        }
        Ok(())
    }

    /// Parse a type argument: `* | (+ | -)? FieldTypeSignature`
    fn parse_type_argument(&mut self) -> Result<()> {
        match self.peek() {
            Some('*') => {
                self.advance();
            }
            Some('+' | '-') => {
                self.advance();
                self.parse_field_type_signature()?;
            }
            _ => {
                self.parse_field_type_signature()?;
            }
        }
        Ok(())
    }

    /// Parse an array type signature: `[ TypeSignature`
    fn parse_array_type_signature(&mut self) -> Result<()> {
        self.expect('[')?;
        self.parse_type_signature()?;
        Ok(())
    }

    /// Parse a type variable signature: `T Identifier ;`
    fn parse_type_variable_signature(&mut self) -> Result<()> {
        self.expect('T')?;
        let name = self.parse_identifier()?;
        self.expect(';')?;

        // Validate that the type variable is declared
        if !self.context.has_type_variable(&name) {
            return Err(VerificationError {
                context: "Signature".to_string(),
                message: format!("Undefined type variable '{name}'"),
            });
        }
        Ok(())
    }

    /// Parse a type signature (used in method signatures)
    fn parse_type_signature(&mut self) -> Result<()> {
        if let Some('B' | 'C' | 'D' | 'F' | 'I' | 'J' | 'S' | 'Z') = self.peek() {
            self.advance();
            Ok(())
        } else {
            self.parse_field_type_signature()?;
            Ok(())
        }
    }

    /// Parse a return type: `TypeSignature | V`
    fn parse_return_type(&mut self) -> Result<()> {
        if self.peek() == Some('V') {
            self.advance();
            Ok(())
        } else {
            self.parse_type_signature()
        }
    }

    /// Parse throws signature: `^ (ClassTypeSignature | TypeVariableSignature)`
    fn parse_throws_signature(&mut self) -> Result<()> {
        self.expect('^')?;
        match self.peek() {
            Some('L') => self.parse_class_type_signature(),
            Some('T') => self.parse_type_variable_signature(),
            Some(c) => Err(VerificationError {
                context: "Signature".to_string(),
                message: format!("Invalid throws signature starting with '{c}'"),
            }),
            None => Err(VerificationError {
                context: "Signature".to_string(),
                message: "Unexpected end of throws signature".to_string(),
            }),
        }
    }
}

/// Verify a class signature.
///
/// Grammar: `TypeParameters? SuperclassSignature SuperinterfaceSignature*`
///
/// # Errors
/// Returns an error if the signature is malformed or contains undefined type variables.
pub(crate) fn verify_class_signature(signature: &str) -> Result<()> {
    if signature.is_empty() {
        return Err(VerificationError {
            context: "Signature".to_string(),
            message: "Class signature cannot be empty".to_string(),
        });
    }

    let mut parser = SignatureParser::new(signature);

    // Optional type parameters
    if parser.peek() == Some('<') {
        parser.parse_type_parameters()?;
    }

    // Superclass signature (required)
    parser.parse_class_type_signature()?;

    // Zero or more superinterface signatures
    while !parser.is_eof() {
        parser.parse_class_type_signature()?;
    }

    Ok(())
}

/// Verify a method signature.
///
/// Grammar: `TypeParameters? ( TypeSignature* ) ReturnType ThrowsSignature*`
///
/// # Errors
/// Returns an error if the signature is malformed or contains undefined type variables.
pub(crate) fn verify_method_signature(signature: &str) -> Result<()> {
    if signature.is_empty() {
        return Err(VerificationError {
            context: "Signature".to_string(),
            message: "Method signature cannot be empty".to_string(),
        });
    }

    let mut parser = SignatureParser::new(signature);

    // Optional type parameters
    if parser.peek() == Some('<') {
        parser.parse_type_parameters()?;
    }

    // Parameter types
    parser.expect('(')?;
    while parser.peek() != Some(')') {
        parser.parse_type_signature()?;
    }
    parser.expect(')')?;

    // Return type
    parser.parse_return_type()?;

    // Throws signatures
    while parser.peek() == Some('^') {
        parser.parse_throws_signature()?;
    }

    if !parser.is_eof() {
        return Err(VerificationError {
            context: "Signature".to_string(),
            message: format!(
                "Unexpected characters at end of method signature: '{}'",
                &signature[parser.pos..]
            ),
        });
    }

    Ok(())
}

/// Verify a field signature.
///
/// Grammar: `FieldTypeSignature`
///
/// # Errors
/// Returns an error if the signature is malformed or contains undefined type variables.
pub(crate) fn verify_field_signature(signature: &str, class_type_params: &[String]) -> Result<()> {
    if signature.is_empty() {
        return Err(VerificationError {
            context: "Signature".to_string(),
            message: "Field signature cannot be empty".to_string(),
        });
    }

    let mut parser = SignatureParser::new(signature);

    // Add class type parameters to context
    for param in class_type_params {
        parser.context.add_type_variable(param);
    }

    parser.parse_field_type_signature()?;

    if !parser.is_eof() {
        return Err(VerificationError {
            context: "Signature".to_string(),
            message: format!(
                "Unexpected characters at end of field signature: '{}'",
                &signature[parser.pos..]
            ),
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Class signature tests

    #[test]
    fn test_simple_class_signature() {
        assert!(verify_class_signature("Ljava/lang/Object;").is_ok());
    }

    #[test]
    fn test_class_signature_with_superinterface() {
        assert!(verify_class_signature("Ljava/lang/Object;Ljava/io/Serializable;").is_ok());
    }

    #[test]
    fn test_class_signature_with_type_parameters() {
        assert!(verify_class_signature("<T:Ljava/lang/Object;>Ljava/lang/Object;").is_ok());
    }

    #[test]
    fn test_class_signature_with_multiple_type_parameters() {
        assert!(
            verify_class_signature("<K:Ljava/lang/Object;V:Ljava/lang/Object;>Ljava/lang/Object;")
                .is_ok()
        );
    }

    #[test]
    fn test_class_signature_with_parameterized_superclass() {
        assert!(
            verify_class_signature("<E:Ljava/lang/Object;>Ljava/util/AbstractList<TE;>;").is_ok()
        );
    }

    #[test]
    fn test_class_signature_with_interface_bound() {
        assert!(
            verify_class_signature(
                "<T:Ljava/lang/Object;:Ljava/io/Serializable;>Ljava/lang/Object;"
            )
            .is_ok()
        );
    }

    #[test]
    fn test_class_signature_with_no_class_bound() {
        // Type parameter with only interface bounds (no class bound)
        assert!(verify_class_signature("<T::Ljava/io/Serializable;>Ljava/lang/Object;").is_ok());
    }

    #[test]
    fn test_class_signature_empty() {
        let result = verify_class_signature("");
        assert!(result.is_err());
        if let Err(VerificationError { message, .. }) = result {
            assert!(message.contains("empty"));
        }
    }

    #[test]
    fn test_class_signature_invalid_no_superclass() {
        let result = verify_class_signature("<T:Ljava/lang/Object;>");
        assert!(result.is_err());
    }

    #[test]
    fn test_class_signature_empty_type_params() {
        let result = verify_class_signature("<>Ljava/lang/Object;");
        assert!(result.is_err());
    }

    // Method signature tests

    #[test]
    fn test_simple_method_signature() {
        assert!(verify_method_signature("()V").is_ok());
    }

    #[test]
    fn test_method_signature_with_params() {
        assert!(verify_method_signature("(II)I").is_ok());
    }

    #[test]
    fn test_method_signature_with_object_params() {
        assert!(verify_method_signature("(Ljava/lang/String;)Ljava/lang/String;").is_ok());
    }

    #[test]
    fn test_method_signature_with_type_parameters() {
        assert!(verify_method_signature("<T:Ljava/lang/Object;>(TT;)TT;").is_ok());
    }

    #[test]
    fn test_method_signature_with_throws() {
        assert!(verify_method_signature("()V^Ljava/io/IOException;").is_ok());
    }

    #[test]
    fn test_method_signature_with_multiple_throws() {
        assert!(verify_method_signature("()V^Ljava/io/IOException;^Ljava/lang/Exception;").is_ok());
    }

    #[test]
    fn test_method_signature_with_type_variable_throws() {
        assert!(verify_method_signature("<E:Ljava/lang/Exception;>()V^TE;").is_ok());
    }

    #[test]
    fn test_method_signature_with_array_param() {
        assert!(verify_method_signature("([Ljava/lang/String;)V").is_ok());
    }

    #[test]
    fn test_method_signature_with_wildcard() {
        assert!(verify_method_signature("(Ljava/util/List<*>;)V").is_ok());
    }

    #[test]
    fn test_method_signature_with_extends_wildcard() {
        assert!(verify_method_signature("(Ljava/util/List<+Ljava/lang/Number;>;)V").is_ok());
    }

    #[test]
    fn test_method_signature_with_super_wildcard() {
        assert!(verify_method_signature("(Ljava/util/List<-Ljava/lang/Number;>;)V").is_ok());
    }

    #[test]
    fn test_method_signature_empty() {
        let result = verify_method_signature("");
        assert!(result.is_err());
    }

    #[test]
    fn test_method_signature_undefined_type_variable() {
        let result = verify_method_signature("(TT;)V");
        assert!(result.is_err());
        if let Err(VerificationError { message, .. }) = result {
            assert!(message.contains("Undefined type variable"));
        }
    }

    // Field signature tests

    #[test]
    fn test_simple_field_signature() {
        assert!(verify_field_signature("Ljava/lang/String;", &[]).is_ok());
    }

    #[test]
    fn test_field_signature_parameterized() {
        assert!(verify_field_signature("Ljava/util/List<Ljava/lang/String;>;", &[]).is_ok());
    }

    #[test]
    fn test_field_signature_with_type_variable() {
        assert!(verify_field_signature("TT;", &["T".to_string()]).is_ok());
    }

    #[test]
    fn test_field_signature_array() {
        assert!(verify_field_signature("[Ljava/lang/String;", &[]).is_ok());
    }

    #[test]
    fn test_field_signature_nested_array() {
        assert!(verify_field_signature("[[Ljava/lang/String;", &[]).is_ok());
    }

    #[test]
    fn test_field_signature_primitive_array() {
        assert!(verify_field_signature("[I", &[]).is_ok());
    }

    #[test]
    fn test_field_signature_empty() {
        let result = verify_field_signature("", &[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_field_signature_undefined_type_variable() {
        let result = verify_field_signature("TT;", &[]);
        assert!(result.is_err());
        if let Err(VerificationError { message, .. }) = result {
            assert!(message.contains("Undefined type variable"));
        }
    }

    #[test]
    fn test_field_signature_inner_class() {
        assert!(
            verify_field_signature(
                "Ljava/util/Map<TK;TV;>.Entry<TK;TV;>;",
                &["K".to_string(), "V".to_string()]
            )
            .is_ok()
        );
    }

    // Type parameter context tests

    #[test]
    fn test_signature_context_add_and_check() {
        let mut ctx = SignatureContext::new();
        assert!(!ctx.has_type_variable("T"));
        ctx.add_type_variable("T");
        assert!(ctx.has_type_variable("T"));
    }

    // Edge cases

    #[test]
    fn test_class_signature_with_nested_generics() {
        assert!(
            verify_class_signature(
                "<T:Ljava/lang/Object;>Ljava/util/HashMap<Ljava/lang/String;Ljava/util/List<TT;>;>;"
            )
            .is_ok()
        );
    }

    #[test]
    fn test_method_signature_complex() {
        assert!(verify_method_signature(
            "<T:Ljava/lang/Object;U:Ljava/lang/Object;>(TT;TU;Ljava/util/Map<TT;TU;>;)Ljava/util/List<TT;>;"
        ).is_ok());
    }
}
