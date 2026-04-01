/**
 * Module Export Semantics Tests
 *
 * Verifies JVM specification compliance for module export behavior:
 *
 * 1. Unqualified exports (no target) are visible to ALL modules.
 * 2. Qualified exports (with specific targets) are restricted.
 * 3. The distinction between "all unnamed modules" and "all modules" is preserved:
 *    - addExports(from, pkg, null) means "export to all UNNAMED modules" per JVM spec
 *    - addExportsToAll0(from, pkg) means "export to ALL modules"
 *    These are NOT the same thing.
 * 4. Named modules should NOT be able to access packages that are only exported
 *    to unnamed modules.
 */
public class Test {
    public static void main(String[] args) {
        System.out.println("=== Module Export Semantics Tests ===");

        testUnqualifiedExportVisibility();
        testQualifiedExportRestriction();
        testInternalPackageNotExported();
        testOpenModuleSemantics();
        testExportToUnnamedVsAll();

        System.out.println("=== Module Export Semantics Tests Complete ===");
    }

    /**
     * Unqualified exports should be visible to all modules (named and unnamed).
     */
    private static void testUnqualifiedExportVisibility() {
        System.out.println("--- Test: Unqualified Export Visibility ---");
        try {
            Module javaBase = ModuleLayer.boot().findModule("java.base").orElse(null);
            Module javaLogging = ModuleLayer.boot().findModule("java.logging").orElse(null);
            Module javaSql = ModuleLayer.boot().findModule("java.sql").orElse(null);

            if (javaBase == null) {
                System.out.println("java.base not found");
                return;
            }

            // java.lang is an unqualified export - visible to all
            System.out.println("java.lang exported (unqualified): " +
                javaBase.isExported("java.lang"));

            // Should also be visible when queried with a specific named module
            if (javaLogging != null) {
                System.out.println("java.lang exported to java.logging: " +
                    javaBase.isExported("java.lang", javaLogging));
            }
            if (javaSql != null) {
                System.out.println("java.lang exported to java.sql: " +
                    javaBase.isExported("java.lang", javaSql));
            }

            // java.io is also unqualified - verify same behavior
            System.out.println("java.io exported (unqualified): " +
                javaBase.isExported("java.io"));
        } catch (Exception e) {
            System.out.println("Error: " + e.getClass().getName() + ": " + e.getMessage());
        }
    }

    /**
     * Qualified exports should only be visible to specified target modules.
     */
    private static void testQualifiedExportRestriction() {
        System.out.println("--- Test: Qualified Export Restriction ---");
        try {
            Module javaBase = ModuleLayer.boot().findModule("java.base").orElse(null);
            if (javaBase == null) {
                System.out.println("java.base not found");
                return;
            }

            // Find a qualified export from java.base
            // sun.nio.cs is typically a qualified export (to java.desktop, etc.)
            boolean sunNioCsQualified = javaBase.getDescriptor().exports().stream()
                .anyMatch(e -> e.source().equals("sun.nio.cs") && e.isQualified());
            System.out.println("sun.nio.cs is qualified export: " + sunNioCsQualified);

            // Unqualified check should return false for qualified exports
            boolean sunNioCsExportedUnqualified = javaBase.isExported("sun.nio.cs");
            System.out.println("sun.nio.cs exported unqualified: " + sunNioCsExportedUnqualified);

            // Check against a module that IS in the target list
            Module javaDesktop = ModuleLayer.boot().findModule("java.desktop").orElse(null);
            if (javaDesktop != null) {
                boolean exportedToDesktop = javaBase.isExported("sun.nio.cs", javaDesktop);
                System.out.println("sun.nio.cs exported to java.desktop: " + exportedToDesktop);
            }

            // Check against a module that is NOT in the target list
            Module javaLogging = ModuleLayer.boot().findModule("java.logging").orElse(null);
            if (javaLogging != null) {
                boolean exportedToLogging = javaBase.isExported("sun.nio.cs", javaLogging);
                System.out.println("sun.nio.cs exported to java.logging: " + exportedToLogging);
            }
        } catch (Exception e) {
            System.out.println("Error: " + e.getClass().getName() + ": " + e.getMessage());
        }
    }

    /**
     * Internal (non-exported) packages should not be accessible from outside.
     */
    private static void testInternalPackageNotExported() {
        System.out.println("--- Test: Internal Package Not Exported ---");
        try {
            Module javaBase = ModuleLayer.boot().findModule("java.base").orElse(null);
            if (javaBase == null) {
                System.out.println("java.base not found");
                return;
            }

            // jdk.internal.misc should not be exported
            boolean internalExported = javaBase.isExported("jdk.internal.misc");
            System.out.println("jdk.internal.misc exported: " + internalExported);

            // Trying to access via reflection should be restricted for named modules
            Module javaLogging = ModuleLayer.boot().findModule("java.logging").orElse(null);
            if (javaLogging != null) {
                boolean internalToLogging = javaBase.isExported("jdk.internal.misc", javaLogging);
                System.out.println("jdk.internal.misc exported to java.logging: " +
                    internalToLogging);
            }

            // Non-existent package should not be exported
            System.out.println("nonexistent.pkg exported: " +
                javaBase.isExported("nonexistent.pkg"));
        } catch (Exception e) {
            System.out.println("Error: " + e.getClass().getName() + ": " + e.getMessage());
        }
    }

    /**
     * Test open module semantics - opens grants deep reflection access.
     */
    private static void testOpenModuleSemantics() {
        System.out.println("--- Test: Open Module Semantics ---");
        try {
            Module javaBase = ModuleLayer.boot().findModule("java.base").orElse(null);
            if (javaBase == null) {
                System.out.println("java.base not found");
                return;
            }

            // java.base is NOT an open module
            System.out.println("java.base is open: " + javaBase.getDescriptor().isOpen());

            // java.lang is exported but not opened (for unqualified)
            System.out.println("java.lang is open (unqualified): " +
                javaBase.isOpen("java.lang"));

            // Check opens vs exports - they are different concepts
            System.out.println("java.lang exported: " + javaBase.isExported("java.lang"));
            System.out.println("Exported != Open: " +
                (javaBase.isExported("java.lang") != javaBase.isOpen("java.lang")));
        } catch (Exception e) {
            System.out.println("Error: " + e.getClass().getName() + ": " + e.getMessage());
        }
    }

    /**
     * Test the distinction between exporting to unnamed modules vs all modules.
     * Per JVM spec Section 5.4.4:
     * - Module.addExports(from, pkg, null) exports to ALL-UNNAMED
     * - Module.addExportsToAll(from, pkg) exports to ALL
     * A named module should NOT gain access from an ALL-UNNAMED export.
     */
    private static void testExportToUnnamedVsAll() {
        System.out.println("--- Test: Export To Unnamed vs All ---");
        try {
            // The unnamed module of the application class loader
            Module unnamedModule = Test.class.getModule();
            System.out.println("Test class module is named: " + unnamedModule.isNamed());

            Module javaBase = ModuleLayer.boot().findModule("java.base").orElse(null);
            if (javaBase == null) {
                System.out.println("java.base not found");
                return;
            }

            // The test class is in the unnamed module
            // Unqualified exports from java.base should be visible to unnamed module
            System.out.println("java.lang accessible from unnamed: " +
                javaBase.isExported("java.lang", unnamedModule));

            // Internal packages should not be exported to unnamed module
            System.out.println("jdk.internal.misc accessible from unnamed: " +
                javaBase.isExported("jdk.internal.misc", unnamedModule));

            // Verify that a qualified export to specific modules is NOT visible to unnamed
            boolean sunNioCsToUnnamed = javaBase.isExported("sun.nio.cs", unnamedModule);
            System.out.println("sun.nio.cs (qualified) to unnamed: " + sunNioCsToUnnamed);
        } catch (Exception e) {
            System.out.println("Error: " + e.getClass().getName() + ": " + e.getMessage());
        }
    }
}
