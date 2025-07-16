import java.util.Random;

public class Test {
    private static final Random random = new Random(42L); // Fixed seed for reproducibility

    public static boolean millerRabinTest(long n, int k) {
        if (n < 2) return false;
        if (n == 2 || n == 3) return true;
        if (n % 2 == 0) return false;

        // Write n-1 as d * 2^r
        long d = n - 1;
        int r = 0;
        while (d % 2 == 0) {
            d /= 2;
            r++;
        }

        // Perform k rounds of testing
        for (int i = 0; i < k; i++) {
            long a = 2 + random.nextLong() % (n - 3);
            long x = modularExponentiation(a, d, n);

            if (x == 1 || x == n - 1) continue;

            boolean composite = true;
            for (int j = 0; j < r - 1; j++) {
                x = modularExponentiation(x, 2, n);
                if (x == n - 1) {
                    composite = false;
                    break;
                }
            }

            if (composite) return false;
        }

        return true;
    }

    private static long modularExponentiation(long base, long exponent, long modulus) {
        long result = 1;
        base = base % modulus;

        while (exponent > 0) {
            if (exponent % 2 == 1) {
                result = (result * base) % modulus;
            }
            exponent = exponent >> 1;
            base = (base * base) % modulus;
        }

        return result;
    }

    public static void main(String[] args) {
        long[] testNumbers = {97, 100, 101, 561, 1009, 1013, 2047, 3571};
        int k = 5; // Number of rounds

        System.out.println("Miller-Rabin Primality Test Results:");
        for (long n : testNumbers) {
            boolean isPrime = millerRabinTest(n, k);
            System.out.println(n + " is " + (isPrime ? "probably prime" : "composite"));
        }
    }
}
