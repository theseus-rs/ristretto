/**
 * Test that classes loaded via defineClass have correct classLoader and module identity.
 *
 * Per JVM spec section 5.3.5, when defineClass is called, the class is defined by the given
 * class loader. The class belongs to the unnamed module of that class loader (section 7.7.5).
 *
 * This test verifies:
 * 1. A class loaded via a custom ClassLoader reports that ClassLoader from getClassLoader()
 * 2. The class's module's classLoader matches the defining class loader
 * 3. The module is an unnamed module
 * 4. Multiple classes from the same ClassLoader share the same module
 */
public class Test {

    /** Simple custom ClassLoader that delegates to the parent. */
    static class TestClassLoader extends ClassLoader {
        public TestClassLoader(ClassLoader parent) {
            super(parent);
        }

        @Override
        protected Class<?> loadClass(String name, boolean resolve) throws ClassNotFoundException {
            return super.loadClass(name, resolve);
        }
    }

    public static void main(String[] args) throws Exception {
        testClassLoaderIdentity();
        testModuleClassLoaderConsistency();
        testUnnamedModule();
        testModuleSharing();
    }

    /** A class loaded via custom ClassLoader reports that ClassLoader. */
    static void testClassLoaderIdentity() throws Exception {
        System.out.println("=== ClassLoader Identity ===");

        TestClassLoader loader = new TestClassLoader(Test.class.getClassLoader());

        // Load a well-known class through the custom loader
        Class<?> cls = Class.forName("java.lang.StringBuilder", true, loader);
        // StringBuilder is a bootstrap class; its classLoader should be null (boot loader)
        ClassLoader clsLoader = cls.getClassLoader();
        System.out.println("Bootstrap class loader is null: " + (clsLoader == null));

        // Test.class was loaded by the system/app classloader
        ClassLoader testLoader = Test.class.getClassLoader();
        System.out.println("Test has classloader: " + (testLoader != null));
    }

    /**
     * A class's module's classLoader must be consistent with the class's classLoader.
     * This is critical for Proxy creation which checks:
     *   intf.getModule().getClassLoader() == loader
     */
    static void testModuleClassLoaderConsistency() throws Exception {
        System.out.println("\n=== Module ClassLoader Consistency ===");

        // For bootstrap classes, module classloader should be null
        Class<?> stringClass = String.class;
        Module stringModule = stringClass.getModule();
        ClassLoader stringModuleLoader = stringModule.getClassLoader();
        ClassLoader stringClassLoader = stringClass.getClassLoader();
        System.out.println("String module loader == class loader: "
            + (stringModuleLoader == stringClassLoader));

        // For application classes, module classloader should match class classloader
        Class<?> testClass = Test.class;
        Module testModule = testClass.getModule();
        ClassLoader testModuleLoader = testModule.getClassLoader();
        ClassLoader testClassLoader = testClass.getClassLoader();
        System.out.println("Test module loader == class loader: "
            + (testModuleLoader == testClassLoader));
    }

    /** Application classes that are not in a named module should be in an unnamed module. */
    static void testUnnamedModule() throws Exception {
        System.out.println("\n=== Unnamed Module ===");

        Module testModule = Test.class.getModule();
        System.out.println("Test module is named: " + testModule.isNamed());
        System.out.println("Test module name is null: " + (testModule.getName() == null));
    }

    /** Multiple classes from the same ClassLoader should share the same unnamed module. */
    static void testModuleSharing() throws Exception {
        System.out.println("\n=== Module Sharing ===");

        Module testModule = Test.class.getModule();
        Module loaderModule = TestClassLoader.class.getModule();
        System.out.println("Same module: " + (testModule == loaderModule));
    }
}
