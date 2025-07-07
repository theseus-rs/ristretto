public class Test {
    public static void main(String[] args) {
        // Test all byte arithmetic operations with bounds checking

        // Addition tests
        testByteAdd();

        // Subtraction tests
        testByteSub();

        // Multiplication tests
        testByteMul();

        // Division tests
        testByteDiv();

        // Remainder tests
        testByteRem();

        // Negation tests
        testByteNeg();

        // Increment tests
        testByteInc();

        // Comparison tests
        testByteCompare();

        // Conversion tests
        testByteConversions();
    }

    static void testByteAdd() {
        // Normal addition
        byte a = 50;
        byte b = 25;
        int result = a + b; // byte arithmetic promotes to int
        System.out.println(result);

        // Overflow test
        byte max = 127;
        byte one = 1;
        int overflow = max + one;
        System.out.println(overflow);

        // Underflow test
        byte min = -128;
        byte negOne = -1;
        int underflow = min + negOne;
        System.out.println(underflow);
    }

    static void testByteSub() {
        byte a = 50;
        byte b = 25;
        int result = a - b;
        System.out.println(result);

        // Underflow test
        byte min = -128;
        byte one = 1;
        int underflow = min - one;
        System.out.println(underflow);

        // Overflow test
        byte max = 127;
        byte negOne = -1;
        int overflow = max - negOne;
        System.out.println(overflow);
    }

    static void testByteMul() {
        byte a = 10;
        byte b = 5;
        int result = a * b;
        System.out.println(result);

        // Overflow test
        byte max = 127;
        byte two = 2;
        int overflow = max * two;
        System.out.println(overflow);

        // Zero test
        byte zero = 0;
        int zeroResult = a * zero;
        System.out.println(zeroResult);
    }

    static void testByteDiv() {
        byte a = 50;
        byte b = 5;
        int result = a / b;
        System.out.println(result);

        // Division by one
        byte one = 1;
        int divByOne = a / one;
        System.out.println(divByOne);

        // Negative division
        byte neg = -25;
        int negDiv = a / neg;
        System.out.println(negDiv);

        // Min value division
        byte min = -128;
        byte negativeOne = -1;
        int minDiv = min / negativeOne;
        System.out.println(minDiv);
    }

    static void testByteRem() {
        byte a = 50;
        byte b = 7;
        int result = a % b;
        System.out.println(result);

        // Remainder with negative
        byte neg = -7;
        int negRem = a % neg;
        System.out.println(negRem);

        // Negative dividend
        byte negA = -50;
        int negDividend = negA % b;
        System.out.println(negDividend);
    }

    static void testByteNeg() {
        byte positive = 50;
        int negResult = -positive;
        System.out.println(negResult);

        byte negative = -50;
        int posResult = -negative;
        System.out.println(posResult);

        // Min value negation (overflow)
        byte min = -128;
        int minNeg = -min;
        System.out.println(minNeg);

        byte zero = 0;
        int zeroNeg = -zero;
        System.out.println(zeroNeg);
    }

    static void testByteInc() {
        byte val = 50;
        val++;
        System.out.println((int)val);

        // Increment at boundary
        byte max = 126;
        max++;
        System.out.println((int)max);

        // Overflow increment
        byte maxVal = 127;
        maxVal++;
        System.out.println((int)maxVal);
    }

    static void testByteCompare() {
        byte a = 50;
        byte b = 25;
        byte c = 50;

        System.out.println(a > b ? 1 : 0);
        System.out.println(a < b ? 1 : 0);
        System.out.println(a == c ? 1 : 0);
        System.out.println(a != b ? 1 : 0);
        System.out.println(a >= c ? 1 : 0);
        System.out.println(a <= b ? 1 : 0);
    }

    static void testByteConversions() {
        byte b = 100;

        // Byte to other types
        short s = b;
        System.out.println(s);

        int i = b;
        System.out.println(i);

        long l = b;
        System.out.println(l);

        float f = b;
        System.out.println(f);

        double d = b;
        System.out.println(d);

        // From other types to byte (with explicit cast)
        int fromInt = 200; // Will overflow
        byte backToByte = (byte)fromInt;
        System.out.println((int)backToByte);
    }
}

