public class Test {
    public static void main(String[] args) {
        testIfIcmpNe();
    }

    static void testIfIcmpNe() {
        // Test if_icmpne with equal values
        int a = 5;
        int b = 5;
        if (a != b) {
            System.out.println(false);
        } else {
            System.out.println(true);
        }

        // Test if_icmpne with different values
        int c = 10;
        int d = 20;
        if (c != d) {
            System.out.println(true);
        } else {
            System.out.println(false);
        }

        // Test if_icmpne with zero and non-zero
        int zero = 0;
        int one = 1;
        if (zero != one) {
            System.out.println(true);
        } else {
            System.out.println(false);
        }

        // Test if_icmpne with negative values
        int neg1 = -5;
        int neg2 = -10;
        if (neg1 != neg2) {
            System.out.println(true);
        } else {
            System.out.println(false);
        }

        // Test if_icmpne with maximum and minimum
        int max = 2147483647;
        int min = -2147483648;
        if (max != min) {
            System.out.println(true);
        } else {
            System.out.println(false);
        }

        // Test if_icmpne with same negative values
        int negA = -15;
        int negB = -15;
        if (negA != negB) {
            System.out.println(false);
        } else {
            System.out.println(true);
        }
    }
}
