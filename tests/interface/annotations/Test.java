/** Test interface annotations and metadata */
import java.lang.annotation.*;

// Define test annotations
@Retention(RetentionPolicy.RUNTIME)
@Target({ElementType.TYPE, ElementType.METHOD, ElementType.FIELD})
@interface TestAnnotation {
    String value() default "test";
    int priority() default 1;
}

@Retention(RetentionPolicy.RUNTIME)
@Target(ElementType.TYPE)
@interface InterfaceMarker {
    String category();
}

@Retention(RetentionPolicy.RUNTIME)
@Target(ElementType.METHOD)
@interface MethodInfo {
    String description();
    boolean deprecated() default false;
}

// Annotated interfaces
@TestAnnotation("AnnotatedInterface")
@InterfaceMarker(category = "test")
interface AnnotatedInterface {
    @TestAnnotation("constantField")
    String ANNOTATED_CONSTANT = "annotated";

    @MethodInfo(description = "Basic annotated method")
    void annotatedMethod();

    @MethodInfo(description = "Deprecated method", deprecated = true)
    @Deprecated
    void deprecatedMethod();

    @TestAnnotation(value = "defaultMethod", priority = 2)
    @MethodInfo(description = "Default method with annotations")
    default void annotatedDefault() {
        System.out.println("AnnotatedInterface.annotatedDefault");
    }

    @TestAnnotation("staticMethod")
    @MethodInfo(description = "Static method with annotations")
    static void annotatedStatic() {
        System.out.println("AnnotatedInterface.annotatedStatic");
    }
}

@TestAnnotation("ChildInterface")
interface ChildAnnotatedInterface extends AnnotatedInterface {
    @Override
    @MethodInfo(description = "Overridden annotated method")
    default void annotatedDefault() {
        System.out.println("ChildAnnotatedInterface.annotatedDefault - overridden");
        AnnotatedInterface.super.annotatedDefault();
    }

    @TestAnnotation("childMethod")
    @MethodInfo(description = "Child-specific method")
    void childMethod();
}

// Implementation with annotations
@TestAnnotation("Implementation")
class AnnotatedImplementation implements ChildAnnotatedInterface {
    @Override
    @MethodInfo(description = "Implemented annotated method")
    public void annotatedMethod() {
        System.out.println("AnnotatedImplementation.annotatedMethod");
    }

    @Override
    public void deprecatedMethod() {
        System.out.println("AnnotatedImplementation.deprecatedMethod");
    }

    @Override
    @TestAnnotation("overriddenChild")
    public void childMethod() {
        System.out.println("AnnotatedImplementation.childMethod");
    }
}

// Functional interface with annotations
@FunctionalInterface
@TestAnnotation("FunctionalAnnotated")
@InterfaceMarker(category = "functional")
interface AnnotatedFunctional {
    @MethodInfo(description = "Functional method")
    void execute(String input);

    @TestAnnotation("functionalDefault")
    @MethodInfo(description = "Default in functional interface")
    default void defaultInFunctional() {
        System.out.println("AnnotatedFunctional.defaultInFunctional");
    }
}

public class Test {
    public static void main(String[] args) {
        System.out.println("=== Interface Annotations Test ===");

        // Test class annotations
        AnnotatedImplementation impl = new AnnotatedImplementation();
        Class<?> implClass = impl.getClass();

        System.out.println("Implementation class annotations:");
        Annotation[] classAnnotations = implClass.getAnnotations();
        for (Annotation annotation : classAnnotations) {
            System.out.println("  " + annotation);
        }

        // Test interface annotations
        Class<?> interfaceClass = AnnotatedInterface.class;
        System.out.println("\nAnnotatedInterface annotations:");
        Annotation[] interfaceAnnotations = interfaceClass.getAnnotations();
        for (Annotation annotation : interfaceAnnotations) {
            System.out.println("  " + annotation);
            if (annotation instanceof TestAnnotation) {
                TestAnnotation testAnn = (TestAnnotation) annotation;
                System.out.println("    value: " + testAnn.value() + ", priority: " + testAnn.priority());
            }
            if (annotation instanceof InterfaceMarker) {
                InterfaceMarker marker = (InterfaceMarker) annotation;
                System.out.println("    category: " + marker.category());
            }
        }

        // Test method annotations
        System.out.println("\nMethod annotations:");
        try {
            java.lang.reflect.Method annotatedMethod = interfaceClass.getMethod("annotatedMethod");
            System.out.println("annotatedMethod annotations:");
            for (Annotation annotation : annotatedMethod.getAnnotations()) {
                System.out.println("  " + annotation);
            }

            java.lang.reflect.Method defaultMethod = interfaceClass.getMethod("annotatedDefault");
            System.out.println("annotatedDefault annotations:");
            for (Annotation annotation : defaultMethod.getAnnotations()) {
                System.out.println("  " + annotation);
            }

            java.lang.reflect.Method staticMethod = interfaceClass.getMethod("annotatedStatic");
            System.out.println("annotatedStatic annotations:");
            for (Annotation annotation : staticMethod.getAnnotations()) {
                System.out.println("  " + annotation);
            }

        } catch (NoSuchMethodException e) {
            System.out.println("Method not found: " + e.getMessage());
        }

        // Test field annotations
        System.out.println("\nField annotations:");
        try {
            java.lang.reflect.Field constantField = interfaceClass.getField("ANNOTATED_CONSTANT");
            System.out.println("ANNOTATED_CONSTANT annotations:");
            for (Annotation annotation : constantField.getAnnotations()) {
                System.out.println("  " + annotation);
            }
        } catch (NoSuchFieldException e) {
            System.out.println("Field not found: " + e.getMessage());
        }

        // Test inherited annotations
        Class<?> childInterface = ChildAnnotatedInterface.class;
        System.out.println("\nChildAnnotatedInterface annotations:");
        for (Annotation annotation : childInterface.getAnnotations()) {
            System.out.println("  " + annotation);
        }

        // Test annotation inheritance behavior
        System.out.println("\nAnnotation inheritance tests:");
        System.out.println("AnnotatedInterface.isAnnotationPresent(TestAnnotation.class): " +
                          interfaceClass.isAnnotationPresent(TestAnnotation.class));
        System.out.println("ChildAnnotatedInterface.isAnnotationPresent(TestAnnotation.class): " +
                          childInterface.isAnnotationPresent(TestAnnotation.class));
        System.out.println("Implementation.isAnnotationPresent(TestAnnotation.class): " +
                          implClass.isAnnotationPresent(TestAnnotation.class));

        // Test functional interface annotations
        Class<?> functionalClass = AnnotatedFunctional.class;
        System.out.println("\nFunctional interface annotations:");
        for (Annotation annotation : functionalClass.getAnnotations()) {
            System.out.println("  " + annotation);
        }

        // Test lambda with annotated functional interface
        AnnotatedFunctional lambda = input -> System.out.println("Lambda processing: " + input);
        lambda.execute("test input");
        lambda.defaultInFunctional();

        System.out.println("lambda instanceof AnnotatedFunctional: " + (lambda instanceof AnnotatedFunctional));

        // Test annotation on lambda class (generated)
        Class<?> lambdaClass = lambda.getClass();
        System.out.println("Lambda class: " + lambdaClass.getName());
        System.out.println("Lambda class annotations: " + lambdaClass.getAnnotations().length);

        // Test method invocation and annotation checking
        impl.annotatedMethod();
        impl.deprecatedMethod();
        impl.childMethod();
        impl.annotatedDefault();

        // Test static method with annotations
        AnnotatedInterface.annotatedStatic();

        // Test annotation values
        TestAnnotation implAnnotation = implClass.getAnnotation(TestAnnotation.class);
        if (implAnnotation != null) {
            System.out.println("\nImplementation TestAnnotation values:");
            System.out.println("  value: " + implAnnotation.value());
            System.out.println("  priority: " + implAnnotation.priority());
        }

        System.out.println("Interface annotations tests completed");
    }
}
