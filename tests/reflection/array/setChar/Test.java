/** Test java.lang.reflect.Array.setChar for char arrays. */
public class Test {
    public static void main(String[] args) throws Exception {
        testBasic();
        testWidening();
        testNullArray();
        testIndexOutOfBounds();
        testTypeMismatch();
    }

    static void testBasic() {
        char[] array = new char[3];
        java.lang.reflect.Array.setChar(array, 0, 'A');
        java.lang.reflect.Array.setChar(array, 1, 'z');
        java.lang.reflect.Array.setChar(array, 2, '0');
        System.out.println("char[0]: " + array[0]);
        System.out.println("char[1]: " + array[1]);
        System.out.println("char[2]: " + array[2]);
    }

    static void testWidening() {
        int[] intArray = new int[1];
        java.lang.reflect.Array.setChar(intArray, 0, 'A');
        System.out.println("char into int[0]: " + intArray[0]);
    }

    static void testNullArray() {
        try {
            java.lang.reflect.Array.setChar(null, 0, 'A');
        } catch (NullPointerException e) {
            System.out.println("NullPointerException caught");
        }
    }

    static void testIndexOutOfBounds() {
        char[] array = {'A'};
        try {
            java.lang.reflect.Array.setChar(array, -1, 'B');
        } catch (ArrayIndexOutOfBoundsException e) {
            System.out.println("Negative index: ArrayIndexOutOfBoundsException caught");
        }
        try {
            java.lang.reflect.Array.setChar(array, 1, 'B');
        } catch (ArrayIndexOutOfBoundsException e) {
            System.out.println("Over index: ArrayIndexOutOfBoundsException caught");
        }
    }

    static void testTypeMismatch() {
        boolean[] array = new boolean[1];
        try {
            java.lang.reflect.Array.setChar(array, 0, 'A');
        } catch (IllegalArgumentException e) {
            System.out.println("IllegalArgumentException caught");
        }
    }
}
