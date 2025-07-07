public class Test {
    public static void main(String[] args) {
        // Test all int arithmetic operations with bounds checking

        testIntAdd();
        testIntSub();
        testIntMul();
        testIntDiv();
        testIntRem();
        testIntNeg();
        testIntInc();
        testIntCompare();
        testIntConversions();
        testIntBitwise();
        testIntShift();
    }

    static void testIntAdd() {
        int a = 1000000;
        int b = 500000;
        int result = a + b;
        System.out.println(result);

        // Overflow test
        int max = 2147483647;
        int one = 1;
        int overflow = max + one;
        System.out.println(overflow);

        // Underflow test
        int min = -2147483648;
        int negOne = -1;
        int underflow = min + negOne;
        System.out.println(underflow);
    }

    static void testIntSub() {
        int a = 1000000;
        int b = 300000;
        int result = a - b;
        System.out.println(result);

        // Underflow test
        int min = -2147483648;
        int one = 1;
        int underflow = min - one;
        System.out.println(underflow);

        // Overflow test
        int max = 2147483647;
        int negOne = -1;
        int overflow = max - negOne;
        System.out.println(overflow);
    }

    static void testIntMul() {
        int a = 10000;
        int b = 5000;
        int result = a * b;
        System.out.println(result);

        // Overflow test
        int max = 2147483647;
        int two = 2;
        int overflow = max * two;
        System.out.println(overflow);

        // Zero test
        int zero = 0;
        int zeroResult = a * zero;
        System.out.println(zeroResult);
    }

    static void testIntDiv() {
        int a = 1000000;
        int b = 250;
        int result = a / b;
        System.out.println(result);

        // Division by one
        int one = 1;
        int divByOne = a / one;
        System.out.println(divByOne);

        // Negative division
        int neg = -250;
        int negDiv = a / neg;
        System.out.println(negDiv);

        // Min value division
        int min = -2147483648;
        int negativeOne = -1;
        int minDiv = min / negativeOne;
        System.out.println(minDiv);
    }

    static void testIntRem() {
        int a = 1000000;
        int b = 333;
        int result = a % b;
        System.out.println(result);

        // Remainder with negative
        int neg = -333;
        int negRem = a % neg;
        System.out.println(negRem);

        // Negative dividend
        int negA = -1000000;
        int negDividend = negA % b;
        System.out.println(negDividend);
    }

    static void testIntNeg() {
        int positive = 1000000;
        int negResult = -positive;
        System.out.println(negResult);

        int negative = -1000000;
        int posResult = -negative;
        System.out.println(posResult);

        // Min value negation (overflow)
        int min = -2147483648;
        int minNeg = -min;
        System.out.println(minNeg);

        int zero = 0;
        int zeroNeg = -zero;
        System.out.println(zeroNeg);
    }

    static void testIntInc() {
        int val = 1000000;
        val++;
        System.out.println(val);

        // Increment at boundary
        int max = 2147483646;
        max++;
        System.out.println(max);

        // Overflow increment
        int maxVal = 2147483647;
        maxVal++;
        System.out.println(maxVal);
    }

    static void testIntCompare() {
        int a = 1000000;
        int b = 500000;
        int c = 1000000;

        System.out.println(a > b ? 1 : 0);
        System.out.println(a < b ? 1 : 0);
        System.out.println(a == c ? 1 : 0);
        System.out.println(a != b ? 1 : 0);
        System.out.println(a >= c ? 1 : 0);
        System.out.println(a <= b ? 1 : 0);
    }

    static void testIntConversions() {
        int i = 100000;

        // Int to smaller types (explicit cast)
        byte b = (byte)i;
        System.out.println((int)b);

        short s = (short)i;
        System.out.println((int)s);

        // Int to larger types
        long l = i;
        System.out.println(l);

        float f = i;
        System.out.println(f);

        double d = i;
        System.out.println(d);

        // From other types to int
        long fromLong = 5000000000L; // Will overflow
        int backToInt = (int)fromLong;
        System.out.println(backToInt);
    }

    static void testIntBitwise() {
        int a = 0x0000FFFF; // 65535
        int b = 0x0F0F0F0F; // 252645135

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

    static void testIntShift() {
        int val = 100000;

        // Left shift
        int leftShift = val << 4;
        System.out.println(leftShift);

        // Right shift (arithmetic)
        int rightShift = val >> 4;
        System.out.println(rightShift);

        // Unsigned right shift
        int unsignedRightShift = val >>> 4;
        System.out.println(unsignedRightShift);

        // Test with negative value
        int negVal = -100000;
        int negRightShift = negVal >> 4;
        System.out.println(negRightShift);

        int negUnsignedRightShift = negVal >>> 4;
        System.out.println(negUnsignedRightShift);

        // Shift by 0
        int shiftZero = val << 0;
        System.out.println(shiftZero);

        // Large shift amount (should mask to 5 bits)
        int largeShift = val << 33; // Same as << 1
        System.out.println(largeShift);
    }
}

