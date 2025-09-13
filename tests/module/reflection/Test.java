/**
 * Module Reflection and Access Tests
 * Tests module reflection capabilities, accessibility, and exports
 */
public class Test {
    public static void main(String[] args) {
        System.out.println("=== Module Reflection and Access Tests ===");

        // Test 1: Module accessibility
        testModuleAccessibility();

        // Test 2: Package exports
        testPackageExports();

        // Test 3: Module reads
        testModuleReads();

        // Test 4: Class access across modules
        testCrossModuleAccess();

        System.out.println("=== Module Reflection Tests Complete ===");
    }

    private static void testModuleAccessibility() {
        System.out.println("--- Test: Module Accessibility ---");
        try {
            Module currentModule = Test.class.getModule();
            Module javaBase = ModuleLayer.boot().findModule("java.base").orElse(null);

            if (javaBase != null) {
                System.out.println("Can read java.base: " + currentModule.canRead(javaBase));
                System.out.println("java.base can read current: " + javaBase.canRead(currentModule));

                // Test package access
                String javaLangPackage = "java.lang";
                System.out.println("java.base exports java.lang: " +
                    javaBase.isExported(javaLangPackage));
                System.out.println("java.base exports java.lang to current: " +
                    javaBase.isExported(javaLangPackage, currentModule));
            }
        } catch (Exception e) {
            System.out.println("Error testing accessibility: " + e.getMessage());
        }
    }

    private static void testPackageExports() {
        System.out.println("--- Test: Package Exports ---");
        try {
            Module javaBase = ModuleLayer.boot().findModule("java.base").orElse(null);
            if (javaBase != null && javaBase.getDescriptor() != null) {
                System.out.println("java.base exported packages:");
                javaBase.getDescriptor().exports().stream()
                    .limit(10)
                    .forEach(export -> {
                        System.out.println("  " + export.source() +
                            (export.isQualified() ? " (qualified)" : " (unqualified)"));
                        if (export.isQualified()) {
                            System.out.println("    targets: " + export.targets());
                        }
                    });
            }
        } catch (Exception e) {
            System.out.println("Error testing package exports: " + e.getMessage());
        }
    }

    private static void testModuleReads() {
        System.out.println("--- Test: Module Reads ---");
        try {
            Module currentModule = Test.class.getModule();

            // Test what the current module reads
            System.out.println("Current module reads:");
            ModuleLayer.boot().modules().stream()
                .filter(currentModule::canRead)
                .limit(10)
                .forEach(m -> System.out.println("  " + m.getName()));

        } catch (Exception e) {
            System.out.println("Error testing module reads: " + e.getMessage());
        }
    }

    private static void testCrossModuleAccess() {
        System.out.println("--- Test: Cross Module Access ---");
        try {
            // Test access to java.lang.String (should work)
            Class<?> stringClass = Class.forName("java.lang.String");
            System.out.println("Can access String class: " + (stringClass != null));
            System.out.println("String module: " + stringClass.getModule().getName());

            // Test access to java.util.List (should work)
            Class<?> listClass = Class.forName("java.util.List");
            System.out.println("Can access List class: " + (listClass != null));
            System.out.println("List module: " + listClass.getModule().getName());

        } catch (ClassNotFoundException e) {
            System.out.println("Class not found: " + e.getMessage());
        } catch (Exception e) {
            System.out.println("Error testing cross module access: " + e.getMessage());
        }
    }
}
