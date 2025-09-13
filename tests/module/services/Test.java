import java.nio.charset.spi.CharsetProvider;
import java.util.ServiceLoader;

/**
 * Module Services and Providers Tests
 * Tests service loading, provider registration, and service interfaces
 */
public class Test {
    public static void main(String[] args) {
        System.out.println("=== Module Services and Providers Tests ===");

        // Test 1: Service loader functionality
        testServiceLoader();

        // Test 2: Module provides/uses
        testModuleProvidesUses();

        // Test 3: Built-in service providers
        testBuiltInServices();

        // Test 4: Service iteration
        testServiceIteration();

        System.out.println("=== Module Services Tests Complete ===");
    }

    private static void testServiceLoader() {
        System.out.println("--- Test: Service Loader ---");
        try {
            // Test loading charset providers (common service)
            ServiceLoader<java.nio.charset.spi.CharsetProvider> charsetLoader =
                ServiceLoader.load(java.nio.charset.spi.CharsetProvider.class);
            System.out.println("CharsetProvider service loader: " + charsetLoader);

            int providerCount = 0;
            for (java.nio.charset.spi.CharsetProvider provider : charsetLoader) {
                providerCount++;
                System.out.println("  Provider " + providerCount + ": " + provider.getClass().getName());
                if (providerCount >= 3) break; // Limit output
            }
            System.out.println("Total charset providers found: " + providerCount);

        } catch (Exception e) {
            System.out.println("Error testing service loader: " + e.getMessage());
        }
    }

    private static void testModuleProvidesUses() {
        System.out.println("--- Test: Module Provides/Uses ---");
        try {
            // Check java.base module for services
            Module javaBase = ModuleLayer.boot().findModule("java.base").orElse(null);
            if (javaBase != null && javaBase.getDescriptor() != null) {
                System.out.println("java.base provides services:");
                javaBase.getDescriptor().provides().stream()
                    .limit(5)
                    .forEach(provides -> {
                        System.out.println("  Service: " + provides.service());
                        System.out.println("    Providers: " + provides.providers());
                    });

                System.out.println("java.base uses services:");
                javaBase.getDescriptor().uses().stream()
                    .limit(5)
                    .forEach(uses -> System.out.println("  " + uses));
            }

        } catch (Exception e) {
            System.out.println("Error testing provides/uses: " + e.getMessage());
        }
    }

    private static void testBuiltInServices() {
        System.out.println("--- Test: Built-in Services ---");
        try {
            // Test file system provider service
            ServiceLoader<java.nio.file.spi.FileSystemProvider> fsLoader =
                ServiceLoader.load(java.nio.file.spi.FileSystemProvider.class);
            System.out.println("FileSystemProvider services:");
            int fsCount = 0;
            for (java.nio.file.spi.FileSystemProvider provider : fsLoader) {
                fsCount++;
                System.out.println("  " + fsCount + ": " + provider.getScheme() +
                    " (" + provider.getClass().getName() + ")");
                if (fsCount >= 5) break;
            }

            // Test locale service provider
            ServiceLoader<java.util.spi.LocaleServiceProvider> localeLoader =
                ServiceLoader.load(java.util.spi.LocaleServiceProvider.class);
            System.out.println("LocaleServiceProvider count: " +
                localeLoader.stream().count());

        } catch (Exception e) {
            System.out.println("Error testing built-in services: " + e.getMessage());
        }
    }

    private static void testServiceIteration() {
        System.out.println("--- Test: Service Iteration ---");
        try {
            ServiceLoader<java.nio.charset.spi.CharsetProvider> loader =
                ServiceLoader.load(java.nio.charset.spi.CharsetProvider.class);

            // Test stream API
            System.out.println("Using stream API:");
            long streamCount = loader.stream()
                .peek(provider -> System.out.println("  Provider type: " + provider.type()))
                .count();
            System.out.println("Stream count: " + streamCount);

            // Test iterator
            System.out.println("Using iterator:");
            int iterCount = 0;
            for (java.nio.charset.spi.CharsetProvider provider : loader) {
                iterCount++;
                if (iterCount >= 2) break; // Limit output
            }
            System.out.println("Iterator count (limited): " + iterCount);

            // Test reload
            loader.reload();
            System.out.println("Service loader reloaded successfully");

        } catch (Exception e) {
            System.out.println("Error testing service iteration: " + e.getMessage());
        }
    }
}
