/** Test instanceof behavior with reference arrays and covariance */
public class Test {
    public static void main(String[] args) {
        // Test reference arrays instanceof Object, Cloneable, Serializable
        String[] stringArray = new String[5];
        System.out.println("String[] instanceof Object: " + (stringArray instanceof Object));
        System.out.println("String[] instanceof Cloneable: " + (stringArray instanceof Cloneable));
        System.out.println("String[] instanceof java.io.Serializable: " + (stringArray instanceof java.io.Serializable));
        System.out.println("String[] instanceof String[]: " + (stringArray instanceof String[]));

        // Test reference array covariance - String[] is assignable to Object[]
        System.out.println("String[] instanceof Object[]: " + (stringArray instanceof Object[]));
        System.out.println("String[] instanceof CharSequence[]: " + (stringArray instanceof CharSequence[]));
        System.out.println("String[] instanceof Comparable[]: " + (stringArray instanceof Comparable[]));
        System.out.println("String[] instanceof java.io.Serializable[]: " + (stringArray instanceof java.io.Serializable[]));

        // Test Object[] with various reference types
        Object[] objectArray = new Object[5];
        System.out.println("Object[] instanceof Object: " + (objectArray instanceof Object));
        System.out.println("Object[] instanceof Object[]: " + (objectArray instanceof Object[]));
        System.out.println("Object[] instanceof String[]: " + (objectArray instanceof String[]));
        System.out.println("Object[] instanceof Cloneable[]: " + (objectArray instanceof Cloneable[]));

        // Test multidimensional reference arrays
        String[][] stringArray2D = new String[3][3];
        System.out.println("String[][] instanceof Object: " + (stringArray2D instanceof Object));
        System.out.println("String[][] instanceof Object[]: " + (stringArray2D instanceof Object[]));
        System.out.println("String[][] instanceof Object[][]: " + (stringArray2D instanceof Object[][]));
        System.out.println("String[][] instanceof String[][]: " + (stringArray2D instanceof String[][]));
        System.out.println("String[][] instanceof CharSequence[][]: " + (stringArray2D instanceof CharSequence[][]));

        // Test arrays of different reference types
        Integer[] integerArray = new Integer[5];
        System.out.println("Integer[] instanceof Object[]: " + (integerArray instanceof Object[]));
        System.out.println("Integer[] instanceof Number[]: " + (integerArray instanceof Number[]));
        System.out.println("Integer[] instanceof Comparable[]: " + (integerArray instanceof Comparable[]));

        // Test null arrays
        String[] nullStringArray = null;
        System.out.println("null String[] instanceof Object: " + (nullStringArray instanceof Object));
        System.out.println("null String[] instanceof String[]: " + (nullStringArray instanceof String[]));
        System.out.println("null String[] instanceof Object[]: " + (nullStringArray instanceof Object[]));
    }
}
