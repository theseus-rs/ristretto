public class Test {
    public static void main(String[] args) {
        // Test all short arithmetic operations with bounds checking

        testShortAdd();
        testShortSub();
        testShortMul();
        testShortDiv();
        testShortRem();
        testShortNeg();
        testShortInc();
        testShortCompare();
        testShortConversions();
        testShortBitwise();
        testShortShift();
    }

    static void testShortAdd() {
        short a = 1000;
        short b = 500;
        int result = a + b;
        System.out.println(result);

        // Overflow test
        short max = 32767;
        short one = 1;
        int overflow = max + one;
        System.out.println(overflow);

        // Underflow test
        short min = -32768;
        short negOne = -1;
        int underflow = min + negOne;
        System.out.println(underflow);
    }

    static void testShortSub() {
        short a = 1000;
        short b = 300;
        int result = a - b;
        System.out.println(result);

        // Underflow test
        short min = -32768;
        short one = 1;
        int underflow = min - one;
        System.out.println(underflow);

        // Overflow test
        short max = 32767;
        short negOne = -1;
        int overflow = max - negOne;
        System.out.println(overflow);
    }

    static void testShortMul() {
        short a = 100;
        short b = 50;
        int result = a * b;
        System.out.println(result);

        // Overflow test
        short max = 32767;
        short two = 2;
        int overflow = max * two;
        System.out.println(overflow);

        // Zero test
        short zero = 0;
        int zeroResult = a * zero;
        System.out.println(zeroResult);
    }

    static void testShortDiv() {
        short a = 1000;
        short b = 25;
        int result = a / b;
        System.out.println(result);

        // Division by one
        short one = 1;
        int divByOne = a / one;
        System.out.println(divByOne);

        // Negative division
        short neg = -25;
        int negDiv = a / neg;
        System.out.println(negDiv);

        // Min value division
        short min = -32768;
        short negativeOne = -1;
        int minDiv = min / negativeOne;
        System.out.println(minDiv);
    }

    static void testShortRem() {
        short a = 1000;
        short b = 33;
        int result = a % b;
        System.out.println(result);

        // Remainder with negative
        short neg = -33;
        int negRem = a % neg;
        System.out.println(negRem);

        // Negative dividend
        short negA = -1000;
        int negDividend = negA % b;
        System.out.println(negDividend);
    }

    static void testShortNeg() {
        short positive = 1000;
        int negResult = -positive;
        System.out.println(negResult);

        short negative = -1000;
        int posResult = -negative;
        System.out.println(posResult);

        // Min value negation (overflow)
        short min = -32768;
        int minNeg = -min;
        System.out.println(minNeg);

        short zero = 0;
        int zeroNeg = -zero;
        System.out.println(zeroNeg);
    }

    static void testShortInc() {
        short val = 1000;
        val++;
        System.out.println((int)val);

        // Increment at boundary
        short max = 32766;
        max++;
        System.out.println((int)max);

        // Overflow increment
        short maxVal = 32767;
        maxVal++;
        System.out.println((int)maxVal);
    }

    static void testShortCompare() {
        short a = 1000;
        short b = 500;
        short c = 1000;

        System.out.println(a > b ? 1 : 0);
        System.out.println(a < b ? 1 : 0);
        System.out.println(a == c ? 1 : 0);
        System.out.println(a != b ? 1 : 0);
        System.out.println(a >= c ? 1 : 0);
        System.out.println(a <= b ? 1 : 0);
    }

    static void testShortConversions() {
        short s = 1000;

        // Short to other types
        byte b = (byte)s; // Explicit cast needed
        System.out.println((int)b);

        int i = s;
        System.out.println(i);

        long l = s;
        System.out.println(l);

        float f = s;
        System.out.println(f);

        double d = s;
        System.out.println(d);

        // From other types to short
        int fromInt = 50000; // Will overflow
        short backToShort = (short)fromInt;
        System.out.println((int)backToShort);
    }

    static void testShortBitwise() {
        short a = 0x00FF; // 255
        short b = 0x0F0F; // 3855

        // Bitwise AND
        int and = a & b;
        System.out.println(and);

        // Bitwise OR
        int or = a | b;
        System.out.println(or);

        // Bitwise XOR
        int xor = a ^ b;
        System.out.println(xor);

        // Bitwise NOT
        int not = ~a;
        System.out.println(not);
    }

    static void testShortShift() {
        short val = 1000;

        // Left shift
        int leftShift = val << 2;
        System.out.println(leftShift);

        // Right shift (arithmetic)
        int rightShift = val >> 2;
        System.out.println(rightShift);

        // Unsigned right shift
        int unsignedRightShift = val >>> 2;
        System.out.println(unsignedRightShift);

        // Test with negative value
        short negVal = -1000;
        int negRightShift = negVal >> 2;
        System.out.println(negRightShift);

        int negUnsignedRightShift = negVal >>> 2;
        System.out.println(negUnsignedRightShift);
    }
}

