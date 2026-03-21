/** Test java.lang.reflect.Array.getFloat for float arrays. */
public class Test {
    public static void main(String[] args) throws Exception {
        testBasic();
        testWidening();
        testNullArray();
        testIndexOutOfBounds();
        testTypeMismatch();
    }

    static void testBasic() {
        float[] array = {3.14f, -2.5f, Float.MAX_VALUE};
        System.out.println("float[0]: " + java.lang.reflect.Array.getFloat(array, 0));
        System.out.println("float[1]: " + java.lang.reflect.Array.getFloat(array, 1));
        System.out.println("float[2]: " + java.lang.reflect.Array.getFloat(array, 2));
    }

    static void testWidening() {
        int[] intArray = {42};
        System.out.println("int as float: " + java.lang.reflect.Array.getFloat(intArray, 0));
        byte[] byteArray = {10};
        System.out.println("byte as float: " + java.lang.reflect.Array.getFloat(byteArray, 0));
    }

    static void testNullArray() {
        try {
            java.lang.reflect.Array.getFloat(null, 0);
        } catch (NullPointerException e) {
            System.out.println("NullPointerException caught");
        }
    }

    static void testIndexOutOfBounds() {
        float[] array = {1.0f};
        try {
            java.lang.reflect.Array.getFloat(array, -1);
        } catch (ArrayIndexOutOfBoundsException e) {
            System.out.println("Negative index: ArrayIndexOutOfBoundsException caught");
        }
        try {
            java.lang.reflect.Array.getFloat(array, 1);
        } catch (ArrayIndexOutOfBoundsException e) {
            System.out.println("Over index: ArrayIndexOutOfBoundsException caught");
        }
    }

    static void testTypeMismatch() {
        boolean[] array = {true};
        try {
            java.lang.reflect.Array.getFloat(array, 0);
        } catch (IllegalArgumentException e) {
            System.out.println("IllegalArgumentException caught");
        }
    }
}
