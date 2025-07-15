/** Test annotation reflection operations. */
import java.lang.annotation.*;

public class Test {
    @Retention(RetentionPolicy.RUNTIME)
    @Target({ElementType.TYPE, ElementType.METHOD, ElementType.FIELD, ElementType.PARAMETER})
    @interface CustomAnnotation {
        String value() default "default";
        int number() default 42;
        String[] array() default {"a", "b"};
        Class<?> type() default String.class;
    }

    @Retention(RetentionPolicy.RUNTIME)
    @Target(ElementType.METHOD)
    @interface MethodAnnotation {
        String description();
    }

    @Retention(RetentionPolicy.RUNTIME)
    @Target(ElementType.PARAMETER)
    @interface ParameterAnnotation {
        String name();
    }

    @CustomAnnotation(value = "class", number = 100, array = {"x", "y", "z"}, type = Integer.class)
    static class AnnotatedClass {
        @CustomAnnotation("field")
        public String annotatedField;

        public String unannotatedField;

        @MethodAnnotation(description = "test method")
        @CustomAnnotation(value = "method", number = 200)
        public void annotatedMethod(
            @ParameterAnnotation(name = "param1") String arg1,
            String arg2,
            @ParameterAnnotation(name = "param3") int arg3
        ) {}

        public void unannotatedMethod() {}
    }

    public static void main(String[] args) throws Exception {
        Class<?> clazz = Class.forName("Test$AnnotatedClass");

        // Test class annotations
        System.out.println("=== Class Annotations ===");
        Annotation[] classAnnotations = clazz.getAnnotations();
        System.out.println("Class annotations count: " + classAnnotations.length);

        CustomAnnotation classAnnotation = clazz.getAnnotation(CustomAnnotation.class);
        if (classAnnotation != null) {
            System.out.println("Class annotation value: " + classAnnotation.value());
            System.out.println("Class annotation number: " + classAnnotation.number());
            System.out.println("Class annotation array length: " + classAnnotation.array().length);
            System.out.println("Class annotation type: " + classAnnotation.type().getName());
        }

        System.out.println("Class is annotated with CustomAnnotation: " + clazz.isAnnotationPresent(CustomAnnotation.class));

        // Test field annotations
        System.out.println("\n=== Field Annotations ===");
        java.lang.reflect.Field annotatedField = clazz.getField("annotatedField");
        Annotation[] fieldAnnotations = annotatedField.getAnnotations();
        System.out.println("Annotated field annotations count: " + fieldAnnotations.length);

        CustomAnnotation fieldAnnotation = annotatedField.getAnnotation(CustomAnnotation.class);
        if (fieldAnnotation != null) {
            System.out.println("Field annotation value: " + fieldAnnotation.value());
        }

        java.lang.reflect.Field unannotatedField = clazz.getField("unannotatedField");
        System.out.println("Unannotated field annotations count: " + unannotatedField.getAnnotations().length);

        // Test method annotations
        System.out.println("\n=== Method Annotations ===");
        java.lang.reflect.Method annotatedMethod = clazz.getMethod("annotatedMethod", String.class, String.class, int.class);
        Annotation[] methodAnnotations = annotatedMethod.getAnnotations();
        System.out.println("Annotated method annotations count: " + methodAnnotations.length);

        MethodAnnotation methodAnnotation = annotatedMethod.getAnnotation(MethodAnnotation.class);
        if (methodAnnotation != null) {
            System.out.println("Method annotation description: " + methodAnnotation.description());
        }

        CustomAnnotation methodCustomAnnotation = annotatedMethod.getAnnotation(CustomAnnotation.class);
        if (methodCustomAnnotation != null) {
            System.out.println("Method custom annotation value: " + methodCustomAnnotation.value());
            System.out.println("Method custom annotation number: " + methodCustomAnnotation.number());
        }

        // Test parameter annotations
        System.out.println("\n=== Parameter Annotations ===");
        Annotation[][] parameterAnnotations = annotatedMethod.getParameterAnnotations();
        System.out.println("Parameter count: " + parameterAnnotations.length);

        for (int i = 0; i < parameterAnnotations.length; i++) {
            System.out.println("Parameter " + i + " annotations count: " + parameterAnnotations[i].length);
            for (Annotation annotation : parameterAnnotations[i]) {
                if (annotation instanceof ParameterAnnotation) {
                    ParameterAnnotation paramAnnotation = (ParameterAnnotation) annotation;
                    System.out.println("Parameter " + i + " name: " + paramAnnotation.name());
                }
            }
        }

        // Test annotation type introspection
        System.out.println("\n=== Annotation Type Introspection ===");
        Class<?> annotationType = CustomAnnotation.class;
        System.out.println("Is annotation type: " + annotationType.isAnnotation());

        java.lang.reflect.Method[] annotationMethods = annotationType.getDeclaredMethods();
        System.out.println("Annotation methods count: " + annotationMethods.length);
        for (java.lang.reflect.Method method : annotationMethods) {
            System.out.println("Annotation method: " + method.getName() + " -> " + method.getReturnType().getName());
            Object defaultValue = method.getDefaultValue();
            if (defaultValue != null) {
                if (defaultValue.getClass().isArray()) {
                    String[] array = (String[]) defaultValue;
                    System.out.println("Default value (array): [" + String.join(", ", array) + "]");
                } else {
                    System.out.println("Default value: " + defaultValue);
                }
            }
        }
    }
}
