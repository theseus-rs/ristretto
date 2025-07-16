import java.util.Random;

public class Test {
    public static void main(String[] args) {
        System.out.println("Testing nextBytes(byte[]) method");

        Random r = new Random(42L); // Fixed seed for reproducible results

        // Test with different array sizes
        int[] sizes = {1, 5, 16, 32, 100};

        for (int size : sizes) {
            System.out.println("\nTesting with array size " + size + ":");
            byte[] bytes = new byte[size];
            r.nextBytes(bytes);

            System.out.print("Bytes: ");
            for (int i = 0; i < Math.min(size, 10); i++) { // Print first 10 bytes
                System.out.print(bytes[i] + " ");
            }
            if (size > 10) {
                System.out.print("...");
            }
            System.out.println();

            // Check that bytes are filled (not all zeros)
            boolean hasNonZero = false;
            for (byte b : bytes) {
                if (b != 0) {
                    hasNonZero = true;
                    break;
                }
            }
            System.out.println("Array contains non-zero values: " + hasNonZero);
        }

        // Test with empty array
        System.out.println("\nTesting with empty array:");
        byte[] emptyArray = new byte[0];
        r.nextBytes(emptyArray);
        System.out.println("Empty array length after nextBytes: " + emptyArray.length);

        // Test reproducibility with same seed
        System.out.println("\nTesting reproducibility:");
        Random r1 = new Random(12345L);
        Random r2 = new Random(12345L);

        byte[] bytes1 = new byte[10];
        byte[] bytes2 = new byte[10];

        r1.nextBytes(bytes1);
        r2.nextBytes(bytes2);

        boolean identical = true;
        for (int i = 0; i < 10; i++) {
            if (bytes1[i] != bytes2[i]) {
                identical = false;
                break;
            }
        }
        System.out.println("Arrays from same seed are identical: " + identical);

        System.out.println("Test completed successfully");
    }
}
