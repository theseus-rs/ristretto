import java.lang.annotation.*;

// Custom meta-annotation
@Retention(RetentionPolicy.RUNTIME)
@Target(ElementType.ANNOTATION_TYPE)
@interface MetaInfo {
    String author() default "unknown";
    String version() default "1.0";
    String description() default "";
}

// Annotation with meta-annotations
@Retention(RetentionPolicy.RUNTIME)
@Target({ElementType.TYPE, ElementType.METHOD})
@Documented
@MetaInfo(author = "TestAuthor", version = "2.0", description = "Test annotation with meta-info")
@interface CustomAnnotation {
    String value() default "custom";
    int priority() default 1;
}

// Another annotation to test multiple meta-annotations
@Retention(RetentionPolicy.RUNTIME)
@Target({ElementType.TYPE})
@Inherited
@Documented
@MetaInfo(author = "AnotherAuthor", version = "3.0")
@interface InheritableAnnotation {
    String name() default "inheritable";
}

// Base class with inheritable annotation
@InheritableAnnotation(name = "base")
class BaseClass {
    @CustomAnnotation("baseMethod")
    public void baseMethod() {
    }
}

@CustomAnnotation("TestClass")
public class Test extends BaseClass {

    @CustomAnnotation(value = "testMethod", priority = 5)
    public void testMethod() {
    }

    @Override
    @CustomAnnotation("overriddenMethod")
    public void baseMethod() {
        super.baseMethod();
    }

    public static void main(String[] args) {
        System.out.println("=== Meta-Annotations Test ===");

        // Test meta-annotations on CustomAnnotation
        Class<CustomAnnotation> customAnnClass = CustomAnnotation.class;
        System.out.println("CustomAnnotation annotations:");

        Annotation[] metaAnnotations = customAnnClass.getAnnotations();
        for (Annotation ann : metaAnnotations) {
            System.out.println("  " + ann.annotationType().getSimpleName());

            if (ann instanceof MetaInfo) {
                MetaInfo metaInfo = (MetaInfo) ann;
                System.out.println("    author: " + metaInfo.author());
                System.out.println("    version: " + metaInfo.version());
                System.out.println("    description: " + metaInfo.description());
            }
        }

        // Test Retention meta-annotation
        Retention retention = customAnnClass.getAnnotation(Retention.class);
        if (retention != null) {
            System.out.println("CustomAnnotation retention: " + retention.value());
        }

        // Test Target meta-annotation
        Target target = customAnnClass.getAnnotation(Target.class);
        if (target != null) {
            System.out.print("CustomAnnotation targets: [");
            ElementType[] targets = target.value();
            for (int i = 0; i < targets.length; i++) {
                System.out.print(targets[i]);
                if (i < targets.length - 1) System.out.print(", ");
            }
            System.out.println("]");
        }

        // Test Documented meta-annotation
        System.out.println("CustomAnnotation is documented: " + customAnnClass.isAnnotationPresent(Documented.class));

        // Test meta-annotations on InheritableAnnotation
        System.out.println("\nInheritableAnnotation annotations:");
        Class<InheritableAnnotation> inheritableAnnClass = InheritableAnnotation.class;
        Annotation[] inheritableMetaAnnotations = inheritableAnnClass.getAnnotations();
        for (Annotation ann : inheritableMetaAnnotations) {
            System.out.println("  " + ann.annotationType().getSimpleName());
        }

        // Test inherited annotation behavior
        System.out.println("\n=== Inheritance Test ===");
        Class<?> testClass = Test.class;
        Class<?> baseClass = BaseClass.class;

        System.out.println("BaseClass has InheritableAnnotation: " + baseClass.isAnnotationPresent(InheritableAnnotation.class));
        System.out.println("Test has InheritableAnnotation: " + testClass.isAnnotationPresent(InheritableAnnotation.class));

        // Note: @Inherited only works for class-level annotations, not method-level
        InheritableAnnotation inherited = testClass.getAnnotation(InheritableAnnotation.class);
        if (inherited != null) {
            System.out.println("Inherited annotation name: " + inherited.name());
        }

        // Test annotation on methods
        try {
            java.lang.reflect.Method testMethod = testClass.getDeclaredMethod("testMethod");
            CustomAnnotation methodAnn = testMethod.getAnnotation(CustomAnnotation.class);
            if (methodAnn != null) {
                System.out.println("testMethod annotation value: " + methodAnn.value());
                System.out.println("testMethod annotation priority: " + methodAnn.priority());
            }
        } catch (Exception e) {
            System.out.println("Error accessing testMethod: " + e.getMessage());
        }

        // Test getDeclaredAnnotations vs getAnnotations
        System.out.println("\n=== Declared vs All Annotations ===");
        Annotation[] declaredAnnotations = testClass.getDeclaredAnnotations();
        Annotation[] allAnnotations = testClass.getAnnotations();

        System.out.println("Declared annotations count: " + declaredAnnotations.length);
        System.out.println("All annotations count: " + allAnnotations.length);

        System.out.println("Declared annotations:");
        for (Annotation ann : declaredAnnotations) {
            System.out.println("  " + ann.annotationType().getSimpleName());
        }

        System.out.println("All annotations:");
        for (Annotation ann : allAnnotations) {
            System.out.println("  " + ann.annotationType().getSimpleName());
        }
    }
}
