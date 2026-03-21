/** Test java.lang.reflect.Array.getByte for byte arrays. */
public class Test {
    public static void main(String[] args) throws Exception {
        testBasic();
        testNullArray();
        testIndexOutOfBounds();
        testTypeMismatch();
    }

    static void testBasic() {
        byte[] array = {1, -1, 127};
        System.out.println("byte[0]: " + java.lang.reflect.Array.getByte(array, 0));
        System.out.println("byte[1]: " + java.lang.reflect.Array.getByte(array, 1));
        System.out.println("byte[2]: " + java.lang.reflect.Array.getByte(array, 2));
    }

    static void testNullArray() {
        try {
            java.lang.reflect.Array.getByte(null, 0);
        } catch (NullPointerException e) {
            System.out.println("NullPointerException caught");
        }
    }

    static void testIndexOutOfBounds() {
        byte[] array = {1};
        try {
            java.lang.reflect.Array.getByte(array, -1);
        } catch (ArrayIndexOutOfBoundsException e) {
            System.out.println("Negative index: ArrayIndexOutOfBoundsException caught");
        }
        try {
            java.lang.reflect.Array.getByte(array, 1);
        } catch (ArrayIndexOutOfBoundsException e) {
            System.out.println("Over index: ArrayIndexOutOfBoundsException caught");
        }
    }

    static void testTypeMismatch() {
        boolean[] array = {true};
        try {
            java.lang.reflect.Array.getByte(array, 0);
        } catch (IllegalArgumentException e) {
            System.out.println("IllegalArgumentException caught");
        }
    }
}
