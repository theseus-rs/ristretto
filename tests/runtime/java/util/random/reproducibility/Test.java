import java.util.Random;

public class Test {
    public static void main(String[] args) {
        System.out.println("Testing Random reproducibility and determinism");

        // Test that same seed produces identical sequences across different instances
        System.out.println("Testing reproducibility across different Random instances:");
        long seed = 987654321L;
        Random r1 = new Random(seed);
        Random r2 = new Random(seed);

        boolean allMatch = true;
        for (int i = 0; i < 100; i++) {
            int val1 = r1.nextInt();
            int val2 = r2.nextInt();
            if (i < 10) {
                System.out.println("Iteration " + i + ": r1=" + val1 + ", r2=" + val2 + ", match=" + (val1 == val2));
            }
            if (val1 != val2) {
                allMatch = false;
                if (i >= 10) {
                    System.out.println("Mismatch at iteration " + i + ": r1=" + val1 + ", r2=" + val2);
                    break;
                }
            }
        }
        System.out.println("All 100 values match: " + allMatch);

        // Test reproducibility with mixed method calls
        System.out.println("\nTesting reproducibility with mixed method calls:");
        Random r3 = new Random(seed);
        Random r4 = new Random(seed);

        boolean mixedMatch = true;
        for (int i = 0; i < 10; i++) {
            // Call different methods in same order
            int int1 = r3.nextInt(1000);
            int int2 = r4.nextInt(1000);

            boolean bool1 = r3.nextBoolean();
            boolean bool2 = r4.nextBoolean();

            double dbl1 = r3.nextDouble();
            double dbl2 = r4.nextDouble();

            System.out.println("Iteration " + i + ":");
            System.out.println("  int: " + int1 + " == " + int2 + " ? " + (int1 == int2));
            System.out.println("  bool: " + bool1 + " == " + bool2 + " ? " + (bool1 == bool2));
            System.out.println("  double: " + dbl1 + " == " + dbl2 + " ? " + (dbl1 == dbl2));

            if (int1 != int2 || bool1 != bool2 || dbl1 != dbl2) {
                mixedMatch = false;
            }
        }
        System.out.println("All mixed method calls match: " + mixedMatch);

        // Test that resetting seed restores determinism
        System.out.println("\nTesting seed reset restores determinism:");
        Random r5 = new Random(seed);
        int[] originalSequence = new int[5];
        for (int i = 0; i < 5; i++) {
            originalSequence[i] = r5.nextInt(1000);
        }

        // Generate some more values to change internal state
        for (int i = 0; i < 100; i++) {
            r5.nextInt();
        }

        // Reset seed and verify we get same sequence
        r5.setSeed(seed);
        boolean resetMatch = true;
        for (int i = 0; i < 5; i++) {
            int value = r5.nextInt(1000);
            System.out.println("Reset sequence[" + i + "]: " + value + " (expected: " + originalSequence[i] + ")");
            if (value != originalSequence[i]) {
                resetMatch = false;
            }
        }
        System.out.println("Seed reset restores sequence: " + resetMatch);

        System.out.println("Test completed successfully");
    }
}
