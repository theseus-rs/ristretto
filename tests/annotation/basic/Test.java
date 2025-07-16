import java.lang.annotation.*;

// Basic annotation without elements
@interface SimpleMarker {
}

// Annotation with single element
@interface SingleElement {
    String value();
}

// Annotation with multiple elements
@interface MultipleElements {
    String name();
    int version();
    boolean enabled() default true;
}

// Test class using annotations
@SimpleMarker
@SingleElement("test")
@MultipleElements(name = "BasicTest", version = 1)
public class Test {

    @SimpleMarker
    private String field;

    @SingleElement("method")
    @MultipleElements(name = "testMethod", version = 2, enabled = false)
    public void annotatedMethod() {
    }

    public static void main(String[] args) {
        System.out.println("=== Basic Annotations Test ===");

        // Test class annotations
        Class<?> clazz = Test.class;
        System.out.println("Class has SimpleMarker: " + clazz.isAnnotationPresent(SimpleMarker.class));
        System.out.println("Class has SingleElement: " + clazz.isAnnotationPresent(SingleElement.class));
        System.out.println("Class has MultipleElements: " + clazz.isAnnotationPresent(MultipleElements.class));

        // Test SingleElement annotation
        SingleElement singleElement = clazz.getAnnotation(SingleElement.class);
        if (singleElement != null) {
            System.out.println("SingleElement value: " + singleElement.value());
        }

        // Test MultipleElements annotation
        MultipleElements multipleElements = clazz.getAnnotation(MultipleElements.class);
        if (multipleElements != null) {
            System.out.println("MultipleElements name: " + multipleElements.name());
            System.out.println("MultipleElements version: " + multipleElements.version());
            System.out.println("MultipleElements enabled: " + multipleElements.enabled());
        }

        // Test method annotations
        try {
            java.lang.reflect.Method method = clazz.getDeclaredMethod("annotatedMethod");
            System.out.println("Method has SimpleMarker: " + method.isAnnotationPresent(SimpleMarker.class));
            System.out.println("Method has SingleElement: " + method.isAnnotationPresent(SingleElement.class));
            System.out.println("Method has MultipleElements: " + method.isAnnotationPresent(MultipleElements.class));

            SingleElement methodSingle = method.getAnnotation(SingleElement.class);
            if (methodSingle != null) {
                System.out.println("Method SingleElement value: " + methodSingle.value());
            }

            MultipleElements methodMultiple = method.getAnnotation(MultipleElements.class);
            if (methodMultiple != null) {
                System.out.println("Method MultipleElements name: " + methodMultiple.name());
                System.out.println("Method MultipleElements version: " + methodMultiple.version());
                System.out.println("Method MultipleElements enabled: " + methodMultiple.enabled());
            }
        } catch (Exception e) {
            System.out.println("Error accessing method: " + e.getMessage());
        }

        // Test field annotations
        try {
            java.lang.reflect.Field field = clazz.getDeclaredField("field");
            System.out.println("Field has SimpleMarker: " + field.isAnnotationPresent(SimpleMarker.class));
        } catch (Exception e) {
            System.out.println("Error accessing field: " + e.getMessage());
        }
    }
}
