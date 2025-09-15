import java.lang.annotation.*;

// Enum for testing
enum Priority {
    LOW, MEDIUM, HIGH
}

// Annotation for testing
@Retention(RetentionPolicy.RUNTIME)
@interface SimpleAnnotation {
    String value() default "simple";
}

// Annotation with all value types
@Retention(RetentionPolicy.RUNTIME)
@interface AllValueTypes {
    // Primitive types
    boolean booleanValue() default true;
    byte byteValue() default 42;
    short shortValue() default 1000;
    int intValue() default 12345;
    long longValue() default 9876543210L;
    float floatValue() default 3.14f;
    double doubleValue() default 2.718281828;
    char charValue() default 'A';

    // String
    String stringValue() default "default";

    // Class
    Class<?> classValue() default Object.class;

    // Enum
    Priority priorityValue() default Priority.MEDIUM;

    // Annotation
    SimpleAnnotation annotationValue() default @SimpleAnnotation("nested");

    // Arrays
    int[] intArray() default {1, 2, 3};
    String[] stringArray() default {"a", "b", "c"};
    Class<?>[] classArray() default {String.class, Integer.class};
    Priority[] priorityArray() default {Priority.LOW, Priority.HIGH};
    SimpleAnnotation[] annotationArray() default {@SimpleAnnotation("first"), @SimpleAnnotation("second")};
}

@AllValueTypes(
    booleanValue = false,
    byteValue = 100,
    shortValue = 2000,
    intValue = 54321,
    longValue = 1234567890L,
    floatValue = 1.41f,
    doubleValue = 1.414213562,
    charValue = 'Z',
    stringValue = "custom",
    classValue = String.class,
    priorityValue = Priority.HIGH,
    annotationValue = @SimpleAnnotation("custom_nested"),
    intArray = {10, 20, 30, 40},
    stringArray = {"x", "y", "z"},
    classArray = {Integer.class, Double.class, Boolean.class},
    priorityArray = {Priority.HIGH, Priority.MEDIUM, Priority.LOW},
    annotationArray = {@SimpleAnnotation("custom1"), @SimpleAnnotation("custom2"), @SimpleAnnotation("custom3")}
)
public class Test {

    public static void main(String[] args) {
        System.out.println("=== Annotation Values Test ===");

        Class<?> clazz = Test.class;
        AllValueTypes annotation = clazz.getAnnotation(AllValueTypes.class);

        if (annotation != null) {
            // Test primitive values
            System.out.println("booleanValue: " + annotation.booleanValue());
            System.out.println("byteValue: " + annotation.byteValue());
            System.out.println("shortValue: " + annotation.shortValue());
            System.out.println("intValue: " + annotation.intValue());
            System.out.println("longValue: " + annotation.longValue());
            System.out.println("floatValue: " + annotation.floatValue());
            System.out.println("doubleValue: " + annotation.doubleValue());
            System.out.println("charValue: " + annotation.charValue());

            // Test String
            System.out.println("stringValue: " + annotation.stringValue());

            // Test Class
            System.out.println("classValue: " + annotation.classValue().getName());

            // Test Enum
            System.out.println("priorityValue: " + annotation.priorityValue());

            // Test nested annotation
            SimpleAnnotation nested = annotation.annotationValue();
            System.out.println("annotationValue: " + nested.value());

            // Test arrays
            int[] intArray = annotation.intArray();
            System.out.print("intArray: [");
            for (int i = 0; i < intArray.length; i++) {
                System.out.print(intArray[i]);
                if (i < intArray.length - 1) System.out.print(", ");
            }
            System.out.println("]");

            String[] stringArray = annotation.stringArray();
            System.out.print("stringArray: [");
            for (int i = 0; i < stringArray.length; i++) {
                System.out.print("\"" + stringArray[i] + "\"");
                if (i < stringArray.length - 1) System.out.print(", ");
            }
            System.out.println("]");

            Class<?>[] classArray = annotation.classArray();
            System.out.print("classArray: [");
            for (int i = 0; i < classArray.length; i++) {
                System.out.print(classArray[i].getSimpleName());
                if (i < classArray.length - 1) System.out.print(", ");
            }
            System.out.println("]");

            Priority[] priorityArray = annotation.priorityArray();
            System.out.print("priorityArray: [");
            for (int i = 0; i < priorityArray.length; i++) {
                System.out.print(priorityArray[i]);
                if (i < priorityArray.length - 1) System.out.print(", ");
            }
            System.out.println("]");

            SimpleAnnotation[] annotationArray = annotation.annotationArray();
            System.out.print("annotationArray: [");
            for (int i = 0; i < annotationArray.length; i++) {
                System.out.print("@SimpleAnnotation(\"" + annotationArray[i].value() + "\")");
                if (i < annotationArray.length - 1) System.out.print(", ");
            }
            System.out.println("]");
        } else {
            System.out.println("AllValueTypes annotation not found");
        }

        // Test default values
        System.out.println("\n=== Testing Default Values ===");
        try {
            // Create a test class with annotation using defaults
            @AllValueTypes
            class DefaultTest {}

            AllValueTypes defaultAnnotation = DefaultTest.class.getAnnotation(AllValueTypes.class);
            if (defaultAnnotation != null) {
                System.out.println("Default booleanValue: " + defaultAnnotation.booleanValue());
                System.out.println("Default intValue: " + defaultAnnotation.intValue());
                System.out.println("Default stringValue: " + defaultAnnotation.stringValue());
                System.out.println("Default classValue: " + defaultAnnotation.classValue().getSimpleName());
                System.out.println("Default priorityValue: " + defaultAnnotation.priorityValue());
                System.out.println("Default annotationValue: " + defaultAnnotation.annotationValue().value());

                int[] defaultIntArray = defaultAnnotation.intArray();
                System.out.print("Default intArray: [");
                for (int i = 0; i < defaultIntArray.length; i++) {
                    System.out.print(defaultIntArray[i]);
                    if (i < defaultIntArray.length - 1) System.out.print(", ");
                }
                System.out.println("]");
            }
        } catch (Exception e) {
            System.out.println("Error testing default values: " + e.getMessage());
        }
    }
}
