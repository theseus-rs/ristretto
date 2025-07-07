public class Test {
    public static void main(String[] args) {
        testIfNe();
    }

    static void testIfNe() {
        // Test ifne with zero
        int zero = 0;
        if (zero != 0) {
            System.out.println(false);
        } else {
            System.out.println(true);
        }

        // Test ifne with non-zero positive
        int positive = 5;
        if (positive != 0) {
            System.out.println(true);
        } else {
            System.out.println(false);
        }

        // Test ifne with negative
        int negative = -1;
        if (negative != 0) {
            System.out.println(true);
        } else {
            System.out.println(false);
        }

        // Test ifne with maximum int
        int max = 2147483647;
        if (max != 0) {
            System.out.println(true);
        } else {
            System.out.println(false);
        }

        // Test ifne with minimum int
        int min = -2147483648;
        if (min != 0) {
            System.out.println(true);
        } else {
            System.out.println(false);
        }
    }
}
