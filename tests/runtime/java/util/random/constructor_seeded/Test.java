import java.util.Random;

public class Test {
    public static void main(String[] args) {
        System.out.println("Testing Random(long seed) constructor");

        long seed = 12345L;
        Random r1 = new Random(seed);
        Random r2 = new Random(seed);

        // Same seed should produce identical sequences
        System.out.println("Testing identical sequences with same seed:");
        for (int i = 0; i < 10; i++) {
            int val1 = r1.nextInt();
            int val2 = r2.nextInt();
            System.out.println("Iteration " + i + ": r1=" + val1 + ", r2=" + val2 + ", equal=" + (val1 == val2));
        }

        // Different seeds should produce different sequences
        Random r3 = new Random(54321L);
        r1 = new Random(seed); // Reset r1

        System.out.println("\nTesting different sequences with different seeds:");
        for (int i = 0; i < 5; i++) {
            int val1 = r1.nextInt();
            int val3 = r3.nextInt();
            System.out.println("Iteration " + i + ": r1=" + val1 + ", r3=" + val3 + ", different=" + (val1 != val3));
        }

        System.out.println("Test completed successfully");
    }
}
