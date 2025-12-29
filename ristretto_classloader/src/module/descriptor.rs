//! Module descriptor representation.
//!
//! This module provides types for representing module descriptors as defined by the JVMS.

use crate::module::error::{ModuleError, Result};
use ristretto_classfile::Constant;
use ristretto_classfile::attributes::Attribute;
use ristretto_classfile::{ClassFile, ConstantPool};
use std::collections::BTreeSet;
use std::fmt;

// Re-export flags from ristretto_classfile for public API
pub use ristretto_classfile::attributes::ExportsFlags;
pub use ristretto_classfile::attributes::ModuleAccessFlags as ModuleFlags;
pub use ristretto_classfile::attributes::OpensFlags;
pub use ristretto_classfile::attributes::RequiresFlags;

/// A module requires directive.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Requires {
    /// Name of the required module.
    pub name: String,
    /// Flags for this requires directive.
    pub flags: RequiresFlags,
    /// Optional version of the required module.
    pub version: Option<String>,
}

impl Requires {
    /// Returns true if this is a transitive dependency.
    #[must_use]
    pub fn is_transitive(&self) -> bool {
        self.flags.contains(RequiresFlags::TRANSITIVE)
    }

    /// Returns true if this is a static dependency (compile-time only).
    #[must_use]
    pub fn is_static(&self) -> bool {
        self.flags.contains(RequiresFlags::STATIC_PHASE)
    }

    /// Returns true if this is a mandated dependency.
    #[must_use]
    pub fn is_mandated(&self) -> bool {
        self.flags.contains(RequiresFlags::MANDATED)
    }
}

/// A module exports directive.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Exports {
    /// Package name being exported.
    pub package: String,
    /// Optional list of target modules. If empty, export is unqualified (to all).
    pub targets: Option<Vec<String>>,
}

impl Exports {
    /// Returns true if this is a qualified export (to specific modules only).
    #[must_use]
    pub fn is_qualified(&self) -> bool {
        self.targets.is_some()
    }

    /// Returns true if the package is exported to the given module.
    #[must_use]
    pub fn exports_to(&self, module: &str) -> bool {
        match &self.targets {
            None => true,
            Some(targets) => targets.iter().any(|t| t == module),
        }
    }
}

/// A module opens directive.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Opens {
    /// Package name being opened.
    pub package: String,
    /// Optional list of target modules. If empty, open is unqualified (to all).
    pub targets: Option<Vec<String>>,
}

impl Opens {
    /// Returns true if this is a qualified open (to specific modules only).
    #[must_use]
    pub fn is_qualified(&self) -> bool {
        self.targets.is_some()
    }

    /// Returns true if the package is opened to the given module.
    #[must_use]
    pub fn opens_to(&self, module: &str) -> bool {
        match &self.targets {
            None => true,
            Some(targets) => targets.iter().any(|t| t == module),
        }
    }
}

/// A module provides directive.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Provides {
    /// Service interface being provided.
    pub service: String,
    /// Implementation classes.
    pub implementations: Vec<String>,
}

/// A module descriptor representing the module-info.class content.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModuleDescriptor {
    /// Module name.
    pub name: String,
    /// Module flags.
    pub flags: ModuleFlags,
    /// Optional module version.
    pub version: Option<String>,
    /// Required modules.
    pub requires: Vec<Requires>,
    /// Exported packages.
    pub exports: Vec<Exports>,
    /// Opened packages.
    pub opens: Vec<Opens>,
    /// Services used by this module.
    pub uses: Vec<String>,
    /// Services provided by this module.
    pub provides: Vec<Provides>,
    /// All packages in this module.
    pub packages: BTreeSet<String>,
    /// Main class (if specified).
    pub main_class: Option<String>,
}

impl ModuleDescriptor {
    /// Creates a new module descriptor with the given name.
    #[must_use]
    pub fn new(name: String) -> Self {
        Self {
            name,
            flags: ModuleFlags::empty(),
            version: None,
            requires: Vec::new(),
            exports: Vec::new(),
            opens: Vec::new(),
            uses: Vec::new(),
            provides: Vec::new(),
            packages: BTreeSet::new(),
            main_class: None,
        }
    }

    /// Returns true if this is an open module.
    #[must_use]
    pub fn is_open(&self) -> bool {
        self.flags.contains(ModuleFlags::OPEN)
    }

    /// Returns true if this is an automatic module.
    #[must_use]
    pub fn is_automatic(&self) -> bool {
        // Automatic modules don't have module-info.class, so they're created differently
        // and are identified by not having explicit requires to java.base
        false // Will be set by the caller when creating automatic modules
    }

    /// Returns true if the package is exported (unqualified or to the target).
    #[must_use]
    pub fn exports_package(&self, package: &str, to_module: Option<&str>) -> bool {
        for export in &self.exports {
            if export.package == package {
                return match to_module {
                    Some(module) => export.exports_to(module),
                    None => !export.is_qualified(),
                };
            }
        }
        false
    }

    /// Returns true if the package is opened (unqualified or to the target).
    #[must_use]
    pub fn opens_package(&self, package: &str, to_module: Option<&str>) -> bool {
        // Open modules have all packages implicitly opened
        if self.is_open() {
            return self.packages.contains(package);
        }

        for open in &self.opens {
            if open.package == package {
                return match to_module {
                    Some(module) => open.opens_to(module),
                    None => !open.is_qualified(),
                };
            }
        }
        false
    }

    /// Returns true if this module requires the given module.
    #[must_use]
    pub fn requires_module(&self, module: &str) -> bool {
        self.requires.iter().any(|r| r.name == module)
    }

    /// Parses a module descriptor from a class file.
    ///
    /// # Errors
    ///
    /// Returns an error if the class file is not a valid module-info.class.
    pub fn from_class_file(class_file: &ClassFile) -> Result<Self> {
        // Find the Module attribute
        let mut module_attr = None;
        let mut packages_attr = None;
        let mut main_class_attr = None;

        for attr in &class_file.attributes {
            match attr {
                Attribute::Module { .. } => module_attr = Some(attr),
                Attribute::ModulePackages { .. } => packages_attr = Some(attr),
                Attribute::ModuleMainClass { .. } => main_class_attr = Some(attr),
                _ => {}
            }
        }

        let Some(Attribute::Module {
            module_name_index,
            flags,
            version_index,
            requires,
            exports,
            opens,
            uses,
            provides,
            ..
        }) = module_attr
        else {
            return Err(ModuleError::DescriptorParseError(
                "No Module attribute found".to_string(),
            ));
        };

        let constant_pool = &class_file.constant_pool;
        let name = Self::get_module_name(constant_pool, *module_name_index)?;
        let version = Self::parse_version(constant_pool, *version_index)?;
        let parsed_requires = Self::parse_requires(constant_pool, requires)?;
        let parsed_exports = Self::parse_exports(constant_pool, exports)?;
        let parsed_opens = Self::parse_opens(constant_pool, opens)?;
        let parsed_uses = Self::parse_uses(constant_pool, uses)?;
        let parsed_provides = Self::parse_provides(constant_pool, provides)?;
        let parsed_packages = Self::parse_packages(constant_pool, packages_attr)?;
        let main_class = Self::parse_main_class(constant_pool, main_class_attr)?;

        Ok(Self {
            name,
            flags: *flags,
            version,
            requires: parsed_requires,
            exports: parsed_exports,
            opens: parsed_opens,
            uses: parsed_uses,
            provides: parsed_provides,
            packages: parsed_packages,
            main_class,
        })
    }

    /// Parses the module version from the constant pool.
    ///
    /// # Errors
    ///
    /// Returns an error if the version index is invalid.
    fn parse_version(constant_pool: &ConstantPool, version_index: u16) -> Result<Option<String>> {
        if version_index != 0 {
            Ok(Some(Self::get_utf8(constant_pool, version_index)?))
        } else {
            Ok(None)
        }
    }

    /// Parses the requires directives from the `Requires` attributes.
    ///
    /// # Errors
    ///
    /// Returns an error if any required module name or version cannot be resolved.
    fn parse_requires(
        constant_pool: &ConstantPool,
        requires: &[ristretto_classfile::attributes::Requires],
    ) -> Result<Vec<Requires>> {
        let mut parsed = Vec::with_capacity(requires.len());
        for requires_entry in requires {
            let required_module_name = Self::get_module_name(constant_pool, requires_entry.index)?;
            let required_module_version = if requires_entry.version_index != 0 {
                Some(Self::get_utf8(constant_pool, requires_entry.version_index)?)
            } else {
                None
            };
            parsed.push(Requires {
                name: required_module_name,
                flags: requires_entry.flags,
                version: required_module_version,
            });
        }
        Ok(parsed)
    }

    /// Parses the exports directives from the `Exports` attributes.
    ///
    /// # Errors
    ///
    /// Returns an error if any exported package name or target module cannot be resolved.
    fn parse_exports(
        constant_pool: &ConstantPool,
        exports: &[ristretto_classfile::attributes::Exports],
    ) -> Result<Vec<Exports>> {
        let mut parsed = Vec::with_capacity(exports.len());
        for export_entry in exports {
            let package = Self::get_package_name(constant_pool, export_entry.index)?;
            let targets = if export_entry.to_index.is_empty() {
                None
            } else {
                let mut target_names = Vec::with_capacity(export_entry.to_index.len());
                for &target_index in &export_entry.to_index {
                    target_names.push(Self::get_module_name(constant_pool, target_index)?);
                }
                Some(target_names)
            };
            parsed.push(Exports { package, targets });
        }
        Ok(parsed)
    }

    /// Parses the opens directives from the `Opens` attributes.
    ///
    /// # Errors
    ///
    /// Returns an error if any opened package name or target module cannot be resolved.
    fn parse_opens(
        constant_pool: &ConstantPool,
        opens: &[ristretto_classfile::attributes::Opens],
    ) -> Result<Vec<Opens>> {
        let mut parsed = Vec::with_capacity(opens.len());
        for opens_entry in opens {
            let package = Self::get_package_name(constant_pool, opens_entry.index)?;
            let targets = if opens_entry.to_index.is_empty() {
                None
            } else {
                let mut target_names = Vec::with_capacity(opens_entry.to_index.len());
                for &target_index in &opens_entry.to_index {
                    target_names.push(Self::get_module_name(constant_pool, target_index)?);
                }
                Some(target_names)
            };
            parsed.push(Opens { package, targets });
        }
        Ok(parsed)
    }

    /// Parses the uses directives from the `Uses` attributes.
    ///
    /// # Errors
    ///
    /// Returns an error if any used service interface cannot be resolved.
    fn parse_uses(constant_pool: &ConstantPool, uses: &[u16]) -> Result<Vec<String>> {
        let mut parsed = Vec::with_capacity(uses.len());
        for &use_index in uses {
            let service = Self::get_class_name(constant_pool, use_index)?;
            parsed.push(service);
        }
        Ok(parsed)
    }

    /// Parses the provides directives from the `Provides` attributes.
    ///
    /// # Errors
    ///
    /// Returns an error if any service interface or implementation class cannot be resolved.
    fn parse_provides(
        constant_pool: &ConstantPool,
        provides: &[ristretto_classfile::attributes::Provides],
    ) -> Result<Vec<Provides>> {
        let mut parsed = Vec::with_capacity(provides.len());
        for provides_entry in provides {
            let service = Self::get_class_name(constant_pool, provides_entry.index)?;
            let mut implementations = Vec::with_capacity(provides_entry.with_index.len());
            for &implementation_index in &provides_entry.with_index {
                implementations.push(Self::get_class_name(constant_pool, implementation_index)?);
            }
            parsed.push(Provides {
                service,
                implementations,
            });
        }
        Ok(parsed)
    }

    /// Parses the packages from the `ModulePackages` attribute.
    ///
    /// # Errors
    ///
    /// Returns an error if any package name cannot be resolved.
    fn parse_packages(
        constant_pool: &ConstantPool,
        packages_attr: Option<&Attribute>,
    ) -> Result<BTreeSet<String>> {
        let mut parsed = BTreeSet::new();
        if let Some(Attribute::ModulePackages {
            package_indexes, ..
        }) = packages_attr
        {
            for &package_index in package_indexes {
                let package = Self::get_package_name(constant_pool, package_index)?;
                parsed.insert(package);
            }
        }
        Ok(parsed)
    }

    /// Parses the main class from the `ModuleMainClass` attribute.
    ///
    /// # Errors
    ///
    /// Returns an error if the main class name cannot be resolved.
    fn parse_main_class(
        constant_pool: &ConstantPool,
        main_class_attr: Option<&Attribute>,
    ) -> Result<Option<String>> {
        if let Some(Attribute::ModuleMainClass {
            main_class_index, ..
        }) = main_class_attr
        {
            Ok(Some(Self::get_class_name(
                constant_pool,
                *main_class_index,
            )?))
        } else {
            Ok(None)
        }
    }

    /// Creates an automatic module descriptor from a JAR file name.
    ///
    /// # Errors
    ///
    /// Returns an error if the name cannot be derived.
    pub fn automatic_from_jar_name(
        jar_name: &str,
        manifest_module_name: Option<&str>,
        packages: BTreeSet<String>,
    ) -> Result<Self> {
        let name = if let Some(manifest_name) = manifest_module_name {
            validate_module_name(manifest_name)?;
            manifest_name.to_string()
        } else {
            derive_automatic_module_name(jar_name)?
        };

        let mut descriptor = Self::new(name);

        // Automatic modules implicitly require java.base
        descriptor.requires.push(Requires {
            name: "java.base".to_string(),
            flags: RequiresFlags::MANDATED,
            version: None,
        });

        // Automatic modules export all their packages (unqualified)
        for package in &packages {
            descriptor.exports.push(Exports {
                package: package.clone(),
                targets: None,
            });
        }

        // Automatic modules open all their packages (for reflection)
        for package in &packages {
            descriptor.opens.push(Opens {
                package: package.clone(),
                targets: None,
            });
        }

        descriptor.packages = packages;

        Ok(descriptor)
    }

    /// Retrieves a UTF-8 string from the constant pool.
    ///
    /// # Errors
    ///
    /// Returns an error if the index does not point to a UTF-8 constant.
    fn get_utf8(constant_pool: &ConstantPool, index: u16) -> Result<String> {
        match constant_pool.get(index) {
            Some(Constant::Utf8(s)) => Ok(s.clone()),
            _ => Err(ModuleError::DescriptorParseError(format!(
                "Expected UTF8 at index {index}"
            ))),
        }
    }

    /// Retrieves a module name from the constant pool.
    ///
    /// # Errors
    ///
    /// Returns an error if the index does not point to a Module constant.
    fn get_module_name(constant_pool: &ConstantPool, index: u16) -> Result<String> {
        match constant_pool.get(index) {
            Some(Constant::Module(name_index)) => Self::get_utf8(constant_pool, *name_index),
            _ => Err(ModuleError::DescriptorParseError(format!(
                "Expected Module at index {index}"
            ))),
        }
    }

    /// Retrieves a package name from the constant pool.
    ///
    /// # Errors
    ///
    /// Returns an error if the index does not point to a Package constant.
    fn get_package_name(constant_pool: &ConstantPool, index: u16) -> Result<String> {
        match constant_pool.get(index) {
            Some(Constant::Package(name_index)) => Self::get_utf8(constant_pool, *name_index),
            _ => Err(ModuleError::DescriptorParseError(format!(
                "Expected Package at index {index}"
            ))),
        }
    }

    /// Retrieves a package name from the constant pool.
    ///
    /// # Errors
    ///
    /// Returns an error if the index does not point to a Package constant.
    fn get_class_name(constant_pool: &ConstantPool, index: u16) -> Result<String> {
        match constant_pool.get(index) {
            Some(Constant::Class(name_index)) => Self::get_utf8(constant_pool, *name_index),
            _ => Err(ModuleError::DescriptorParseError(format!(
                "Expected Class at index {index}"
            ))),
        }
    }
}

impl fmt::Display for ModuleDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_open() {
            write!(f, "open ")?;
        }
        write!(f, "module {}", self.name)?;
        if let Some(ref version) = self.version {
            write!(f, "@{version}")?;
        }
        Ok(())
    }
}

/// Validates a module name according to JPMS rules.
///
/// # Errors
///
/// Returns an error if the name is invalid.
fn validate_module_name(name: &str) -> Result<()> {
    if name.is_empty() {
        return Err(ModuleError::InvalidModuleName("empty name".to_string()));
    }

    // Module names follow Java qualified identifier rules
    for part in name.split('.') {
        if part.is_empty() {
            return Err(ModuleError::InvalidModuleName(format!(
                "empty component in {name}"
            )));
        }
        // Check first character is a Java identifier start
        let mut chars = part.chars();
        if let Some(first) = chars.next()
            && !first.is_alphabetic()
            && first != '_'
            && first != '$'
        {
            return Err(ModuleError::InvalidModuleName(format!(
                "invalid start character in {name}"
            )));
        }
        // Check remaining characters
        for c in chars {
            if !c.is_alphanumeric() && c != '_' && c != '$' {
                return Err(ModuleError::InvalidModuleName(format!(
                    "invalid character '{c}' in {name}"
                )));
            }
        }
    }

    Ok(())
}

/// Derives an automatic module name from a JAR file name.
///
/// The algorithm follows `OpenJDK` behavior:
/// 1. Drop the `.jar` extension
/// 2. Find and remove version suffix (pattern: `-[0-9]+.*$`)
/// 3. Replace non-alphanumeric characters with dots
/// 4. Collapse consecutive dots to single dot
/// 5. Remove leading and trailing dots
///
/// # Errors
///
/// Returns an error if the derived name is invalid.
fn derive_automatic_module_name(jar_name: &str) -> Result<String> {
    // Extract base name from path
    let base = jar_name
        .rsplit(std::path::MAIN_SEPARATOR)
        .next()
        .unwrap_or(jar_name);

    // Remove .jar extension
    let name = base.strip_suffix(".jar").unwrap_or(base);

    if name.is_empty() {
        return Err(ModuleError::InvalidAutomaticModuleName(
            jar_name.to_string(),
        ));
    }

    // Find version suffix (pattern: -[0-9].*)
    let name = strip_version_suffix(name);

    // Replace non-alphanumeric (except .) with dots
    let mut result = String::with_capacity(name.len());
    let mut last_was_dot = true; // Treat start as if preceded by dot to skip leading dots

    for c in name.chars() {
        if c.is_ascii_alphanumeric() {
            result.push(c);
            last_was_dot = false;
        } else if !last_was_dot {
            result.push('.');
            last_was_dot = true;
        }
    }

    // Remove trailing dot
    while result.ends_with('.') {
        result.pop();
    }

    if result.is_empty() {
        return Err(ModuleError::InvalidAutomaticModuleName(
            jar_name.to_string(),
        ));
    }

    // Validate the derived name
    validate_module_name(&result)?;

    Ok(result)
}

/// Strip version suffix from a module name.
/// Matches pattern: `-[0-9].*$` (e.g., "-1.0.0", "-2.1-SNAPSHOT")
fn strip_version_suffix(name: &str) -> &str {
    let bytes = name.as_bytes();
    for i in (0..bytes.len().saturating_sub(1)).rev() {
        if bytes[i] == b'-' && bytes.get(i + 1).is_some_and(u8::is_ascii_digit) {
            return &name[..i];
        }
    }
    name
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derive_automatic_module_name_simple() {
        assert_eq!(
            derive_automatic_module_name("foo-bar.jar").unwrap(),
            "foo.bar"
        );
    }

    #[test]
    fn test_derive_automatic_module_name_with_version() {
        assert_eq!(
            derive_automatic_module_name("foo-bar-1.2.3.jar").unwrap(),
            "foo.bar"
        );
    }

    #[test]
    fn test_derive_automatic_module_name_complex() {
        assert_eq!(
            derive_automatic_module_name("guava-31.1-jre.jar").unwrap(),
            "guava"
        );
    }

    #[test]
    fn test_derive_automatic_module_name_underscores() {
        assert_eq!(
            derive_automatic_module_name("my_library-2.0.jar").unwrap(),
            "my.library"
        );
    }

    #[test]
    fn test_validate_module_name_valid() {
        assert!(validate_module_name("java.base").is_ok());
        assert!(validate_module_name("com.example.mymodule").is_ok());
        assert!(validate_module_name("mymodule").is_ok());
    }

    #[test]
    fn test_validate_module_name_invalid() {
        assert!(validate_module_name("").is_err());
        assert!(validate_module_name(".foo").is_err());
        assert!(validate_module_name("foo.").is_err());
        assert!(validate_module_name("foo..bar").is_err());
        assert!(validate_module_name("1foo").is_err());
    }

    #[test]
    fn test_requires_flags() {
        let requires_entry = Requires {
            name: "java.base".to_string(),
            flags: RequiresFlags::TRANSITIVE | RequiresFlags::MANDATED,
            version: None,
        };
        assert!(requires_entry.is_transitive());
        assert!(requires_entry.is_mandated());
        assert!(!requires_entry.is_static());
    }

    #[test]
    fn test_exports_qualified() {
        let export = Exports {
            package: "com.example.internal".to_string(),
            targets: Some(vec!["com.example.other".to_string()]),
        };
        assert!(export.is_qualified());
        assert!(export.exports_to("com.example.other"));
        assert!(!export.exports_to("com.example.unknown"));
    }

    #[test]
    fn test_exports_unqualified() {
        let export = Exports {
            package: "com.example.api".to_string(),
            targets: None,
        };
        assert!(!export.is_qualified());
        assert!(export.exports_to("any.module"));
    }
}
