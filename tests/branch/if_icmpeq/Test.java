public class Test {
    public static void main(String[] args) {
        testIfIcmpEq();
    }

    static void testIfIcmpEq() {
        // Test if_icmpeq with equal values
        int a = 5;
        int b = 5;
        if (a == b) {
            System.out.println(true);
        } else {
            System.out.println(false);
        }

        // Test if_icmpeq with different values
        int c = 10;
        int d = 20;
        if (c == d) {
            System.out.println(false);
        } else {
            System.out.println(true);
        }

        // Test if_icmpeq with zero values
        int zero1 = 0;
        int zero2 = 0;
        if (zero1 == zero2) {
            System.out.println(true);
        } else {
            System.out.println(false);
        }

        // Test if_icmpeq with negative values
        int neg1 = -5;
        int neg2 = -5;
        if (neg1 == neg2) {
            System.out.println(true);
        } else {
            System.out.println(false);
        }

        // Test if_icmpeq with maximum values
        int max1 = 2147483647;
        int max2 = 2147483647;
        if (max1 == max2) {
            System.out.println(true);
        } else {
            System.out.println(false);
        }

        // Test if_icmpeq with minimum values
        int min1 = -2147483648;
        int min2 = -2147483648;
        if (min1 == min2) {
            System.out.println(true);
        } else {
            System.out.println(false);
        }

        // Test if_icmpeq with different signs
        int pos = 5;
        int neg = -5;
        if (pos == neg) {
            System.out.println(false);
        } else {
            System.out.println(true);
        }
    }
}
