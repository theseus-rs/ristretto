import java.util.Locale;

public class Test {
    public static void main(String[] args) {
        testLocaleConstants();
    }

    private static void testLocaleConstants() {
        System.out.println("=== Locale Constants Tests ===");
        
        printLocaleConstant("CANADA", Locale.CANADA);
        printLocaleConstant("CANADA_FRENCH", Locale.CANADA_FRENCH);
        printLocaleConstant("CHINA", Locale.CHINA);
        printLocaleConstant("CHINESE", Locale.CHINESE);
        printLocaleConstant("ENGLISH", Locale.ENGLISH);
        printLocaleConstant("FRANCE", Locale.FRANCE);
        printLocaleConstant("FRENCH", Locale.FRENCH);
        printLocaleConstant("GERMAN", Locale.GERMAN);
        printLocaleConstant("GERMANY", Locale.GERMANY);
        printLocaleConstant("ITALIAN", Locale.ITALIAN);
        printLocaleConstant("ITALY", Locale.ITALY);
        printLocaleConstant("JAPAN", Locale.JAPAN);
        printLocaleConstant("JAPANESE", Locale.JAPANESE);
        printLocaleConstant("KOREA", Locale.KOREA);
        printLocaleConstant("KOREAN", Locale.KOREAN);
        printLocaleConstant("PRC", Locale.PRC);
        printLocaleConstant("ROOT", Locale.ROOT);
        printLocaleConstant("SIMPLIFIED_CHINESE", Locale.SIMPLIFIED_CHINESE);
        printLocaleConstant("TAIWAN", Locale.TAIWAN);
        printLocaleConstant("TRADITIONAL_CHINESE", Locale.TRADITIONAL_CHINESE);
        printLocaleConstant("UK", Locale.UK);
        printLocaleConstant("US", Locale.US);
    }

    private static void printLocaleConstant(String name, Locale locale) {
        System.out.println("Locale." + name + ": " + locale + 
                         " (Language: " + locale.getLanguage() + 
                         ", Country: " + locale.getCountry() + ")");
    }
}
