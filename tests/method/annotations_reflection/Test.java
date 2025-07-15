/** Test method annotations and reflection behavior. */
import java.lang.annotation.*;
import java.lang.reflect.*;

public class Test {
    // Custom annotations for testing
    @Retention(RetentionPolicy.RUNTIME)
    @Target(ElementType.METHOD)
    @interface TestAnnotation {
        String value() default "default";
        int priority() default 0;
    }

    @Retention(RetentionPolicy.RUNTIME)
    @Target(ElementType.PARAMETER)
    @interface ParamAnnotation {
        String name();
    }

    @Retention(RetentionPolicy.CLASS)
    @Target(ElementType.METHOD)
    @interface CompileTimeAnnotation {
        String info();
    }

    // Methods with various annotations
    @TestAnnotation("important")
    public void annotatedMethod() {
        System.out.println("Annotated method called");
    }

    @TestAnnotation(value = "critical", priority = 10)
    public static void criticalMethod() {
        System.out.println("Critical method called");
    }

    @CompileTimeAnnotation(info = "compile-time only")
    public void compileTimeAnnotatedMethod() {
        System.out.println("Compile-time annotated method");
    }

    @TestAnnotation("parameterized")
    public void methodWithAnnotatedParams(
        @ParamAnnotation(name = "first") String param1,
        @ParamAnnotation(name = "second") int param2) {
        System.out.println("Method with annotated params: " + param1 + ", " + param2);
    }

    @Deprecated
    @TestAnnotation("deprecated")
    public void deprecatedMethod() {
        System.out.println("Deprecated method called");
    }

    // Method for reflection testing
    public static void reflectionTest() throws Exception {
        System.out.println("=== Testing Method Reflection ===");

        Class<?> clazz = Test.class;
        Method[] methods = clazz.getDeclaredMethods();

        for (Method method : methods) {
            if (method.isAnnotationPresent(TestAnnotation.class)) {
                TestAnnotation annotation = method.getAnnotation(TestAnnotation.class);
                System.out.println("Method: " + method.getName());
                System.out.println("  Annotation value: " + annotation.value());
                System.out.println("  Annotation priority: " + annotation.priority());

                // Test parameter annotations
                Parameter[] params = method.getParameters();
                for (int i = 0; i < params.length; i++) {
                    Parameter param = params[i];
                    if (param.isAnnotationPresent(ParamAnnotation.class)) {
                        ParamAnnotation paramAnnot = param.getAnnotation(ParamAnnotation.class);
                        System.out.println("  Parameter " + i + " name: " + paramAnnot.name());
                    }
                }

                // Test method invocation via reflection
                if (method.getParameterCount() == 0) {
                    if (Modifier.isStatic(method.getModifiers())) {
                        method.invoke(null);
                    } else {
                        Test instance = new Test();
                        method.invoke(instance);
                    }
                } else if (method.getName().equals("methodWithAnnotatedParams")) {
                    Test instance = new Test();
                    method.invoke(instance, "reflected", 42);
                }
                System.out.println();
            }
        }
    }

    // Test method modifiers via reflection
    public static void testMethodModifiers() throws Exception {
        System.out.println("=== Testing Method Modifiers ===");

        Class<?> clazz = Test.class;

        Method[] methods = {
            clazz.getMethod("annotatedMethod"),
            clazz.getMethod("criticalMethod"),
            clazz.getMethod("main", String[].class)
        };

        for (Method method : methods) {
            System.out.println("Method: " + method.getName());
            System.out.println("  Public: " + Modifier.isPublic(method.getModifiers()));
            System.out.println("  Static: " + Modifier.isStatic(method.getModifiers()));
            System.out.println("  Final: " + Modifier.isFinal(method.getModifiers()));
            System.out.println("  Abstract: " + Modifier.isAbstract(method.getModifiers()));
            System.out.println("  Synchronized: " + Modifier.isSynchronized(method.getModifiers()));
            System.out.println("  Native: " + Modifier.isNative(method.getModifiers()));
            System.out.println();
        }
    }

    // Test method parameter types and return types
    public static void testMethodSignatures() throws Exception {
        System.out.println("=== Testing Method Signatures ===");

        Method method = Test.class.getMethod("methodWithAnnotatedParams", String.class, int.class);

        System.out.println("Method: " + method.getName());
        System.out.println("Return type: " + method.getReturnType().getSimpleName());

        Class<?>[] paramTypes = method.getParameterTypes();
        System.out.println("Parameter types:");
        for (int i = 0; i < paramTypes.length; i++) {
            System.out.println("  " + i + ": " + paramTypes[i].getSimpleName());
        }

        Type[] genericParamTypes = method.getGenericParameterTypes();
        System.out.println("Generic parameter types:");
        for (int i = 0; i < genericParamTypes.length; i++) {
            System.out.println("  " + i + ": " + genericParamTypes[i].getTypeName());
        }
    }

    public static void main(String[] args) throws Exception {
        reflectionTest();
        testMethodModifiers();
        testMethodSignatures();

        System.out.println("=== Testing Annotation Inheritance ===");
        Test test = new Test();
        test.deprecatedMethod(); // Should show deprecation warning at compile time
    }
}
