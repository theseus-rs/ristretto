public class Test {
    public static void main(String[] args) {
        // Test all primitive type conversions with bounds checking
        testIntToByte();
        testIntToShort();
        testIntToChar();
        testIntToLong();
        testIntToFloat();
        testIntToDouble();

        testLongToInt();
        testLongToFloat();
        testLongToDouble();

        testFloatToInt();
        testFloatToLong();
        testFloatToDouble();

        testDoubleToInt();
        testDoubleToLong();
        testDoubleToFloat();

        testByteToInt();
        testByteToLong();
        testByteToFloat();
        testByteToDouble();

        testShortToInt();
        testShortToLong();
        testShortToFloat();
        testShortToDouble();

        testCharToInt();
        testCharToLong();
        testCharToFloat();
        testCharToDouble();

        testBoundsChecking();
    }

    // INT to other types (i2b, i2s, i2c, i2l, i2f, i2d)
    public static void testIntToByte() {
        int value = 300;
        byte result = (byte) value;
        System.out.println(result);

        value = 127;
        result = (byte) value;
        System.out.println(result);

        value = -129;
        result = (byte) value;
        System.out.println(result);
    }

    public static void testIntToShort() {
        int value = 70000;
        short result = (short) value;
        System.out.println(result);

        value = 32767;
        result = (short) value;
        System.out.println(result);

        value = -32769;
        result = (short) value;
        System.out.println(result);
    }

    public static void testIntToChar() {
        int value = 65;
        char result = (char) value;
        System.out.println((int) result);

        value = -1;
        result = (char) value;
        System.out.println((int) result);

        value = 70000;
        result = (char) value;
        System.out.println((int) result);
    }

    public static void testIntToLong() {
        int value = 2147483647;
        long result = (long) value;
        System.out.println(result);

        value = -2147483648;
        result = (long) value;
        System.out.println(result);
    }

    public static void testIntToFloat() {
        int value = 2147483647;
        float result = (float) value;
        System.out.println(result);

        value = 16777217;
        result = (float) value;
        System.out.println(result);
    }

    public static void testIntToDouble() {
        int value = 2147483647;
        double result = (double) value;
        System.out.println(result);

        value = -2147483648;
        result = (double) value;
        System.out.println(result);
    }

    // LONG to other types (l2i, l2f, l2d)
    public static void testLongToInt() {
        long value = 2147483648L;
        int result = (int) value;
        System.out.println(result);

        value = -2147483649L;
        result = (int) value;
        System.out.println(result);

        value = 2147483647L;
        result = (int) value;
        System.out.println(result);
    }

    public static void testLongToFloat() {
        long value = 9223372036854775807L;
        float result = (float) value;
        System.out.println(result);

        value = 16777217L;
        result = (float) value;
        System.out.println(result);
    }

    public static void testLongToDouble() {
        long value = 9223372036854775807L;
        double result = (double) value;
        System.out.println(result);

        value = 9007199254740993L;
        result = (double) value;
        System.out.println(result);
    }

    // FLOAT to other types (f2i, f2l, f2d)
    public static void testFloatToInt() {
        float value = 2.5f;
        int result = (int) value;
        System.out.println(result);

        value = 2147483648.0f;
        result = (int) value;
        System.out.println(result);

        value = -2147483649.0f;
        result = (int) value;
        System.out.println(result);

        value = Float.NaN;
        result = (int) value;
        System.out.println(result);

        value = Float.POSITIVE_INFINITY;
        result = (int) value;
        System.out.println(result);

        value = Float.NEGATIVE_INFINITY;
        result = (int) value;
        System.out.println(result);
    }

    public static void testFloatToLong() {
        float value = 2.5f;
        long result = (long) value;
        System.out.println(result);

        value = 9.223372e18f;
        result = (long) value;
        System.out.println(result);

        value = -9.223372e18f;
        result = (long) value;
        System.out.println(result);

        value = Float.NaN;
        result = (long) value;
        System.out.println(result);

        value = Float.POSITIVE_INFINITY;
        result = (long) value;
        System.out.println(result);

        value = Float.NEGATIVE_INFINITY;
        result = (long) value;
        System.out.println(result);
    }

    public static void testFloatToDouble() {
        float value = 3.14159f;
        double result = (double) value;
        System.out.println(result);

        value = Float.MAX_VALUE;
        result = (double) value;
        System.out.println(result);

        value = Float.NaN;
        result = (double) value;
        System.out.println(Double.isNaN(result));

        value = Float.POSITIVE_INFINITY;
        result = (double) value;
        System.out.println(Double.isInfinite(result) && result > 0);
    }

    // DOUBLE to other types (d2i, d2l, d2f)
    public static void testDoubleToInt() {
        double value = 2.7;
        int result = (int) value;
        System.out.println(result);

        value = 2147483648.0;
        result = (int) value;
        System.out.println(result);

        value = -2147483649.0;
        result = (int) value;
        System.out.println(result);

        value = Double.NaN;
        result = (int) value;
        System.out.println(result);

        value = Double.POSITIVE_INFINITY;
        result = (int) value;
        System.out.println(result);

        value = Double.NEGATIVE_INFINITY;
        result = (int) value;
        System.out.println(result);
    }

    public static void testDoubleToLong() {
        double value = 2.7;
        long result = (long) value;
        System.out.println(result);

        value = 9.223372036854776e18;
        result = (long) value;
        System.out.println(result);

        value = -9.223372036854776e18;
        result = (long) value;
        System.out.println(result);

        value = Double.NaN;
        result = (long) value;
        System.out.println(result);

        value = Double.POSITIVE_INFINITY;
        result = (long) value;
        System.out.println(result);

        value = Double.NEGATIVE_INFINITY;
        result = (long) value;
        System.out.println(result);
    }

    public static void testDoubleToFloat() {
        double value = 3.141592653589793;
        float result = (float) value;
        System.out.println(result);

        value = 1.7976931348623157e308;
        result = (float) value;
        System.out.println(Float.isInfinite(result));

        value = Double.MIN_VALUE;
        result = (float) value;
        System.out.println(result);

        value = Double.NaN;
        result = (float) value;
        System.out.println(Float.isNaN(result));
    }

    // BYTE to other types (implicit conversions - no specific bytecode instructions)
    public static void testByteToInt() {
        byte value = 127;
        int result = value;
        System.out.println(result);

        value = -128;
        result = value;
        System.out.println(result);
    }

    public static void testByteToLong() {
        byte value = 127;
        long result = value;
        System.out.println(result);

        value = -128;
        result = value;
        System.out.println(result);
    }

    public static void testByteToFloat() {
        byte value = 127;
        float result = value;
        System.out.println(result);

        value = -128;
        result = value;
        System.out.println(result);
    }

    public static void testByteToDouble() {
        byte value = 127;
        double result = value;
        System.out.println(result);

        value = -128;
        result = value;
        System.out.println(result);
    }

    // SHORT to other types (implicit conversions - no specific bytecode instructions)
    public static void testShortToInt() {
        short value = 32767;
        int result = value;
        System.out.println(result);

        value = -32768;
        result = value;
        System.out.println(result);
    }

    public static void testShortToLong() {
        short value = 32767;
        long result = value;
        System.out.println(result);

        value = -32768;
        result = value;
        System.out.println(result);
    }

    public static void testShortToFloat() {
        short value = 32767;
        float result = value;
        System.out.println(result);

        value = -32768;
        result = value;
        System.out.println(result);
    }

    public static void testShortToDouble() {
        short value = 32767;
        double result = value;
        System.out.println(result);

        value = -32768;
        result = value;
        System.out.println(result);
    }

    // CHAR to other types (implicit conversions - no specific bytecode instructions)
    public static void testCharToInt() {
        char value = 'A';
        int result = value;
        System.out.println(result);

        value = '\u0000';
        result = value;
        System.out.println(result);

        value = '\uffff';
        result = value;
        System.out.println(result);
    }

    public static void testCharToLong() {
        char value = 'A';
        long result = value;
        System.out.println(result);

        value = '\uffff';
        result = value;
        System.out.println(result);
    }

    public static void testCharToFloat() {
        char value = 'A';
        float result = value;
        System.out.println(result);

        value = '\uffff';
        result = value;
        System.out.println(result);
    }

    public static void testCharToDouble() {
        char value = 'A';
        double result = value;
        System.out.println(result);

        value = '\uffff';
        result = value;
        System.out.println(result);
    }

    // Additional bounds checking tests
    public static void testBoundsChecking() {
        // Test extreme values for each conversion

        // Test int to byte with extreme values
        int intMax = Integer.MAX_VALUE;
        byte byteFromIntMax = (byte) intMax;
        System.out.println(byteFromIntMax);

        int intMin = Integer.MIN_VALUE;
        byte byteFromIntMin = (byte) intMin;
        System.out.println(byteFromIntMin);

        // Test long to int with extreme values
        long longMax = Long.MAX_VALUE;
        int intFromLongMax = (int) longMax;
        System.out.println(intFromLongMax);

        long longMin = Long.MIN_VALUE;
        int intFromLongMin = (int) longMin;
        System.out.println(intFromLongMin);

        // Test float/double special values
        float floatNaN = Float.NaN;
        int intFromNaN = (int) floatNaN;
        System.out.println(intFromNaN);

        double doubleInf = Double.POSITIVE_INFINITY;
        long longFromInf = (long) doubleInf;
        System.out.println(longFromInf);

        double doubleNegInf = Double.NEGATIVE_INFINITY;
        long longFromNegInf = (long) doubleNegInf;
        System.out.println(longFromNegInf);

        // Test precision loss in float/double conversions
        long precisionTest = 9007199254740993L;
        float floatFromLong = (float) precisionTest;
        long longFromFloat = (long) floatFromLong;
        System.out.println(longFromFloat != precisionTest);

        double doubleFromLong = (double) precisionTest;
        long longFromDouble = (long) doubleFromLong;
        System.out.println(longFromDouble == precisionTest);
    }
}

