/** Test exception handling in reflection operations. */
public class Test {
    static class TestClass {
        private String privateField = "private";
        public final String finalField = "final";

        private void privateMethod() {
            System.out.println("Private method called");
        }

        public void throwingMethod() throws Exception {
            throw new Exception("Method threw exception");
        }

        public void runtimeExceptionMethod() {
            throw new RuntimeException("Runtime exception");
        }

        private TestClass() {
            System.out.println("Private constructor called");
        }

        public TestClass(String value) {
            if (value == null) {
                throw new IllegalArgumentException("Value cannot be null");
            }
        }
    }

    public static void main(String[] args) throws Exception {
        Class<?> clazz = TestClass.class;

        // Test NoSuchMethodException
        System.out.println("=== NoSuchMethodException Tests ===");
        try {
            clazz.getMethod("nonExistentMethod");
            System.out.println("ERROR: Should have thrown NoSuchMethodException");
        } catch (NoSuchMethodException e) {
            System.out.println("Correctly caught NoSuchMethodException: " + e.getMessage());
        }

        try {
            clazz.getDeclaredMethod("privateMethod", String.class);
            System.out.println("ERROR: Should have thrown NoSuchMethodException");
        } catch (NoSuchMethodException e) {
            System.out.println("Correctly caught NoSuchMethodException for wrong params: " + e.getMessage());
        }

        // Test NoSuchFieldException
        System.out.println("\n=== NoSuchFieldException Tests ===");
        try {
            clazz.getField("nonExistentField");
            System.out.println("ERROR: Should have thrown NoSuchFieldException");
        } catch (NoSuchFieldException e) {
            System.out.println("Correctly caught NoSuchFieldException: " + e.getMessage());
        }

        try {
            clazz.getField("privateField");
            System.out.println("ERROR: Should have thrown NoSuchFieldException");
        } catch (NoSuchFieldException e) {
            System.out.println("Correctly caught NoSuchFieldException for private field: " + e.getMessage());
        }

        // Test IllegalAccessException
        System.out.println("\n=== IllegalAccessException Tests ===");
        TestClass instance = new TestClass("test");

        try {
            java.lang.reflect.Field privateField = clazz.getDeclaredField("privateField");
            privateField.get(instance);
            System.out.println("ERROR: Should have thrown IllegalAccessException");
        } catch (IllegalAccessException e) {
            System.out.println("Correctly caught IllegalAccessException for field: " + e.getMessage());
        }

        try {
            java.lang.reflect.Method privateMethod = clazz.getDeclaredMethod("privateMethod");
            privateMethod.invoke(instance);
            System.out.println("ERROR: Should have thrown IllegalAccessException");
        } catch (IllegalAccessException e) {
            System.out.println("Correctly caught IllegalAccessException for method: " + e.getMessage());
        }

        // Test successful access after setAccessible
        System.out.println("\n=== Successful Access After setAccessible ===");
        java.lang.reflect.Field privateField = clazz.getDeclaredField("privateField");
        privateField.setAccessible(true);
        Object value = privateField.get(instance);
        System.out.println("Private field value after setAccessible: " + value);

        java.lang.reflect.Method privateMethod = clazz.getDeclaredMethod("privateMethod");
        privateMethod.setAccessible(true);
        privateMethod.invoke(instance);

        // Test InvocationTargetException
        System.out.println("\n=== InvocationTargetException Tests ===");
        java.lang.reflect.Method throwingMethod = clazz.getMethod("throwingMethod");
        try {
            throwingMethod.invoke(instance);
            System.out.println("ERROR: Should have thrown InvocationTargetException");
        } catch (java.lang.reflect.InvocationTargetException e) {
            System.out.println("Correctly caught InvocationTargetException");
            System.out.println("Target exception type: " + e.getTargetException().getClass().getName());
            System.out.println("Target exception message: " + e.getTargetException().getMessage());
        }

        java.lang.reflect.Method runtimeExceptionMethod = clazz.getMethod("runtimeExceptionMethod");
        try {
            runtimeExceptionMethod.invoke(instance);
            System.out.println("ERROR: Should have thrown InvocationTargetException");
        } catch (java.lang.reflect.InvocationTargetException e) {
            System.out.println("Correctly caught InvocationTargetException for runtime exception");
            System.out.println("Target exception type: " + e.getTargetException().getClass().getName());
        }

        // Test IllegalArgumentException for wrong parameter types
        System.out.println("\n=== IllegalArgumentException Tests ===");
        java.lang.reflect.Method throwingMethodRef = clazz.getMethod("throwingMethod");
        try {
            throwingMethodRef.invoke(instance, "unexpected parameter");
            System.out.println("ERROR: Should have thrown IllegalArgumentException");
        } catch (IllegalArgumentException e) {
            System.out.println("Correctly caught IllegalArgumentException for wrong params: " + e.getMessage());
        }

        // Test field type mismatch
        try {
            privateField.set(instance, Integer.valueOf(42));
            System.out.println("ERROR: Should have thrown IllegalArgumentException");
        } catch (IllegalArgumentException e) {
            System.out.println("Correctly caught IllegalArgumentException for field type mismatch: " + e.getMessage());
        }

        // Test InstantiationException with private constructor
        System.out.println("\n=== InstantiationException Tests ===");
        try {
            java.lang.reflect.Constructor<?> privateCtor = clazz.getDeclaredConstructor();
            privateCtor.newInstance();
            System.out.println("ERROR: Should have thrown IllegalAccessException");
        } catch (IllegalAccessException e) {
            System.out.println("Correctly caught IllegalAccessException for private constructor: " + e.getMessage());
        }

        // Test constructor with exception
        java.lang.reflect.Constructor<?> throwingCtor = clazz.getDeclaredConstructor(String.class);
        try {
            throwingCtor.newInstance((String) null);
            System.out.println("ERROR: Should have thrown InvocationTargetException");
        } catch (java.lang.reflect.InvocationTargetException e) {
            System.out.println("Correctly caught InvocationTargetException from constructor");
            System.out.println("Target exception: " + e.getTargetException().getClass().getName());
        }

        // Test NullPointerException for null instance
        System.out.println("\n=== NullPointerException Tests ===");
        try {
            throwingMethodRef.invoke(null);
            System.out.println("ERROR: Should have thrown NullPointerException");
        } catch (NullPointerException e) {
            System.out.println("Correctly caught NullPointerException for null instance: " + e.getMessage());
        }

        try {
            privateField.get(null);
            System.out.println("ERROR: Should have thrown NullPointerException");
        } catch (NullPointerException e) {
            System.out.println("Correctly caught NullPointerException for null field access: " + e.getMessage());
        }

        // Test ClassNotFoundException
        System.out.println("\n=== ClassNotFoundException Tests ===");
        try {
            Class.forName("com.nonexistent.Class");
            System.out.println("ERROR: Should have thrown ClassNotFoundException");
        } catch (ClassNotFoundException e) {
            System.out.println("Correctly caught ClassNotFoundException: " + e.getMessage());
        }
    }
}

