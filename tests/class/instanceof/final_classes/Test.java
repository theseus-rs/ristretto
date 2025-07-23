/** Test instanceof behavior with final classes and standard Java library classes */
public class Test {
    public static void main(String[] args) {
        // Test String (final class) instanceof scenarios
        String str = "Hello World";
        System.out.println("String instanceof String: " + (str instanceof String));
        System.out.println("String instanceof Object: " + (str instanceof Object));
        System.out.println("String instanceof CharSequence: " + (str instanceof CharSequence));
        System.out.println("String instanceof Comparable: " + (str instanceof Comparable));
        System.out.println("String instanceof java.io.Serializable: " + (str instanceof java.io.Serializable));

        // Test StringBuilder (not final) instanceof scenarios
        StringBuilder sb = new StringBuilder("Hello");
        System.out.println("StringBuilder instanceof StringBuilder: " + (sb instanceof StringBuilder));
        System.out.println("StringBuilder instanceof Object: " + (sb instanceof Object));
        System.out.println("StringBuilder instanceof CharSequence: " + (sb instanceof CharSequence));
        System.out.println("StringBuilder instanceof Appendable: " + (sb instanceof Appendable));
        System.out.println("StringBuilder instanceof java.io.Serializable: " + (sb instanceof java.io.Serializable));

        // Test StringBuffer instanceof scenarios
        StringBuffer sbuf = new StringBuffer("Hello");
        System.out.println("StringBuffer instanceof StringBuffer: " + (sbuf instanceof StringBuffer));
        System.out.println("StringBuffer instanceof Object: " + (sbuf instanceof Object));
        System.out.println("StringBuffer instanceof CharSequence: " + (sbuf instanceof CharSequence));
        System.out.println("StringBuffer instanceof Appendable: " + (sbuf instanceof Appendable));
        System.out.println("StringBuffer instanceof java.io.Serializable: " + (sbuf instanceof java.io.Serializable));

        // Test Class object instanceof scenarios
        Class<?> stringClass = String.class;
        System.out.println("Class instanceof Class: " + (stringClass instanceof Class));
        System.out.println("Class instanceof Object: " + (stringClass instanceof Object));
        System.out.println("Class instanceof java.io.Serializable: " + (stringClass instanceof java.io.Serializable));

        // Test Exception hierarchy
        Exception ex = new Exception("Test exception");
        System.out.println("Exception instanceof Exception: " + (ex instanceof Exception));
        System.out.println("Exception instanceof Throwable: " + (ex instanceof Throwable));
        System.out.println("Exception instanceof Object: " + (ex instanceof Object));
        System.out.println("Exception instanceof java.io.Serializable: " + (ex instanceof java.io.Serializable));
        System.out.println("Exception instanceof RuntimeException: " + (ex instanceof RuntimeException));

        RuntimeException runtimeEx = new RuntimeException("Runtime exception");
        System.out.println("RuntimeException instanceof RuntimeException: " + (runtimeEx instanceof RuntimeException));
        System.out.println("RuntimeException instanceof Exception: " + (runtimeEx instanceof Exception));
        System.out.println("RuntimeException instanceof Throwable: " + (runtimeEx instanceof Throwable));
        System.out.println("RuntimeException instanceof Object: " + (runtimeEx instanceof Object));

        // Test specific exceptions
        NullPointerException npe = new NullPointerException("NPE");
        System.out.println("NullPointerException instanceof NullPointerException: " + (npe instanceof NullPointerException));
        System.out.println("NullPointerException instanceof RuntimeException: " + (npe instanceof RuntimeException));
        System.out.println("NullPointerException instanceof Exception: " + (npe instanceof Exception));
        System.out.println("NullPointerException instanceof Throwable: " + (npe instanceof Throwable));

        // Test Thread class
        Thread thread = new Thread();
        System.out.println("Thread instanceof Thread: " + (thread instanceof Thread));
        System.out.println("Thread instanceof Object: " + (thread instanceof Object));
        System.out.println("Thread instanceof Runnable: " + (thread instanceof Runnable));

        // Test with collection framework classes
        java.util.ArrayList<String> list = new java.util.ArrayList<>();
        System.out.println("ArrayList instanceof ArrayList: " + (list instanceof java.util.ArrayList));
        System.out.println("ArrayList instanceof java.util.List: " + (list instanceof java.util.List));
        System.out.println("ArrayList instanceof java.util.Collection: " + (list instanceof java.util.Collection));
        System.out.println("ArrayList instanceof java.util.AbstractList: " + (list instanceof java.util.AbstractList));
        System.out.println("ArrayList instanceof Object: " + (list instanceof Object));
        System.out.println("ArrayList instanceof java.io.Serializable: " + (list instanceof java.io.Serializable));
        System.out.println("ArrayList instanceof Cloneable: " + (list instanceof Cloneable));
        System.out.println("ArrayList instanceof java.util.Set: " + (list instanceof java.util.Set));

        // Test with Map
        java.util.HashMap<String, Integer> map = new java.util.HashMap<>();
        System.out.println("HashMap instanceof HashMap: " + (map instanceof java.util.HashMap));
        System.out.println("HashMap instanceof java.util.Map: " + (map instanceof java.util.Map));
        System.out.println("HashMap instanceof java.util.AbstractMap: " + (map instanceof java.util.AbstractMap));
        System.out.println("HashMap instanceof Object: " + (map instanceof Object));
        System.out.println("HashMap instanceof java.io.Serializable: " + (map instanceof java.io.Serializable));
        System.out.println("HashMap instanceof Cloneable: " + (map instanceof Cloneable));
        System.out.println("HashMap instanceof java.util.List: " + (map instanceof java.util.List));
    }
}
