/** Test java.lang.reflect.Array.getBoolean for boolean arrays. */
public class Test {
    public static void main(String[] args) throws Exception {
        testBasic();
        testNullArray();
        testIndexOutOfBounds();
        testTypeMismatch();
    }

    static void testBasic() {
        boolean[] array = {true, false, true};
        System.out.println("boolean[0]: " + java.lang.reflect.Array.getBoolean(array, 0));
        System.out.println("boolean[1]: " + java.lang.reflect.Array.getBoolean(array, 1));
        System.out.println("boolean[2]: " + java.lang.reflect.Array.getBoolean(array, 2));
    }

    static void testNullArray() {
        try {
            java.lang.reflect.Array.getBoolean(null, 0);
        } catch (NullPointerException e) {
            System.out.println("NullPointerException caught");
        }
    }

    static void testIndexOutOfBounds() {
        boolean[] array = {true};
        try {
            java.lang.reflect.Array.getBoolean(array, -1);
        } catch (ArrayIndexOutOfBoundsException e) {
            System.out.println("Negative index: ArrayIndexOutOfBoundsException caught");
        }
        try {
            java.lang.reflect.Array.getBoolean(array, 1);
        } catch (ArrayIndexOutOfBoundsException e) {
            System.out.println("Over index: ArrayIndexOutOfBoundsException caught");
        }
    }

    static void testTypeMismatch() {
        int[] array = {42};
        try {
            java.lang.reflect.Array.getBoolean(array, 0);
        } catch (IllegalArgumentException e) {
            System.out.println("IllegalArgumentException caught");
        }
    }
}
