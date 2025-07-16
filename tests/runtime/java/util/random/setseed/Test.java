import java.util.Random;

public class Test {
    public static void main(String[] args) {
        System.out.println("Testing setSeed(long seed) method");

        Random r = new Random(42L);

        // Generate some initial values
        System.out.println("Initial sequence with seed 42:");
        int[] initialValues = new int[5];
        for (int i = 0; i < 5; i++) {
            initialValues[i] = r.nextInt(100);
            System.out.println("Value[" + i + "] = " + initialValues[i]);
        }

        // Reset with same seed and verify same sequence
        System.out.println("\nResetting to seed 42 and generating same sequence:");
        r.setSeed(42L);
        boolean sequenceMatches = true;
        for (int i = 0; i < 5; i++) {
            int value = r.nextInt(100);
            System.out.println("Value[" + i + "] = " + value + " (expected: " + initialValues[i] + ")");
            if (value != initialValues[i]) {
                sequenceMatches = false;
            }
        }
        System.out.println("Sequence matches after setSeed: " + sequenceMatches);

        // Test with different seed
        System.out.println("\nSetting different seed (12345) and generating new sequence:");
        r.setSeed(12345L);
        for (int i = 0; i < 5; i++) {
            int value = r.nextInt(100);
            System.out.println("Value[" + i + "] = " + value);
        }

        // Test setSeed with negative value
        System.out.println("\nTesting setSeed with negative value (-999):");
        r.setSeed(-999L);
        for (int i = 0; i < 3; i++) {
            int value = r.nextInt(100);
            System.out.println("Value[" + i + "] = " + value);
        }

        // Test setSeed with zero
        System.out.println("\nTesting setSeed with zero:");
        r.setSeed(0L);
        for (int i = 0; i < 3; i++) {
            int value = r.nextInt(100);
            System.out.println("Value[" + i + "] = " + value);
        }

        // Test setSeed with Long.MAX_VALUE
        System.out.println("\nTesting setSeed with Long.MAX_VALUE:");
        r.setSeed(Long.MAX_VALUE);
        for (int i = 0; i < 3; i++) {
            int value = r.nextInt(100);
            System.out.println("Value[" + i + "] = " + value);
        }

        System.out.println("Test completed successfully");
    }
}
