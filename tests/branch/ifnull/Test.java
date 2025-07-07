public class Test {
    public static void main(String[] args) {
        testIfNull();
    }

    static void testIfNull() {
        // Test ifnull with null reference
        String nullStr = null;
        if (nullStr == null) {
            System.out.println(true);
        } else {
            System.out.println(false);
        }

        // Test ifnull with non-null reference
        String str = "hello";
        if (str == null) {
            System.out.println(false);
        } else {
            System.out.println(true);
        }

        // Test ifnull with empty string
        String empty = "";
        if (empty == null) {
            System.out.println(false);
        } else {
            System.out.println(true);
        }

        // Test ifnull with array
        int[] arr = new int[5];
        if (arr == null) {
            System.out.println(false);
        } else {
            System.out.println(true);
        }

        // Test ifnull with null array
        int[] nullArr = null;
        if (nullArr == null) {
            System.out.println(true);
        } else {
            System.out.println(false);
        }

        // Test ifnull with object
        Object obj = new Object();
        if (obj == null) {
            System.out.println(false);
        } else {
            System.out.println(true);
        }

        // Test ifnull with null object
        Object nullObj = null;
        if (nullObj == null) {
            System.out.println(true);
        } else {
            System.out.println(false);
        }
    }
}
