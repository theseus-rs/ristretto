/** Test java.lang.reflect.Array.setLong for long arrays. */
public class Test {
    public static void main(String[] args) throws Exception {
        testBasic();
        testWidening();
        testNullArray();
        testIndexOutOfBounds();
        testTypeMismatch();
    }

    static void testBasic() {
        long[] array = new long[3];
        java.lang.reflect.Array.setLong(array, 0, 100L);
        java.lang.reflect.Array.setLong(array, 1, -100L);
        java.lang.reflect.Array.setLong(array, 2, 9223372036854775807L);
        System.out.println("long[0]: " + array[0]);
        System.out.println("long[1]: " + array[1]);
        System.out.println("long[2]: " + array[2]);
    }

    static void testWidening() {
        float[] floatArray = new float[1];
        java.lang.reflect.Array.setLong(floatArray, 0, 42L);
        System.out.println("long into float[0]: " + floatArray[0]);
    }

    static void testNullArray() {
        try {
            java.lang.reflect.Array.setLong(null, 0, 1L);
        } catch (NullPointerException e) {
            System.out.println("NullPointerException caught");
        }
    }

    static void testIndexOutOfBounds() {
        long[] array = {1L};
        try {
            java.lang.reflect.Array.setLong(array, -1, 0L);
        } catch (ArrayIndexOutOfBoundsException e) {
            System.out.println("Negative index: ArrayIndexOutOfBoundsException caught");
        }
        try {
            java.lang.reflect.Array.setLong(array, 1, 0L);
        } catch (ArrayIndexOutOfBoundsException e) {
            System.out.println("Over index: ArrayIndexOutOfBoundsException caught");
        }
    }

    static void testTypeMismatch() {
        boolean[] array = new boolean[1];
        try {
            java.lang.reflect.Array.setLong(array, 0, 1L);
        } catch (IllegalArgumentException e) {
            System.out.println("IllegalArgumentException caught");
        }
    }
}
