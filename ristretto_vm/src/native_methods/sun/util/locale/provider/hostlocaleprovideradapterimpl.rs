use crate::native_methods::registry::{MethodRegistry, JAVA_17};
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/util/locale/provider/HostLocaleProviderAdapterImpl";

/// Register all native methods for `sun.util.locale.provider.HostLocaleProviderAdapterImpl`.
#[expect(clippy::too_many_lines)]
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() >= JAVA_17 {
        registry.register(
            CLASS_NAME,
            "getCalendarDisplayStrings",
            "(Ljava/lang/String;II)[Ljava/lang/String;",
            get_calendar_display_strings,
        );
    }

    registry.register(
        CLASS_NAME,
        "getAmPmStrings",
        "(Ljava/lang/String;[Ljava/lang/String;)[Ljava/lang/String;",
        get_am_pm_strings,
    );
    registry.register(
        CLASS_NAME,
        "getCalendarID",
        "(Ljava/lang/String;)Ljava/lang/String;",
        get_calendar_id,
    );
    registry.register(
        CLASS_NAME,
        "getCalendarInt",
        "(Ljava/lang/String;I)I",
        get_calendar_int,
    );
    registry.register(
        CLASS_NAME,
        "getCurrencySymbol",
        "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
        get_currency_symbol,
    );
    registry.register(
        CLASS_NAME,
        "getDateTimePatternNative",
        "(IILjava/lang/String;)Ljava/lang/String;",
        get_date_time_pattern_native,
    );
    registry.register(
        CLASS_NAME,
        "getDecimalSeparator",
        "(Ljava/lang/String;C)C",
        get_decimal_separator,
    );
    registry.register(
        CLASS_NAME,
        "getDefaultLocale",
        "(I)Ljava/lang/String;",
        get_default_locale,
    );
    registry.register(
        CLASS_NAME,
        "getDisplayString",
        "(Ljava/lang/String;ILjava/lang/String;)Ljava/lang/String;",
        get_display_string,
    );
    registry.register(
        CLASS_NAME,
        "getEras",
        "(Ljava/lang/String;[Ljava/lang/String;)[Ljava/lang/String;",
        get_eras,
    );
    registry.register(
        CLASS_NAME,
        "getExponentSeparator",
        "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
        get_exponent_separator,
    );
    registry.register(
        CLASS_NAME,
        "getGroupingSeparator",
        "(Ljava/lang/String;C)C",
        get_grouping_separator,
    );
    registry.register(
        CLASS_NAME,
        "getInfinity",
        "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
        get_infinity,
    );
    registry.register(
        CLASS_NAME,
        "getInternationalCurrencySymbol",
        "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
        get_international_currency_symbol,
    );
    registry.register(
        CLASS_NAME,
        "getMinusSign",
        "(Ljava/lang/String;C)C",
        get_minus_sign,
    );
    registry.register(
        CLASS_NAME,
        "getMonetaryDecimalSeparator",
        "(Ljava/lang/String;C)C",
        get_monetary_decimal_separator,
    );
    registry.register(
        CLASS_NAME,
        "getMonths",
        "(Ljava/lang/String;[Ljava/lang/String;)[Ljava/lang/String;",
        get_months,
    );
    registry.register(
        CLASS_NAME,
        "getNaN",
        "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
        get_nan,
    );
    registry.register(
        CLASS_NAME,
        "getNumberPatternNative",
        "(ILjava/lang/String;)Ljava/lang/String;",
        get_number_pattern_native,
    );
    registry.register(
        CLASS_NAME,
        "getPerMill",
        "(Ljava/lang/String;C)C",
        get_per_mill,
    );
    registry.register(
        CLASS_NAME,
        "getPercent",
        "(Ljava/lang/String;C)C",
        get_percent,
    );
    registry.register(
        CLASS_NAME,
        "getShortMonths",
        "(Ljava/lang/String;[Ljava/lang/String;)[Ljava/lang/String;",
        get_short_months,
    );
    registry.register(
        CLASS_NAME,
        "getShortWeekdays",
        "(Ljava/lang/String;[Ljava/lang/String;)[Ljava/lang/String;",
        get_short_weekdays,
    );
    registry.register(
        CLASS_NAME,
        "getTimeZoneDisplayString",
        "(Ljava/lang/String;ILjava/lang/String;)Ljava/lang/String;",
        get_time_zone_display_string,
    );
    registry.register(
        CLASS_NAME,
        "getWeekdays",
        "(Ljava/lang/String;[Ljava/lang/String;)[Ljava/lang/String;",
        get_weekdays,
    );
    registry.register(
        CLASS_NAME,
        "getZeroDigit",
        "(Ljava/lang/String;C)C",
        get_zero_digit,
    );
}

#[async_recursion(?Send)]
async fn get_am_pm_strings(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.util.locale.provider.HostLocaleProviderAdapterImpl.getAmPmStrings(Ljava/lang/String;[Ljava/lang/String;)[Ljava/lang/String")
}

#[async_recursion(?Send)]
async fn get_calendar_display_strings(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.util.locale.provider.HostLocaleProviderAdapterImpl.getCalendarDisplayStrings(Ljava/lang/String;II)[Ljava/lang/String")
}

#[async_recursion(?Send)]
async fn get_calendar_id(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.util.locale.provider.HostLocaleProviderAdapterImpl.getCalendarID(Ljava/lang/String;)Ljava/lang/String")
}

#[async_recursion(?Send)]
async fn get_calendar_int(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.util.locale.provider.HostLocaleProviderAdapterImpl.getCalendarInt(Ljava/lang/String;I)I")
}

#[async_recursion(?Send)]
async fn get_currency_symbol(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.util.locale.provider.HostLocaleProviderAdapterImpl.getCurrencySymbol(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String")
}

#[async_recursion(?Send)]
async fn get_date_time_pattern_native(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.util.locale.provider.HostLocaleProviderAdapterImpl.getDateTimePatternNative(IILjava/lang/String;)Ljava/lang/String")
}

#[async_recursion(?Send)]
async fn get_decimal_separator(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.util.locale.provider.HostLocaleProviderAdapterImpl.getDecimalSeparator(Ljava/lang/String;C)C")
}

#[async_recursion(?Send)]
async fn get_default_locale(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.util.locale.provider.HostLocaleProviderAdapterImpl.getDefaultLocale(I)Ljava/lang/String")
}

#[async_recursion(?Send)]
async fn get_display_string(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.util.locale.provider.HostLocaleProviderAdapterImpl.getDisplayString(Ljava/lang/String;ILjava/lang/String;)Ljava/lang/String")
}

#[async_recursion(?Send)]
async fn get_eras(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.util.locale.provider.HostLocaleProviderAdapterImpl.getEras(Ljava/lang/String;[Ljava/lang/String;)[Ljava/lang/String")
}

#[async_recursion(?Send)]
async fn get_exponent_separator(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.util.locale.provider.HostLocaleProviderAdapterImpl.getExponentSeparator(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String")
}

#[async_recursion(?Send)]
async fn get_grouping_separator(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.util.locale.provider.HostLocaleProviderAdapterImpl.getGroupingSeparator(Ljava/lang/String;C)C")
}

#[async_recursion(?Send)]
async fn get_infinity(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.util.locale.provider.HostLocaleProviderAdapterImpl.getInfinity(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String")
}

#[async_recursion(?Send)]
async fn get_international_currency_symbol(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.util.locale.provider.HostLocaleProviderAdapterImpl.getInternationalCurrencySymbol(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String")
}

#[async_recursion(?Send)]
async fn get_minus_sign(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "sun.util.locale.provider.HostLocaleProviderAdapterImpl.getMinusSign(Ljava/lang/String;C)C"
    )
}

#[async_recursion(?Send)]
async fn get_monetary_decimal_separator(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.util.locale.provider.HostLocaleProviderAdapterImpl.getMonetaryDecimalSeparator(Ljava/lang/String;C)C")
}

#[async_recursion(?Send)]
async fn get_months(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.util.locale.provider.HostLocaleProviderAdapterImpl.getMonths(Ljava/lang/String;[Ljava/lang/String;)[Ljava/lang/String")
}

#[async_recursion(?Send)]
async fn get_nan(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.util.locale.provider.HostLocaleProviderAdapterImpl.getNaN(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String")
}

#[async_recursion(?Send)]
async fn get_number_pattern_native(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.util.locale.provider.HostLocaleProviderAdapterImpl.getNumberPatternNative(ILjava/lang/String;)Ljava/lang/String")
}

#[async_recursion(?Send)]
async fn get_per_mill(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.util.locale.provider.HostLocaleProviderAdapterImpl.getPerMill(Ljava/lang/String;C)C")
}

#[async_recursion(?Send)]
async fn get_percent(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.util.locale.provider.HostLocaleProviderAdapterImpl.getPercent(Ljava/lang/String;C)C")
}

#[async_recursion(?Send)]
async fn get_short_months(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.util.locale.provider.HostLocaleProviderAdapterImpl.getShortMonths(Ljava/lang/String;[Ljava/lang/String;)[Ljava/lang/String")
}

#[async_recursion(?Send)]
async fn get_short_weekdays(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.util.locale.provider.HostLocaleProviderAdapterImpl.getShortWeekdays(Ljava/lang/String;[Ljava/lang/String;)[Ljava/lang/String")
}

#[async_recursion(?Send)]
async fn get_time_zone_display_string(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.util.locale.provider.HostLocaleProviderAdapterImpl.getTimeZoneDisplayString(Ljava/lang/String;ILjava/lang/String;)Ljava/lang/String")
}

#[async_recursion(?Send)]
async fn get_weekdays(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.util.locale.provider.HostLocaleProviderAdapterImpl.getWeekdays(Ljava/lang/String;[Ljava/lang/String;)[Ljava/lang/String")
}

#[async_recursion(?Send)]
async fn get_zero_digit(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "sun.util.locale.provider.HostLocaleProviderAdapterImpl.getZeroDigit(Ljava/lang/String;C)C"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.util.locale.provider.HostLocaleProviderAdapterImpl.getAmPmStrings(Ljava/lang/String;[Ljava/lang/String;)[Ljava/lang/String"
    )]
    async fn test_get_am_pm_strings() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_am_pm_strings(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.util.locale.provider.HostLocaleProviderAdapterImpl.getCalendarDisplayStrings(Ljava/lang/String;II)[Ljava/lang/String"
    )]
    async fn test_get_calendar_display_strings() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_calendar_display_strings(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.util.locale.provider.HostLocaleProviderAdapterImpl.getCalendarID(Ljava/lang/String;)Ljava/lang/String"
    )]
    async fn test_get_calendar_id() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_calendar_id(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.util.locale.provider.HostLocaleProviderAdapterImpl.getCalendarInt(Ljava/lang/String;I)I"
    )]
    async fn test_get_calendar_int() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_calendar_int(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.util.locale.provider.HostLocaleProviderAdapterImpl.getCurrencySymbol(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String"
    )]
    async fn test_get_currency_symbol() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_currency_symbol(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.util.locale.provider.HostLocaleProviderAdapterImpl.getDateTimePatternNative(IILjava/lang/String;)Ljava/lang/String"
    )]
    async fn test_get_date_time_pattern_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_date_time_pattern_native(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.util.locale.provider.HostLocaleProviderAdapterImpl.getDecimalSeparator(Ljava/lang/String;C)C"
    )]
    async fn test_get_decimal_separator() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_decimal_separator(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.util.locale.provider.HostLocaleProviderAdapterImpl.getDefaultLocale(I)Ljava/lang/String"
    )]
    async fn test_get_default_locale() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_default_locale(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.util.locale.provider.HostLocaleProviderAdapterImpl.getDisplayString(Ljava/lang/String;ILjava/lang/String;)Ljava/lang/String"
    )]
    async fn test_get_display_string() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_display_string(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.util.locale.provider.HostLocaleProviderAdapterImpl.getEras(Ljava/lang/String;[Ljava/lang/String;)[Ljava/lang/String"
    )]
    async fn test_get_eras() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_eras(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.util.locale.provider.HostLocaleProviderAdapterImpl.getExponentSeparator(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String"
    )]
    async fn test_get_exponent_separator() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_exponent_separator(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.util.locale.provider.HostLocaleProviderAdapterImpl.getGroupingSeparator(Ljava/lang/String;C)C"
    )]
    async fn test_get_grouping_separator() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_grouping_separator(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.util.locale.provider.HostLocaleProviderAdapterImpl.getInfinity(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String"
    )]
    async fn test_get_infinity() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_infinity(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.util.locale.provider.HostLocaleProviderAdapterImpl.getInternationalCurrencySymbol(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String"
    )]
    async fn test_get_international_currency_symbol() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_international_currency_symbol(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.util.locale.provider.HostLocaleProviderAdapterImpl.getMinusSign(Ljava/lang/String;C)C"
    )]
    async fn test_get_minus_sign() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_minus_sign(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.util.locale.provider.HostLocaleProviderAdapterImpl.getMonetaryDecimalSeparator(Ljava/lang/String;C)C"
    )]
    async fn test_get_monetary_decimal_separator() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_monetary_decimal_separator(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.util.locale.provider.HostLocaleProviderAdapterImpl.getMonths(Ljava/lang/String;[Ljava/lang/String;)[Ljava/lang/String"
    )]
    async fn test_get_months() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_months(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.util.locale.provider.HostLocaleProviderAdapterImpl.getNaN(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String"
    )]
    async fn test_get_nan() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_nan(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.util.locale.provider.HostLocaleProviderAdapterImpl.getNumberPatternNative(ILjava/lang/String;)Ljava/lang/String"
    )]
    async fn test_get_number_pattern_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_number_pattern_native(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.util.locale.provider.HostLocaleProviderAdapterImpl.getPerMill(Ljava/lang/String;C)C"
    )]
    async fn test_get_per_mill() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_per_mill(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.util.locale.provider.HostLocaleProviderAdapterImpl.getPercent(Ljava/lang/String;C)C"
    )]
    async fn test_get_percent() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_percent(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.util.locale.provider.HostLocaleProviderAdapterImpl.getShortMonths(Ljava/lang/String;[Ljava/lang/String;)[Ljava/lang/String"
    )]
    async fn test_get_short_months() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_short_months(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.util.locale.provider.HostLocaleProviderAdapterImpl.getShortWeekdays(Ljava/lang/String;[Ljava/lang/String;)[Ljava/lang/String"
    )]
    async fn test_get_short_weekdays() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_short_weekdays(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.util.locale.provider.HostLocaleProviderAdapterImpl.getTimeZoneDisplayString(Ljava/lang/String;ILjava/lang/String;)Ljava/lang/String"
    )]
    async fn test_get_time_zone_display_string() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_time_zone_display_string(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.util.locale.provider.HostLocaleProviderAdapterImpl.getWeekdays(Ljava/lang/String;[Ljava/lang/String;)[Ljava/lang/String"
    )]
    async fn test_get_weekdays() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_weekdays(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.util.locale.provider.HostLocaleProviderAdapterImpl.getZeroDigit(Ljava/lang/String;C)C"
    )]
    async fn test_get_zero_digit() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_zero_digit(thread, Parameters::default()).await;
    }
}
