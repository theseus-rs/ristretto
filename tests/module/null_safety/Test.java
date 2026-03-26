/**
 * Module Null Safety Tests
 * Tests that module field maps (exportedPackages, openPackages) are never null,
 * and that Configuration/ModuleLayer parent hierarchies are properly initialized.
 */
public class Test {
    public static void main(String[] args) {
        System.out.println("=== Module Null Safety Tests ===");

        testIsExportedOnModuleWithNoExports();
        testIsOpenOnModuleWithNoOpens();
        testConfigurationParents();
        testModuleLayerParents();

        System.out.println("=== Module Null Safety Tests Complete ===");
    }

    /**
     * Test that Module.isExported() works on modules that have no explicit exports.
     * If exportedPackages is null, this would throw NullPointerException.
     */
    private static void testIsExportedOnModuleWithNoExports() {
        System.out.println("--- Test: isExported on module with no exports ---");
        try {
            ModuleLayer bootLayer = ModuleLayer.boot();
            boolean foundModuleWithNoExports = false;

            // Sort modules by name for deterministic output across JVM implementations
            java.util.List<Module> sorted = bootLayer.modules().stream()
                .filter(m -> m.getDescriptor() != null && m.getDescriptor().exports().isEmpty())
                .sorted(java.util.Comparator.comparing(Module::getName))
                .collect(java.util.stream.Collectors.toList());

            if (!sorted.isEmpty()) {
                Module m = sorted.get(0);
                // This module has no exports; calling isExported must not NPE
                boolean result = m.isExported("nonexistent.pkg");
                System.out.println(m.getName() + ".isExported(\"nonexistent.pkg\"): " + result);
                foundModuleWithNoExports = true;
            }
            System.out.println("Found module with no exports: " + foundModuleWithNoExports);
        } catch (NullPointerException e) {
            System.out.println("NullPointerException: exportedPackages is null");
        } catch (Exception e) {
            System.out.println("Error: " + e.getClass().getName() + ": " + e.getMessage());
        }
    }

    /**
     * Test that Module.isOpen() works on modules that have no explicit opens.
     * If openPackages is null, this would throw NullPointerException.
     */
    private static void testIsOpenOnModuleWithNoOpens() {
        System.out.println("--- Test: isOpen on module with no opens ---");
        try {
            Module javaBase = ModuleLayer.boot().findModule("java.base").orElse(null);
            if (javaBase == null) {
                System.out.println("java.base not found");
                return;
            }

            // java.base has no unqualified opens; calling isOpen must not NPE
            boolean result = javaBase.isOpen("java.lang");
            System.out.println("java.base.isOpen(\"java.lang\"): " + result);

            // Also test isOpen with a target module
            Module javaLogging = ModuleLayer.boot().findModule("java.logging").orElse(null);
            if (javaLogging != null) {
                boolean result2 = javaBase.isOpen("java.lang", javaLogging);
                System.out.println("java.base.isOpen(\"java.lang\", java.logging): " + result2);
            }
            System.out.println("No NullPointerException thrown");
        } catch (NullPointerException e) {
            System.out.println("NullPointerException: openPackages is null");
        } catch (Exception e) {
            System.out.println("Error: " + e.getClass().getName() + ": " + e.getMessage());
        }
    }

    /**
     * Test that Configuration.parents() returns a non-empty list containing
     * Configuration.empty() for the boot layer configuration.
     */
    private static void testConfigurationParents() {
        System.out.println("--- Test: Configuration parents ---");
        try {
            java.lang.module.Configuration bootConfig = ModuleLayer.boot().configuration();
            System.out.println("Boot configuration exists: " + (bootConfig != null));

            java.util.List<java.lang.module.Configuration> parents = bootConfig.parents();
            System.out.println("Configuration parents count: " + parents.size());
            System.out.println("Configuration has parent: " + !parents.isEmpty());

            if (!parents.isEmpty()) {
                java.lang.module.Configuration parent = parents.get(0);
                // The parent should be Configuration.empty()
                System.out.println("Parent is Configuration.empty(): " +
                    (parent == java.lang.module.Configuration.empty()));
                System.out.println("Parent modules empty: " + parent.modules().isEmpty());
            }
        } catch (Exception e) {
            System.out.println("Error: " + e.getClass().getName() + ": " + e.getMessage());
        }
    }

    /**
     * Test that ModuleLayer.parents() returns a non-empty list containing
     * ModuleLayer.empty() for the boot layer.
     */
    private static void testModuleLayerParents() {
        System.out.println("--- Test: ModuleLayer parents ---");
        try {
            ModuleLayer bootLayer = ModuleLayer.boot();
            java.util.List<ModuleLayer> parents = bootLayer.parents();
            System.out.println("ModuleLayer parents count: " + parents.size());
            System.out.println("ModuleLayer has parent: " + !parents.isEmpty());

            if (!parents.isEmpty()) {
                ModuleLayer parent = parents.get(0);
                // The parent should be ModuleLayer.empty()
                System.out.println("Parent is ModuleLayer.empty(): " +
                    (parent == ModuleLayer.empty()));
                System.out.println("Parent modules empty: " + parent.modules().isEmpty());
            }
        } catch (Exception e) {
            System.out.println("Error: " + e.getClass().getName() + ": " + e.getMessage());
        }
    }
}
