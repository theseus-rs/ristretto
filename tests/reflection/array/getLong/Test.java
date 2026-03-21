/** Test java.lang.reflect.Array.getLong for long arrays. */
public class Test {
    public static void main(String[] args) throws Exception {
        testBasic();
        testWidening();
        testNullArray();
        testIndexOutOfBounds();
        testTypeMismatch();
    }

    static void testBasic() {
        long[] array = {100L, -100L, 9223372036854775807L};
        System.out.println("long[0]: " + java.lang.reflect.Array.getLong(array, 0));
        System.out.println("long[1]: " + java.lang.reflect.Array.getLong(array, 1));
        System.out.println("long[2]: " + java.lang.reflect.Array.getLong(array, 2));
    }

    static void testWidening() {
        int[] intArray = {42};
        System.out.println("int as long: " + java.lang.reflect.Array.getLong(intArray, 0));
        byte[] byteArray = {10};
        System.out.println("byte as long: " + java.lang.reflect.Array.getLong(byteArray, 0));
    }

    static void testNullArray() {
        try {
            java.lang.reflect.Array.getLong(null, 0);
        } catch (NullPointerException e) {
            System.out.println("NullPointerException caught");
        }
    }

    static void testIndexOutOfBounds() {
        long[] array = {1L};
        try {
            java.lang.reflect.Array.getLong(array, -1);
        } catch (ArrayIndexOutOfBoundsException e) {
            System.out.println("Negative index: ArrayIndexOutOfBoundsException caught");
        }
        try {
            java.lang.reflect.Array.getLong(array, 1);
        } catch (ArrayIndexOutOfBoundsException e) {
            System.out.println("Over index: ArrayIndexOutOfBoundsException caught");
        }
    }

    static void testTypeMismatch() {
        boolean[] array = {true};
        try {
            java.lang.reflect.Array.getLong(array, 0);
        } catch (IllegalArgumentException e) {
            System.out.println("IllegalArgumentException caught");
        }
    }
}
