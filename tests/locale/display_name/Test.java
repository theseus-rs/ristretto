import java.util.Locale;

public class Test {
    public static void main(String[] args) {
        testDisplayNames();
    }

    private static void testDisplayNames() {
        System.out.println("=== Locale Display Name Tests ===");
        
        Locale us = Locale.US;
        Locale germany = Locale.GERMANY;
        Locale france = Locale.FRANCE;
        
        System.out.println("Target: US, Display in US");
        System.out.println("  Name: " + us.getDisplayName(us));
        System.out.println("  Language: " + us.getDisplayLanguage(us));
        System.out.println("  Country: " + us.getDisplayCountry(us));
        
        System.out.println("Target: GERMANY, Display in US");
        System.out.println("  Name: " + germany.getDisplayName(us));
        System.out.println("  Language: " + germany.getDisplayLanguage(us));
        System.out.println("  Country: " + germany.getDisplayCountry(us));
        
        System.out.println("Target: US, Display in GERMANY");
        System.out.println("  Name: " + us.getDisplayName(germany));
        System.out.println("  Language: " + us.getDisplayLanguage(germany));
        System.out.println("  Country: " + us.getDisplayCountry(germany));
        
        System.out.println("Target: FRANCE, Display in FRANCE");
        System.out.println("  Name: " + france.getDisplayName(france));
        System.out.println("  Language: " + france.getDisplayLanguage(france));
        System.out.println("  Country: " + france.getDisplayCountry(france));

        // Default locale versions
        System.out.println("\nUsing default locale (" + Locale.getDefault() + "):");
        System.out.println("  US Name: " + us.getDisplayName());
        System.out.println("  Germany Name: " + germany.getDisplayName());
    }
}
