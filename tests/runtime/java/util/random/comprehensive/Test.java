import java.util.Random;

public class Test {
    public static void main(String[] args) {
        System.out.println("Testing Random comprehensive behavior and integration");

        // Test complete workflow with all methods
        System.out.println("Testing complete Random workflow:");
        Random r = new Random(123456789L);

        System.out.println("1. Constructor with seed 123456789");
        System.out.println("2. Testing all methods in sequence:");

        // Test each method type
        int intValue = r.nextInt();
        System.out.println("   nextInt() = " + intValue);

        int boundedInt = r.nextInt(100);
        System.out.println("   nextInt(100) = " + boundedInt);

        long longValue = r.nextLong();
        System.out.println("   nextLong() = " + longValue);

        boolean boolValue = r.nextBoolean();
        System.out.println("   nextBoolean() = " + boolValue);

        float floatValue = r.nextFloat();
        System.out.println("   nextFloat() = " + floatValue);

        double doubleValue = r.nextDouble();
        System.out.println("   nextDouble() = " + doubleValue);

        double gaussianValue = r.nextGaussian();
        System.out.println("   nextGaussian() = " + gaussianValue);

        byte[] bytes = new byte[5];
        r.nextBytes(bytes);
        System.out.print("   nextBytes(5) = [");
        for (int i = 0; i < bytes.length; i++) {
            System.out.print(bytes[i]);
            if (i < bytes.length - 1) System.out.print(", ");
        }
        System.out.println("]");

        // Test state preservation through multiple operations
        System.out.println("\n3. Testing state preservation through mixed operations:");
        Random r1 = new Random(999L);
        Random r2 = new Random(999L);

        // Perform identical mixed operations on both
        boolean stateMatches = true;
        for (int i = 0; i < 5; i++) {
            int op = i % 4;
            switch (op) {
                case 0:
                    int i1 = r1.nextInt(50);
                    int i2 = r2.nextInt(50);
                    System.out.println("   Op " + i + " nextInt(50): " + i1 + " == " + i2 + " ? " + (i1 == i2));
                    if (i1 != i2) stateMatches = false;
                    break;
                case 1:
                    boolean b1 = r1.nextBoolean();
                    boolean b2 = r2.nextBoolean();
                    System.out.println("   Op " + i + " nextBoolean(): " + b1 + " == " + b2 + " ? " + (b1 == b2));
                    if (b1 != b2) stateMatches = false;
                    break;
                case 2:
                    float f1 = r1.nextFloat();
                    float f2 = r2.nextFloat();
                    System.out.println("   Op " + i + " nextFloat(): " + f1 + " == " + f2 + " ? " + (f1 == f2));
                    if (f1 != f2) stateMatches = false;
                    break;
                case 3:
                    double d1 = r1.nextDouble();
                    double d2 = r2.nextDouble();
                    System.out.println("   Op " + i + " nextDouble(): " + d1 + " == " + d2 + " ? " + (d1 == d2));
                    if (d1 != d2) stateMatches = false;
                    break;
            }
        }
        System.out.println("State matches through mixed operations: " + stateMatches);

        // Test setSeed behavior in complex scenario
        System.out.println("\n4. Testing setSeed in complex scenario:");
        Random r3 = new Random();

        // Generate some values
        r3.nextInt();
        r3.nextDouble();
        r3.nextBoolean();

        // Set known seed and capture sequence
        r3.setSeed(777L);
        int[] sequence1 = new int[3];
        for (int i = 0; i < 3; i++) {
            sequence1[i] = r3.nextInt(1000);
        }

        // Do more operations to change state
        for (int i = 0; i < 50; i++) {
            r3.nextLong();
            r3.nextGaussian();
        }

        // Reset to same seed and verify sequence restoration
        r3.setSeed(777L);
        boolean sequenceRestored = true;
        for (int i = 0; i < 3; i++) {
            int value = r3.nextInt(1000);
            System.out.println("   Sequence restore[" + i + "]: " + value + " (expected: " + sequence1[i] + ")");
            if (value != sequence1[i]) {
                sequenceRestored = false;
            }
        }
        System.out.println("Sequence restored after complex operations: " + sequenceRestored);

        // Final validation - ensure Random is still functional
        System.out.println("\n5. Final validation - Random still functional:");
        boolean stillFunctional = true;
        try {
            r3.nextInt();
            r3.nextInt(10);
            r3.nextLong();
            r3.nextBoolean();
            r3.nextFloat();
            r3.nextDouble();
            r3.nextGaussian();
            r3.nextBytes(new byte[1]);
            System.out.println("   All methods executed without exception");
        } catch (Exception e) {
            stillFunctional = false;
            System.out.println("   ERROR: Exception during final validation: " + e);
        }
        System.out.println("Random instance still functional: " + stillFunctional);

        System.out.println("Comprehensive test completed successfully");
    }
}
