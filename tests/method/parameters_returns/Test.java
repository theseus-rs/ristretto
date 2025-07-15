/** Test method parameters and return types including varargs and generics. */
public class Test {
    // Method with various parameter types
    public static int primitiveParameters(int a, double b, boolean c, char d) {
        System.out.println("int: " + a + ", double: " + b + ", boolean: " + c + ", char: " + d);
        return a + (int)b + (c ? 1 : 0) + (int)d;
    }

    // Method with object parameters
    public static String objectParameters(String str, Object obj, Integer wrapper) {
        System.out.println("String: " + str + ", Object: " + obj + ", Integer: " + wrapper);
        return str + obj + wrapper;
    }

    // Method with array parameters
    public static void arrayParameters(int[] intArray, String[] stringArray) {
        System.out.print("int[]: ");
        for (int i : intArray) System.out.print(i + " ");
        System.out.print(", String[]: ");
        for (String s : stringArray) System.out.print(s + " ");
        System.out.println();
    }

    // Method with varargs
    public static void varargsMethod(String prefix, int... numbers) {
        System.out.print(prefix + ": ");
        for (int num : numbers) System.out.print(num + " ");
        System.out.println();
    }

    // Generic method
    public static <T> T genericMethod(T input, Class<T> type) {
        System.out.println("Generic method with type: " + type.getSimpleName() + ", value: " + input);
        return input;
    }

    // Method with generic bounds
    public static <T extends Number> double boundedGeneric(T number) {
        System.out.println("Bounded generic with Number: " + number);
        return number.doubleValue();
    }

    // Method returning arrays
    public static int[] returnArray(int size) {
        int[] array = new int[size];
        for (int i = 0; i < size; i++) array[i] = i * i;
        return array;
    }

    public static void main(String[] args) {
        // Test primitive parameters
        int result = primitiveParameters(42, 3.14, true, 'A');
        System.out.println("Result: " + result);

        // Test object parameters
        String objResult = objectParameters("Hello", new Integer(100), 200);
        System.out.println("Object result: " + objResult);

        // Test array parameters
        arrayParameters(new int[]{1, 2, 3}, new String[]{"a", "b", "c"});

        // Test varargs
        varargsMethod("Numbers", 1, 2, 3, 4, 5);
        varargsMethod("Empty");
        varargsMethod("Single", 42);

        // Test generics
        String genericStr = genericMethod("Generic String", String.class);
        Integer genericInt = genericMethod(123, Integer.class);

        // Test bounded generics
        double doubleResult = boundedGeneric(42);
        double floatResult = boundedGeneric(3.14f);

        // Test array return
        int[] squares = returnArray(5);
        System.out.print("Squares: ");
        for (int sq : squares) System.out.print(sq + " ");
        System.out.println();
    }
}

