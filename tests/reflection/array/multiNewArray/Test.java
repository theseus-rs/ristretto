/** Test java.lang.reflect.Array.newInstance(Class, int[]) for creating multi-dimensional arrays. */
public class Test {
    public static void main(String[] args) throws Exception {
        test2DArray();
        test3DArray();
        testObjectMultiArray();
        testNegativeSize();
    }

    static void test2DArray() {
        Object array = java.lang.reflect.Array.newInstance(int.class, new int[]{3, 4});
        System.out.println("2D int array class: " + array.getClass().getName());
        System.out.println("2D int array length: " + java.lang.reflect.Array.getLength(array));

        Object row0 = java.lang.reflect.Array.get(array, 0);
        System.out.println("row0 class: " + row0.getClass().getName());
        System.out.println("row0 length: " + java.lang.reflect.Array.getLength(row0));
    }

    static void test3DArray() {
        Object array = java.lang.reflect.Array.newInstance(double.class, new int[]{2, 3, 4});
        System.out.println("3D double array class: " + array.getClass().getName());
        System.out.println("3D double array length: " + java.lang.reflect.Array.getLength(array));

        Object slice0 = java.lang.reflect.Array.get(array, 0);
        System.out.println("slice0 class: " + slice0.getClass().getName());
        System.out.println("slice0 length: " + java.lang.reflect.Array.getLength(slice0));

        Object row0 = java.lang.reflect.Array.get(slice0, 0);
        System.out.println("row0 length: " + java.lang.reflect.Array.getLength(row0));
    }

    static void testObjectMultiArray() {
        Object array = java.lang.reflect.Array.newInstance(String.class, new int[]{2, 3});
        System.out.println("2D String array class: " + array.getClass().getName());
        System.out.println("2D String array length: " + java.lang.reflect.Array.getLength(array));
    }

    static void testNegativeSize() {
        try {
            java.lang.reflect.Array.newInstance(int.class, new int[]{-1});
        } catch (NegativeArraySizeException e) {
            System.out.println("NegativeArraySizeException caught");
        }
    }
}
