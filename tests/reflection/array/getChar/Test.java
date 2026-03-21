/** Test java.lang.reflect.Array.getChar for char arrays. */
public class Test {
    public static void main(String[] args) throws Exception {
        testBasic();
        testNullArray();
        testIndexOutOfBounds();
        testTypeMismatch();
    }

    static void testBasic() {
        char[] array = {'A', 'z', '0'};
        System.out.println("char[0]: " + java.lang.reflect.Array.getChar(array, 0));
        System.out.println("char[1]: " + java.lang.reflect.Array.getChar(array, 1));
        System.out.println("char[2]: " + java.lang.reflect.Array.getChar(array, 2));
    }

    static void testNullArray() {
        try {
            java.lang.reflect.Array.getChar(null, 0);
        } catch (NullPointerException e) {
            System.out.println("NullPointerException caught");
        }
    }

    static void testIndexOutOfBounds() {
        char[] array = {'A'};
        try {
            java.lang.reflect.Array.getChar(array, -1);
        } catch (ArrayIndexOutOfBoundsException e) {
            System.out.println("Negative index: ArrayIndexOutOfBoundsException caught");
        }
        try {
            java.lang.reflect.Array.getChar(array, 1);
        } catch (ArrayIndexOutOfBoundsException e) {
            System.out.println("Over index: ArrayIndexOutOfBoundsException caught");
        }
    }

    static void testTypeMismatch() {
        int[] array = {42};
        try {
            java.lang.reflect.Array.getChar(array, 0);
        } catch (IllegalArgumentException e) {
            System.out.println("IllegalArgumentException caught");
        }
    }
}
