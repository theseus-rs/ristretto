/** Numeric/slot correctness only; no persistent roots or performance claim. */
public final class CanonicalNumericEdgeWorkload {
    static final class WideHolder {
        int sum;
        int[] values;
        WideHolder(long wide, double decimal, int tail, int[] values) {
            this.sum = (int) wide + (int) decimal + tail;
            this.values = values;
        }
    }
    static float pickFloat(int selector) {
        selector &= 15;
        if (selector == 0) return Float.NaN;
        if (selector == 1) return Float.POSITIVE_INFINITY;
        if (selector == 2) return Float.NEGATIVE_INFINITY;
        if (selector == 3) return 0.0f;
        if (selector == 4) return -0.0f;
        if (selector == 5) return Float.MAX_VALUE;
        if (selector == 6) return -Float.MAX_VALUE;
        if (selector == 7) return 2147483520.0f;
        if (selector == 8) return 2147483648.0f;
        if (selector == 9) return -2147483904.0f;
        if (selector == 10) return 0x1.fffffep62f;
        if (selector == 11) return 0x1.0p63f;
        if (selector == 12) return -0x1.0p63f;
        if (selector == 13) return -1.9f;
        if (selector == 14) return 1.9f;
        return Float.MIN_VALUE;
    }
    static double pickDouble(int selector) {
        selector &= 15;
        if (selector == 0) return Double.NaN;
        if (selector == 1) return Double.POSITIVE_INFINITY;
        if (selector == 2) return Double.NEGATIVE_INFINITY;
        if (selector == 3) return 0.0;
        if (selector == 4) return -0.0;
        if (selector == 5) return Double.MAX_VALUE;
        if (selector == 6) return -Double.MAX_VALUE;
        if (selector == 7) return 2147483647.9;
        if (selector == 8) return 2147483648.0;
        if (selector == 9) return -2147483648.9;
        if (selector == 10) return 0x1.fffffffffffffp62;
        if (selector == 11) return 0x1.0p63;
        if (selector == 12) return -0x1.0p63;
        if (selector == 13) return -1.9;
        if (selector == 14) return 1.9;
        return Double.MIN_VALUE;
    }
    static long expectedFloatLong(int selector) {
        selector &= 15;
        if (selector == 0) return 0L;
        if (selector == 1) return Long.MAX_VALUE;
        if (selector == 2) return Long.MIN_VALUE;
        if (selector == 3) return 0L;
        if (selector == 4) return 0L;
        if (selector == 5) return Long.MAX_VALUE;
        if (selector == 6) return Long.MIN_VALUE;
        if (selector == 7) return 2147483520L;
        if (selector == 8) return 2147483648L;
        if (selector == 9) return -2147483904L;
        if (selector == 10) return 9223371487098961920L;
        if (selector == 11) return Long.MAX_VALUE;
        if (selector == 12) return Long.MIN_VALUE;
        if (selector == 13) return -1L;
        if (selector == 14) return 1L;
        return 0L;
    }
    static long expectedDoubleLong(int selector) {
        selector &= 15;
        if (selector == 0) return 0L;
        if (selector == 1) return Long.MAX_VALUE;
        if (selector == 2) return Long.MIN_VALUE;
        if (selector == 3) return 0L;
        if (selector == 4) return 0L;
        if (selector == 5) return Long.MAX_VALUE;
        if (selector == 6) return Long.MIN_VALUE;
        if (selector == 7) return 2147483647L;
        if (selector == 8) return 2147483648L;
        if (selector == 9) return -2147483648L;
        if (selector == 10) return 9223372036854774784L;
        if (selector == 11) return Long.MAX_VALUE;
        if (selector == 12) return Long.MIN_VALUE;
        if (selector == 13) return -1L;
        if (selector == 14) return 1L;
        return 0L;
    }
    static float floatIdentity(float value) { return value; }
    static double doubleIdentity(double value) { return value; }
    static float[] floatArray(int selector) {
        float[] values = new float[1];
        values[0] = pickFloat(selector);
        return values;
    }
    static double[] doubleArray(int selector) {
        double[] values = new double[1];
        values[0] = pickDouble(selector);
        return values;
    }
    public static int floatToInt(int selector) {
        return (int) floatIdentity(floatArray(selector)[0]);
    }
    public static int doubleToInt(int selector) {
        return (int) doubleIdentity(doubleArray(selector)[0]);
    }
    public static int floatToLong(int selector) {
        long actual = (long) floatIdentity(floatArray(selector)[0]);
        return actual == expectedFloatLong(selector) ? 1 : -1;
    }
    public static int doubleToLong(int selector) {
        long actual = (long) doubleIdentity(doubleArray(selector)[0]);
        return actual == expectedDoubleLong(selector) ? 1 : -1;
    }
    public static int floatCompare(int selector) {
        float value = floatIdentity(floatArray(selector)[0]);
        int result = 0;
        if (value < 0.0f) result |= 1;
        if (value > 0.0f) result |= 2;
        if (value == 0.0f) result |= 4;
        if (value != value) result |= 8;
        return result;
    }
    public static int doubleCompare(int selector) {
        double value = doubleIdentity(doubleArray(selector)[0]);
        int result = 0;
        if (value < 0.0) result |= 1;
        if (value > 0.0) result |= 2;
        if (value == 0.0) result |= 4;
        if (value != value) result |= 8;
        return result;
    }
    static long mixedLong(int first, long wide, int middle, double decimal, int tail, int[] values) {
        if (wide != 0x100000000L + first || middle != 7 || tail != 11
                || decimal != (double) first + 0.25) return -9999999999L;
        values[0] = values[0] + first;
        return wide + middle + tail + (long) decimal;
    }
    public static int mixedSlots(int value) {
        int[] values = new int[1];
        values[0] = value;
        int part = value & 31;
        long result = mixedLong(part, 0x100000000L + part, 7, (double) part + 0.25, 11, values);
        return (int) result + values[0];
    }
    public static int constructorSlots(int value) {
        int[] values = new int[1];
        values[0] = value;
        int part = value & 31;
        WideHolder holder = new WideHolder(0x100000000L + part, (double) part + 0.25, 9, values);
        return holder.sum + holder.values[0];
    }
}
