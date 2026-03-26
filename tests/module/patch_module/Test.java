import java.lang.module.Configuration;
import java.lang.module.ModuleDescriptor;
import java.lang.module.ModuleFinder;
import java.lang.module.ModuleReference;
import java.util.Optional;
import java.util.Set;

/**
 * Patch Module Tests
 * Tests the --patch-module concept via the Module API.
 * While --patch-module is a JVM command-line option, we test the underlying
 * module system behavior: package membership, addExports, addOpens, and
 * how modules interact with patched content.
 */
public class Test {
    public static void main(String[] args) {
        System.out.println("=== Patch Module Tests ===");

        testModulePackageMembership();
        testAddExportsToUnnamed();
        testAddOpensToUnnamed();
        testAddReadsEdge();
        testModuleAnnotations();

        System.out.println("=== Patch Module Tests Complete ===");
    }

    /**
     * Test package membership queries; the basis for understanding which
     * packages belong to which module.
     */
    private static void testModulePackageMembership() {
        System.out.println("--- Test: Module Package Membership ---");
        try {
            Module javaBase = ModuleLayer.boot().findModule("java.base").orElse(null);
            if (javaBase == null) {
                System.out.println("java.base not found");
                return;
            }

            // java.lang should be in java.base
            System.out.println("java.base contains java.lang: " +
                javaBase.getDescriptor().packages().contains("java.lang"));
            System.out.println("java.base contains java.util: " +
                javaBase.getDescriptor().packages().contains("java.util"));
            System.out.println("java.base contains java.io: " +
                javaBase.getDescriptor().packages().contains("java.io"));

            // sun.misc should also be in java.base
            System.out.println("java.base contains sun.misc: " +
                javaBase.getDescriptor().packages().contains("sun.misc"));

            // java.sql should NOT be in java.base (it's in java.sql module)
            System.out.println("java.base contains java.sql: " +
                javaBase.getDescriptor().packages().contains("java.sql"));

            System.out.println("java.base total packages: " +
                javaBase.getDescriptor().packages().size());
        } catch (Exception e) {
            System.out.println("Error: " + e.getMessage());
        }
    }

    /**
     * Test adding exports from a named module to the unnamed module.
     * This mirrors what --patch-module + --add-exports would accomplish.
     */
    private static void testAddExportsToUnnamed() {
        System.out.println("--- Test: Add Exports to Unnamed ---");
        try {
            Module javaBase = ModuleLayer.boot().findModule("java.base").orElse(null);
            Module unnamed = Test.class.getModule();

            if (javaBase == null) {
                System.out.println("java.base not found");
                return;
            }

            // java.lang is unconditionally exported; should be accessible
            System.out.println("java.lang exported to unnamed: " +
                javaBase.isExported("java.lang", unnamed));
            System.out.println("java.lang exported to all: " +
                javaBase.isExported("java.lang"));

            // sun.security.ssl is not exported; check baseline
            boolean sslExported = javaBase.isExported("sun.security.ssl", unnamed);
            System.out.println("sun.security.ssl exported to unnamed (before): " + sslExported);

            // Attempting to add exports from unnamed code should throw IllegalCallerException
            try {
                javaBase.addExports("sun.security.ssl", unnamed);
                System.out.println("addExports succeeded (unexpected)");
            } catch (IllegalCallerException e) {
                System.out.println("addExports denied: " + e.getClass().getSimpleName());
            }
        } catch (Exception e) {
            System.out.println("Error: " + e.getClass().getSimpleName() + ": " + e.getMessage());
        }
    }

    /**
     * Test adding opens from a named module to the unnamed module.
     * This mirrors what --add-opens would accomplish.
     */
    private static void testAddOpensToUnnamed() {
        System.out.println("--- Test: Add Opens to Unnamed ---");
        try {
            Module javaBase = ModuleLayer.boot().findModule("java.base").orElse(null);
            Module unnamed = Test.class.getModule();

            if (javaBase == null) {
                System.out.println("java.base not found");
                return;
            }

            // Check if java.lang is open (it may or may not be depending on JDK)
            System.out.println("java.lang is open to unnamed: " +
                javaBase.isOpen("java.lang", unnamed));
            System.out.println("java.lang is open to all: " +
                javaBase.isOpen("java.lang"));

            // Internal packages should not be open
            System.out.println("sun.security.ssl open to unnamed: " +
                javaBase.isOpen("sun.security.ssl", unnamed));

            // Try to add opens from unnamed (should fail)
            try {
                javaBase.addOpens("sun.security.ssl", unnamed);
                System.out.println("addOpens succeeded (unexpected)");
            } catch (IllegalCallerException e) {
                System.out.println("addOpens denied: " + e.getClass().getSimpleName());
            }
        } catch (Exception e) {
            System.out.println("Error: " + e.getClass().getSimpleName() + ": " + e.getMessage());
        }
    }

    /**
     * Test adding read edges between modules.
     */
    private static void testAddReadsEdge() {
        System.out.println("--- Test: Add Reads Edge ---");
        try {
            Module unnamed = Test.class.getModule();
            Module javaBase = ModuleLayer.boot().findModule("java.base").orElse(null);
            Module javaLogging = ModuleLayer.boot().findModule("java.logging").orElse(null);

            if (javaBase == null || javaLogging == null) {
                System.out.println("Required modules not found");
                return;
            }

            // Unnamed module can read all named modules
            System.out.println("Unnamed reads java.base: " + unnamed.canRead(javaBase));
            System.out.println("Unnamed reads java.logging: " + unnamed.canRead(javaLogging));

            // Named modules read what they declared
            System.out.println("java.logging reads java.base: " +
                javaLogging.canRead(javaBase));

            // Every module can read itself
            System.out.println("java.base reads itself: " + javaBase.canRead(javaBase));
            System.out.println("Unnamed reads itself: " + unnamed.canRead(unnamed));
        } catch (Exception e) {
            System.out.println("Error: " + e.getClass().getSimpleName() + ": " + e.getMessage());
        }
    }

    /**
     * Test module annotations and metadata that could be affected by patching.
     */
    private static void testModuleAnnotations() {
        System.out.println("--- Test: Module Annotations ---");
        try {
            Module javaBase = ModuleLayer.boot().findModule("java.base").orElse(null);
            if (javaBase == null) {
                System.out.println("java.base not found");
                return;
            }

            // Check for annotations on java.base
            java.lang.annotation.Annotation[] annotations = javaBase.getAnnotations();
            System.out.println("java.base annotation count: " + annotations.length);

            java.lang.annotation.Annotation[] declaredAnnotations = javaBase.getDeclaredAnnotations();
            System.out.println("java.base declared annotation count: " + declaredAnnotations.length);

            // Module should be named
            System.out.println("java.base is named: " + javaBase.isNamed());
            System.out.println("java.base name: " + javaBase.getName());
        } catch (Exception e) {
            System.out.println("Error: " + e.getClass().getSimpleName() + ": " + e.getMessage());
        }
    }
}
