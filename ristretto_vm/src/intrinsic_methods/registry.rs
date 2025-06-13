use crate::Result;
use crate::intrinsic_methods::intrinsics;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::{JAVA_11, JAVA_17, JAVA_21, JAVA_24, Version};
use ristretto_classloader::Value;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

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
    methods: &'static phf::Map<&'static str, IntrinsicMethod>,
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
    pub fn new(version: &Version) -> Self {
        let java_major_version = version.java();
        let methods = if java_major_version >= JAVA_24.java() {
            &intrinsics::JAVA_24
        } else if java_major_version >= JAVA_21.java() {
            &intrinsics::JAVA_21
        } else if java_major_version >= JAVA_17.java() {
            &intrinsics::JAVA_17
        } else if java_major_version >= JAVA_11.java() {
            &intrinsics::JAVA_11
        } else {
            &intrinsics::JAVA_8
        };
        MethodRegistry { methods }
    }

    /// Returns a reference to the map of all registered intrinsic methods.
    ///
    /// This function provides access to the internal map that stores all registered intrinsic
    /// methods. The keys of the map are method signatures, while the values are the
    /// `IntrinsicMethod` function pointers.
    pub(crate) fn methods(&self) -> &'static phf::Map<&'static str, IntrinsicMethod> {
        self.methods
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
        // Create the method signature in the format "class_name.method_name(method_descriptor)"
        // This is an optimization over using format!() for string concatenation.
        let mut method_signature = String::with_capacity(
            class_name.len() + 1 + method_name.len() + method_descriptor.len(),
        );
        method_signature.push_str(class_name);
        method_signature.push('.');
        method_signature.push_str(method_name);
        method_signature.push_str(method_descriptor);

        self.methods.get(&method_signature)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vm;
    use ristretto_classloader::runtime;

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

    /// Get all the intrinsic methods for a given Java runtime.
    async fn get_intrinsic_methods(version: &str) -> Result<Vec<String>> {
        let (_java_home, _java_version, class_loader) =
            runtime::version_class_loader(version).await?;
        let class_path = class_loader.class_path();
        let class_names = class_path.class_names().await?;
        let mut intrinsic_methods = Vec::new();
        for class_name in class_names {
            let lower_class_name = class_name.to_lowercase();
            // Skip GraalVM and Hotspot classes
            if lower_class_name.contains("graalvm") || lower_class_name.contains("hotspot") {
                continue;
            }

            let class = class_loader.load(&class_name).await?;
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

    /// Get all the methods for a given Java version.
    fn get_registry_methods(version: &str) -> Result<Vec<String>> {
        let version_major = version.split_once('.').unwrap_or_default().0;
        let java_major_version: u16 = version_major.parse()?;
        let version = Version::from(java_major_version + vm::CLASS_FILE_MAJOR_VERSION_OFFSET, 0)?;
        let method_registry = MethodRegistry::new(&version);
        let mut registry_methods = method_registry
            .methods()
            .keys()
            .filter(|&method| {
                // Skip internal JVM runtime classes
                !method.starts_with("java/lang/invoke/DirectMethodHandle$Holder")
            })
            .map(|method| method.to_string())
            .collect::<Vec<String>>();
        registry_methods.sort();
        Ok(registry_methods)
    }

    /// Verify that all the intrinsic methods are registered for a given runtime
    async fn test_runtime(version: &str) -> Result<()> {
        #[cfg(target_os = "macos")]
        let intrinsic_methods = get_intrinsic_methods(version).await?;
        let registry_methods = get_registry_methods(version)?;
        // Required methods for ristretto
        #[expect(unused_mut)]
        #[expect(clippy::useless_vec)]
        let mut required_methods = vec![
            "java/lang/ClassLoader.initSystemClassLoader()Ljava/lang/ClassLoader;".to_string(),
            "java/lang/System.allowSecurityManager()Z".to_string(),
            "java/lang/System.getSecurityManager()Ljava/lang/SecurityManager;".to_string(),
            "java/lang/System.setSecurityManager(Ljava/lang/SecurityManager;)V".to_string(),
            "jdk/internal/module/ModuleBootstrap.boot()Ljava/lang/ModuleLayer;".to_string(),
        ];
        #[cfg(target_os = "windows")]
        {
            required_methods.push("java/io/WinNTFileSystem.initIDs()V".to_string());
            required_methods.push("sun/io/Win32ErrorMode.setErrorMode(J)J".to_string());
        }

        let missing_required_methods = required_methods
            .iter()
            .filter(|method| !registry_methods.contains(method))
            .cloned()
            .collect::<Vec<String>>();
        #[cfg(target_os = "macos")]
        let missing_methods = intrinsic_methods
            .iter()
            .filter(|method| !registry_methods.contains(method))
            .cloned()
            .collect::<Vec<String>>();
        // Disable the check for extra methods for now as the OS intrinsic methods are not excluded
        // from the registry methods.
        // let extra_methods = registry_methods
        //     .iter()
        //     .filter(|method| {
        //         !intrinsic_methods.contains(method) && !required_methods.contains(method)
        //     })
        //     .cloned()
        //     .collect::<Vec<String>>();
        let extra_methods = Vec::<String>::new();

        let mut errors = Vec::new();
        if !missing_required_methods.is_empty() {
            errors.push(format!(
                "Missing required methods {}:\n{}\n",
                missing_required_methods.len(),
                missing_required_methods.join("\n"),
            ));
        }
        #[cfg(target_os = "macos")]
        if !missing_methods.is_empty() {
            errors.push(format!(
                "Missing methods {}:\n{}\n",
                missing_methods.len(),
                missing_methods.join("\n"),
            ));
        }
        if !extra_methods.is_empty() {
            errors.push(format!(
                "Extra methods {}:\n{}\n",
                extra_methods.len(),
                extra_methods.join("\n"),
            ));
        }
        let errors = errors.join("\n");
        assert_eq!("", errors);
        Ok(())
    }

    #[tokio::test]
    async fn test_runtime_v8() -> Result<()> {
        test_runtime("8.452.09.1").await
    }

    #[tokio::test]
    async fn test_runtime_v11() -> Result<()> {
        test_runtime("11.0.27.6.1").await
    }

    #[tokio::test]
    async fn test_runtime_v17() -> Result<()> {
        test_runtime("17.0.15.6.1").await
    }

    #[tokio::test]
    async fn test_runtime_v21() -> Result<()> {
        test_runtime("21.0.7.6.1").await
    }

    #[tokio::test]
    async fn test_runtime_v24() -> Result<()> {
        test_runtime("24.0.1.9.1").await
    }
}
