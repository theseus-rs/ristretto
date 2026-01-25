import java.util.Locale;
import java.util.Arrays;
import java.util.Comparator;

public class Test {
    public static void main(String[] args) {
        testAvailableLocales();
    }

    private static void testAvailableLocales() {
        System.out.println("=== Available Locales Tests ===");
        Locale[] available = Locale.getAvailableLocales();
        System.out.println("Number of available locales: " + available.length);
        
        // Sort for consistent output
        Arrays.sort(available, new Comparator<Locale>() {
            @Override
            public int compare(Locale l1, Locale l2) {
                return l1.toString().compareTo(l2.toString());
            }
        });

        // Print first 10 and last 10 to avoid huge output logs but verify some content
        System.out.println("First 10 available locales:");
        for (int i = 0; i < Math.min(10, available.length); i++) {
            System.out.println("  " + available[i]);
        }
        
        System.out.println("Checking for specific common locales:");
        checkLocalePresence(available, "en_US");
        checkLocalePresence(available, "de_DE");
        checkLocalePresence(available, "ja_JP");
        checkLocalePresence(available, "zh_CN");
    }

    private static void checkLocalePresence(Locale[] locales, String localeString) {
        boolean found = false;
        for (Locale l : locales) {
            if (l.toString().equals(localeString)) {
                found = true;
                break;
            }
        }
        System.out.println("Contains " + localeString + ": " + found);
    }
}
