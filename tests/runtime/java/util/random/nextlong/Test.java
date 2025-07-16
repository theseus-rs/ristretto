import java.util.Random;

public class Test {
    public static void main(String[] args) {
        System.out.println("Testing nextLong() method");

        Random r = new Random(42L); // Fixed seed for reproducible results

        System.out.println("Generating 10 random longs:");
        for (int i = 0; i < 10; i++) {
            long value = r.nextLong();
            System.out.println("nextLong()[" + i + "] = " + value);
        }

        // Test that values are within long range
        System.out.println("\nTesting range constraints:");
        boolean allInRange = true;
        for (int i = 0; i < 100; i++) {
            long value = r.nextLong();
            if (value < Long.MIN_VALUE || value > Long.MAX_VALUE) {
                allInRange = false;
                break;
            }
        }
        System.out.println("All values within long range: " + allInRange);

        // Test distribution of positive vs negative values
        int positiveCount = 0;
        int negativeCount = 0;
        for (int i = 0; i < 1000; i++) {
            long value = r.nextLong();
            if (value >= 0) {
                positiveCount++;
            } else {
                negativeCount++;
            }
        }
        System.out.println("Positive values: " + positiveCount + ", Negative values: " + negativeCount);

        System.out.println("Test completed successfully");
    }
}
