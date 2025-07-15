public class Test {
    public static int gcd(int a, int b) {
        if (b == 0) {
            return a;
        }
        return gcd(b, a % b);
    }

    public static int gcdIterative(int a, int b) {
        while (b != 0) {
            int temp = b;
            b = a % b;
            a = temp;
        }
        return a;
    }

    public static int lcm(int a, int b) {
        return (a * b) / gcd(a, b);
    }

    public static void main(String[] args) {
        int a = 48;
        int b = 18;

        System.out.println("Numbers: " + a + " and " + b);
        System.out.println("GCD (recursive): " + gcd(a, b));
        System.out.println("GCD (iterative): " + gcdIterative(a, b));
        System.out.println("LCM: " + lcm(a, b));

        // Test with larger numbers
        int x = 1071;
        int y = 462;
        System.out.println("\nNumbers: " + x + " and " + y);
        System.out.println("GCD: " + gcd(x, y));
        System.out.println("LCM: " + lcm(x, y));
    }
}

