import java.lang.module.ModuleFinder;
import java.lang.module.ModuleReference;

/**
 * Module Path and Resources Tests
 * Tests module path operations, resource access, and class loading within modules
 */
public class Test {
    public static void main(String[] args) {
        System.out.println("=== Module Path and Resources Tests ===");

        // Test 1: Module path analysis
        testModulePath();

        // Test 2: Resource access across modules
        testResourceAccess();

        // Test 3: Class loading within modules
        testClassLoading();

        // Test 4: Module location and source
        testModuleLocation();

        System.out.println("=== Module Path Tests Complete ===");
    }

    private static void testModulePath() {
        System.out.println("--- Test: Module Path ---");
        try {
            // Get system property for module path
            String modulePath = System.getProperty("jdk.module.path");
            System.out.println("Module path property: " + modulePath);

            String upgradeModulePath = System.getProperty("jdk.module.upgrade.path");
            System.out.println("Upgrade module path: " + upgradeModulePath);

            // Check boot module path
            String bootModulePath = System.getProperty("sun.boot.module.path");
            System.out.println("Boot module path: " + bootModulePath);

            // Test current working directory
            System.out.println("Current working directory: " + System.getProperty("user.dir"));

        } catch (Exception e) {
            System.out.println("Error testing module path: " + e.getMessage());
        }
    }

    private static void testResourceAccess() {
        System.out.println("--- Test: Resource Access ---");
        try {
            // Test resource access within current module
            Module currentModule = Test.class.getModule();
            ClassLoader loader = Test.class.getClassLoader();

            System.out.println("Current module: " + currentModule);
            System.out.println("Class loader: " + loader);

            // Try to access a common resource
            java.net.URL resource = Test.class.getResource("/Test.java");
            System.out.println("Resource /Test.java found: " + (resource != null));

            // Try relative resource
            java.net.URL relativeResource = Test.class.getResource("Test.java");
            System.out.println("Relative Test.java found: " + (relativeResource != null));

            // Test resource from java.base module
            Module javaBase = ModuleLayer.boot().findModule("java.base").orElse(null);
            if (javaBase != null) {
                try {
                    java.io.InputStream stream = javaBase.getResourceAsStream("java/lang/String.class");
                    System.out.println("String.class from java.base: " + (stream != null));
                    if (stream != null) {
                        stream.close();
                    }
                } catch (Exception e) {
                    System.out.println("Cannot access String.class: " + e.getMessage());
                }
            }

        } catch (Exception e) {
            System.out.println("Error testing resource access: " + e.getMessage());
        }
    }

    private static void testClassLoading() {
        System.out.println("--- Test: Class Loading ---");
        try {
            // Test loading classes from different modules
            Class<?> stringClass = Class.forName("java.lang.String");
            System.out.println("Loaded String class from: " + stringClass.getModule().getName());
            System.out.println("String class loader: " + stringClass.getClassLoader());

            Class<?> listClass = Class.forName("java.util.List");
            System.out.println("Loaded List class from: " + listClass.getModule().getName());

            // Test current class
            Class<?> currentClass = Test.class;
            System.out.println("Current class module: " + currentClass.getModule().getName());
            System.out.println("Current class loader: " + currentClass.getClassLoader());

            // Test primitive classes
            Class<?> intClass = int.class;
            System.out.println("int.class module: " + intClass.getModule());
            System.out.println("int.class is named module: " + intClass.getModule().isNamed());

        } catch (ClassNotFoundException e) {
            System.out.println("Class not found: " + e.getMessage());
        } catch (Exception e) {
            System.out.println("Error testing class loading: " + e.getMessage());
        }
    }

    private static void testModuleLocation() {
        System.out.println("--- Test: Module Location ---");
        try {
            ModuleFinder systemFinder = ModuleFinder.ofSystem();

            // Check locations of system modules
            String[] testModules = {"java.base", "java.logging", "java.desktop"};

            for (String moduleName : testModules) {
                java.util.Optional<ModuleReference> moduleRef = systemFinder.find(moduleName);
                if (moduleRef.isPresent()) {
                    System.out.println("Module: " + moduleName);
                    java.util.Optional<java.net.URI> location = moduleRef.get().location();
                    System.out.println("  Location: " + location.orElse(null));
                    System.out.println("  Has location: " + location.isPresent());
                } else {
                    System.out.println("Module not found: " + moduleName);
                }
            }

            // Test current module location
            Module currentModule = Test.class.getModule();
            System.out.println("Current module location info:");
            System.out.println("  Module: " + currentModule);
            System.out.println("  Is named: " + currentModule.isNamed());

        } catch (Exception e) {
            System.out.println("Error testing module location: " + e.getMessage());
        }
    }
}
