/** Test java.lang.reflect.Array.setDouble for double arrays. */
public class Test {
    public static void main(String[] args) throws Exception {
        testBasic();
        testNullArray();
        testIndexOutOfBounds();
        testTypeMismatch();
    }

    static void testBasic() {
        double[] array = new double[3];
        java.lang.reflect.Array.setDouble(array, 0, 3.14159);
        java.lang.reflect.Array.setDouble(array, 1, -2.71828);
        java.lang.reflect.Array.setDouble(array, 2, Double.MAX_VALUE);
        System.out.println("double[0]: " + array[0]);
        System.out.println("double[1]: " + array[1]);
        System.out.println("double[2]: " + array[2]);
    }

    static void testNullArray() {
        try {
            java.lang.reflect.Array.setDouble(null, 0, 1.0);
        } catch (NullPointerException e) {
            System.out.println("NullPointerException caught");
        }
    }

    static void testIndexOutOfBounds() {
        double[] array = {1.0};
        try {
            java.lang.reflect.Array.setDouble(array, -1, 0.0);
        } catch (ArrayIndexOutOfBoundsException e) {
            System.out.println("Negative index: ArrayIndexOutOfBoundsException caught");
        }
        try {
            java.lang.reflect.Array.setDouble(array, 1, 0.0);
        } catch (ArrayIndexOutOfBoundsException e) {
            System.out.println("Over index: ArrayIndexOutOfBoundsException caught");
        }
    }

    static void testTypeMismatch() {
        boolean[] array = new boolean[1];
        try {
            java.lang.reflect.Array.setDouble(array, 0, 1.0);
        } catch (IllegalArgumentException e) {
            System.out.println("IllegalArgumentException caught");
        }
    }
}
