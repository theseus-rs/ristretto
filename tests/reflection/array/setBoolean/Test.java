/** Test java.lang.reflect.Array.setBoolean for boolean arrays. */
public class Test {
    public static void main(String[] args) throws Exception {
        testBasic();
        testNullArray();
        testIndexOutOfBounds();
        testTypeMismatch();
    }

    static void testBasic() {
        boolean[] array = new boolean[3];
        java.lang.reflect.Array.setBoolean(array, 0, true);
        java.lang.reflect.Array.setBoolean(array, 1, false);
        java.lang.reflect.Array.setBoolean(array, 2, true);
        System.out.println("boolean[0]: " + array[0]);
        System.out.println("boolean[1]: " + array[1]);
        System.out.println("boolean[2]: " + array[2]);
    }

    static void testNullArray() {
        try {
            java.lang.reflect.Array.setBoolean(null, 0, true);
        } catch (NullPointerException e) {
            System.out.println("NullPointerException caught");
        }
    }

    static void testIndexOutOfBounds() {
        boolean[] array = {true};
        try {
            java.lang.reflect.Array.setBoolean(array, -1, false);
        } catch (ArrayIndexOutOfBoundsException e) {
            System.out.println("Negative index: ArrayIndexOutOfBoundsException caught");
        }
        try {
            java.lang.reflect.Array.setBoolean(array, 1, false);
        } catch (ArrayIndexOutOfBoundsException e) {
            System.out.println("Over index: ArrayIndexOutOfBoundsException caught");
        }
    }

    static void testTypeMismatch() {
        int[] array = new int[1];
        try {
            java.lang.reflect.Array.setBoolean(array, 0, true);
        } catch (IllegalArgumentException e) {
            System.out.println("IllegalArgumentException caught");
        }
    }
}
