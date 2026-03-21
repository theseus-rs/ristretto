/** Test java.lang.reflect.Array.get for all array types. */
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
        boolean[] array = {true, false, true};
        Object value0 = java.lang.reflect.Array.get(array, 0);
        Object value1 = java.lang.reflect.Array.get(array, 1);
        Object value2 = java.lang.reflect.Array.get(array, 2);
        System.out.println("boolean[0]: " + value0);
        System.out.println("boolean[1]: " + value1);
        System.out.println("boolean[2]: " + value2);
        System.out.println("boolean type: " + value0.getClass().getName());
    }

    static void testByteArray() {
        byte[] array = {1, -1, 127};
        Object value0 = java.lang.reflect.Array.get(array, 0);
        Object value1 = java.lang.reflect.Array.get(array, 1);
        Object value2 = java.lang.reflect.Array.get(array, 2);
        System.out.println("byte[0]: " + value0);
        System.out.println("byte[1]: " + value1);
        System.out.println("byte[2]: " + value2);
        System.out.println("byte type: " + value0.getClass().getName());
    }

    static void testCharArray() {
        char[] array = {'A', 'z', '0'};
        Object value0 = java.lang.reflect.Array.get(array, 0);
        Object value1 = java.lang.reflect.Array.get(array, 1);
        Object value2 = java.lang.reflect.Array.get(array, 2);
        System.out.println("char[0]: " + value0);
        System.out.println("char[1]: " + value1);
        System.out.println("char[2]: " + value2);
        System.out.println("char type: " + value0.getClass().getName());
    }

    static void testShortArray() {
        short[] array = {100, -100, 32767};
        Object value0 = java.lang.reflect.Array.get(array, 0);
        Object value1 = java.lang.reflect.Array.get(array, 1);
        Object value2 = java.lang.reflect.Array.get(array, 2);
        System.out.println("short[0]: " + value0);
        System.out.println("short[1]: " + value1);
        System.out.println("short[2]: " + value2);
        System.out.println("short type: " + value0.getClass().getName());
    }

    static void testIntArray() {
        int[] array = {42, -1, 2147483647};
        Object value0 = java.lang.reflect.Array.get(array, 0);
        Object value1 = java.lang.reflect.Array.get(array, 1);
        Object value2 = java.lang.reflect.Array.get(array, 2);
        System.out.println("int[0]: " + value0);
        System.out.println("int[1]: " + value1);
        System.out.println("int[2]: " + value2);
        System.out.println("int type: " + value0.getClass().getName());
    }

    static void testLongArray() {
        long[] array = {100L, -100L, 9223372036854775807L};
        Object value0 = java.lang.reflect.Array.get(array, 0);
        Object value1 = java.lang.reflect.Array.get(array, 1);
        Object value2 = java.lang.reflect.Array.get(array, 2);
        System.out.println("long[0]: " + value0);
        System.out.println("long[1]: " + value1);
        System.out.println("long[2]: " + value2);
        System.out.println("long type: " + value0.getClass().getName());
    }

    static void testFloatArray() {
        float[] array = {3.14f, -2.5f, Float.MAX_VALUE};
        Object value0 = java.lang.reflect.Array.get(array, 0);
        Object value1 = java.lang.reflect.Array.get(array, 1);
        Object value2 = java.lang.reflect.Array.get(array, 2);
        System.out.println("float[0]: " + value0);
        System.out.println("float[1]: " + value1);
        System.out.println("float[2]: " + value2);
        System.out.println("float type: " + value0.getClass().getName());
    }

    static void testDoubleArray() {
        double[] array = {3.14159, -2.71828, Double.MAX_VALUE};
        Object value0 = java.lang.reflect.Array.get(array, 0);
        Object value1 = java.lang.reflect.Array.get(array, 1);
        Object value2 = java.lang.reflect.Array.get(array, 2);
        System.out.println("double[0]: " + value0);
        System.out.println("double[1]: " + value1);
        System.out.println("double[2]: " + value2);
        System.out.println("double type: " + value0.getClass().getName());
    }

    static void testObjectArray() {
        String[] array = {"hello", "world", null};
        Object value0 = java.lang.reflect.Array.get(array, 0);
        Object value1 = java.lang.reflect.Array.get(array, 1);
        Object value2 = java.lang.reflect.Array.get(array, 2);
        System.out.println("Object[0]: " + value0);
        System.out.println("Object[1]: " + value1);
        System.out.println("Object[2]: " + value2);
        System.out.println("Object type: " + value0.getClass().getName());
    }

    static void testNullArray() {
        try {
            java.lang.reflect.Array.get(null, 0);
        } catch (NullPointerException e) {
            System.out.println("NullPointerException caught");
        }
    }

    static void testIndexOutOfBounds() {
        int[] array = {1, 2, 3};
        try {
            java.lang.reflect.Array.get(array, -1);
        } catch (ArrayIndexOutOfBoundsException e) {
            System.out.println("Negative index: ArrayIndexOutOfBoundsException caught");
        }
        try {
            java.lang.reflect.Array.get(array, 3);
        } catch (ArrayIndexOutOfBoundsException e) {
            System.out.println("Over index: ArrayIndexOutOfBoundsException caught");
        }
    }
}
