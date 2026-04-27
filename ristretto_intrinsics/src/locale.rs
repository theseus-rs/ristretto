//! Host locale detection.
//!
//! Detects the operating system's default locale and returns it as a `(language, Option<country>)`
//! pair formatted to match `java.util.Locale` conventions, mirroring `OpenJDK`'s
//! `java_props_md.c` (Linux/Unix), `java_props_macosx.c` (macOS), and the Windows
//! `java_props_md.c`. The fallback when detection fails is `("en", None)`.

use std::sync::OnceLock;

static CACHED: OnceLock<(String, Option<String>)> = OnceLock::new();

/// Detects the host default locale and returns it as a `(language, Option<country>)` pair,
/// mirroring the behavior of `OpenJDK`'s `java_props_md.c` / `java_props_macosx.c`.
///
/// The lookup is delegated to a platform-specific `platform::default_locale()` implementation:
///
/// - **macOS**: `CFLocaleCopyPreferredLanguages` for the primary language. When that value lacks a
///   region (e.g., `"en"`) or only contains a script subtag (e.g., `"en-Latn"`), the region is
///   appended from `CFLocaleGetIdentifier(CFLocaleCopyCurrent())`. Mirrors `getMacOSXLocale` in
///   `java_props_macosx.c`.
/// - **Windows**: `GetUserDefaultLCID()` followed by `GetLocaleInfoW` queries for
///   `LOCALE_SISO639LANGNAME` and `LOCALE_SISO3166CTRYNAME`, with the same Norwegian remapping
///   (`nb` -> `no_NO`, `nn` -> `no_NO`) as `SetupI18nProps` in `java_props_md.c`.
/// - **Linux/Unix**: prefer env vars (`LC_ALL` -> `LC_MESSAGES` -> `LANG`) which matches what
///   `setlocale()` would return, then fall back to `sys_locale::get_locale()`. The encoding
///   (`.UTF-8`) and variant (`@euro`) suffixes are stripped, and the C/POSIX locale is mapped to
///   `"en"` per `OpenJDK`'s `mapLookup(language_names, ...)` table.
///
/// The returned language is lowercased and the country (if present) is uppercased to match
/// `java.util.Locale` conventions. When detection fails, the fallback is `("en", None)`.
///
/// The result is memoized in a `OnceLock`: `OpenJDK` reads the host locale once at VM init for
/// `user.language` / `user.country`, and the platform queries (CoreFoundation on macOS,
/// `GetLocaleInfoW` on Windows) are not cheap enough to repeat on every intrinsic call from
/// `HostLocaleProviderAdapterImpl.getDefaultLocale`.
pub(crate) fn detect_default_locale() -> (String, Option<String>) {
    CACHED
        .get_or_init(|| {
            let (mut language, mut country) = platform::default_locale();
            language.make_ascii_lowercase();
            if let Some(c) = country.as_mut() {
                c.make_ascii_uppercase();
            }
            (language, country)
        })
        .clone()
}

#[cfg(target_os = "macos")]
mod platform {
    //! macOS host locale lookup using Core Foundation, matching `OpenJDK`'s `getMacOSXLocale` in
    //! `java_props_macosx.c`.

    #![expect(unsafe_code)]

    use core_foundation_sys::array::{CFArrayGetCount, CFArrayGetValueAtIndex};
    use core_foundation_sys::base::{Boolean, CFIndex, CFRange, CFRelease, CFTypeRef};
    use core_foundation_sys::locale::{
        CFLocaleCopyCurrent, CFLocaleCopyPreferredLanguages, CFLocaleGetIdentifier,
    };
    use core_foundation_sys::string::{
        CFStringGetBytes, CFStringGetLength, CFStringRef, kCFStringEncodingUTF8,
    };

    /// Returns the macOS system locale as `(language, Option<region>)`, mirroring `OpenJDK`'s
    /// `getMacOSXLocale` in `java_props_macosx.c`.
    ///
    /// Calls `CFLocaleCopyPreferredLanguages()` for the primary language. When that value lacks a
    /// region (e.g., `"en"`) or only contains a script subtag (e.g., `"en-Latn"`), the region is
    /// taken from `CFLocaleGetIdentifier(CFLocaleCopyCurrent())` (e.g., `"en_US"` -> `"US"`).
    /// Falls back to `("en", None)` when the Core Foundation calls return nothing.
    pub(super) fn default_locale() -> (String, Option<String>) {
        let Some(primary) = preferred_primary_language() else {
            return ("en".to_string(), None);
        };

        let (language, embedded_region) = match primary.split_once('-') {
            // No hyphen at all: just a language tag.
            None => (primary, None),
            // A 4-character subtag is a script (e.g. "en-Latn"); ignore it and look up the region.
            Some((lang, subtag)) if subtag.len() == 4 => (lang.to_string(), None),
            // Otherwise the second segment is the region (e.g. "en-US" or "zh-Hant-TW").
            Some((lang, rest)) => {
                let region = rest.rsplit('-').next().unwrap_or(rest).to_string();
                (lang.to_string(), Some(region))
            }
        };

        if let Some(region) = embedded_region {
            return (language, Some(region));
        }

        let region = current_locale_identifier()
            .and_then(|ident| ident.rsplit('_').next().map(str::to_string))
            .filter(|s| !s.is_empty());
        (language, region)
    }

    /// Returns the first entry from `CFLocaleCopyPreferredLanguages()`.
    fn preferred_primary_language() -> Option<String> {
        let result = unsafe {
            let languages = CFLocaleCopyPreferredLanguages();
            if languages.is_null() {
                return None;
            }
            let count = CFArrayGetCount(languages);
            if count <= 0 {
                CFRelease(languages.cast::<core::ffi::c_void>());
                return None;
            }
            let primary: CFStringRef = CFArrayGetValueAtIndex(languages, 0).cast();
            let value = cfstring_to_string(primary);
            CFRelease(languages.cast::<core::ffi::c_void>());
            value
        };
        result.filter(|s| !s.is_empty())
    }

    /// Returns `CFLocaleGetIdentifier(CFLocaleCopyCurrent())` (e.g., `"en_US"`).
    fn current_locale_identifier() -> Option<String> {
        unsafe {
            let cflocale = CFLocaleCopyCurrent();
            if cflocale.is_null() {
                return None;
            }
            let ident: CFStringRef = CFLocaleGetIdentifier(cflocale);
            let value = cfstring_to_string(ident);
            CFRelease(cflocale.cast::<core::ffi::c_void>() as CFTypeRef);
            value
        }
    }

    /// Converts a `CFStringRef` into a Rust `String`. Returns `None` if the pointer is null or the
    /// conversion fails.
    unsafe fn cfstring_to_string(s: CFStringRef) -> Option<String> {
        if s.is_null() {
            return None;
        }
        let length = unsafe { CFStringGetLength(s) };
        if length <= 0 {
            return Some(String::new());
        }
        let range = CFRange {
            location: 0,
            length,
        };
        let mut needed: CFIndex = 0;
        let converted = unsafe {
            CFStringGetBytes(
                s,
                range,
                kCFStringEncodingUTF8,
                0,
                Boolean::from(false),
                std::ptr::null_mut(),
                0,
                &raw mut needed,
            )
        };
        if converted == 0 || needed <= 0 {
            return None;
        }
        let mut buffer = vec![0u8; usize::try_from(needed).ok()?];
        let written = unsafe {
            CFStringGetBytes(
                s,
                range,
                kCFStringEncodingUTF8,
                0,
                Boolean::from(false),
                buffer.as_mut_ptr(),
                needed,
                std::ptr::null_mut(),
            )
        };
        if written == 0 {
            return None;
        }
        String::from_utf8(buffer).ok()
    }
}

#[cfg(target_family = "windows")]
mod platform {
    //! Windows host locale lookup using `GetUserDefaultLCID` + `GetLocaleInfoW`, matching the
    //! `SetupI18nProps` flow in `OpenJDK`'s `java_props_md.c` (windows).

    #![expect(unsafe_code)]

    use windows_sys::Win32::Globalization::{
        GetLocaleInfoW, GetUserDefaultLCID, LOCALE_SISO639LANGNAME, LOCALE_SISO3166CTRYNAME,
    };

    /// Returns the Windows user default locale as `(language, Option<country>)`, mirroring
    /// `SetupI18nProps` in `java_props_md.c` (windows).
    ///
    /// Queries `GetUserDefaultLCID()` then `GetLocaleInfoW` with `LOCALE_SISO639LANGNAME` for the
    /// language and `LOCALE_SISO3166CTRYNAME` for the country. Applies `OpenJDK`'s Norwegian
    /// remappings (`nb` / `nn` -> `no_NO`, regardless of the LCID-reported country, matching
    /// `SetupI18nProps`). Falls back to `("en", None)` when both queries fail.
    pub(super) fn default_locale() -> (String, Option<String>) {
        let lcid = unsafe { GetUserDefaultLCID() };
        let language = locale_info(lcid, LOCALE_SISO639LANGNAME);
        let country = locale_info(lcid, LOCALE_SISO3166CTRYNAME);

        match (language, country) {
            (None, None) => ("en".to_string(), None),
            (Some(lang), _) if lang == "nb" || lang == "nn" => {
                ("no".to_string(), Some("NO".to_string()))
            }
            (Some(lang), country) => (lang, country),
            (None, country) => ("en".to_string(), country),
        }
    }

    /// Calls `GetLocaleInfoW(lcid, lc_type, ..)` and returns the resulting string with the
    /// trailing NUL stripped. Returns `None` if the call fails or yields an empty value.
    fn locale_info(lcid: u32, lc_type: u32) -> Option<String> {
        // OpenJDK uses an 8-character buffer (PROPSIZE = 9 including NUL); 16 is more than enough.
        let mut buffer = [0u16; 16];
        let written = unsafe {
            GetLocaleInfoW(
                lcid,
                lc_type,
                buffer.as_mut_ptr(),
                i32::try_from(buffer.len()).ok()?,
            )
        };
        if written <= 0 {
            return None;
        }
        let len = usize::try_from(written).ok()?.saturating_sub(1);
        let value = String::from_utf16(&buffer[..len]).ok()?;
        if value.is_empty() { None } else { Some(value) }
    }
}

#[cfg(not(any(target_os = "macos", target_family = "windows")))]
mod platform {
    //! Linux / other Unix host locale lookup, mirroring the `setlocale` chain that `OpenJDK`'s
    //! `java_props_md.c` reads on Linux.

    use std::env;

    /// Returns the Unix system locale as `(language, Option<country>)` by reading the standard
    /// POSIX environment variables in `OpenJDK`'s priority order (`LC_ALL` -> `LC_MESSAGES` ->
    /// `LANG`), then falling back to `sys_locale::get_locale()`. The encoding (`.UTF-8`) and
    /// variant (`@euro`) suffixes are stripped. The C/POSIX locale is mapped to `("en", None)`.
    pub(super) fn default_locale() -> (String, Option<String>) {
        let raw = env_locale().or_else(sys_locale::get_locale);
        match raw {
            None => ("en".to_string(), None),
            Some(raw) => parse_posix_locale(&raw),
        }
    }

    /// Reads the locale from POSIX environment variables in `OpenJDK`'s priority order.
    fn env_locale() -> Option<String> {
        ["LC_ALL", "LC_MESSAGES", "LANG"]
            .into_iter()
            .find_map(|name| env::var(name).ok().filter(|s| !s.is_empty()))
    }

    /// Parses `language[_COUNTRY][.encoding][@variant]` (or BCP 47 `language-COUNTRY` from
    /// `sys_locale`) into `(language, Option<country>)`. Maps the POSIX `C` / `POSIX` locale and
    /// any empty value to `("en", None)`.
    pub(super) fn parse_posix_locale(raw: &str) -> (String, Option<String>) {
        let head = raw
            .split(['@', '.'])
            .next()
            .unwrap_or("")
            .trim_matches(char::is_whitespace);

        // Both '_' (POSIX) and '-' (BCP 47) act as separators between language and country.
        let (language, country) = match head.split_once(['_', '-']) {
            Some((lang, rest)) => (lang, Some(rest)),
            None => (head, None),
        };

        if language.is_empty()
            || language.eq_ignore_ascii_case("C")
            || language.eq_ignore_ascii_case("POSIX")
        {
            return ("en".to_string(), None);
        }

        let country = country.filter(|s| !s.is_empty()).map(str::to_string);
        (language.to_string(), country)
    }
}

#[cfg(test)]
mod tests {
    use super::detect_default_locale;

    #[test]
    fn detect_default_locale_returns_value() {
        let (language, country) = detect_default_locale();
        eprintln!("detected locale: {language}_{country:?}");
        assert!(!language.is_empty(), "language should be non-empty");
    }

    #[test]
    fn detect_default_locale_lowercases_language_uppercases_country() {
        let (language, country) = detect_default_locale();
        let mut expected_language = language.clone();
        expected_language.make_ascii_lowercase();
        assert_eq!(language, expected_language);
        if let Some(c) = country {
            let mut expected_country = c.clone();
            expected_country.make_ascii_uppercase();
            assert_eq!(c, expected_country);
        }
    }

    #[cfg(not(any(target_os = "macos", target_family = "windows")))]
    mod unix {
        use super::super::platform::parse_posix_locale;

        #[test]
        fn parses_full_posix_locale() {
            assert_eq!(
                parse_posix_locale("en_US.UTF-8"),
                ("en".to_string(), Some("US".to_string()))
            );
        }

        #[test]
        fn parses_bcp47_locale() {
            assert_eq!(
                parse_posix_locale("en-US"),
                ("en".to_string(), Some("US".to_string()))
            );
        }

        #[test]
        fn strips_modifier_and_encoding() {
            assert_eq!(
                parse_posix_locale("de_DE.UTF-8@euro"),
                ("de".to_string(), Some("DE".to_string()))
            );
        }

        #[test]
        fn maps_c_locale_to_en() {
            assert_eq!(parse_posix_locale("C"), ("en".to_string(), None));
            assert_eq!(parse_posix_locale("C.UTF-8"), ("en".to_string(), None));
            assert_eq!(parse_posix_locale("POSIX"), ("en".to_string(), None));
        }

        #[test]
        fn maps_empty_locale_to_en() {
            assert_eq!(parse_posix_locale(""), ("en".to_string(), None));
        }

        #[test]
        fn handles_language_without_country() {
            assert_eq!(parse_posix_locale("ja"), ("ja".to_string(), None));
        }
    }
}
