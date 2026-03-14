use ahash::AHashMap;
use ristretto_classloader::Value;
use ristretto_types::Error::InternalError;
use ristretto_types::JavaObject;
use ristretto_types::Thread;
use ristretto_types::{Result, VM};
use std::borrow::Cow;
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
/// # Errors
/// Returns an error if the VM cannot be accessed or properties cannot be converted.
///
/// # Note
///
/// This function is called during Java system initialization when`System.getProperties()` is
/// invoked from Java code.
pub async fn system<T: Thread + 'static>(thread: &Arc<T>) -> Result<AHashMap<&'static str, Value>> {
    let vm = thread.vm()?;
    let system_properties = system_properties(&vm)?;
    let mut properties = AHashMap::with_capacity(system_properties.len());
    for (key, value) in system_properties {
        let value_str: &str = &value;
        let value = value_str.to_object(thread).await?;
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
fn system_properties<V: VM>(vm: &V) -> Result<AHashMap<&'static str, Cow<'static, str>>> {
    let mut properties = AHashMap::with_capacity(64);
    let java_home = vm.java_home().to_string_lossy().to_string();
    let class_file_version = vm.java_class_file_version();
    let major_java_version = class_file_version.java();
    let major_class_version = class_file_version.major();
    let minor_class_version = class_file_version.minor();
    let locale = sys_locale::get_locale().unwrap_or_else(|| String::from("en-US"));
    let locale_parts = locale.split('-').collect::<Vec<&str>>();
    let language = locale_parts.first().copied().unwrap_or("en");
    let country = locale_parts.get(1).copied().unwrap_or("");
    let language_owned = language.to_string();
    let country_owned = country.to_string();

    properties.insert("file.encoding", "UTF-8".into());
    properties.insert("file.separator", MAIN_SEPARATOR_STR.into());

    properties.insert("format.country", Cow::Owned(country_owned.clone()));
    properties.insert("format.language", Cow::Owned(language_owned.clone()));
    // TODO: implement format.script
    properties.insert("format.script", Cow::Borrowed(""));
    // TODO: implement format.variant
    properties.insert("format.variant", Cow::Borrowed(""));

    // TODO: implement ftp.nonProxyHosts
    properties.insert("ftp.nonProxyHosts", Cow::Borrowed(""));
    // TODO: implement ftp.proxyHost
    properties.insert("ftp.proxyHost", Cow::Borrowed(""));
    // TODO: implement ftp.proxyPort
    properties.insert("ftp.proxyPort", Cow::Borrowed(""));

    // TODO: implement http.nonProxyHosts
    properties.insert("http.nonProxyHosts", Cow::Borrowed(""));
    // TODO: implement http.proxyHost
    properties.insert("http.proxyHost", Cow::Borrowed(""));
    // TODO: implement http.proxyPort
    properties.insert("http.proxyPort", Cow::Borrowed(""));
    // TODO: implement https.proxyHost
    properties.insert("https.proxyHost", Cow::Borrowed(""));
    // TODO: implement https.proxyPort
    properties.insert("https.proxyPort", Cow::Borrowed(""));

    let class_path = vm.class_path().to_string();
    properties.insert("java.class.path", class_path.into());
    properties.insert(
        "java.class.version",
        format!("{major_class_version}.{minor_class_version}").into(),
    );
    properties.insert("java.compiler", "no JIT".into());
    // TODO: implement java.ext.dirs
    properties.insert("java.ext.dirs", Cow::Borrowed(""));
    properties.insert("java.home", java_home.into());

    let tmp_dir = env::temp_dir();
    properties.insert(
        "java.io.tmpdir",
        tmp_dir.to_string_lossy().into_owned().into(),
    );

    // TODO: implement java.library.path
    properties.insert("java.library.path", Cow::Borrowed(""));
    properties.insert(
        "java.specification.name",
        "Java Platform API Specification".into(),
    );
    properties.insert("java.specification.vendor", "Oracle Corporation".into());
    // TODO: implement java.specification.maintenance.version
    properties.insert("java.specification.maintenance.version", Cow::Borrowed(""));
    properties.insert("java.vendor", "ristretto".into());
    properties.insert(
        "java.vendor.url",
        "https://github.com/theseus-rs/ristretto".into(),
    );
    let vm_version = env!("CARGO_PKG_VERSION");
    properties.insert("java.vendor.version", vm_version.into());
    let java_version = vm.java_version();
    properties.insert("java.version", java_version.to_string().into());
    let architecture_bits = usize::BITS;
    let vm_name =
        format!("ristretto {vm_version} (Java {java_version}) {architecture_bits}-bit VM");
    properties.insert("java.vm.name", vm_name.into());
    properties.insert(
        "java.vm.specification.name",
        "Java Virtual Machine Specification".into(),
    );
    properties.insert(
        "java.vm.specification.vendor",
        "Oracle and Ristretto".into(),
    );
    properties.insert(
        "java.vm.specification.version",
        Cow::Owned(major_java_version.to_string()),
    );
    properties.insert("java.vm.vendor", "ristretto".into());
    properties.insert("java.vm.version", vm_version.into());

    #[cfg(not(target_os = "windows"))]
    properties.insert("line.separator", "\n".into());
    #[cfg(target_os = "windows")]
    properties.insert("line.separator", "\r\n".into());

    properties.insert("native.encoding", native_encoding());

    properties.insert("os.arch", ARCH.into());
    let os = match OS {
        "linux" => "Linux",
        "macos" => "Mac OS X",
        "windows" => "Windows",
        _ => OS,
    };
    properties.insert("os.name", os.into());

    let os_information = os_info::get();
    let os_version = os_information.version().to_string();
    properties.insert("os.version", os_version.into());

    #[cfg(not(target_os = "windows"))]
    properties.insert("path.separator", ":".into());
    #[cfg(target_os = "windows")]
    properties.insert("path.separator", ";".into());

    // TODO: implement socksNonProxyHosts
    properties.insert("socksNonProxyHosts", Cow::Borrowed(""));
    // TODO: implement socksProxyHost
    properties.insert("socksProxyHost", Cow::Borrowed(""));
    // TODO: implement socksProxyPort
    properties.insert("socksProxyPort", Cow::Borrowed(""));

    properties.insert("stderr.encoding", Cow::Borrowed("UTF-8"));
    properties.insert("stdin.encoding", console_input_encoding());
    properties.insert("stdout.encoding", Cow::Borrowed("UTF-8"));

    // TODO: implement sun.arch.abi
    properties.insert("sun.arch.abi", Cow::Borrowed(""));
    properties.insert(
        "sun.arch.data.model",
        Cow::Owned(architecture_bits.to_string()),
    );
    #[cfg(target_endian = "little")]
    properties.insert("sun.cpu.endian", "little".into());
    #[cfg(target_endian = "big")]
    properties.insert("sun.cpu.endian", "big".into());
    // TODO: implement sun.cpu.isalist
    properties.insert("sun.cpu.isalist", Cow::Borrowed(""));
    #[cfg(target_endian = "little")]
    properties.insert("sun.io.unicode.encoding", "UnicodeLittle".into());
    #[cfg(target_endian = "big")]
    properties.insert("sun.io.unicode.encoding", "UnicodeBig".into());
    properties.insert("sun.jnu.encoding", native_encoding());
    // TODO: implement sun.os.patch.level
    properties.insert("sun.os.patch.level", Cow::Borrowed(""));
    properties.insert("sun.stderr.encoding", Cow::Borrowed("UTF-8"));
    properties.insert("sun.stdout.encoding", Cow::Borrowed("UTF-8"));

    properties.insert("user.country", Cow::Owned(country_owned));
    let current_dir = env::current_dir().map_err(|error| InternalError(error.to_string()))?;
    properties.insert(
        "user.dir",
        current_dir.to_string_lossy().into_owned().into(),
    );
    let home_dir = dirs::home_dir().unwrap_or_default();
    properties.insert("user.home", home_dir.to_string_lossy().into_owned().into());
    properties.insert("user.language", Cow::Owned(language_owned));
    let username = whoami::username().map_err(|error| InternalError(error.to_string()))?;
    properties.insert("user.name", username.into());
    // TODO: implement user.script
    properties.insert("user.script", Cow::Borrowed(""));
    // TODO: implement user.variant
    properties.insert("user.variant", Cow::Borrowed(""));
    Ok(properties)
}

/// Returns the Java charset name for the native (ANSI) code page.
#[cfg_attr(target_os = "windows", expect(unsafe_code))]
fn native_encoding() -> Cow<'static, str> {
    #[cfg(target_os = "windows")]
    {
        unsafe extern "system" {
            fn GetACP() -> u32;
        }
        let cp = unsafe { GetACP() };
        code_page_to_charset(cp)
    }
    #[cfg(not(target_os = "windows"))]
    {
        Cow::Borrowed("UTF-8")
    }
}

/// Returns the Java charset name for the console output code page.
#[cfg_attr(target_os = "windows", expect(unsafe_code))]
fn console_output_encoding() -> Cow<'static, str> {
    #[cfg(target_os = "windows")]
    {
        unsafe extern "system" {
            fn GetConsoleOutputCP() -> u32;
        }
        let cp = unsafe { GetConsoleOutputCP() };
        code_page_to_charset(cp)
    }
    #[cfg(not(target_os = "windows"))]
    {
        Cow::Borrowed("UTF-8")
    }
}

/// Returns the Java charset name for the console input code page.
#[cfg_attr(target_os = "windows", expect(unsafe_code))]
fn console_input_encoding() -> Cow<'static, str> {
    #[cfg(target_os = "windows")]
    {
        unsafe extern "system" {
            fn GetConsoleCP() -> u32;
        }
        let cp = unsafe { GetConsoleCP() };
        code_page_to_charset(cp)
    }
    #[cfg(not(target_os = "windows"))]
    {
        Cow::Borrowed("UTF-8")
    }
}

/// Converts a Windows code page number to a Java charset name.
#[cfg(target_os = "windows")]
fn code_page_to_charset(code_page: u32) -> Cow<'static, str> {
    match code_page {
        65001 => Cow::Borrowed("UTF-8"),
        _ => Cow::Owned(format!("Cp{code_page}")),
    }
}
