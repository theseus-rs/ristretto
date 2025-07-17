/** Test primitive type conversions and their class types. */
public class Test {
    public static void main(String[] args) {
        // Test all primitive type conversions with bounds checking
        System.out.println("=== Primitive Type Conversions ===");
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
    }

    static void testIntToByte() {
        System.out.println("--- Int to Byte ---");
        int[] values = {0, 127, 128, -128, -129, Integer.MAX_VALUE, Integer.MIN_VALUE};
        for (int value : values) {
            System.out.println(value + " -> " + (byte)value);
        }
    }

    static void testIntToShort() {
        System.out.println("--- Int to Short ---");
        int[] values = {0, 32767, 32768, -32768, -32769, Integer.MAX_VALUE, Integer.MIN_VALUE};
        for (int value : values) {
            System.out.println(value + " -> " + (short)value);
        }
    }

    static void testIntToChar() {
        System.out.println("--- Int to Char ---");
        int[] values = {0, 65535, 65536, -1, Integer.MAX_VALUE, Integer.MIN_VALUE};
        for (int value : values) {
            System.out.println(value + " -> " + (int)(char)value);
        }
    }

    static void testIntToLong() {
        System.out.println("--- Int to Long ---");
        int[] values = {0, Integer.MAX_VALUE, Integer.MIN_VALUE};
        for (int value : values) {
            System.out.println(value + " -> " + (long)value);
        }
    }

    static void testIntToFloat() {
        System.out.println("--- Int to Float ---");
        int[] values = {0, Integer.MAX_VALUE, Integer.MIN_VALUE};
        for (int value : values) {
            System.out.println(value + " -> " + (float)value);
        }
    }

    static void testIntToDouble() {
        System.out.println("--- Int to Double ---");
        int[] values = {0, Integer.MAX_VALUE, Integer.MIN_VALUE};
        for (int value : values) {
            System.out.println(value + " -> " + (double)value);
        }
    }

    static void testLongToInt() {
        System.out.println("--- Long to Int ---");
        long[] values = {0L, Integer.MAX_VALUE, (long)Integer.MAX_VALUE + 1, Integer.MIN_VALUE, (long)Integer.MIN_VALUE - 1, Long.MAX_VALUE, Long.MIN_VALUE};
        for (long value : values) {
            System.out.println(value + " -> " + (int)value);
        }
    }

    static void testLongToFloat() {
        System.out.println("--- Long to Float ---");
        long[] values = {0L, Long.MAX_VALUE, Long.MIN_VALUE};
        for (long value : values) {
            System.out.println(value + " -> " + (float)value);
        }
    }

    static void testLongToDouble() {
        System.out.println("--- Long to Double ---");
        long[] values = {0L, Long.MAX_VALUE, Long.MIN_VALUE};
        for (long value : values) {
            System.out.println(value + " -> " + (double)value);
        }
    }

    static void testFloatToInt() {
        System.out.println("--- Float to Int ---");
        float[] values = {0.0f, 1.5f, -1.5f, Float.MAX_VALUE, Float.MIN_VALUE, Float.POSITIVE_INFINITY, Float.NEGATIVE_INFINITY, Float.NaN};
        for (float value : values) {
            System.out.println(value + " -> " + (int)value);
        }
    }

    static void testFloatToLong() {
        System.out.println("--- Float to Long ---");
        float[] values = {0.0f, 1.5f, -1.5f, Float.MAX_VALUE, Float.MIN_VALUE, Float.POSITIVE_INFINITY, Float.NEGATIVE_INFINITY, Float.NaN};
        for (float value : values) {
            System.out.println(value + " -> " + (long)value);
        }
    }

    static void testFloatToDouble() {
        System.out.println("--- Float to Double ---");
        float[] values = {0.0f, 1.5f, -1.5f, Float.MAX_VALUE, Float.MIN_VALUE, Float.POSITIVE_INFINITY, Float.NEGATIVE_INFINITY, Float.NaN};
        for (float value : values) {
            System.out.println(value + " -> " + (double)value);
        }
    }

    static void testDoubleToInt() {
        System.out.println("--- Double to Int ---");
        double[] values = {0.0d, 1.5d, -1.5d, Double.MAX_VALUE, Double.MIN_VALUE, Double.POSITIVE_INFINITY, Double.NEGATIVE_INFINITY, Double.NaN};
        for (double value : values) {
            System.out.println(value + " -> " + (int)value);
        }
    }

    static void testDoubleToLong() {
        System.out.println("--- Double to Long ---");
        double[] values = {0.0d, 1.5d, -1.5d, Double.MAX_VALUE, Double.MIN_VALUE, Double.POSITIVE_INFINITY, Double.NEGATIVE_INFINITY, Double.NaN};
        for (double value : values) {
            System.out.println(value + " -> " + (long)value);
        }
    }

    static void testDoubleToFloat() {
        System.out.println("--- Double to Float ---");
        double[] values = {0.0d, 1.5d, -1.5d, Double.MAX_VALUE, Double.MIN_VALUE, Double.POSITIVE_INFINITY, Double.NEGATIVE_INFINITY, Double.NaN};
        for (double value : values) {
            System.out.println(value + " -> " + (float)value);
        }
    }

    static void testByteToInt() {
        System.out.println("--- Byte to Int ---");
        byte[] values = {0, Byte.MAX_VALUE, Byte.MIN_VALUE};
        for (byte value : values) {
            System.out.println(value + " -> " + (int)value);
        }
    }

    static void testByteToLong() {
        System.out.println("--- Byte to Long ---");
        byte[] values = {0, Byte.MAX_VALUE, Byte.MIN_VALUE};
        for (byte value : values) {
            System.out.println(value + " -> " + (long)value);
        }
    }

    static void testByteToFloat() {
        System.out.println("--- Byte to Float ---");
        byte[] values = {0, Byte.MAX_VALUE, Byte.MIN_VALUE};
        for (byte value : values) {
            System.out.println(value + " -> " + (float)value);
        }
    }

    static void testByteToDouble() {
        System.out.println("--- Byte to Double ---");
        byte[] values = {0, Byte.MAX_VALUE, Byte.MIN_VALUE};
        for (byte value : values) {
            System.out.println(value + " -> " + (double)value);
        }
    }

    static void testShortToInt() {
        System.out.println("--- Short to Int ---");
        short[] values = {0, Short.MAX_VALUE, Short.MIN_VALUE};
        for (short value : values) {
            System.out.println(value + " -> " + (int)value);
        }
    }

    static void testShortToLong() {
        System.out.println("--- Short to Long ---");
        short[] values = {0, Short.MAX_VALUE, Short.MIN_VALUE};
        for (short value : values) {
            System.out.println(value + " -> " + (long)value);
        }
    }

    static void testShortToFloat() {
        System.out.println("--- Short to Float ---");
        short[] values = {0, Short.MAX_VALUE, Short.MIN_VALUE};
        for (short value : values) {
            System.out.println(value + " -> " + (float)value);
        }
    }

    static void testShortToDouble() {
        System.out.println("--- Short to Double ---");
        short[] values = {0, Short.MAX_VALUE, Short.MIN_VALUE};
        for (short value : values) {
            System.out.println(value + " -> " + (double)value);
        }
    }

    static void testCharToInt() {
        System.out.println("--- Char to Int ---");
        char[] values = {0, 'A', Character.MAX_VALUE, Character.MIN_VALUE};
        for (char value : values) {
            System.out.println((int)value + " -> " + (int)value);
        }
    }

    static void testCharToLong() {
        System.out.println("--- Char to Long ---");
        char[] values = {0, 'A', Character.MAX_VALUE, Character.MIN_VALUE};
        for (char value : values) {
            System.out.println((int)value + " -> " + (long)value);
        }
    }

    static void testCharToFloat() {
        System.out.println("--- Char to Float ---");
        char[] values = {0, 'A', Character.MAX_VALUE, Character.MIN_VALUE};
        for (char value : values) {
            System.out.println((int)value + " -> " + (float)value);
        }
    }

    static void testCharToDouble() {
        System.out.println("--- Char to Double ---");
        char[] values = {0, 'A', Character.MAX_VALUE, Character.MIN_VALUE};
        for (char value : values) {
            System.out.println((int)value + " -> " + (double)value);
        }
    }
}
