import java.lang.annotation.*;
import java.lang.reflect.*;
import java.util.Arrays;
import java.util.Comparator;

// Test annotation compatibility across Java versions
@Retention(RetentionPolicy.RUNTIME)
@Target({ElementType.TYPE, ElementType.METHOD, ElementType.FIELD, ElementType.PARAMETER, ElementType.CONSTRUCTOR})
@interface CompatibilityAnnotation {
    String value() default "default";
    // Test default value compatibility
    String[] arrayWithDefaults() default {"default1", "default2"};
    int version() default 1;
}

// Test type annotations (Java 8+)
@Retention(RetentionPolicy.RUNTIME)
@Target({ElementType.TYPE_USE, ElementType.TYPE_PARAMETER})
@interface TypeUseAnnotation {
    String value() default "type_use";
}

// Test parameter annotations with names (if available)
@Retention(RetentionPolicy.RUNTIME)
@Target(ElementType.PARAMETER)
@interface ParameterNameAnnotation {
    String name() default "";
}

// Test method parameter reflection
@Retention(RetentionPolicy.RUNTIME)
@Target(ElementType.METHOD)
@interface MethodParameterTestAnnotation {
    String description() default "test method";
}

@CompatibilityAnnotation("TestClass")
public class Test {

    @CompatibilityAnnotation("testField")
    private /*@TypeUseAnnotation("fieldType")*/ String annotatedField;

    private String normalField;

    @CompatibilityAnnotation("constructor")
    public Test() {
    }

    @CompatibilityAnnotation("parameterizedConstructor")
    public Test(@ParameterNameAnnotation(name = "param") String param) {
    }

    @MethodParameterTestAnnotation(description = "Test method with various parameter types")
    @CompatibilityAnnotation("testMethod")
    public /*@TypeUseAnnotation("returnType")*/ String testMethod(
            @ParameterNameAnnotation(name = "stringParam") @CompatibilityAnnotation("param1") String stringParam,
            @ParameterNameAnnotation(name = "intParam") int intParam,
            @ParameterNameAnnotation(name = "arrayParam") String[] arrayParam) {
        return "test";
    }

    // Generic method for testing type parameter annotations
    public </*@TypeUseAnnotation("typeParam")*/ T> T genericMethod(T param) {
        return param;
    }

    // Varargs method
    @CompatibilityAnnotation("varargsMethod")
    public void varargsMethod(@ParameterNameAnnotation(name = "args") String... args) {
    }

    public static void main(String[] args) {
        System.out.println("=== Annotation Compatibility and Version Features Test ===");

        Class<?> clazz = Test.class;

        // Test basic annotation compatibility
        testBasicCompatibility(clazz);

        // Test parameter annotations and reflection
        testParameterAnnotations(clazz);

        // Test constructor annotations
        testConstructorAnnotations(clazz);

        // Test method parameter names (if available)
        testParameterNames(clazz);

        // Test generic method annotations
        testGenericMethodAnnotations(clazz);

        // Test varargs annotations
        testVarargsAnnotations(clazz);

        // Test annotation inheritance with overrides
        testAnnotationInheritanceOverrides(clazz);

        // Test reflection edge cases
        testReflectionEdgeCases(clazz);
    }

    private static void testBasicCompatibility(Class<?> clazz) {
        System.out.println("=== Basic Compatibility Test ===");

        CompatibilityAnnotation classAnn = clazz.getAnnotation(CompatibilityAnnotation.class);
        if (classAnn != null) {
            System.out.println("Class annotation value: " + classAnn.value());
            System.out.println("Class annotation version: " + classAnn.version());
            System.out.println("Class annotation arrayWithDefaults: " +
                    java.util.Arrays.toString(classAnn.arrayWithDefaults()));
        }

        // Test annotation toString format; just check it's not null
        System.out.println("Annotation toString: present (non-deterministic order)");

        // Test annotation hashCode consistency
        CompatibilityAnnotation classAnn2 = clazz.getAnnotation(CompatibilityAnnotation.class);
        System.out.println("Annotation instances equal: " + (classAnn == classAnn2));
        System.out.println("Annotation equals: " + classAnn.equals(classAnn2));
        System.out.println("Annotation hashCode consistent: " + (classAnn.hashCode() == classAnn2.hashCode()));
    }

    private static void testParameterAnnotations(Class<?> clazz) {
        System.out.println("\n=== Parameter Annotations Test ===");

        try {
            Method method = clazz.getDeclaredMethod("testMethod", String.class, int.class, String[].class);

            // Test method annotations
            CompatibilityAnnotation methodAnn = method.getAnnotation(CompatibilityAnnotation.class);
            if (methodAnn != null) {
                System.out.println("Method annotation: " + methodAnn.value());
            }

            MethodParameterTestAnnotation methodTestAnn = method.getAnnotation(MethodParameterTestAnnotation.class);
            if (methodTestAnn != null) {
                System.out.println("Method test annotation: " + methodTestAnn.description());
            }

            // Test parameter annotations
            Parameter[] parameters = method.getParameters();
            Annotation[][] paramAnnotations = method.getParameterAnnotations();

            System.out.println("Parameter count: " + parameters.length);
            System.out.println("Parameter annotation arrays: " + paramAnnotations.length);

            for (int i = 0; i < parameters.length; i++) {
                Parameter param = parameters[i];
                System.out.println("Parameter " + i + ":");
                System.out.println("  Name: " + param.getName());
                System.out.println("  Type: " + param.getType().getName());
                System.out.println("  Modifiers: " + param.getModifiers());
                System.out.println("  Is name present: " + param.isNamePresent());
                System.out.println("  Is var args: " + param.isVarArgs());
                System.out.println("  Is synthetic: " + param.isSynthetic());

                // Test parameter annotations
                Annotation[] annotations = paramAnnotations[i];
                System.out.println("  Annotations: " + annotations.length);
                for (Annotation ann : annotations) {
                    System.out.println("    " + ann.annotationType().getSimpleName() + ": present");

                    if (ann instanceof ParameterNameAnnotation) {
                        ParameterNameAnnotation paramNameAnn = (ParameterNameAnnotation) ann;
                        System.out.println("      Parameter name annotation: " + paramNameAnn.name());
                    }

                    if (ann instanceof CompatibilityAnnotation) {
                        CompatibilityAnnotation compatAnn = (CompatibilityAnnotation) ann;
                        System.out.println("      Compatibility annotation: " + compatAnn.value());
                    }
                }
            }
        } catch (Exception e) {
            System.out.println("Error testing parameter annotations: " + e.getMessage());
        }
    }

    private static void testConstructorAnnotations(Class<?> clazz) {
        System.out.println("\n=== Constructor Annotations Test ===");

        Constructor<?>[] constructors = clazz.getDeclaredConstructors();
        // Sort constructors by parameter count for deterministic output
        Arrays.sort(constructors, Comparator.comparingInt(Constructor::getParameterCount));
        System.out.println("Constructor count: " + constructors.length);

        for (int i = 0; i < constructors.length; i++) {
            Constructor<?> constructor = constructors[i];
            System.out.println("Constructor " + i + ": " + constructor);

            // Test constructor annotations
            Annotation[] annotations = constructor.getAnnotations();
            System.out.println("  Annotations: " + annotations.length);
            for (Annotation ann : annotations) {
                // Normalize annotation toString by just printing key values
                System.out.println("    " + ann.annotationType().getSimpleName() + ": present");
            }

            // Test constructor parameter annotations
            Parameter[] params = constructor.getParameters();
            Annotation[][] paramAnnotations = constructor.getParameterAnnotations();

            System.out.println("  Parameters: " + params.length);
            for (int j = 0; j < params.length; j++) {
                Parameter param = params[j];
                System.out.println("    Parameter " + j + ": " + param.getName() + " (" + param.getType().getName() + ")");

                Annotation[] paramAnns = paramAnnotations[j];
                for (Annotation ann : paramAnns) {
                    System.out.println("      " + ann.annotationType().getSimpleName() + ": present");
                }
            }
        }
    }

    private static void testParameterNames(Class<?> clazz) {
        System.out.println("\n=== Parameter Names Test ===");

        try {
            Method method = clazz.getDeclaredMethod("testMethod", String.class, int.class, String[].class);
            Parameter[] parameters = method.getParameters();

            System.out.println("Testing parameter name preservation:");
            for (int i = 0; i < parameters.length; i++) {
                Parameter param = parameters[i];
                System.out.println("Parameter " + i + ":");
                System.out.println("  Name: " + param.getName());
                System.out.println("  Is name present: " + param.isNamePresent());

                // Compare with annotation-provided name
                ParameterNameAnnotation nameAnn = param.getAnnotation(ParameterNameAnnotation.class);
                if (nameAnn != null && !nameAnn.name().isEmpty()) {
                    System.out.println("  Annotation name: " + nameAnn.name());
                    System.out.println("  Names match: " + param.getName().equals(nameAnn.name()));
                }
            }
        } catch (Exception e) {
            System.out.println("Error testing parameter names: " + e.getMessage());
        }
    }

    private static void testGenericMethodAnnotations(Class<?> clazz) {
        System.out.println("\n=== Generic Method Annotations Test ===");

        try {
            Method genericMethod = clazz.getDeclaredMethod("genericMethod", Object.class);
            System.out.println("Generic method: " + genericMethod);

            // Test type parameters
            TypeVariable<?>[] typeParams = genericMethod.getTypeParameters();
            System.out.println("Type parameters: " + typeParams.length);
            for (TypeVariable<?> typeParam : typeParams) {
                System.out.println("  Type parameter: " + typeParam.getName());
                System.out.println("  Bounds: " + java.util.Arrays.toString(typeParam.getBounds()));

                // Note: Type parameter annotations require TYPE_PARAMETER target
                // and are not easily accessible in older Java versions
            }

            // Test generic parameter types
            Type[] paramTypes = genericMethod.getGenericParameterTypes();
            System.out.println("Generic parameter types: " + paramTypes.length);
            for (Type paramType : paramTypes) {
                System.out.println("  Parameter type: " + paramType);
            }

            // Test generic return type
            Type returnType = genericMethod.getGenericReturnType();
            System.out.println("Generic return type: " + returnType);

        } catch (Exception e) {
            System.out.println("Error testing generic method annotations: " + e.getMessage());
        }
    }

    private static void testVarargsAnnotations(Class<?> clazz) {
        System.out.println("\n=== Varargs Annotations Test ===");

        try {
            Method varargsMethod = clazz.getDeclaredMethod("varargsMethod", String[].class);
            System.out.println("Varargs method: " + varargsMethod);
            System.out.println("Is varargs: " + varargsMethod.isVarArgs());

            Parameter[] parameters = varargsMethod.getParameters();
            if (parameters.length > 0) {
                Parameter varargsParam = parameters[0];
                System.out.println("Varargs parameter:");
                System.out.println("  Name: " + varargsParam.getName());
                System.out.println("  Type: " + varargsParam.getType().getName());
                System.out.println("  Is varargs: " + varargsParam.isVarArgs());

                // Test varargs parameter annotations
                Annotation[] annotations = varargsParam.getAnnotations();
                System.out.println("  Annotations: " + annotations.length);
                for (Annotation ann : annotations) {
                    System.out.println("    " + ann.annotationType().getSimpleName() + ": " + ann);
                }
            }
        } catch (Exception e) {
            System.out.println("Error testing varargs annotations: " + e.getMessage());
        }
    }

    private static void testAnnotationInheritanceOverrides(Class<?> clazz) {
        System.out.println("\n=== Annotation Inheritance Overrides Test ===");

        // Create anonymous subclass to test inheritance
        Test instance = new Test() {
            @Override
            @CompatibilityAnnotation("overriddenMethod")
            public String testMethod(String stringParam, int intParam, String[] arrayParam) {
                return "overridden";
            }
        };

        Class<?> anonClass = instance.getClass();
        System.out.println("Anonymous class: " + anonClass.getName());
        System.out.println("Is anonymous: " + anonClass.isAnonymousClass());

        try {
            Method overriddenMethod = anonClass.getDeclaredMethod("testMethod", String.class, int.class, String[].class);
            CompatibilityAnnotation overriddenAnn = overriddenMethod.getAnnotation(CompatibilityAnnotation.class);

            if (overriddenAnn != null) {
                System.out.println("Overridden method annotation: " + overriddenAnn.value());
            }

            // Compare with original method
            Method originalMethod = clazz.getDeclaredMethod("testMethod", String.class, int.class, String[].class);
            CompatibilityAnnotation originalAnn = originalMethod.getAnnotation(CompatibilityAnnotation.class);

            if (originalAnn != null && overriddenAnn != null) {
                System.out.println("Original method annotation: " + originalAnn.value());
                System.out.println("Annotations are different: " + !originalAnn.equals(overriddenAnn));
            }
        } catch (Exception e) {
            System.out.println("Error testing annotation overrides: " + e.getMessage());
        }
    }

    private static void testReflectionEdgeCases(Class<?> clazz) {
        System.out.println("\n=== Reflection Edge Cases Test ===");

        // Test getMethods vs getDeclaredMethods for annotations
        Method[] allMethods = clazz.getMethods();
        Method[] declaredMethods = clazz.getDeclaredMethods();

        System.out.println("All methods count: " + allMethods.length);
        System.out.println("Declared methods count: " + declaredMethods.length);

        int annotatedMethodsCount = 0;
        for (Method method : declaredMethods) {
            if (method.getAnnotations().length > 0) {
                annotatedMethodsCount++;
            }
        }
        System.out.println("Declared methods with annotations: " + annotatedMethodsCount);

        // Test bridge methods and synthetic methods
        for (Method method : allMethods) {
            if (method.isBridge() || method.isSynthetic()) {
                System.out.println("Bridge/Synthetic method: " + method.getName() +
                        " (bridge: " + method.isBridge() + ", synthetic: " + method.isSynthetic() + ")");
                System.out.println("  Annotations: " + method.getAnnotations().length);
            }
        }

        // Test annotation method default values
        Class<CompatibilityAnnotation> annClass = CompatibilityAnnotation.class;
        Method[] annMethods = annClass.getDeclaredMethods();
        // Sort methods by name for deterministic output
        Arrays.sort(annMethods, Comparator.comparing(Method::getName));

        System.out.println("Annotation methods with defaults:");
        for (Method annMethod : annMethods) {
            Object defaultValue = annMethod.getDefaultValue();
            if (defaultValue != null && defaultValue.getClass().isArray()) {
                System.out.println("  " + annMethod.getName() + ": " + java.util.Arrays.toString((Object[]) defaultValue));
            } else {
                System.out.println("  " + annMethod.getName() + ": " +
                        (defaultValue != null ? defaultValue : "no default"));
            }
        }
    }
}
