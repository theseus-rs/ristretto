/**
 * Module Security and Access Control Tests
 * Tests module security policies, access controls, and reflection restrictions
 */
public class Test {
    public static void main(String[] args) {
        System.out.println("=== Module Security and Access Control Tests ===");

        // Test 1: Reflection access controls
        testReflectionAccess();

        // Test 2: Package access restrictions
        testPackageAccess();

        // Test 3: Module opens directive
        testModuleOpens();

        // Test 4: Deep reflection attempts
        testDeepReflection();

        System.out.println("=== Module Security Tests Complete ===");
    }

    private static void testReflectionAccess() {
        System.out.println("--- Test: Reflection Access ---");
        try {
            // Test reflection on public API
            Class<?> stringClass = String.class;
            System.out.println("String class accessible: " + (stringClass != null));

            // Test getting public methods
            java.lang.reflect.Method[] publicMethods = stringClass.getMethods();
            System.out.println("String public methods count: " + publicMethods.length);

            // Test getting declared methods (should work for public classes)
            java.lang.reflect.Method[] declaredMethods = stringClass.getDeclaredMethods();
            System.out.println("String declared methods count: " + declaredMethods.length);

            // Test field access
            java.lang.reflect.Field[] fields = stringClass.getDeclaredFields();
            System.out.println("String declared fields count: " + fields.length);

        } catch (SecurityException e) {
            System.out.println("Security exception during reflection: " + e.getMessage());
        } catch (Exception e) {
            System.out.println("Error testing reflection access: " + e.getMessage());
        }
    }

    private static void testPackageAccess() {
        System.out.println("--- Test: Package Access ---");
        try {
            // Test access to internal packages
            Package[] packages = Package.getPackages();
            System.out.println("Total accessible packages: " + packages.length);

            // Look for internal packages
            boolean hasInternalPackages = false;
            for (Package pkg : packages) {
                if (pkg.getName().contains(".internal") || pkg.getName().contains(".impl")) {
                    hasInternalPackages = true;
                    System.out.println("Internal package found: " + pkg.getName());
                    break;
                }
            }
            System.out.println("Has internal packages visible: " + hasInternalPackages);

            // Test specific package access
            Package javaLangPackage = Package.getPackage("java.lang");
            System.out.println("java.lang package: " + javaLangPackage);
            if (javaLangPackage != null) {
                System.out.println("  Implementation title: " + javaLangPackage.getImplementationTitle());
                System.out.println("  Implementation version: " + javaLangPackage.getImplementationVersion());
            }

        } catch (Exception e) {
            System.out.println("Error testing package access: " + e.getMessage());
        }
    }

    private static void testModuleOpens() {
        System.out.println("--- Test: Module Opens ---");
        try {
            ModuleLayer bootLayer = ModuleLayer.boot();

            // Look for modules with opens directives
            bootLayer.modules().stream()
                .filter(Module::isNamed)
                .filter(m -> m.getDescriptor() != null)
                .filter(m -> !m.getDescriptor().opens().isEmpty())
                .limit(3)
                .forEach(module -> {
                    System.out.println("Module " + module.getName() + " opens:");
                    module.getDescriptor().opens().forEach(opens -> {
                        System.out.println("  Package: " + opens.source());
                        if (opens.isQualified()) {
                            System.out.println("    To modules: " + opens.targets());
                        } else {
                            System.out.println("    To all modules");
                        }
                    });
                });

            // Test if current module is open
            Module currentModule = Test.class.getModule();
            if (currentModule.getDescriptor() != null) {
                System.out.println("Current module opens count: " +
                    currentModule.getDescriptor().opens().size());
                System.out.println("Current module is open: " +
                    currentModule.getDescriptor().isOpen());
            } else {
                System.out.println("Current module has no descriptor (unnamed)");
            }

        } catch (Exception e) {
            System.out.println("Error testing module opens: " + e.getMessage());
        }
    }

    private static void testDeepReflection() {
        System.out.println("--- Test: Deep Reflection ---");
        try {
            // Test setAccessible on private fields
            Class<?> stringClass = String.class;
            java.lang.reflect.Field[] fields = stringClass.getDeclaredFields();

            if (fields.length > 0) {
                java.lang.reflect.Field firstField = fields[0];
                System.out.println("Testing field: " + firstField.getName());
                System.out.println("Field accessible: " + firstField.isAccessible());

                try {
                    firstField.setAccessible(true);
                    System.out.println("setAccessible(true) succeeded");
                } catch (Exception e) {
                    System.out.println("setAccessible(true) failed: " + e.getMessage());
                }
            }

            // Test accessing constructor
            try {
                java.lang.reflect.Constructor<String> constructor =
                    String.class.getDeclaredConstructor(char[].class);
                System.out.println("Found char[] constructor: " + (constructor != null));
                constructor.setAccessible(true);
                System.out.println("Constructor setAccessible succeeded");
            } catch (Exception e) {
                System.out.println("Constructor access failed: " + e.getMessage());
            }

        } catch (Exception e) {
            System.out.println("Error testing deep reflection: " + e.getMessage());
        }
    }
}
