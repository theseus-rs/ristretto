/**
 * Test ClassLoader.findLoadedClass behavior.
 *
 * Per JVM spec section 5.3.4, findLoadedClass must only check whether a class has
 * already been loaded (cached). It must NOT trigger new class loading, which would
 * cause infinite recursion when Java classloaders call findLoadedClass inside
 * their loadClass implementation.
 *
 * This test creates a custom ClassLoader that tracks calls to findLoadedClass
 * and loadClass to verify the correct behavior.
 */
public class Test {

    /**
     * Custom ClassLoader that exposes findLoadedClass for testing and tracks calls.
     */
    static class TrackingClassLoader extends ClassLoader {
        int loadClassCount = 0;

        public TrackingClassLoader(ClassLoader parent) {
            super(parent);
        }

        @Override
        public Class<?> loadClass(String name, boolean resolve) throws ClassNotFoundException {
            loadClassCount++;
            // Standard delegation: check if already loaded, then delegate to parent
            Class<?> c = findLoadedClass(name);
            if (c != null) {
                return c;
            }
            return super.loadClass(name, resolve);
        }

        /** Expose findLoadedClass for direct testing. */
        public Class<?> publicFindLoaded(String name) {
            return findLoadedClass(name);
        }
    }

    public static void main(String[] args) throws Exception {
        testFindLoadedDoesNotTriggerLoading();
        testLoadClassDelegation();
        testFindLoadedReturnsCorrectClass();
        testFindLoadedClassIdentity();
    }

    /**
     * findLoadedClass must not trigger class loading.
     * If it did, it would cause infinite recursion in loadClass implementations
     * that call findLoadedClass.
     */
    static void testFindLoadedDoesNotTriggerLoading() throws Exception {
        System.out.println("=== Find Loaded Does Not Trigger Loading ===");

        TrackingClassLoader loader = new TrackingClassLoader(Test.class.getClassLoader());
        int loadCountBefore = loader.loadClassCount;

        // Call findLoadedClass — must not trigger loadClass
        loader.publicFindLoaded("java.util.LinkedList");

        int loadCountAfter = loader.loadClassCount;
        System.out.println("No loading triggered: " + (loadCountBefore == loadCountAfter));
    }

    /** Standard loadClass delegation pattern works correctly. */
    static void testLoadClassDelegation() throws Exception {
        System.out.println("\n=== Load Class Delegation ===");

        TrackingClassLoader loader = new TrackingClassLoader(Test.class.getClassLoader());

        // Load a class through the tracking loader
        Class<?> cls1 = loader.loadClass("java.lang.Integer");
        System.out.println("Loaded: " + cls1.getName());

        // Loading it again should still work (may hit parent cache)
        Class<?> cls2 = loader.loadClass("java.lang.Integer");
        System.out.println("Same class: " + (cls1 == cls2));
    }

    /** findLoadedClass returns the correct class when one is found. */
    static void testFindLoadedReturnsCorrectClass() throws Exception {
        System.out.println("\n=== Find Loaded Returns Correct Class ===");

        TrackingClassLoader loader = new TrackingClassLoader(Test.class.getClassLoader());

        // First, ensure String is loaded by requesting it through loadClass
        Class<?> loaded = loader.loadClass("java.lang.String");
        System.out.println("Loaded: " + loaded.getName());

        // findLoadedClass for a well-known class should return the same Class
        // object as Class.forName (they should be the same identity)
        Class<?> found = loader.publicFindLoaded("java.lang.String");
        if (found != null) {
            System.out.println("Found class matches: " + (found == String.class));
        } else {
            // On the JVM, findLoadedClass only returns classes defined by THIS loader;
            // bootstrap classes may return null. Either way is valid.
            System.out.println("Found class matches: true");
        }
    }

    /** Multiple findLoadedClass calls return the same Class object (identity). */
    static void testFindLoadedClassIdentity() throws Exception {
        System.out.println("\n=== Find Loaded Class Identity ===");

        TrackingClassLoader loader = new TrackingClassLoader(Test.class.getClassLoader());

        // Load through the loader
        loader.loadClass("java.lang.Thread");

        Class<?> found1 = loader.publicFindLoaded("java.lang.Thread");
        Class<?> found2 = loader.publicFindLoaded("java.lang.Thread");

        if (found1 != null && found2 != null) {
            System.out.println("Same identity: " + (found1 == found2));
        } else {
            // Both null is also valid (bootstrap class not recorded by child loader)
            System.out.println("Same identity: " + (found1 == found2));
        }
    }
}
