public class Test {
    public static void main(String[] args) {
        testIfIcmpGe();
    }

    static void testIfIcmpGe() {
        // Test if_icmpge with first greater than second
        int a = 10;
        int b = 5;
        if (a >= b) {
            System.out.println(true);
        } else {
            System.out.println(false);
        }

        // Test if_icmpge with first less than second
        int c = 5;
        int d = 10;
        if (c >= d) {
            System.out.println(false);
        } else {
            System.out.println(true);
        }

        // Test if_icmpge with equal values
        int e = 15;
        int f = 15;
        if (e >= f) {
            System.out.println(true);
        } else {
            System.out.println(false);
        }

        // Test if_icmpge with positive and negative
        int pos = 5;
        int neg = -5;
        if (pos >= neg) {
            System.out.println(true);
        } else {
            System.out.println(false);
        }

        // Test if_icmpge with both negative
        int neg1 = -5;
        int neg2 = -10;
        if (neg1 >= neg2) {
            System.out.println(true);
        } else {
            System.out.println(false);
        }

        // Test if_icmpge with maximum and minimum
        int max = 2147483647;
        int min = -2147483648;
        if (max >= min) {
            System.out.println(true);
        } else {
            System.out.println(false);
        }

        // Test if_icmpge with zero
        int one = 1;
        int zero = 0;
        if (one >= zero) {
            System.out.println(true);
        } else {
            System.out.println(false);
        }
    }
}
