/** Test java.lang.reflect.Array.getLength for various array types. */
public class Test {
    public static void main(String[] args) throws Exception {
        testPrimitiveArrays();
        testObjectArray();
        testEmptyArray();
        testNullArray();
        testNotAnArray();
    }

    static void testPrimitiveArrays() {
        boolean[] boolArray = {true, false};
        System.out.println("boolean length: " + java.lang.reflect.Array.getLength(boolArray));
        byte[] byteArray = {1, 2, 3};
        System.out.println("byte length: " + java.lang.reflect.Array.getLength(byteArray));
        char[] charArray = {'A', 'B', 'C', 'D'};
        System.out.println("char length: " + java.lang.reflect.Array.getLength(charArray));
        short[] shortArray = {1, 2, 3, 4, 5};
        System.out.println("short length: " + java.lang.reflect.Array.getLength(shortArray));
        int[] intArray = {1};
        System.out.println("int length: " + java.lang.reflect.Array.getLength(intArray));
        long[] longArray = {1L, 2L};
        System.out.println("long length: " + java.lang.reflect.Array.getLength(longArray));
        float[] floatArray = {1.0f, 2.0f, 3.0f};
        System.out.println("float length: " + java.lang.reflect.Array.getLength(floatArray));
        double[] doubleArray = {1.0, 2.0, 3.0, 4.0};
        System.out.println("double length: " + java.lang.reflect.Array.getLength(doubleArray));
    }

    static void testObjectArray() {
        String[] strArray = {"hello", "world"};
        System.out.println("String length: " + java.lang.reflect.Array.getLength(strArray));
        Object[] objArray = new Object[10];
        System.out.println("Object length: " + java.lang.reflect.Array.getLength(objArray));
    }

    static void testEmptyArray() {
        int[] empty = {};
        System.out.println("empty length: " + java.lang.reflect.Array.getLength(empty));
    }

    static void testNullArray() {
        try {
            java.lang.reflect.Array.getLength(null);
        } catch (NullPointerException e) {
            System.out.println("NullPointerException caught");
        }
    }

    static void testNotAnArray() {
        try {
            java.lang.reflect.Array.getLength("not an array");
        } catch (IllegalArgumentException e) {
            System.out.println("IllegalArgumentException caught");
        }
    }
}
