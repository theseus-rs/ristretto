/**
 * Module Annotations and Metadata Tests
 * Tests module annotations, version information, and metadata access
 */
public class Test {
    public static void main(String[] args) {
        System.out.println("=== Module Annotations and Metadata Tests ===");

        // Test 1: Module annotations
        testModuleAnnotations();

        // Test 2: Module version information
        testModuleVersions();

        // Test 3: Module main class
        testModuleMainClass();

        // Test 4: Module packages
        testModulePackages();

        System.out.println("=== Module Annotations Tests Complete ===");
    }

    private static void testModuleAnnotations() {
        System.out.println("--- Test: Module Annotations ---");
        try {
            ModuleLayer bootLayer = ModuleLayer.boot();

            // Check annotations on modules
            bootLayer.modules().stream()
                .filter(Module::isNamed)
                .filter(m -> m.getDescriptor() != null)
                .limit(5)
                .forEach(module -> {
                    System.out.println("Module: " + module.getName());

                    // Check for common annotations
                    java.lang.annotation.Annotation[] annotations = module.getAnnotations();
                    System.out.println("  Annotations count: " + annotations.length);

                    for (java.lang.annotation.Annotation annotation : annotations) {
                        System.out.println("  Annotation: " + annotation.annotationType().getSimpleName());
                    }

                    // Check if deprecated
                    boolean isDeprecated = module.isAnnotationPresent(Deprecated.class);
                    System.out.println("  Is deprecated: " + isDeprecated);
                });

        } catch (Exception e) {
            System.out.println("Error testing module annotations: " + e.getMessage());
        }
    }

    private static void testModuleVersions() {
        System.out.println("--- Test: Module Versions ---");
        try {
            ModuleLayer bootLayer = ModuleLayer.boot();

            // Check version information
            bootLayer.modules().stream()
                .filter(Module::isNamed)
                .filter(m -> m.getDescriptor() != null)
                .limit(5)
                .forEach(module -> {
                    System.out.println("Module: " + module.getName());

                    java.util.Optional<java.lang.module.ModuleDescriptor.Version> version =
                        module.getDescriptor().version();
                    System.out.println("  Has version: " + version.isPresent());
                    if (version.isPresent()) {
                        System.out.println("  Version: " + version.get());
                    }

                    // Check raw version string
                    java.util.Optional<String> rawVersion = module.getDescriptor().rawVersion();
                    System.out.println("  Has raw version: " + rawVersion.isPresent());
                    if (rawVersion.isPresent()) {
                        System.out.println("  Raw version: " + rawVersion.get());
                    }
                });

        } catch (Exception e) {
            System.out.println("Error testing module versions: " + e.getMessage());
        }
    }

    private static void testModuleMainClass() {
        System.out.println("--- Test: Module Main Class ---");
        try {
            ModuleLayer bootLayer = ModuleLayer.boot();

            // Check for modules with main classes
            bootLayer.modules().stream()
                .filter(Module::isNamed)
                .filter(m -> m.getDescriptor() != null)
                .filter(m -> m.getDescriptor().mainClass().isPresent())
                .limit(3)
                .forEach(module -> {
                    System.out.println("Module: " + module.getName());
                    System.out.println("  Main class: " + module.getDescriptor().mainClass().get());
                });

            // Count modules with main classes
            long mainClassCount = bootLayer.modules().stream()
                .filter(Module::isNamed)
                .filter(m -> m.getDescriptor() != null)
                .filter(m -> m.getDescriptor().mainClass().isPresent())
                .count();
            System.out.println("Total modules with main class: " + mainClassCount);

        } catch (Exception e) {
            System.out.println("Error testing module main class: " + e.getMessage());
        }
    }

    private static void testModulePackages() {
        System.out.println("--- Test: Module Packages ---");
        try {
            Module javaBase = ModuleLayer.boot().findModule("java.base").orElse(null);
            if (javaBase != null && javaBase.getDescriptor() != null) {
                System.out.println("java.base packages:");
                System.out.println("  Total packages: " + javaBase.getDescriptor().packages().size());

                // Show first 10 packages
                javaBase.getDescriptor().packages().stream()
                    .limit(10)
                    .sorted()
                    .forEach(pkg -> System.out.println("  " + pkg));

                // Check if specific packages are present
                boolean hasJavaLang = javaBase.getDescriptor().packages().contains("java.lang");
                boolean hasJavaUtil = javaBase.getDescriptor().packages().contains("java.util");
                System.out.println("  Contains java.lang: " + hasJavaLang);
                System.out.println("  Contains java.util: " + hasJavaUtil);
            }

            // Test current module packages
            Module currentModule = Test.class.getModule();
            System.out.println("Current module packages:");
            if (currentModule.getDescriptor() != null) {
                System.out.println("  Package count: " + currentModule.getDescriptor().packages().size());
            } else {
                System.out.println("  No descriptor (unnamed module)");
                System.out.println("  Current package: " + Test.class.getPackage());
            }

        } catch (Exception e) {
            System.out.println("Error testing module packages: " + e.getMessage());
        }
    }
}
