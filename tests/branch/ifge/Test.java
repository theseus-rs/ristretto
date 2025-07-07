public class Test {
    public static void main(String[] args) {
        testIfGe();
    }

    static void testIfGe() {
        // Test ifge with zero
        int zero = 0;
        if (zero >= 0) {
            System.out.println(true);
        } else {
            System.out.println(false);
        }

        // Test ifge with positive
        int positive = 5;
        if (positive >= 0) {
            System.out.println(true);
        } else {
            System.out.println(false);
        }

        // Test ifge with negative
        int negative = -1;
        if (negative >= 0) {
            System.out.println(false);
        } else {
            System.out.println(true);
        }

        // Test ifge with maximum int
        int max = 2147483647;
        if (max >= 0) {
            System.out.println(true);
        } else {
            System.out.println(false);
        }

        // Test ifge with minimum int
        int min = -2147483648;
        if (min >= 0) {
            System.out.println(false);
        } else {
            System.out.println(true);
        }
    }
}
