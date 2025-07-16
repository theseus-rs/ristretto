import java.util.Random;

public class Test {
    public static void main(String[] args) {
        System.out.println("Testing nextInt() method");

        Random r = new Random(42L); // Fixed seed for reproducible results

        System.out.println("Generating 10 random integers:");
        for (int i = 0; i < 10; i++) {
            int value = r.nextInt();
            System.out.println("nextInt()[" + i + "] = " + value);
        }

        // Test that values are within int range
        System.out.println("\nTesting range constraints:");
        boolean allInRange = true;
        for (int i = 0; i < 100; i++) {
            int value = r.nextInt();
            if (value < Integer.MIN_VALUE || value > Integer.MAX_VALUE) {
                allInRange = false;
                break;
            }
        }
        System.out.println("All values within int range: " + allInRange);

        System.out.println("Test completed successfully");
    }
}
