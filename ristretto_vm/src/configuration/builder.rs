//! Configuration builder for the Ristretto Virtual Machine.
//!
//! This module provides the `ConfigurationBuilder` struct for creating
//! VM configurations with a fluent interface.

use super::{
    Configuration, MainModule, ModuleExport, ModuleOpens, ModulePatch, ModuleRead, VerifyMode,
};
use crate::Error::InternalError;
use crate::Result;
use ristretto_classloader::{ClassPath, DEFAULT_JAVA_VERSION};
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::io::{Read, Write, stderr, stdin, stdout};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Builder for creating a `Configuration` with a fluent interface.
///
/// This builder provides methods to set all the configuration options
/// and then create a `Configuration` instance with the `build()` method.
///
/// # Example
///
/// ```rust
/// use ristretto_vm::ConfigurationBuilder;
/// use ristretto_classloader::ClassPath;
///
/// let config = ConfigurationBuilder::new()
///     .class_path(ClassPath::from(&["."]))
///     .main_class("com.example.Main")
///     .java_version("21")
///     .build()
///     .unwrap();
/// ```
pub struct ConfigurationBuilder {
    class_path: Option<ClassPath>,
    main_class: Option<String>,
    jar: Option<PathBuf>,
    java_home: Option<PathBuf>,
    java_version: Option<String>,
    system_properties: HashMap<String, String>,
    interpreted: bool,
    batch_compilation: bool,
    preview_features: bool,
    verify_mode: VerifyMode,
    // JPMS module configuration fields
    module_path: Vec<PathBuf>,
    upgrade_module_path: Vec<PathBuf>,
    main_module: Option<MainModule>,
    add_modules: Vec<String>,
    limit_modules: HashSet<String>,
    add_reads: Vec<ModuleRead>,
    add_exports: Vec<ModuleExport>,
    add_opens: Vec<ModuleOpens>,
    patch_modules: Vec<ModulePatch>,
    // I/O streams
    stdin: Arc<Mutex<dyn Read + Send + Sync>>,
    stdout: Arc<Mutex<dyn Write + Send + Sync>>,
    stderr: Arc<Mutex<dyn Write + Send + Sync>>,
}

impl ConfigurationBuilder {
    /// Creates a new `ConfigurationBuilder` with default values.
    ///
    /// Default values include:
    /// - No class path (will be set to "." when building)
    /// - No main class
    /// - No JAR file
    /// - No Java home
    /// - Default Java version (will be set when building if no Java home is provided)
    /// - Empty system properties
    /// - JIT mode enabled
    /// - Background batch compilation enabled
    /// - Preview features disabled
    /// - Verify mode set to Remote (verify only remote/untrusted classes)
    /// - Standard output and error streams directed to system stdout/stderr
    #[must_use]
    pub fn new() -> Self {
        ConfigurationBuilder {
            class_path: None,
            main_class: None,
            jar: None,
            java_home: None,
            java_version: None,
            system_properties: HashMap::new(),
            interpreted: false,
            batch_compilation: true,
            preview_features: false,
            verify_mode: VerifyMode::default(),
            module_path: Vec::new(),
            upgrade_module_path: Vec::new(),
            main_module: None,
            add_modules: Vec::new(),
            limit_modules: HashSet::new(),
            add_reads: Vec::new(),
            add_exports: Vec::new(),
            add_opens: Vec::new(),
            patch_modules: Vec::new(),
            stdin: Arc::new(Mutex::new(stdin())),
            stdout: Arc::new(Mutex::new(stdout())),
            stderr: Arc::new(Mutex::new(stderr())),
        }
    }

    /// Set the VM class path
    #[must_use]
    pub fn class_path(mut self, class_path: ClassPath) -> Self {
        self.class_path = Some(class_path);
        self
    }

    /// Set the main class to run
    #[must_use]
    pub fn main_class<S: AsRef<str>>(mut self, main_class: S) -> Self {
        self.main_class = Some(main_class.as_ref().to_string());
        self
    }

    /// Set the jar
    #[must_use]
    pub fn jar(mut self, jar: PathBuf) -> Self {
        self.jar = Some(jar);
        self
    }

    /// Set the Java home path
    #[must_use]
    pub fn java_home(mut self, java_home: PathBuf) -> Self {
        self.java_home = Some(java_home);
        self
    }

    /// Set the Java version
    #[must_use]
    pub fn java_version<S: AsRef<str>>(mut self, version: S) -> Self {
        self.java_version = Some(version.as_ref().to_string());
        self
    }

    /// Add a system property
    #[must_use]
    pub fn add_system_property<K, V>(mut self, key: K, value: V) -> Self
    where
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let key = key.as_ref().to_string();
        let value = value.as_ref().to_string();
        self.system_properties.insert(key, value);
        self
    }

    /// Set all system properties
    #[must_use]
    pub fn system_properties(mut self, properties: HashMap<String, String>) -> Self {
        self.system_properties = properties;
        self
    }

    /// Configure the VM to run in interpreted mode
    #[must_use]
    pub fn interpreted(mut self, interpreted: bool) -> Self {
        self.interpreted = interpreted;
        self
    }

    /// Configure the VM to use background batch compilation
    #[must_use]
    pub fn batch_compilation(mut self, batch_compilation: bool) -> Self {
        self.batch_compilation = batch_compilation;
        self
    }

    /// Enable preview features
    #[must_use]
    pub fn preview_features(mut self) -> Self {
        self.preview_features = true;
        self
    }

    /// Set the class verification mode
    #[must_use]
    pub fn verify_mode(mut self, verify_mode: VerifyMode) -> Self {
        self.verify_mode = verify_mode;
        self
    }

    /// Set the module path (-p or --module-path)
    #[must_use]
    pub fn module_path(mut self, paths: Vec<PathBuf>) -> Self {
        self.module_path = paths;
        self
    }

    /// Add a path to the module path
    #[must_use]
    pub fn add_module_path(mut self, path: impl Into<PathBuf>) -> Self {
        self.module_path.push(path.into());
        self
    }

    /// Set the upgrade module path (--upgrade-module-path)
    #[must_use]
    pub fn upgrade_module_path(mut self, paths: Vec<PathBuf>) -> Self {
        self.upgrade_module_path = paths;
        self
    }

    /// Add a path to the upgrade module path
    #[must_use]
    pub fn add_upgrade_module_path(mut self, path: impl Into<PathBuf>) -> Self {
        self.upgrade_module_path.push(path.into());
        self
    }

    /// Set the main module (--module or -m)
    #[must_use]
    pub fn main_module(mut self, main_module: MainModule) -> Self {
        self.main_module = Some(main_module);
        self
    }

    /// Set the additional modules to add (--add-modules)
    #[must_use]
    pub fn set_add_modules(mut self, modules: Vec<String>) -> Self {
        self.add_modules = modules;
        self
    }

    /// Add a module to the additional modules list (--add-modules)
    #[must_use]
    pub fn add_module(mut self, module: impl Into<String>) -> Self {
        self.add_modules.push(module.into());
        self
    }

    /// Set the modules to limit observability (--limit-modules)
    #[must_use]
    pub fn set_limit_modules(mut self, modules: HashSet<String>) -> Self {
        self.limit_modules = modules;
        self
    }

    /// Add a module to the limit modules set (--limit-modules)
    #[must_use]
    pub fn limit_module(mut self, module: impl Into<String>) -> Self {
        self.limit_modules.insert(module.into());
        self
    }

    /// Set the read edges to add (--add-reads)
    #[must_use]
    pub fn set_add_reads(mut self, reads: Vec<ModuleRead>) -> Self {
        self.add_reads = reads;
        self
    }

    /// Add a read edge (--add-reads)
    #[must_use]
    pub fn add_read(mut self, read: ModuleRead) -> Self {
        self.add_reads.push(read);
        self
    }

    /// Set the exports to add (--add-exports)
    #[must_use]
    pub fn set_add_exports(mut self, exports: Vec<ModuleExport>) -> Self {
        self.add_exports = exports;
        self
    }

    /// Add an export (--add-exports)
    #[must_use]
    pub fn add_export(mut self, export: ModuleExport) -> Self {
        self.add_exports.push(export);
        self
    }

    /// Set the opens to add (--add-opens)
    #[must_use]
    pub fn set_add_opens(mut self, opens: Vec<ModuleOpens>) -> Self {
        self.add_opens = opens;
        self
    }

    /// Add an opens directive (--add-opens)
    #[must_use]
    pub fn add_opens(mut self, opens: ModuleOpens) -> Self {
        self.add_opens.push(opens);
        self
    }

    /// Set the patch modules (--patch-module)
    #[must_use]
    pub fn set_patch_modules(mut self, patches: Vec<ModulePatch>) -> Self {
        self.patch_modules = patches;
        self
    }

    /// Add a patch module (--patch-module)
    #[must_use]
    pub fn add_patch(mut self, patch: ModulePatch) -> Self {
        self.patch_modules.push(patch);
        self
    }

    /// Set the standard input stream
    #[must_use]
    pub fn stdin(mut self, stdin: Arc<Mutex<dyn Read + Send + Sync>>) -> Self {
        self.stdin = stdin;
        self
    }

    /// Set the standard output stream
    #[must_use]
    pub fn stdout(mut self, stdout: Arc<Mutex<dyn Write + Send + Sync>>) -> Self {
        self.stdout = stdout;
        self
    }

    /// Set the standard error stream
    #[must_use]
    pub fn stderr(mut self, stderr: Arc<Mutex<dyn Write + Send + Sync>>) -> Self {
        self.stderr = stderr;
        self
    }

    /// Build the configuration
    ///
    /// # Errors
    ///
    /// An error will be returned if:
    /// - Both Java home and Java version are specified (mutually exclusive)
    pub fn build(self) -> Result<Configuration> {
        let class_path = self.class_path.unwrap_or_else(|| ClassPath::from(&["."]));

        let java_home = self.java_home;
        let java_version = if let Some(java_version) = self.java_version {
            if java_home.is_some() {
                return Err(InternalError(
                    "Cannot specify both Java home and version".to_string(),
                ));
            }
            Some(java_version)
        } else if java_home.is_none() {
            Some(DEFAULT_JAVA_VERSION.to_string())
        } else {
            None
        };

        Ok(Configuration {
            class_path,
            main_class: self.main_class,
            jar: self.jar,
            java_home,
            java_version,
            system_properties: self.system_properties,
            interpreted: self.interpreted,
            batch_compilation: self.batch_compilation,
            preview_features: self.preview_features,
            verify_mode: self.verify_mode,
            module_path: self.module_path,
            upgrade_module_path: self.upgrade_module_path,
            main_module: self.main_module,
            add_modules: self.add_modules,
            limit_modules: self.limit_modules,
            add_reads: self.add_reads,
            add_exports: self.add_exports,
            add_opens: self.add_opens,
            patch_modules: self.patch_modules,
            stdin: self.stdin,
            stdout: self.stdout,
            stderr: self.stderr,
        })
    }
}

#[expect(clippy::missing_fields_in_debug)]
impl Debug for ConfigurationBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConfigurationBuilder")
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

impl Default for ConfigurationBuilder {
    fn default() -> Self {
        ConfigurationBuilder::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ristretto_classloader::DEFAULT_JAVA_VERSION;

    #[test]
    fn test_new() {
        let builder = ConfigurationBuilder::new();
        let config = builder.build().unwrap();
        assert_eq!(&ClassPath::from(&["."]), config.class_path());
    }

    #[test]
    fn test_default() {
        let builder = ConfigurationBuilder::default();
        let config = builder.build().unwrap();
        assert_eq!(
            Some(&DEFAULT_JAVA_VERSION.to_string()),
            config.java_version()
        );
    }

    #[test]
    fn test_class_path() {
        let config = ConfigurationBuilder::new()
            .class_path(ClassPath::from(&["/lib", "/classes"]))
            .build()
            .unwrap();
        let cp_str = config.class_path().to_string();
        assert!(cp_str.contains("lib"));
    }

    #[test]
    fn test_main_class() {
        let config = ConfigurationBuilder::new()
            .main_class("com.example.Main")
            .build()
            .unwrap();
        assert_eq!(Some(&"com.example.Main".to_string()), config.main_class());
    }

    #[test]
    fn test_jar() {
        let config = ConfigurationBuilder::new()
            .jar(PathBuf::from("/path/to/app.jar"))
            .build()
            .unwrap();
        assert_eq!(Some(&PathBuf::from("/path/to/app.jar")), config.jar());
    }

    #[test]
    fn test_java_home() {
        let config = ConfigurationBuilder::new()
            .java_home(PathBuf::from("/usr/lib/jvm/java-21"))
            .build()
            .unwrap();
        assert_eq!(
            Some(&PathBuf::from("/usr/lib/jvm/java-21")),
            config.java_home()
        );
        assert!(config.java_version().is_none());
    }

    #[test]
    fn test_java_version() {
        let config = ConfigurationBuilder::new()
            .java_version("21")
            .build()
            .unwrap();
        assert_eq!(Some(&"21".to_string()), config.java_version());
    }

    #[test]
    fn test_java_home_and_version_error() {
        let result = ConfigurationBuilder::new()
            .java_home(PathBuf::from("/jvm"))
            .java_version("21")
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn test_add_system_property() {
        let config = ConfigurationBuilder::new()
            .add_system_property("key", "value")
            .build()
            .unwrap();
        assert_eq!(
            Some(&"value".to_string()),
            config.system_properties().get("key")
        );
    }

    #[test]
    fn test_system_properties() {
        let mut props = HashMap::new();
        props.insert("k1".to_string(), "v1".to_string());
        props.insert("k2".to_string(), "v2".to_string());

        let config = ConfigurationBuilder::new()
            .system_properties(props)
            .build()
            .unwrap();
        assert_eq!(2, config.system_properties().len());
    }

    #[test]
    fn test_interpreted() {
        let config = ConfigurationBuilder::new()
            .interpreted(true)
            .build()
            .unwrap();
        assert!(config.interpreted());
    }

    #[test]
    fn test_batch_compilation() {
        let config = ConfigurationBuilder::new()
            .batch_compilation(false)
            .build()
            .unwrap();
        assert!(!config.batch_compilation());
    }

    #[test]
    fn test_preview_features() {
        let config = ConfigurationBuilder::new()
            .preview_features()
            .build()
            .unwrap();
        assert!(config.preview_features());
    }

    #[test]
    fn test_verify_mode() {
        let config = ConfigurationBuilder::new()
            .verify_mode(VerifyMode::None)
            .build()
            .unwrap();
        assert_eq!(VerifyMode::None, config.verify_mode());
    }

    #[test]
    fn test_module_path() {
        let config = ConfigurationBuilder::new()
            .module_path(vec![PathBuf::from("/mods")])
            .build()
            .unwrap();
        assert_eq!(1, config.module_path().len());
    }

    #[test]
    fn test_add_module_path() {
        let config = ConfigurationBuilder::new()
            .add_module_path("/mods1")
            .add_module_path("/mods2")
            .build()
            .unwrap();
        assert_eq!(2, config.module_path().len());
    }

    #[test]
    fn test_upgrade_module_path() {
        let config = ConfigurationBuilder::new()
            .upgrade_module_path(vec![PathBuf::from("/upgrade")])
            .build()
            .unwrap();
        assert_eq!(1, config.upgrade_module_path().len());
    }

    #[test]
    fn test_add_upgrade_module_path() {
        let config = ConfigurationBuilder::new()
            .add_upgrade_module_path("/upgrade1")
            .add_upgrade_module_path("/upgrade2")
            .build()
            .unwrap();
        assert_eq!(2, config.upgrade_module_path().len());
    }

    #[test]
    fn test_main_module() {
        let config = ConfigurationBuilder::new()
            .main_module(MainModule::new("my.module"))
            .build()
            .unwrap();
        assert!(config.is_module_mode());
        assert_eq!(Some("my.module"), config.main_module_name());
    }

    #[test]
    fn test_set_add_modules() {
        let config = ConfigurationBuilder::new()
            .set_add_modules(vec!["java.sql".to_string(), "java.xml".to_string()])
            .build()
            .unwrap();
        assert_eq!(2, config.add_modules().len());
    }

    #[test]
    fn test_add_module() {
        let config = ConfigurationBuilder::new()
            .add_module("java.sql")
            .add_module("java.xml")
            .build()
            .unwrap();
        assert_eq!(2, config.add_modules().len());
    }

    #[test]
    fn test_set_limit_modules() {
        let mut modules = HashSet::new();
        modules.insert("java.base".to_string());

        let config = ConfigurationBuilder::new()
            .set_limit_modules(modules)
            .build()
            .unwrap();
        assert_eq!(1, config.limit_modules().len());
    }

    #[test]
    fn test_limit_module() {
        let config = ConfigurationBuilder::new()
            .limit_module("java.base")
            .limit_module("java.logging")
            .build()
            .unwrap();
        assert_eq!(2, config.limit_modules().len());
    }

    #[test]
    fn test_set_add_reads() {
        let config = ConfigurationBuilder::new()
            .set_add_reads(vec![ModuleRead::new("my.module", "java.sql")])
            .build()
            .unwrap();
        assert_eq!(1, config.add_reads().len());
    }

    #[test]
    fn test_add_read() {
        let config = ConfigurationBuilder::new()
            .add_read(ModuleRead::new("my.module", "java.sql"))
            .add_read(ModuleRead::new("my.module", "java.xml"))
            .build()
            .unwrap();
        assert_eq!(2, config.add_reads().len());
    }

    #[test]
    fn test_set_add_exports() {
        let config = ConfigurationBuilder::new()
            .set_add_exports(vec![ModuleExport::new(
                "java.base",
                "java.lang",
                "ALL-UNNAMED",
            )])
            .build()
            .unwrap();
        assert_eq!(1, config.add_exports().len());
    }

    #[test]
    fn test_add_export() {
        let config = ConfigurationBuilder::new()
            .add_export(ModuleExport::new("java.base", "java.lang", "my.module"))
            .build()
            .unwrap();
        assert_eq!(1, config.add_exports().len());
    }

    #[test]
    fn test_set_add_opens() {
        let config = ConfigurationBuilder::new()
            .set_add_opens(vec![ModuleOpens::new(
                "java.base",
                "java.lang.reflect",
                "ALL-UNNAMED",
            )])
            .build()
            .unwrap();
        assert_eq!(1, config.add_opens().len());
    }

    #[test]
    fn test_add_opens() {
        let config = ConfigurationBuilder::new()
            .add_opens(ModuleOpens::new(
                "java.base",
                "java.lang.reflect",
                "my.module",
            ))
            .build()
            .unwrap();
        assert_eq!(1, config.add_opens().len());
    }

    #[test]
    fn test_set_patch_modules() {
        let config = ConfigurationBuilder::new()
            .set_patch_modules(vec![ModulePatch::new("java.base", "/patch")])
            .build()
            .unwrap();
        assert_eq!(1, config.patch_modules().len());
    }

    #[test]
    fn test_add_patch() {
        let config = ConfigurationBuilder::new()
            .add_patch(ModulePatch::new("java.base", "/patch"))
            .build()
            .unwrap();
        assert_eq!(1, config.patch_modules().len());
    }

    #[test]
    fn test_debug() {
        let builder = ConfigurationBuilder::new().main_class("com.example.Main");
        let debug_str = format!("{builder:?}");
        assert!(debug_str.contains("ConfigurationBuilder"));
        assert!(debug_str.contains("com.example.Main"));
    }

    #[test]
    fn test_full_configuration() {
        let config = ConfigurationBuilder::new()
            .class_path(ClassPath::from(&["."]))
            .main_class("com.example.Main")
            .java_version("21")
            .add_system_property("user.name", "test")
            .interpreted(false)
            .batch_compilation(true)
            .preview_features()
            .verify_mode(VerifyMode::All)
            .module_path(vec![PathBuf::from("/mods")])
            .main_module(MainModule::with_main_class("my.module", "Main"))
            .add_module("java.sql")
            .limit_module("java.base")
            .add_read(ModuleRead::new("my.module", "java.sql"))
            .add_export(ModuleExport::new("java.base", "java.lang", "ALL-UNNAMED"))
            .add_opens(ModuleOpens::new(
                "java.base",
                "java.lang.reflect",
                "ALL-UNNAMED",
            ))
            .add_patch(ModulePatch::new("java.base", "/patch"))
            .build()
            .unwrap();

        assert!(config.preview_features());
        assert!(config.is_module_mode());
        assert_eq!(1, config.module_path().len());
        assert_eq!(1, config.add_modules().len());
        assert_eq!(1, config.limit_modules().len());
        assert_eq!(1, config.add_reads().len());
        assert_eq!(1, config.add_exports().len());
        assert_eq!(1, config.add_opens().len());
        assert_eq!(1, config.patch_modules().len());
    }
}
