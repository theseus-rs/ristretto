//! VM configuration for the Ristretto Virtual Machine.
//!
//! This module provides the `Configuration` struct which contains all settings
//! needed to configure and run the VM.

use super::{MainModule, ModuleExport, ModuleOpens, ModulePatch, ModuleRead};
pub use ristretto_classfile::VerifyMode;
use ristretto_classloader::ClassPath;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Configuration for the Ristretto Virtual Machine (VM).
///
/// This struct contains all the settings needed to configure and run
/// the VM, including classpath, main class, Java version, system properties,
/// I/O streams, and JPMS module configuration.
///
/// Use [`ConfigurationBuilder`](super::ConfigurationBuilder) to create instances.
pub struct Configuration {
    pub(super) class_path: ClassPath,
    pub(super) main_class: Option<String>,
    pub(super) jar: Option<PathBuf>,
    pub(super) java_home: Option<PathBuf>,
    pub(super) java_version: Option<String>,
    pub(super) system_properties: HashMap<String, String>,
    pub(super) interpreted: bool,
    pub(super) batch_compilation: bool,
    pub(super) preview_features: bool,
    pub(super) verify_mode: VerifyMode,
    // JPMS module configuration fields
    pub(super) module_path: Vec<PathBuf>,
    pub(super) upgrade_module_path: Vec<PathBuf>,
    pub(super) main_module: Option<MainModule>,
    pub(super) add_modules: Vec<String>,
    pub(super) limit_modules: HashSet<String>,
    pub(super) add_reads: Vec<ModuleRead>,
    pub(super) add_exports: Vec<ModuleExport>,
    pub(super) add_opens: Vec<ModuleOpens>,
    pub(super) patch_modules: Vec<ModulePatch>,
    // I/O streams
    pub(super) stdin: Arc<Mutex<dyn Read + Send + Sync>>,
    pub(super) stdout: Arc<Mutex<dyn Write + Send + Sync>>,
    pub(super) stderr: Arc<Mutex<dyn Write + Send + Sync>>,
}

impl Configuration {
    /// Returns a reference to the class path configuration.
    ///
    /// The class path determines where the VM will look for classes during execution.
    #[must_use]
    pub fn class_path(&self) -> &ClassPath {
        &self.class_path
    }

    /// Returns the name of the main class to be executed, if specified.
    ///
    /// This is the class containing the `main` method that will be invoked when the VM starts.
    /// Returns `None` if no main class has been specified.
    #[must_use]
    pub fn main_class(&self) -> Option<&String> {
        self.main_class.as_ref()
    }

    /// Returns the path to the JAR file to be executed, if specified.
    ///
    /// When running a JAR file, the VM will look for the main class in the JAR's manifest.
    /// Returns `None` if no JAR file has been specified.
    #[must_use]
    pub fn jar(&self) -> Option<&PathBuf> {
        self.jar.as_ref()
    }

    /// Returns the path to the Java home directory, if specified.
    ///
    /// This is used to locate the standard Java libraries.
    /// Returns `None` if no Java home has been specified.
    #[must_use]
    pub fn java_home(&self) -> Option<&PathBuf> {
        self.java_home.as_ref()
    }

    /// Returns the Java version to be used, if specified.
    ///
    /// This determines which version of the Java standard libraries will be used.
    /// Returns `None` if no specific Java version has been requested.
    #[must_use]
    pub fn java_version(&self) -> Option<&String> {
        self.java_version.as_ref()
    }

    /// Returns a reference to the system properties map.
    ///
    /// System properties are key-value pairs accessible via `System.getProperty()` in Java code.
    /// These properties configure various aspects of the Java runtime environment.
    #[must_use]
    pub fn system_properties(&self) -> &HashMap<String, String> {
        &self.system_properties
    }

    /// Returns whether the VM should run in interpreted mode.
    ///
    /// When `true`, the VM will interpret bytecode rather than using JIT compilation.
    /// This may be slower but can be useful for debugging or testing.
    #[must_use]
    pub fn interpreted(&self) -> bool {
        self.interpreted
    }

    /// Returns whether Java preview features should be enabled.
    ///
    /// When `true`, the VM will allow the use of preview features from the Java language.
    /// Preview features are not finalized and may change in future Java releases.
    #[must_use]
    pub fn preview_features(&self) -> bool {
        self.preview_features
    }

    /// Returns a reference to the standard input stream.
    ///
    /// This stream is used for normal input from the VM and executed Java programs.
    /// The stream is wrapped in an `Arc<Mutex>` to allow for thread-safe access.
    #[must_use]
    pub fn stdin(&self) -> Arc<Mutex<dyn Read + Send + Sync>> {
        self.stdin.clone()
    }

    /// Returns a reference to the standard output stream.
    ///
    /// This stream is used for normal output from the VM and executed Java programs.
    /// The stream is wrapped in an `Arc<Mutex>` to allow for thread-safe access.
    #[must_use]
    pub fn stdout(&self) -> Arc<Mutex<dyn Write + Send + Sync>> {
        self.stdout.clone()
    }

    /// Returns a reference to the standard error stream.
    ///
    /// This stream is used for error messages from the VM and executed Java programs.
    /// The stream is wrapped in an `Arc<Mutex>` to allow for thread-safe access.
    #[must_use]
    pub fn stderr(&self) -> Arc<Mutex<dyn Write + Send + Sync>> {
        self.stderr.clone()
    }

    /// Returns whether background batch compilation is enabled.
    ///
    /// When `true`, the VM will perform compilation in the background, allowing
    /// for faster startup times in some cases.
    #[must_use]
    pub fn batch_compilation(&self) -> bool {
        self.batch_compilation
    }

    /// Returns the verification mode for class files.
    #[must_use]
    pub fn verify_mode(&self) -> VerifyMode {
        self.verify_mode
    }

    /// Returns the module path (-p or --module-path).
    #[must_use]
    pub fn module_path(&self) -> &[PathBuf] {
        &self.module_path
    }

    /// Returns the upgrade module path (--upgrade-module-path).
    #[must_use]
    pub fn upgrade_module_path(&self) -> &[PathBuf] {
        &self.upgrade_module_path
    }

    /// Returns the main module specification (--module or -m), if set.
    #[must_use]
    pub fn main_module(&self) -> Option<&MainModule> {
        self.main_module.as_ref()
    }

    /// Returns the main module name, if specified.
    #[must_use]
    pub fn main_module_name(&self) -> Option<&str> {
        self.main_module.as_ref().map(|m| m.name.as_str())
    }

    /// Returns the main class within the main module, if specified.
    #[must_use]
    pub fn main_module_class(&self) -> Option<&str> {
        self.main_module
            .as_ref()
            .and_then(|m| m.main_class.as_deref())
    }

    /// Returns true if module mode is enabled (--module was specified).
    #[must_use]
    pub fn is_module_mode(&self) -> bool {
        self.main_module.is_some()
    }

    /// Returns the additional modules to add (--add-modules).
    #[must_use]
    pub fn add_modules(&self) -> &[String] {
        &self.add_modules
    }

    /// Returns the modules to limit observability (--limit-modules).
    #[must_use]
    pub fn limit_modules(&self) -> &HashSet<String> {
        &self.limit_modules
    }

    /// Returns the read edges to add (--add-reads).
    #[must_use]
    pub fn add_reads(&self) -> &[ModuleRead] {
        &self.add_reads
    }

    /// Returns the exports to add (--add-exports).
    #[must_use]
    pub fn add_exports(&self) -> &[ModuleExport] {
        &self.add_exports
    }

    /// Returns the opens to add (--add-opens).
    #[must_use]
    pub fn add_opens(&self) -> &[ModuleOpens] {
        &self.add_opens
    }

    /// Returns the patch modules (--patch-module).
    #[must_use]
    pub fn patch_modules(&self) -> &[ModulePatch] {
        &self.patch_modules
    }
}

#[expect(clippy::missing_fields_in_debug)]
impl Debug for Configuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Configuration")
            .field("class_path", &self.class_path)
            .field("main_class", &self.main_class)
            .field("jar", &self.jar)
            .field("java_home", &self.java_home)
            .field("java_version", &self.java_version)
            .field("system_properties", &self.system_properties)
            .field("interpreted", &self.interpreted)
            .field("batch_compilation", &self.batch_compilation)
            .field("preview_features", &self.preview_features)
            .field("verify_mode", &self.verify_mode)
            .field("module_path", &self.module_path)
            .field("upgrade_module_path", &self.upgrade_module_path)
            .field("main_module", &self.main_module)
            .field("add_modules", &self.add_modules)
            .field("limit_modules", &self.limit_modules)
            .field("add_reads", &self.add_reads)
            .field("add_exports", &self.add_exports)
            .field("add_opens", &self.add_opens)
            .field("patch_modules", &self.patch_modules)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::super::ConfigurationBuilder;
    use super::*;
    use ristretto_classloader::DEFAULT_JAVA_VERSION;

    type Result<T> = std::result::Result<T, crate::Error>;

    #[test]
    fn test_class_path() -> Result<()> {
        let config = ConfigurationBuilder::new()
            .class_path(ClassPath::from(&["/path/to/classes"]))
            .build()?;
        assert!(config.class_path().to_string().contains("path/to/classes"));
        Ok(())
    }

    #[test]
    fn test_main_class() -> Result<()> {
        let config = ConfigurationBuilder::new()
            .main_class("com.example.Main")
            .build()?;
        assert_eq!(Some(&"com.example.Main".to_string()), config.main_class());
        Ok(())
    }

    #[test]
    fn test_main_class_none() -> Result<()> {
        let config = ConfigurationBuilder::new().build()?;
        assert!(config.main_class().is_none());
        Ok(())
    }

    #[test]
    fn test_jar() -> Result<()> {
        let config = ConfigurationBuilder::new()
            .jar(PathBuf::from("/path/to/app.jar"))
            .build()?;
        assert_eq!(Some(&PathBuf::from("/path/to/app.jar")), config.jar());
        Ok(())
    }

    #[test]
    fn test_java_home() -> Result<()> {
        let config = ConfigurationBuilder::new()
            .java_home(PathBuf::from("/usr/lib/jvm/java-21"))
            .build()?;
        assert_eq!(
            Some(&PathBuf::from("/usr/lib/jvm/java-21")),
            config.java_home()
        );
        assert!(config.java_version().is_none());
        Ok(())
    }

    #[test]
    fn test_java_version() -> Result<()> {
        let config = ConfigurationBuilder::new().java_version("21").build()?;
        assert_eq!(Some(&"21".to_string()), config.java_version());
        Ok(())
    }

    #[test]
    fn test_default_java_version() -> Result<()> {
        let config = ConfigurationBuilder::new().build()?;
        assert_eq!(
            Some(&DEFAULT_JAVA_VERSION.to_string()),
            config.java_version()
        );
        Ok(())
    }

    #[test]
    fn test_system_properties() -> Result<()> {
        let config = ConfigurationBuilder::new()
            .add_system_property("key1", "value1")
            .add_system_property("key2", "value2")
            .build()?;
        let props = config.system_properties();
        assert_eq!(Some(&"value1".to_string()), props.get("key1"));
        assert_eq!(Some(&"value2".to_string()), props.get("key2"));
        Ok(())
    }

    #[test]
    fn test_interpreted() -> Result<()> {
        let config = ConfigurationBuilder::new().interpreted(true).build()?;
        assert!(config.interpreted());
        Ok(())
    }

    #[test]
    fn test_preview_features() -> Result<()> {
        let config = ConfigurationBuilder::new().preview_features().build()?;
        assert!(config.preview_features());
        Ok(())
    }

    #[test]
    fn test_batch_compilation() -> Result<()> {
        let config = ConfigurationBuilder::new()
            .batch_compilation(false)
            .build()?;
        assert!(!config.batch_compilation());
        Ok(())
    }

    #[test]
    fn test_verify_mode() -> Result<()> {
        let config = ConfigurationBuilder::new()
            .verify_mode(VerifyMode::All)
            .build()?;
        assert_eq!(VerifyMode::All, config.verify_mode());
        Ok(())
    }

    #[test]
    fn test_module_path() -> Result<()> {
        let config = ConfigurationBuilder::new()
            .module_path(vec![PathBuf::from("/mods")])
            .build()?;
        assert_eq!(1, config.module_path().len());
        assert_eq!(PathBuf::from("/mods"), config.module_path()[0]);
        Ok(())
    }

    #[test]
    fn test_upgrade_module_path() -> Result<()> {
        let config = ConfigurationBuilder::new()
            .upgrade_module_path(vec![PathBuf::from("/upgrade")])
            .build()?;
        assert_eq!(1, config.upgrade_module_path().len());
        Ok(())
    }

    #[test]
    fn test_main_module() -> Result<()> {
        let config = ConfigurationBuilder::new()
            .main_module(MainModule::with_main_class("my.module", "com.example.Main"))
            .build()?;
        assert!(config.is_module_mode());
        assert_eq!(Some("my.module"), config.main_module_name());
        assert_eq!(Some("com.example.Main"), config.main_module_class());
        Ok(())
    }

    #[test]
    fn test_add_modules() -> Result<()> {
        let config = ConfigurationBuilder::new()
            .add_module("java.sql")
            .add_module("java.xml")
            .build()?;
        assert_eq!(2, config.add_modules().len());
        Ok(())
    }

    #[test]
    fn test_limit_modules() -> Result<()> {
        let config = ConfigurationBuilder::new()
            .limit_module("java.base")
            .build()?;
        assert!(config.limit_modules().contains("java.base"));
        Ok(())
    }

    #[test]
    fn test_add_reads() -> Result<()> {
        let config = ConfigurationBuilder::new()
            .add_read(ModuleRead::new("my.module", "java.sql"))
            .build()?;
        assert_eq!(1, config.add_reads().len());
        Ok(())
    }

    #[test]
    fn test_add_exports() -> Result<()> {
        let config = ConfigurationBuilder::new()
            .add_export(ModuleExport::new("java.base", "java.lang", "ALL-UNNAMED"))
            .build()?;
        assert_eq!(1, config.add_exports().len());
        Ok(())
    }

    #[test]
    fn test_add_opens() -> Result<()> {
        let config = ConfigurationBuilder::new()
            .add_opens(ModuleOpens::new(
                "java.base",
                "java.lang.reflect",
                "ALL-UNNAMED",
            ))
            .build()?;
        assert_eq!(1, config.add_opens().len());
        Ok(())
    }

    #[test]
    fn test_patch_modules() -> Result<()> {
        let config = ConfigurationBuilder::new()
            .add_patch(ModulePatch::new("java.base", "/patch"))
            .build()?;
        assert_eq!(1, config.patch_modules().len());
        Ok(())
    }

    #[test]
    fn test_debug() -> Result<()> {
        let config = ConfigurationBuilder::new()
            .main_class("com.example.Main")
            .build()?;
        let debug_str = format!("{config:?}");
        assert!(debug_str.contains("Configuration"));
        assert!(debug_str.contains("com.example.Main"));
        Ok(())
    }
}
