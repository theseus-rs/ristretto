/**
 * Basic Module System Tests
 * Tests fundamental module operations and introspection
 */
public class Test {
    public static void main(String[] args) {
        System.out.println("=== Basic Module System Tests ===");

        // Test 1: Get current module
        testCurrentModule();

        // Test 2: Test unnamed module
        testUnnamedModule();

        // Test 3: Test module layer
        testModuleLayer();

        // Test 4: Test module descriptor access
        testModuleDescriptor();

        System.out.println("=== Basic Module Tests Complete ===");
    }

    private static void testCurrentModule() {
        System.out.println("--- Test: Current Module ---");
        try {
            Module currentModule = Test.class.getModule();
            System.out.println("Current module: " + currentModule.getName());
            System.out.println("Is named: " + currentModule.isNamed());
            System.out.println("Module string: " + currentModule.toString());
        } catch (Exception e) {
            System.out.println("Error getting current module: " + e.getMessage());
        }
    }

    private static void testUnnamedModule() {
        System.out.println("--- Test: Unnamed Module ---");
        try {
            Module unnamedModule = Test.class.getModule();
            System.out.println("Unnamed module name: " + unnamedModule.getName());
            System.out.println("Is unnamed module: " + !unnamedModule.isNamed());
            System.out.println("Unnamed module layer: " + unnamedModule.getLayer());
        } catch (Exception e) {
            System.out.println("Error with unnamed module: " + e.getMessage());
        }
    }

    private static void testModuleLayer() {
        System.out.println("--- Test: Module Layer ---");
        try {
            ModuleLayer bootLayer = ModuleLayer.boot();
            System.out.println("Boot layer: " + bootLayer);
            System.out.println("Boot layer modules count: " + bootLayer.modules().size());

            // List some boot layer modules
            System.out.println("First 5 boot layer modules:");
            bootLayer.modules().stream()
                .limit(5)
                .forEach(m -> System.out.println("  " + m.getName()));

        } catch (Exception e) {
            System.out.println("Error with module layer: " + e.getMessage());
        }
    }

    private static void testModuleDescriptor() {
        System.out.println("--- Test: Module Descriptor ---");
        try {
            // Test with java.base module
            Module javaBase = ModuleLayer.boot().findModule("java.base").orElse(null);
            if (javaBase != null) {
                System.out.println("java.base module found: " + javaBase.getName());
                if (javaBase.getDescriptor() != null) {
                    System.out.println("java.base is open: " + javaBase.getDescriptor().isOpen());
                    System.out.println("java.base is automatic: " + javaBase.getDescriptor().isAutomatic());
                    System.out.println("java.base exports count: " + javaBase.getDescriptor().exports().size());
                }
            } else {
                System.out.println("java.base module not found");
            }
        } catch (Exception e) {
            System.out.println("Error with module descriptor: " + e.getMessage());
        }
    }
}
