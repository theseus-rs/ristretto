public class Test {
    public static void main(String[] args) {
        testIfAcmpEq();
    }

    static void testIfAcmpEq() {
        // Test if_acmpeq with same object reference
        String str1 = "hello";
        String str2 = str1;
        if (str1 == str2) {
            System.out.println(true);
        } else {
            System.out.println(false);
        }

        // Test if_acmpeq with different object references
        String str3 = new String("hello");
        String str4 = new String("hello");
        if (str3 == str4) {
            System.out.println(false);
        } else {
            System.out.println(true);
        }

        // Test if_acmpeq with null references
        String nullStr1 = null;
        String nullStr2 = null;
        if (nullStr1 == nullStr2) {
            System.out.println(true);
        } else {
            System.out.println(false);
        }

        // Test if_acmpeq with null and non-null
        String str5 = "test";
        String nullStr3 = null;
        if (str5 == nullStr3) {
            System.out.println(false);
        } else {
            System.out.println(true);
        }

        // Test if_acmpeq with array references
        int[] arr1 = new int[]{1, 2, 3};
        int[] arr2 = arr1;
        if (arr1 == arr2) {
            System.out.println(true);
        } else {
            System.out.println(false);
        }

        // Test if_acmpeq with different arrays
        int[] arr3 = new int[]{1, 2, 3};
        int[] arr4 = new int[]{1, 2, 3};
        if (arr3 == arr4) {
            System.out.println(false);
        } else {
            System.out.println(true);
        }
    }
}
