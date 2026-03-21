/** Test java.lang.reflect.Array.setByte for byte arrays. */
public class Test {
    public static void main(String[] args) throws Exception {
        testBasic();
        testWidening();
        testNullArray();
        testIndexOutOfBounds();
        testTypeMismatch();
    }

    static void testBasic() {
        byte[] array = new byte[3];
        java.lang.reflect.Array.setByte(array, 0, (byte) 1);
        java.lang.reflect.Array.setByte(array, 1, (byte) -1);
        java.lang.reflect.Array.setByte(array, 2, (byte) 127);
        System.out.println("byte[0]: " + array[0]);
        System.out.println("byte[1]: " + array[1]);
        System.out.println("byte[2]: " + array[2]);
    }

    static void testWidening() {
        int[] intArray = new int[1];
        java.lang.reflect.Array.setByte(intArray, 0, (byte) 42);
        System.out.println("byte into int[0]: " + intArray[0]);
    }

    static void testNullArray() {
        try {
            java.lang.reflect.Array.setByte(null, 0, (byte) 1);
        } catch (NullPointerException e) {
            System.out.println("NullPointerException caught");
        }
    }

    static void testIndexOutOfBounds() {
        byte[] array = {1};
        try {
            java.lang.reflect.Array.setByte(array, -1, (byte) 0);
        } catch (ArrayIndexOutOfBoundsException e) {
            System.out.println("Negative index: ArrayIndexOutOfBoundsException caught");
        }
        try {
            java.lang.reflect.Array.setByte(array, 1, (byte) 0);
        } catch (ArrayIndexOutOfBoundsException e) {
            System.out.println("Over index: ArrayIndexOutOfBoundsException caught");
        }
    }

    static void testTypeMismatch() {
        boolean[] array = new boolean[1];
        try {
            java.lang.reflect.Array.setByte(array, 0, (byte) 1);
        } catch (IllegalArgumentException e) {
            System.out.println("IllegalArgumentException caught");
        }
    }
}
