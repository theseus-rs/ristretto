use crate::Error::InternalError;
use crate::java_object::JavaObject;
use crate::thread::Thread;
use crate::{Result, VM};
use ahash::AHashMap;
use ristretto_classloader::Value;
use std::env;
use std::env::consts::{ARCH, OS};
use std::path::MAIN_SEPARATOR_STR;
use std::sync::Arc;

/// Retrieves system properties as Java objects for the JVM.
///
/// This function fetches all system properties from the VM and converts each string value
/// into a Java object representation. The properties include information about the runtime
/// environment such as OS details, file paths, user information, Java version, etc.
///
/// # Note
///
/// This function is called during Java system initialization when`System.getProperties()` is
/// invoked from Java code.
pub(crate) async fn system(thread: &Arc<Thread>) -> Result<AHashMap<&'static str, Value>> {
    let vm = thread.vm()?;
    let system_properties = system_properties(&vm)?;
    let mut properties = AHashMap::default();
    for (key, value) in system_properties {
        let value = value.to_object(thread).await?;
        properties.insert(key, value);
    }
    Ok(properties)
}

/// Creates a map of all standard Java system properties.
///
/// This function initializes a collection of system properties that should be available in a Java
/// runtime environment. It includes properties related to:
/// - File system information (separators, encoding)
/// - Locale and formatting settings
/// - Network proxy configurations
/// - Java runtime details (version, class path, VM specification)
/// - Operating system information
/// - User environment (home directory, username, working directory)
///
/// # Note
///
/// This function is used internally during JVM initialization or when properties need to be exposed
/// to Java code via `System.getProperties()`.
#[expect(clippy::too_many_lines)]
fn system_properties(vm: &VM) -> Result<AHashMap<&'static str, String>> {
    let mut properties = AHashMap::default();
    let java_home = vm.java_home().to_string_lossy().to_string();
    let class_file_version = vm.java_class_file_version();
    let major_java_version = class_file_version.java();
    let major_class_version = class_file_version.major();
    let minor_class_version = class_file_version.minor();
    let locale = sys_locale::get_locale().unwrap_or_else(|| String::from("en-US"));
    let locale_parts = locale.split('-').collect::<Vec<&str>>();
    let language = *locale_parts.first().unwrap_or(&"en");
    let country = *locale_parts.get(1).unwrap_or(&"");

    properties.insert("file.encoding", "UTF-8".to_string());
    properties.insert("file.separator", MAIN_SEPARATOR_STR.to_string());

    properties.insert("format.country", country.to_string());
    properties.insert("format.language", language.to_string());
    // TODO: implement format.script
    properties.insert("format.script", String::new());
    // TODO: implement format.variant
    properties.insert("format.variant", String::new());

    // TODO: implement ftp.nonProxyHosts
    properties.insert("ftp.nonProxyHosts", String::new());
    // TODO: implement ftp.proxyHost
    properties.insert("ftp.proxyHost", String::new());
    // TODO: implement ftp.proxyPort
    properties.insert("ftp.proxyPort", String::new());

    // TODO: implement http.nonProxyHosts
    properties.insert("http.nonProxyHosts", String::new());
    // TODO: implement http.proxyHost
    properties.insert("http.proxyHost", String::new());
    // TODO: implement http.proxyPort
    properties.insert("http.proxyPort", String::new());
    // TODO: implement https.proxyHost
    properties.insert("https.proxyHost", String::new());
    // TODO: implement https.proxyPort
    properties.insert("https.proxyPort", String::new());

    let class_path = vm.configuration().class_path().to_string();
    properties.insert("java.class.path", class_path);
    properties.insert(
        "java.class.version",
        format!("{major_class_version}.{minor_class_version}"),
    );
    properties.insert("java.compiler", "no JIT".to_string());
    // TODO: implement java.ext.dirs
    properties.insert("java.ext.dirs", String::new());
    properties.insert("java.home", java_home);

    let tmp_dir = env::temp_dir();
    properties.insert("java.io.tmpdir", format!("{}", tmp_dir.to_string_lossy()));

    // TODO: implement java.library.path
    properties.insert("java.library.path", String::new());
    properties.insert(
        "java.specification.name",
        "Java Platform API Specification".to_string(),
    );
    properties.insert(
        "java.specification.vendor",
        "Oracle Corporation".to_string(),
    );
    // TODO: implement java.specification.maintenance.version
    properties.insert("java.specification.maintenance.version", String::new());
    properties.insert("java.vendor", "ristretto".to_string());
    properties.insert(
        "java.vendor.url",
        "https://github.com/theseus-rs/ristretto".to_string(),
    );
    let vm_version = env!("CARGO_PKG_VERSION");
    properties.insert("java.vendor.version", vm_version.to_string());
    let java_version = vm.java_version();
    properties.insert("java.version", java_version.to_string());
    let architecture_bits = usize::BITS;
    let vm_name =
        format!("ristretto {vm_version} (Java {java_version}) {architecture_bits}-bit VM");
    properties.insert("java.vm.name", vm_name);
    properties.insert(
        "java.vm.specification.name",
        "Java Virtual Machine Specification".to_string(),
    );
    properties.insert(
        "java.vm.specification.vendor",
        "Oracle and Ristretto".to_string(),
    );
    properties.insert(
        "java.vm.specification.version",
        format!("{major_java_version}"),
    );
    properties.insert("java.vm.vendor", "ristretto".to_string());
    properties.insert("java.vm.version", vm_version.to_string());

    #[cfg(not(target_os = "windows"))]
    properties.insert("line.separator", "\n".to_string());
    #[cfg(target_os = "windows")]
    properties.insert("line.separator", "\r\n".to_string());

    properties.insert("native.encoding", "UTF8".to_string());

    properties.insert("os.arch", ARCH.to_string());
    let os = match OS {
        "linux" => "Linux",
        "macos" => "Mac OS X",
        "windows" => "Windows",
        _ => OS,
    };
    properties.insert("os.name", os.to_string());

    let os_information = os_info::get();
    let os_version = format!("{}", os_information.version());
    properties.insert("os.version", os_version);

    #[cfg(not(target_os = "windows"))]
    properties.insert("path.separator", ":".to_string());
    #[cfg(target_os = "windows")]
    properties.insert("path.separator", ";".to_string());

    // TODO: implement socksNonProxyHosts
    properties.insert("socksNonProxyHosts", String::new());
    // TODO: implement socksProxyHost
    properties.insert("socksProxyHost", String::new());
    // TODO: implement socksProxyPort
    properties.insert("socksProxyPort", String::new());

    properties.insert("stderr.encoding", "UTF-8".to_string());
    properties.insert("stdin.encoding", "UTF-8".to_string());
    properties.insert("stdout.encoding", "UTF-8".to_string());

    // TODO: implement sun.arch.abi
    properties.insert("sun.arch.abi", String::new());
    properties.insert("sun.arch.data.model", format!("{architecture_bits}"));
    #[cfg(target_endian = "little")]
    properties.insert("sun.cpu.endian", "little".to_string());
    #[cfg(target_endian = "big")]
    properties.insert("sun.cpu.endian", "big".to_string());
    // TODO: implement sun.cpu.isalist
    properties.insert("sun.cpu.isalist", String::new());
    properties.insert("sun.io.unicode.encoding", "UnicodeBig".to_string());
    properties.insert("sun.jnu.encoding", "UTF-8".to_string());
    // TODO: implement sun.os.patch.level
    properties.insert("sun.os.patch.level", String::new());
    properties.insert("sun.stderr.encoding", "UTF-8".to_string());
    properties.insert("sun.stdout.encoding", "UTF-8".to_string());

    properties.insert("user.country", country.to_string());
    let current_dir = env::current_dir().map_err(|error| InternalError(error.to_string()))?;
    properties.insert("user.dir", format!("{}", current_dir.to_string_lossy()));
    let home_dir = dirs::home_dir().unwrap_or_default();
    properties.insert("user.home", format!("{}", home_dir.to_string_lossy()));
    properties.insert("user.language", language.to_string());
    let username = whoami::username().map_err(|error| InternalError(error.to_string()))?;
    properties.insert("user.name", username);
    // TODO: implement user.script
    properties.insert("user.script", String::new());
    // TODO: implement user.variant
    properties.insert("user.variant", String::new());
    Ok(properties)
}
