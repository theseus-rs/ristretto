import java.util.Random;

public class Test {
    public static void main(String[] args) {
        System.out.println("Testing Random statistical distribution properties");

        Random r = new Random(42L);
        int sampleSize = 100000;

        // Test uniform distribution of nextInt(bound)
        System.out.println("Testing uniform distribution of nextInt(10) over " + sampleSize + " samples:");
        int[] buckets = new int[10];
        for (int i = 0; i < sampleSize; i++) {
            int value = r.nextInt(10);
            buckets[value]++;
        }

        double expectedCount = sampleSize / 10.0;
        System.out.println("Expected count per bucket: " + expectedCount);
        for (int i = 0; i < 10; i++) {
            double percentage = (buckets[i] * 100.0) / sampleSize;
            double deviation = Math.abs(buckets[i] - expectedCount) / expectedCount * 100;
            System.out.println("Bucket " + i + ": " + buckets[i] + " (" + String.format("%.2f", percentage) + "%, deviation: " + String.format("%.2f", deviation) + "%)");
        }

        // Test boolean distribution
        System.out.println("\nTesting boolean distribution over " + sampleSize + " samples:");
        int trueCount = 0;
        for (int i = 0; i < sampleSize; i++) {
            if (r.nextBoolean()) {
                trueCount++;
            }
        }
        int falseCount = sampleSize - trueCount;
        double truePercentage = (trueCount * 100.0) / sampleSize;
        double falsePercentage = (falseCount * 100.0) / sampleSize;

        System.out.println("True: " + trueCount + " (" + String.format("%.2f", truePercentage) + "%)");
        System.out.println("False: " + falseCount + " (" + String.format("%.2f", falsePercentage) + "%)");
        System.out.println("Deviation from 50%: " + String.format("%.2f", Math.abs(truePercentage - 50.0)) + "%");

        // Test float distribution in ranges
        System.out.println("\nTesting float distribution in quarters over " + (sampleSize/10) + " samples:");
        int[] floatQuarters = new int[4];
        for (int i = 0; i < sampleSize/10; i++) {
            float value = r.nextFloat();
            if (value < 0.25f) floatQuarters[0]++;
            else if (value < 0.5f) floatQuarters[1]++;
            else if (value < 0.75f) floatQuarters[2]++;
            else floatQuarters[3]++;
        }

        for (int i = 0; i < 4; i++) {
            double percentage = (floatQuarters[i] * 100.0) / (sampleSize/10);
            System.out.println("Quarter " + i + " [" + (i*0.25) + "-" + ((i+1)*0.25) + "): " + floatQuarters[i] + " (" + String.format("%.2f", percentage) + "%)");
        }

        // Test double precision
        System.out.println("\nTesting double precision - checking for duplicate values:");
        java.util.Set<Double> doubleSet = new java.util.HashSet<>();
        int duplicates = 0;
        for (int i = 0; i < 10000; i++) {
            double value = r.nextDouble();
            if (!doubleSet.add(value)) {
                duplicates++;
            }
        }
        System.out.println("Duplicate doubles in 10000 samples: " + duplicates);
        System.out.println("Unique doubles: " + doubleSet.size());

        System.out.println("Test completed successfully");
    }
}
