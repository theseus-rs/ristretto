/** Test java.lang.reflect.Array.set for all array types. */
public class Test {
    public static void main(String[] args) throws Exception {
        testBooleanArray();
        testByteArray();
        testCharArray();
        testShortArray();
        testIntArray();
        testLongArray();
        testFloatArray();
        testDoubleArray();
        testObjectArray();
        testNullArray();
        testIndexOutOfBounds();
    }

    static void testBooleanArray() {
        boolean[] array = new boolean[2];
        java.lang.reflect.Array.set(array, 0, Boolean.TRUE);
        java.lang.reflect.Array.set(array, 1, Boolean.FALSE);
        System.out.println("boolean[0]: " + array[0]);
        System.out.println("boolean[1]: " + array[1]);
    }

    static void testByteArray() {
        byte[] array = new byte[2];
        java.lang.reflect.Array.set(array, 0, (byte) 42);
        java.lang.reflect.Array.set(array, 1, (byte) -1);
        System.out.println("byte[0]: " + array[0]);
        System.out.println("byte[1]: " + array[1]);
    }

    static void testCharArray() {
        char[] array = new char[2];
        java.lang.reflect.Array.set(array, 0, 'X');
        java.lang.reflect.Array.set(array, 1, 'Y');
        System.out.println("char[0]: " + array[0]);
        System.out.println("char[1]: " + array[1]);
    }

    static void testShortArray() {
        short[] array = new short[2];
        java.lang.reflect.Array.set(array, 0, (short) 1000);
        java.lang.reflect.Array.set(array, 1, (short) -1000);
        System.out.println("short[0]: " + array[0]);
        System.out.println("short[1]: " + array[1]);
    }

    static void testIntArray() {
        int[] array = new int[2];
        java.lang.reflect.Array.set(array, 0, 42);
        java.lang.reflect.Array.set(array, 1, -42);
        System.out.println("int[0]: " + array[0]);
        System.out.println("int[1]: " + array[1]);
    }

    static void testLongArray() {
        long[] array = new long[2];
        java.lang.reflect.Array.set(array, 0, 100L);
        java.lang.reflect.Array.set(array, 1, -100L);
        System.out.println("long[0]: " + array[0]);
        System.out.println("long[1]: " + array[1]);
    }

    static void testFloatArray() {
        float[] array = new float[2];
        java.lang.reflect.Array.set(array, 0, 3.14f);
        java.lang.reflect.Array.set(array, 1, -2.5f);
        System.out.println("float[0]: " + array[0]);
        System.out.println("float[1]: " + array[1]);
    }

    static void testDoubleArray() {
        double[] array = new double[2];
        java.lang.reflect.Array.set(array, 0, 3.14159);
        java.lang.reflect.Array.set(array, 1, -2.71828);
        System.out.println("double[0]: " + array[0]);
        System.out.println("double[1]: " + array[1]);
    }

    static void testObjectArray() {
        String[] array = new String[3];
        java.lang.reflect.Array.set(array, 0, "hello");
        java.lang.reflect.Array.set(array, 1, "world");
        java.lang.reflect.Array.set(array, 2, null);
        System.out.println("Object[0]: " + array[0]);
        System.out.println("Object[1]: " + array[1]);
        System.out.println("Object[2]: " + array[2]);
    }

    static void testNullArray() {
        try {
            java.lang.reflect.Array.set(null, 0, "value");
        } catch (NullPointerException e) {
            System.out.println("NullPointerException caught");
        }
    }

    static void testIndexOutOfBounds() {
        int[] array = {1, 2, 3};
        try {
            java.lang.reflect.Array.set(array, -1, 0);
        } catch (ArrayIndexOutOfBoundsException e) {
            System.out.println("Negative index: ArrayIndexOutOfBoundsException caught");
        }
        try {
            java.lang.reflect.Array.set(array, 3, 0);
        } catch (ArrayIndexOutOfBoundsException e) {
            System.out.println("Over index: ArrayIndexOutOfBoundsException caught");
        }
    }
}
