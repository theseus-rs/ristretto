/** Test instanceof behavior with edge cases and special scenarios */
public class Test {
    public static void main(String[] args) {
        // Test with wrapper classes and autoboxing scenarios
        Integer intObj = 42;
        System.out.println("Integer instanceof Integer: " + (intObj instanceof Integer));
        System.out.println("Integer instanceof Number: " + (intObj instanceof Number));
        System.out.println("Integer instanceof Object: " + (intObj instanceof Object));
        System.out.println("Integer instanceof Comparable: " + (intObj instanceof Comparable));
        System.out.println("Integer instanceof java.io.Serializable: " + (intObj instanceof java.io.Serializable));

        // Test with different Number subclasses
        Number numAsInt = 42;
        Number numAsDouble = 3.14;
        System.out.println("Number (Integer) instanceof Number: " + (numAsInt instanceof Number));
        System.out.println("Number (Integer) instanceof Integer: " + (numAsInt instanceof Integer));
        System.out.println("Number (Integer) instanceof Double: " + (numAsInt instanceof Double));
        System.out.println("Number (Double) instanceof Number: " + (numAsDouble instanceof Number));
        System.out.println("Number (Double) instanceof Double: " + (numAsDouble instanceof Double));
        System.out.println("Number (Double) instanceof Integer: " + (numAsDouble instanceof Integer));

        // Test with Boolean
        Boolean boolObj = true;
        System.out.println("Boolean instanceof Boolean: " + (boolObj instanceof Boolean));
        System.out.println("Boolean instanceof Object: " + (boolObj instanceof Object));
        System.out.println("Boolean instanceof Comparable: " + (boolObj instanceof Comparable));
        System.out.println("Boolean instanceof java.io.Serializable: " + (boolObj instanceof java.io.Serializable));

        // Test with Character
        Character charObj = 'A';
        System.out.println("Character instanceof Character: " + (charObj instanceof Character));
        System.out.println("Character instanceof Object: " + (charObj instanceof Object));
        System.out.println("Character instanceof Comparable: " + (charObj instanceof Comparable));
        System.out.println("Character instanceof java.io.Serializable: " + (charObj instanceof java.io.Serializable));

        // Test with arrays of wrapper types
        Integer[] intArray = new Integer[5];
        System.out.println("Integer[] instanceof Object: " + (intArray instanceof Object));
        System.out.println("Integer[] instanceof Integer[]: " + (intArray instanceof Integer[]));
        System.out.println("Integer[] instanceof Number[]: " + (intArray instanceof Number[]));
        System.out.println("Integer[] instanceof Object[]: " + (intArray instanceof Object[]));
        System.out.println("Integer[] instanceof Comparable[]: " + (intArray instanceof Comparable[]));

        // Test multidimensional arrays with wrapper types
        Integer[][] intArray2D = new Integer[3][3];
        System.out.println("Integer[][] instanceof Object: " + (intArray2D instanceof Object));
        System.out.println("Integer[][] instanceof Integer[][]: " + (intArray2D instanceof Integer[][]));
        System.out.println("Integer[][] instanceof Number[][]: " + (intArray2D instanceof Number[][]));
        System.out.println("Integer[][] instanceof Object[][]: " + (intArray2D instanceof Object[][]));
        System.out.println("Integer[][] instanceof Object[]: " + (intArray2D instanceof Object[]));

        // Test with empty arrays
        String[] emptyStringArray = new String[0];
        System.out.println("String[0] instanceof String[]: " + (emptyStringArray instanceof String[]));
        System.out.println("String[0] instanceof Object[]: " + (emptyStringArray instanceof Object[]));
        System.out.println("String[0] instanceof Object: " + (emptyStringArray instanceof Object));

        // Test with Object arrays containing different types
        Object[] mixedArray = new Object[3];
        mixedArray[0] = "String";
        mixedArray[1] = 42;
        mixedArray[2] = true;
        System.out.println("Object[] instanceof Object[]: " + (mixedArray instanceof Object[]));
        System.out.println("Object[] instanceof String[]: " + (mixedArray instanceof String[]));
        System.out.println("Object[] instanceof Integer[]: " + (mixedArray instanceof Integer[]));
        System.out.println("Object[] instanceof Object: " + (mixedArray instanceof Object));

        // Test with specific array instances
        Object stringArrayAsObject = new String[5];
        System.out.println("String[] as Object instanceof Object: " + (stringArrayAsObject instanceof Object));
        System.out.println("String[] as Object instanceof String[]: " + (stringArrayAsObject instanceof String[]));
        System.out.println("String[] as Object instanceof Object[]: " + (stringArrayAsObject instanceof Object[]));
        System.out.println("String[] as Object instanceof int[]: " + (stringArrayAsObject instanceof int[]));

        Object intArrayAsObject = new int[5];
        System.out.println("int[] as Object instanceof Object: " + (intArrayAsObject instanceof Object));
        System.out.println("int[] as Object instanceof int[]: " + (intArrayAsObject instanceof int[]));
        System.out.println("int[] as Object instanceof Object[]: " + (intArrayAsObject instanceof Object[]));
        System.out.println("int[] as Object instanceof Integer[]: " + (intArrayAsObject instanceof Integer[]));
    }
}
