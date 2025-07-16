import java.util.Random;

public class Test {
    public static void main(String[] args) {
        System.out.println("Testing nextFloat() method");

        Random r = new Random(42L); // Fixed seed for reproducible results

        System.out.println("Generating 10 random floats:");
        for (int i = 0; i < 10; i++) {
            float value = r.nextFloat();
            System.out.println("nextFloat()[" + i + "] = " + value);
        }

        // Test that all values are in range [0.0, 1.0)
        System.out.println("\nTesting range constraints [0.0, 1.0):");
        boolean allInRange = true;
        float minValue = Float.MAX_VALUE;
        float maxValue = Float.MIN_VALUE;

        for (int i = 0; i < 10000; i++) {
            float value = r.nextFloat();
            if (value < 0.0f || value >= 1.0f) {
                allInRange = false;
                System.out.println("ERROR: Value " + value + " is out of range [0.0, 1.0)");
            }
            if (value < minValue) minValue = value;
            if (value > maxValue) maxValue = value;
        }

        System.out.println("All values in range [0.0, 1.0): " + allInRange);
        System.out.println("Min value observed: " + minValue);
        System.out.println("Max value observed: " + maxValue);

        System.out.println("Test completed successfully");
    }
}
