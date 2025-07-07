public class Test {
    public static void main(String[] args) {
        testIfIcmpLt();
    }

    static void testIfIcmpLt() {
        // Test if_icmplt with first less than second
        int a = 5;
        int b = 10;
        if (a < b) {
            System.out.println(true);
        } else {
            System.out.println(false);
        }

        // Test if_icmplt with first greater than second
        int c = 20;
        int d = 10;
        if (c < d) {
            System.out.println(false);
        } else {
            System.out.println(true);
        }

        // Test if_icmplt with equal values
        int e = 15;
        int f = 15;
        if (e < f) {
            System.out.println(false);
        } else {
            System.out.println(true);
        }

        // Test if_icmplt with negative and positive
        int neg = -5;
        int pos = 5;
        if (neg < pos) {
            System.out.println(true);
        } else {
            System.out.println(false);
        }

        // Test if_icmplt with both negative
        int neg1 = -10;
        int neg2 = -5;
        if (neg1 < neg2) {
            System.out.println(true);
        } else {
            System.out.println(false);
        }

        // Test if_icmplt with minimum and maximum
        int min = -2147483648;
        int max = 2147483647;
        if (min < max) {
            System.out.println(true);
        } else {
            System.out.println(false);
        }

        // Test if_icmplt with zero
        int zero = 0;
        int one = 1;
        if (zero < one) {
            System.out.println(true);
        } else {
            System.out.println(false);
        }
    }
}
