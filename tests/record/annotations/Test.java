import java.lang.annotation.*;
import java.lang.reflect.*;

// Custom annotations for testing
@Retention(RetentionPolicy.RUNTIME)
@Target({ElementType.TYPE, ElementType.RECORD_COMPONENT, ElementType.PARAMETER})
@interface MyAnnotation {
    String value() default "";
    int priority() default 1;
}

@Retention(RetentionPolicy.RUNTIME)
@Target(ElementType.RECORD_COMPONENT)
@interface NotNull {
}

@Retention(RetentionPolicy.RUNTIME)
@Target(ElementType.RECORD_COMPONENT)
@interface Range {
    int min() default 0;
    int max() default 100;
}

public class Test {

    // Annotated record
    @MyAnnotation("PersonRecord")
    record Person(@NotNull String name, @Range(min = 0, max = 150) int age) {}

    // Record with multiple annotations
    record Product(
        @MyAnnotation("ProductName") @NotNull String name,
        @MyAnnotation(value = "ProductPrice", priority = 2) double price,
        @Range(min = 0, max = 1000) int quantity
    ) {}

    // Record with annotation on type
    @MyAnnotation("ConfigRecord")
    record Config(String host, int port, boolean secure) {}

    public static void main(String[] args) {
        System.out.println("=== Annotations Tests ===");

        testRecordAnnotations();
        testComponentAnnotations();
        testAnnotationReflection();
        testMultipleAnnotations();
        testAnnotationValues();
    }

    public static void testRecordAnnotations() {
        System.out.println("--- Test Record Annotations ---");
        Class<Person> personClass = Person.class;

        System.out.println("Person class is annotated: " + personClass.isAnnotationPresent(MyAnnotation.class));

        if (personClass.isAnnotationPresent(MyAnnotation.class)) {
            MyAnnotation annotation = personClass.getAnnotation(MyAnnotation.class);
            System.out.println("Annotation value: " + annotation.value());
            System.out.println("Annotation priority: " + annotation.priority());
        }

        Class<Config> configClass = Config.class;
        System.out.println("Config class is annotated: " + configClass.isAnnotationPresent(MyAnnotation.class));

        if (configClass.isAnnotationPresent(MyAnnotation.class)) {
            MyAnnotation annotation = configClass.getAnnotation(MyAnnotation.class);
            System.out.println("Config annotation value: " + annotation.value());
        }
    }

    public static void testComponentAnnotations() {
        System.out.println("--- Test Component Annotations ---");
        Class<Person> personClass = Person.class;
        RecordComponent[] components = personClass.getRecordComponents();

        for (RecordComponent component : components) {
            System.out.println("Component: " + component.getName());

            if (component.isAnnotationPresent(NotNull.class)) {
                System.out.println("  Has @NotNull annotation");
            }

            if (component.isAnnotationPresent(Range.class)) {
                Range range = component.getAnnotation(Range.class);
                System.out.println("  Has @Range annotation: min=" + range.min() + ", max=" + range.max());
            }

            Annotation[] annotations = component.getAnnotations();
            System.out.println("  Total annotations: " + annotations.length);
        }
    }

    public static void testAnnotationReflection() {
        System.out.println("--- Test Annotation Reflection ---");
        Class<Product> productClass = Product.class;
        RecordComponent[] components = productClass.getRecordComponents();

        for (RecordComponent component : components) {
            System.out.println("Component: " + component.getName());

            Annotation[] annotations = component.getAnnotations();
            for (Annotation annotation : annotations) {
                System.out.println("  Annotation type: " + annotation.annotationType().getSimpleName());

                if (annotation instanceof MyAnnotation) {
                    MyAnnotation myAnnotation = (MyAnnotation) annotation;
                    System.out.println("    Value: " + myAnnotation.value());
                    System.out.println("    Priority: " + myAnnotation.priority());
                }

                if (annotation instanceof Range) {
                    Range range = (Range) annotation;
                    System.out.println("    Min: " + range.min());
                    System.out.println("    Max: " + range.max());
                }
            }
        }
    }

    public static void testMultipleAnnotations() {
        System.out.println("--- Test Multiple Annotations ---");
        Product product = new Product("Laptop", 999.99, 50);
        Class<Product> productClass = Product.class;

        try {
            RecordComponent nameComponent = null;
            RecordComponent priceComponent = null;

            for (RecordComponent component : productClass.getRecordComponents()) {
                if ("name".equals(component.getName())) {
                    nameComponent = component;
                } else if ("price".equals(component.getName())) {
                    priceComponent = component;
                }
            }

            if (nameComponent != null) {
                System.out.println("Name component annotations:");
                System.out.println("  @MyAnnotation present: " + nameComponent.isAnnotationPresent(MyAnnotation.class));
                System.out.println("  @NotNull present: " + nameComponent.isAnnotationPresent(NotNull.class));
            }

            if (priceComponent != null) {
                System.out.println("Price component annotations:");
                System.out.println("  @MyAnnotation present: " + priceComponent.isAnnotationPresent(MyAnnotation.class));
                if (priceComponent.isAnnotationPresent(MyAnnotation.class)) {
                    MyAnnotation annotation = priceComponent.getAnnotation(MyAnnotation.class);
                    System.out.println("    Value: " + annotation.value());
                    System.out.println("    Priority: " + annotation.priority());
                }
            }
        } catch (Exception e) {
            System.out.println("Error accessing annotations: " + e.getMessage());
        }
    }

    public static void testAnnotationValues() {
        System.out.println("--- Test Annotation Values ---");
        Person person = new Person("Alice", 30);

        System.out.println("Created person: " + person);
        System.out.println("Person name: " + person.name());
        System.out.println("Person age: " + person.age());

        // Annotations don't affect runtime behavior directly
        // but can be used for validation, documentation, etc.
        System.out.println("Annotations are metadata only - person created successfully");
    }
}
