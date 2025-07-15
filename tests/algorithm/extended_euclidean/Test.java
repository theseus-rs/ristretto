public class Test {
    public static class ExtendedGCDResult {
        int gcd, x, y;

        ExtendedGCDResult(int gcd, int x, int y) {
            this.gcd = gcd;
            this.x = x;
            this.y = y;
        }
    }

    public static ExtendedGCDResult extendedGCD(int a, int b) {
        if (b == 0) {
            return new ExtendedGCDResult(a, 1, 0);
        }

        ExtendedGCDResult result = extendedGCD(b, a % b);
        int x = result.y;
        int y = result.x - (a / b) * result.y;

        return new ExtendedGCDResult(result.gcd, x, y);
    }

    public static int modularInverse(int a, int m) {
        ExtendedGCDResult result = extendedGCD(a, m);
        if (result.gcd != 1) {
            return -1; // Modular inverse doesn't exist
        }
        return (result.x % m + m) % m;
    }

    public static void main(String[] args) {
        int a = 35;
        int b = 15;

        ExtendedGCDResult result = extendedGCD(a, b);

        System.out.println("Extended Euclidean Algorithm");
        System.out.println("Numbers: " + a + " and " + b);
        System.out.println("GCD: " + result.gcd);
        System.out.println("Coefficients: x = " + result.x + ", y = " + result.y);
        System.out.println("Verification: " + a + " * " + result.x + " + " + b + " * " + result.y + " = " + (a * result.x + b * result.y));

        // Modular inverse example
        int num = 3;
        int mod = 11;
        int inverse = modularInverse(num, mod);

        System.out.println("\nModular Inverse:");
        System.out.println("Inverse of " + num + " modulo " + mod + " is: " + inverse);
        if (inverse != -1) {
            System.out.println("Verification: " + num + " * " + inverse + " ≡ " + ((num * inverse) % mod) + " (mod " + mod + ")");
        }
    }
}

