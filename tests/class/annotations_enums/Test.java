/** Test instanceof behavior with annotations and enums */
enum Color {
    RED, GREEN, BLUE
}

enum Size {
    SMALL, MEDIUM, LARGE;

    public void display() {
        System.out.println("Size: " + this);
    }
}

@interface TestAnnotation {
    String value() default "test";
}

@TestAnnotation("example")
class AnnotatedClass {
    public void method() {}
}

public class Test {
    public static void main(String[] args) {
        // Test enum instanceof scenarios
        Color red = Color.RED;
        System.out.println("Color.RED instanceof Color: " + (red instanceof Color));
        System.out.println("Color.RED instanceof Enum: " + (red instanceof Enum));
        System.out.println("Color.RED instanceof Object: " + (red instanceof Object));
        System.out.println("Color.RED instanceof Comparable: " + (red instanceof Comparable));
        System.out.println("Color.RED instanceof java.io.Serializable: " + (red instanceof java.io.Serializable));

        // Test different enum values
        Color green = Color.GREEN;
        Color blue = Color.BLUE;
        System.out.println("Color.GREEN instanceof Color: " + (green instanceof Color));
        System.out.println("Color.BLUE instanceof Color: " + (blue instanceof Color));
        System.out.println("Color.GREEN instanceof Enum: " + (green instanceof Enum));
        System.out.println("Color.BLUE instanceof Enum: " + (blue instanceof Enum));

        // Test Size enum with methods
        Size medium = Size.MEDIUM;
        System.out.println("Size.MEDIUM instanceof Size: " + (medium instanceof Size));
        System.out.println("Size.MEDIUM instanceof Enum: " + (medium instanceof Enum));
        System.out.println("Size.MEDIUM instanceof Object: " + (medium instanceof Object));
        System.out.println("Size.MEDIUM instanceof Comparable: " + (medium instanceof Comparable));

        // Test enum through Enum reference
        Enum<?> enumRef = Color.RED;
        System.out.println("Enum ref (Color.RED) instanceof Enum: " + (enumRef instanceof Enum));
        System.out.println("Enum ref (Color.RED) instanceof Color: " + (enumRef instanceof Color));
        System.out.println("Enum ref (Color.RED) instanceof Size: " + (enumRef instanceof Size));
        System.out.println("Enum ref (Color.RED) instanceof Object: " + (enumRef instanceof Object));
        System.out.println("Enum ref (Color.RED) instanceof Comparable: " + (enumRef instanceof Comparable));

        // Test enum arrays
        Color[] colorArray = {Color.RED, Color.GREEN, Color.BLUE};
        System.out.println("Color[] instanceof Object: " + (colorArray instanceof Object));
        System.out.println("Color[] instanceof Color[]: " + (colorArray instanceof Color[]));
        System.out.println("Color[] instanceof Enum[]: " + (colorArray instanceof Enum[]));
        System.out.println("Color[] instanceof Object[]: " + (colorArray instanceof Object[]));
        System.out.println("Color[] instanceof Comparable[]: " + (colorArray instanceof Comparable[]));

        // Test annotated class
        AnnotatedClass annotated = new AnnotatedClass();
        System.out.println("AnnotatedClass instanceof AnnotatedClass: " + (annotated instanceof AnnotatedClass));
        System.out.println("AnnotatedClass instanceof Object: " + (annotated instanceof Object));

        // Test Class objects of enums
        Class<?> colorClass = Color.class;
        System.out.println("Color.class instanceof Class: " + (colorClass instanceof Class));
        System.out.println("Color.class instanceof Object: " + (colorClass instanceof Object));

        // Test with null enum references
        Color nullColor = null;
        System.out.println("null Color instanceof Color: " + (nullColor instanceof Color));
        System.out.println("null Color instanceof Enum: " + (nullColor instanceof Enum));
        System.out.println("null Color instanceof Object: " + (nullColor instanceof Object));

        // Test polymorphic enum references
        Object colorAsObject = Color.BLUE;
        System.out.println("Color as Object instanceof Object: " + (colorAsObject instanceof Object));
        System.out.println("Color as Object instanceof Color: " + (colorAsObject instanceof Color));
        System.out.println("Color as Object instanceof Enum: " + (colorAsObject instanceof Enum));
        System.out.println("Color as Object instanceof Size: " + (colorAsObject instanceof Size));
        System.out.println("Color as Object instanceof String: " + (colorAsObject instanceof String));

        // Test Comparable reference with enum
        Comparable<?> comparableColor = Color.RED;
        System.out.println("Color as Comparable instanceof Comparable: " + (comparableColor instanceof Comparable));
        System.out.println("Color as Comparable instanceof Color: " + (comparableColor instanceof Color));
        System.out.println("Color as Comparable instanceof Enum: " + (comparableColor instanceof Enum));
        System.out.println("Color as Comparable instanceof Object: " + (comparableColor instanceof Object));

        // Test multidimensional enum arrays
        Color[][] colorArray2D = new Color[2][3];
        System.out.println("Color[][] instanceof Object: " + (colorArray2D instanceof Object));
        System.out.println("Color[][] instanceof Color[][]: " + (colorArray2D instanceof Color[][]));
        System.out.println("Color[][] instanceof Enum[][]: " + (colorArray2D instanceof Enum[][]));
        System.out.println("Color[][] instanceof Object[][]: " + (colorArray2D instanceof Object[][]));
        System.out.println("Color[][] instanceof Object[]: " + (colorArray2D instanceof Object[]));
    }
}
