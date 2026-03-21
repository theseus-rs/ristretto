/** Test java.lang.reflect.Array.getDouble for double arrays. */
public class Test {
    public static void main(String[] args) throws Exception {
        testBasic();
        testWidening();
        testNullArray();
        testIndexOutOfBounds();
        testTypeMismatch();
    }

    static void testBasic() {
        double[] array = {3.14159, -2.71828, Double.MAX_VALUE};
        System.out.println("double[0]: " + java.lang.reflect.Array.getDouble(array, 0));
        System.out.println("double[1]: " + java.lang.reflect.Array.getDouble(array, 1));
        System.out.println("double[2]: " + java.lang.reflect.Array.getDouble(array, 2));
    }

    static void testWidening() {
        int[] intArray = {42};
        System.out.println("int as double: " + java.lang.reflect.Array.getDouble(intArray, 0));
        float[] floatArray = {3.14f};
        System.out.println("float as double: " + java.lang.reflect.Array.getDouble(floatArray, 0));
    }

    static void testNullArray() {
        try {
            java.lang.reflect.Array.getDouble(null, 0);
        } catch (NullPointerException e) {
            System.out.println("NullPointerException caught");
        }
    }

    static void testIndexOutOfBounds() {
        double[] array = {1.0};
        try {
            java.lang.reflect.Array.getDouble(array, -1);
        } catch (ArrayIndexOutOfBoundsException e) {
            System.out.println("Negative index: ArrayIndexOutOfBoundsException caught");
        }
        try {
            java.lang.reflect.Array.getDouble(array, 1);
        } catch (ArrayIndexOutOfBoundsException e) {
            System.out.println("Over index: ArrayIndexOutOfBoundsException caught");
        }
    }

    static void testTypeMismatch() {
        boolean[] array = {true};
        try {
            java.lang.reflect.Array.getDouble(array, 0);
        } catch (IllegalArgumentException e) {
            System.out.println("IllegalArgumentException caught");
        }
    }
}
