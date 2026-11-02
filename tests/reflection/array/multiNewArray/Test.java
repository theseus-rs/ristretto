/** Test java.lang.reflect.Array.newInstance(Class, int[]) for creating multi-dimensional arrays. */
public class Test {
    public static void main(String[] args) throws Exception {
        test2DArray();
        test3DArray();
        testObjectMultiArray();
        testPrimitiveLeaves();
        testIndependentRows();
        testArrayComponent(String[].class, String[][].class, String[][][].class, String[][][][].class);
        testArrayComponent(int[].class, int[][].class, int[][][].class, int[][][][].class);
        testArrayComponent(String[][].class, String[][][].class, String[][][][].class, String[][][][][].class);
        testArrayComponent(int[][].class, int[][][].class, int[][][][].class, int[][][][][].class);
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
        for (int[] dimensions : new int[][]{{-1}, {2, -1}, {0, -1}, {2, 0, -1}}) {
            try {
                java.lang.reflect.Array.newInstance(int.class, dimensions);
                throw new AssertionError("negative dimension accepted");
            } catch (NegativeArraySizeException e) {
                System.out.println("NegativeArraySizeException caught: "
                        + java.util.Arrays.toString(dimensions));
            }
        }
    }

    static void testPrimitiveLeaves() {
        Class<?>[] components = {boolean.class, byte.class, char.class, short.class,
                int.class, long.class, float.class, double.class};
        Object[] zeros = {false, (byte) 0, (char) 0, (short) 0, 0, 0L, 0.0f, 0.0d};
        for (int i = 0; i < components.length; i++) {
            Object[] array = (Object[]) java.lang.reflect.Array.newInstance(components[i], new int[]{2, 3});
            if (array[0] == array[1]) throw new AssertionError("shared primitive rows");
            for (Object row : array) {
                if (row.getClass().getComponentType() != components[i]
                        || java.lang.reflect.Array.getLength(row) != 3) {
                    throw new AssertionError("wrong primitive row type or length");
                }
                for (int j = 0; j < 3; j++) {
                    if (!java.lang.reflect.Array.get(row, j).equals(zeros[i])) {
                        throw new AssertionError("primitive leaf was not zero initialized");
                    }
                }
            }
            System.out.println("primitive leaves: " + components[i].getName());
        }
    }

    static void testIndependentRows() {
        int[][][] numbers = (int[][][]) java.lang.reflect.Array.newInstance(int.class, new int[]{2, 2, 3});
        numbers[0][0][0] = 7;
        if (numbers[0][1][0] != 0 || numbers[1][0][0] != 0) {
            throw new AssertionError("shared nested primitive rows");
        }
        String[][] strings = (String[][]) java.lang.reflect.Array.newInstance(String.class, new int[]{2, 3});
        strings[0][0] = "changed";
        if (strings[1][0] != null) throw new AssertionError("shared reference rows");
        System.out.println("independent rows");
    }

    static void testArrayComponent(Class<?> component, Class<?> oneDimension,
            Class<?> twoDimensions, Class<?> threeDimensions) {
        int[][] dimensions = {{2}, {2, 3}, {2, 1, 3}, {0, 3}, {2, 0}};
        Class<?>[] expected = {oneDimension, twoDimensions, threeDimensions, twoDimensions, twoDimensions};
        for (int i = 0; i < dimensions.length; i++) {
            Object array = java.lang.reflect.Array.newInstance(component, dimensions[i]);
            checkArray(array, expected[i], dimensions[i], 0);
            System.out.println("array component: " + component.getName()
                    + ", dimensions: " + java.util.Arrays.toString(dimensions[i]));
        }
    }

    static void checkArray(Object array, Class<?> expected, int[] dimensions, int depth) {
        if (array.getClass() != expected
                || java.lang.reflect.Array.getLength(array) != dimensions[depth]) {
            throw new AssertionError("wrong array type or length at depth " + depth);
        }
        for (int i = 0; i < dimensions[depth]; i++) {
            Object element = java.lang.reflect.Array.get(array, i);
            if (depth + 1 < dimensions.length) {
                checkArray(element, expected.getComponentType(), dimensions, depth + 1);
            } else if (element != null) {
                throw new AssertionError("component arrays must remain unallocated");
            }
        }
    }
}
