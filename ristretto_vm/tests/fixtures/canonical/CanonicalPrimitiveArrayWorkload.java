/** Count-bounded correctness fixture; all public entry points are static (I)I. */
public final class CanonicalPrimitiveArrayWorkload {
    static final class Holder {
        int[] values;
        Holder(int[] values) { this.values = values; }
    }

    public static int booleans(int value) {
        boolean[] values = new boolean[3];
        values[0] = value != 0;
        values[2] = !values[0];
        if (values[1]) return -1000001;
        return (values[0] ? 1 : 0) + (values[2] ? 2 : 0) + values.length;
    }

    public static int bytes(int value) {
        byte[] values = new byte[3];
        values[0] = (byte) value;
        values[2] = (byte) (value + 1);
        if (values[1] != 0) return -1000002;
        return values[0] * 3 + values[2] + values.length;
    }

    public static int chars(int value) {
        char[] values = new char[3];
        values[0] = (char) value;
        values[2] = (char) (value + 1);
        if (values[1] != 0) return -1000003;
        return values[0] * 3 + values[2] + values.length;
    }

    public static int shorts(int value) {
        short[] values = new short[3];
        values[0] = (short) value;
        values[2] = (short) (value + 1);
        if (values[1] != 0) return -1000004;
        return values[0] * 3 + values[2] + values.length;
    }

    public static int ints(int value) {
        int[] values = new int[3];
        for (int index = 0; index < values.length; index++) values[index] = value + index;
        int sum = 0;
        for (int index = 0; index < values.length; index++) sum += values[index];
        return sum;
    }

    public static int longs(int value) {
        long base = 0x1234567800000000L + (long) value;
        long[] values = new long[3];
        values[0] = base;
        values[2] = -base;
        if (values[1] != 0L || values[0] != base || values[2] != -base) return -1000006;
        return (int) values[0] + values.length;
    }

    public static int floats(int value) {
        int scaled = (value & 1023) - 511;
        float first = (float) scaled + 0.25f;
        float[] values = new float[3];
        values[0] = first;
        values[2] = first + 0.5f;
        if (values[1] != 0.0f || values[0] != first || values[2] != first + 0.5f) return -1000007;
        return (int) (values[0] * 4.0f) + (int) (values[2] * 4.0f) + values.length;
    }

    public static int doubles(int value) {
        int scaled = (value & 2047) - 1023;
        double first = (double) scaled + 0.125;
        double[] values = new double[3];
        values[0] = first;
        values[2] = first + 0.25;
        if (values[1] != 0.0 || values[0] != first || values[2] != first + 0.25) return -1000008;
        return (int) (values[0] * 8.0) + (int) (values[2] * 8.0) + values.length;
    }

    static int[] create(int value) {
        int[] values = new int[3];
        values[0] = value;
        values[2] = value + 1;
        return values;
    }

    static int[] identity(int[] values) { return values; }

    static int[] detach(Holder holder) {
        int[] values = holder.values;
        holder.values = null;
        return values;
    }

    public static int returned(int value) {
        int[] first = identity(create(value));
        int[] second = identity(first);
        if (first != second) return -1000009;
        first = null;
        return second[0] + second[2] + second.length;
    }

    public static int fieldReturned(int value) {
        Holder holder = new Holder(create(value));
        int[] values = detach(holder);
        if (holder.values != null) return -1000010;
        return values[0] + values[2] + values.length;
    }

    public static int run(int value) {
        return booleans(value) + bytes(value) + chars(value) + shorts(value)
            + ints(value) + longs(value) + floats(value) + doubles(value)
            + returned(value) + fieldReturned(value);
    }

    public static int failNegative(int length) { return new int[length].length; }
    public static int failBounds(int index) {
        int[] values = new int[2];
        values[0] = 11;
        return values[index];
    }
    public static int failStoreBounds(int index) {
        int[] values = new int[2];
        values[index] = 17;
        return values[0];
    }
    public static int failNull(int index) {
        int[] values = null;
        return values[index];
    }
    public static int failNullLength(int ignored) {
        int[] values = null;
        return values.length;
    }
    public static int failNullStore(int value) {
        int[] values = null;
        values[-1] = value;
        return 0;
    }
}
