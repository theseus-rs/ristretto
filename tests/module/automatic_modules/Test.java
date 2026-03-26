import java.lang.module.Configuration;
import java.lang.module.ModuleDescriptor;
import java.lang.module.ModuleFinder;
import java.lang.module.ModuleReference;
import java.util.Set;

/**
 * Automatic Module Tests
 * Tests automatic module detection, naming, and behavior.
 * Automatic modules are JARs placed on the module path without a module-info.class.
 * Per the JPMS spec, they export all packages, read all other modules, and their
 * name is derived from the JAR file name.
 */
public class Test {
    public static void main(String[] args) {
        System.out.println("=== Automatic Module Tests ===");

        testSystemModulesNotAutomatic();
        testAutomaticModuleDescriptorProperties();
        testModuleDescriptorBuilder();
        testAutomaticModuleNameDerivation();

        System.out.println("=== Automatic Module Tests Complete ===");
    }

    /**
     * Verify that all system modules in the boot layer are NOT automatic modules.
     * System modules always have a module-info.class.
     */
    private static void testSystemModulesNotAutomatic() {
        System.out.println("--- Test: System Modules Are Not Automatic ---");
        try {
            ModuleFinder systemFinder = ModuleFinder.ofSystem();
            long totalModules = 0;
            long automaticCount = 0;

            for (ModuleReference ref : systemFinder.findAll()) {
                totalModules++;
                if (ref.descriptor().isAutomatic()) {
                    automaticCount++;
                    System.out.println("  Automatic system module: " + ref.descriptor().name());
                }
            }

            System.out.println("Total system modules: " + totalModules);
            System.out.println("Automatic system modules: " + automaticCount);
            System.out.println("All system modules are explicit: " + (automaticCount == 0));
        } catch (Exception e) {
            System.out.println("Error: " + e.getMessage());
        }
    }

    /**
     * Verify properties of automatic vs explicit module descriptors.
     * Automatic modules have isAutomatic()=true and isOpen()=true.
     */
    private static void testAutomaticModuleDescriptorProperties() {
        System.out.println("--- Test: Automatic Module Descriptor Properties ---");
        try {
            // java.base should not be automatic
            Module javaBase = ModuleLayer.boot().findModule("java.base").orElse(null);
            if (javaBase != null) {
                ModuleDescriptor desc = javaBase.getDescriptor();
                System.out.println("java.base isAutomatic: " + desc.isAutomatic());
                System.out.println("java.base isOpen: " + desc.isOpen());
                System.out.println("java.base modifiers: " + desc.modifiers());
            }

            // Check a few more system modules
            String[] moduleNames = {"java.logging", "java.xml", "java.sql"};
            for (String name : moduleNames) {
                Module mod = ModuleLayer.boot().findModule(name).orElse(null);
                if (mod != null && mod.getDescriptor() != null) {
                    System.out.println(name + " isAutomatic: " + mod.getDescriptor().isAutomatic());
                }
            }

            // The unnamed module (classpath) should not have a descriptor
            Module unnamed = Test.class.getModule();
            System.out.println("Test class module is named: " + unnamed.isNamed());
            System.out.println("Test class module descriptor: " + unnamed.getDescriptor());
        } catch (Exception e) {
            System.out.println("Error: " + e.getMessage());
        }
    }

    /**
     * Test the ModuleDescriptor.newModule builder with automatic module type.
     * This verifies the API for creating automatic module descriptors.
     */
    private static void testModuleDescriptorBuilder() {
        System.out.println("--- Test: Module Descriptor Builder ---");
        try {
            // Build an explicit (non-automatic) module descriptor
            ModuleDescriptor explicit = ModuleDescriptor.newModule("com.example.explicit")
                .exports("com.example.api")
                .requires("java.base")
                .build();

            System.out.println("Explicit module name: " + explicit.name());
            System.out.println("Explicit module isAutomatic: " + explicit.isAutomatic());
            System.out.println("Explicit module isOpen: " + explicit.isOpen());
            System.out.println("Explicit module exports: " + explicit.exports().size());
            System.out.println("Explicit module requires: " + explicit.requires().size());

            // Build an automatic module descriptor
            ModuleDescriptor automatic = ModuleDescriptor.newAutomaticModule("com.example.auto")
                .packages(Set.of("com.example.auto.internal", "com.example.auto.api"))
                .build();

            System.out.println("Automatic module name: " + automatic.name());
            System.out.println("Automatic module isAutomatic: " + automatic.isAutomatic());
            System.out.println("Automatic module isOpen: " + automatic.isOpen());
            System.out.println("Automatic module packages: " + automatic.packages().stream()
                .sorted().collect(java.util.stream.Collectors.joining(", ")));

            // Build an open module descriptor
            ModuleDescriptor open = ModuleDescriptor.newOpenModule("com.example.open")
                .requires("java.base")
                .build();

            System.out.println("Open module name: " + open.name());
            System.out.println("Open module isAutomatic: " + open.isAutomatic());
            System.out.println("Open module isOpen: " + open.isOpen());
        } catch (Exception e) {
            System.out.println("Error: " + e.getClass().getSimpleName() + ": " + e.getMessage());
        }
    }

    /**
     * Test that automatic module name derivation follows the JPMS naming rules:
     * 1. Replace non-alphanumeric chars (except dots) with dots
     * 2. Remove leading/trailing dots
     * 3. Collapse consecutive dots
     * This is tested via the ModuleDescriptor.newAutomaticModule API.
     */
    private static void testAutomaticModuleNameDerivation() {
        System.out.println("--- Test: Automatic Module Name Properties ---");
        try {
            // Test valid automatic module names
            String[] validNames = {
                "my.auto.module",
                "com.example",
                "simple"
            };

            for (String name : validNames) {
                try {
                    ModuleDescriptor desc = ModuleDescriptor.newAutomaticModule(name).build();
                    System.out.println("Automatic module '" + name + "' created: " + desc.name());
                } catch (IllegalArgumentException e) {
                    System.out.println("Automatic module '" + name + "' rejected: " +
                        e.getClass().getSimpleName());
                }
            }

            // Test invalid automatic module names
            String[] invalidNames = {
                "",        // empty
                ".leading.dot",
                "trailing.dot."
            };

            for (String name : invalidNames) {
                try {
                    ModuleDescriptor desc = ModuleDescriptor.newAutomaticModule(name).build();
                    System.out.println("Automatic module '" + name + "' created: " + desc.name());
                } catch (IllegalArgumentException e) {
                    System.out.println("Automatic module '" + name + "' rejected: " +
                        e.getClass().getSimpleName());
                }
            }
        } catch (Exception e) {
            System.out.println("Error: " + e.getClass().getSimpleName() + ": " + e.getMessage());
        }
    }
}
