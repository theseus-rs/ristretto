import java.nio.charset.Charset;
import java.nio.charset.spi.CharsetProvider;
import java.util.ArrayList;
import java.util.Collections;
import java.util.HashSet;
import java.util.List;
import java.util.ServiceLoader;
import java.util.Set;

/**
 * Module Service Providers Tests
 *
 * Verifies that ServiceLoader discovers providers from ALL modules that declare
 * a "provides" for the same service interface. In the JDK, CharsetProvider is
 * provided by both java.base (sun.nio.cs.StandardCharsets) and jdk.charsets
 * (sun.nio.cs.ext.ExtendedCharsets). If the module system incorrectly overwrites
 * provider registrations, only the last module's providers will be found.
 */
public class Test {
    public static void main(String[] args) {
        System.out.println("=== Module Service Providers Tests ===");

        testMultiModuleCharsetProviders();
        testProviderModuleOrigins();
        testCharsetAvailability();

        System.out.println("=== Module Service Providers Tests Complete ===");
    }

    /**
     * Test that ServiceLoader finds CharsetProvider implementations from multiple modules.
     * java.base provides sun.nio.cs.StandardCharsets.
     * jdk.charsets provides sun.nio.cs.ext.ExtendedCharsets.
     * Both must be discovered.
     */
    private static void testMultiModuleCharsetProviders() {
        System.out.println("--- Test: Multi-Module Charset Providers ---");
        try {
            ServiceLoader<CharsetProvider> loader = ServiceLoader.load(CharsetProvider.class);
            List<String> providerNames = new ArrayList<>();
            Set<String> providerModules = new HashSet<>();
            for (CharsetProvider provider : loader) {
                providerNames.add(provider.getClass().getName());
                Module mod = provider.getClass().getModule();
                if (mod != null && mod.getName() != null) {
                    providerModules.add(mod.getName());
                }
            }
            Collections.sort(providerNames);
            System.out.println("CharsetProvider count: " + providerNames.size());
            System.out.println("CharsetProvider count >= 2: " + (providerNames.size() >= 2));
            for (String name : providerNames) {
                System.out.println("  Provider: " + name);
            }

            List<String> sortedModules = new ArrayList<>(providerModules);
            Collections.sort(sortedModules);
            System.out.println("Provider modules: " + sortedModules);
            System.out.println("Providers from multiple modules: " + (providerModules.size() >= 2));
        } catch (Exception e) {
            System.out.println("Error: " + e.getClass().getName() + ": " + e.getMessage());
        }
    }

    /**
     * Test that providers retain their correct module origin.
     * Each provider class should report the module it was loaded from.
     */
    private static void testProviderModuleOrigins() {
        System.out.println("--- Test: Provider Module Origins ---");
        try {
            ServiceLoader<CharsetProvider> loader = ServiceLoader.load(CharsetProvider.class);
            List<String> lines = new ArrayList<>();
            for (CharsetProvider provider : loader) {
                String className = provider.getClass().getName();
                Module mod = provider.getClass().getModule();
                String moduleName = (mod != null && mod.getName() != null) ? mod.getName() : "unnamed";
                lines.add(className + " from " + moduleName);
            }
            Collections.sort(lines);
            for (String line : lines) {
                System.out.println("  " + line);
            }
        } catch (Exception e) {
            System.out.println("Error: " + e.getClass().getName() + ": " + e.getMessage());
        }
    }

    /**
     * Test that charsets from multiple providers are actually available.
     * US-ASCII comes from java.base's StandardCharsets.
     * Big5 comes from jdk.charsets' ExtendedCharsets.
     * Both must be available for ServiceLoader to have found all providers.
     */
    private static void testCharsetAvailability() {
        System.out.println("--- Test: Charset Availability ---");
        try {
            // US-ASCII is from java.base (StandardCharsets)
            boolean hasUsAscii = Charset.isSupported("US-ASCII");
            System.out.println("US-ASCII supported (java.base): " + hasUsAscii);

            // UTF-8 is from java.base
            boolean hasUtf8 = Charset.isSupported("UTF-8");
            System.out.println("UTF-8 supported (java.base): " + hasUtf8);

            // Big5 is from jdk.charsets (ExtendedCharsets)
            boolean hasBig5 = Charset.isSupported("Big5");
            System.out.println("Big5 supported (jdk.charsets): " + hasBig5);

            // EUC-JP is from jdk.charsets
            boolean hasEucJp = Charset.isSupported("EUC-JP");
            System.out.println("EUC-JP supported (jdk.charsets): " + hasEucJp);
        } catch (Exception e) {
            System.out.println("Error: " + e.getClass().getName() + ": " + e.getMessage());
        }
    }
}
