import java.util.Locale;

public class Test {
    public static void main(String[] args) {
        testDefaultLocale();
        testSetDefault();
    }

    private static void testDefaultLocale() {
        System.out.println("=== Default Locale Tests ===");
        Locale defaultLocale = Locale.getDefault();
        System.out.println("Default locale: " + defaultLocale);
        System.out.println("Default language: " + defaultLocale.getLanguage());
        System.out.println("Default country: " + defaultLocale.getCountry());
    }

    private static void testSetDefault() {
        System.out.println("\n=== Set Default Locale Tests ===");
        Locale original = Locale.getDefault();
        try {
            System.out.println("Setting default to US");
            Locale.setDefault(Locale.US);
            System.out.println("New default: " + Locale.getDefault());
            
            System.out.println("Setting default to GERMANY");
            Locale.setDefault(Locale.GERMANY);
            System.out.println("New default: " + Locale.getDefault());
        } finally {
            // Restore original default
            Locale.setDefault(original);
        }
    }
}
