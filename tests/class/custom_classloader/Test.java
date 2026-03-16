/** Test Class.forName with custom ClassLoader implementations. */
public class Test {

    /** Custom ClassLoader that delegates to the parent for known classes. */
    static class DelegatingClassLoader extends ClassLoader {
        public DelegatingClassLoader(ClassLoader parent) {
            super(parent);
        }

        @Override
        protected Class<?> loadClass(String name, boolean resolve) throws ClassNotFoundException {
            return super.loadClass(name, resolve);
        }
    }

    /** Custom ClassLoader that rejects a specific class name. */
    static class RejectingClassLoader extends ClassLoader {
        private final String rejectedName;

        public RejectingClassLoader(ClassLoader parent, String rejectedName) {
            super(parent);
            this.rejectedName = rejectedName;
        }

        @Override
        protected Class<?> loadClass(String name, boolean resolve) throws ClassNotFoundException {
            if (name.equals(rejectedName)) {
                throw new ClassNotFoundException("Rejected: " + name);
            }
            return super.loadClass(name, resolve);
        }
    }

    public static void main(String[] args) throws Exception {
        testCustomClassLoaderDelegation();
        testForNameWithNullLoader();
        testForNameClassNotFound();
        testForNameWithInitialize();
        testRejectingClassLoader();
    }

    static void testCustomClassLoaderDelegation() throws Exception {
        System.out.println("=== Custom ClassLoader Delegation ===");

        DelegatingClassLoader loader = new DelegatingClassLoader(
            Test.class.getClassLoader()
        );

        // Load a standard class through the custom loader
        Class<?> stringClass = Class.forName("java.lang.String", true, loader);
        System.out.println("Loaded String: " + stringClass.getName());
        System.out.println("Is String class: " + (stringClass == String.class));

        // Load another standard class
        Class<?> integerClass = Class.forName("java.lang.Integer", true, loader);
        System.out.println("Loaded Integer: " + integerClass.getName());
    }

    static void testForNameWithNullLoader() throws Exception {
        System.out.println("\n=== ForName with Null Loader ===");

        // Null loader should use bootstrap/system loader
        Class<?> stringClass = Class.forName("java.lang.String", true, null);
        System.out.println("Loaded via null loader: " + stringClass.getName());
        System.out.println("Same as String.class: " + (stringClass == String.class));
    }

    static void testForNameClassNotFound() {
        System.out.println("\n=== ForName ClassNotFoundException ===");

        DelegatingClassLoader loader = new DelegatingClassLoader(
            Test.class.getClassLoader()
        );

        try {
            Class.forName("com.nonexistent.FakeClass", true, loader);
            System.out.println("ERROR: Should have thrown ClassNotFoundException");
        } catch (ClassNotFoundException e) {
            System.out.println("ClassNotFoundException thrown: true");
            System.out.println("Message contains class name: " +
                e.getMessage().contains("com.nonexistent.FakeClass"));
        }
    }

    static void testForNameWithInitialize() throws Exception {
        System.out.println("\n=== ForName With/Without Initialize ===");

        DelegatingClassLoader loader = new DelegatingClassLoader(
            Test.class.getClassLoader()
        );

        // Load without initialization
        Class<?> cls1 = Class.forName("java.lang.StringBuilder", false, loader);
        System.out.println("Loaded without init: " + cls1.getName());

        // Load with initialization
        Class<?> cls2 = Class.forName("java.lang.StringBuilder", true, loader);
        System.out.println("Loaded with init: " + cls2.getName());

        // Both should reference the same class
        System.out.println("Same class: " + (cls1 == cls2));
    }

    static void testRejectingClassLoader() {
        System.out.println("\n=== Rejecting ClassLoader ===");

        RejectingClassLoader loader = new RejectingClassLoader(
            Test.class.getClassLoader(), "java.lang.Thread"
        );

        // Load a class that is NOT rejected
        try {
            Class<?> stringClass = Class.forName("java.lang.String", true, loader);
            System.out.println("Loaded non-rejected class: " + stringClass.getName());
        } catch (ClassNotFoundException e) {
            System.out.println("ERROR: Should have loaded String");
        }

        // Load a class that IS rejected
        try {
            Class.forName("java.lang.Thread", true, loader);
            System.out.println("ERROR: Should have thrown ClassNotFoundException");
        } catch (ClassNotFoundException e) {
            System.out.println("Rejected class threw exception: true");
            System.out.println("Message contains rejected: " +
                e.getMessage().contains("Rejected"));
        }
    }
}
