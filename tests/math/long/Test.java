public class Test {
    public static void main(String[] args) {
        // Test all long arithmetic operations with bounds checking

        testLongAdd();
        testLongSub();
        testLongMul();
        testLongDiv();
        testLongRem();
        testLongNeg();
        testLongInc();
        testLongCompare();
        testLongConversions();
        testLongBitwise();
        testLongShift();
    }

    static void testLongAdd() {
        long a = 1000000000000L;
        long b = 500000000000L;
        long result = a + b;
        System.out.println(result);

        // Overflow test
        long max = 9223372036854775807L;
        long one = 1L;
        long overflow = max + one;
        System.out.println(overflow);

        // Underflow test
        long min = -9223372036854775808L;
        long negOne = -1L;
        long underflow = min + negOne;
        System.out.println(underflow);
    }

    static void testLongSub() {
        long a = 1000000000000L;
        long b = 300000000000L;
        long result = a - b;
        System.out.println(result);

        // Underflow test
        long min = -9223372036854775808L;
        long one = 1L;
        long underflow = min - one;
        System.out.println(underflow);

        // Overflow test
        long max = 9223372036854775807L;
        long negOne = -1L;
        long overflow = max - negOne;
        System.out.println(overflow);
    }

    static void testLongMul() {
        long a = 1000000000L;
        long b = 5000L;
        long result = a * b;
        System.out.println(result);

        // Overflow test
        long max = 9223372036854775807L;
        long two = 2L;
        long overflow = max * two;
        System.out.println(overflow);

        // Zero test
        long zero = 0L;
        long zeroResult = a * zero;
        System.out.println(zeroResult);
    }

    static void testLongDiv() {
        long a = 1000000000000L;
        long b = 2500L;
        long result = a / b;
        System.out.println(result);

        // Division by one
        long one = 1L;
        long divByOne = a / one;
        System.out.println(divByOne);

        // Negative division
        long neg = -2500L;
        long negDiv = a / neg;
        System.out.println(negDiv);

        // Min value division
        long min = -9223372036854775808L;
        long negativeOne = -1L;
        long minDiv = min / negativeOne;
        System.out.println(minDiv);
    }

    static void testLongRem() {
        long a = 1000000000000L;
        long b = 3333L;
        long result = a % b;
        System.out.println(result);

        // Remainder with negative
        long neg = -3333L;
        long negRem = a % neg;
        System.out.println(negRem);

        // Negative dividend
        long negA = -1000000000000L;
        long negDividend = negA % b;
        System.out.println(negDividend);
    }

    static void testLongNeg() {
        long positive = 1000000000000L;
        long negResult = -positive;
        System.out.println(negResult);

        long negative = -1000000000000L;
        long posResult = -negative;
        System.out.println(posResult);

        // Min value negation (overflow)
        long min = -9223372036854775808L;
        long minNeg = -min;
        System.out.println(minNeg);

        long zero = 0L;
        long zeroNeg = -zero;
        System.out.println(zeroNeg);
    }

    static void testLongInc() {
        long val = 1000000000000L;
        val++;
        System.out.println(val);

        // Increment at boundary
        long max = 9223372036854775806L;
        max++;
        System.out.println(max);

        // Overflow increment
        long maxVal = 9223372036854775807L;
        maxVal++;
        System.out.println(maxVal);
    }

    static void testLongCompare() {
        long a = 1000000000000L;
        long b = 500000000000L;
        long c = 1000000000000L;

        System.out.println(a > b ? 1 : 0);
        System.out.println(a < b ? 1 : 0);
        System.out.println(a == c ? 1 : 0);
        System.out.println(a != b ? 1 : 0);
        System.out.println(a >= c ? 1 : 0);
        System.out.println(a <= b ? 1 : 0);
    }

    static void testLongConversions() {
        long l = 10000000000L;

        // Long to smaller types (explicit cast)
        byte b = (byte)l;
        System.out.println((int)b);

        short s = (short)l;
        System.out.println((int)s);

        int i = (int)l;
        System.out.println(i);

        // Long to floating point
        float f = l;
        System.out.println(f);

        double d = l;
        System.out.println(d);

        // From floating point to long
        double fromDouble = 1.5e15;
        long backToLong = (long)fromDouble;
        System.out.println(backToLong);
    }

    static void testLongBitwise() {
        long a = 0x00000000FFFFFFFFL; // 4294967295
        long b = 0x0F0F0F0F0F0F0F0FL; // 1085102592571150095

        // Bitwise AND
        long and = a & b;
        System.out.println(and);

        // Bitwise OR
        long or = a | b;
        System.out.println(or);

        // Bitwise XOR
        long xor = a ^ b;
        System.out.println(xor);

        // Bitwise NOT
        long not = ~a;
        System.out.println(not);
    }

    static void testLongShift() {
        long val = 1000000000000L;

        // Left shift
        long leftShift = val << 8;
        System.out.println(leftShift);

        // Right shift (arithmetic)
        long rightShift = val >> 8;
        System.out.println(rightShift);

        // Unsigned right shift
        long unsignedRightShift = val >>> 8;
        System.out.println(unsignedRightShift);

        // Test with negative value
        long negVal = -1000000000000L;
        long negRightShift = negVal >> 8;
        System.out.println(negRightShift);

        long negUnsignedRightShift = negVal >>> 8;
        System.out.println(negUnsignedRightShift);

        // Shift by 0
        long shiftZero = val << 0;
        System.out.println(shiftZero);

        // Large shift amount (should mask to 6 bits for long)
        long largeShift = val << 65; // Same as << 1
        System.out.println(largeShift);
    }
}

