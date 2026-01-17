/** Test varargs method reflection metadata. */
import java.lang.reflect.*;
import java.util.Arrays;
import java.util.Comparator;

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

        // Test default methods in interfaces
        System.out.println("\n=== Default Methods in Interfaces ===");

        Class<?> interfaceClass = DefaultMethodInterface.class;
        Method[] interfaceMethods = interfaceClass.getDeclaredMethods();
        // Sort methods for deterministic output
        Arrays.sort(interfaceMethods, Comparator.comparing(Method::getName));

        for (Method method : interfaceMethods) {
            System.out.println("Interface method: " + method.getName());
            System.out.println("  Is default: " + method.isDefault());
            System.out.println("  Is abstract: " + Modifier.isAbstract(method.getModifiers()));
            System.out.println("  Is static: " + Modifier.isStatic(method.getModifiers()));
            System.out.println("  Modifiers: " + Modifier.toString(method.getModifiers()));
            System.out.println();
        }

        // Test functional interface
        System.out.println("=== Functional Interface ===");

        Class<?> functionalClass = FunctionalInterfaceTest.class;
        System.out.println("Is functional interface: " + functionalClass.isAnnotationPresent(FunctionalInterface.class));

        Method[] functionalMethods = functionalClass.getDeclaredMethods();
        // Sort methods for deterministic output
        Arrays.sort(functionalMethods, Comparator.comparing(Method::getName));
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

        Method[] varargsMethodsArray = {simpleVarargs, mixedVarargs};

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
    }
}
