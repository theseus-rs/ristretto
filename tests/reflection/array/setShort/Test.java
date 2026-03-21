/** Test java.lang.reflect.Array.setShort for short arrays. */
public class Test {
    public static void main(String[] args) throws Exception {
        testBasic();
        testWidening();
        testNullArray();
        testIndexOutOfBounds();
        testTypeMismatch();
    }

    static void testBasic() {
        short[] array = new short[3];
        java.lang.reflect.Array.setShort(array, 0, (short) 100);
        java.lang.reflect.Array.setShort(array, 1, (short) -100);
        java.lang.reflect.Array.setShort(array, 2, (short) 32767);
        System.out.println("short[0]: " + array[0]);
        System.out.println("short[1]: " + array[1]);
        System.out.println("short[2]: " + array[2]);
    }

    static void testWidening() {
        int[] intArray = new int[1];
        java.lang.reflect.Array.setShort(intArray, 0, (short) 1000);
        System.out.println("short into int[0]: " + intArray[0]);
    }

    static void testNullArray() {
        try {
            java.lang.reflect.Array.setShort(null, 0, (short) 1);
        } catch (NullPointerException e) {
            System.out.println("NullPointerException caught");
        }
    }

    static void testIndexOutOfBounds() {
        short[] array = {1};
        try {
            java.lang.reflect.Array.setShort(array, -1, (short) 0);
        } catch (ArrayIndexOutOfBoundsException e) {
            System.out.println("Negative index: ArrayIndexOutOfBoundsException caught");
        }
        try {
            java.lang.reflect.Array.setShort(array, 1, (short) 0);
        } catch (ArrayIndexOutOfBoundsException e) {
            System.out.println("Over index: ArrayIndexOutOfBoundsException caught");
        }
    }

    static void testTypeMismatch() {
        boolean[] array = new boolean[1];
        try {
            java.lang.reflect.Array.setShort(array, 0, (short) 1);
        } catch (IllegalArgumentException e) {
            System.out.println("IllegalArgumentException caught");
        }
    }
}
