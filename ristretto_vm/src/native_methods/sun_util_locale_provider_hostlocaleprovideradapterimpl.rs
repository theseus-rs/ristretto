use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_17: Version = Version::Java17 { minor: 0 };

/// Register all native methods for `sun.util.locale.provider.HostLocaleProviderAdapterImpl`.
#[expect(clippy::too_many_lines)]
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/util/locale/provider/HostLocaleProviderAdapterImpl";
    let java_version = registry.java_version();

    if java_version >= &JAVA_17 {
        registry.register(
            class_name,
            "getCalendarDisplayStrings",
            "(Ljava/lang/String;II)[Ljava/lang/String;",
            get_calendar_display_strings,
        );
    }

    registry.register(
        class_name,
        "getAmPmStrings",
        "(Ljava/lang/String;[Ljava/lang/String;)[Ljava/lang/String;",
        get_am_pm_strings,
    );
    registry.register(
        class_name,
        "getCalendarID",
        "(Ljava/lang/String;)Ljava/lang/String;",
        get_calendar_id,
    );
    registry.register(
        class_name,
        "getCalendarInt",
        "(Ljava/lang/String;I)I",
        get_calendar_int,
    );
    registry.register(
        class_name,
        "getCurrencySymbol",
        "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
        get_currency_symbol,
    );
    registry.register(
        class_name,
        "getDateTimePatternNative",
        "(IILjava/lang/String;)Ljava/lang/String;",
        get_date_time_pattern_native,
    );
    registry.register(
        class_name,
        "getDecimalSeparator",
        "(Ljava/lang/String;C)C",
        get_decimal_separator,
    );
    registry.register(
        class_name,
        "getDefaultLocale",
        "(I)Ljava/lang/String;",
        get_default_locale,
    );
    registry.register(
        class_name,
        "getDisplayString",
        "(Ljava/lang/String;ILjava/lang/String;)Ljava/lang/String;",
        get_display_string,
    );
    registry.register(
        class_name,
        "getEras",
        "(Ljava/lang/String;[Ljava/lang/String;)[Ljava/lang/String;",
        get_eras,
    );
    registry.register(
        class_name,
        "getExponentSeparator",
        "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
        get_exponent_separator,
    );
    registry.register(
        class_name,
        "getGroupingSeparator",
        "(Ljava/lang/String;C)C",
        get_grouping_separator,
    );
    registry.register(
        class_name,
        "getInfinity",
        "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
        get_infinity,
    );
    registry.register(
        class_name,
        "getInternationalCurrencySymbol",
        "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
        get_international_currency_symbol,
    );
    registry.register(
        class_name,
        "getMinusSign",
        "(Ljava/lang/String;C)C",
        get_minus_sign,
    );
    registry.register(
        class_name,
        "getMonetaryDecimalSeparator",
        "(Ljava/lang/String;C)C",
        get_monetary_decimal_separator,
    );
    registry.register(
        class_name,
        "getMonths",
        "(Ljava/lang/String;[Ljava/lang/String;)[Ljava/lang/String;",
        get_months,
    );
    registry.register(
        class_name,
        "getNaN",
        "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
        get_na_n,
    );
    registry.register(
        class_name,
        "getNumberPatternNative",
        "(ILjava/lang/String;)Ljava/lang/String;",
        get_number_pattern_native,
    );
    registry.register(
        class_name,
        "getPerMill",
        "(Ljava/lang/String;C)C",
        get_per_mill,
    );
    registry.register(
        class_name,
        "getPercent",
        "(Ljava/lang/String;C)C",
        get_percent,
    );
    registry.register(
        class_name,
        "getShortMonths",
        "(Ljava/lang/String;[Ljava/lang/String;)[Ljava/lang/String;",
        get_short_months,
    );
    registry.register(
        class_name,
        "getShortWeekdays",
        "(Ljava/lang/String;[Ljava/lang/String;)[Ljava/lang/String;",
        get_short_weekdays,
    );
    registry.register(
        class_name,
        "getTimeZoneDisplayString",
        "(Ljava/lang/String;ILjava/lang/String;)Ljava/lang/String;",
        get_time_zone_display_string,
    );
    registry.register(
        class_name,
        "getWeekdays",
        "(Ljava/lang/String;[Ljava/lang/String;)[Ljava/lang/String;",
        get_weekdays,
    );
    registry.register(
        class_name,
        "getZeroDigit",
        "(Ljava/lang/String;C)C",
        get_zero_digit,
    );
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_am_pm_strings(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_calendar_display_strings(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_calendar_id(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_calendar_int(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_currency_symbol(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_date_time_pattern_native(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_decimal_separator(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_default_locale(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_display_string(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_eras(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_exponent_separator(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_grouping_separator(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_infinity(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_international_currency_symbol(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_minus_sign(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_monetary_decimal_separator(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_months(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_na_n(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_number_pattern_native(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_per_mill(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_percent(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_short_months(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_short_weekdays(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_time_zone_display_string(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_weekdays(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_zero_digit(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
