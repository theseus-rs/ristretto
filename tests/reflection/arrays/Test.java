/** Test array reflection operations. */
public class Test {
    static class A {
        public String[] stringArray = {"a", "b", "c"};
        public int[][] intMatrix = {{1, 2}, {3, 4}, {5, 6}};
        public Object[] objectArray = {new String("test"), Integer.valueOf(42), Boolean.TRUE};
    }

    public static void main(String[] args) throws Exception {
        // Test array class creation
        Class<?> stringArrayClass = Class.forName("[Ljava.lang.String;");
        System.out.println("String array class name: " + stringArrayClass.getName());
        System.out.println("Is array: " + stringArrayClass.isArray());
        System.out.println("Component type: " + stringArrayClass.getComponentType().getName());

        // Test multi-dimensional array
        Class<?> intMatrixClass = int[][].class;
        System.out.println("Int matrix class name: " + intMatrixClass.getName());
        System.out.println("Component type: " + intMatrixClass.getComponentType().getName());
        System.out.println("Component is array: " + intMatrixClass.getComponentType().isArray());

        // Test primitive array
        Class<?> intArrayClass = int[].class;
        System.out.println("Int array class name: " + intArrayClass.getName());
        System.out.println("Component type: " + intArrayClass.getComponentType().getName());
        System.out.println("Component is primitive: " + intArrayClass.getComponentType().isPrimitive());

        // Test array creation using reflection
        Object stringArray = java.lang.reflect.Array.newInstance(String.class, 3);
        System.out.println("Created array type: " + stringArray.getClass().getName());
        System.out.println("Array length: " + java.lang.reflect.Array.getLength(stringArray));

        // Test array element access
        java.lang.reflect.Array.set(stringArray, 0, "first");
        java.lang.reflect.Array.set(stringArray, 1, "second");
        java.lang.reflect.Array.set(stringArray, 2, "third");

        Object element = java.lang.reflect.Array.get(stringArray, 1);
        System.out.println("Array element at index 1: " + element);

        // Test multi-dimensional array creation
        Object intMatrix = java.lang.reflect.Array.newInstance(int.class, 2, 3);
        System.out.println("Created matrix type: " + intMatrix.getClass().getName());
        System.out.println("Matrix length: " + java.lang.reflect.Array.getLength(intMatrix));

        // Access matrix element
        Object row = java.lang.reflect.Array.get(intMatrix, 0);
        java.lang.reflect.Array.setInt(row, 0, 100);
        int value = java.lang.reflect.Array.getInt(row, 0);
        System.out.println("Matrix[0][0] value: " + value);

        // Test array of different dimensions
        int[] dimensions = {2, 3, 4};
        Object multiArray = java.lang.reflect.Array.newInstance(String.class, dimensions);
        System.out.println("Multi-dimensional array type: " + multiArray.getClass().getName());

        // Test primitive array operations
        Object intArray = java.lang.reflect.Array.newInstance(int.class, 5);
        for (int i = 0; i < 5; i++) {
            java.lang.reflect.Array.setInt(intArray, i, i * 10);
        }

        for (int i = 0; i < 5; i++) {
            int val = java.lang.reflect.Array.getInt(intArray, i);
            System.out.println("Int array[" + i + "] = " + val);
        }

        // Test array field access
        Class<?> clazz = Class.forName("Test$A");
        A instance = new A();

        java.lang.reflect.Field stringArrayField = clazz.getField("stringArray");
        Object fieldArray = stringArrayField.get(instance);
        System.out.println("Field array length: " + java.lang.reflect.Array.getLength(fieldArray));

        // Test array type comparison
        Class<?> stringArrayType1 = String[].class;
        Class<?> stringArrayType2 = java.lang.reflect.Array.newInstance(String.class, 0).getClass();
        System.out.println("Array types equal: " + stringArrayType1.equals(stringArrayType2));

        // Test array assignment compatibility
        Object[] objectArray = new String[3];
        System.out.println("String array assignable to Object array: " + Object[].class.isAssignableFrom(String[].class));
        System.out.println("Object array assignable to String array: " + String[].class.isAssignableFrom(Object[].class));
    }
}
