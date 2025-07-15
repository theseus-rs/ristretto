/** Test synthetic members and bridge methods in reflection. */
import java.lang.reflect.*;

public class Test {
    // Generic class to create bridge methods
    static class GenericParent<T> {
        public T getValue() {
            return null;
        }

        public void setValue(T value) {
        }
    }

    static class StringChild extends GenericParent<String> {
        @Override
        public String getValue() {
            return "string value";
        }

        @Override
        public void setValue(String value) {
            System.out.println("Setting string value: " + value);
        }
    }

    // Inner class to create synthetic members
    class InnerClass {
        private String value = "inner";

        public void accessOuter() {
            System.out.println("Outer field: " + Test.this.outerField);
        }
    }

    private String outerField = "outer";

    public static void main(String[] args) throws Exception {
        // Test bridge methods
        System.out.println("=== Bridge Methods ===");
        Class<?> stringChildClass = StringChild.class;
        Method[] methods = stringChildClass.getDeclaredMethods();

        int bridgeCount = 0;
        int normalCount = 0;

        for (Method method : methods) {
            System.out.println("Method: " + method.getName());
            System.out.println("  Return type: " + method.getReturnType().getName());
            System.out.println("  Generic return type: " + method.getGenericReturnType().getTypeName());
            System.out.println("  Is bridge: " + method.isBridge());
            System.out.println("  Is synthetic: " + method.isSynthetic());
            System.out.println("  Modifiers: " + Modifier.toString(method.getModifiers()));

            if (method.isBridge()) {
                bridgeCount++;
            } else {
                normalCount++;
            }

            // Show parameter types for bridge vs non-bridge methods
            Class<?>[] paramTypes = method.getParameterTypes();
            for (int i = 0; i < paramTypes.length; i++) {
                System.out.println("  Parameter " + i + ": " + paramTypes[i].getName());
            }
            System.out.println();
        }

        System.out.println("Bridge methods count: " + bridgeCount);
        System.out.println("Normal methods count: " + normalCount);

        // Test method invocation on bridge vs non-bridge
        StringChild instance = new StringChild();
        for (Method method : methods) {
            if (method.getName().equals("getValue")) {
                Object result = method.invoke(instance);
                System.out.println("Method " + method.getName() +
                    (method.isBridge() ? " (bridge)" : " (normal)") +
                    " result: " + result);
            }
        }

        // Test synthetic members in inner classes
        System.out.println("\n=== Synthetic Members in Inner Classes ===");
        Test outer = new Test();
        InnerClass inner = outer.new InnerClass();
        Class<?> innerClass = inner.getClass();

        // Check synthetic fields (reference to outer instance)
        Field[] fields = innerClass.getDeclaredFields();
        System.out.println("Inner class fields:");
        for (Field field : fields) {
            System.out.println("Field: " + field.getName());
            System.out.println("  Type: " + field.getType().getName());
            System.out.println("  Is synthetic: " + field.isSynthetic());
            System.out.println("  Modifiers: " + Modifier.toString(field.getModifiers()));

            if (field.isSynthetic()) {
                field.setAccessible(true);
                Object value = field.get(inner);
                System.out.println("  Synthetic field value type: " + (value != null ? value.getClass().getName() : "null"));
                System.out.println("  Points to outer instance: " + (value == outer));
            }
            System.out.println();
        }

        // Check synthetic methods (access methods for private outer members)
        Method[] innerMethods = innerClass.getDeclaredMethods();
        System.out.println("Inner class methods:");
        for (Method method : innerMethods) {
            System.out.println("Method: " + method.getName());
            System.out.println("  Is synthetic: " + method.isSynthetic());
            System.out.println("  Modifiers: " + Modifier.toString(method.getModifiers()));
            System.out.println();
        }

        // Check outer class for synthetic access methods
        System.out.println("Outer class synthetic methods:");
        Method[] outerMethods = Test.class.getDeclaredMethods();
        for (Method method : outerMethods) {
            if (method.isSynthetic()) {
                System.out.println("Synthetic method: " + method.getName());
                System.out.println("  Return type: " + method.getReturnType().getName());
                System.out.println("  Parameter count: " + method.getParameterCount());
                Class<?>[] paramTypes = method.getParameterTypes();
                for (int i = 0; i < paramTypes.length; i++) {
                    System.out.println("  Parameter " + i + ": " + paramTypes[i].getName());
                }
                System.out.println();
            }
        }

        // Test enum synthetic methods
        System.out.println("\n=== Enum Synthetic Methods ===");
        enum TestEnum { VALUE1, VALUE2 }

        Method[] enumMethods = TestEnum.class.getDeclaredMethods();
        for (Method method : enumMethods) {
            System.out.println("Enum method: " + method.getName());
            System.out.println("  Is synthetic: " + method.isSynthetic());
            System.out.println("  Modifiers: " + Modifier.toString(method.getModifiers()));
            System.out.println("  Return type: " + method.getReturnType().getName());

            if (method.getName().equals("values") || method.getName().equals("valueOf")) {
                System.out.println("  Special enum method");
            }
            System.out.println();
        }

        // Test lambda synthetic methods
        System.out.println("\n=== Lambda Synthetic Methods ===");
        Runnable lambda = () -> System.out.println("Lambda executed");
        Class<?> lambdaClass = lambda.getClass();

        System.out.println("Lambda class: " + lambdaClass.getName());
        System.out.println("Is synthetic class: " + lambdaClass.isSynthetic());

        Method[] lambdaMethods = lambdaClass.getDeclaredMethods();
        for (Method method : lambdaMethods) {
            System.out.println("Lambda method: " + method.getName());
            System.out.println("  Is synthetic: " + method.isSynthetic());
            System.out.println("  Modifiers: " + Modifier.toString(method.getModifiers()));
            System.out.println();
        }

        // Test constructor synthetic parameters
        System.out.println("\n=== Constructor Synthetic Parameters ===");
        Constructor<?>[] innerConstructors = innerClass.getDeclaredConstructors();
        for (Constructor<?> constructor : innerConstructors) {
            System.out.println("Inner constructor parameter count: " + constructor.getParameterCount());
            Class<?>[] paramTypes = constructor.getParameterTypes();
            Parameter[] params = constructor.getParameters();

            for (int i = 0; i < paramTypes.length; i++) {
                System.out.println("  Parameter " + i + ": " + paramTypes[i].getName());
                if (i < params.length) {
                    System.out.println("    Is synthetic: " + params[i].isSynthetic());
                    System.out.println("    Name: " + params[i].getName());
                }
            }
            System.out.println();
        }

        // Test generic type erasure effects
        System.out.println("\n=== Generic Type Erasure Effects ===");
        Method parentGetValue = GenericParent.class.getMethod("getValue");
        Method childGetValue = null;
        Method bridgeGetValue = null;

        for (Method method : stringChildClass.getDeclaredMethods()) {
            if (method.getName().equals("getValue")) {
                if (method.isBridge()) {
                    bridgeGetValue = method;
                } else {
                    childGetValue = method;
                }
            }
        }

        if (parentGetValue != null) {
            System.out.println("Parent getValue return type: " + parentGetValue.getReturnType().getName());
            System.out.println("Parent getValue generic return type: " + parentGetValue.getGenericReturnType().getTypeName());
        }

        if (childGetValue != null) {
            System.out.println("Child getValue return type: " + childGetValue.getReturnType().getName());
            System.out.println("Child getValue generic return type: " + childGetValue.getGenericReturnType().getTypeName());
        }

        if (bridgeGetValue != null) {
            System.out.println("Bridge getValue return type: " + bridgeGetValue.getReturnType().getName());
            System.out.println("Bridge getValue generic return type: " + bridgeGetValue.getGenericReturnType().getTypeName());
        }

        // Test accessing synthetic members
        System.out.println("\n=== Accessing Synthetic Members ===");
        for (Field field : fields) {
            if (field.isSynthetic()) {
                field.setAccessible(true);
                try {
                    Object syntheticValue = field.get(inner);
                    System.out.println("Successfully accessed synthetic field: " + field.getName());
                } catch (Exception e) {
                    System.out.println("Failed to access synthetic field: " + e.getMessage());
                }
            }
        }

        // Test method.isSynthetic() vs method.isBridge() distinction
        System.out.println("\n=== Synthetic vs Bridge Distinction ===");
        for (Method method : stringChildClass.getDeclaredMethods()) {
            if (method.isSynthetic() || method.isBridge()) {
                System.out.println("Method: " + method.getName());
                System.out.println("  Synthetic: " + method.isSynthetic());
                System.out.println("  Bridge: " + method.isBridge());
                System.out.println("  Both: " + (method.isSynthetic() && method.isBridge()));
            }
        }
    }
}
