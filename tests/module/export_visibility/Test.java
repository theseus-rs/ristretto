/**
 * Module Export Visibility Tests
 * Tests that unqualified exports are visible across all modules (EVERYONE_MODULE)
 * and that Module.getClassLoader() returns the correct loader.
 */
public class Test {
    public static void main(String[] args) {
        System.out.println("=== Module Export Visibility Tests ===");

        testUnqualifiedExportsVisible();
        testQualifiedExportsRestricted();
        testModuleLoaderField();
        testIsExportedWithTargetModule();

        System.out.println("=== Module Export Visibility Tests Complete ===");
    }

    /**
     * Test that unqualified exports from java.base are visible to all modules.
     * If EVERYONE_MODULE is null, isExported() would return false for unqualified exports.
     */
    private static void testUnqualifiedExportsVisible() {
        System.out.println("--- Test: Unqualified exports visible ---");
        try {
            Module javaBase = ModuleLayer.boot().findModule("java.base").orElse(null);
            if (javaBase == null) {
                System.out.println("java.base not found");
                return;
            }

            // java.lang is an unqualified export from java.base
            System.out.println("java.lang exported: " + javaBase.isExported("java.lang"));
            System.out.println("java.io exported: " + javaBase.isExported("java.io"));
            System.out.println("java.util exported: " + javaBase.isExported("java.util"));
            System.out.println("java.net exported: " + javaBase.isExported("java.net"));

            // Verify isExported with a specific target module (should also be true for unqualified)
            Module javaLogging = ModuleLayer.boot().findModule("java.logging").orElse(null);
            if (javaLogging != null) {
                System.out.println("java.lang exported to java.logging: " +
                    javaBase.isExported("java.lang", javaLogging));
            }
        } catch (Exception e) {
            System.out.println("Error: " + e.getClass().getName() + ": " + e.getMessage());
        }
    }

    /**
     * Test that qualified exports are restricted to their target modules.
     */
    private static void testQualifiedExportsRestricted() {
        System.out.println("--- Test: Qualified exports restricted ---");
        try {
            Module javaBase = ModuleLayer.boot().findModule("java.base").orElse(null);
            if (javaBase == null) {
                System.out.println("java.base not found");
                return;
            }

            // sun.nio.cs is a qualified export; should not be exported to everyone
            boolean sunNioCsExported = javaBase.isExported("sun.nio.cs");
            System.out.println("sun.nio.cs exported (unqualified): " + sunNioCsExported);

            // Non-existent package should not be exported
            System.out.println("nonexistent.pkg exported: " +
                javaBase.isExported("nonexistent.pkg"));
        } catch (Exception e) {
            System.out.println("Error: " + e.getClass().getName() + ": " + e.getMessage());
        }
    }

    /**
     * Test that Module.getClassLoader() returns the correct value.
     * Boot-loaded modules should return null (bootstrap class loader).
     */
    private static void testModuleLoaderField() {
        System.out.println("--- Test: Module loader field ---");
        try {
            Module javaBase = ModuleLayer.boot().findModule("java.base").orElse(null);
            if (javaBase == null) {
                System.out.println("java.base not found");
                return;
            }

            // java.base is loaded by the bootstrap class loader (null)
            ClassLoader baseLoader = javaBase.getClassLoader();
            System.out.println("java.base loader is null: " + (baseLoader == null));

            // Test via ModuleLayer.findLoader
            ClassLoader layerLoader = ModuleLayer.boot().findLoader("java.base");
            System.out.println("findLoader(java.base) is null: " + (layerLoader == null));
        } catch (Exception e) {
            System.out.println("Error: " + e.getClass().getName() + ": " + e.getMessage());
        }
    }

    /**
     * Test isExported with a target module for various combinations.
     */
    private static void testIsExportedWithTargetModule() {
        System.out.println("--- Test: isExported with target module ---");
        try {
            Module javaBase = ModuleLayer.boot().findModule("java.base").orElse(null);
            Module javaSql = ModuleLayer.boot().findModule("java.sql").orElse(null);

            if (javaBase == null || javaSql == null) {
                System.out.println("Required modules not found");
                return;
            }

            // Unqualified export should be visible to any specific module
            System.out.println("java.lang exported to java.sql: " +
                javaBase.isExported("java.lang", javaSql));

            // java.sql exports java.sql package (unqualified)
            System.out.println("java.sql exports java.sql: " +
                javaSql.isExported("java.sql"));
            System.out.println("java.sql exports java.sql to java.base: " +
                javaSql.isExported("java.sql", javaBase));
        } catch (Exception e) {
            System.out.println("Error: " + e.getClass().getName() + ": " + e.getMessage());
        }
    }
}
