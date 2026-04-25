use crate::intrinsic_methods::intrinsics;
use crate::thread::Thread;
use crate::{Parameters, Result};
use ahash::AHashMap;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tracing::error;

/// An intrinsic method represents a native Java method required by the Java Virtual Machine (JVM)
/// that is implemented in Rust.
///
/// Intrinsic methods are native functions that implement Java functionality directly
/// in Rust rather than in Java bytecode. These methods are registered with the VM
/// and are called when their corresponding Java native methods are invoked.
///
/// # Usage
///
/// Intrinsic methods are registered in the `MethodRegistry` with their corresponding
/// Java class name, method name, and method descriptor. When a Java program calls
/// a native method, the VM looks up the implementation in this registry and executes
/// the corresponding Rust function.
#[cfg(not(target_family = "wasm"))]
pub type IntrinsicMethod = fn(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Pin<Box<dyn Future<Output = Result<Option<Value>>> + Send>>;

/// An intrinsic method represents a native Java method required by the Java Virtual Machine (JVM)
/// that is implemented in Rust.
///
/// Intrinsic methods are native functions that implement Java functionality directly
/// in Rust rather than in Java bytecode. These methods are registered with the VM
/// and are called when their corresponding Java native methods are invoked.
///
/// # Usage
///
/// Intrinsic methods are registered in the `MethodRegistry` with their corresponding
/// Java class name, method name, and method descriptor. When a Java program calls
/// a native method, the VM looks up the implementation in this registry and executes
/// the corresponding Rust function.
#[cfg(target_family = "wasm")]
pub type IntrinsicMethod = fn(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Pin<Box<dyn Future<Output = Result<Option<Value>>>>>;

/// Registry for mapping Java intrinsic methods to their Rust implementations.
///
/// The `MethodRegistry` maintains a mapping between Java native methods and their
/// corresponding Rust implementations (intrinsic methods). It's a core component
/// of the JVM implementation that handles the execution of native code when
/// Java code calls a native method.
///
/// # Java Version Support
///
/// The registry is version-aware and will register different sets of native methods
/// based on the Java version being targeted. This allows for compatibility with
/// multiple Java versions while providing the appropriate native implementations
/// for each version.
///
/// # Registration Process
///
/// Methods are registered with a unique key composed of:
/// - The fully qualified class name (e.g., `java/lang/Object`)
/// - The method name (e.g., `hashCode`)
/// - The method descriptor (e.g., `()I`)
///
/// This forms a fully qualified method signature like `java/lang/Object.hashCode()I`.
#[derive(Debug)]
pub struct MethodRegistry {
    methods: &'static AHashMap<&'static str, IntrinsicMethod>,
}

impl MethodRegistry {
    /// Creates a new method registry configured for the specified Java major version.
    ///
    /// This constructor initializes an empty registry that will be configured for the specified
    /// Java major version. The version determines which set of native methods will be registered
    /// when `initialize()` is called.
    ///
    /// # Arguments
    ///
    /// `java_major_version` - The major Java version number (e.g., 8, 11, 17, 21, 24)
    ///
    /// # Returns
    ///
    /// A new empty `MethodRegistry` configured for the specified Java version.
    #[inline]
    #[must_use]
    pub fn new(version: &Version) -> Self {
        let methods: &'static AHashMap<&'static str, IntrinsicMethod> = match version.major() {
            69.. => &intrinsics::JAVA_25,
            65.. => &intrinsics::JAVA_21,
            61.. => &intrinsics::JAVA_17,
            55.. => &intrinsics::JAVA_11,
            _ => &intrinsics::JAVA_8,
        };
        MethodRegistry { methods }
    }

    /// Returns a reference to the map of all registered intrinsic methods.
    ///
    /// This function provides access to the internal map that stores all registered intrinsic
    /// methods. The keys of the map are method signatures, while the values are the
    /// `IntrinsicMethod` function pointers.
    pub(crate) fn methods(&self) -> &'static AHashMap<&'static str, IntrinsicMethod> {
        self.methods
    }

    /// Returns the set of intrinsic method signatures that *would* be registered for the given
    /// Java major version on the given target OS, regardless of the host platform on which the
    /// VM was compiled.
    ///
    /// This is intended for cross OS introspection from tests; the slice is plain string data
    /// (no function pointers) so it is always populated, including for OSes the current host is
    /// not actually compiled for.
    ///
    /// `os` accepts `"macos"`, `"linux"`, or `"windows"`. Any other value yields an empty slice.
    #[cfg(test)]
    #[must_use]
    pub fn signatures_for_os(version: &Version, os: &str) -> &'static [&'static str] {
        match (version.major(), os) {
            (69.., "macos") => intrinsics::JAVA_25_MACOS_SIGNATURES,
            (69.., "linux") => intrinsics::JAVA_25_LINUX_SIGNATURES,
            (69.., "windows") => intrinsics::JAVA_25_WINDOWS_SIGNATURES,
            (65.., "macos") => intrinsics::JAVA_21_MACOS_SIGNATURES,
            (65.., "linux") => intrinsics::JAVA_21_LINUX_SIGNATURES,
            (65.., "windows") => intrinsics::JAVA_21_WINDOWS_SIGNATURES,
            (61.., "macos") => intrinsics::JAVA_17_MACOS_SIGNATURES,
            (61.., "linux") => intrinsics::JAVA_17_LINUX_SIGNATURES,
            (61.., "windows") => intrinsics::JAVA_17_WINDOWS_SIGNATURES,
            (55.., "macos") => intrinsics::JAVA_11_MACOS_SIGNATURES,
            (55.., "linux") => intrinsics::JAVA_11_LINUX_SIGNATURES,
            (55.., "windows") => intrinsics::JAVA_11_WINDOWS_SIGNATURES,
            (_, "macos") => intrinsics::JAVA_8_MACOS_SIGNATURES,
            (_, "linux") => intrinsics::JAVA_8_LINUX_SIGNATURES,
            (_, "windows") => intrinsics::JAVA_8_WINDOWS_SIGNATURES,
            _ => &[],
        }
    }

    /// Looks up a intrinsic method implementation by its fully qualified signature.
    ///
    /// This method attempts to find an intrinsic method implementation by its Java class name,
    /// method name, and method descriptor. If found, it returns a reference to the
    /// `IntrinsicMethod` function; otherwise, it returns `None`.
    pub(crate) fn method(
        &self,
        class_name: &str,
        method_name: &str,
        method_descriptor: &str,
    ) -> Option<&IntrinsicMethod> {
        // Try to use a stack buffer to avoid heap allocation for typical method signatures
        // Most Java method signatures are under 256 bytes
        const STACK_BUFFER_SIZE: usize = 256;
        let total_len = class_name.len() + 1 + method_name.len() + method_descriptor.len();

        if total_len <= STACK_BUFFER_SIZE {
            // Fast path: use stack buffer for small signatures
            let mut buffer = [0u8; STACK_BUFFER_SIZE];
            let mut pos = 0;

            buffer[pos..pos + class_name.len()].copy_from_slice(class_name.as_bytes());
            pos += class_name.len();
            buffer[pos] = b'.';
            pos += 1;
            buffer[pos..pos + method_name.len()].copy_from_slice(method_name.as_bytes());
            pos += method_name.len();
            buffer[pos..pos + method_descriptor.len()]
                .copy_from_slice(method_descriptor.as_bytes());
            pos += method_descriptor.len();

            let Ok(method_signature) = std::str::from_utf8(&buffer[..pos]) else {
                error!(
                    "Failed to construct method signature for {class_name}.{method_name}{method_descriptor}"
                );
                return None;
            };
            self.methods.get(method_signature)
        } else {
            // Slow path: fall back to heap allocation for unusually long signatures
            let mut method_signature = String::with_capacity(
                class_name.len() + 1 + method_name.len() + method_descriptor.len(),
            );
            method_signature.push_str(class_name);
            method_signature.push('.');
            method_signature.push_str(method_name);
            method_signature.push_str(method_descriptor);
            self.methods.get(method_signature.as_str())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vm;
    use ristretto_classfile::{JAVA_21, JavaStr};
    use ristretto_classloader::runtime;
    use ristretto_classloader::{
        JAVA_8_VERSION, JAVA_11_VERSION, JAVA_17_VERSION, JAVA_21_VERSION, JAVA_25_VERSION,
    };

    #[tokio::test]
    async fn test_method() -> Result<()> {
        let method_registry = MethodRegistry::new(&JAVA_21);
        let result = method_registry.method("java/lang/Object", "hashCode", "()I");
        assert!(result.is_some());
        Ok(())
    }

    #[tokio::test]
    async fn test_method_not_found() -> Result<()> {
        let method_registry = MethodRegistry::new(&JAVA_21);
        let result = method_registry.method("foo", "hashCode", "()I");
        assert!(result.is_none());
        Ok(())
    }

    /// Get all the intrinsic methods for a given Java runtime targeting the specified OS / arch.
    async fn get_intrinsic_methods(version: &str, os: &str, arch: &str) -> Result<Vec<String>> {
        let (_java_home, _java_version, class_loader) =
            runtime::version_class_loader_for_os(version, os, arch).await?;
        let class_path = class_loader.class_path();
        let class_names = class_path.class_names().await?;
        let mut intrinsic_methods = Vec::new();
        for class_name in class_names {
            let lower_class_name = class_name.to_lowercase();
            // Skip GraalVM and Hotspot classes
            if lower_class_name.contains("graalvm") || lower_class_name.contains("hotspot") {
                continue;
            }

            let class = class_loader
                .load(JavaStr::try_from_str(&class_name)?)
                .await?;
            for method in class.methods() {
                if method.is_native() {
                    let method_name = method.name();
                    let method_descriptor = method.descriptor();
                    intrinsic_methods
                        .push(format!("{class_name}.{method_name}{method_descriptor}"));
                }
            }
        }
        intrinsic_methods.sort();
        Ok(intrinsic_methods)
    }

    /// Get all the methods registered for a given Java version on the given target OS,
    /// regardless of the host platform.
    fn get_registry_methods(version: &str, os: &str) -> Result<Vec<String>> {
        let version_major = version.split_once('.').unwrap_or_default().0;
        let java_major_version: u16 = version_major.parse()?;
        let class_file_version =
            Version::from(java_major_version + vm::CLASS_FILE_MAJOR_VERSION_OFFSET, 0)?;
        let mut registry_methods: Vec<String> =
            MethodRegistry::signatures_for_os(&class_file_version, os)
                .iter()
                .map(|s| (*s).to_string())
                .collect();
        registry_methods.sort();
        Ok(registry_methods)
    }

    /// Verify that all the intrinsic methods are registered for a given runtime targeting the
    /// specified OS / architecture.
    fn test_runtime(version: &str, os: &str, arch: &str) -> Result<()> {
        let intrinsic_methods = {
            let runtime = tokio::runtime::Runtime::new()?;
            runtime.block_on(get_intrinsic_methods(version, os, arch))?
        };
        let registry_methods = get_registry_methods(version, os)?;

        // Parse Java major version to conditionally include methods
        let version_major: u16 = version.split_once('.').unwrap_or_default().0.parse()?;

        // Required methods for ristretto; some are version-specific or OS-specific
        let mut required_methods = vec![
            "java/lang/System.allowSecurityManager()Z".to_string(),
            "java/lang/System.getSecurityManager()Ljava/lang/SecurityManager;".to_string(),
            "java/lang/System.setSecurityManager(Ljava/lang/SecurityManager;)V".to_string(),
            "jdk/internal/module/ModuleBootstrap.boot()Ljava/lang/ModuleLayer;".to_string(),
        ];
        // Ristretto-specific intrinsic needed to mask ACC_SUPER from class modifiers; the
        // JDK exposes this as a non-native method in Java 25+, but we still require the
        // intrinsic dispatch.
        if version_major >= 25 {
            required_methods.push("java/lang/Class.getModifiers()I".to_string());
        }
        // initSystemClassLoader is only a native method in Java 8 and earlier
        if version_major <= 8 {
            required_methods.push(
                "java/lang/ClassLoader.initSystemClassLoader()Ljava/lang/ClassLoader;".to_string(),
            );
        }

        let missing_required_methods = required_methods
            .iter()
            .filter(|method| !registry_methods.contains(method))
            .cloned()
            .collect::<Vec<String>>();
        let missing_methods = intrinsic_methods
            .iter()
            .filter(|method| !registry_methods.contains(method))
            .cloned()
            .collect::<Vec<String>>();
        let extra_methods = registry_methods
            .iter()
            .filter(|method| {
                !intrinsic_methods.contains(method) && !required_methods.contains(method)
            })
            .cloned()
            .collect::<Vec<String>>();

        let mut errors = Vec::new();
        if !missing_required_methods.is_empty() {
            errors.push(format!(
                "[{os}-{arch}] Missing required methods {}:\n{}\n",
                missing_required_methods.len(),
                missing_required_methods.join("\n"),
            ));
        }
        if !missing_methods.is_empty() {
            errors.push(format!(
                "[{os}-{arch}] Missing methods {}:\n{}\n",
                missing_methods.len(),
                missing_methods.join("\n"),
            ));
        }
        if !extra_methods.is_empty() {
            errors.push(format!(
                "[{os}-{arch}] Extra methods {}:\n{}\n",
                extra_methods.len(),
                extra_methods.join("\n"),
            ));
        }
        let errors = errors.join("\n");
        assert_eq!("", errors);
        Ok(())
    }

    #[test]
    fn test_runtime_v8_linux() -> Result<()> {
        test_runtime(JAVA_8_VERSION, "linux", "x64")
    }

    #[test]
    fn test_runtime_v8_macos() -> Result<()> {
        test_runtime(JAVA_8_VERSION, "macos", "aarch64")
    }

    #[test]
    fn test_runtime_v8_windows() -> Result<()> {
        test_runtime(JAVA_8_VERSION, "windows", "x64")
    }

    #[test]
    fn test_runtime_v11_linux() -> Result<()> {
        test_runtime(JAVA_11_VERSION, "linux", "x64")
    }

    #[test]
    fn test_runtime_v11_macos() -> Result<()> {
        test_runtime(JAVA_11_VERSION, "macos", "aarch64")
    }

    #[test]
    fn test_runtime_v11_windows() -> Result<()> {
        test_runtime(JAVA_11_VERSION, "windows", "x64")
    }

    #[test]
    fn test_runtime_v17_linux() -> Result<()> {
        test_runtime(JAVA_17_VERSION, "linux", "x64")
    }

    #[test]
    fn test_runtime_v17_macos() -> Result<()> {
        test_runtime(JAVA_17_VERSION, "macos", "aarch64")
    }

    #[test]
    fn test_runtime_v17_windows() -> Result<()> {
        test_runtime(JAVA_17_VERSION, "windows", "x64")
    }

    #[test]
    fn test_runtime_v21_linux() -> Result<()> {
        test_runtime(JAVA_21_VERSION, "linux", "x64")
    }

    #[test]
    fn test_runtime_v21_macos() -> Result<()> {
        test_runtime(JAVA_21_VERSION, "macos", "aarch64")
    }

    #[test]
    fn test_runtime_v21_windows() -> Result<()> {
        test_runtime(JAVA_21_VERSION, "windows", "x64")
    }

    #[test]
    fn test_runtime_v25_linux() -> Result<()> {
        test_runtime(JAVA_25_VERSION, "linux", "x64")
    }

    #[test]
    fn test_runtime_v25_macos() -> Result<()> {
        test_runtime(JAVA_25_VERSION, "macos", "aarch64")
    }

    #[test]
    fn test_runtime_v25_windows() -> Result<()> {
        test_runtime(JAVA_25_VERSION, "windows", "x64")
    }
}
