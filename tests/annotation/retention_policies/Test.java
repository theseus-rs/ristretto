import java.lang.annotation.*;

// SOURCE retention - not available at runtime
@Retention(RetentionPolicy.SOURCE)
@interface SourceRetention {
    String value() default "source";
}

// CLASS retention - available in class file but not at runtime
@Retention(RetentionPolicy.CLASS)
@interface ClassRetention {
    String value() default "class";
}

// RUNTIME retention - available at runtime via reflection
@Retention(RetentionPolicy.RUNTIME)
@interface RuntimeRetention {
    String value() default "runtime";
}

@SourceRetention("test")
@ClassRetention("test")
@RuntimeRetention("test")
public class Test {

    @SourceRetention
    @ClassRetention
    @RuntimeRetention
    private String field;

    @SourceRetention("method")
    @ClassRetention("method")
    @RuntimeRetention("method")
    public void annotatedMethod() {
    }

    public static void main(String[] args) {
        System.out.println("=== Retention Policy Test ===");

        Class<?> clazz = Test.class;

        // Test class annotations - only RUNTIME should be visible
        System.out.println("Class has SourceRetention: " + clazz.isAnnotationPresent(SourceRetention.class));
        System.out.println("Class has ClassRetention: " + clazz.isAnnotationPresent(ClassRetention.class));
        System.out.println("Class has RuntimeRetention: " + clazz.isAnnotationPresent(RuntimeRetention.class));

        RuntimeRetention runtime = clazz.getAnnotation(RuntimeRetention.class);
        if (runtime != null) {
            System.out.println("RuntimeRetention value: " + runtime.value());
        }

        // Test method annotations
        try {
            java.lang.reflect.Method method = clazz.getDeclaredMethod("annotatedMethod");
            System.out.println("Method has SourceRetention: " + method.isAnnotationPresent(SourceRetention.class));
            System.out.println("Method has ClassRetention: " + method.isAnnotationPresent(ClassRetention.class));
            System.out.println("Method has RuntimeRetention: " + method.isAnnotationPresent(RuntimeRetention.class));

            RuntimeRetention methodRuntime = method.getAnnotation(RuntimeRetention.class);
            if (methodRuntime != null) {
                System.out.println("Method RuntimeRetention value: " + methodRuntime.value());
            }
        } catch (Exception e) {
            System.out.println("Error accessing method: " + e.getMessage());
        }

        // Test field annotations
        try {
            java.lang.reflect.Field field = clazz.getDeclaredField("field");
            System.out.println("Field has SourceRetention: " + field.isAnnotationPresent(SourceRetention.class));
            System.out.println("Field has ClassRetention: " + field.isAnnotationPresent(ClassRetention.class));
            System.out.println("Field has RuntimeRetention: " + field.isAnnotationPresent(RuntimeRetention.class));
        } catch (Exception e) {
            System.out.println("Error accessing field: " + e.getMessage());
        }

        // Test getAllAnnotations
        Annotation[] classAnnotations = clazz.getAnnotations();
        System.out.println("Total class annotations at runtime: " + classAnnotations.length);
        for (Annotation ann : classAnnotations) {
            System.out.println("Found annotation: " + ann.annotationType().getSimpleName());
        }
    }
}
