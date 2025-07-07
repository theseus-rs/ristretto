public class Test {
    public static void main(String[] args) {
        testIfIcmpGt();
    }

    static void testIfIcmpGt() {
        // Test if_icmpgt with first greater than second
        int a = 10;
        int b = 5;
        if (a > b) {
            System.out.println(true);
        } else {
            System.out.println(false);
        }

        // Test if_icmpgt with first less than second
        int c = 5;
        int d = 10;
        if (c > d) {
            System.out.println(false);
        } else {
            System.out.println(true);
        }

        // Test if_icmpgt with equal values
        int e = 15;
        int f = 15;
        if (e > f) {
            System.out.println(false);
        } else {
            System.out.println(true);
        }

        // Test if_icmpgt with positive and negative
        int pos = 5;
        int neg = -5;
        if (pos > neg) {
            System.out.println(true);
        } else {
            System.out.println(false);
        }

        // Test if_icmpgt with both negative
        int neg1 = -5;
        int neg2 = -10;
        if (neg1 > neg2) {
            System.out.println(true);
        } else {
            System.out.println(false);
        }

        // Test if_icmpgt with maximum and minimum
        int max = 2147483647;
        int min = -2147483648;
        if (max > min) {
            System.out.println(true);
        } else {
            System.out.println(false);
        }

        // Test if_icmpgt with zero
        int one = 1;
        int zero = 0;
        if (one > zero) {
            System.out.println(true);
        } else {
            System.out.println(false);
        }

        // Additional test cases
        // Test if_icmpgt with first value as zero
        int g = 0;
        int h = 5;
        if (g > h) {
            System.out.println(false);
        } else {
            System.out.println(true);
        }

        // Test if_icmpgt with second value as zero
        int i = 5;
        int j = 0;
        if (i > j) {
            System.out.println(true);
        } else {
            System.out.println(false);
        }

        // Test if_icmpgt with both values as zero
        int k = 0;
        int l = 0;
        if (k > l) {
            System.out.println(false);
        } else {
            System.out.println(true);
        }

        // Test if_icmpgt with large positive and negative values
        int largePos = 100000;
        int largeNeg = -100000;
        if (largePos > largeNeg) {
            System.out.println(true);
        } else {
            System.out.println(false);
        }

        // Test if_icmpgt with small positive and negative values
        int smallPos = 1;
        int smallNeg = -1;
        if (smallPos > smallNeg) {
            System.out.println(true);
        } else {
            System.out.println(false);
        }
    }
}
