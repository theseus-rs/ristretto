import java.util.Random;

public class Test {
    public static void main(String[] args) {
        System.out.println("Testing nextInt(int bound) method");

        Random r = new Random(42L); // Fixed seed for reproducible results

        // Test various bounds
        int[] bounds = {1, 5, 10, 100, 1000};

        for (int bound : bounds) {
            System.out.println("\nTesting bound = " + bound + ":");
            boolean allInRange = true;
            for (int i = 0; i < 20; i++) {
                int value = r.nextInt(bound);
                System.out.println("nextInt(" + bound + ")[" + i + "] = " + value);
                if (value < 0 || value >= bound) {
                    allInRange = false;
                    System.out.println("ERROR: Value " + value + " is out of range [0, " + bound + ")");
                }
            }
            System.out.println("All values in range [0, " + bound + "): " + allInRange);
        }

        // Test edge case with bound = 1
        System.out.println("\nTesting edge case with bound = 1:");
        for (int i = 0; i < 5; i++) {
            int value = r.nextInt(1);
            System.out.println("nextInt(1)[" + i + "] = " + value + " (should always be 0)");
        }

        System.out.println("Test completed successfully");
    }
}
