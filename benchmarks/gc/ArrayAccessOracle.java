import java.lang.reflect.Array;
import java.lang.reflect.InvocationTargetException;
import java.lang.reflect.Method;

/** Independent oracle for the exact exported array-access bytecode. No timings. */
public final class ArrayAccessOracle {
    private static int checks;
    private static void check(boolean condition, String label) {
        if (!condition) throw new AssertionError(label);
        checks++;
    }
    private static void same(Object actual, Object expected, String label) {
        if (expected instanceof Float) {
            check(Float.floatToRawIntBits((Float) actual) == Float.floatToRawIntBits((Float) expected), label);
        } else if (expected instanceof Double) {
            check(Double.doubleToRawLongBits((Double) actual) == Double.doubleToRawLongBits((Double) expected), label);
        } else check(expected == null ? actual == null : expected.equals(actual), label);
    }
    private static void failure(Method method, Class<?> expected, Object... arguments) throws Exception {
        try {
            method.invoke(null, arguments);
            throw new AssertionError("missing " + expected + " from " + method);
        } catch (InvocationTargetException error) {
            check(error.getCause().getClass() == expected, "wrong exception: " + error.getCause());
        }
    }
    private static Object[] values(char kind) {
        switch (kind) {
            case 'J': return new Object[] {Long.MIN_VALUE, -1L, 0L, Long.MAX_VALUE};
            case 'F': return new Object[] {0.0f, -0.0f, Float.intBitsToFloat(1), Float.POSITIVE_INFINITY, Float.intBitsToFloat(0x7fc01234)};
            case 'D': return new Object[] {0.0d, -0.0d, Double.longBitsToDouble(1), Double.POSITIVE_INFINITY, Double.longBitsToDouble(0x7ff8123456789abcL)};
            case 'A': return new Object[] {null};
            default: return new Object[] {Integer.MIN_VALUE, -65537, -1, 0, 2, 3, 128, 255, 65535, Integer.MAX_VALUE};
        }
    }
    private static Object stored(char kind, Object value) {
        if (!(value instanceof Integer)) return value;
        int number = (Integer) value;
        switch (kind) {
            case 'Z': return number & 1;
            case 'B': return (int) (byte) number;
            case 'C': return (int) (char) number;
            case 'S': return (int) (short) number;
            default: return number;
        }
    }
    public static void main(String[] args) throws Exception {
        Class<?> fixture = Class.forName("ArrayAccessContinuations");
        char[] kinds = {'Z','B','C','S','I','J','F','D','A'};
        Class<?>[] components = {boolean.class, byte.class, char.class, short.class, int.class,
                                 long.class, float.class, double.class, Object.class};
        for (int k = 0; k < kinds.length; k++) {
            char kind = kinds[k];
            Object array = Array.newInstance(components[k], 1);
            Class<?> valueClass = "ZBCSI".indexOf(kind) >= 0 ? int.class : components[k];
            Method length = fixture.getMethod("length_" + kind, array.getClass());
            Method load = fixture.getMethod("load_" + kind, array.getClass(), int.class);
            Method store = kind == 'A' ? null : fixture.getMethod("store_" + kind, array.getClass(), int.class, valueClass);
            Method wide = kind == 'A' ? null : fixture.getMethod("wide_" + kind, array.getClass(), int.class, valueClass);
            same(length.invoke(null, array), 1, "length");
            failure(length, NullPointerException.class, (Object) null);
            for (Object value : values(kind)) {
                if (wide != null) same(wide.invoke(null, array, 0, value), value, "wide " + kind);
                same(load.invoke(null, array, 0), stored(kind, value), "loaded " + kind);
            }
            for (int index : new int[] {-1, 1}) {
                failure(load, ArrayIndexOutOfBoundsException.class, array, index);
                if (store != null) {
                    Object before = load.invoke(null, array, 0);
                    failure(store, ArrayIndexOutOfBoundsException.class, array, index, values(kind)[0]);
                    same(load.invoke(null, array, 0), before, "failed store changed content");
                }
            }
            failure(load, NullPointerException.class, null, -1);
            if (store != null) failure(store, NullPointerException.class, null, -1, values(kind)[0]);
        }
        Method reference = fixture.getMethod("load_A", Object[].class, int.class);
        int[] child = {13};
        check(reference.invoke(null, new Object[] {child}, 0) == child, "reference identity");
        Method caught = fixture.getMethod("caught", int[].class, int.class);
        same(caught.invoke(null, new int[] {11}, 1), -1, "caught bounds");
        same(caught.invoke(null, new int[] {11}, 0), 11, "protected load");
        System.out.println("array-access oracle passed " + checks + " checks");
    }
}
