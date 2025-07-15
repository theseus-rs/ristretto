import java.lang.annotation.*;

// Type-specific annotations
@Retention(RetentionPolicy.RUNTIME)
@Target(ElementType.TYPE)
@interface TypeOnly {
    String value() default "type";
}

@Retention(RetentionPolicy.RUNTIME)
@Target(ElementType.FIELD)
@interface FieldOnly {
    String value() default "field";
}

@Retention(RetentionPolicy.RUNTIME)
@Target(ElementType.METHOD)
@interface MethodOnly {
    String value() default "method";
}

@Retention(RetentionPolicy.RUNTIME)
@Target(ElementType.PARAMETER)
@interface ParameterOnly {
    String value() default "parameter";
}

@Retention(RetentionPolicy.RUNTIME)
@Target(ElementType.CONSTRUCTOR)
@interface ConstructorOnly {
    String value() default "constructor";
}

@Retention(RetentionPolicy.RUNTIME)
@Target(ElementType.LOCAL_VARIABLE)
@interface LocalVariableOnly {
    String value() default "local";
}

@Retention(RetentionPolicy.RUNTIME)
@Target({ElementType.TYPE, ElementType.METHOD})
@interface TypeAndMethod {
    String value() default "type_and_method";
}

@TypeOnly("TestClass")
@TypeAndMethod("OnClass")
public class Test {

    @FieldOnly("testField")
    private String field;

    @ConstructorOnly("testConstructor")
    public Test() {
    }

    @MethodOnly("testMethod")
    @TypeAndMethod("OnMethod")
    public void annotatedMethod(@ParameterOnly("param1") String param) {
        @LocalVariableOnly("localVar")
        String local = "test";
        System.out.println("Local variable: " + local);
    }

    public static void main(String[] args) {
        System.out.println("=== Target Types Test ===");

        Class<?> clazz = Test.class;

        // Test class annotations
        System.out.println("Class has TypeOnly: " + clazz.isAnnotationPresent(TypeOnly.class));
        System.out.println("Class has TypeAndMethod: " + clazz.isAnnotationPresent(TypeAndMethod.class));

        TypeOnly typeOnly = clazz.getAnnotation(TypeOnly.class);
        if (typeOnly != null) {
            System.out.println("TypeOnly value: " + typeOnly.value());
        }

        TypeAndMethod typeAndMethod = clazz.getAnnotation(TypeAndMethod.class);
        if (typeAndMethod != null) {
            System.out.println("TypeAndMethod on class value: " + typeAndMethod.value());
        }

        // Test field annotations
        try {
            java.lang.reflect.Field field = clazz.getDeclaredField("field");
            System.out.println("Field has FieldOnly: " + field.isAnnotationPresent(FieldOnly.class));

            FieldOnly fieldOnly = field.getAnnotation(FieldOnly.class);
            if (fieldOnly != null) {
                System.out.println("FieldOnly value: " + fieldOnly.value());
            }
        } catch (Exception e) {
            System.out.println("Error accessing field: " + e.getMessage());
        }

        // Test constructor annotations
        try {
            java.lang.reflect.Constructor<?> constructor = clazz.getDeclaredConstructor();
            System.out.println("Constructor has ConstructorOnly: " + constructor.isAnnotationPresent(ConstructorOnly.class));

            ConstructorOnly constructorOnly = constructor.getAnnotation(ConstructorOnly.class);
            if (constructorOnly != null) {
                System.out.println("ConstructorOnly value: " + constructorOnly.value());
            }
        } catch (Exception e) {
            System.out.println("Error accessing constructor: " + e.getMessage());
        }

        // Test method annotations
        try {
            java.lang.reflect.Method method = clazz.getDeclaredMethod("annotatedMethod", String.class);
            System.out.println("Method has MethodOnly: " + method.isAnnotationPresent(MethodOnly.class));
            System.out.println("Method has TypeAndMethod: " + method.isAnnotationPresent(TypeAndMethod.class));

            MethodOnly methodOnly = method.getAnnotation(MethodOnly.class);
            if (methodOnly != null) {
                System.out.println("MethodOnly value: " + methodOnly.value());
            }

            TypeAndMethod methodTypeAndMethod = method.getAnnotation(TypeAndMethod.class);
            if (methodTypeAndMethod != null) {
                System.out.println("TypeAndMethod on method value: " + methodTypeAndMethod.value());
            }

            // Test parameter annotations
            Annotation[][] paramAnnotations = method.getParameterAnnotations();
            System.out.println("Parameter annotation arrays: " + paramAnnotations.length);
            if (paramAnnotations.length > 0) {
                System.out.println("First parameter annotations: " + paramAnnotations[0].length);
                for (Annotation ann : paramAnnotations[0]) {
                    if (ann instanceof ParameterOnly) {
                        System.out.println("ParameterOnly value: " + ((ParameterOnly) ann).value());
                    }
                }
            }
        } catch (Exception e) {
            System.out.println("Error accessing method: " + e.getMessage());
        }

        // Note: Local variable annotations are not accessible via reflection
        System.out.println("Note: Local variable annotations are not accessible via reflection");
    }
}
