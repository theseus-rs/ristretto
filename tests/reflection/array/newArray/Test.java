/** Test java.lang.reflect.Array.newInstance(Class, int) for creating single-dimension arrays. */
public class Test {
    public static void main(String[] args) throws Exception {
        testPrimitiveArrays();
        testObjectArray();
        testZeroLength();
        testNegativeSize();
        testNullComponentType();
    }

    static void testPrimitiveArrays() {
        Object boolArray = java.lang.reflect.Array.newInstance(boolean.class, 3);
        System.out.println("boolean array class: " + boolArray.getClass().getName());
        System.out.println("boolean array length: " + java.lang.reflect.Array.getLength(boolArray));

        Object byteArray = java.lang.reflect.Array.newInstance(byte.class, 2);
        System.out.println("byte array class: " + byteArray.getClass().getName());
        System.out.println("byte array length: " + java.lang.reflect.Array.getLength(byteArray));

        Object charArray = java.lang.reflect.Array.newInstance(char.class, 4);
        System.out.println("char array class: " + charArray.getClass().getName());
        System.out.println("char array length: " + java.lang.reflect.Array.getLength(charArray));

        Object shortArray = java.lang.reflect.Array.newInstance(short.class, 5);
        System.out.println("short array class: " + shortArray.getClass().getName());
        System.out.println("short array length: " + java.lang.reflect.Array.getLength(shortArray));

        Object intArray = java.lang.reflect.Array.newInstance(int.class, 6);
        System.out.println("int array class: " + intArray.getClass().getName());
        System.out.println("int array length: " + java.lang.reflect.Array.getLength(intArray));

        Object longArray = java.lang.reflect.Array.newInstance(long.class, 7);
        System.out.println("long array class: " + longArray.getClass().getName());
        System.out.println("long array length: " + java.lang.reflect.Array.getLength(longArray));

        Object floatArray = java.lang.reflect.Array.newInstance(float.class, 8);
        System.out.println("float array class: " + floatArray.getClass().getName());
        System.out.println("float array length: " + java.lang.reflect.Array.getLength(floatArray));

        Object doubleArray = java.lang.reflect.Array.newInstance(double.class, 9);
        System.out.println("double array class: " + doubleArray.getClass().getName());
        System.out.println("double array length: " + java.lang.reflect.Array.getLength(doubleArray));
    }

    static void testObjectArray() {
        Object strArray = java.lang.reflect.Array.newInstance(String.class, 5);
        System.out.println("String array class: " + strArray.getClass().getName());
        System.out.println("String array length: " + java.lang.reflect.Array.getLength(strArray));

        // Set and get a value
        java.lang.reflect.Array.set(strArray, 0, "hello");
        System.out.println("String[0]: " + java.lang.reflect.Array.get(strArray, 0));
    }

    static void testZeroLength() {
        Object array = java.lang.reflect.Array.newInstance(int.class, 0);
        System.out.println("zero length: " + java.lang.reflect.Array.getLength(array));
    }

    static void testNegativeSize() {
        try {
            java.lang.reflect.Array.newInstance(int.class, -1);
        } catch (NegativeArraySizeException e) {
            System.out.println("NegativeArraySizeException caught");
        }
    }

    static void testNullComponentType() {
        try {
            java.lang.reflect.Array.newInstance(null, 5);
        } catch (NullPointerException e) {
            System.out.println("NullPointerException caught");
        }
    }
}
