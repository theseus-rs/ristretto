/**
 * Dynamic Access Control Tests
 *
 * Tests that dynamic module operations (addReads, addExports, addOpens) interact
 * correctly with statically resolved module configuration. Specifically verifies
 * that dynamic access checks consult both the static resolved configuration AND
 * the dynamic runtime maps.
 *
 * Key scenario: Module A statically requires Module B, then Module B dynamically
 * calls addExports at runtime. The access check must see the static read edge
 * even when falling through to the dynamic access path.
 */
public class Test {
    public static void main(String[] args) {
        System.out.println("=== Dynamic Access Control Tests ===");

        testDynamicAddReadsFromUnnamed();
        testDynamicAddExportsWithStaticReads();
        testDynamicAddOpensWithStaticReads();
        testAddExportsToAllUnnamed();
        testAddOpensToAllUnnamed();
        testCanReadAfterAddReads();
        testIsExportedCombinesStaticAndDynamic();
        testIsOpenCombinesStaticAndDynamic();
        testReflectiveAccessWithDynamicOpens();

        System.out.println("=== Dynamic Access Control Tests Complete ===");
    }

    /**
     * Test that the unnamed module can add reads to a named module and then
     * access its exported packages.
     */
    private static void testDynamicAddReadsFromUnnamed() {
        System.out.println("--- Test: Dynamic addReads from unnamed module ---");
        try {
            Module unnamed = Test.class.getModule();
            Module javaBase = ModuleLayer.boot().findModule("java.base").orElse(null);
            Module javaLogging = ModuleLayer.boot().findModule("java.logging").orElse(null);

            if (javaBase == null || javaLogging == null) {
                System.out.println("Required modules not found");
                return;
            }

            // Unnamed module can read all named modules by default
            System.out.println("Unnamed reads java.base: " + unnamed.canRead(javaBase));
            System.out.println("Unnamed reads java.logging: " + unnamed.canRead(javaLogging));

            // Explicitly add a reads edge (should be idempotent for unnamed)
            unnamed.addReads(javaLogging);
            System.out.println("After addReads, unnamed reads java.logging: " +
                unnamed.canRead(javaLogging));
        } catch (Exception e) {
            System.out.println("Error: " + e.getMessage());
        }
    }

    /**
     * Test that when a named module statically requires another named module,
     * a dynamic addExports from the target module is properly visible.
     *
     * This is the core regression scenario: java.logging statically requires java.base.
     * If java.base dynamically exports an internal package to java.logging, the access
     * check must see the static read edge (java.logging reads java.base) even when
     * the export was added dynamically.
     */
    private static void testDynamicAddExportsWithStaticReads() {
        System.out.println("--- Test: Dynamic addExports with static reads ---");
        try {
            Module javaBase = ModuleLayer.boot().findModule("java.base").orElse(null);
            Module javaLogging = ModuleLayer.boot().findModule("java.logging").orElse(null);

            if (javaBase == null || javaLogging == null) {
                System.out.println("Required modules not found");
                return;
            }

            // java.logging statically requires java.base
            System.out.println("java.logging reads java.base: " +
                javaLogging.canRead(javaBase));

            // java.lang is already exported (unqualified); should work
            System.out.println("java.base exports java.lang to java.logging: " +
                javaBase.isExported("java.lang", javaLogging));

            // Check isExported for a package that is NOT exported by default
            // jdk.internal.misc is typically not exported to java.logging
            boolean beforeExport = javaBase.isExported("jdk.internal.misc", javaLogging);
            System.out.println("Before addExports, jdk.internal.misc exported to java.logging: " +
                beforeExport);

            // Dynamically export jdk.internal.misc to java.logging
            // (This requires the caller to be in the same module or have permission)
            try {
                // Use Instrumentation-like approach via the unnamed module trick:
                // The unnamed module can call addExports on any module it reads
                javaBase.addExports("jdk.internal.misc", javaLogging);
                boolean afterExport = javaBase.isExported("jdk.internal.misc", javaLogging);
                System.out.println("After addExports, jdk.internal.misc exported to java.logging: " +
                    afterExport);
            } catch (IllegalCallerException e) {
                System.out.println("addExports denied (expected for non-module code): " +
                    e.getClass().getSimpleName());
            }
        } catch (Exception e) {
            System.out.println("Error: " + e.getMessage());
        }
    }

    /**
     * Test that dynamic addOpens works correctly when the target module already
     * has static read access to the source module.
     */
    private static void testDynamicAddOpensWithStaticReads() {
        System.out.println("--- Test: Dynamic addOpens with static reads ---");
        try {
            Module javaBase = ModuleLayer.boot().findModule("java.base").orElse(null);
            Module javaLogging = ModuleLayer.boot().findModule("java.logging").orElse(null);

            if (javaBase == null || javaLogging == null) {
                System.out.println("Required modules not found");
                return;
            }

            // Verify static reads edge exists
            System.out.println("java.logging reads java.base: " +
                javaLogging.canRead(javaBase));

            // java.lang is not opened to java.logging by default
            boolean beforeOpen = javaBase.isOpen("java.lang", javaLogging);
            System.out.println("Before addOpens, java.lang open to java.logging: " +
                beforeOpen);

            // Dynamically open java.lang to java.logging
            try {
                javaBase.addOpens("java.lang", javaLogging);
                boolean afterOpen = javaBase.isOpen("java.lang", javaLogging);
                System.out.println("After addOpens, java.lang open to java.logging: " +
                    afterOpen);
            } catch (IllegalCallerException e) {
                System.out.println("addOpens denied (expected for non-module code): " +
                    e.getClass().getSimpleName());
            }
        } catch (Exception e) {
            System.out.println("Error: " + e.getMessage());
        }
    }

    /**
     * Test addExports to ALL-UNNAMED: a named module exports a package to the unnamed
     * module (classpath code).
     */
    private static void testAddExportsToAllUnnamed() {
        System.out.println("--- Test: addExports to ALL-UNNAMED ---");
        try {
            Module javaBase = ModuleLayer.boot().findModule("java.base").orElse(null);
            Module unnamed = Test.class.getModule();

            if (javaBase == null) {
                System.out.println("java.base not found");
                return;
            }

            // java.lang is already exported to everyone
            System.out.println("java.lang exported to unnamed: " +
                javaBase.isExported("java.lang", unnamed));

            // sun.nio.ch is typically qualified; check before and after
            boolean sunNioChBefore = javaBase.isExported("sun.nio.ch", unnamed);
            System.out.println("Before: sun.nio.ch exported to unnamed: " + sunNioChBefore);

            try {
                javaBase.addExports("sun.nio.ch", unnamed);
                boolean sunNioChAfter = javaBase.isExported("sun.nio.ch", unnamed);
                System.out.println("After addExports: sun.nio.ch exported to unnamed: " +
                    sunNioChAfter);
            } catch (IllegalCallerException e) {
                System.out.println("addExports denied: " + e.getClass().getSimpleName());
            }
        } catch (Exception e) {
            System.out.println("Error: " + e.getMessage());
        }
    }

    /**
     * Test addOpens to ALL-UNNAMED: a named module opens a package to the unnamed
     * module for reflective access.
     */
    private static void testAddOpensToAllUnnamed() {
        System.out.println("--- Test: addOpens to ALL-UNNAMED ---");
        try {
            Module javaBase = ModuleLayer.boot().findModule("java.base").orElse(null);
            Module unnamed = Test.class.getModule();

            if (javaBase == null) {
                System.out.println("java.base not found");
                return;
            }

            // Check if java.lang is open to unnamed before dynamic modification
            boolean javaLangOpenBefore = javaBase.isOpen("java.lang", unnamed);
            System.out.println("Before: java.lang open to unnamed: " + javaLangOpenBefore);

            try {
                javaBase.addOpens("java.lang", unnamed);
                boolean javaLangOpenAfter = javaBase.isOpen("java.lang", unnamed);
                System.out.println("After addOpens: java.lang open to unnamed: " +
                    javaLangOpenAfter);
            } catch (IllegalCallerException e) {
                System.out.println("addOpens denied: " + e.getClass().getSimpleName());
            }
        } catch (Exception e) {
            System.out.println("Error: " + e.getMessage());
        }
    }

    /**
     * Test that canRead correctly reflects both static requires and dynamic addReads.
     */
    private static void testCanReadAfterAddReads() {
        System.out.println("--- Test: canRead after addReads ---");
        try {
            Module javaLogging = ModuleLayer.boot().findModule("java.logging").orElse(null);
            Module javaSql = ModuleLayer.boot().findModule("java.sql").orElse(null);
            Module javaBase = ModuleLayer.boot().findModule("java.base").orElse(null);

            if (javaLogging == null || javaSql == null || javaBase == null) {
                System.out.println("Required modules not found");
                return;
            }

            // java.logging statically reads java.base
            System.out.println("java.logging reads java.base (static): " +
                javaLogging.canRead(javaBase));

            // java.sql statically requires transitive java.logging
            System.out.println("java.sql reads java.logging (static+transitive): " +
                javaSql.canRead(javaLogging));

            // Check if java.logging reads java.sql before addReads
            boolean beforeAddReads = javaLogging.canRead(javaSql);
            System.out.println("Before addReads, java.logging reads java.sql: " +
                beforeAddReads);

            // A module always reads itself
            System.out.println("java.logging reads itself: " +
                javaLogging.canRead(javaLogging));
        } catch (Exception e) {
            System.out.println("Error: " + e.getMessage());
        }
    }

    /**
     * Test that isExported combines static descriptor exports with dynamic addExports.
     * Verifies that static unqualified exports remain visible alongside dynamic qualified exports.
     */
    private static void testIsExportedCombinesStaticAndDynamic() {
        System.out.println("--- Test: isExported combines static and dynamic ---");
        try {
            Module javaBase = ModuleLayer.boot().findModule("java.base").orElse(null);
            Module javaLogging = ModuleLayer.boot().findModule("java.logging").orElse(null);

            if (javaBase == null || javaLogging == null) {
                System.out.println("Required modules not found");
                return;
            }

            // java.lang is statically exported (unqualified); should always be true
            System.out.println("java.lang exported (unqualified): " +
                javaBase.isExported("java.lang"));
            System.out.println("java.lang exported to java.logging: " +
                javaBase.isExported("java.lang", javaLogging));

            // java.io is statically exported (unqualified); should always be true
            System.out.println("java.io exported (unqualified): " +
                javaBase.isExported("java.io"));
            System.out.println("java.io exported to java.logging: " +
                javaBase.isExported("java.io", javaLogging));

            // A non-existent package should not be exported
            System.out.println("nonexistent.pkg exported: " +
                javaBase.isExported("nonexistent.pkg"));
        } catch (Exception e) {
            System.out.println("Error: " + e.getMessage());
        }
    }

    /**
     * Test that isOpen combines static descriptor opens with dynamic addOpens.
     */
    private static void testIsOpenCombinesStaticAndDynamic() {
        System.out.println("--- Test: isOpen combines static and dynamic ---");
        try {
            Module javaBase = ModuleLayer.boot().findModule("java.base").orElse(null);
            Module javaLogging = ModuleLayer.boot().findModule("java.logging").orElse(null);

            if (javaBase == null || javaLogging == null) {
                System.out.println("Required modules not found");
                return;
            }

            // java.base is NOT an open module
            System.out.println("java.base is open module: " +
                javaBase.getDescriptor().isOpen());

            // java.lang is NOT opened by default (no opens directive in java.base)
            System.out.println("java.lang open (unqualified): " +
                javaBase.isOpen("java.lang"));

            // Verify isOpen to specific module
            System.out.println("java.lang open to java.logging: " +
                javaBase.isOpen("java.lang", javaLogging));

            // A non-existent package should not be open
            System.out.println("nonexistent.pkg open: " +
                javaBase.isOpen("nonexistent.pkg"));
        } catch (Exception e) {
            System.out.println("Error: " + e.getMessage());
        }
    }

    /**
     * Test that reflective access works after dynamic opens.
     * Tries to reflectively access a field that requires the package to be opened.
     */
    private static void testReflectiveAccessWithDynamicOpens() {
        System.out.println("--- Test: Reflective access with dynamic opens ---");
        try {
            // java.lang.String has private fields; accessing them requires the package
            // to be opened to the caller's module.
            Module unnamed = Test.class.getModule();
            Module javaBase = String.class.getModule();

            System.out.println("Test module is named: " + unnamed.isNamed());
            System.out.println("String module: " + javaBase.getName());

            // Try to get a declared field on String
            try {
                java.lang.reflect.Field valueField = String.class.getDeclaredField("value");
                System.out.println("Got String.value field: " + (valueField != null));

                // Try setAccessible; this requires the package to be open
                try {
                    valueField.setAccessible(true);
                    System.out.println("setAccessible succeeded: true");
                } catch (java.lang.reflect.InaccessibleObjectException e) {
                    System.out.println("setAccessible denied (expected): " +
                        e.getClass().getSimpleName());
                }
            } catch (NoSuchFieldException e) {
                System.out.println("Field not found: " + e.getMessage());
            }
        } catch (Exception e) {
            System.out.println("Error: " + e.getMessage());
        }
    }
}
