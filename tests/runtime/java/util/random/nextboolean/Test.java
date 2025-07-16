import java.util.Random;

public class Test {
    public static void main(String[] args) {
        System.out.println("Testing nextBoolean() method");

        Random r = new Random(42L); // Fixed seed for reproducible results

        System.out.println("Generating 20 random booleans:");
        int trueCount = 0;
        int falseCount = 0;

        for (int i = 0; i < 20; i++) {
            boolean value = r.nextBoolean();
            System.out.println("nextBoolean()[" + i + "] = " + value);
            if (value) {
                trueCount++;
            } else {
                falseCount++;
            }
        }

        System.out.println("True count: " + trueCount + ", False count: " + falseCount);

        // Test distribution over larger sample
        trueCount = 0;
        falseCount = 0;
        for (int i = 0; i < 10000; i++) {
            if (r.nextBoolean()) {
                trueCount++;
            } else {
                falseCount++;
            }
        }

        System.out.println("Distribution over 10000 samples:");
        System.out.println("True: " + trueCount + " (" + (trueCount * 100.0 / 10000) + "%)");
        System.out.println("False: " + falseCount + " (" + (falseCount * 100.0 / 10000) + "%)");

        System.out.println("Test completed successfully");
    }
}
