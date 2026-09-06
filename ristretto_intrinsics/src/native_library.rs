//! Per-VM dynamic native-library management.

#![allow(unsafe_code)]

#[cfg(not(target_family = "wasm"))]
use libloading::Library;
use parking_lot::RwLock;
use portable_atomic::{AtomicI64, Ordering};
use ristretto_types::JavaError::UnsatisfiedLinkError;
use ristretto_types::{Result, VM};
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
#[cfg(not(target_family = "wasm"))]
use std::path::Path;
use std::sync::Arc;

/// JNI interface versions defined by the JNI specification.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum JniVersion {
    /// JNI 1.1.
    V1_1 = 0x0001_0001,
    /// JNI 1.2.
    V1_2 = 0x0001_0002,
    /// JNI 1.4.
    V1_4 = 0x0001_0004,
    /// JNI 1.6.
    V1_6 = 0x0001_0006,
    /// JNI 1.8.
    V1_8 = 0x0001_0008,
    /// JNI 9.
    V9 = 0x0009_0000,
    /// JNI 10.
    V10 = 0x000a_0000,
    /// JNI 19.
    V19 = 0x0013_0000,
    /// JNI 20.
    V20 = 0x0014_0000,
    /// JNI 21.
    V21 = 0x0015_0000,
    /// JNI 24.
    V24 = 0x0018_0000,
}

impl From<JniVersion> for i32 {
    fn from(version: JniVersion) -> Self {
        version as Self
    }
}

enum LibraryKind {
    Intrinsic,
    #[cfg(not(target_family = "wasm"))]
    Dynamic(Library),
}

struct LoadedLibrary {
    name: String,
    kind: LibraryKind,
}

impl fmt::Debug for LoadedLibrary {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LoadedLibrary")
            .field("name", &self.name)
            .field(
                "kind",
                &match self.kind {
                    LibraryKind::Intrinsic => "intrinsic",
                    #[cfg(not(target_family = "wasm"))]
                    LibraryKind::Dynamic(_) => "dynamic",
                },
            )
            .finish()
    }
}

/// Native libraries loaded by a single virtual machine.
pub struct NativeLibraries {
    next_handle: AtomicI64,
    libraries: RwLock<HashMap<i64, LoadedLibrary>>,
    initialized: RwLock<HashSet<i64>>,
}

impl fmt::Debug for NativeLibraries {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("NativeLibraries")
            .field("next_handle", &self.next_handle.load(Ordering::Relaxed))
            .field("library_count", &self.libraries.read().len())
            .field("initialized_count", &self.initialized.read().len())
            .finish()
    }
}

impl Default for NativeLibraries {
    fn default() -> Self {
        Self {
            next_handle: AtomicI64::new(1),
            libraries: RwLock::new(HashMap::new()),
            initialized: RwLock::new(HashSet::new()),
        }
    }
}

impl NativeLibraries {
    /// Loads a native library and returns its opaque VM handle and JNI version.
    ///
    /// # Errors
    ///
    /// Returns an error when the host dynamic loader cannot open `name`.
    pub fn load(&self, name: &str, intrinsic: bool) -> Result<(i64, JniVersion)> {
        if let Some((&handle, _)) = self
            .libraries
            .read()
            .iter()
            .find(|(_, library)| library.name == name)
        {
            return Ok((handle, JniVersion::V1_6));
        }

        let kind = if intrinsic {
            LibraryKind::Intrinsic
        } else {
            #[cfg(target_family = "wasm")]
            {
                return Err(UnsatisfiedLinkError(
                    "dynamic native libraries are not supported on WebAssembly".to_string(),
                )
                .into());
            }
            #[cfg(not(target_family = "wasm"))]
            {
                // SAFETY: loading arbitrary native code is the operation requested by
                // System.load/System.loadLibrary. The Library value remains stored until unload.
                let library = unsafe { Library::new(Path::new(name)) }
                    .map_err(|error| UnsatisfiedLinkError(error.to_string()))?;
                LibraryKind::Dynamic(library)
            }
        };
        let handle = self.next_handle.fetch_add(1, Ordering::Relaxed);
        self.libraries.write().insert(
            handle,
            LoadedLibrary {
                name: name.to_string(),
                kind,
            },
        );
        Ok((handle, JniVersion::V1_6))
    }

    /// Looks up a native symbol in the library identified by `handle`.
    #[must_use]
    pub fn find(&self, handle: i64, name: &str) -> Option<i64> {
        #[cfg(not(target_family = "wasm"))]
        {
            let libraries = self.libraries.read();
            let library = libraries.get(&handle)?;
            let LibraryKind::Dynamic(library) = &library.kind else {
                return None;
            };
            // SAFETY: the symbol is not invoked here. Its address is valid while the Library is
            // retained in this manager.
            let symbol = unsafe { library.get::<unsafe extern "C" fn()>(name.as_bytes()) }.ok()?;
            let address = *symbol as usize;
            i64::try_from(address).ok()
        }
        #[cfg(target_family = "wasm")]
        {
            let _ = (handle, name);
            None
        }
    }

    /// Looks up a native symbol in the most recently loaded matching library.
    #[must_use]
    pub fn find_any(&self, names: &[String]) -> Option<i64> {
        self.resolve(names).map(|(_, address)| address)
    }

    /// Resolves a symbol and returns both its owning library and address.
    #[must_use]
    pub fn resolve(&self, names: &[String]) -> Option<(i64, i64)> {
        #[cfg(not(target_family = "wasm"))]
        {
            let libraries = self.libraries.read();
            for (handle, library) in libraries.iter() {
                let LibraryKind::Dynamic(dynamic) = &library.kind else {
                    continue;
                };
                for name in names {
                    // SAFETY: the symbol is not invoked here and the library remains loaded.
                    if let Ok(symbol) =
                        unsafe { dynamic.get::<unsafe extern "C" fn()>(name.as_bytes()) }
                    {
                        let address = *symbol as usize;
                        if let Ok(address) = i64::try_from(address) {
                            return Some((*handle, address));
                        }
                    }
                }
            }
            None
        }
        #[cfg(target_family = "wasm")]
        {
            let _ = names;
            None
        }
    }

    /// Returns whether `JNI_OnLoad` still needs to run for a library.
    #[must_use]
    pub fn needs_initialization(&self, handle: i64) -> bool {
        !self.initialized.read().contains(&handle)
    }

    /// Marks a library's `JNI_OnLoad` initialization as complete.
    pub fn mark_initialized(&self, handle: i64) {
        self.initialized.write().insert(handle);
    }

    /// Unloads a library. Dropping the stored `Library` releases the host handle.
    pub fn unload(&self, handle: i64) {
        self.initialized.write().remove(&handle);
        self.libraries.write().remove(&handle);
    }
}

/// Returns the per-VM native-library manager.
///
/// # Errors
///
/// Returns an error if the VM resource manager cannot create or retrieve the manager.
pub fn libraries<V: VM + ?Sized>(vm: &V) -> Result<Arc<NativeLibraries>> {
    vm.resource_manager().get_or_init(NativeLibraries::default)
}

/// Returns true for standard-library native libraries implemented by Ristretto intrinsics.
#[must_use]
pub fn is_intrinsic_native_library(path: &str) -> bool {
    let file_name = path.rsplit(['/', '\\']).next().unwrap_or(path);
    let without_prefix = file_name.strip_prefix("lib").unwrap_or(file_name);
    let stem = [".dylib", ".jnilib", ".dll", ".so", ".a"]
        .iter()
        .find_map(|suffix| without_prefix.strip_suffix(suffix))
        .unwrap_or(without_prefix);
    matches!(
        stem,
        "awt"
            | "javajpeg"
            | "jpeg"
            | "jimage"
            | "net"
            | "nio"
            | "prefs"
            | "sctp"
            | "unpack"
            | "zip"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jni_versions() {
        let versions = [
            (JniVersion::V1_1, 0x0001_0001),
            (JniVersion::V1_2, 0x0001_0002),
            (JniVersion::V1_4, 0x0001_0004),
            (JniVersion::V1_6, 0x0001_0006),
            (JniVersion::V1_8, 0x0001_0008),
            (JniVersion::V9, 0x0009_0000),
            (JniVersion::V10, 0x000a_0000),
            (JniVersion::V19, 0x0013_0000),
            (JniVersion::V20, 0x0014_0000),
            (JniVersion::V21, 0x0015_0000),
            (JniVersion::V24, 0x0018_0000),
        ];

        for (version, value) in versions {
            assert_eq!(i32::from(version), value);
        }
        assert_eq!("V1_6", format!("{:?}", JniVersion::V1_6));
    }

    #[test]
    fn test_default_and_debug() {
        let libraries = NativeLibraries::default();
        assert_eq!(
            "NativeLibraries { next_handle: 1, library_count: 0, initialized_count: 0 }",
            format!("{libraries:?}")
        );
    }

    #[test]
    fn test_load_intrinsic_and_unload() -> Result<()> {
        let libraries = NativeLibraries::default();

        let (handle, version) = libraries.load("zip", true)?;
        assert_eq!(handle, 1);
        assert_eq!(version, JniVersion::V1_6);
        assert_eq!(libraries.load("zip", true)?, (handle, version));
        assert_eq!(libraries.find(handle, "missing"), None);

        assert!(libraries.needs_initialization(handle));
        libraries.mark_initialized(handle);
        assert!(!libraries.needs_initialization(handle));

        libraries.unload(handle);
        assert!(libraries.needs_initialization(handle));
        assert_eq!(libraries.find(handle, "missing"), None);
        assert_eq!(libraries.load("zip", true)?.0, 2);
        Ok(())
    }

    #[test]
    fn test_load_missing_dynamic_library() {
        let libraries = NativeLibraries::default();
        let result = libraries.load(
            "/path/that/does/not/exist/libristretto_missing_native_library.so",
            false,
        );
        assert!(result.is_err());
    }

    #[cfg(all(target_family = "unix", not(target_family = "wasm")))]
    fn test_dynamic_library() -> (Library, &'static str) {
        (libloading::os::unix::Library::this().into(), "malloc")
    }

    #[cfg(target_family = "windows")]
    fn test_dynamic_library() -> (Library, &'static str) {
        // SAFETY: kernel32.dll is a Windows system library and remains owned by this test.
        let library = unsafe { Library::new("kernel32.dll") }.expect("load kernel32.dll");
        (library, "GetCurrentProcessId")
    }

    #[cfg(not(target_family = "wasm"))]
    #[test]
    fn test_find_and_resolve_dynamic_symbol() {
        let libraries = NativeLibraries::default();
        let (library, symbol_name) = test_dynamic_library();
        libraries.libraries.write().insert(
            7,
            LoadedLibrary {
                name: "test".to_string(),
                kind: LibraryKind::Dynamic(library),
            },
        );

        let address = libraries.find(7, symbol_name).expect("system symbol");
        assert_ne!(address, 0);
        assert_eq!(libraries.find(7, "ristretto_missing_symbol"), None);
        assert_eq!(libraries.find(8, symbol_name), None);

        let names = vec![
            "ristretto_missing_symbol".to_string(),
            symbol_name.to_string(),
        ];
        assert_eq!(libraries.find_any(&names), Some(address));
        assert_eq!(libraries.resolve(&names), Some((7, address)));
        assert!(format!("{:?}", libraries.libraries.read().get(&7)).contains("dynamic"));
    }

    #[test]
    fn test_resolve_without_dynamic_libraries() -> Result<()> {
        let libraries = NativeLibraries::default();
        libraries.load("net", true)?;
        let names = vec!["malloc".to_string()];
        assert_eq!(libraries.find_any(&names), None);
        assert_eq!(libraries.resolve(&names), None);
        Ok(())
    }

    #[tokio::test]
    async fn test_libraries_are_scoped_to_vm() -> Result<()> {
        let (vm, _thread) = crate::test::thread().await?;
        let first = libraries(vm.as_ref())?;
        let second = libraries(vm.as_ref())?;
        assert!(Arc::ptr_eq(&first, &second));
        Ok(())
    }

    #[test]
    fn test_is_intrinsic_native_library() {
        for path in [
            "awt",
            "javajpeg",
            "jpeg",
            "libjavajpeg.so",
            "javajpeg.dll",
            "jimage",
            "net",
            "nio.dll",
            "libprefs.so",
            "libsctp.so",
            "unpack",
            "/usr/lib/libzip.dylib",
            r"C:\Java\bin\net.dll",
            "libnio.jnilib",
            "libsctp.a",
        ] {
            assert!(is_intrinsic_native_library(path), "{path}");
        }

        for path in ["", "libjava.so", "network.dll", "libzip.so.1"] {
            assert!(!is_intrinsic_native_library(path), "{path}");
        }
    }
}
