/** Test java.lang.reflect.Array.getShort for short arrays. */
public class Test {
    public static void main(String[] args) throws Exception {
        testBasic();
        testWidening();
        testNullArray();
        testIndexOutOfBounds();
        testTypeMismatch();
    }

    static void testBasic() {
        short[] array = {100, -100, 32767};
        System.out.println("short[0]: " + java.lang.reflect.Array.getShort(array, 0));
        System.out.println("short[1]: " + java.lang.reflect.Array.getShort(array, 1));
        System.out.println("short[2]: " + java.lang.reflect.Array.getShort(array, 2));
    }

    static void testWidening() {
        byte[] byteArray = {42};
        System.out.println("byte as short: " + java.lang.reflect.Array.getShort(byteArray, 0));
    }

    static void testNullArray() {
        try {
            java.lang.reflect.Array.getShort(null, 0);
        } catch (NullPointerException e) {
            System.out.println("NullPointerException caught");
        }
    }

    static void testIndexOutOfBounds() {
        short[] array = {1};
        try {
            java.lang.reflect.Array.getShort(array, -1);
        } catch (ArrayIndexOutOfBoundsException e) {
            System.out.println("Negative index: ArrayIndexOutOfBoundsException caught");
        }
        try {
            java.lang.reflect.Array.getShort(array, 1);
        } catch (ArrayIndexOutOfBoundsException e) {
            System.out.println("Over index: ArrayIndexOutOfBoundsException caught");
        }
    }

    static void testTypeMismatch() {
        boolean[] array = {true};
        try {
            java.lang.reflect.Array.getShort(array, 0);
        } catch (IllegalArgumentException e) {
            System.out.println("IllegalArgumentException caught");
        }
    }
}
