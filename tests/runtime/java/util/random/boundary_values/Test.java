import java.util.Random;

public class Test {
    public static void main(String[] args) {
        System.out.println("Testing Random boundary values and extreme cases");

        Random r = new Random(42L);

        // Test nextInt with boundary values
        System.out.println("Testing nextInt with various boundary values:");

        // Test with 1 (should always return 0)
        System.out.println("nextInt(1) - should always return 0:");
        for (int i = 0; i < 5; i++) {
            int value = r.nextInt(1);
            System.out.println("  nextInt(1)[" + i + "] = " + value);
        }

        // Test with 2 (should return 0 or 1)
        System.out.println("\nnextInt(2) - should return 0 or 1:");
        for (int i = 0; i < 10; i++) {
            int value = r.nextInt(2);
            System.out.println("  nextInt(2)[" + i + "] = " + value);
        }

        // Test with large values
        System.out.println("\nTesting with large bounds:");
        int[] largeBounds = {1000000, 10000000, 100000000, Integer.MAX_VALUE};
        for (int bound : largeBounds) {
            int value = r.nextInt(bound);
            System.out.println("nextInt(" + bound + ") = " + value + " (in range: " + (value >= 0 && value < bound) + ")");
        }

        // Test extreme long values
        System.out.println("\nTesting extreme long values:");
        long[] extremeLongs = new long[5];
        for (int i = 0; i < 5; i++) {
            extremeLongs[i] = r.nextLong();
        }

        System.out.println("Sample of extreme long values:");
        for (int i = 0; i < 5; i++) {
            System.out.println("nextLong()[" + i + "] = " + extremeLongs[i]);
        }

        // Test byte array boundary cases
        System.out.println("\nTesting byte arrays with boundary sizes:");

        // Very large array
        byte[] largeArray = new byte[10000];
        r.nextBytes(largeArray);
        boolean hasVariation = false;
        byte firstByte = largeArray[0];
        for (int i = 1; i < largeArray.length && !hasVariation; i++) {
            if (largeArray[i] != firstByte) {
                hasVariation = true;
            }
        }
        System.out.println("Large array (10000 bytes) has variation: " + hasVariation);

        // Check distribution in large array
        int[] byteDistribution = new int[256];
        for (byte b : largeArray) {
            byteDistribution[b & 0xFF]++;
        }

        int nonZeroBuckets = 0;
        for (int count : byteDistribution) {
            if (count > 0) nonZeroBuckets++;
        }
        System.out.println("Non-zero buckets in byte distribution: " + nonZeroBuckets + "/256");

        // Test float/double edge cases
        System.out.println("\nTesting float/double edge cases:");
        float minFloat = Float.MAX_VALUE;
        float maxFloat = Float.MIN_VALUE;

        for (int i = 0; i < 10000; i++) {
            float f = r.nextFloat();
            if (f < minFloat) minFloat = f;
            if (f > maxFloat) maxFloat = f;

            // Verify range
            if (f < 0.0f || f >= 1.0f) {
                System.out.println("ERROR: float out of range: " + f);
            }
        }

        System.out.println("Float range observed: [" + minFloat + ", " + maxFloat + "]");
        System.out.println("Min is very small (< 0.001): " + (minFloat < 0.001f));
        System.out.println("Max is close to 1.0 (> 0.999): " + (maxFloat > 0.999f));

        System.out.println("Test completed successfully");
    }
}
