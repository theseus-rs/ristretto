import java.util.Random;

public class Test {
    public static void main(String[] args) {
        System.out.println("Testing nextDouble() method");

        Random r = new Random(42L); // Fixed seed for reproducible results

        System.out.println("Generating 10 random doubles:");
        for (int i = 0; i < 10; i++) {
            double value = r.nextDouble();
            System.out.println("nextDouble()[" + i + "] = " + value);
        }

        // Test that all values are in range [0.0, 1.0)
        System.out.println("\nTesting range constraints [0.0, 1.0):");
        boolean allInRange = true;
        double minValue = Double.MAX_VALUE;
        double maxValue = Double.MIN_VALUE;

        for (int i = 0; i < 10000; i++) {
            double value = r.nextDouble();
            if (value < 0.0 || value >= 1.0) {
                allInRange = false;
                System.out.println("ERROR: Value " + value + " is out of range [0.0, 1.0)");
            }
            if (value < minValue) minValue = value;
            if (value > maxValue) maxValue = value;
        }

        System.out.println("All values in range [0.0, 1.0): " + allInRange);
        System.out.println("Min value observed: " + minValue);
        System.out.println("Max value observed: " + maxValue);

        // Test precision
        System.out.println("\nTesting precision with fixed seed:");
        r.setSeed(12345L);
        for (int i = 0; i < 5; i++) {
            double value = r.nextDouble();
            System.out.println("Precise double[" + i + "] = " + String.format("%.15f", value));
        }

        System.out.println("Test completed successfully");
    }
}
