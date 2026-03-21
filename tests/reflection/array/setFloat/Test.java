/** Test java.lang.reflect.Array.setFloat for float arrays. */
public class Test {
    public static void main(String[] args) throws Exception {
        testBasic();
        testWidening();
        testNullArray();
        testIndexOutOfBounds();
        testTypeMismatch();
    }

    static void testBasic() {
        float[] array = new float[3];
        java.lang.reflect.Array.setFloat(array, 0, 3.14f);
        java.lang.reflect.Array.setFloat(array, 1, -2.5f);
        java.lang.reflect.Array.setFloat(array, 2, Float.MAX_VALUE);
        System.out.println("float[0]: " + array[0]);
        System.out.println("float[1]: " + array[1]);
        System.out.println("float[2]: " + array[2]);
    }

    static void testWidening() {
        double[] doubleArray = new double[1];
        java.lang.reflect.Array.setFloat(doubleArray, 0, 3.14f);
        System.out.println("float into double[0]: " + doubleArray[0]);
    }

    static void testNullArray() {
        try {
            java.lang.reflect.Array.setFloat(null, 0, 1.0f);
        } catch (NullPointerException e) {
            System.out.println("NullPointerException caught");
        }
    }

    static void testIndexOutOfBounds() {
        float[] array = {1.0f};
        try {
            java.lang.reflect.Array.setFloat(array, -1, 0.0f);
        } catch (ArrayIndexOutOfBoundsException e) {
            System.out.println("Negative index: ArrayIndexOutOfBoundsException caught");
        }
        try {
            java.lang.reflect.Array.setFloat(array, 1, 0.0f);
        } catch (ArrayIndexOutOfBoundsException e) {
            System.out.println("Over index: ArrayIndexOutOfBoundsException caught");
        }
    }

    static void testTypeMismatch() {
        boolean[] array = new boolean[1];
        try {
            java.lang.reflect.Array.setFloat(array, 0, 1.0f);
        } catch (IllegalArgumentException e) {
            System.out.println("IllegalArgumentException caught");
        }
    }
}
