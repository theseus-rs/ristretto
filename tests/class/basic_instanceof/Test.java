/** Test basic instanceof behavior with classes and null values */
public class Test {
    public static void main(String[] args) {
        // Test null instanceof - should always be false
        Object nullRef = null;
        System.out.println("null instanceof Object: " + (nullRef instanceof Object));
        System.out.println("null instanceof String: " + (nullRef instanceof String));
        System.out.println("null instanceof Test: " + (nullRef instanceof Test));

        // Test same class instanceof
        Test testObj = new Test();
        System.out.println("Test instanceof Test: " + (testObj instanceof Test));
        System.out.println("Test instanceof Object: " + (testObj instanceof Object));

        // Test String instanceof
        String str = "Hello";
        System.out.println("String instanceof String: " + (str instanceof String));
        System.out.println("String instanceof Object: " + (str instanceof Object));
        System.out.println("String instanceof CharSequence: " + (str instanceof CharSequence));
        System.out.println("String instanceof Serializable: " + (str instanceof java.io.Serializable));
        System.out.println("String instanceof Comparable: " + (str instanceof Comparable));

        // Test Object instanceof
        Object obj = new Object();
        System.out.println("Object instanceof Object: " + (obj instanceof Object));
        System.out.println("Object instanceof String: " + (obj instanceof String));
    }
}

