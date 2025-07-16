import java.util.Random;

public class Test {
    public static void main(String[] args) {
        System.out.println("Testing Random edge cases and error conditions");

        Random r = new Random(42L);

        // Test nextInt with invalid bounds
        System.out.println("Testing nextInt with invalid bounds:");

        try {
            int value = r.nextInt(0);
            System.out.println("ERROR: nextInt(0) should throw IllegalArgumentException but returned: " + value);
        } catch (IllegalArgumentException e) {
            System.out.println("SUCCESS: nextInt(0) threw IllegalArgumentException: " + e.getMessage());
        } catch (Exception e) {
            System.out.println("ERROR: nextInt(0) threw unexpected exception: " + e.getClass().getName() + " - " + e.getMessage());
        }

        try {
            int value = r.nextInt(-1);
            System.out.println("ERROR: nextInt(-1) should throw IllegalArgumentException but returned: " + value);
        } catch (IllegalArgumentException e) {
            System.out.println("SUCCESS: nextInt(-1) threw IllegalArgumentException: " + e.getMessage());
        } catch (Exception e) {
            System.out.println("ERROR: nextInt(-1) threw unexpected exception: " + e.getClass().getName() + " - " + e.getMessage());
        }

        try {
            int value = r.nextInt(-100);
            System.out.println("ERROR: nextInt(-100) should throw IllegalArgumentException but returned: " + value);
        } catch (IllegalArgumentException e) {
            System.out.println("SUCCESS: nextInt(-100) threw IllegalArgumentException: " + e.getMessage());
        } catch (Exception e) {
            System.out.println("ERROR: nextInt(-100) threw unexpected exception: " + e.getClass().getName() + " - " + e.getMessage());
        }

        // Test nextBytes with null array
        System.out.println("\nTesting nextBytes with null array:");
        try {
            r.nextBytes(null);
            System.out.println("ERROR: nextBytes(null) should throw NullPointerException");
        } catch (NullPointerException e) {
            // TODO: The message in ristretto varies from other implementations, so we can't check it.
            // System.out.println("SUCCESS: nextBytes(null) threw NullPointerException: " + e.getMessage());
            System.out.println("SUCCESS: nextBytes(null) threw NullPointerException");
        } catch (Exception e) {
            System.out.println("ERROR: nextBytes(null) threw unexpected exception: " + e.getClass().getName() + " - " + e.getMessage());
        }

        // Test edge values for nextInt with maximum bound
        System.out.println("\nTesting nextInt with Integer.MAX_VALUE bound:");
        try {
            int value = r.nextInt(Integer.MAX_VALUE);
            System.out.println("nextInt(Integer.MAX_VALUE) = " + value);
            System.out.println("Value is non-negative: " + (value >= 0));
            System.out.println("Value is less than bound: " + (value < Integer.MAX_VALUE));
        } catch (Exception e) {
            System.out.println("ERROR: nextInt(Integer.MAX_VALUE) threw exception: " + e.getClass().getName() + " - " + e.getMessage());
        }

        System.out.println("Test completed successfully");
    }
}
