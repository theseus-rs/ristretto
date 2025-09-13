import java.lang.module.Configuration;
import java.lang.module.ResolvedModule;

/**
 * Module Dependencies and Requirements Tests
 * Tests module dependencies, requires statements, and transitive dependencies
 */
public class Test {
    public static void main(String[] args) {
        System.out.println("=== Module Dependencies and Requirements Tests ===");

        // Test 1: Module requires analysis
        testModuleRequires();

        // Test 2: Transitive dependencies
        testTransitiveDependencies();

        // Test 3: Static dependencies
        testStaticDependencies();

        // Test 4: Dependency resolution
        testDependencyResolution();

        System.out.println("=== Module Dependencies Tests Complete ===");
    }

    private static void testModuleRequires() {
        System.out.println("--- Test: Module Requires ---");
        try {
            // Analyze java.base requires (should be minimal)
            Module javaBase = ModuleLayer.boot().findModule("java.base").orElse(null);
            if (javaBase != null && javaBase.getDescriptor() != null) {
                System.out.println("java.base requires:");
                javaBase.getDescriptor().requires().forEach(requires -> {
                    System.out.println("  " + requires.name() +
                        (requires.modifiers().contains(java.lang.module.ModuleDescriptor.Requires.Modifier.TRANSITIVE) ? " (transitive)" : "") +
                        (requires.modifiers().contains(java.lang.module.ModuleDescriptor.Requires.Modifier.STATIC) ? " (static)" : ""));
                });
                System.out.println("java.base requires count: " + javaBase.getDescriptor().requires().size());
            }

            // Analyze java.logging requires (more complex)
            Module javaLogging = ModuleLayer.boot().findModule("java.logging").orElse(null);
            if (javaLogging != null && javaLogging.getDescriptor() != null) {
                System.out.println("java.logging requires:");
                javaLogging.getDescriptor().requires().forEach(requires -> {
                    System.out.println("  " + requires.name() +
                        (requires.modifiers().contains(java.lang.module.ModuleDescriptor.Requires.Modifier.TRANSITIVE) ? " (transitive)" : "") +
                        (requires.modifiers().contains(java.lang.module.ModuleDescriptor.Requires.Modifier.STATIC) ? " (static)" : ""));
                });
            }

        } catch (Exception e) {
            System.out.println("Error testing module requires: " + e.getMessage());
        }
    }

    private static void testTransitiveDependencies() {
        System.out.println("--- Test: Transitive Dependencies ---");
        try {
            ModuleLayer bootLayer = ModuleLayer.boot();

            // Look for modules with transitive dependencies
            bootLayer.modules().stream()
                .filter(Module::isNamed)
                .filter(m -> m.getDescriptor() != null)
                .filter(m -> m.getDescriptor().requires().stream()
                    .anyMatch(req -> req.modifiers().contains(
                        java.lang.module.ModuleDescriptor.Requires.Modifier.TRANSITIVE)))
                .limit(3)
                .forEach(module -> {
                    System.out.println("Module " + module.getName() + " has transitive requires:");
                    module.getDescriptor().requires().stream()
                        .filter(req -> req.modifiers().contains(
                            java.lang.module.ModuleDescriptor.Requires.Modifier.TRANSITIVE))
                        .forEach(req -> System.out.println("  " + req.name() + " (transitive)"));
                });

        } catch (Exception e) {
            System.out.println("Error testing transitive dependencies: " + e.getMessage());
        }
    }

    private static void testStaticDependencies() {
        System.out.println("--- Test: Static Dependencies ---");
        try {
            ModuleLayer bootLayer = ModuleLayer.boot();

            // Look for modules with static dependencies
            long staticCount = bootLayer.modules().stream()
                .filter(Module::isNamed)
                .filter(m -> m.getDescriptor() != null)
                .mapToLong(m -> m.getDescriptor().requires().stream()
                    .filter(req -> req.modifiers().contains(
                        java.lang.module.ModuleDescriptor.Requires.Modifier.STATIC))
                    .count())
                .sum();

            System.out.println("Total static dependencies in boot layer: " + staticCount);

            // Find specific modules with static dependencies
            bootLayer.modules().stream()
                .filter(Module::isNamed)
                .filter(m -> m.getDescriptor() != null)
                .filter(m -> m.getDescriptor().requires().stream()
                    .anyMatch(req -> req.modifiers().contains(
                        java.lang.module.ModuleDescriptor.Requires.Modifier.STATIC)))
                .limit(2)
                .forEach(module -> {
                    System.out.println("Module " + module.getName() + " has static requires:");
                    module.getDescriptor().requires().stream()
                        .filter(req -> req.modifiers().contains(
                            java.lang.module.ModuleDescriptor.Requires.Modifier.STATIC))
                        .forEach(req -> System.out.println("  " + req.name() + " (static)"));
                });

        } catch (Exception e) {
            System.out.println("Error testing static dependencies: " + e.getMessage());
        }
    }

    private static void testDependencyResolution() {
        System.out.println("--- Test: Dependency Resolution ---");
        try {
            ModuleLayer bootLayer = ModuleLayer.boot();
            Configuration config = bootLayer.configuration();

            // Test if we can find resolved modules
            java.util.Optional<ResolvedModule> javaBaseResolved =
                config.findModule("java.base");
            System.out.println("java.base resolved: " + javaBaseResolved.isPresent());

            if (javaBaseResolved.isPresent()) {
                ResolvedModule resolved = javaBaseResolved.get();
                System.out.println("java.base reads " + resolved.reads().size() + " modules");
                System.out.println("First 3 modules java.base reads:");
                resolved.reads().stream()
                    .limit(3)
                    .forEach(read -> System.out.println("  " + read.name()));
            }

            // Count total resolved modules
            System.out.println("Total resolved modules in configuration: " +
                config.modules().size());

        } catch (Exception e) {
            System.out.println("Error testing dependency resolution: " + e.getMessage());
        }
    }
}
