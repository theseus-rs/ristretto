/** Test java.lang.reflect.Array.setInt for int arrays. */
public class Test {
    public static void main(String[] args) throws Exception {
        testBasic();
        testWidening();
        testNullArray();
        testIndexOutOfBounds();
        testTypeMismatch();
    }

    static void testBasic() {
        int[] array = new int[3];
        java.lang.reflect.Array.setInt(array, 0, 42);
        java.lang.reflect.Array.setInt(array, 1, -1);
        java.lang.reflect.Array.setInt(array, 2, 2147483647);
        System.out.println("int[0]: " + array[0]);
        System.out.println("int[1]: " + array[1]);
        System.out.println("int[2]: " + array[2]);
    }

    static void testWidening() {
        long[] longArray = new long[1];
        java.lang.reflect.Array.setInt(longArray, 0, 42);
        System.out.println("int into long[0]: " + longArray[0]);
    }

    static void testNullArray() {
        try {
            java.lang.reflect.Array.setInt(null, 0, 1);
        } catch (NullPointerException e) {
            System.out.println("NullPointerException caught");
        }
    }

    static void testIndexOutOfBounds() {
        int[] array = {1};
        try {
            java.lang.reflect.Array.setInt(array, -1, 0);
        } catch (ArrayIndexOutOfBoundsException e) {
            System.out.println("Negative index: ArrayIndexOutOfBoundsException caught");
        }
        try {
            java.lang.reflect.Array.setInt(array, 1, 0);
        } catch (ArrayIndexOutOfBoundsException e) {
            System.out.println("Over index: ArrayIndexOutOfBoundsException caught");
        }
    }

    static void testTypeMismatch() {
        boolean[] array = new boolean[1];
        try {
            java.lang.reflect.Array.setInt(array, 0, 1);
        } catch (IllegalArgumentException e) {
            System.out.println("IllegalArgumentException caught");
        }
    }
}
