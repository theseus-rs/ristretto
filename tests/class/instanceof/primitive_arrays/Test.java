/** Test instanceof behavior with primitive arrays */
public class Test {
    public static void main(String[] args) {
        // Test primitive arrays instanceof Object, Cloneable, Serializable
        int[] intArray = new int[5];
        System.out.println("int[] instanceof Object: " + (intArray instanceof Object));
        System.out.println("int[] instanceof Cloneable: " + (intArray instanceof Cloneable));
        System.out.println("int[] instanceof java.io.Serializable: " + (intArray instanceof java.io.Serializable));
        System.out.println("int[] instanceof int[]: " + (intArray instanceof int[]));

        // Test different primitive array types are not assignable to each other
        byte[] byteArray = new byte[5];
        System.out.println("byte[] instanceof Object: " + (byteArray instanceof Object));
        System.out.println("byte[] instanceof Cloneable: " + (byteArray instanceof Cloneable));
        System.out.println("byte[] instanceof java.io.Serializable: " + (byteArray instanceof java.io.Serializable));
        System.out.println("byte[] instanceof byte[]: " + (byteArray instanceof byte[]));

        // Test all primitive array types
        boolean[] boolArray = new boolean[5];
        char[] charArray = new char[5];
        short[] shortArray = new short[5];
        long[] longArray = new long[5];
        float[] floatArray = new float[5];
        double[] doubleArray = new double[5];

        System.out.println("boolean[] instanceof Object: " + (boolArray instanceof Object));
        System.out.println("char[] instanceof Object: " + (charArray instanceof Object));
        System.out.println("short[] instanceof Object: " + (shortArray instanceof Object));
        System.out.println("long[] instanceof Object: " + (longArray instanceof Object));
        System.out.println("float[] instanceof Object: " + (floatArray instanceof Object));
        System.out.println("double[] instanceof Object: " + (doubleArray instanceof Object));

        // Test multidimensional primitive arrays
        int[][] intArray2D = new int[3][3];
        System.out.println("int[][] instanceof Object: " + (intArray2D instanceof Object));
        System.out.println("int[][] instanceof Cloneable: " + (intArray2D instanceof Cloneable));
        System.out.println("int[][] instanceof java.io.Serializable: " + (intArray2D instanceof java.io.Serializable));
        System.out.println("int[][] instanceof int[][]: " + (intArray2D instanceof int[][]));
        System.out.println("int[][] instanceof Object[]: " + (intArray2D instanceof Object[]));
    }
}
