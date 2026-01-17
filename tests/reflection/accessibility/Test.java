/** Test accessibility and setAccessible operations in reflection. */
import java.lang.reflect.*;
import java.util.Arrays;
import java.util.Comparator;

public class Test {
    // Various access levels for testing
    public static String publicStaticField = "public static";
    protected static String protectedStaticField = "protected static";
    static String packageStaticField = "package static";
    private static String privateStaticField = "private static";

    public String publicInstanceField = "public instance";
    protected String protectedInstanceField = "protected instance";
    String packageInstanceField = "package instance";
    private String privateInstanceField = "private instance";

    public final String publicFinalField = "public final";
    private final String privateFinalField = "private final";

    // Methods with various access levels
    public static void publicStaticMethod() {
        System.out.println("Public static method");
    }

    protected static void protectedStaticMethod() {
        System.out.println("Protected static method");
    }

    static void packageStaticMethod() {
        System.out.println("Package static method");
    }

    private static void privateStaticMethod() {
        System.out.println("Private static method");
    }

    public void publicInstanceMethod() {
        System.out.println("Public instance method");
    }

    protected void protectedInstanceMethod() {
        System.out.println("Protected instance method");
    }

    void packageInstanceMethod() {
        System.out.println("Package instance method");
    }

    private void privateInstanceMethod() {
        System.out.println("Private instance method");
    }

    // Constructors with various access levels
    public Test() {
        System.out.println("Public constructor");
    }

    protected Test(int x) {
        System.out.println("Protected constructor");
    }

    Test(String s) {
        System.out.println("Package constructor");
    }

    private Test(boolean b) {
        System.out.println("Private constructor");
    }

    public static void main(String[] args) throws Exception {
        Class<?> clazz = Test.class;
        Test instance = new Test();

        // Test field accessibility
        System.out.println("=== Field Accessibility ===");

        Field[] allFields = clazz.getDeclaredFields();
        // Sort fields by name for deterministic output
        Arrays.sort(allFields, Comparator.comparing(Field::getName));
        for (Field field : allFields) {
            if (!field.isSynthetic()) {
                testFieldAccessibility(field, instance);
            }
        }

        // Test method accessibility
        System.out.println("\n=== Method Accessibility ===");

        Method[] allMethods = clazz.getDeclaredMethods();
        // Sort methods by name for deterministic output
        Arrays.sort(allMethods, Comparator.comparing(Method::getName));
        for (Method method : allMethods) {
            if (!method.isSynthetic() && !method.getName().equals("main") &&
                !method.getName().equals("testFieldAccessibility") &&
                !method.getName().equals("testMethodAccessibility") &&
                !method.getName().equals("testConstructorAccessibility") &&
                !method.getName().equals("createConstructorArgs") &&
                method.getParameterCount() == 0) {
                testMethodAccessibility(method, instance);
            }
        }

        // Test constructor accessibility
        System.out.println("\n=== Constructor Accessibility ===");

        Constructor<?>[] allConstructors = clazz.getDeclaredConstructors();
        // Sort constructors by parameter count, then by first param type name
        Arrays.sort(allConstructors, (c1, c2) -> {
            int cmp = Integer.compare(c1.getParameterCount(), c2.getParameterCount());
            if (cmp != 0) return cmp;
            String t1 = c1.getParameterCount() > 0 ? c1.getParameterTypes()[0].getName() : "";
            String t2 = c2.getParameterCount() > 0 ? c2.getParameterTypes()[0].getName() : "";
            return t1.compareTo(t2);
        });
        for (Constructor<?> constructor : allConstructors) {
            testConstructorAccessibility(constructor);
        }

        // Test setAccessible behavior
        System.out.println("\n=== setAccessible Behavior ===");

        Field privateField = clazz.getDeclaredField("privateInstanceField");
        System.out.println("Private field initially accessible: " + privateField.isAccessible());

        // Try to access without setAccessible
        try {
            privateField.get(instance);
            System.out.println("ERROR: Should have thrown IllegalAccessException");
        } catch (IllegalAccessException e) {
            System.out.println("Correctly blocked private field access: " + e.getMessage());
        }

        // Set accessible and try again
        privateField.setAccessible(true);
        System.out.println("Private field after setAccessible(true): " + privateField.isAccessible());

        try {
            String value = (String) privateField.get(instance);
            System.out.println("Successfully accessed private field: " + value);
        } catch (IllegalAccessException e) {
            System.out.println("Still blocked after setAccessible: " + e.getMessage());
        }

        // Test setAccessible(false)
        privateField.setAccessible(false);
        System.out.println("Private field after setAccessible(false): " + privateField.isAccessible());

        // Test final field modification
        System.out.println("\n=== Final Field Modification ===");

        Field publicFinalField = clazz.getDeclaredField("publicFinalField");
        Field privateFinalField = clazz.getDeclaredField("privateFinalField");

        System.out.println("Public final field initial value: " + publicFinalField.get(instance));
        System.out.println("Public final field is final: " + Modifier.isFinal(publicFinalField.getModifiers()));

        try {
            publicFinalField.set(instance, "modified public final");
            System.out.println("Successfully modified public final field");
        } catch (IllegalAccessException e) {
            System.out.println("Blocked modification of public final field: " + e.getMessage());
        }

        privateFinalField.setAccessible(true);
        System.out.println("Private final field initial value: " + privateFinalField.get(instance));

        try {
            privateFinalField.set(instance, "modified private final");
            System.out.println("Successfully modified private final field: " + privateFinalField.get(instance));
        } catch (IllegalAccessException e) {
            System.out.println("Blocked modification of private final field: " + e.getMessage());
        }

        // Test static field access
        System.out.println("\n=== Static Field Access ===");

        Field privateStaticField = clazz.getDeclaredField("privateStaticField");
        privateStaticField.setAccessible(true);

        String staticValue = (String) privateStaticField.get(null);
        System.out.println("Private static field value: " + staticValue);

        privateStaticField.set(null, "modified private static");
        String modifiedStaticValue = (String) privateStaticField.get(null);
        System.out.println("Modified private static field value: " + modifiedStaticValue);

        // Test cross-instance field access
        System.out.println("\n=== Cross-Instance Field Access ===");

        Test anotherInstance = new Test();
        Field instanceField = clazz.getDeclaredField("privateInstanceField");
        instanceField.setAccessible(true);

        String value1 = (String) instanceField.get(instance);
        String value2 = (String) instanceField.get(anotherInstance);

        System.out.println("Instance 1 private field: " + value1);
        System.out.println("Instance 2 private field: " + value2);

        instanceField.set(anotherInstance, "modified instance 2");
        System.out.println("After modification - Instance 1: " + instanceField.get(instance));
        System.out.println("After modification - Instance 2: " + instanceField.get(anotherInstance));

        // Test accessibility inheritance
        System.out.println("\n=== Accessibility Inheritance ===");

        // Test if setAccessible affects the same field obtained multiple times
        Field field1 = clazz.getDeclaredField("privateInstanceField");
        Field field2 = clazz.getDeclaredField("privateInstanceField");

        System.out.println("Field1 initially accessible: " + field1.isAccessible());
        System.out.println("Field2 initially accessible: " + field2.isAccessible());

        field1.setAccessible(true);
        System.out.println("After field1.setAccessible(true):");
        System.out.println("Field1 accessible: " + field1.isAccessible());
        System.out.println("Field2 accessible: " + field2.isAccessible());

        // Test security manager effects (if applicable)
        System.out.println("\n=== Security Manager Effects ===");
        SecurityManager sm = System.getSecurityManager();
        System.out.println("Security manager present: " + (sm != null));

        if (sm != null) {
            try {
                Field restrictedField = String.class.getDeclaredField("value");
                restrictedField.setAccessible(true);
                System.out.println("Successfully set String.value accessible");
            } catch (SecurityException e) {
                System.out.println("Security manager blocked access: " + e.getMessage());
            } catch (NoSuchFieldException e) {
                System.out.println("String.value field not found");
            }
        }

        // Test accessibility with different class loaders
        System.out.println("\n=== Class Loader Effects ===");

        ClassLoader currentLoader = Test.class.getClassLoader();
        ClassLoader systemLoader = ClassLoader.getSystemClassLoader();

        // Normalize classloader names to avoid implementation differences
        String currentLoaderName = currentLoader != null ? currentLoader.getClass().getSimpleName() : "Bootstrap";
        if (currentLoaderName.contains("AppClassLoader") || currentLoaderName.contains("BuiltinClassLoader")) {
            currentLoaderName = "ApplicationClassLoader";
        }
        System.out.println("Current class loader: " + currentLoaderName);
        System.out.println("System class loader: " + systemLoader.getClass().getSimpleName());
        // The comparison depends on implementation details
        System.out.println("Loaders are compatible: " + (currentLoader != null && systemLoader != null));
    }

    private static void testFieldAccessibility(Field field, Test instance) throws Exception {
        System.out.println("Field: " + field.getName());
        System.out.println("  Modifiers: " + Modifier.toString(field.getModifiers()));
        System.out.println("  Initially accessible: " + field.isAccessible());

        boolean needsInstance = !Modifier.isStatic(field.getModifiers());
        Object target = needsInstance ? instance : null;

        try {
            Object value = field.get(target);
            System.out.println("  Access without setAccessible: SUCCESS (" + value + ")");
        } catch (IllegalAccessException e) {
            System.out.println("  Access without setAccessible: BLOCKED");

            field.setAccessible(true);
            try {
                Object value = field.get(target);
                System.out.println("  Access with setAccessible: SUCCESS (" + value + ")");
            } catch (IllegalAccessException e2) {
                System.out.println("  Access with setAccessible: STILL BLOCKED");
            }
        }
        System.out.println();
    }

    private static void testMethodAccessibility(Method method, Test instance) throws Exception {
        System.out.println("Method: " + method.getName());
        System.out.println("  Modifiers: " + Modifier.toString(method.getModifiers()));
        System.out.println("  Initially accessible: " + method.isAccessible());

        boolean needsInstance = !Modifier.isStatic(method.getModifiers());
        Object target = needsInstance ? instance : null;

        try {
            method.invoke(target);
            System.out.println("  Invoke without setAccessible: SUCCESS");
        } catch (IllegalAccessException e) {
            System.out.println("  Invoke without setAccessible: BLOCKED");

            method.setAccessible(true);
            try {
                method.invoke(target);
                System.out.println("  Invoke with setAccessible: SUCCESS");
            } catch (IllegalAccessException e2) {
                System.out.println("  Invoke with setAccessible: STILL BLOCKED");
            }
        }
        System.out.println();
    }

    private static void testConstructorAccessibility(Constructor<?> constructor) throws Exception {
        System.out.println("Constructor: " + constructor.getParameterCount() + " params");
        System.out.println("  Modifiers: " + Modifier.toString(constructor.getModifiers()));
        System.out.println("  Initially accessible: " + constructor.isAccessible());

        Object[] args = createConstructorArgs(constructor);

        try {
            Object instance = constructor.newInstance(args);
            System.out.println("  Instantiate without setAccessible: SUCCESS");
        } catch (IllegalAccessException e) {
            System.out.println("  Instantiate without setAccessible: BLOCKED");

            constructor.setAccessible(true);
            try {
                Object instance = constructor.newInstance(args);
                System.out.println("  Instantiate with setAccessible: SUCCESS");
            } catch (IllegalAccessException e2) {
                System.out.println("  Instantiate with setAccessible: STILL BLOCKED");
            }
        }
        System.out.println();
    }

    private static Object[] createConstructorArgs(Constructor<?> constructor) {
        Class<?>[] paramTypes = constructor.getParameterTypes();
        Object[] args = new Object[paramTypes.length];

        for (int i = 0; i < paramTypes.length; i++) {
            if (paramTypes[i] == int.class) {
                args[i] = 42;
            } else if (paramTypes[i] == String.class) {
                args[i] = "test";
            } else if (paramTypes[i] == boolean.class) {
                args[i] = true;
            } else {
                args[i] = null;
            }
        }

        return args;
    }
}
