public class Test {
    public static void main(String[] args) {
        // Test all float arithmetic operations with special cases

        testFloatAdd();
        testFloatSub();
        testFloatMul();
        testFloatDiv();
        testFloatRem();
        testFloatNeg();
        testFloatCompare();
        testFloatConversions();
        testFloatSpecialValues();
        testFloatPrecision();
    }

    static void testFloatAdd() {
        float a = 123.456f;
        float b = 78.9f;
        float result = a + b;
        System.out.println(result);

        // Large numbers
        float large1 = 1.5e30f;
        float large2 = 2.5e30f;
        float largeSum = large1 + large2;
        System.out.println(largeSum);

        // Adding infinity
        float inf = Float.POSITIVE_INFINITY;
        float finite = 100.0f;
        float infSum = inf + finite;
        System.out.println(infSum);

        // Adding opposite infinities
        float negInf = Float.NEGATIVE_INFINITY;
        float nanResult = inf + negInf;
        System.out.println(nanResult);
    }

    static void testFloatSub() {
        float a = 123.456f;
        float b = 45.678f;
        float result = a - b;
        System.out.println(result);

        // Subtracting from infinity
        float inf = Float.POSITIVE_INFINITY;
        float finite = 100.0f;
        float infSub = inf - finite;
        System.out.println(infSub);

        // Subtracting same infinities
        float nanResult = inf - inf;
        System.out.println(nanResult);

        // Very small difference
        float closeA = 1.0000001f;
        float closeB = 1.0000000f;
        float smallDiff = closeA - closeB;
        System.out.println(smallDiff);
    }

    static void testFloatMul() {
        float a = 12.5f;
        float b = 8.4f;
        float result = a * b;
        System.out.println(result);

        // Multiply by zero
        float zero = 0.0f;
        float zeroResult = a * zero;
        System.out.println(zeroResult);

        // Multiply infinity by zero
        float inf = Float.POSITIVE_INFINITY;
        float nanResult = inf * zero;
        System.out.println(nanResult);

        // Large multiplication causing overflow
        float large = Float.MAX_VALUE;
        float overflow = large * 2.0f;
        System.out.println(overflow);

        // Small multiplication causing underflow
        float small = Float.MIN_VALUE;
        float underflow = small * 0.5f;
        System.out.println(underflow);
    }

    static void testFloatDiv() {
        float a = 100.0f;
        float b = 3.0f;
        float result = a / b;
        System.out.println(result);

        // Division by zero
        float zero = 0.0f;
        float divByZero = a / zero;
        System.out.println(divByZero);

        // Zero divided by zero
        float nanResult = zero / zero;
        System.out.println(nanResult);

        // Infinity divided by infinity
        float inf = Float.POSITIVE_INFINITY;
        float infDiv = inf / inf;
        System.out.println(infDiv);

        // Very small division
        float small = Float.MIN_VALUE;
        float smallDiv = small / 2.0f;
        System.out.println(smallDiv);
    }

    static void testFloatRem() {
        float a = 10.5f;
        float b = 3.2f;
        float result = a % b;
        System.out.println(result);

        // Remainder with negative
        float neg = -3.2f;
        float negRem = a % neg;
        System.out.println(negRem);

        // Negative dividend
        float negA = -10.5f;
        float negDividend = negA % b;
        System.out.println(negDividend);

        // Remainder with infinity
        float inf = Float.POSITIVE_INFINITY;
        float infRem = a % inf;
        System.out.println(infRem);

        // Infinity remainder finite
        float nanResult = inf % a;
        System.out.println(nanResult);
    }

    static void testFloatNeg() {
        float positive = 123.456f;
        float negResult = -positive;
        System.out.println(negResult);

        float negative = -123.456f;
        float posResult = -negative;
        System.out.println(posResult);

        // Negate zero
        float zero = 0.0f;
        float negZero = -zero;
        System.out.println(negZero);

        // Negate infinity
        float inf = Float.POSITIVE_INFINITY;
        float negInf = -inf;
        System.out.println(negInf);

        // Negate NaN
        float nan = Float.NaN;
        float negNan = -nan;
        System.out.println(negNan);
    }

    static void testFloatCompare() {
        float a = 123.456f;
        float b = 78.9f;
        float c = 123.456f;

        System.out.println(a > b ? 1 : 0);
        System.out.println(a < b ? 1 : 0);
        System.out.println(a == c ? 1 : 0);
        System.out.println(a != b ? 1 : 0);
        System.out.println(a >= c ? 1 : 0);
        System.out.println(a <= b ? 1 : 0);

        // Compare with NaN
        float nan = Float.NaN;
        System.out.println(a > nan ? 1 : 0);
        System.out.println(a < nan ? 1 : 0);
        System.out.println(a == nan ? 1 : 0);
        System.out.println(nan == nan ? 1 : 0);

        // Compare infinities
        float posInf = Float.POSITIVE_INFINITY;
        float negInf = Float.NEGATIVE_INFINITY;
        System.out.println(posInf > a ? 1 : 0);
        System.out.println(negInf < a ? 1 : 0);
    }

    static void testFloatConversions() {
        float f = 123.456f;

        // Float to integer types (truncation)
        byte b = (byte)f;
        System.out.println((int)b);

        short s = (short)f;
        System.out.println((int)s);

        int i = (int)f;
        System.out.println(i);

        long l = (long)f;
        System.out.println(l);

        // Float to double
        double d = f;
        System.out.println(d);

        // From other types to float
        int fromInt = 1000000;
        float backToFloat = fromInt;
        System.out.println(backToFloat);

        // Large long to float (precision loss)
        long largeLong = 123456789123456789L;
        float fromLargeLong = largeLong;
        System.out.println(fromLargeLong);
    }

    static void testFloatSpecialValues() {
        // Test special float values
        float posInf = Float.POSITIVE_INFINITY;
        System.out.println(posInf);

        float negInf = Float.NEGATIVE_INFINITY;
        System.out.println(negInf);

        float nan = Float.NaN;
        System.out.println(nan);

        float maxValue = Float.MAX_VALUE;
        System.out.println(maxValue);

        float minValue = Float.MIN_VALUE;
        System.out.println(minValue);

        float minNormal = Float.MIN_NORMAL;
        System.out.println(minNormal);

        // Test isNaN and isInfinite equivalent behavior
        System.out.println(nan != nan ? 1 : 0); // NaN comparison
        System.out.println(posInf == Float.POSITIVE_INFINITY ? 1 : 0);
    }

    static void testFloatPrecision() {
        // Test precision limits
        float base = 1.0f;
        float epsilon = 1.0e-7f;
        float sum = base + epsilon;
        System.out.println(sum);
        System.out.println(sum == base ? 1 : 0);

        // Test denormalized numbers
        float tiny = Float.MIN_VALUE;
        float halfTiny = tiny / 2.0f;
        System.out.println(halfTiny);

        // Test rounding behavior
        float round1 = 0.1f + 0.2f; // Classic floating point issue
        System.out.println(round1);
        System.out.println(round1 == 0.3f ? 1 : 0);
    }
}

