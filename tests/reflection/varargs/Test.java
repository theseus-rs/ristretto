/** Test varargs and default methods in reflection. */
import java.lang.reflect.*;

public class Test {
    // Varargs methods
    public static void simpleVarargs(String... args) {
        System.out.println("Simple varargs called with " + args.length + " arguments");
    }

    public static void mixedVarargs(int fixed, String... variable) {
        System.out.println("Mixed varargs: fixed=" + fixed + ", variable count=" + variable.length);
    }

    public static void primitiveVarargs(int... numbers) {
        System.out.println("Primitive varargs with " + numbers.length + " numbers");
    }

    public static void objectVarargs(Object... objects) {
        System.out.println("Object varargs with " + objects.length + " objects");
    }

    // Interface with default methods
    interface DefaultMethodInterface {
        void abstractMethod();

        default void defaultMethod() {
            System.out.println("Default method implementation");
        }

        default String defaultMethodWithReturn() {
            return "default return value";
        }

        static void staticInterfaceMethod() {
            System.out.println("Static interface method");
        }
    }

    // Implementation of interface with default methods
    static class DefaultMethodImpl implements DefaultMethodInterface {
        @Override
        public void abstractMethod() {
            System.out.println("Abstract method implementation");
        }

        @Override
        public void defaultMethod() {
            System.out.println("Overridden default method");
        }
    }

    // Functional interface
    @FunctionalInterface
    interface FunctionalInterfaceTest {
        void functionalMethod(String input);

        default void defaultInFunctional() {
            System.out.println("Default in functional interface");
        }
    }

    public static void main(String[] args) throws Exception {
        // Test varargs method reflection
        System.out.println("=== Varargs Method Reflection ===");

        Method simpleVarargs = Test.class.getMethod("simpleVarargs", String[].class);
        System.out.println("Simple varargs method: " + simpleVarargs.getName());
        System.out.println("Is varargs: " + simpleVarargs.isVarArgs());
        System.out.println("Parameter count: " + simpleVarargs.getParameterCount());

        Parameter[] simpleParams = simpleVarargs.getParameters();
        for (Parameter param : simpleParams) {
            System.out.println("Parameter: " + param.getName());
            System.out.println("  Type: " + param.getType().getName());
            System.out.println("  Is varargs: " + param.isVarArgs());
            System.out.println("  Is array: " + param.getType().isArray());
        }

        // Test mixed varargs
        Method mixedVarargs = Test.class.getMethod("mixedVarargs", int.class, String[].class);
        System.out.println("\nMixed varargs method: " + mixedVarargs.getName());
        System.out.println("Is varargs: " + mixedVarargs.isVarArgs());

        Parameter[] mixedParams = mixedVarargs.getParameters();
        for (int i = 0; i < mixedParams.length; i++) {
            Parameter param = mixedParams[i];
            System.out.println("Parameter " + i + ": " + param.getName());
            System.out.println("  Type: " + param.getType().getName());
            System.out.println("  Is varargs: " + param.isVarArgs());
        }

        // Test varargs method invocation
        System.out.println("\n=== Varargs Method Invocation ===");

        // Invoke with array
        simpleVarargs.invoke(null, (Object) new String[]{"a", "b", "c"});

        // Invoke with individual arguments (reflection handles conversion)
        mixedVarargs.invoke(null, 42, new String[]{"x", "y"});

        // Test primitive varargs
        Method primitiveVarargs = Test.class.getMethod("primitiveVarargs", int[].class);
        primitiveVarargs.invoke(null, (Object) new int[]{1, 2, 3, 4, 5});

        // Test object varargs
        Method objectVarargs = Test.class.getMethod("objectVarargs", Object[].class);
        objectVarargs.invoke(null, (Object) new Object[]{"string", 42, true});

        // Test default methods in interfaces
        System.out.println("\n=== Default Methods in Interfaces ===");

        Class<?> interfaceClass = DefaultMethodInterface.class;
        Method[] interfaceMethods = interfaceClass.getDeclaredMethods();

        for (Method method : interfaceMethods) {
            System.out.println("Interface method: " + method.getName());
            System.out.println("  Is default: " + method.isDefault());
            System.out.println("  Is abstract: " + Modifier.isAbstract(method.getModifiers()));
            System.out.println("  Is static: " + Modifier.isStatic(method.getModifiers()));
            System.out.println("  Modifiers: " + Modifier.toString(method.getModifiers()));
            System.out.println();
        }

        // Test default method invocation
        System.out.println("=== Default Method Invocation ===");
        DefaultMethodImpl impl = new DefaultMethodImpl();

        Method abstractMethod = interfaceClass.getMethod("abstractMethod");
        Method defaultMethod = interfaceClass.getMethod("defaultMethod");
        Method defaultWithReturn = interfaceClass.getMethod("defaultMethodWithReturn");
        Method staticMethod = interfaceClass.getMethod("staticInterfaceMethod");

        // Invoke abstract method (implemented)
        abstractMethod.invoke(impl);

        // Invoke overridden default method
        defaultMethod.invoke(impl);

        // Invoke default method with return value
        Object returnValue = defaultWithReturn.invoke(impl);
        System.out.println("Default method return value: " + returnValue);

        // Invoke static interface method
        staticMethod.invoke(null);

        // Test accessing original default method implementation
        System.out.println("\n=== Original Default Method Access ===");

        // Create an instance that doesn't override the default method
        DefaultMethodInterface directImpl = new DefaultMethodInterface() {
            @Override
            public void abstractMethod() {
                System.out.println("Direct abstract implementation");
            }
        };

        // Invoke default method on non-overriding implementation
        defaultMethod.invoke(directImpl);
        Object directReturnValue = defaultWithReturn.invoke(directImpl);
        System.out.println("Direct default method return: " + directReturnValue);

        // Test functional interface
        System.out.println("\n=== Functional Interface ===");

        Class<?> functionalClass = FunctionalInterfaceTest.class;
        System.out.println("Is functional interface: " + functionalClass.isAnnotationPresent(FunctionalInterface.class));

        Method[] functionalMethods = functionalClass.getDeclaredMethods();
        int abstractCount = 0;
        int defaultCount = 0;

        for (Method method : functionalMethods) {
            System.out.println("Functional interface method: " + method.getName());
            System.out.println("  Is default: " + method.isDefault());
            System.out.println("  Is abstract: " + Modifier.isAbstract(method.getModifiers()));

            if (method.isDefault()) {
                defaultCount++;
            } else if (Modifier.isAbstract(method.getModifiers())) {
                abstractCount++;
            }
            System.out.println();
        }

        System.out.println("Abstract methods in functional interface: " + abstractCount);
        System.out.println("Default methods in functional interface: " + defaultCount);

        // Test method parameter types with varargs
        System.out.println("\n=== Varargs Parameter Type Analysis ===");

        Method[] varargsMethodsArray = {simpleVarargs, mixedVarargs, primitiveVarargs, objectVarargs};

        for (Method method : varargsMethodsArray) {
            System.out.println("Method: " + method.getName());
            Class<?>[] paramTypes = method.getParameterTypes();
            Type[] genericParamTypes = method.getGenericParameterTypes();

            for (int i = 0; i < paramTypes.length; i++) {
                System.out.println("  Param " + i + ":");
                System.out.println("    Raw type: " + paramTypes[i].getName());
                System.out.println("    Generic type: " + genericParamTypes[i].getTypeName());
                System.out.println("    Is array: " + paramTypes[i].isArray());
                if (paramTypes[i].isArray()) {
                    System.out.println("    Component type: " + paramTypes[i].getComponentType().getName());
                }
            }
            System.out.println();
        }

        // Test varargs with null and empty arrays
        System.out.println("\n=== Varargs Edge Cases ===");

        // Invoke with null
        try {
            simpleVarargs.invoke(null, (Object) null);
            System.out.println("Successfully invoked varargs with null");
        } catch (Exception e) {
            System.out.println("Failed to invoke varargs with null: " + e.getClass().getSimpleName());
        }

        // Invoke with empty array
        simpleVarargs.invoke(null, (Object) new String[0]);
        System.out.println("Successfully invoked varargs with empty array");

        // Test method resolution with varargs
        System.out.println("\n=== Varargs Method Resolution ===");

        // This tests how reflection resolves varargs methods
        try {
            Method resolved1 = Test.class.getMethod("simpleVarargs", String[].class);
            System.out.println("Resolved varargs method: " + resolved1.getName());
        } catch (NoSuchMethodException e) {
            System.out.println("Failed to resolve varargs method: " + e.getMessage());
        }
    }
}
