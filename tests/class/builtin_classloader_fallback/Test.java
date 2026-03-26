/**
 * Test Class.forName with built-in classloaders.
 *
 * Verifies that Class.forName succeeds when loading classes through the
 * system/application classloader (a built-in classloader), even if the
 * Java-side classloader hierarchy cannot resolve the class directly.
 * The VM should fall through to its own class path resolution.
 *
 * Also verifies that custom (non-built-in) classloaders still have their
 * ClassNotFoundException respected, and that Class.forName with
 * initialize=true triggers static initialization per JVM spec §12.4.1.
 */
public class Test {

    /**
     * Witness class whose field is written by InitCanary's static initializer.
     * Reading InitWitness.canaryInitialized does NOT trigger InitCanary's
     * {@code <clinit>}, avoiding the getstatic-triggers-init pitfall.
     */
    static class InitWitness {
        static boolean canaryInitialized = false;
    }

    /** Canary class whose static initializer signals via the witness. */
    static class InitCanary {
        static {
            InitWitness.canaryInitialized = true;
        }
    }

    /** Custom ClassLoader that always throws ClassNotFoundException. */
    static class AlwaysFailingClassLoader extends ClassLoader {
        public AlwaysFailingClassLoader(ClassLoader parent) {
            super(parent);
        }

        @Override
        protected Class<?> loadClass(String name, boolean resolve) throws ClassNotFoundException {
            // Reject everything except java.lang.Object (needed for ClassLoader itself)
            if (name.startsWith("java.lang.")) {
                return super.loadClass(name, resolve);
            }
            throw new ClassNotFoundException("AlwaysFailing: " + name);
        }
    }

    /** Custom CNFE subclass thrown by a classloader. */
    static class DetailedClassNotFoundException extends ClassNotFoundException {
        public DetailedClassNotFoundException(String name) {
            super("Detailed: " + name);
        }
    }

    /** Custom ClassLoader that throws a subclass of ClassNotFoundException. */
    static class SubclassCNFEClassLoader extends ClassLoader {
        public SubclassCNFEClassLoader(ClassLoader parent) {
            super(parent);
        }

        @Override
        protected Class<?> loadClass(String name, boolean resolve) throws ClassNotFoundException {
            if (name.startsWith("java.lang.")) {
                return super.loadClass(name, resolve);
            }
            throw new DetailedClassNotFoundException(name);
        }
    }

    public static void main(String[] args) throws Exception {
        testBuiltinLoaderLoadsCoreClass();
        testBuiltinLoaderWithInitialize();
        testBuiltinLoaderWithoutInitialize();
        testBuiltinLoaderIdentity();
        testCustomLoaderExceptionRespected();
        testNullLoaderFallback();
        testInitializeTrueTriggersStaticInit();
        testCustomLoaderCNFESubclassRespected();
    }

    /** Built-in (system) classloader can load core classes via Class.forName. */
    static void testBuiltinLoaderLoadsCoreClass() throws Exception {
        System.out.println("=== Built-in Loader Loads Core Class ===");

        ClassLoader systemLoader = ClassLoader.getSystemClassLoader();
        Class<?> cls = Class.forName("java.lang.String", true, systemLoader);
        System.out.println("Loaded: " + cls.getName());
        System.out.println("Is String: " + (cls == String.class));
    }

    /** Built-in loader with initialize=true triggers static initialization. */
    static void testBuiltinLoaderWithInitialize() throws Exception {
        System.out.println("\n=== Built-in Loader With Initialize ===");

        ClassLoader systemLoader = ClassLoader.getSystemClassLoader();
        Class<?> cls = Class.forName("java.util.HashMap", true, systemLoader);
        System.out.println("Loaded: " + cls.getName());
        System.out.println("Is HashMap: " + (cls == java.util.HashMap.class));
    }

    /** Built-in loader with initialize=false skips static initialization. */
    static void testBuiltinLoaderWithoutInitialize() throws Exception {
        System.out.println("\n=== Built-in Loader Without Initialize ===");

        ClassLoader systemLoader = ClassLoader.getSystemClassLoader();
        Class<?> cls = Class.forName("java.util.ArrayList", false, systemLoader);
        System.out.println("Loaded: " + cls.getName());
        System.out.println("Is ArrayList: " + (cls == java.util.ArrayList.class));
    }

    /** Multiple forName calls through built-in loader return the same Class object. */
    static void testBuiltinLoaderIdentity() throws Exception {
        System.out.println("\n=== Built-in Loader Identity ===");

        ClassLoader systemLoader = ClassLoader.getSystemClassLoader();
        Class<?> cls1 = Class.forName("java.lang.Integer", true, systemLoader);
        Class<?> cls2 = Class.forName("java.lang.Integer", true, systemLoader);
        System.out.println("Same class object: " + (cls1 == cls2));
        System.out.println("Name: " + cls1.getName());
    }

    /** Custom (non-built-in) classloader's ClassNotFoundException is respected. */
    static void testCustomLoaderExceptionRespected() {
        System.out.println("\n=== Custom Loader Exception Respected ===");

        AlwaysFailingClassLoader loader = new AlwaysFailingClassLoader(
            ClassLoader.getSystemClassLoader()
        );

        try {
            Class.forName("com.example.NonExistent", true, loader);
            System.out.println("ERROR: Should have thrown ClassNotFoundException");
        } catch (ClassNotFoundException e) {
            System.out.println("ClassNotFoundException thrown: true");
            System.out.println("Message contains AlwaysFailing: " +
                e.getMessage().contains("AlwaysFailing"));
        }
    }

    /** Null classloader falls back to bootstrap/system loader. */
    static void testNullLoaderFallback() throws Exception {
        System.out.println("\n=== Null Loader Fallback ===");

        Class<?> cls = Class.forName("java.lang.String", true, null);
        System.out.println("Loaded via null: " + cls.getName());
        System.out.println("Is String: " + (cls == String.class));
    }

    /**
     * Class.forName(name, true, loader) must trigger static initialization (JVM spec §12.4.1).
     * Uses InitCanary whose static block sets a flag on the separate InitWitness class,
     * so we can observe the flag without a getstatic on InitCanary (which would itself
     * trigger initialization and make the test vacuous).
     */
    static void testInitializeTrueTriggersStaticInit() throws Exception {
        System.out.println("\n=== Initialize True Triggers Static Init ===");

        // Reading the witness field does NOT trigger InitCanary's <clinit>
        System.out.println("Before forName: initialized=" + InitWitness.canaryInitialized);

        Class.forName("Test$InitCanary", true, ClassLoader.getSystemClassLoader());

        System.out.println("After forName: initialized=" + InitWitness.canaryInitialized);
        if (!InitWitness.canaryInitialized) {
            throw new RuntimeException("InitCanary was not initialized!");
        }
    }

    /**
     * Custom classloader throwing a subclass of ClassNotFoundException must have
     * its exception respected (not swallowed by VM fallback).
     * Per JVMS §5.3, the specified classloader is the defining authority.
     */
    static void testCustomLoaderCNFESubclassRespected() {
        System.out.println("\n=== Custom Loader CNFE Subclass Respected ===");

        SubclassCNFEClassLoader loader = new SubclassCNFEClassLoader(
            ClassLoader.getSystemClassLoader()
        );

        try {
            Class.forName("com.example.NonExistent", true, loader);
            System.out.println("ERROR: Should have thrown ClassNotFoundException");
        } catch (ClassNotFoundException e) {
            System.out.println("ClassNotFoundException thrown: true");
            System.out.println("Is subclass: " + (e instanceof DetailedClassNotFoundException));
            System.out.println("Message contains Detailed: " +
                e.getMessage().contains("Detailed"));
        }
    }
}
