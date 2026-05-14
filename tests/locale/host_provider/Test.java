import java.text.DateFormatSymbols;
import java.time.chrono.IsoChronology;
import java.time.format.DateTimeFormatterBuilder;
import java.time.format.FormatStyle;
import java.util.Calendar;
import java.util.Locale;
import java.util.TimeZone;

public class Test {
    public static void main(String[] args) {
        System.setProperty("java.locale.providers", "HOST,CLDR");
        Locale.setDefault(Locale.US);
        TimeZone.setDefault(TimeZone.getTimeZone("UTC"));

        boolean supportedPlatform = isHostLocalePlatform();
        System.out.println("=== Host Locale Provider Tests ===");
        System.out.println("Host locale platform: " + supportedPlatform);
        if (!supportedPlatform) {
            return;
        }

        testDateFormatSymbols();
        testCalendarData();
        testDateTimeFormatting();
        testLocaleDisplayNames();
        testTimeZoneNames();
        System.out.println("=== Host Locale Provider Tests Complete ===");
    }

    private static boolean isHostLocalePlatform() {
        String osName = System.getProperty("os.name", "");
        String normalized = osName.toLowerCase(Locale.ROOT);
        return normalized.contains("mac") || normalized.contains("windows");
    }

    private static void testDateFormatSymbols() {
        DateFormatSymbols symbols = DateFormatSymbols.getInstance(Locale.US);
        String[] months = symbols.getMonths();
        String[] shortMonths = symbols.getShortMonths();
        String[] weekdays = symbols.getWeekdays();
        String[] shortWeekdays = symbols.getShortWeekdays();
        String[] eras = symbols.getEras();
        String[] amPm = symbols.getAmPmStrings();

        System.out.println("Date symbols months valid: " + (months.length >= 12 && months[0].length() > 0));
        System.out.println("Date symbols short months valid: " + (shortMonths.length >= 12 && shortMonths[0].length() > 0));
        System.out.println("Date symbols weekdays valid: " + (weekdays.length >= 8 && weekdays[1].length() > 0));
        System.out.println("Date symbols short weekdays valid: " + (shortWeekdays.length >= 8 && shortWeekdays[1].length() > 0));
        System.out.println("Date symbols eras valid: " + (eras.length >= 2 && eras[0].length() > 0));
        System.out.println("Date symbols ampm valid: " + (amPm.length == 2 && amPm[0].length() > 0));
    }

    private static void testCalendarData() {
        Calendar calendar = Calendar.getInstance(Locale.US);
        int firstDay = calendar.getFirstDayOfWeek();
        int minimalDays = calendar.getMinimalDaysInFirstWeek();

        System.out.println("Calendar first day valid: " + (firstDay >= Calendar.SUNDAY && firstDay <= Calendar.SATURDAY));
        System.out.println("Calendar minimal days valid: " + (minimalDays >= 1 && minimalDays <= 7));
        System.out.println("Calendar type valid: " + (calendar.getCalendarType().length() > 0));
    }

    private static void testDateTimeFormatting() {
        String datePattern = DateTimeFormatterBuilder.getLocalizedDateTimePattern(
                FormatStyle.SHORT, null, IsoChronology.INSTANCE, Locale.US);
        String timePattern = DateTimeFormatterBuilder.getLocalizedDateTimePattern(
                null, FormatStyle.SHORT, IsoChronology.INSTANCE, Locale.US);
        String dateTimePattern = DateTimeFormatterBuilder.getLocalizedDateTimePattern(
                FormatStyle.SHORT, FormatStyle.SHORT, IsoChronology.INSTANCE, Locale.US);

        System.out.println("Date pattern valid: " + (datePattern.length() > 0));
        System.out.println("Time pattern valid: " + (timePattern.length() > 0));
        System.out.println("Date-time pattern valid: " + (dateTimePattern.length() > 0));
    }

    private static void testLocaleDisplayNames() {
        Locale locale = Locale.US;

        System.out.println("Locale language display valid: " + (locale.getDisplayLanguage(Locale.US).length() > 0));
        System.out.println("Locale country display valid: " + (locale.getDisplayCountry(Locale.US).length() > 0));
        System.out.println("Locale display name valid: " + (locale.getDisplayName(Locale.US).length() > 0));
    }

    private static void testTimeZoneNames() {
        TimeZone timeZone = TimeZone.getTimeZone("America/Denver");

        System.out.println("Time zone standard valid: " + (timeZone.getDisplayName(false, TimeZone.LONG, Locale.US).length() > 0));
        System.out.println("Time zone daylight valid: " + (timeZone.getDisplayName(true, TimeZone.SHORT, Locale.US).length() > 0));
    }
}
