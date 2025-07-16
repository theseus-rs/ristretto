import java.util.Random;

public class Test {
    public static void main(String[] args) {
        System.out.println("Testing nextGaussian() method");

        Random r = new Random(42L); // Fixed seed for reproducible results

        System.out.println("Generating 10 random Gaussian values:");
        for (int i = 0; i < 10; i++) {
            double value = r.nextGaussian();
            System.out.println("nextGaussian()[" + i + "] = " + value);
        }

        // Test statistical properties of Gaussian distribution
        System.out.println("\nTesting statistical properties over 10000 samples:");
        double sum = 0.0;
        double sumSquares = 0.0;
        int sampleCount = 10000;

        for (int i = 0; i < sampleCount; i++) {
            double value = r.nextGaussian();
            sum += value;
            sumSquares += value * value;
        }

        double mean = sum / sampleCount;
        double variance = (sumSquares / sampleCount) - (mean * mean);
        double stdDev = Math.sqrt(variance);

        System.out.println("Sample mean: " + mean + " (should be close to 0.0)");
        System.out.println("Sample standard deviation: " + stdDev + " (should be close to 1.0)");
        System.out.println("Sample variance: " + variance + " (should be close to 1.0)");

        // Test range distribution
        int inOneStdDev = 0;
        int inTwoStdDev = 0;
        int inThreeStdDev = 0;

        for (int i = 0; i < sampleCount; i++) {
            double value = r.nextGaussian();
            double absValue = Math.abs(value);
            if (absValue <= 1.0) inOneStdDev++;
            if (absValue <= 2.0) inTwoStdDev++;
            if (absValue <= 3.0) inThreeStdDev++;
        }

        System.out.println("Within 1 std dev: " + (inOneStdDev * 100.0 / sampleCount) + "% (expected ~68%)");
        System.out.println("Within 2 std dev: " + (inTwoStdDev * 100.0 / sampleCount) + "% (expected ~95%)");
        System.out.println("Within 3 std dev: " + (inThreeStdDev * 100.0 / sampleCount) + "% (expected ~99.7%)");

        System.out.println("Test completed successfully");
    }
}
