import java.math.BigInteger;

/** Comprehensive test for BigInteger operations. */
public class Test {

    public static void main(String[] args) {
        System.out.println("=== BigInteger Tests ===");

        testConstructors();
        testBasicArithmetic();
        testComparisonOperations();
        testBitwiseOperations();
        testModularArithmetic();
        testPrimeOperations();
        testPowerOperations();
        testGcdLcm();
        testConversions();
        testSpecialCases();
    }

    static void testConstructors() {
        System.out.println("\n--- Constructor Tests ---");

        // String constructor
        BigInteger a = new BigInteger("12345678901234567890");
        System.out.println("String constructor: " + a);

        // Byte array constructor
        byte[] bytes = {1, 2, 3, 4, 5};
        BigInteger b = new BigInteger(bytes);
        System.out.println("Byte array constructor: " + b);

        // Radix constructor
        BigInteger c = new BigInteger("ff", 16);
        System.out.println("Hex constructor: " + c);

        BigInteger d = new BigInteger("1010", 2);
        System.out.println("Binary constructor: " + d);

        // valueOf
        BigInteger e = BigInteger.valueOf(Long.MAX_VALUE);
        System.out.println("valueOf Long.MAX_VALUE: " + e);
    }

    static void testBasicArithmetic() {
        System.out.println("\n--- Basic Arithmetic Tests ---");

        BigInteger a = new BigInteger("123456789012345678901234567890");
        BigInteger b = new BigInteger("987654321098765432109876543210");

        System.out.println("a = " + a);
        System.out.println("b = " + b);

        // Addition
        System.out.println("a + b = " + a.add(b));

        // Subtraction
        System.out.println("a - b = " + a.subtract(b));
        System.out.println("b - a = " + b.subtract(a));

        // Multiplication
        System.out.println("a * b = " + a.multiply(b));

        // Division
        System.out.println("b / a = " + b.divide(a));
        System.out.println("a / b = " + a.divide(b));

        // Remainder
        System.out.println("b % a = " + b.remainder(a));

        // divideAndRemainder
        BigInteger[] divRem = b.divideAndRemainder(a);
        System.out.println("b divideAndRemainder a: [" + divRem[0] + ", " + divRem[1] + "]");

        // Negation
        System.out.println("-a = " + a.negate());

        // Absolute value
        BigInteger negative = new BigInteger("-12345");
        System.out.println("abs(-12345) = " + negative.abs());
    }

    static void testComparisonOperations() {
        System.out.println("\n--- Comparison Tests ---");

        BigInteger a = new BigInteger("1000");
        BigInteger b = new BigInteger("2000");
        BigInteger c = new BigInteger("1000");

        System.out.println("a = " + a);
        System.out.println("b = " + b);
        System.out.println("c = " + c);

        System.out.println("a.compareTo(b) = " + a.compareTo(b));
        System.out.println("b.compareTo(a) = " + b.compareTo(a));
        System.out.println("a.compareTo(c) = " + a.compareTo(c));

        System.out.println("a.equals(b) = " + a.equals(b));
        System.out.println("a.equals(c) = " + a.equals(c));

        System.out.println("a.min(b) = " + a.min(b));
        System.out.println("a.max(b) = " + a.max(b));
    }

    static void testBitwiseOperations() {
        System.out.println("\n--- Bitwise Operations Tests ---");

        BigInteger a = new BigInteger("170"); // 10101010 in binary
        BigInteger b = new BigInteger("85");  // 01010101 in binary

        System.out.println("a = " + a + " (binary: " + a.toString(2) + ")");
        System.out.println("b = " + b + " (binary: " + b.toString(2) + ")");

        // Bitwise AND
        System.out.println("a & b = " + a.and(b) + " (binary: " + a.and(b).toString(2) + ")");

        // Bitwise OR
        System.out.println("a | b = " + a.or(b) + " (binary: " + a.or(b).toString(2) + ")");

        // Bitwise XOR
        System.out.println("a ^ b = " + a.xor(b) + " (binary: " + a.xor(b).toString(2) + ")");

        // Bitwise NOT
        System.out.println("~a = " + a.not());

        // Bit operations
        System.out.println("a.testBit(0) = " + a.testBit(0));
        System.out.println("a.testBit(1) = " + a.testBit(1));
        System.out.println("a.setBit(0) = " + a.setBit(0));
        System.out.println("a.clearBit(1) = " + a.clearBit(1));
        System.out.println("a.flipBit(2) = " + a.flipBit(2));

        // Bit length and count
        System.out.println("a.bitLength() = " + a.bitLength());
        System.out.println("a.bitCount() = " + a.bitCount());

        // Shifts
        System.out.println("a << 2 = " + a.shiftLeft(2));
        System.out.println("a >> 1 = " + a.shiftRight(1));
    }

    static void testModularArithmetic() {
        System.out.println("\n--- Modular Arithmetic Tests ---");

        BigInteger base = new BigInteger("123");
        BigInteger exponent = new BigInteger("456");
        BigInteger modulus = new BigInteger("789");

        System.out.println("base = " + base);
        System.out.println("exponent = " + exponent);
        System.out.println("modulus = " + modulus);

        // Modular exponentiation
        System.out.println("base^exponent mod modulus = " + base.modPow(exponent, modulus));

        // Modular inverse
        BigInteger a = new BigInteger("17");
        BigInteger m = new BigInteger("43");
        System.out.println("17^(-1) mod 43 = " + a.modInverse(m));

        // Verify: a * a^(-1) ≡ 1 (mod m)
        BigInteger inverse = a.modInverse(m);
        System.out.println("Verification: 17 * " + inverse + " mod 43 = " + a.multiply(inverse).mod(m));
    }

    static void testPrimeOperations() {
        System.out.println("\n--- Prime Operations Tests ---");

        // Probable prime test
        BigInteger[] testNumbers = {
            new BigInteger("17"),
            new BigInteger("18"),
            new BigInteger("1009"),
            new BigInteger("1013"),
            new BigInteger("982451653")
        };

        for (BigInteger num : testNumbers) {
            System.out.println(num + ".isProbablePrime(10) = " + num.isProbablePrime(10));
        }

        // Next probable prime
        BigInteger start = new BigInteger("1000");
        System.out.println("Next probable prime after " + start + " = " + start.nextProbablePrime());
    }

    static void testPowerOperations() {
        System.out.println("\n--- Power Operations Tests ---");

        BigInteger base = new BigInteger("2");
        int exponent = 100;

        System.out.println("2^100 = " + base.pow(exponent));

        // Large power
        BigInteger largeBase = new BigInteger("123");
        System.out.println("123^50 = " + largeBase.pow(50));
    }

    static void testGcdLcm() {
        System.out.println("\n--- GCD Tests ---");

        BigInteger a = new BigInteger("48");
        BigInteger b = new BigInteger("18");

        System.out.println("gcd(48, 18) = " + a.gcd(b));

        BigInteger large1 = new BigInteger("123456789012345");
        BigInteger large2 = new BigInteger("987654321098765");
        System.out.println("gcd(" + large1 + ", " + large2 + ") = " + large1.gcd(large2));
    }

    static void testConversions() {
        System.out.println("\n--- Conversion Tests ---");

        BigInteger big = new BigInteger("12345678901234567890");

        System.out.println("Original: " + big);
        System.out.println("toString(2): " + big.toString(2));
        System.out.println("toString(8): " + big.toString(8));
        System.out.println("toString(16): " + big.toString(16));
        System.out.println("toString(36): " + big.toString(36));

        // To primitive types (with overflow checking)
        BigInteger small = new BigInteger("123");
        System.out.println("small.intValue() = " + small.intValue());
        System.out.println("small.longValue() = " + small.longValue());
        System.out.println("small.floatValue() = " + small.floatValue());
        System.out.println("small.doubleValue() = " + small.doubleValue());

        // Byte array
        byte[] bytes = small.toByteArray();
        System.out.print("toByteArray(): [");
        for (int i = 0; i < bytes.length; i++) {
            System.out.print(bytes[i]);
            if (i < bytes.length - 1) System.out.print(", ");
        }
        System.out.println("]");
    }

    static void testSpecialCases() {
        System.out.println("\n--- Special Cases Tests ---");

        // Constants
        System.out.println("BigInteger.ZERO = " + BigInteger.ZERO);
        System.out.println("BigInteger.ONE = " + BigInteger.ONE);
        System.out.println("BigInteger.TEN = " + BigInteger.TEN);

        // Sign operations
        BigInteger positive = new BigInteger("123");
        BigInteger negative = new BigInteger("-123");
        BigInteger zero = BigInteger.ZERO;

        System.out.println("signum(123) = " + positive.signum());
        System.out.println("signum(-123) = " + negative.signum());
        System.out.println("signum(0) = " + zero.signum());

        // Edge cases with zero
        System.out.println("0 + 123 = " + zero.add(positive));
        System.out.println("0 * 123 = " + zero.multiply(positive));
        System.out.println("123 * 0 = " + positive.multiply(zero));

        // Very large numbers
        BigInteger huge1 = new BigInteger("9".repeat(100));
        BigInteger huge2 = new BigInteger("1" + "0".repeat(100));
        System.out.println("huge1 + 1 = " + huge1.add(BigInteger.ONE));
        System.out.println("10^100 - huge1 = " + huge2.subtract(huge1));
    }
}
