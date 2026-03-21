/** Test java.lang.reflect.Array.getInt for int arrays. */
public class Test {
    public static void main(String[] args) throws Exception {
        testBasic();
        testWidening();
        testNullArray();
        testIndexOutOfBounds();
        testTypeMismatch();
    }

    static void testBasic() {
        int[] array = {42, -1, 2147483647};
        System.out.println("int[0]: " + java.lang.reflect.Array.getInt(array, 0));
        System.out.println("int[1]: " + java.lang.reflect.Array.getInt(array, 1));
        System.out.println("int[2]: " + java.lang.reflect.Array.getInt(array, 2));
    }

    static void testWidening() {
        byte[] byteArray = {42};
        System.out.println("byte as int: " + java.lang.reflect.Array.getInt(byteArray, 0));
        short[] shortArray = {1000};
        System.out.println("short as int: " + java.lang.reflect.Array.getInt(shortArray, 0));
        char[] charArray = {'A'};
        System.out.println("char as int: " + java.lang.reflect.Array.getInt(charArray, 0));
    }

    static void testNullArray() {
        try {
            java.lang.reflect.Array.getInt(null, 0);
        } catch (NullPointerException e) {
            System.out.println("NullPointerException caught");
        }
    }

    static void testIndexOutOfBounds() {
        int[] array = {1};
        try {
            java.lang.reflect.Array.getInt(array, -1);
        } catch (ArrayIndexOutOfBoundsException e) {
            System.out.println("Negative index: ArrayIndexOutOfBoundsException caught");
        }
        try {
            java.lang.reflect.Array.getInt(array, 1);
        } catch (ArrayIndexOutOfBoundsException e) {
            System.out.println("Over index: ArrayIndexOutOfBoundsException caught");
        }
    }

    static void testTypeMismatch() {
        boolean[] array = {true};
        try {
            java.lang.reflect.Array.getInt(array, 0);
        } catch (IllegalArgumentException e) {
            System.out.println("IllegalArgumentException caught");
        }
    }
}
