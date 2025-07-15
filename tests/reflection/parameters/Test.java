/** Test parameter reflection operations. */
import java.lang.reflect.*;

public class Test {
    static class TestClass {
        public void methodWithParameters(
            String stringParam,
            int intParam,
            final boolean finalParam,
            String... varArgsParam
        ) {}

        public void methodWithAnnotatedParameters(
            @Deprecated String annotatedParam,
            String normalParam
        ) {}

        public <T> void genericMethod(T genericParam, java.util.List<T> listParam) {}

        private TestClass(String constructorParam, int... varArgsConstructorParam) {}
    }

    public static void main(String[] args) throws Exception {
        Class<?> clazz = TestClass.class;

        // Test basic parameter information
        System.out.println("=== Basic Parameter Information ===");
        Method method = clazz.getMethod("methodWithParameters", String.class, int.class, boolean.class, String[].class);

        Parameter[] parameters = method.getParameters();
        System.out.println("Parameter count: " + parameters.length);

        for (int i = 0; i < parameters.length; i++) {
            Parameter param = parameters[i];
            System.out.println("Parameter " + i + ":");
            System.out.println("  Name: " + param.getName());
            System.out.println("  Type: " + param.getType().getName());
            System.out.println("  Modifiers: " + Modifier.toString(param.getModifiers()));
            System.out.println("  Is varargs: " + param.isVarArgs());
            System.out.println("  Is final: " + Modifier.isFinal(param.getModifiers()));
        }

        // Test parameter types
        System.out.println("\n=== Parameter Types ===");
        Class<?>[] parameterTypes = method.getParameterTypes();
        System.out.println("Parameter types count: " + parameterTypes.length);
        for (int i = 0; i < parameterTypes.length; i++) {
            System.out.println("Parameter type " + i + ": " + parameterTypes[i].getName());
        }

        // Test generic parameter types
        System.out.println("\n=== Generic Parameter Types ===");
        Type[] genericParameterTypes = method.getGenericParameterTypes();
        for (int i = 0; i < genericParameterTypes.length; i++) {
            System.out.println("Generic parameter type " + i + ": " + genericParameterTypes[i].getTypeName());
        }

        // Test parameter annotations
        System.out.println("\n=== Parameter Annotations ===");
        Method annotatedMethod = clazz.getMethod("methodWithAnnotatedParameters", String.class, String.class);
        Parameter[] annotatedParams = annotatedMethod.getParameters();

        for (int i = 0; i < annotatedParams.length; i++) {
            Parameter param = annotatedParams[i];
            java.lang.annotation.Annotation[] annotations = param.getAnnotations();
            System.out.println("Parameter " + i + " annotations count: " + annotations.length);

            if (param.isAnnotationPresent(Deprecated.class)) {
                System.out.println("Parameter " + i + " is deprecated");
            }
        }

        // Test varargs parameter
        System.out.println("\n=== Varargs Parameters ===");
        for (Parameter param : parameters) {
            if (param.isVarArgs()) {
                System.out.println("Varargs parameter: " + param.getName());
                System.out.println("Varargs type: " + param.getType().getName());
                System.out.println("Is array: " + param.getType().isArray());
                System.out.println("Component type: " + param.getType().getComponentType().getName());
            }
        }

        // Test generic method parameters
        System.out.println("\n=== Generic Method Parameters ===");
        Method genericMethod = clazz.getMethod("genericMethod", Object.class, java.util.List.class);
        Type[] genericTypes = genericMethod.getGenericParameterTypes();

        for (int i = 0; i < genericTypes.length; i++) {
            Type genericType = genericTypes[i];
            System.out.println("Generic parameter " + i + ": " + genericType.getTypeName());

            if (genericType instanceof TypeVariable) {
                TypeVariable<?> typeVar = (TypeVariable<?>) genericType;
                System.out.println("  Type variable name: " + typeVar.getName());
            } else if (genericType instanceof ParameterizedType) {
                ParameterizedType paramType = (ParameterizedType) genericType;
                System.out.println("  Parameterized type: " + paramType.getRawType().getTypeName());
                Type[] typeArgs = paramType.getActualTypeArguments();
                for (Type typeArg : typeArgs) {
                    System.out.println("  Type argument: " + typeArg.getTypeName());
                }
            }
        }

        // Test constructor parameters
        System.out.println("\n=== Constructor Parameters ===");
        Constructor<?> constructor = clazz.getDeclaredConstructor(String.class, int[].class);
        Parameter[] constructorParams = constructor.getParameters();

        System.out.println("Constructor parameter count: " + constructorParams.length);
        for (int i = 0; i < constructorParams.length; i++) {
            Parameter param = constructorParams[i];
            System.out.println("Constructor parameter " + i + ": " + param.getName());
            System.out.println("  Type: " + param.getType().getName());
            System.out.println("  Is varargs: " + param.isVarArgs());
        }

        // Test parameter count
        System.out.println("\n=== Parameter Count ===");
        System.out.println("Method parameter count: " + method.getParameterCount());
        System.out.println("Constructor parameter count: " + constructor.getParameterCount());

        // Test parameter naming (may require compilation with -parameters flag)
        System.out.println("\n=== Parameter Naming ===");
        for (Parameter param : parameters) {
            System.out.println("Parameter name available: " + param.isNamePresent());
            if (param.isNamePresent()) {
                System.out.println("Parameter name: " + param.getName());
            } else {
                System.out.println("Parameter synthetic name: " + param.getName());
            }
        }

        // Test parameter with complex types
        System.out.println("\n=== Complex Parameter Types ===");
        // Create a method reference to test parameter reflection on complex types
        java.util.function.Function<String, Integer> func = Integer::valueOf;
        Method funcMethod = func.getClass().getMethod("apply", Object.class);
        Parameter[] funcParams = funcMethod.getParameters();

        for (Parameter param : funcParams) {
            System.out.println("Function parameter: " + param.getName());
            System.out.println("Function parameter type: " + param.getType().getName());
        }
    }
}
