public class Test {
    public static void main(String[] args) {
        testIfIcmpLe();
    }

    static void testIfIcmpLe() {
        // Test if_icmple with first less than second
        int a = 5;
        int b = 10;
        if (a <= b) {
            System.out.println(true);
        } else {
            System.out.println(false);
        }

        // Test if_icmple with first greater than second
        int c = 10;
        int d = 5;
        if (c <= d) {
            System.out.println(false);
        } else {
            System.out.println(true);
        }

        // Test if_icmple with equal values
        int e = 15;
        int f = 15;
        if (e <= f) {
            System.out.println(true);
        } else {
            System.out.println(false);
        }

        // Test if_icmple with negative and positive
        int neg = -5;
        int pos = 5;
        if (neg <= pos) {
            System.out.println(true);
        } else {
            System.out.println(false);
        }

        // Test if_icmple with both negative
        int neg1 = -10;
        int neg2 = -5;
        if (neg1 <= neg2) {
            System.out.println(true);
        } else {
            System.out.println(false);
        }

        // Test if_icmple with minimum and maximum
        int min = -2147483648;
        int max = 2147483647;
        if (min <= max) {
            System.out.println(true);
        } else {
            System.out.println(false);
        }

        // Test if_icmple with zero
        int zero = 0;
        int one = 1;
        if (zero <= one) {
            System.out.println(true);
        } else {
            System.out.println(false);
        }
    }
}
