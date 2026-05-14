#[cfg(all(not(target_os = "linux"), not(target_os = "windows")))]
use crate::locale::detect_default_locale;
#[cfg(target_os = "windows")]
use ristretto_classfile::JAVA_11;
#[cfg(target_os = "macos")]
use ristretto_classfile::JAVA_17;
#[cfg(not(target_os = "linux"))]
use ristretto_classfile::VersionSpecification::Any;
#[cfg(any(target_os = "macos", target_os = "windows"))]
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
#[cfg(not(target_os = "linux"))]
use ristretto_classloader::{Reference, Value};
#[cfg(not(target_os = "linux"))]
use ristretto_macros::async_method;
#[cfg(not(target_os = "linux"))]
use ristretto_macros::intrinsic_method;
#[cfg(not(target_os = "linux"))]
use ristretto_types::JavaObject;
#[cfg(not(target_os = "linux"))]
use ristretto_types::Thread;
#[cfg(not(target_os = "linux"))]
use ristretto_types::VM;
#[cfg(not(target_os = "linux"))]
use ristretto_types::{Parameters, Result};
#[cfg(not(target_os = "linux"))]
use std::sync::Arc;

/// Java number format selector passed to `getNumberPatternNative`.
#[cfg(not(target_os = "linux"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum NumberFormatKind {
    /// Decimal number pattern.
    Decimal,
    /// Currency pattern.
    Currency,
    /// Percent pattern.
    Percent,
    /// Integer-only decimal pattern.
    Integer,
}

#[cfg(not(target_os = "linux"))]
impl NumberFormatKind {
    /// Converts the Java `NumberFormat` selector value into a typed style.
    #[must_use]
    const fn from_i32(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::Decimal),
            1 => Some(Self::Currency),
            2 => Some(Self::Percent),
            3 => Some(Self::Integer),
            _ => None,
        }
    }

    /// Returns the Java `NumberFormat` selector value.
    #[must_use]
    const fn as_i32(self) -> i32 {
        match self {
            Self::Decimal => 0,
            Self::Currency => 1,
            Self::Percent => 2,
            Self::Integer => 3,
        }
    }

    /// Returns the Windows number-format fix table index.
    #[cfg(target_os = "windows")]
    #[must_use]
    const fn windows_fix_index(self) -> usize {
        match self {
            Self::Decimal | Self::Integer => 0,
            Self::Currency => 1,
            Self::Percent => 2,
        }
    }
}

/// macOS calendar integer selector passed to `getCalendarInt`.
#[cfg(target_os = "macos")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum MacCalendarDataKind {
    /// First day of the week for the current calendar.
    FirstDayOfWeek,
    /// Minimum days required in the first week of a year.
    MinimalDaysInFirstWeek,
}

#[cfg(target_os = "macos")]
impl MacCalendarDataKind {
    /// Converts the Java calendar data selector value into a typed macOS selector.
    #[must_use]
    const fn from_i32(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::FirstDayOfWeek),
            1 => Some(Self::MinimalDaysInFirstWeek),
            _ => None,
        }
    }

    /// Returns the Java calendar data selector value.
    #[must_use]
    const fn as_i32(self) -> i32 {
        match self {
            Self::FirstDayOfWeek => 0,
            Self::MinimalDaysInFirstWeek => 1,
        }
    }
}

/// Windows calendar integer selector passed to `getCalendarDataValue`.
#[cfg(target_os = "windows")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum WindowsCalendarDataKind {
    /// First day of the week for the requested locale.
    FirstDayOfWeek,
    /// Windows first-week-of-year rule for the requested locale.
    FirstWeekOfYear,
}

#[cfg(target_os = "windows")]
impl WindowsCalendarDataKind {
    /// Converts the Java calendar data selector value into a typed Windows selector.
    #[must_use]
    const fn from_i32(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::FirstDayOfWeek),
            1 => Some(Self::FirstWeekOfYear),
            _ => None,
        }
    }

    /// Returns the Java calendar data selector value.
    #[must_use]
    const fn as_i32(self) -> i32 {
        match self {
            Self::FirstDayOfWeek => 0,
            Self::FirstWeekOfYear => 1,
        }
    }
}

/// Java `Calendar` field selector used for localized display-name arrays.
#[cfg(not(target_os = "linux"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CalendarDisplayField {
    /// Era display names.
    Era,
    /// Month display names.
    #[cfg(target_os = "windows")]
    Month,
    /// Day-of-week display names.
    DayOfWeek,
    /// AM/PM display names.
    AmPm,
}

#[cfg(not(target_os = "linux"))]
impl CalendarDisplayField {
    /// Converts the Java `Calendar` field value into a display-name field.
    #[must_use]
    const fn from_i32(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::Era),
            #[cfg(target_os = "windows")]
            2 => Some(Self::Month),
            7 => Some(Self::DayOfWeek),
            9 => Some(Self::AmPm),
            _ => None,
        }
    }

    /// Returns the Java `Calendar` field value.
    #[must_use]
    const fn as_i32(self) -> i32 {
        match self {
            Self::Era => 0,
            #[cfg(target_os = "windows")]
            Self::Month => 2,
            Self::DayOfWeek => 7,
            Self::AmPm => 9,
        }
    }
}

/// Windows calendar display style bit flags from `OpenJDK` host locale support.
#[cfg(target_os = "windows")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum WindowsCalendarStyleFlag {
    /// Request abbreviated display names.
    Short,
    /// Request stand-alone names instead of format/genitive names.
    StandAlone,
}

#[cfg(target_os = "windows")]
impl WindowsCalendarStyleFlag {
    /// Returns the Java style bit mask.
    #[must_use]
    const fn mask(self) -> i32 {
        match self {
            Self::Short => 0x0000_0001,
            Self::StandAlone => 0x0000_8000,
        }
    }

    /// Returns true when this style flag is present.
    #[must_use]
    const fn is_set(self, style: i32) -> bool {
        style & self.mask() != 0
    }
}

/// macOS display-name selector passed to `getDisplayString`.
#[cfg(target_os = "macos")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum MacDisplayNameKind {
    /// Localized language display name.
    Language,
    /// Localized script display name.
    Script,
    /// Localized region display name.
    Region,
    /// Localized variant display name.
    Variant,
    /// ISO currency code.
    CurrencyCode,
    /// Localized currency symbol.
    CurrencySymbol,
}

#[cfg(target_os = "macos")]
impl MacDisplayNameKind {
    /// Converts the Java display-name selector value into a typed macOS selector.
    #[must_use]
    const fn from_i32(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::Language),
            1 => Some(Self::Script),
            2 => Some(Self::Region),
            3 => Some(Self::Variant),
            4 => Some(Self::CurrencyCode),
            5 => Some(Self::CurrencySymbol),
            _ => None,
        }
    }

    /// Returns the Java display-name selector value.
    #[must_use]
    const fn as_i32(self) -> i32 {
        match self {
            Self::Language => 0,
            Self::Script => 1,
            Self::Region => 2,
            Self::Variant => 3,
            Self::CurrencyCode => 4,
            Self::CurrencySymbol => 5,
        }
    }
}

/// Windows display-name selector passed to `getDisplayString`.
#[cfg(target_os = "windows")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum WindowsDisplayNameKind {
    /// Localized native currency name for the target locale.
    CurrencyName,
    /// Currency symbol for the target locale.
    CurrencySymbol,
    /// Localized language display name.
    Language,
    /// Localized region display name.
    Region,
}

#[cfg(target_os = "windows")]
impl WindowsDisplayNameKind {
    /// Converts the Java display-name selector value into a typed Windows selector.
    #[must_use]
    const fn from_i32(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::CurrencyName),
            1 => Some(Self::CurrencySymbol),
            2 => Some(Self::Language),
            4 => Some(Self::Region),
            _ => None,
        }
    }

    /// Returns the Java display-name selector value.
    #[must_use]
    const fn as_i32(self) -> i32 {
        match self {
            Self::CurrencyName => 0,
            Self::CurrencySymbol => 1,
            Self::Language => 2,
            Self::Region => 4,
        }
    }
}

/// macOS localized time-zone display-name selector.
#[cfg(target_os = "macos")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum MacTimeZoneNameKind {
    /// Short standard-time name.
    ShortStandard,
    /// Short daylight-saving-time name.
    ShortDaylightSaving,
    /// Long standard-time name.
    LongStandard,
    /// Long daylight-saving-time name.
    LongDaylightSaving,
}

#[cfg(target_os = "macos")]
impl MacTimeZoneNameKind {
    /// Converts the Java time-zone display-name selector into a typed macOS selector.
    #[must_use]
    const fn from_i32(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::ShortStandard),
            1 => Some(Self::ShortDaylightSaving),
            2 => Some(Self::LongStandard),
            3 => Some(Self::LongDaylightSaving),
            _ => None,
        }
    }

    /// Returns the Java time-zone display-name selector value.
    #[must_use]
    const fn as_i32(self) -> i32 {
        match self {
            Self::ShortStandard => 0,
            Self::ShortDaylightSaving => 1,
            Self::LongStandard => 2,
            Self::LongDaylightSaving => 3,
        }
    }
}

#[cfg(not(target_os = "linux"))]
fn default_date_pattern(style: i32) -> Option<&'static str> {
    match style {
        0 => Some("EEEE, MMMM d, yyyy"),
        1 => Some("MMMM d, yyyy"),
        2 => Some("MMM d, yyyy"),
        3 => Some("M/d/yy"),
        _ => None,
    }
}

#[cfg(not(target_os = "linux"))]
fn default_time_pattern(style: i32) -> Option<&'static str> {
    match style {
        0 => Some("h:mm:ss a zzzz"),
        1 => Some("h:mm:ss a z"),
        2 => Some("h:mm:ss a"),
        3 => Some("h:mm a"),
        _ => None,
    }
}

#[cfg(not(target_os = "linux"))]
fn default_date_time_pattern(date_style: i32, time_style: i32) -> String {
    match (
        default_date_pattern(date_style),
        default_time_pattern(time_style),
    ) {
        (Some(date), Some(time)) => format!("{date} {time}"),
        (Some(date), None) => date.to_string(),
        (None, Some(time)) => time.to_string(),
        (None, None) => String::new(),
    }
}

#[cfg(not(target_os = "linux"))]
fn default_number_pattern(style: i32) -> &'static str {
    match NumberFormatKind::from_i32(style) {
        Some(NumberFormatKind::Currency) => "\u{00a4}#,##0.00",
        Some(NumberFormatKind::Percent) => "#,##0%",
        Some(NumberFormatKind::Integer) => "#,##0",
        _ => "#,##0.###",
    }
}

#[cfg(not(target_os = "linux"))]
#[derive(Debug)]
struct StringArrayData {
    len: usize,
    replacements: Vec<(usize, String)>,
}

#[cfg(not(target_os = "linux"))]
fn java_string(value: &Value) -> Option<String> {
    value.as_string().ok()
}

#[cfg(not(target_os = "linux"))]
async fn java_string_value<T: Thread + 'static>(thread: &Arc<T>, value: &str) -> Result<Value> {
    value.to_object(thread).await
}

#[cfg(not(target_os = "linux"))]
async fn new_string_array<T: Thread + 'static>(thread: &Arc<T>, len: usize) -> Result<Value> {
    let class = thread.class("[Ljava/lang/String;").await?;
    let vm = thread.vm()?;
    let elements = vec![Value::Object(None); len];
    let reference = Reference::try_from((class, elements))?;
    Ok(Value::new_object(vm.garbage_collector(), reference))
}

#[cfg(not(target_os = "linux"))]
async fn new_string_array_from_data<T: Thread + 'static>(
    thread: &Arc<T>,
    data: StringArrayData,
) -> Result<Value> {
    let array = new_string_array(thread, data.len).await?;
    set_string_array_replacements(thread, array, data.replacements).await
}

#[cfg(not(target_os = "linux"))]
async fn set_string_array_replacements<T: Thread + 'static>(
    thread: &Arc<T>,
    array: Value,
    replacements: Vec<(usize, String)>,
) -> Result<Value> {
    let mut values = Vec::with_capacity(replacements.len());
    for (index, string) in replacements {
        values.push((index, java_string_value(thread, &string).await?));
    }

    {
        let (_class, mut elements) = array.as_class_vec_mut()?;
        for (index, value) in values {
            if let Some(element) = elements.get_mut(index) {
                *element = value;
            }
        }
    }

    Ok(array)
}

#[cfg(not(target_os = "linux"))]
async fn update_or_create_string_array<T: Thread + 'static>(
    thread: &Arc<T>,
    array: Value,
    data: StringArrayData,
) -> Result<Value> {
    let array = if matches!(array, Value::Object(None)) {
        new_string_array(thread, data.len).await?
    } else {
        array
    };
    set_string_array_replacements(thread, array, data.replacements).await
}

#[cfg(target_os = "macos")]
mod macos {
    #![expect(unsafe_code)]

    use super::{
        CalendarDisplayField, MacCalendarDataKind, MacDisplayNameKind, MacTimeZoneNameKind,
        NumberFormatKind, StringArrayData,
    };
    use core_foundation_sys::array::{CFArrayGetCount, CFArrayGetValueAtIndex, CFArrayRef};
    use core_foundation_sys::base::{
        Boolean, CFIndex, CFRange, CFRelease, CFTypeRef, kCFAllocatorDefault,
    };
    use core_foundation_sys::calendar::{
        CFCalendarCopyCurrent, CFCalendarGetFirstWeekday, CFCalendarGetMinimumDaysInFirstWeek,
    };
    use core_foundation_sys::date_formatter::{
        CFDateFormatterCopyProperty, CFDateFormatterCreate, CFDateFormatterGetFormat,
        CFDateFormatterKey, CFDateFormatterStyle, kCFDateFormatterAMSymbol,
        kCFDateFormatterEraSymbols, kCFDateFormatterFullStyle, kCFDateFormatterLongStyle,
        kCFDateFormatterMediumStyle, kCFDateFormatterMonthSymbols, kCFDateFormatterNoStyle,
        kCFDateFormatterPMSymbol, kCFDateFormatterShortMonthSymbols, kCFDateFormatterShortStyle,
        kCFDateFormatterShortWeekdaySymbols, kCFDateFormatterWeekdaySymbols,
    };
    use core_foundation_sys::locale::{
        CFLocaleCopyCurrent, CFLocaleCopyDisplayNameForPropertyValue, CFLocaleCreate,
        CFLocaleGetValue, kCFJapaneseCalendar, kCFLocaleCalendarIdentifier, kCFLocaleCountryCode,
        kCFLocaleCurrencyCode, kCFLocaleCurrencySymbol, kCFLocaleLanguageCode, kCFLocaleScriptCode,
        kCFLocaleVariantCode,
    };
    use core_foundation_sys::number::kCFNumberIntType;
    use core_foundation_sys::number_formatter::{
        CFNumberFormatterCopyProperty, CFNumberFormatterCreate,
        CFNumberFormatterCreateStringWithValue, CFNumberFormatterGetFormat, CFNumberFormatterKey,
        CFNumberFormatterStyle, kCFNumberFormatterCurrencyDecimalSeparator,
        kCFNumberFormatterCurrencyStyle, kCFNumberFormatterCurrencySymbol,
        kCFNumberFormatterDecimalSeparator, kCFNumberFormatterDecimalStyle,
        kCFNumberFormatterExponentSymbol, kCFNumberFormatterGroupingSeparator,
        kCFNumberFormatterInfinitySymbol, kCFNumberFormatterInternationalCurrencySymbol,
        kCFNumberFormatterMinusSign, kCFNumberFormatterNaNSymbol, kCFNumberFormatterNoStyle,
        kCFNumberFormatterPerMillSymbol, kCFNumberFormatterPercentStyle,
        kCFNumberFormatterPercentSymbol,
    };
    use core_foundation_sys::string::{
        CFStringCreateWithBytes, CFStringGetBytes, CFStringGetCharacterAtIndex, CFStringGetLength,
        CFStringRef, kCFStringEncodingUTF8,
    };
    use core_foundation_sys::timezone::{
        CFTimeZoneCopyLocalizedName, CFTimeZoneCreateWithName, kCFTimeZoneNameStyleDaylightSaving,
        kCFTimeZoneNameStyleShortDaylightSaving, kCFTimeZoneNameStyleShortStandard,
        kCFTimeZoneNameStyleStandard,
    };
    use std::ffi::c_void;
    use std::ptr;

    const JAPANESE_MEIJI_INDEX: usize = 232;

    fn convert_date_formatter_style(style: i32) -> CFDateFormatterStyle {
        match style {
            0 => kCFDateFormatterFullStyle,
            1 => kCFDateFormatterLongStyle,
            2 => kCFDateFormatterMediumStyle,
            3 => kCFDateFormatterShortStyle,
            _ => kCFDateFormatterNoStyle,
        }
    }

    fn convert_number_formatter_style(style: i32) -> CFNumberFormatterStyle {
        match NumberFormatKind::from_i32(style) {
            Some(NumberFormatKind::Currency) => kCFNumberFormatterCurrencyStyle,
            Some(NumberFormatKind::Integer | NumberFormatKind::Decimal) => {
                kCFNumberFormatterDecimalStyle
            }
            Some(NumberFormatKind::Percent) => kCFNumberFormatterPercentStyle,
            None => kCFNumberFormatterNoStyle,
        }
    }

    pub(super) fn date_time_pattern(date_style: i32, time_style: i32) -> Option<String> {
        unsafe {
            let locale = CFLocaleCopyCurrent();
            if locale.is_null() {
                return None;
            }
            let formatter = CFDateFormatterCreate(
                kCFAllocatorDefault,
                locale,
                convert_date_formatter_style(date_style),
                convert_date_formatter_style(time_style),
            );
            let result = if formatter.is_null() {
                None
            } else {
                let format = CFDateFormatterGetFormat(formatter);
                let result = cfstring_to_string(format);
                release(formatter.cast::<c_void>());
                result
            };
            release(locale.cast::<c_void>());
            result
        }
    }

    pub(super) fn calendar_id() -> Option<String> {
        unsafe {
            let locale = CFLocaleCopyCurrent();
            if locale.is_null() {
                return None;
            }
            let calendar = CFLocaleGetValue(locale, kCFLocaleCalendarIdentifier).cast();
            let result = cfstring_to_string(calendar);
            release(locale.cast::<c_void>());
            result
        }
    }

    pub(super) fn calendar_int(r#type: i32) -> i32 {
        unsafe {
            let calendar = CFCalendarCopyCurrent();
            if calendar.is_null() {
                return 0;
            }
            let calendar_identifier = calendar.cast_const().cast();
            let value = match MacCalendarDataKind::from_i32(r#type) {
                Some(MacCalendarDataKind::FirstDayOfWeek) => {
                    CFCalendarGetFirstWeekday(calendar_identifier)
                }
                Some(MacCalendarDataKind::MinimalDaysInFirstWeek) => {
                    CFCalendarGetMinimumDaysInFirstWeek(calendar_identifier)
                }
                None => 0,
            };
            release(calendar.cast::<c_void>());
            i32::try_from(value).unwrap_or(0)
        }
    }

    pub(super) fn number_pattern(style: i32) -> Option<String> {
        unsafe {
            let locale = CFLocaleCopyCurrent();
            if locale.is_null() {
                return None;
            }
            let formatter = CFNumberFormatterCreate(
                kCFAllocatorDefault,
                locale,
                convert_number_formatter_style(style),
            );
            let result = if formatter.is_null() {
                None
            } else {
                let format = CFNumberFormatterGetFormat(formatter);
                let result = cfstring_to_string(format);
                release(formatter.cast::<c_void>());
                result
            };
            release(locale.cast::<c_void>());
            result
        }
    }

    pub(super) fn number_symbol_string(r#type: CFNumberFormatterKey) -> Option<String> {
        unsafe {
            let locale = CFLocaleCopyCurrent();
            if locale.is_null() {
                return None;
            }
            let formatter =
                CFNumberFormatterCreate(kCFAllocatorDefault, locale, kCFNumberFormatterNoStyle);
            let result = if formatter.is_null() {
                None
            } else {
                let string = CFNumberFormatterCopyProperty(formatter, r#type).cast();
                let result = cfstring_to_string(string);
                release_opt(string.cast::<c_void>());
                release(formatter.cast::<c_void>());
                result
            };
            release(locale.cast::<c_void>());
            result
        }
    }

    pub(super) fn number_symbol_char(r#type: CFNumberFormatterKey) -> Option<i32> {
        unsafe {
            let locale = CFLocaleCopyCurrent();
            if locale.is_null() {
                return None;
            }
            let formatter =
                CFNumberFormatterCreate(kCFAllocatorDefault, locale, kCFNumberFormatterNoStyle);
            let result = if formatter.is_null() {
                None
            } else {
                let string = CFNumberFormatterCopyProperty(formatter, r#type).cast();
                let result = cfstring_first_char(string);
                release_opt(string.cast::<c_void>());
                release(formatter.cast::<c_void>());
                result
            };
            release(locale.cast::<c_void>());
            result
        }
    }

    pub(super) fn zero_digit() -> Option<i32> {
        unsafe {
            let locale = CFLocaleCopyCurrent();
            if locale.is_null() {
                return None;
            }
            let formatter =
                CFNumberFormatterCreate(kCFAllocatorDefault, locale, kCFNumberFormatterNoStyle);
            let result = if formatter.is_null() {
                None
            } else {
                let zero = 0_i32;
                let string = CFNumberFormatterCreateStringWithValue(
                    kCFAllocatorDefault,
                    formatter,
                    kCFNumberIntType,
                    (&raw const zero).cast::<c_void>(),
                );
                let result = cfstring_first_char(string);
                release_opt(string.cast::<c_void>());
                release(formatter.cast::<c_void>());
                result
            };
            release(locale.cast::<c_void>());
            result
        }
    }

    pub(super) fn currency_symbol() -> Option<String> {
        number_symbol_string(unsafe { kCFNumberFormatterCurrencySymbol })
    }

    pub(super) fn decimal_separator() -> Option<i32> {
        number_symbol_char(unsafe { kCFNumberFormatterDecimalSeparator })
    }

    pub(super) fn grouping_separator() -> Option<i32> {
        number_symbol_char(unsafe { kCFNumberFormatterGroupingSeparator })
    }

    pub(super) fn infinity() -> Option<String> {
        number_symbol_string(unsafe { kCFNumberFormatterInfinitySymbol })
    }

    pub(super) fn international_currency_symbol() -> Option<String> {
        number_symbol_string(unsafe { kCFNumberFormatterInternationalCurrencySymbol })
    }

    pub(super) fn minus_sign() -> Option<i32> {
        number_symbol_char(unsafe { kCFNumberFormatterMinusSign })
    }

    pub(super) fn monetary_decimal_separator() -> Option<i32> {
        number_symbol_char(unsafe { kCFNumberFormatterCurrencyDecimalSeparator })
    }

    pub(super) fn nan() -> Option<String> {
        number_symbol_string(unsafe { kCFNumberFormatterNaNSymbol })
    }

    pub(super) fn percent() -> Option<i32> {
        number_symbol_char(unsafe { kCFNumberFormatterPercentSymbol })
    }

    pub(super) fn per_mill() -> Option<i32> {
        number_symbol_char(unsafe { kCFNumberFormatterPerMillSymbol })
    }

    pub(super) fn exponent_separator() -> Option<String> {
        number_symbol_string(unsafe { kCFNumberFormatterExponentSymbol })
    }

    pub(super) fn am_pm_strings(style: i32) -> Option<StringArrayData> {
        let mut replacements = Vec::with_capacity(2);
        unsafe {
            let formatter = current_date_formatter(style, style)?;
            let am = CFDateFormatterCopyProperty(formatter, kCFDateFormatterAMSymbol).cast();
            if let Some(value) = cfstring_to_string(am) {
                replacements.push((0, value));
            }
            release_opt(am.cast::<c_void>());
            let pm = CFDateFormatterCopyProperty(formatter, kCFDateFormatterPMSymbol).cast();
            if let Some(value) = cfstring_to_string(pm) {
                replacements.push((1, value));
            }
            release_opt(pm.cast::<c_void>());
            release(formatter.cast::<c_void>());
        }
        Some(StringArrayData {
            len: 2,
            replacements,
        })
    }

    pub(super) fn eras(style: i32) -> Option<StringArrayData> {
        unsafe {
            let formatter = current_date_formatter(style, style)?;
            let eras: CFArrayRef =
                CFDateFormatterCopyProperty(formatter, kCFDateFormatterEraSymbols).cast();
            if eras.is_null() {
                release(formatter.cast::<c_void>());
                return None;
            }
            let era_count = usize::try_from(CFArrayGetCount(eras)).ok()?;
            let japanese = current_calendar_is_japanese();
            let source_index = if japanese && era_count > JAPANESE_MEIJI_INDEX {
                JAPANESE_MEIJI_INDEX
            } else {
                0
            };
            let dest_index = usize::from(source_index == JAPANESE_MEIJI_INDEX);
            let replacements = copy_array_elements(
                eras,
                source_index,
                dest_index,
                era_count.saturating_sub(source_index),
            );
            let len = if source_index == JAPANESE_MEIJI_INDEX {
                replacements.len() + 1
            } else {
                era_count
            };
            release_opt(eras.cast::<c_void>());
            release(formatter.cast::<c_void>());
            Some(StringArrayData { len, replacements })
        }
    }

    pub(super) fn months() -> Option<StringArrayData> {
        date_array(unsafe { kCFDateFormatterMonthSymbols }, 0, 0)
    }

    pub(super) fn short_months() -> Option<StringArrayData> {
        date_array(unsafe { kCFDateFormatterShortMonthSymbols }, 0, 0)
    }

    pub(super) fn weekdays() -> Option<StringArrayData> {
        date_array(unsafe { kCFDateFormatterWeekdaySymbols }, 0, 1)
    }

    pub(super) fn short_weekdays() -> Option<StringArrayData> {
        date_array(unsafe { kCFDateFormatterShortWeekdaySymbols }, 0, 1)
    }

    pub(super) fn calendar_display_strings(field: i32, style: i32) -> Option<StringArrayData> {
        match CalendarDisplayField::from_i32(field) {
            Some(CalendarDisplayField::Era) => eras(style),
            Some(CalendarDisplayField::DayOfWeek) => weekdays_for_style(style),
            Some(CalendarDisplayField::AmPm) => am_pm_strings(style),
            None => None,
        }
    }

    pub(super) fn display_string(tag: &str, r#type: i32, value: &str) -> Option<String> {
        unsafe {
            let tag = cfstring_create(tag)?;
            let locale = CFLocaleCreate(kCFAllocatorDefault, tag);
            release(tag.cast::<c_void>());
            if locale.is_null() {
                return None;
            }
            let value = cfstring_create(value)?;
            let key = match MacDisplayNameKind::from_i32(r#type) {
                Some(MacDisplayNameKind::Language) => kCFLocaleLanguageCode,
                Some(MacDisplayNameKind::Script) => kCFLocaleScriptCode,
                Some(MacDisplayNameKind::Region) => kCFLocaleCountryCode,
                Some(MacDisplayNameKind::Variant) => kCFLocaleVariantCode,
                Some(MacDisplayNameKind::CurrencyCode) => kCFLocaleCurrencyCode,
                Some(MacDisplayNameKind::CurrencySymbol) => kCFLocaleCurrencySymbol,
                None => {
                    release(value.cast::<c_void>());
                    release(locale.cast::<c_void>());
                    return None;
                }
            };
            let display = CFLocaleCopyDisplayNameForPropertyValue(locale, key, value);
            let result = cfstring_to_string(display);
            release_opt(display.cast::<c_void>());
            release(value.cast::<c_void>());
            release(locale.cast::<c_void>());
            result
        }
    }

    pub(super) fn time_zone_display_string(
        tag: &str,
        r#type: i32,
        timezone: &str,
    ) -> Option<String> {
        unsafe {
            let tag = cfstring_create(tag)?;
            let locale = CFLocaleCreate(kCFAllocatorDefault, tag);
            release(tag.cast::<c_void>());
            if locale.is_null() {
                return None;
            }
            let timezone_name = cfstring_create(timezone)?;
            let timezone =
                CFTimeZoneCreateWithName(kCFAllocatorDefault, timezone_name, Boolean::from(false));
            release(timezone_name.cast::<c_void>());
            if timezone.is_null() {
                release(locale.cast::<c_void>());
                return None;
            }
            let style = match MacTimeZoneNameKind::from_i32(r#type) {
                Some(MacTimeZoneNameKind::ShortStandard) => kCFTimeZoneNameStyleShortStandard,
                Some(MacTimeZoneNameKind::ShortDaylightSaving) => {
                    kCFTimeZoneNameStyleShortDaylightSaving
                }
                Some(MacTimeZoneNameKind::LongStandard) => kCFTimeZoneNameStyleStandard,
                Some(MacTimeZoneNameKind::LongDaylightSaving) => kCFTimeZoneNameStyleDaylightSaving,
                None => {
                    release(timezone.cast::<c_void>());
                    release(locale.cast::<c_void>());
                    return None;
                }
            };
            let display = CFTimeZoneCopyLocalizedName(timezone, style, locale);
            let result = cfstring_to_string(display);
            release_opt(display.cast::<c_void>());
            release(timezone.cast::<c_void>());
            release(locale.cast::<c_void>());
            result
        }
    }

    fn date_array(
        key: CFDateFormatterKey,
        source_index: usize,
        dest_index: usize,
    ) -> Option<StringArrayData> {
        unsafe {
            let formatter = current_date_formatter(0, 0)?;
            let array: CFArrayRef = CFDateFormatterCopyProperty(formatter, key).cast();
            if array.is_null() {
                release(formatter.cast::<c_void>());
                return None;
            }
            let count = usize::try_from(CFArrayGetCount(array)).ok()?;
            let replacements = copy_array_elements(array, source_index, dest_index, count);
            release_opt(array.cast::<c_void>());
            release(formatter.cast::<c_void>());
            Some(StringArrayData {
                len: dest_index + count,
                replacements,
            })
        }
    }

    fn weekdays_for_style(style: i32) -> Option<StringArrayData> {
        unsafe {
            let formatter = current_date_formatter(style, style)?;
            let array: CFArrayRef =
                CFDateFormatterCopyProperty(formatter, kCFDateFormatterWeekdaySymbols).cast();
            if array.is_null() {
                release(formatter.cast::<c_void>());
                return None;
            }
            let count = usize::try_from(CFArrayGetCount(array)).ok()?;
            let replacements = copy_array_elements(array, 0, 1, count);
            release_opt(array.cast::<c_void>());
            release(formatter.cast::<c_void>());
            Some(StringArrayData {
                len: count + 1,
                replacements,
            })
        }
    }

    unsafe fn current_date_formatter(
        date_style: i32,
        time_style: i32,
    ) -> Option<core_foundation_sys::date_formatter::CFDateFormatterRef> {
        let locale = unsafe { CFLocaleCopyCurrent() };
        if locale.is_null() {
            return None;
        }
        let formatter = unsafe {
            CFDateFormatterCreate(
                kCFAllocatorDefault,
                locale,
                convert_date_formatter_style(date_style),
                convert_date_formatter_style(time_style),
            )
        };
        unsafe { release(locale.cast::<c_void>()) };
        if formatter.is_null() {
            None
        } else {
            Some(formatter)
        }
    }

    fn current_calendar_is_japanese() -> bool {
        unsafe {
            let locale = CFLocaleCopyCurrent();
            if locale.is_null() {
                return false;
            }
            let calendar = CFLocaleGetValue(locale, kCFLocaleCalendarIdentifier).cast();
            let current = cfstring_to_string(calendar);
            let japanese = cfstring_to_string(kCFJapaneseCalendar);
            release(locale.cast::<c_void>());
            current.is_some_and(|current| japanese.is_some_and(|japanese| current == japanese))
        }
    }

    unsafe fn cfstring_create(value: &str) -> Option<CFStringRef> {
        let len = CFIndex::try_from(value.len()).ok()?;
        let string = unsafe {
            CFStringCreateWithBytes(
                kCFAllocatorDefault,
                value.as_ptr(),
                len,
                kCFStringEncodingUTF8,
                Boolean::from(false),
            )
        };
        if string.is_null() { None } else { Some(string) }
    }

    unsafe fn cfstring_to_string(string: CFStringRef) -> Option<String> {
        if string.is_null() {
            return None;
        }
        let length = unsafe { CFStringGetLength(string) };
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
                string,
                range,
                kCFStringEncodingUTF8,
                0,
                Boolean::from(false),
                ptr::null_mut(),
                0,
                &raw mut needed,
            )
        };
        if converted == 0 || needed <= 0 {
            return None;
        }
        let mut buffer = vec![0_u8; usize::try_from(needed).ok()?];
        let written = unsafe {
            CFStringGetBytes(
                string,
                range,
                kCFStringEncodingUTF8,
                0,
                Boolean::from(false),
                buffer.as_mut_ptr(),
                needed,
                ptr::null_mut(),
            )
        };
        if written == 0 {
            return None;
        }
        String::from_utf8(buffer).ok()
    }

    unsafe fn cfstring_first_char(string: CFStringRef) -> Option<i32> {
        if string.is_null() || unsafe { CFStringGetLength(string) } <= 0 {
            return None;
        }
        Some(i32::from(unsafe { CFStringGetCharacterAtIndex(string, 0) }))
    }

    unsafe fn copy_array_elements(
        array: CFArrayRef,
        source_index: usize,
        dest_index: usize,
        count: usize,
    ) -> Vec<(usize, String)> {
        if array.is_null() {
            return Vec::new();
        }
        let mut replacements = Vec::with_capacity(count);
        for index in 0..count {
            let Some(source) = source_index.checked_add(index) else {
                break;
            };
            let Ok(source) = CFIndex::try_from(source) else {
                break;
            };
            let string = unsafe { CFArrayGetValueAtIndex(array, source) }.cast();
            if let Some(value) = unsafe { cfstring_to_string(string) } {
                replacements.push((dest_index + index, value));
            }
        }
        replacements
    }

    unsafe fn release(pointer: *const c_void) {
        unsafe { CFRelease(pointer as CFTypeRef) };
    }

    unsafe fn release_opt(pointer: *const c_void) {
        if !pointer.is_null() {
            unsafe { release(pointer) };
        }
    }
}

#[cfg(target_os = "windows")]
mod windows {
    #![expect(unsafe_code)]

    use super::{
        CalendarDisplayField, NumberFormatKind, StringArrayData, WindowsCalendarDataKind,
        WindowsCalendarStyleFlag, WindowsDisplayNameKind,
    };
    use std::ptr;
    use windows_sys::Win32::Foundation::LPARAM;
    use windows_sys::Win32::Globalization::{
        CAL_GREGORIAN, CAL_GREGORIAN_ARABIC, CAL_GREGORIAN_ME_FRENCH, CAL_GREGORIAN_US,
        CAL_GREGORIAN_XLIT_ENGLISH, CAL_GREGORIAN_XLIT_FRENCH, CAL_HIJRI, CAL_JAPAN,
        CAL_RETURN_GENITIVE_NAMES, CAL_SABBREVDAYNAME1, CAL_SABBREVDAYNAME2, CAL_SABBREVDAYNAME3,
        CAL_SABBREVDAYNAME4, CAL_SABBREVDAYNAME5, CAL_SABBREVDAYNAME6, CAL_SABBREVDAYNAME7,
        CAL_SABBREVERASTRING, CAL_SABBREVMONTHNAME1, CAL_SABBREVMONTHNAME2, CAL_SABBREVMONTHNAME3,
        CAL_SABBREVMONTHNAME4, CAL_SABBREVMONTHNAME5, CAL_SABBREVMONTHNAME6, CAL_SABBREVMONTHNAME7,
        CAL_SABBREVMONTHNAME8, CAL_SABBREVMONTHNAME9, CAL_SABBREVMONTHNAME10,
        CAL_SABBREVMONTHNAME11, CAL_SABBREVMONTHNAME12, CAL_SABBREVMONTHNAME13, CAL_SDAYNAME1,
        CAL_SDAYNAME2, CAL_SDAYNAME3, CAL_SDAYNAME4, CAL_SDAYNAME5, CAL_SDAYNAME6, CAL_SDAYNAME7,
        CAL_SERASTRING, CAL_SMONTHNAME1, CAL_SMONTHNAME2, CAL_SMONTHNAME3, CAL_SMONTHNAME4,
        CAL_SMONTHNAME5, CAL_SMONTHNAME6, CAL_SMONTHNAME7, CAL_SMONTHNAME8, CAL_SMONTHNAME9,
        CAL_SMONTHNAME10, CAL_SMONTHNAME11, CAL_SMONTHNAME12, CAL_SMONTHNAME13, CAL_TAIWAN,
        CAL_THAI, CAL_UMALQURA, EnumCalendarInfoExEx, GetCalendarInfoEx, GetLocaleInfoEx,
        GetUserDefaultLocaleName, GetUserDefaultUILanguage, LCIDToLocaleName, LOCALE_ICALENDARTYPE,
        LOCALE_ICURRDIGITS, LOCALE_ICURRENCY, LOCALE_IDIGITS, LOCALE_IDIGITSUBSTITUTION,
        LOCALE_IFIRSTDAYOFWEEK, LOCALE_IFIRSTWEEKOFYEAR, LOCALE_ILZERO, LOCALE_INEGATIVEPERCENT,
        LOCALE_INEGCURR, LOCALE_INEGNUMBER, LOCALE_IPOSITIVEPERCENT, LOCALE_RETURN_NUMBER,
        LOCALE_SAM, LOCALE_SGROUPING, LOCALE_SLOCALIZEDCOUNTRYNAME, LOCALE_SLOCALIZEDLANGUAGENAME,
        LOCALE_SLONGDATE, LOCALE_SNATIVECURRNAME, LOCALE_SPM, LOCALE_SSHORTDATE, LOCALE_SSHORTTIME,
        LOCALE_STIMEFORMAT,
    };
    pub(super) use windows_sys::Win32::Globalization::{
        LOCALE_SCURRENCY, LOCALE_SDECIMAL, LOCALE_SINTLSYMBOL, LOCALE_SMONDECIMALSEP, LOCALE_SNAN,
        LOCALE_SNATIVEDIGITS, LOCALE_SNEGATIVESIGN, LOCALE_SPERCENT, LOCALE_SPERMILLE,
        LOCALE_SPOSINFINITY, LOCALE_STHOUSAND,
    };
    use windows_sys::Win32::System::SystemServices::LOCALE_NAME_MAX_LENGTH;
    use windows_sys::core::{BOOL, PCWSTR};

    const BUFLEN: usize = 256;
    const MONTH_TYPES: [u32; 13] = [
        CAL_SMONTHNAME1,
        CAL_SMONTHNAME2,
        CAL_SMONTHNAME3,
        CAL_SMONTHNAME4,
        CAL_SMONTHNAME5,
        CAL_SMONTHNAME6,
        CAL_SMONTHNAME7,
        CAL_SMONTHNAME8,
        CAL_SMONTHNAME9,
        CAL_SMONTHNAME10,
        CAL_SMONTHNAME11,
        CAL_SMONTHNAME12,
        CAL_SMONTHNAME13,
    ];
    const SHORT_MONTH_TYPES: [u32; 13] = [
        CAL_SABBREVMONTHNAME1,
        CAL_SABBREVMONTHNAME2,
        CAL_SABBREVMONTHNAME3,
        CAL_SABBREVMONTHNAME4,
        CAL_SABBREVMONTHNAME5,
        CAL_SABBREVMONTHNAME6,
        CAL_SABBREVMONTHNAME7,
        CAL_SABBREVMONTHNAME8,
        CAL_SABBREVMONTHNAME9,
        CAL_SABBREVMONTHNAME10,
        CAL_SABBREVMONTHNAME11,
        CAL_SABBREVMONTHNAME12,
        CAL_SABBREVMONTHNAME13,
    ];
    const WEEKDAY_TYPES: [u32; 7] = [
        CAL_SDAYNAME7,
        CAL_SDAYNAME1,
        CAL_SDAYNAME2,
        CAL_SDAYNAME3,
        CAL_SDAYNAME4,
        CAL_SDAYNAME5,
        CAL_SDAYNAME6,
    ];
    const SHORT_WEEKDAY_TYPES: [u32; 7] = [
        CAL_SABBREVDAYNAME7,
        CAL_SABBREVDAYNAME1,
        CAL_SABBREVDAYNAME2,
        CAL_SABBREVDAYNAME3,
        CAL_SABBREVDAYNAME4,
        CAL_SABBREVDAYNAME5,
        CAL_SABBREVDAYNAME6,
    ];
    const AMPM_TYPES: [u32; 2] = [LOCALE_SAM, LOCALE_SPM];

    const FIXES: [[[[&str; 16]; 3]; 2]; 2] = [
        [
            [
                [
                    "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
                ],
                [
                    "\u{00a4}",
                    "",
                    "\u{00a4} ",
                    "",
                    "",
                    "",
                    "",
                    "",
                    "",
                    "",
                    "",
                    "",
                    "",
                    "",
                    "",
                    "",
                ],
                [
                    "", "", "%", "% ", "", "", "", "", "", "", "", "", "", "", "", "",
                ],
            ],
            [
                [
                    "(", "-", "- ", "", "", "", "", "", "", "", "", "", "", "", "", "",
                ],
                [
                    "(\u{00a4}",
                    "-\u{00a4}",
                    "\u{00a4}-",
                    "\u{00a4}",
                    "(",
                    "-",
                    "",
                    "",
                    "-",
                    "-\u{00a4} ",
                    "",
                    "\u{00a4} ",
                    "\u{00a4} -",
                    "",
                    "(\u{00a4} ",
                    "(",
                ],
                [
                    "-", "-", "-%", "%-", "%", "", "", "-% ", "", "% ", "% -", "", "", "", "", "",
                ],
            ],
        ],
        [
            [
                [
                    "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
                ],
                [
                    "",
                    "\u{00a4} ",
                    "",
                    " \u{00a4}",
                    "",
                    "",
                    "",
                    "",
                    "",
                    "",
                    "",
                    "",
                    "",
                    "",
                    "",
                    "",
                ],
                [
                    " %", "%", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
                ],
            ],
            [
                [
                    ")", "", " ", "-", " -", "", "", "", "", "", "", "", "", "", "", "",
                ],
                [
                    ")",
                    "",
                    "",
                    "-",
                    "\u{00a4})",
                    "\u{00a4}",
                    "-\u{00a4}",
                    "\u{00a4}-",
                    " \u{00a4}",
                    "",
                    " \u{00a4}-",
                    "-",
                    "",
                    "- \u{00a4}",
                    ")",
                    " \u{00a4})",
                ],
                [
                    " %", "%", "", "", "-", "-%", "%-", "", " %-", "-", "", "- %", "", "", "", "",
                ],
            ],
        ],
    ];

    pub(super) fn default_locale(cat: i32) -> String {
        let locale = if cat == 0 {
            unsafe {
                let langid = u32::from(GetUserDefaultUILanguage());
                locale_name_from_lcid(langid)
            }
        } else {
            user_default_locale_name()
        }
        .or_else(user_default_locale_name)
        .unwrap_or_else(|| "en".to_string());
        locale.replace('-', "_")
    }

    pub(super) fn locale_info_string(tag: &str, r#type: u32) -> Option<String> {
        let locale = locale_name(tag);
        let mut buffer = [0_u16; BUFLEN];
        let written = unsafe {
            GetLocaleInfoEx(
                locale.as_ptr(),
                r#type,
                buffer.as_mut_ptr(),
                i32::try_from(buffer.len()).ok()?,
            )
        };
        string_from_buffer(&buffer, written)
    }

    pub(super) fn locale_info_char(tag: &str, r#type: u32) -> Option<i32> {
        locale_info_string(tag, r#type).and_then(|value| value.encode_utf16().next().map(i32::from))
    }

    pub(super) fn calendar_data_value(tag: &str, r#type: i32) -> i32 {
        let r#type = match WindowsCalendarDataKind::from_i32(r#type) {
            Some(WindowsCalendarDataKind::FirstDayOfWeek) => LOCALE_IFIRSTDAYOFWEEK,
            Some(WindowsCalendarDataKind::FirstWeekOfYear) => LOCALE_IFIRSTWEEKOFYEAR,
            None => return -1,
        };
        locale_info_number(tag, r#type)
            .and_then(|value| i32::try_from(value).ok())
            .unwrap_or(-1)
    }

    pub(super) fn calendar_id(tag: &str) -> i32 {
        let Some(r#type) = locale_info_number(tag, LOCALE_ICALENDARTYPE) else {
            return -1;
        };
        match r#type {
            CAL_GREGORIAN
            | CAL_GREGORIAN_US
            | CAL_JAPAN
            | CAL_TAIWAN
            | CAL_HIJRI
            | CAL_THAI
            | CAL_GREGORIAN_ME_FRENCH
            | CAL_GREGORIAN_ARABIC
            | CAL_GREGORIAN_XLIT_ENGLISH
            | CAL_GREGORIAN_XLIT_FRENCH
            | CAL_UMALQURA => i32::try_from(r#type).unwrap_or(-1),
            _ => -1,
        }
    }

    pub(super) fn am_pm_strings(tag: &str) -> Option<StringArrayData> {
        replace_calendar_array_elements(tag, -1, &AMPM_TYPES, 0, 0, false)
    }

    pub(super) fn eras(tag: &str, calid: i32, style: i32) -> Option<StringArrayData> {
        eras_impl(tag, calid, style)
    }

    pub(super) fn months(tag: &str) -> Option<StringArrayData> {
        replace_calendar_array_elements(tag, -1, &MONTH_TYPES, 0, 0, true)
    }

    pub(super) fn short_months(tag: &str) -> Option<StringArrayData> {
        replace_calendar_array_elements(tag, -1, &SHORT_MONTH_TYPES, 0, 0, true)
    }

    pub(super) fn weekdays(tag: &str) -> Option<StringArrayData> {
        replace_calendar_array_elements(tag, -1, &WEEKDAY_TYPES, 1, 0, true)
    }

    pub(super) fn short_weekdays(tag: &str) -> Option<StringArrayData> {
        replace_calendar_array_elements(tag, -1, &SHORT_WEEKDAY_TYPES, 1, 0, true)
    }

    pub(super) fn calendar_display_strings(
        tag: &str,
        calid: i32,
        field: i32,
        style: i32,
    ) -> Option<StringArrayData> {
        match CalendarDisplayField::from_i32(field) {
            Some(CalendarDisplayField::Era) => eras_impl(tag, calid, style),
            Some(CalendarDisplayField::AmPm) => {
                replace_calendar_array_elements(tag, calid, &AMPM_TYPES, 0, style, false)
            }
            Some(CalendarDisplayField::DayOfWeek) => {
                let types = if WindowsCalendarStyleFlag::Short.is_set(style) {
                    &SHORT_WEEKDAY_TYPES
                } else {
                    &WEEKDAY_TYPES
                };
                replace_calendar_array_elements(tag, calid, types, 0, style, true)
            }
            Some(CalendarDisplayField::Month) => {
                let types = if WindowsCalendarStyleFlag::Short.is_set(style) {
                    &SHORT_MONTH_TYPES
                } else {
                    &MONTH_TYPES
                };
                replace_calendar_array_elements(tag, calid, types, 0, style, true)
            }
            None => None,
        }
    }

    pub(super) fn date_time_pattern(tag: &str, date_style: i32, time_style: i32) -> Option<String> {
        let r#type = if date_style == 0 || date_style == 1 {
            Some(LOCALE_SLONGDATE)
        } else if date_style == 2 || date_style == 3 {
            Some(LOCALE_SSHORTDATE)
        } else if time_style == 0 || time_style == 1 {
            Some(LOCALE_STIMEFORMAT)
        } else if time_style == 2 || time_style == 3 {
            Some(LOCALE_SSHORTTIME)
        } else {
            None
        }?;
        locale_info_string(tag, r#type)
    }

    pub(super) fn number_pattern(tag: &str, style: i32) -> String {
        let mut pattern = String::new();
        pattern.push_str(&fix_part(tag, style, true, true));
        let number = number_part(tag, style);
        pattern.push_str(&number);
        pattern.push_str(&fix_part(tag, style, true, false));
        pattern.push(';');
        pattern.push_str(&fix_part(tag, style, false, true));
        pattern.push_str(&number);
        pattern.push_str(&fix_part(tag, style, false, false));
        pattern
    }

    pub(super) fn display_string(tag: &str, r#type: i32, value: &str) -> Option<String> {
        let (locale_type, query) = match WindowsDisplayNameKind::from_i32(r#type) {
            Some(WindowsDisplayNameKind::CurrencyName) => (LOCALE_SNATIVECURRNAME, tag),
            Some(WindowsDisplayNameKind::CurrencySymbol) => (LOCALE_SCURRENCY, tag),
            Some(WindowsDisplayNameKind::Language) => (LOCALE_SLOCALIZEDLANGUAGENAME, value),
            Some(WindowsDisplayNameKind::Region) => (LOCALE_SLOCALIZEDCOUNTRYNAME, value),
            None => return None,
        };
        locale_info_string(query, locale_type)
    }

    pub(super) fn is_native_digit(tag: &str) -> bool {
        locale_info_number(tag, LOCALE_IDIGITSUBSTITUTION).is_some_and(|value| value == 2)
    }

    fn locale_info_number(tag: &str, r#type: u32) -> Option<u32> {
        let locale = locale_name(tag);
        let mut number = 0_u32;
        let got = unsafe {
            GetLocaleInfoEx(
                locale.as_ptr(),
                r#type | LOCALE_RETURN_NUMBER,
                (&raw mut number).cast::<u16>(),
                i32::try_from(size_of::<u32>()).ok()?,
            )
        };
        if got == 0 { None } else { Some(number) }
    }

    fn calendar_info_string(tag: &str, calid: i32, r#type: u32) -> Option<String> {
        let locale = locale_name(tag);
        let mut buffer = [0_u16; BUFLEN];
        let got = unsafe {
            GetCalendarInfoEx(
                locale.as_ptr(),
                u32::try_from(calid).ok()?,
                ptr::null(),
                r#type,
                buffer.as_mut_ptr(),
                i32::try_from(buffer.len()).ok()?,
                ptr::null_mut(),
            )
        };
        string_from_buffer(&buffer, got)
    }

    fn replace_calendar_array_elements(
        tag: &str,
        mut calid: i32,
        types: &[u32],
        offset: usize,
        style: i32,
        calendar_lookup: bool,
    ) -> Option<StringArrayData> {
        if calid < 0 {
            calid = calendar_id(tag);
        }
        if calid == -1 {
            return None;
        }
        let genitive = if WindowsCalendarStyleFlag::StandAlone.is_set(style) {
            0
        } else {
            CAL_RETURN_GENITIVE_NAMES
        };
        let mut replacements = Vec::with_capacity(types.len());
        for (index, calendar_type) in types.iter().enumerate() {
            let calendar_type = *calendar_type | genitive;
            let value = if calendar_lookup {
                calendar_info_string(tag, calid, calendar_type)
                    .or_else(|| locale_info_string(tag, calendar_type))
            } else {
                locale_info_string(tag, calendar_type)
            };
            if let Some(value) = value.filter(|value| !value.is_empty()) {
                replacements.push((index + offset, value));
            }
        }
        Some(StringArrayData {
            len: offset + types.len(),
            replacements,
        })
    }

    fn eras_impl(tag: &str, mut calid: i32, style: i32) -> Option<StringArrayData> {
        if calid < 0 {
            calid = calendar_id(tag);
        }
        if calid == -1 {
            return None;
        }
        let r#type = if WindowsCalendarStyleFlag::Short.is_set(style) {
            CAL_SABBREVERASTRING
        } else {
            CAL_SERASTRING
        };
        let eras = enum_calendar_info(tag, calid, r#type)?;
        let era_count = eras.len();
        let mut replacements = Vec::with_capacity(era_count + 1);
        let array_len = era_count + 1;
        for (era_index, era) in eras.into_iter().enumerate() {
            let index = era_count.saturating_sub(era_index);
            if index < array_len && !era.is_empty() {
                replacements.push((index, era));
            }
        }
        if calid == i32::try_from(CAL_JAPAN).ok()?
            && let Some(mut gregorian) =
                enum_calendar_info(tag, i32::try_from(CAL_GREGORIAN).ok()?, r#type)
            && let Some(first) = gregorian.pop()
        {
            replacements.push((0, first));
        }
        Some(StringArrayData {
            len: array_len,
            replacements,
        })
    }

    fn enum_calendar_info(tag: &str, calid: i32, r#type: u32) -> Option<Vec<String>> {
        let locale = locale_name(tag);
        let mut buffer = Vec::<u16>::new();
        let lparam = LPARAM::try_from((&raw mut buffer).addr()).ok()?;
        let got = unsafe {
            EnumCalendarInfoExEx(
                Some(enum_calendar_info_proc),
                locale.as_ptr(),
                u32::try_from(calid).ok()?,
                ptr::null(),
                r#type,
                lparam,
            )
        };
        if got == 0 || buffer.is_empty() {
            return None;
        }
        Some(
            String::from_utf16_lossy(&buffer)
                .split(',')
                .filter(|value| !value.is_empty())
                .map(str::to_string)
                .collect(),
        )
    }

    unsafe extern "system" fn enum_calendar_info_proc(
        info: PCWSTR,
        _calid: u32,
        _reserved: PCWSTR,
        lparam: LPARAM,
    ) -> BOOL {
        let Some(buffer) = (unsafe { (lparam as *mut Vec<u16>).as_mut() }) else {
            return 0;
        };
        let mut current = info;
        while !current.is_null() && unsafe { *current } != 0 {
            buffer.push(unsafe { *current });
            current = unsafe { current.add(1) };
        }
        buffer.push(u16::from(b','));
        1
    }

    fn number_part(tag: &str, style: i32) -> String {
        let digits = match NumberFormatKind::from_i32(style) {
            Some(NumberFormatKind::Currency) => {
                locale_info_number(tag, LOCALE_ICURRDIGITS).unwrap_or(0)
            }
            Some(NumberFormatKind::Integer) => 0,
            Some(NumberFormatKind::Decimal | NumberFormatKind::Percent) | None => {
                locale_info_number(tag, LOCALE_IDIGITS).unwrap_or(0)
            }
        };
        let leading_zero = locale_info_number(tag, LOCALE_ILZERO).unwrap_or(0);
        let grouping = locale_info_string(tag, LOCALE_SGROUPING).unwrap_or_default();
        let mut integer = String::new();
        for ch in grouping.chars().rev() {
            if ch == ';' {
                continue;
            }
            let repnum = ch.to_digit(10).unwrap_or(0);
            if repnum > 0 {
                integer.push('#');
                integer.push(',');
                for _ in 1..repnum {
                    integer.push('#');
                }
            }
        }
        integer.push(if leading_zero == 0 { '#' } else { '0' });
        if digits > 0 {
            integer.push('.');
            for _ in 0..digits {
                integer.push('#');
            }
        }
        integer
    }

    fn fix_part(tag: &str, style: i32, positive: bool, prefix: bool) -> String {
        let number_format_kind =
            NumberFormatKind::from_i32(style).unwrap_or(NumberFormatKind::Decimal);
        let pattern = if positive {
            match number_format_kind {
                NumberFormatKind::Currency => {
                    locale_info_number(tag, LOCALE_ICURRENCY).unwrap_or(0)
                }
                NumberFormatKind::Percent => {
                    locale_info_number(tag, LOCALE_IPOSITIVEPERCENT).unwrap_or(0)
                }
                _ => 0,
            }
        } else {
            match number_format_kind {
                NumberFormatKind::Currency => locale_info_number(tag, LOCALE_INEGCURR).unwrap_or(0),
                NumberFormatKind::Percent => {
                    locale_info_number(tag, LOCALE_INEGATIVEPERCENT).unwrap_or(0)
                }
                _ => locale_info_number(tag, LOCALE_INEGNUMBER).unwrap_or(0),
            }
        };
        let prefix_index = usize::from(!prefix);
        let positive_index = usize::from(!positive);
        let style_index = number_format_kind.windows_fix_index();
        let pattern_index = usize::try_from(pattern).unwrap_or(0);
        FIXES
            .get(prefix_index)
            .and_then(|values| values.get(positive_index))
            .and_then(|values| values.get(style_index))
            .and_then(|values| values.get(pattern_index))
            .copied()
            .unwrap_or("")
            .to_string()
    }

    fn locale_name(tag: &str) -> Vec<u16> {
        if tag == "und" {
            wide_null("en")
        } else {
            wide_null(tag)
        }
    }

    fn wide_null(value: &str) -> Vec<u16> {
        value.encode_utf16().chain([0]).collect()
    }

    fn string_from_buffer(buffer: &[u16], written: i32) -> Option<String> {
        if written <= 0 {
            return None;
        }
        let len = usize::try_from(written).ok()?.saturating_sub(1);
        let value = String::from_utf16(&buffer[..len]).ok()?;
        if value.is_empty() { None } else { Some(value) }
    }

    fn user_default_locale_name() -> Option<String> {
        let mut buffer = [0_u16; LOCALE_NAME_MAX_LENGTH as usize];
        let written = unsafe {
            GetUserDefaultLocaleName(buffer.as_mut_ptr(), i32::try_from(buffer.len()).ok()?)
        };
        string_from_buffer(&buffer, written)
    }

    fn locale_name_from_lcid(lcid: u32) -> Option<String> {
        let mut buffer = [0_u16; LOCALE_NAME_MAX_LENGTH as usize];
        let written = unsafe {
            LCIDToLocaleName(
                lcid,
                buffer.as_mut_ptr(),
                i32::try_from(buffer.len()).ok()?,
                0,
            )
        };
        string_from_buffer(&buffer, written)
    }
}

#[cfg(not(target_os = "linux"))]
#[intrinsic_method(
    "sun/util/locale/provider/HostLocaleProviderAdapterImpl.getAmPmStrings(Ljava/lang/String;[Ljava/lang/String;)[Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_am_pm_strings<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let default_array = parameters.pop()?;
    let tag = parameters.pop()?;
    #[cfg(target_os = "macos")]
    let data = {
        let _tag = java_string(&tag);
        macos::am_pm_strings(0)
    };
    #[cfg(target_os = "windows")]
    let data = java_string(&tag).and_then(|tag| windows::am_pm_strings(&tag));
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    let data: Option<StringArrayData> = {
        let _ = &tag;
        None
    };
    let result = match data {
        Some(data) => update_or_create_string_array(&thread, default_array, data).await?,
        None => default_array,
    };
    Ok(Some(result))
}

#[cfg(target_os = "macos")]
#[intrinsic_method(
    "sun/util/locale/provider/HostLocaleProviderAdapterImpl.getCalendarDisplayStrings(Ljava/lang/String;II)[Ljava/lang/String;",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn get_calendar_display_strings<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let style = parameters.pop_int()?;
    let field = parameters.pop_int()?;
    let tag = parameters.pop()?;
    let _tag = java_string(&tag);
    let data = macos::calendar_display_strings(field, style);
    match data {
        Some(data) => Ok(Some(new_string_array_from_data(&thread, data).await?)),
        None => Ok(Some(Value::Object(None))),
    }
}

#[cfg(target_os = "macos")]
#[intrinsic_method(
    "sun/util/locale/provider/HostLocaleProviderAdapterImpl.getCalendarID(Ljava/lang/String;)Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_calendar_id<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let tag = parameters.pop()?;
    let _tag = java_string(&tag);
    match macos::calendar_id() {
        Some(calendar_id) => Ok(Some(calendar_id.to_object(&thread).await?)),
        None => Ok(Some(Value::Object(None))),
    }
}

#[cfg(target_os = "macos")]
#[intrinsic_method(
    "sun/util/locale/provider/HostLocaleProviderAdapterImpl.getCalendarInt(Ljava/lang/String;I)I",
    Any
)]
#[async_method]
pub async fn get_calendar_int<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let r#type = parameters.pop_int()?;
    let tag = parameters.pop()?;
    let _tag = java_string(&tag);
    let value = macos::calendar_int(r#type);
    Ok(Some(Value::Int(value)))
}

#[cfg(not(target_os = "linux"))]
#[intrinsic_method(
    "sun/util/locale/provider/HostLocaleProviderAdapterImpl.getCurrencySymbol(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_currency_symbol<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let default_symbol = parameters.pop()?;
    let tag = parameters.pop()?;
    let symbol = {
        #[cfg(target_os = "macos")]
        {
            let _tag = java_string(&tag);
            macos::currency_symbol()
        }
        #[cfg(target_os = "windows")]
        {
            java_string(&tag)
                .and_then(|tag| windows::locale_info_string(&tag, windows::LOCALE_SCURRENCY))
        }
        #[cfg(not(any(target_os = "macos", target_os = "windows")))]
        {
            let _ = &tag;
            Option::<String>::None
        }
    };
    match symbol {
        Some(symbol) => Ok(Some(symbol.to_object(&thread).await?)),
        None => Ok(Some(default_symbol)),
    }
}

#[cfg(target_os = "macos")]
#[intrinsic_method(
    "sun/util/locale/provider/HostLocaleProviderAdapterImpl.getDateTimePatternNative(IILjava/lang/String;)Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_date_time_pattern_native<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let tag = parameters.pop()?;
    let time_style = parameters.pop_int()?;
    let date_style = parameters.pop_int()?;
    let _tag = java_string(&tag);
    match macos::date_time_pattern(date_style, time_style) {
        Some(pattern) => Ok(Some(pattern.to_object(&thread).await?)),
        None => Ok(Some(Value::Object(None))),
    }
}

#[cfg(not(target_os = "linux"))]
#[intrinsic_method(
    "sun/util/locale/provider/HostLocaleProviderAdapterImpl.getDecimalSeparator(Ljava/lang/String;C)C",
    Any
)]
#[async_method]
pub async fn get_decimal_separator<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let default_char = parameters.pop_int()?;
    let tag = parameters.pop()?;
    let value = {
        #[cfg(target_os = "macos")]
        {
            let _tag = java_string(&tag);
            macos::decimal_separator()
        }
        #[cfg(target_os = "windows")]
        {
            java_string(&tag)
                .and_then(|tag| windows::locale_info_char(&tag, windows::LOCALE_SDECIMAL))
        }
        #[cfg(not(any(target_os = "macos", target_os = "windows")))]
        {
            let _ = &tag;
            Option::<i32>::None
        }
    }
    .unwrap_or(default_char);
    Ok(Some(Value::Int(value)))
}

#[cfg(not(target_os = "linux"))]
#[intrinsic_method(
    "sun/util/locale/provider/HostLocaleProviderAdapterImpl.getDefaultLocale(I)Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_default_locale<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    #[cfg(target_os = "windows")]
    let cat = parameters.pop_int()?;
    #[cfg(not(target_os = "windows"))]
    let _ = parameters.pop_int()?;
    let locale = {
        #[cfg(target_os = "windows")]
        {
            windows::default_locale(cat)
        }
        #[cfg(not(target_os = "windows"))]
        {
            let (language, country) = detect_default_locale();
            match country {
                Some(country) if !country.is_empty() => format!("{language}_{country}"),
                _ => language,
            }
        }
    };
    Ok(Some(locale.to_object(&thread).await?))
}

#[cfg(not(target_os = "linux"))]
#[intrinsic_method(
    "sun/util/locale/provider/HostLocaleProviderAdapterImpl.getDisplayString(Ljava/lang/String;ILjava/lang/String;)Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_display_string<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let value = parameters.pop()?;
    let r#type = parameters.pop_int()?;
    let tag = parameters.pop()?;
    let Some(tag) = java_string(&tag) else {
        return Ok(Some(Value::Object(None)));
    };
    let Some(value) = java_string(&value) else {
        return Ok(Some(Value::Object(None)));
    };
    let display = {
        #[cfg(target_os = "macos")]
        {
            macos::display_string(&tag, r#type, &value)
        }
        #[cfg(target_os = "windows")]
        {
            windows::display_string(&tag, r#type, &value)
        }
        #[cfg(not(any(target_os = "macos", target_os = "windows")))]
        {
            let _ = (&tag, r#type, &value);
            Option::<String>::None
        }
    };
    match display {
        Some(display) => Ok(Some(display.to_object(&thread).await?)),
        None => Ok(Some(Value::Object(None))),
    }
}

#[cfg(not(target_os = "linux"))]
#[intrinsic_method(
    "sun/util/locale/provider/HostLocaleProviderAdapterImpl.getEras(Ljava/lang/String;[Ljava/lang/String;)[Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_eras<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let default_array = parameters.pop()?;
    let tag = parameters.pop()?;
    let data = {
        #[cfg(target_os = "macos")]
        {
            let _tag = java_string(&tag);
            macos::eras(0)
        }
        #[cfg(target_os = "windows")]
        {
            java_string(&tag).and_then(|tag| windows::eras(&tag, -1, 0))
        }
        #[cfg(not(any(target_os = "macos", target_os = "windows")))]
        {
            let _ = &tag;
            Option::<StringArrayData>::None
        }
    };
    let result = match data {
        Some(data) => update_or_create_string_array(&thread, default_array, data).await?,
        None => default_array,
    };
    Ok(Some(result))
}

#[cfg(target_os = "macos")]
#[intrinsic_method(
    "sun/util/locale/provider/HostLocaleProviderAdapterImpl.getExponentSeparator(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_exponent_separator<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let default_separator = parameters.pop()?;
    let tag = parameters.pop()?;
    let _tag = java_string(&tag);
    match macos::exponent_separator() {
        Some(separator) => Ok(Some(separator.to_object(&thread).await?)),
        None => Ok(Some(default_separator)),
    }
}

#[cfg(not(target_os = "linux"))]
#[intrinsic_method(
    "sun/util/locale/provider/HostLocaleProviderAdapterImpl.getGroupingSeparator(Ljava/lang/String;C)C",
    Any
)]
#[async_method]
pub async fn get_grouping_separator<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let default_char = parameters.pop_int()?;
    let tag = parameters.pop()?;
    let value = {
        #[cfg(target_os = "macos")]
        {
            let _tag = java_string(&tag);
            macos::grouping_separator()
        }
        #[cfg(target_os = "windows")]
        {
            java_string(&tag)
                .and_then(|tag| windows::locale_info_char(&tag, windows::LOCALE_STHOUSAND))
        }
        #[cfg(not(any(target_os = "macos", target_os = "windows")))]
        {
            let _ = &tag;
            Option::<i32>::None
        }
    }
    .unwrap_or(default_char);
    Ok(Some(Value::Int(value)))
}

#[cfg(not(target_os = "linux"))]
#[intrinsic_method(
    "sun/util/locale/provider/HostLocaleProviderAdapterImpl.getInfinity(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_infinity<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let default_string = parameters.pop()?;
    let tag = parameters.pop()?;
    let value = {
        #[cfg(target_os = "macos")]
        {
            let _tag = java_string(&tag);
            macos::infinity()
        }
        #[cfg(target_os = "windows")]
        {
            java_string(&tag)
                .and_then(|tag| windows::locale_info_string(&tag, windows::LOCALE_SPOSINFINITY))
        }
        #[cfg(not(any(target_os = "macos", target_os = "windows")))]
        {
            let _ = &tag;
            Option::<String>::None
        }
    };
    match value {
        Some(value) => Ok(Some(value.to_object(&thread).await?)),
        None => Ok(Some(default_string)),
    }
}

#[cfg(not(target_os = "linux"))]
#[intrinsic_method(
    "sun/util/locale/provider/HostLocaleProviderAdapterImpl.getInternationalCurrencySymbol(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_international_currency_symbol<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let default_symbol = parameters.pop()?;
    let tag = parameters.pop()?;
    let value = {
        #[cfg(target_os = "macos")]
        {
            let _tag = java_string(&tag);
            macos::international_currency_symbol()
        }
        #[cfg(target_os = "windows")]
        {
            java_string(&tag)
                .and_then(|tag| windows::locale_info_string(&tag, windows::LOCALE_SINTLSYMBOL))
        }
        #[cfg(not(any(target_os = "macos", target_os = "windows")))]
        {
            let _ = &tag;
            Option::<String>::None
        }
    };
    match value {
        Some(value) => Ok(Some(value.to_object(&thread).await?)),
        None => Ok(Some(default_symbol)),
    }
}

#[cfg(not(target_os = "linux"))]
#[intrinsic_method(
    "sun/util/locale/provider/HostLocaleProviderAdapterImpl.getMinusSign(Ljava/lang/String;C)C",
    Any
)]
#[async_method]
pub async fn get_minus_sign<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let default_char = parameters.pop_int()?;
    let tag = parameters.pop()?;
    let value = {
        #[cfg(target_os = "macos")]
        {
            let _tag = java_string(&tag);
            macos::minus_sign()
        }
        #[cfg(target_os = "windows")]
        {
            java_string(&tag)
                .and_then(|tag| windows::locale_info_char(&tag, windows::LOCALE_SNEGATIVESIGN))
        }
        #[cfg(not(any(target_os = "macos", target_os = "windows")))]
        {
            let _ = &tag;
            Option::<i32>::None
        }
    }
    .unwrap_or(default_char);
    Ok(Some(Value::Int(value)))
}

#[cfg(not(target_os = "linux"))]
#[intrinsic_method(
    "sun/util/locale/provider/HostLocaleProviderAdapterImpl.getMonetaryDecimalSeparator(Ljava/lang/String;C)C",
    Any
)]
#[async_method]
pub async fn get_monetary_decimal_separator<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let default_char = parameters.pop_int()?;
    let tag = parameters.pop()?;
    let value = {
        #[cfg(target_os = "macos")]
        {
            let _tag = java_string(&tag);
            macos::monetary_decimal_separator()
        }
        #[cfg(target_os = "windows")]
        {
            java_string(&tag)
                .and_then(|tag| windows::locale_info_char(&tag, windows::LOCALE_SMONDECIMALSEP))
        }
        #[cfg(not(any(target_os = "macos", target_os = "windows")))]
        {
            let _ = &tag;
            Option::<i32>::None
        }
    }
    .unwrap_or(default_char);
    Ok(Some(Value::Int(value)))
}

#[cfg(not(target_os = "linux"))]
#[intrinsic_method(
    "sun/util/locale/provider/HostLocaleProviderAdapterImpl.getMonths(Ljava/lang/String;[Ljava/lang/String;)[Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_months<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let default_array = parameters.pop()?;
    let tag = parameters.pop()?;
    let data = {
        #[cfg(target_os = "macos")]
        {
            let _tag = java_string(&tag);
            macos::months()
        }
        #[cfg(target_os = "windows")]
        {
            java_string(&tag).and_then(|tag| windows::months(&tag))
        }
        #[cfg(not(any(target_os = "macos", target_os = "windows")))]
        {
            let _ = &tag;
            Option::<StringArrayData>::None
        }
    };
    let result = match data {
        Some(data) => update_or_create_string_array(&thread, default_array, data).await?,
        None => default_array,
    };
    Ok(Some(result))
}

#[cfg(not(target_os = "linux"))]
#[intrinsic_method(
    "sun/util/locale/provider/HostLocaleProviderAdapterImpl.getNaN(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_nan<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let default_string = parameters.pop()?;
    let tag = parameters.pop()?;
    let value = {
        #[cfg(target_os = "macos")]
        {
            let _tag = java_string(&tag);
            macos::nan()
        }
        #[cfg(target_os = "windows")]
        {
            java_string(&tag)
                .and_then(|tag| windows::locale_info_string(&tag, windows::LOCALE_SNAN))
        }
        #[cfg(not(any(target_os = "macos", target_os = "windows")))]
        {
            let _ = &tag;
            Option::<String>::None
        }
    };
    match value {
        Some(value) => Ok(Some(value.to_object(&thread).await?)),
        None => Ok(Some(default_string)),
    }
}

#[cfg(target_os = "macos")]
#[intrinsic_method(
    "sun/util/locale/provider/HostLocaleProviderAdapterImpl.getNumberPatternNative(ILjava/lang/String;)Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_number_pattern_native<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let tag = parameters.pop()?;
    let style = parameters.pop_int()?;
    let _tag = java_string(&tag);
    match macos::number_pattern(style) {
        Some(pattern) => Ok(Some(pattern.to_object(&thread).await?)),
        None => Ok(Some(Value::Object(None))),
    }
}

#[cfg(not(target_os = "linux"))]
#[intrinsic_method(
    "sun/util/locale/provider/HostLocaleProviderAdapterImpl.getPerMill(Ljava/lang/String;C)C",
    Any
)]
#[async_method]
pub async fn get_per_mill<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let default_char = parameters.pop_int()?;
    let tag = parameters.pop()?;
    let value = {
        #[cfg(target_os = "macos")]
        {
            let _tag = java_string(&tag);
            macos::per_mill()
        }
        #[cfg(target_os = "windows")]
        {
            java_string(&tag)
                .and_then(|tag| windows::locale_info_char(&tag, windows::LOCALE_SPERMILLE))
        }
        #[cfg(not(any(target_os = "macos", target_os = "windows")))]
        {
            let _ = &tag;
            Option::<i32>::None
        }
    }
    .unwrap_or(default_char);
    Ok(Some(Value::Int(value)))
}

#[cfg(not(target_os = "linux"))]
#[intrinsic_method(
    "sun/util/locale/provider/HostLocaleProviderAdapterImpl.getPercent(Ljava/lang/String;C)C",
    Any
)]
#[async_method]
pub async fn get_percent<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let default_char = parameters.pop_int()?;
    let tag = parameters.pop()?;
    let value = {
        #[cfg(target_os = "macos")]
        {
            let _tag = java_string(&tag);
            macos::percent()
        }
        #[cfg(target_os = "windows")]
        {
            java_string(&tag)
                .and_then(|tag| windows::locale_info_char(&tag, windows::LOCALE_SPERCENT))
        }
        #[cfg(not(any(target_os = "macos", target_os = "windows")))]
        {
            let _ = &tag;
            Option::<i32>::None
        }
    }
    .unwrap_or(default_char);
    Ok(Some(Value::Int(value)))
}

#[cfg(not(target_os = "linux"))]
#[intrinsic_method(
    "sun/util/locale/provider/HostLocaleProviderAdapterImpl.getShortMonths(Ljava/lang/String;[Ljava/lang/String;)[Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_short_months<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let default_array = parameters.pop()?;
    let tag = parameters.pop()?;
    let data = {
        #[cfg(target_os = "macos")]
        {
            let _tag = java_string(&tag);
            macos::short_months()
        }
        #[cfg(target_os = "windows")]
        {
            java_string(&tag).and_then(|tag| windows::short_months(&tag))
        }
        #[cfg(not(any(target_os = "macos", target_os = "windows")))]
        {
            let _ = &tag;
            Option::<StringArrayData>::None
        }
    };
    let result = match data {
        Some(data) => update_or_create_string_array(&thread, default_array, data).await?,
        None => default_array,
    };
    Ok(Some(result))
}

#[cfg(not(target_os = "linux"))]
#[intrinsic_method(
    "sun/util/locale/provider/HostLocaleProviderAdapterImpl.getShortWeekdays(Ljava/lang/String;[Ljava/lang/String;)[Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_short_weekdays<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let default_array = parameters.pop()?;
    let tag = parameters.pop()?;
    let data = {
        #[cfg(target_os = "macos")]
        {
            let _tag = java_string(&tag);
            macos::short_weekdays()
        }
        #[cfg(target_os = "windows")]
        {
            java_string(&tag).and_then(|tag| windows::short_weekdays(&tag))
        }
        #[cfg(not(any(target_os = "macos", target_os = "windows")))]
        {
            let _ = &tag;
            Option::<StringArrayData>::None
        }
    };
    let result = match data {
        Some(data) => update_or_create_string_array(&thread, default_array, data).await?,
        None => default_array,
    };
    Ok(Some(result))
}

#[cfg(target_os = "macos")]
#[intrinsic_method(
    "sun/util/locale/provider/HostLocaleProviderAdapterImpl.getTimeZoneDisplayString(Ljava/lang/String;ILjava/lang/String;)Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_time_zone_display_string<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let value = parameters.pop()?;
    let r#type = parameters.pop_int()?;
    let tag = parameters.pop()?;
    let Some(tag) = java_string(&tag) else {
        return Ok(Some(Value::Object(None)));
    };
    let Some(value) = java_string(&value) else {
        return Ok(Some(Value::Object(None)));
    };
    match macos::time_zone_display_string(&tag, r#type, &value) {
        Some(display) => Ok(Some(display.to_object(&thread).await?)),
        None => Ok(Some(Value::Object(None))),
    }
}

#[cfg(not(target_os = "linux"))]
#[intrinsic_method(
    "sun/util/locale/provider/HostLocaleProviderAdapterImpl.getWeekdays(Ljava/lang/String;[Ljava/lang/String;)[Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_weekdays<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let default_array = parameters.pop()?;
    let tag = parameters.pop()?;
    let data = {
        #[cfg(target_os = "macos")]
        {
            let _tag = java_string(&tag);
            macos::weekdays()
        }
        #[cfg(target_os = "windows")]
        {
            java_string(&tag).and_then(|tag| windows::weekdays(&tag))
        }
        #[cfg(not(any(target_os = "macos", target_os = "windows")))]
        {
            let _ = &tag;
            Option::<StringArrayData>::None
        }
    };
    let result = match data {
        Some(data) => update_or_create_string_array(&thread, default_array, data).await?,
        None => default_array,
    };
    Ok(Some(result))
}

#[cfg(not(target_os = "linux"))]
#[intrinsic_method(
    "sun/util/locale/provider/HostLocaleProviderAdapterImpl.getZeroDigit(Ljava/lang/String;C)C",
    Any
)]
#[async_method]
pub async fn get_zero_digit<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let default_char = parameters.pop_int()?;
    let tag = parameters.pop()?;
    let value = {
        #[cfg(target_os = "macos")]
        {
            let _tag = java_string(&tag);
            macos::zero_digit()
        }
        #[cfg(target_os = "windows")]
        {
            java_string(&tag)
                .and_then(|tag| windows::locale_info_char(&tag, windows::LOCALE_SNATIVEDIGITS))
        }
        #[cfg(not(any(target_os = "macos", target_os = "windows")))]
        {
            let _ = &tag;
            Option::<i32>::None
        }
    }
    .unwrap_or(default_char);
    Ok(Some(Value::Int(value)))
}

#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/util/locale/provider/HostLocaleProviderAdapterImpl.getCalendarDataValue(Ljava/lang/String;I)I",
    Any
)]
#[async_method]
pub async fn get_calendar_data_value<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let r#type = parameters.pop_int()?;
    let tag = parameters.pop()?;
    let value = java_string(&tag).map_or(-1, |tag| windows::calendar_data_value(&tag, r#type));
    Ok(Some(Value::Int(value)))
}

#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/util/locale/provider/HostLocaleProviderAdapterImpl.getCalendarDisplayStrings(Ljava/lang/String;III)[Ljava/lang/String;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_calendar_display_strings_windows<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let style = parameters.pop_int()?;
    let field = parameters.pop_int()?;
    let calid = parameters.pop_int()?;
    let tag = parameters.pop()?;
    let data = java_string(&tag)
        .and_then(|tag| windows::calendar_display_strings(&tag, calid, field, style));
    match data {
        Some(data) => Ok(Some(new_string_array_from_data(&thread, data).await?)),
        None => Ok(Some(Value::Object(None))),
    }
}

#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/util/locale/provider/HostLocaleProviderAdapterImpl.getCalendarID(Ljava/lang/String;)I",
    Any
)]
#[async_method]
pub async fn get_calendar_id_windows<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let tag = parameters.pop()?;
    let value = java_string(&tag).map_or(0, |tag| windows::calendar_id(&tag));
    Ok(Some(Value::Int(value)))
}

#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/util/locale/provider/HostLocaleProviderAdapterImpl.getDateTimePattern(IILjava/lang/String;)Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_date_time_pattern<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let tag = parameters.pop()?;
    let time_style = parameters.pop_int()?;
    let date_style = parameters.pop_int()?;
    let pattern =
        java_string(&tag).and_then(|tag| windows::date_time_pattern(&tag, date_style, time_style));
    match pattern {
        Some(pattern) => Ok(Some(pattern.to_object(&thread).await?)),
        None => Ok(Some(Value::Object(None))),
    }
}

#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/util/locale/provider/HostLocaleProviderAdapterImpl.getNumberPattern(ILjava/lang/String;)Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_number_pattern<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let tag = parameters.pop()?;
    let number_style = parameters.pop_int()?;
    let Some(tag) = java_string(&tag) else {
        return Ok(Some(Value::Object(None)));
    };
    let pattern = windows::number_pattern(&tag, number_style);
    Ok(Some(pattern.to_object(&thread).await?))
}

#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/util/locale/provider/HostLocaleProviderAdapterImpl.initialize()Z",
    Any
)]
#[async_method]
pub async fn initialize<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::from(true)))
}

#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/util/locale/provider/HostLocaleProviderAdapterImpl.isNativeDigit(Ljava/lang/String;)Z",
    Any
)]
#[async_method]
pub async fn is_native_digit<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let tag = parameters.pop()?;
    let value = java_string(&tag).is_some_and(|tag| windows::is_native_digit(&tag));
    Ok(Some(Value::from(value)))
}

#[cfg(all(test, not(target_os = "linux"), not(target_family = "wasm")))]
mod tests {
    use super::*;

    #[cfg(target_os = "macos")]
    const DISPLAY_LANGUAGE_TYPE: MacDisplayNameKind = MacDisplayNameKind::Language;
    #[cfg(target_os = "windows")]
    const DISPLAY_LANGUAGE_TYPE: WindowsDisplayNameKind = WindowsDisplayNameKind::Language;

    async fn tag<T: Thread + 'static>(thread: &Arc<T>) -> Value {
        "en-US".to_object(thread).await.expect("tag")
    }

    async fn string_array<T: Thread + 'static>(thread: &Arc<T>, values: &[&str]) -> Value {
        let replacements = values
            .iter()
            .enumerate()
            .map(|(index, value)| (index, (*value).to_string()))
            .collect();
        new_string_array_from_data(
            thread,
            StringArrayData {
                len: values.len(),
                replacements,
            },
        )
        .await
        .expect("string array")
    }

    fn value(result: Result<Option<Value>>) -> Value {
        result.expect("result").expect("value")
    }

    fn default_object_parameters(tag: Value, default: Value) -> Parameters {
        Parameters::new(vec![tag, default])
    }

    fn default_char_parameters(tag: Value, default: char) -> Parameters {
        Parameters::new(vec![tag, Value::from(default)])
    }

    fn display_parameters(tag: Value, r#type: i32, value: Value) -> Parameters {
        Parameters::new(vec![tag, Value::Int(r#type), value])
    }

    fn assert_string_array(value: &Value, min_len: usize, min_strings: usize) {
        let (_class, elements) = value.as_class_vec_ref().expect("array");
        assert!(elements.len() >= min_len);
        let string_count = elements
            .iter()
            .filter(|value| matches!(value, Value::Object(Some(_))))
            .count();
        assert!(string_count >= min_strings);
    }

    fn assert_non_empty_string(value: &Value) {
        assert!(!value.as_string().expect("string").is_empty());
    }

    #[test]
    fn test_default_date_time_pattern() {
        assert_eq!("M/d/yy", default_date_time_pattern(3, -1));
        assert_eq!("h:mm a", default_date_time_pattern(-1, 3));
        assert_eq!("MMM d, yyyy h:mm:ss a", default_date_time_pattern(2, 2));
        assert_eq!("", default_date_time_pattern(-1, -1));
    }

    #[test]
    fn test_default_number_pattern() {
        assert_eq!("#,##0.###", default_number_pattern(0));
        assert_eq!(
            "\u{00a4}#,##0.00",
            default_number_pattern(NumberFormatKind::Currency.as_i32())
        );
        assert_eq!(
            "#,##0%",
            default_number_pattern(NumberFormatKind::Percent.as_i32())
        );
        assert_eq!(
            "#,##0",
            default_number_pattern(NumberFormatKind::Integer.as_i32())
        );
    }

    #[tokio::test]
    async fn test_get_am_pm_strings() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = value(
            get_am_pm_strings(
                thread.clone(),
                default_object_parameters(tag(&thread).await, Value::Object(None)),
            )
            .await,
        );
        assert_string_array(&result, 2, 2);
    }

    #[cfg(target_os = "macos")]
    #[tokio::test]
    async fn test_get_calendar_display_strings() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = value(
            get_calendar_display_strings(
                thread.clone(),
                Parameters::new(vec![
                    tag(&thread).await,
                    Value::Int(CalendarDisplayField::DayOfWeek.as_i32()),
                    Value::Int(0),
                ]),
            )
            .await,
        );
        assert_string_array(&result, 7, 7);
    }

    #[cfg(target_os = "macos")]
    #[tokio::test]
    async fn test_get_calendar_id() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            value(get_calendar_id(thread.clone(), Parameters::new(vec![tag(&thread).await])).await);
        assert_non_empty_string(&result);
    }

    #[cfg(target_os = "macos")]
    #[tokio::test]
    async fn test_get_calendar_int() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = value(
            get_calendar_int(
                thread.clone(),
                Parameters::new(vec![
                    tag(&thread).await,
                    Value::Int(MacCalendarDataKind::FirstDayOfWeek.as_i32()),
                ]),
            )
            .await,
        );
        assert!(result.as_i32().expect("int") > 0);
    }

    #[tokio::test]
    async fn test_get_currency_symbol() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let default = "USD".to_object(&thread).await.expect("string");
        let result = value(
            get_currency_symbol(
                thread.clone(),
                default_object_parameters(tag(&thread).await, default),
            )
            .await,
        );
        assert_non_empty_string(&result);
    }

    #[cfg(target_os = "macos")]
    #[tokio::test]
    async fn test_get_date_time_pattern_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = value(
            get_date_time_pattern_native(
                thread.clone(),
                Parameters::new(vec![Value::Int(3), Value::Int(-1), tag(&thread).await]),
            )
            .await,
        );
        assert_non_empty_string(&result);
    }

    #[tokio::test]
    async fn test_get_decimal_separator() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = value(
            get_decimal_separator(
                thread.clone(),
                default_char_parameters(tag(&thread).await, '.'),
            )
            .await,
        );
        assert_eq!(Value::from('.'), result);
    }

    #[tokio::test]
    async fn test_get_default_locale() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = value(get_default_locale(thread, Parameters::new(vec![Value::Int(0)])).await);
        assert!(!result.as_string().expect("string").is_empty());
    }

    #[tokio::test]
    async fn test_get_display_string() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let value_string = "en".to_object(&thread).await.expect("string");
        let result = value(
            get_display_string(
                thread.clone(),
                display_parameters(
                    tag(&thread).await,
                    DISPLAY_LANGUAGE_TYPE.as_i32(),
                    value_string,
                ),
            )
            .await,
        );
        assert_non_empty_string(&result);
    }

    #[tokio::test]
    async fn test_get_eras() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let default = string_array(&thread, &["BC", "AD"]).await;
        let result = value(
            get_eras(
                thread.clone(),
                default_object_parameters(tag(&thread).await, default),
            )
            .await,
        );
        assert_string_array(&result, 2, 2);
    }

    #[cfg(target_os = "macos")]
    #[tokio::test]
    async fn test_get_exponent_separator() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let default = "E".to_object(&thread).await.expect("string");
        let result = value(
            get_exponent_separator(
                thread.clone(),
                default_object_parameters(tag(&thread).await, default),
            )
            .await,
        );
        assert_non_empty_string(&result);
    }

    #[tokio::test]
    async fn test_get_grouping_separator() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = value(
            get_grouping_separator(
                thread.clone(),
                default_char_parameters(tag(&thread).await, ','),
            )
            .await,
        );
        assert_eq!(Value::from(','), result);
    }

    #[tokio::test]
    async fn test_get_infinity() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let default = "Infinity".to_object(&thread).await.expect("string");
        let result = value(
            get_infinity(
                thread.clone(),
                default_object_parameters(tag(&thread).await, default),
            )
            .await,
        );
        assert_non_empty_string(&result);
    }

    #[tokio::test]
    async fn test_get_international_currency_symbol() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let default = "USD".to_object(&thread).await.expect("string");
        let result = value(
            get_international_currency_symbol(
                thread.clone(),
                default_object_parameters(tag(&thread).await, default),
            )
            .await,
        );
        assert_non_empty_string(&result);
    }

    #[tokio::test]
    async fn test_get_minus_sign() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = value(
            get_minus_sign(
                thread.clone(),
                default_char_parameters(tag(&thread).await, '-'),
            )
            .await,
        );
        assert_eq!(Value::from('-'), result);
    }

    #[tokio::test]
    async fn test_get_monetary_decimal_separator() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = value(
            get_monetary_decimal_separator(
                thread.clone(),
                default_char_parameters(tag(&thread).await, '.'),
            )
            .await,
        );
        assert_eq!(Value::from('.'), result);
    }

    #[tokio::test]
    async fn test_get_months() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = value(
            get_months(
                thread.clone(),
                default_object_parameters(tag(&thread).await, Value::Object(None)),
            )
            .await,
        );
        assert_string_array(&result, 12, 12);
    }

    #[tokio::test]
    async fn test_get_nan() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let default = "NaN".to_object(&thread).await.expect("string");
        let result = value(
            get_nan(
                thread.clone(),
                default_object_parameters(tag(&thread).await, default),
            )
            .await,
        );
        assert_eq!("NaN", result.as_string().expect("string"));
    }

    #[cfg(target_os = "macos")]
    #[tokio::test]
    async fn test_get_number_pattern_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = value(
            get_number_pattern_native(
                thread.clone(),
                Parameters::new(vec![
                    Value::Int(NumberFormatKind::Currency.as_i32()),
                    tag(&thread).await,
                ]),
            )
            .await,
        );
        assert_non_empty_string(&result);
    }

    #[tokio::test]
    async fn test_get_per_mill() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = value(
            get_per_mill(
                thread.clone(),
                default_char_parameters(tag(&thread).await, '\u{2030}'),
            )
            .await,
        );
        assert!(matches!(result, Value::Int(_)));
    }

    #[tokio::test]
    async fn test_get_percent() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = value(
            get_percent(
                thread.clone(),
                default_char_parameters(tag(&thread).await, '%'),
            )
            .await,
        );
        assert_eq!(Value::from('%'), result);
    }

    #[tokio::test]
    async fn test_get_short_months() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = value(
            get_short_months(
                thread.clone(),
                default_object_parameters(tag(&thread).await, Value::Object(None)),
            )
            .await,
        );
        assert_string_array(&result, 12, 12);
    }

    #[tokio::test]
    async fn test_get_short_weekdays() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = value(
            get_short_weekdays(
                thread.clone(),
                default_object_parameters(tag(&thread).await, Value::Object(None)),
            )
            .await,
        );
        assert_string_array(&result, 7, 7);
    }

    #[cfg(target_os = "macos")]
    #[tokio::test]
    async fn test_get_time_zone_display_string() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let default = "America/Denver".to_object(&thread).await.expect("string");
        let result = value(
            get_time_zone_display_string(
                thread.clone(),
                display_parameters(
                    tag(&thread).await,
                    MacTimeZoneNameKind::ShortStandard.as_i32(),
                    default,
                ),
            )
            .await,
        );
        assert_non_empty_string(&result);
    }

    #[tokio::test]
    async fn test_get_weekdays() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = value(
            get_weekdays(
                thread.clone(),
                default_object_parameters(tag(&thread).await, Value::Object(None)),
            )
            .await,
        );
        assert_string_array(&result, 7, 7);
    }

    #[tokio::test]
    async fn test_get_zero_digit() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = value(
            get_zero_digit(
                thread.clone(),
                default_char_parameters(tag(&thread).await, '0'),
            )
            .await,
        );
        assert_eq!(Value::from('0'), result);
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_calendar_data_value() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_calendar_data_value(
            thread.clone(),
            Parameters::new(vec![
                tag(&thread).await,
                Value::Int(WindowsCalendarDataKind::FirstDayOfWeek.as_i32()),
            ]),
        )
        .await;
        assert!(matches!(value(result), Value::Int(_)));
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_calendar_display_strings_windows() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_calendar_display_strings_windows(
            thread.clone(),
            Parameters::new(vec![
                tag(&thread).await,
                Value::Int(0),
                Value::Int(CalendarDisplayField::DayOfWeek.as_i32()),
                Value::Int(WindowsCalendarStyleFlag::Short.mask()),
            ]),
        )
        .await;
        assert_string_array(&value(result), 7, 7);
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_calendar_id_windows() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            get_calendar_id_windows(thread.clone(), Parameters::new(vec![tag(&thread).await]))
                .await;
        assert!(matches!(value(result), Value::Int(_)));
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_date_time_pattern() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_date_time_pattern(
            thread.clone(),
            Parameters::new(vec![Value::Int(0), Value::Int(0), tag(&thread).await]),
        )
        .await;
        assert_non_empty_string(&value(result));
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_number_pattern() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_number_pattern(
            thread.clone(),
            Parameters::new(vec![Value::Int(0), tag(&thread).await]),
        )
        .await;
        assert_non_empty_string(&value(result));
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_initialize() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = initialize(thread, Parameters::default()).await;
        assert_eq!(Value::from(true), value(result));
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_is_native_digit() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            is_native_digit(thread.clone(), Parameters::new(vec![tag(&thread).await])).await;
        assert!(matches!(value(result), Value::Int(_)));
    }
}
