import java.math.BigDecimal;
import java.math.BigInteger;
import java.math.MathContext;
import java.math.RoundingMode;

/** Tests for BigDecimal operations. */
public class Test {

    public static void main(String[] args) {
        System.out.println("=== BigDecimal Tests ===");

        testConstructors();
        testBasicArithmetic();
        testPrecisionAndScale();
        testRoundingModes();
        testComparisonOperations();
        testMathContext();
        testConversions();
        testSpecialOperations();
        testSpecialCases();
    }

    static void testConstructors() {
        System.out.println("\n--- Constructor Tests ---");

        // String constructor
        BigDecimal a = new BigDecimal("123.456789");
        System.out.println("String constructor: " + a);

        // Double constructor (note: can introduce precision issues)
        BigDecimal b = new BigDecimal(123.456);
        System.out.println("Double constructor: " + b);

        // int/long constructors
        BigDecimal c = new BigDecimal(123456);
        System.out.println("Int constructor: " + c);

        // BigInteger constructor
        BigInteger bigInt = new BigInteger("123456789012345678901234567890");
        BigDecimal d = new BigDecimal(bigInt);
        System.out.println("BigInteger constructor: " + d);

        // BigInteger with scale
        BigDecimal e = new BigDecimal(bigInt, 10);
        System.out.println("BigInteger with scale 10: " + e);

        // valueOf methods
        BigDecimal f = BigDecimal.valueOf(123456, 3);
        System.out.println("valueOf(123456, 3): " + f);

        BigDecimal g = BigDecimal.valueOf(123.456);
        System.out.println("valueOf(123.456): " + g);
    }

    static void testBasicArithmetic() {
        System.out.println("\n--- Basic Arithmetic Tests ---");

        BigDecimal a = new BigDecimal("123.456");
        BigDecimal b = new BigDecimal("78.9");

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
        System.out.println("a / b = " + a.divide(b, 10, RoundingMode.HALF_UP));

        // Division with MathContext
        MathContext mc = new MathContext(5, RoundingMode.HALF_UP);
        System.out.println("a / b (MathContext 5 digits): " + a.divide(b, mc));

        // Remainder
        System.out.println("a % b = " + a.remainder(b));

        // divideAndRemainder
        BigDecimal[] divRem = a.divideAndRemainder(b);
        System.out.println("a divideAndRemainder b: [" + divRem[0] + ", " + divRem[1] + "]");

        // Negation
        System.out.println("-a = " + a.negate());

        // Absolute value
        BigDecimal negative = new BigDecimal("-123.456");
        System.out.println("abs(-123.456) = " + negative.abs());
    }

    static void testPrecisionAndScale() {
        System.out.println("\n--- Precision and Scale Tests ---");

        BigDecimal[] testValues = {
            new BigDecimal("123.456"),
            new BigDecimal("0.00123"),
            new BigDecimal("1230000"),
            new BigDecimal("0.000"),
            new BigDecimal("123.000"),
            new BigDecimal("12300")
        };

        for (BigDecimal val : testValues) {
            System.out.println(val + " -> precision: " + val.precision() + ", scale: " + val.scale());
        }

        // Scale manipulation
        BigDecimal base = new BigDecimal("123.456");
        System.out.println("\nScale manipulation for " + base + ":");
        System.out.println("setScale(2, HALF_UP): " + base.setScale(2, RoundingMode.HALF_UP));
        System.out.println("setScale(5, HALF_UP): " + base.setScale(5, RoundingMode.HALF_UP));
        System.out.println("setScale(0, HALF_UP): " + base.setScale(0, RoundingMode.HALF_UP));

        // Strip trailing zeros
        BigDecimal withZeros = new BigDecimal("123.4500");
        System.out.println("stripTrailingZeros(" + withZeros + "): " + withZeros.stripTrailingZeros());
    }

    static void testRoundingModes() {
        System.out.println("\n--- Rounding Modes Tests ---");

        BigDecimal[] testValues = {
            new BigDecimal("2.5"),
            new BigDecimal("3.5"),
            new BigDecimal("-2.5"),
            new BigDecimal("-3.5"),
            new BigDecimal("2.1"),
            new BigDecimal("2.9")
        };

        RoundingMode[] modes = {
            RoundingMode.UP,
            RoundingMode.DOWN,
            RoundingMode.CEILING,
            RoundingMode.FLOOR,
            RoundingMode.HALF_UP,
            RoundingMode.HALF_DOWN,
            RoundingMode.HALF_EVEN
        };

        for (BigDecimal val : testValues) {
            System.out.println("\nRounding " + val + " to 0 decimal places:");
            for (RoundingMode mode : modes) {
                System.out.println("  " + mode + ": " + val.setScale(0, mode));
            }
        }
    }

    static void testComparisonOperations() {
        System.out.println("\n--- Comparison Tests ---");

        BigDecimal a = new BigDecimal("123.45");
        BigDecimal b = new BigDecimal("123.450");
        BigDecimal c = new BigDecimal("123.46");

        System.out.println("a = " + a);
        System.out.println("b = " + b);
        System.out.println("c = " + c);

        System.out.println("a.compareTo(b) = " + a.compareTo(b));
        System.out.println("a.compareTo(c) = " + a.compareTo(c));
        System.out.println("c.compareTo(a) = " + c.compareTo(a));

        System.out.println("a.equals(b) = " + a.equals(b));
        System.out.println("a.equals(c) = " + a.equals(c));

        // Note: equals considers scale, compareTo doesn't
        System.out.println("123.45 equals 123.450? " + a.equals(b));
        System.out.println("123.45 compareTo 123.450? " + a.compareTo(b));

        System.out.println("a.min(c) = " + a.min(c));
        System.out.println("a.max(c) = " + a.max(c));
    }

    static void testMathContext() {
        System.out.println("\n--- MathContext Tests ---");

        BigDecimal a = new BigDecimal("1");
        BigDecimal b = new BigDecimal("3");

        // Different precision contexts
        MathContext[] contexts = {
            new MathContext(3, RoundingMode.HALF_UP),
            new MathContext(5, RoundingMode.HALF_UP),
            new MathContext(10, RoundingMode.HALF_UP),
            MathContext.DECIMAL32,
            MathContext.DECIMAL64,
            MathContext.DECIMAL128
        };

        System.out.println("1/3 with different MathContexts:");
        for (MathContext mc : contexts) {
            BigDecimal result = a.divide(b, mc);
            System.out.println("  " + mc + ": " + result);
        }

        // Arithmetic with MathContext
        BigDecimal x = new BigDecimal("12.345");
        BigDecimal y = new BigDecimal("6.789");
        MathContext mc5 = new MathContext(5);

        System.out.println("\nArithmetic with 5-digit precision:");
        System.out.println("(" + x + " + " + y + ") = " + x.add(y, mc5));
        System.out.println("(" + x + " * " + y + ") = " + x.multiply(y, mc5));
    }

    static void testConversions() {
        System.out.println("\n--- Conversion Tests ---");

        BigDecimal big = new BigDecimal("123456.789");

        System.out.println("Original: " + big);
        System.out.println("intValue(): " + big.intValue());
        System.out.println("longValue(): " + big.longValue());
        System.out.println("floatValue(): " + big.floatValue());
        System.out.println("doubleValue(): " + big.doubleValue());

        // Exact conversions
        BigDecimal exact = new BigDecimal("123");
        System.out.println("intValueExact(): " + exact.intValueExact());
        System.out.println("longValueExact(): " + exact.longValueExact());

        // BigInteger conversion
        System.out.println("toBigInteger(): " + big.toBigInteger());
        System.out.println("toBigIntegerExact() for 123.000: " + new BigDecimal("123.000").toBigIntegerExact());

        // String representations
        System.out.println("toString(): " + big.toString());
        System.out.println("toEngineeringString(): " + big.toEngineeringString());
        System.out.println("toPlainString(): " + big.toPlainString());

        // Scientific notation
        BigDecimal scientific = new BigDecimal("1.23E+5");
        System.out.println("Scientific " + scientific + " -> plain: " + scientific.toPlainString());
    }

    static void testSpecialOperations() {
        System.out.println("\n--- Special Operations Tests ---");

        BigDecimal base = new BigDecimal("2");

        // Power operations
        System.out.println("2^10 = " + base.pow(10));
        System.out.println("2^0 = " + base.pow(0));

        // Power with MathContext
        MathContext mc = new MathContext(15);
        System.out.println("2^100 (15 digits): " + base.pow(100, mc));

        // Square root (Java 9+)
        BigDecimal sqrt2 = new BigDecimal("2");
        try {
            System.out.println("sqrt(2) (10 digits): " + sqrt2.sqrt(new MathContext(10)));
        } catch (Exception e) {
            System.out.println("sqrt method not available (requires Java 9+)");
        }

        // Move decimal point
        BigDecimal decimal = new BigDecimal("123.456");
        System.out.println("movePointLeft(2): " + decimal.movePointLeft(2));
        System.out.println("movePointRight(2): " + decimal.movePointRight(2));

        // Scale by power of 10
        System.out.println("scaleByPowerOfTen(3): " + decimal.scaleByPowerOfTen(3));
        System.out.println("scaleByPowerOfTen(-2): " + decimal.scaleByPowerOfTen(-2));

        // Ulp (unit in the last place)
        System.out.println("ulp() of " + decimal + ": " + decimal.ulp());
    }

    static void testSpecialCases() {
        System.out.println("\n--- Special Cases Tests ---");

        // Constants
        System.out.println("BigDecimal.ZERO = " + BigDecimal.ZERO);
        System.out.println("BigDecimal.ONE = " + BigDecimal.ONE);
        System.out.println("BigDecimal.TEN = " + BigDecimal.TEN);

        // Sign operations
        BigDecimal positive = new BigDecimal("123.456");
        BigDecimal negative = new BigDecimal("-123.456");
        BigDecimal zero = BigDecimal.ZERO;

        System.out.println("signum(123.456) = " + positive.signum());
        System.out.println("signum(-123.456) = " + negative.signum());
        System.out.println("signum(0) = " + zero.signum());

        // Edge cases with zero
        System.out.println("0 + 123.456 = " + zero.add(positive));
        System.out.println("0 * 123.456 = " + zero.multiply(positive));
        System.out.println("123.456 * 0 = " + positive.multiply(zero));

        // Very small and large numbers
        BigDecimal verySmall = new BigDecimal("1E-100");
        BigDecimal veryLarge = new BigDecimal("1E+100");
        System.out.println("Very small: " + verySmall);
        System.out.println("Very large: " + veryLarge);
        System.out.println("Very small + very large: " + verySmall.add(veryLarge));

        // Precision edge cases
        BigDecimal precise = new BigDecimal("0.1");
        BigDecimal sum = BigDecimal.ZERO;
        for (int i = 0; i < 10; i++) {
            sum = sum.add(precise);
        }
        System.out.println("0.1 added 10 times: " + sum);
        System.out.println("Is it exactly 1.0? " + sum.equals(BigDecimal.ONE));

        // Division by zero handling (will throw exception)
        try {
            BigDecimal result = positive.divide(zero);
            System.out.println("Division by zero result: " + result);
        } catch (ArithmeticException e) {
            System.out.println("Division by zero throws: " + e.getClass().getSimpleName());
        }

        // Non-terminating decimal
        try {
            BigDecimal one = BigDecimal.ONE;
            BigDecimal three = new BigDecimal("3");
            BigDecimal result = one.divide(three); // This will throw without rounding mode
            System.out.println("1/3 without rounding: " + result);
        } catch (ArithmeticException e) {
            System.out.println("1/3 without rounding throws: " + e.getClass().getSimpleName());
        }
    }
}
