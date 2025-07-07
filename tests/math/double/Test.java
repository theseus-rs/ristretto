public class Test {
    public static void main(String[] args) {
        // Test all double arithmetic operations with special cases

        testDoubleAdd();
        testDoubleSub();
        testDoubleMul();
        testDoubleDiv();
        testDoubleRem();
        testDoubleNeg();
        testDoubleCompare();
        testDoubleConversions();
        testDoubleSpecialValues();
        testDoublePrecision();
    }

    static void testDoubleAdd() {
        double a = 123.456789;
        double b = 78.987654;
        double result = a + b;
        System.out.println(result);

        // Large numbers
        double large1 = 1.5e200;
        double large2 = 2.5e200;
        double largeSum = large1 + large2;
        System.out.println(largeSum);

        // Adding infinity
        double inf = Double.POSITIVE_INFINITY;
        double finite = 100.0;
        double infSum = inf + finite;
        System.out.println(infSum);

        // Adding opposite infinities
        double negInf = Double.NEGATIVE_INFINITY;
        double nanResult = inf + negInf;
        System.out.println(nanResult);
    }

    static void testDoubleSub() {
        double a = 123.456789;
        double b = 45.678901;
        double result = a - b;
        System.out.println(result);

        // Subtracting from infinity
        double inf = Double.POSITIVE_INFINITY;
        double finite = 100.0;
        double infSub = inf - finite;
        System.out.println(infSub);

        // Subtracting same infinities
        double nanResult = inf - inf;
        System.out.println(nanResult);

        // Very small difference
        double closeA = 1.0000000000000001;
        double closeB = 1.0000000000000000;
        double smallDiff = closeA - closeB;
        System.out.println(smallDiff);
    }

    static void testDoubleMul() {
        double a = 12.5678;
        double b = 8.4321;
        double result = a * b;
        System.out.println(result);

        // Multiply by zero
        double zero = 0.0;
        double zeroResult = a * zero;
        System.out.println(zeroResult);

        // Multiply infinity by zero
        double inf = Double.POSITIVE_INFINITY;
        double nanResult = inf * zero;
        System.out.println(nanResult);

        // Large multiplication causing overflow
        double large = Double.MAX_VALUE;
        double overflow = large * 2.0;
        System.out.println(overflow);

        // Small multiplication causing underflow
        double small = Double.MIN_VALUE;
        double underflow = small * 0.5;
        System.out.println(underflow);
    }

    static void testDoubleDiv() {
        double a = 100.0;
        double b = 3.0;
        double result = a / b;
        System.out.println(result);

        // Division by zero
        double zero = 0.0;
        double divByZero = a / zero;
        System.out.println(divByZero);

        // Zero divided by zero
        double nanResult = zero / zero;
        System.out.println(nanResult);

        // Infinity divided by infinity
        double inf = Double.POSITIVE_INFINITY;
        double infDiv = inf / inf;
        System.out.println(infDiv);

        // Very small division
        double small = Double.MIN_VALUE;
        double smallDiv = small / 2.0;
        System.out.println(smallDiv);
    }

    static void testDoubleRem() {
        double a = 10.5;
        double b = 3.2;
        double result = a % b;
        System.out.println(result);

        // Remainder with negative
        double neg = -3.2;
        double negRem = a % neg;
        System.out.println(negRem);

        // Negative dividend
        double negA = -10.5;
        double negDividend = negA % b;
        System.out.println(negDividend);

        // Remainder with infinity
        double inf = Double.POSITIVE_INFINITY;
        double infRem = a % inf;
        System.out.println(infRem);

        // Infinity remainder finite
        double nanResult = inf % a;
        System.out.println(nanResult);
    }

    static void testDoubleNeg() {
        double positive = 123.456789;
        double negResult = -positive;
        System.out.println(negResult);

        double negative = -123.456789;
        double posResult = -negative;
        System.out.println(posResult);

        // Negate zero
        double zero = 0.0;
        double negZero = -zero;
        System.out.println(negZero);

        // Negate infinity
        double inf = Double.POSITIVE_INFINITY;
        double negInf = -inf;
        System.out.println(negInf);

        // Negate NaN
        double nan = Double.NaN;
        double negNan = -nan;
        System.out.println(negNan);
    }

    static void testDoubleCompare() {
        double a = 123.456789;
        double b = 78.987654;
        double c = 123.456789;

        System.out.println(a > b ? 1 : 0);
        System.out.println(a < b ? 1 : 0);
        System.out.println(a == c ? 1 : 0);
        System.out.println(a != b ? 1 : 0);
        System.out.println(a >= c ? 1 : 0);
        System.out.println(a <= b ? 1 : 0);

        // Compare with NaN
        double nan = Double.NaN;
        System.out.println(a > nan ? 1 : 0);
        System.out.println(a < nan ? 1 : 0);
        System.out.println(a == nan ? 1 : 0);
        System.out.println(nan == nan ? 1 : 0);

        // Compare infinities
        double posInf = Double.POSITIVE_INFINITY;
        double negInf = Double.NEGATIVE_INFINITY;
        System.out.println(posInf > a ? 1 : 0);
        System.out.println(negInf < a ? 1 : 0);
    }

    static void testDoubleConversions() {
        double d = 123.456789;

        // Double to integer types (truncation)
        byte b = (byte)d;
        System.out.println((int)b);

        short s = (short)d;
        System.out.println((int)s);

        int i = (int)d;
        System.out.println(i);

        long l = (long)d;
        System.out.println(l);

        // Double to float (precision loss)
        float f = (float)d;
        System.out.println(f);

        // From other types to double
        long largeLong = 123456789123456789L;
        double fromLong = largeLong;
        System.out.println(fromLong);

        // Very large long (precision loss)
        long veryLarge = 9223372036854775807L;
        double fromVeryLarge = veryLarge;
        System.out.println(fromVeryLarge);
    }

    static void testDoubleSpecialValues() {
        // Test special double values
        double posInf = Double.POSITIVE_INFINITY;
        System.out.println(posInf);

        double negInf = Double.NEGATIVE_INFINITY;
        System.out.println(negInf);

        double nan = Double.NaN;
        System.out.println(nan);

        double maxValue = Double.MAX_VALUE;
        System.out.println(maxValue);

        double minValue = Double.MIN_VALUE;
        System.out.println(minValue);

        double minNormal = Double.MIN_NORMAL;
        System.out.println(minNormal);

        // Test isNaN and isInfinite equivalent behavior
        System.out.println(nan != nan ? 1 : 0); // NaN comparison
        System.out.println(posInf == Double.POSITIVE_INFINITY ? 1 : 0);
    }

    static void testDoublePrecision() {
        // Test precision limits
        double base = 1.0;
        double epsilon = 1.0e-15;
        double sum = base + epsilon;
        System.out.println(sum);
        System.out.println(sum == base ? 1 : 0);

        // Test denormalized numbers
        double tiny = Double.MIN_VALUE;
        double halfTiny = tiny / 2.0;
        System.out.println(halfTiny);

        // Test rounding behavior
        double round1 = 0.1 + 0.2; // Classic floating point issue
        System.out.println(round1);
        System.out.println(round1 == 0.3 ? 1 : 0);

        // Test high precision arithmetic
        double highPrec1 = 1.0000000000000001;
        double highPrec2 = 1.0000000000000002;
        double highPrecSum = highPrec1 + highPrec2;
        System.out.println(highPrecSum);

        // Test very small numbers near machine epsilon
        double machineEps = 2.220446049250313e-16;
        double epsTest = 1.0 + machineEps;
        System.out.println(epsTest);
        System.out.println(epsTest > 1.0 ? 1 : 0);
    }
}

