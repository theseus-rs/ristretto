import java.lang.module.Configuration;
import java.lang.module.ModuleFinder;
import java.lang.module.ModuleReference;

/**
 * Module Layer and Configuration Tests
 * Tests module layer operations, configuration, and boot layer functionality
 */
public class Test {
    public static void main(String[] args) {
        System.out.println("=== Module Layer and Configuration Tests ===");

        // Test 1: Boot layer analysis
        testBootLayer();

        // Test 2: Module configuration
        testModuleConfiguration();

        // Test 3: Module finder operations
        testModuleFinder();

        // Test 4: Layer hierarchy
        testLayerHierarchy();

        System.out.println("=== Module Layer Tests Complete ===");
    }

    private static void testBootLayer() {
        System.out.println("--- Test: Boot Layer Analysis ---");
        try {
            ModuleLayer bootLayer = ModuleLayer.boot();
            System.out.println("Boot layer: " + bootLayer);
            System.out.println("Boot layer parents: " + bootLayer.parents());
            System.out.println("Boot layer is empty: " + bootLayer.parents().isEmpty());

            // Count modules by type
            long namedModules = bootLayer.modules().stream()
                .filter(Module::isNamed)
                .count();
            long unnamedModules = bootLayer.modules().stream()
                .filter(m -> !m.isNamed())
                .count();

            System.out.println("Named modules in boot layer: " + namedModules);
            System.out.println("Unnamed modules in boot layer: " + unnamedModules);
            System.out.println("Total modules in boot layer: " + bootLayer.modules().size());

        } catch (Exception e) {
            System.out.println("Error analyzing boot layer: " + e.getMessage());
        }
    }

    private static void testModuleConfiguration() {
        System.out.println("--- Test: Module Configuration ---");
        try {
            ModuleLayer bootLayer = ModuleLayer.boot();
            Configuration config = bootLayer.configuration();
            System.out.println("Boot layer configuration: " + config);
            System.out.println("Configuration parents: " + config.parents());

            // List some modules in configuration
            System.out.println("Modules in configuration (first 10):");
            config.modules().stream()
                .limit(10)
                .forEach(resolved -> {
                    System.out.println("  " + resolved.name());
                    System.out.println("    reads: " + resolved.reads().size() + " modules");
                });

        } catch (Exception e) {
            System.out.println("Error testing configuration: " + e.getMessage());
        }
    }

    private static void testModuleFinder() {
        System.out.println("--- Test: Module Finder ---");
        try {
            // Test system module finder
            ModuleFinder systemFinder = ModuleFinder.ofSystem();
            System.out.println("System module finder: " + systemFinder);

            // Find specific modules
            java.util.Optional<ModuleReference> javaBase = systemFinder.find("java.base");
            System.out.println("Found java.base: " + javaBase.isPresent());

            if (javaBase.isPresent()) {
                ModuleReference ref = javaBase.get();
                System.out.println("java.base descriptor: " + ref.descriptor().name());
                System.out.println("java.base location: " + ref.location().orElse(null));
            }

            // Try to find a non-existent module
            java.util.Optional<ModuleReference> nonExistent = systemFinder.find("non.existent.module");
            System.out.println("Found non-existent module: " + nonExistent.isPresent());

        } catch (Exception e) {
            System.out.println("Error testing module finder: " + e.getMessage());
        }
    }

    private static void testLayerHierarchy() {
        System.out.println("--- Test: Layer Hierarchy ---");
        try {
            ModuleLayer bootLayer = ModuleLayer.boot();

            // Check if current class is in boot layer
            Module currentModule = Test.class.getModule();
            boolean inBootLayer = bootLayer.modules().contains(currentModule);
            System.out.println("Current module in boot layer: " + inBootLayer);
            System.out.println("Current module layer: " + currentModule.getLayer());

            // Compare layers
            if (currentModule.getLayer() != null) {
                System.out.println("Current layer equals boot layer: " +
                    currentModule.getLayer().equals(bootLayer));
                System.out.println("Current layer parents: " +
                    currentModule.getLayer().parents());
            }

        } catch (Exception e) {
            System.out.println("Error testing layer hierarchy: " + e.getMessage());
        }
    }
}
