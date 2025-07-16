import java.lang.annotation.*;
import java.lang.reflect.*;

// Test annotations for reflection
@Retention(RetentionPolicy.RUNTIME)
@Target({ElementType.TYPE, ElementType.METHOD, ElementType.FIELD, ElementType.PARAMETER, ElementType.CONSTRUCTOR})
@interface TestAnnotation {
    String value() default "test";
    int number() default 42;
    boolean flag() default true;
}

@Retention(RetentionPolicy.RUNTIME)
@Target(ElementType.METHOD)
@interface MethodInfo {
    String description();
    String[] tags() default {};
    int version() default 1;
}

@TestAnnotation(value = "ProcessingTest", number = 100, flag = false)
public class Test {

    @TestAnnotation("field")
    private String annotatedField;

    private String normalField;

    @TestAnnotation(value = "constructor", number = 0)
    public Test() {
    }

    @TestAnnotation("method1")
    @MethodInfo(description = "First test method", tags = {"test", "demo"}, version = 2)
    public void method1(@TestAnnotation("param") String parameter) {
    }

    @MethodInfo(description = "Second test method")
    public int method2(String param1, @TestAnnotation("param2") int param2) {
        return param2;
    }

    public void normalMethod() {
    }

    public static void main(String[] args) {
        System.out.println("=== Annotation Processing and Reflection Test ===");

        Class<?> clazz = Test.class;

        // Test annotation processing on class
        System.out.println("=== Class Annotation Processing ===");
        processClassAnnotations(clazz);

        // Test annotation processing on fields
        System.out.println("\n=== Field Annotation Processing ===");
        processFieldAnnotations(clazz);

        // Test annotation processing on constructors
        System.out.println("\n=== Constructor Annotation Processing ===");
        processConstructorAnnotations(clazz);

        // Test annotation processing on methods
        System.out.println("\n=== Method Annotation Processing ===");
        processMethodAnnotations(clazz);

        // Test annotation equality and hash codes
        System.out.println("\n=== Annotation Equality Test ===");
        testAnnotationEquality(clazz);

        // Test annotation toString
        System.out.println("\n=== Annotation toString Test ===");
        testAnnotationToString(clazz);
    }

    private static void processClassAnnotations(Class<?> clazz) {
        System.out.println("Processing class: " + clazz.getSimpleName());

        Annotation[] annotations = clazz.getAnnotations();
        System.out.println("Total annotations: " + annotations.length);

        for (Annotation annotation : annotations) {
            System.out.println("Found annotation: " + annotation.annotationType().getSimpleName());
            processAnnotation(annotation);
        }

        // Test specific annotation lookup
        TestAnnotation testAnn = clazz.getAnnotation(TestAnnotation.class);
        if (testAnn != null) {
            System.out.println("TestAnnotation found via getAnnotation()");
        }
    }

    private static void processFieldAnnotations(Class<?> clazz) {
        Field[] fields = clazz.getDeclaredFields();
        System.out.println("Processing " + fields.length + " fields");

        for (Field field : fields) {
            System.out.println("Field: " + field.getName());
            Annotation[] annotations = field.getAnnotations();
            System.out.println("  Annotations: " + annotations.length);

            for (Annotation annotation : annotations) {
                System.out.println("  Found: " + annotation.annotationType().getSimpleName());
                processAnnotation(annotation);
            }
        }
    }

    private static void processConstructorAnnotations(Class<?> clazz) {
        Constructor<?>[] constructors = clazz.getDeclaredConstructors();
        System.out.println("Processing " + constructors.length + " constructors");

        for (Constructor<?> constructor : constructors) {
            System.out.println("Constructor: " + constructor);
            Annotation[] annotations = constructor.getAnnotations();
            System.out.println("  Annotations: " + annotations.length);

            for (Annotation annotation : annotations) {
                System.out.println("  Found: " + annotation.annotationType().getSimpleName());
                processAnnotation(annotation);
            }
        }
    }

    private static void processMethodAnnotations(Class<?> clazz) {
        Method[] methods = clazz.getDeclaredMethods();
        System.out.println("Processing " + methods.length + " methods");

        for (Method method : methods) {
            if (method.getName().equals("main") || method.getName().startsWith("process") ||
                    method.getName().startsWith("test")) {
                continue; // Skip test helper methods
            }

            System.out.println("Method: " + method.getName());
            Annotation[] annotations = method.getAnnotations();
            System.out.println("  Method annotations: " + annotations.length);

            for (Annotation annotation : annotations) {
                System.out.println("  Found: " + annotation.annotationType().getSimpleName());
                processAnnotation(annotation);
            }

            // Process parameter annotations
            Annotation[][] paramAnnotations = method.getParameterAnnotations();
            System.out.println("  Parameters: " + paramAnnotations.length);
            for (int i = 0; i < paramAnnotations.length; i++) {
                System.out.println("    Parameter " + i + " annotations: " + paramAnnotations[i].length);
                for (Annotation paramAnn : paramAnnotations[i]) {
                    System.out.println("    Found: " + paramAnn.annotationType().getSimpleName());
                    processAnnotation(paramAnn);
                }
            }
        }
    }

    private static void processAnnotation(Annotation annotation) {
        Class<? extends Annotation> annotationType = annotation.annotationType();
        Method[] methods = annotationType.getDeclaredMethods();

        for (Method method : methods) {
            try {
                Object value = method.invoke(annotation);
                System.out.println("    " + method.getName() + " = " + formatValue(value));
            } catch (Exception e) {
                System.out.println("    Error getting " + method.getName() + ": " + e.getMessage());
            }
        }
    }

    private static String formatValue(Object value) {
        if (value == null) {
            return "null";
        } else if (value.getClass().isArray()) {
            Object[] array = (Object[]) value;
            StringBuilder sb = new StringBuilder("[");
            for (int i = 0; i < array.length; i++) {
                sb.append(array[i]);
                if (i < array.length - 1) sb.append(", ");
            }
            sb.append("]");
            return sb.toString();
        } else {
            return value.toString();
        }
    }

    private static void testAnnotationEquality(Class<?> clazz) {
        TestAnnotation ann1 = clazz.getAnnotation(TestAnnotation.class);
        TestAnnotation ann2 = clazz.getAnnotation(TestAnnotation.class);

        System.out.println("Same annotation instances: " + (ann1 == ann2));
        System.out.println("Annotation equals: " + (ann1 != null && ann1.equals(ann2)));
        if (ann1 != null && ann2 != null) {
            System.out.println("Hash codes equal: " + (ann1.hashCode() == ann2.hashCode()));
        }

        // Test annotation type
        if (ann1 != null) {
            Class<? extends Annotation> annotationType = ann1.annotationType();
            System.out.println("Annotation type: " + annotationType.getName());
            System.out.println("Is annotation: " + annotationType.isAnnotation());
        }
    }

    private static void testAnnotationToString(Class<?> clazz) {
        TestAnnotation ann = clazz.getAnnotation(TestAnnotation.class);
        if (ann != null) {
            System.out.println("Annotation toString: " + ann.toString());
        }

        try {
            Method method1 = clazz.getDeclaredMethod("method1", String.class);
            MethodInfo methodInfo = method1.getAnnotation(MethodInfo.class);
            if (methodInfo != null) {
                System.out.println("MethodInfo toString: " + methodInfo.toString());
            }
        } catch (Exception e) {
            System.out.println("Error getting method annotation: " + e.getMessage());
        }
    }
}
