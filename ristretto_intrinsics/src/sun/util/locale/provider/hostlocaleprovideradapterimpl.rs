#[cfg(target_os = "windows")]
use ristretto_classfile::JAVA_11;
#[cfg(target_os = "macos")]
use ristretto_classfile::JAVA_17;
#[cfg(not(target_os = "linux"))]
use ristretto_classfile::VersionSpecification::Any;
#[cfg(any(target_os = "macos", target_os = "windows"))]
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
#[cfg(not(target_os = "linux"))]
use ristretto_classloader::Value;
#[cfg(not(target_os = "linux"))]
use ristretto_macros::async_method;
#[cfg(not(target_os = "linux"))]
use ristretto_macros::intrinsic_method;
#[cfg(target_os = "windows")]
use ristretto_types::JavaError;
#[cfg(not(target_os = "linux"))]
use ristretto_types::JavaObject;
#[cfg(not(target_os = "linux"))]
use ristretto_types::Thread;
#[cfg(not(target_os = "linux"))]
use ristretto_types::{Parameters, Result};
#[cfg(not(target_os = "linux"))]
use std::sync::Arc;

#[cfg(not(target_os = "linux"))]
#[intrinsic_method(
    "sun/util/locale/provider/HostLocaleProviderAdapterImpl.getAmPmStrings(Ljava/lang/String;[Ljava/lang/String;)[Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_am_pm_strings<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let default_array = parameters.pop()?;
    let _tag = parameters.pop()?;
    Ok(Some(default_array))
}

#[cfg(target_os = "macos")]
#[intrinsic_method(
    "sun/util/locale/provider/HostLocaleProviderAdapterImpl.getCalendarDisplayStrings(Ljava/lang/String;II)[Ljava/lang/String;",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn get_calendar_display_strings<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Object(None)))
}

#[cfg(target_os = "macos")]
#[intrinsic_method(
    "sun/util/locale/provider/HostLocaleProviderAdapterImpl.getCalendarID(Ljava/lang/String;)Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_calendar_id<T: Thread + 'static>(
    thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some("gregory".to_object(&thread).await?))
}

#[cfg(target_os = "macos")]
#[intrinsic_method(
    "sun/util/locale/provider/HostLocaleProviderAdapterImpl.getCalendarInt(Ljava/lang/String;I)I",
    Any
)]
#[async_method]
pub async fn get_calendar_int<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(1)))
}

#[cfg(not(target_os = "linux"))]
#[intrinsic_method(
    "sun/util/locale/provider/HostLocaleProviderAdapterImpl.getCurrencySymbol(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_currency_symbol<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let default_symbol = parameters.pop()?;
    Ok(Some(default_symbol))
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
    let _tag = parameters.pop()?;
    let _time_style = parameters.pop_int()?;
    let _date_style = parameters.pop_int()?;
    Ok(Some("yyyy-MM-dd HH:mm:ss".to_object(&thread).await?))
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
    Ok(Some(Value::Int(default_char)))
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
    let _cat = parameters.pop_int()?;
    let (language, country) = crate::properties::detect_default_locale();
    let locale = if country.is_empty() {
        language
    } else {
        format!("{language}_{country}")
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
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let value = parameters.pop()?;
    Ok(Some(value))
}

#[cfg(not(target_os = "linux"))]
#[intrinsic_method(
    "sun/util/locale/provider/HostLocaleProviderAdapterImpl.getEras(Ljava/lang/String;[Ljava/lang/String;)[Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_eras<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let default_array = parameters.pop()?;
    Ok(Some(default_array))
}

#[cfg(target_os = "macos")]
#[intrinsic_method(
    "sun/util/locale/provider/HostLocaleProviderAdapterImpl.getExponentSeparator(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_exponent_separator<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let default_separator = parameters.pop()?;
    Ok(Some(default_separator))
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
    Ok(Some(Value::Int(default_char)))
}

#[cfg(not(target_os = "linux"))]
#[intrinsic_method(
    "sun/util/locale/provider/HostLocaleProviderAdapterImpl.getInfinity(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_infinity<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let default_string = parameters.pop()?;
    Ok(Some(default_string))
}

#[cfg(not(target_os = "linux"))]
#[intrinsic_method(
    "sun/util/locale/provider/HostLocaleProviderAdapterImpl.getInternationalCurrencySymbol(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_international_currency_symbol<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let default_symbol = parameters.pop()?;
    Ok(Some(default_symbol))
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
    Ok(Some(Value::Int(default_char)))
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
    Ok(Some(Value::Int(default_char)))
}

#[cfg(not(target_os = "linux"))]
#[intrinsic_method(
    "sun/util/locale/provider/HostLocaleProviderAdapterImpl.getMonths(Ljava/lang/String;[Ljava/lang/String;)[Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_months<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let default_array = parameters.pop()?;
    Ok(Some(default_array))
}

#[cfg(not(target_os = "linux"))]
#[intrinsic_method(
    "sun/util/locale/provider/HostLocaleProviderAdapterImpl.getNaN(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_nan<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let default_string = parameters.pop()?;
    Ok(Some(default_string))
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
    let _tag = parameters.pop()?;
    let style = parameters.pop_int()?;
    let pattern = match style {
        1 => "¤ #,##0.00", // CURRENCY
        2 => "#,##0%",     // PERCENT
        3 => "#,##0",      // INTEGER
        _ => "#,##0.###",  // NUMBER (0) and other
    };
    Ok(Some(pattern.to_object(&thread).await?))
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
    Ok(Some(Value::Int(default_char)))
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
    Ok(Some(Value::Int(default_char)))
}

#[cfg(not(target_os = "linux"))]
#[intrinsic_method(
    "sun/util/locale/provider/HostLocaleProviderAdapterImpl.getShortMonths(Ljava/lang/String;[Ljava/lang/String;)[Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_short_months<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let default_array = parameters.pop()?;
    Ok(Some(default_array))
}

#[cfg(not(target_os = "linux"))]
#[intrinsic_method(
    "sun/util/locale/provider/HostLocaleProviderAdapterImpl.getShortWeekdays(Ljava/lang/String;[Ljava/lang/String;)[Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_short_weekdays<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let default_array = parameters.pop()?;
    Ok(Some(default_array))
}

#[cfg(target_os = "macos")]
#[intrinsic_method(
    "sun/util/locale/provider/HostLocaleProviderAdapterImpl.getTimeZoneDisplayString(Ljava/lang/String;ILjava/lang/String;)Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_time_zone_display_string<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let value = parameters.pop()?;
    Ok(Some(value))
}

#[cfg(not(target_os = "linux"))]
#[intrinsic_method(
    "sun/util/locale/provider/HostLocaleProviderAdapterImpl.getWeekdays(Ljava/lang/String;[Ljava/lang/String;)[Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_weekdays<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let default_array = parameters.pop()?;
    Ok(Some(default_array))
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
    Ok(Some(Value::Int(default_char)))
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
    let _type_ = parameters.pop_int()?;
    let _jlangtag = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun/util/locale/provider/HostLocaleProviderAdapterImpl.getCalendarDataValue(Ljava/lang/String;I)I".to_string()).into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/util/locale/provider/HostLocaleProviderAdapterImpl.getCalendarDisplayStrings(Ljava/lang/String;III)[Ljava/lang/String;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_calendar_display_strings_windows_ge_v11_v1<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _style = parameters.pop_int()?;
    let _field = parameters.pop_int()?;
    let _calid = parameters.pop_int()?;
    let _jlangtag = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun/util/locale/provider/HostLocaleProviderAdapterImpl.getCalendarDisplayStrings(Ljava/lang/String;III)[Ljava/lang/String;".to_string()).into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/util/locale/provider/HostLocaleProviderAdapterImpl.getCalendarID(Ljava/lang/String;)I",
    Any
)]
#[async_method]
pub async fn get_calendar_id_windows_v1<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _jlangtag = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/util/locale/provider/HostLocaleProviderAdapterImpl.getCalendarID(Ljava/lang/String;)I"
            .to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/util/locale/provider/HostLocaleProviderAdapterImpl.getDateTimePattern(IILjava/lang/String;)Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_date_time_pattern<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _jlangtag = parameters.pop_reference()?;
    let _time_style = parameters.pop_int()?;
    let _date_style = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError("sun/util/locale/provider/HostLocaleProviderAdapterImpl.getDateTimePattern(IILjava/lang/String;)Ljava/lang/String;".to_string()).into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/util/locale/provider/HostLocaleProviderAdapterImpl.getNumberPattern(ILjava/lang/String;)Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_number_pattern<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _jlangtag = parameters.pop_reference()?;
    let _number_style = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError("sun/util/locale/provider/HostLocaleProviderAdapterImpl.getNumberPattern(ILjava/lang/String;)Ljava/lang/String;".to_string()).into())
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
    Err(JavaError::UnsatisfiedLinkError(
        "sun/util/locale/provider/HostLocaleProviderAdapterImpl.initialize()Z".to_string(),
    )
    .into())
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
    let _jlangtag = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/util/locale/provider/HostLocaleProviderAdapterImpl.isNativeDigit(Ljava/lang/String;)Z"
            .to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/util/locale/provider/HostLocaleProviderAdapterImpl.getCalendarDataValue(Ljava/lang/String;I)I",
    Any
)]
#[async_method]
pub async fn get_calendar_data_value_windows<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _type_ = parameters.pop_int()?;
    let _jlangtag = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun/util/locale/provider/HostLocaleProviderAdapterImpl.getCalendarDataValue(Ljava/lang/String;I)I".to_string()).into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/util/locale/provider/HostLocaleProviderAdapterImpl.getCalendarDisplayStrings(Ljava/lang/String;III)[Ljava/lang/String;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_calendar_display_strings_windows_ge_v11_v2<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _style = parameters.pop_int()?;
    let _field = parameters.pop_int()?;
    let _calid = parameters.pop_int()?;
    let _jlangtag = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun/util/locale/provider/HostLocaleProviderAdapterImpl.getCalendarDisplayStrings(Ljava/lang/String;III)[Ljava/lang/String;".to_string()).into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/util/locale/provider/HostLocaleProviderAdapterImpl.getCalendarID(Ljava/lang/String;)I",
    Any
)]
#[async_method]
pub async fn get_calendar_id_windows_v2<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _jlangtag = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/util/locale/provider/HostLocaleProviderAdapterImpl.getCalendarID(Ljava/lang/String;)I"
            .to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/util/locale/provider/HostLocaleProviderAdapterImpl.getDateTimePattern(IILjava/lang/String;)Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_date_time_pattern_windows<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _jlangtag = parameters.pop_reference()?;
    let _time_style = parameters.pop_int()?;
    let _date_style = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError("sun/util/locale/provider/HostLocaleProviderAdapterImpl.getDateTimePattern(IILjava/lang/String;)Ljava/lang/String;".to_string()).into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/util/locale/provider/HostLocaleProviderAdapterImpl.getNumberPattern(ILjava/lang/String;)Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_number_pattern_windows<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _jlangtag = parameters.pop_reference()?;
    let _number_style = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError("sun/util/locale/provider/HostLocaleProviderAdapterImpl.getNumberPattern(ILjava/lang/String;)Ljava/lang/String;".to_string()).into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/util/locale/provider/HostLocaleProviderAdapterImpl.initialize()Z",
    Any
)]
#[async_method]
pub async fn initialize_windows<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/util/locale/provider/HostLocaleProviderAdapterImpl.initialize()Z".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/util/locale/provider/HostLocaleProviderAdapterImpl.isNativeDigit(Ljava/lang/String;)Z",
    Any
)]
#[async_method]
pub async fn is_native_digit_windows<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _jlangtag = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/util/locale/provider/HostLocaleProviderAdapterImpl.isNativeDigit(Ljava/lang/String;)Z"
            .to_string(),
    )
    .into())
}
#[cfg(all(test, not(target_os = "linux")))]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_am_pm_strings() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_am_pm_strings(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[cfg(target_os = "macos")]
    #[tokio::test]
    async fn test_get_calendar_display_strings() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_calendar_display_strings(thread, Parameters::default()).await;
        assert!(result.is_ok());
    }

    #[cfg(target_os = "macos")]
    #[tokio::test]
    async fn test_get_calendar_id() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_calendar_id(thread, Parameters::default()).await;
        assert!(result.is_ok());
    }

    #[cfg(target_os = "macos")]
    #[tokio::test]
    async fn test_get_calendar_int() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_calendar_int(thread, Parameters::default()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_currency_symbol() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_currency_symbol(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[cfg(target_os = "macos")]
    #[tokio::test]
    async fn test_get_date_time_pattern_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_date_time_pattern_native(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_decimal_separator() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_decimal_separator(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_default_locale() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_default_locale(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_display_string() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_display_string(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_eras() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_eras(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[cfg(target_os = "macos")]
    #[tokio::test]
    async fn test_get_exponent_separator() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_exponent_separator(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_grouping_separator() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_grouping_separator(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_infinity() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_infinity(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_international_currency_symbol() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_international_currency_symbol(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_minus_sign() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_minus_sign(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_monetary_decimal_separator() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_monetary_decimal_separator(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_months() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_months(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_nan() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_nan(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[cfg(target_os = "macos")]
    #[tokio::test]
    async fn test_get_number_pattern_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_number_pattern_native(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_per_mill() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_per_mill(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_percent() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_percent(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_short_months() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_short_months(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_short_weekdays() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_short_weekdays(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[cfg(target_os = "macos")]
    #[tokio::test]
    async fn test_get_time_zone_display_string() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_time_zone_display_string(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_weekdays() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_weekdays(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_zero_digit() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_zero_digit(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_calendar_data_value() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_calendar_data_value(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun/util/locale/provider/HostLocaleProviderAdapterImpl.getCalendarDataValue(Ljava/lang/String;I)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_calendar_display_strings_windows_ge_v11_v1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_calendar_display_strings_windows_ge_v11_v1(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/util/locale/provider/HostLocaleProviderAdapterImpl.getCalendarDisplayStrings(Ljava/lang/String;III)[Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_calendar_id_windows_v1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            get_calendar_id_windows_v1(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/util/locale/provider/HostLocaleProviderAdapterImpl.getCalendarID(Ljava/lang/String;)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_date_time_pattern() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_date_time_pattern(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Int(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun/util/locale/provider/HostLocaleProviderAdapterImpl.getDateTimePattern(IILjava/lang/String;)Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_number_pattern() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_number_pattern(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun/util/locale/provider/HostLocaleProviderAdapterImpl.getNumberPattern(ILjava/lang/String;)Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_initialize() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = initialize(thread, Parameters::default()).await;
        assert_eq!(
            "sun/util/locale/provider/HostLocaleProviderAdapterImpl.initialize()Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_is_native_digit() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = is_native_digit(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/util/locale/provider/HostLocaleProviderAdapterImpl.isNativeDigit(Ljava/lang/String;)Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_calendar_data_value_windows() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_calendar_data_value_windows(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun/util/locale/provider/HostLocaleProviderAdapterImpl.getCalendarDataValue(Ljava/lang/String;I)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_calendar_display_strings_windows_ge_v11_v2() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_calendar_display_strings_windows_ge_v11_v2(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/util/locale/provider/HostLocaleProviderAdapterImpl.getCalendarDisplayStrings(Ljava/lang/String;III)[Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_calendar_id_windows_v2() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            get_calendar_id_windows_v2(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/util/locale/provider/HostLocaleProviderAdapterImpl.getCalendarID(Ljava/lang/String;)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_date_time_pattern_windows() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_date_time_pattern_windows(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Int(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun/util/locale/provider/HostLocaleProviderAdapterImpl.getDateTimePattern(IILjava/lang/String;)Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_number_pattern_windows() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_number_pattern_windows(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun/util/locale/provider/HostLocaleProviderAdapterImpl.getNumberPattern(ILjava/lang/String;)Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_initialize_windows() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = initialize_windows(thread, Parameters::default()).await;
        assert_eq!(
            "sun/util/locale/provider/HostLocaleProviderAdapterImpl.initialize()Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_is_native_digit_windows() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            is_native_digit_windows(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/util/locale/provider/HostLocaleProviderAdapterImpl.isNativeDigit(Ljava/lang/String;)Z",
            result.unwrap_err().to_string()
        );
    }
}
