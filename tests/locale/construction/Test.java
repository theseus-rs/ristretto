import java.util.Locale;

public class Test {
    public static void main(String[] args) {
        testConstructors();
        testForLanguageTag();
        testBuilder();
    }

    private static void testConstructors() {
        System.out.println("=== Locale Constructors Tests ===");
        
        Locale l1 = new Locale("en");
        System.out.println("new Locale(\"en\"): " + l1);
        
        Locale l2 = new Locale("en", "US");
        System.out.println("new Locale(\"en\", \"US\"): " + l2);
        
        Locale l3 = new Locale("en", "US", "WIN");
        System.out.println("new Locale(\"en\", \"US\", \"WIN\"): " + l3);

        Locale l4 = new Locale("fr");
        System.out.println("new Locale(\"fr\"): " + l4);
    }

    private static void testForLanguageTag() {
        System.out.println("\n=== Locale.forLanguageTag Tests ===");
        
        Locale l1 = Locale.forLanguageTag("en-US");
        System.out.println("forLanguageTag(\"en-US\"): " + l1);
        
        Locale l2 = Locale.forLanguageTag("zh-CN");
        System.out.println("forLanguageTag(\"zh-CN\"): " + l2);
        
        Locale l3 = Locale.forLanguageTag("und");
        System.out.println("forLanguageTag(\"und\"): " + l3);
        
        Locale l4 = Locale.forLanguageTag("i-klingon"); // Grandfathered tag example
        System.out.println("forLanguageTag(\"i-klingon\"): " + l4); 
        
        Locale l5 = Locale.forLanguageTag("de-DE-u-co-phonebk"); // Extension
        System.out.println("forLanguageTag(\"de-DE-u-co-phonebk\"): " + l5);
    }

    private static void testBuilder() {
        System.out.println("\n=== Locale.Builder Tests ===");
        
        Locale.Builder builder = new Locale.Builder();
        builder.setLanguage("en").setRegion("GB");
        Locale l1 = builder.build();
        System.out.println("Builder(en, GB): " + l1);
        
        builder.clear();
        builder.setLanguage("fr").setRegion("CA").setVariant("POSIX");
        Locale l2 = builder.build();
        System.out.println("Builder(fr, CA, POSIX): " + l2);
    }
}
