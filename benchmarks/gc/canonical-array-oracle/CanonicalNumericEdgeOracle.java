/** HotSpot-only oracle: never include this peer driver in Program's class set. */
public final class CanonicalNumericEdgeOracle {
    static final int[] FLOAT_INTS = { 0, Integer.MAX_VALUE, Integer.MIN_VALUE, 0, 0,
        Integer.MAX_VALUE, Integer.MIN_VALUE, 2147483520, Integer.MAX_VALUE, Integer.MIN_VALUE,
        Integer.MAX_VALUE, Integer.MAX_VALUE, Integer.MIN_VALUE, -1, 1, 0 };
    static final int[] DOUBLE_INTS = { 0, Integer.MAX_VALUE, Integer.MIN_VALUE, 0, 0,
        Integer.MAX_VALUE, Integer.MIN_VALUE, Integer.MAX_VALUE, Integer.MAX_VALUE, Integer.MIN_VALUE,
        Integer.MAX_VALUE, Integer.MAX_VALUE, Integer.MIN_VALUE, -1, 1, 0 };
    static final int[] COMPARES = { 8, 2, 1, 4, 4, 2, 1, 2, 2, 1, 2, 2, 1, 1, 2, 2 };
    static int invoke(String method, int value) {
        switch (method) {
            case "floatToInt": return CanonicalNumericEdgeWorkload.floatToInt(value);
            case "doubleToInt": return CanonicalNumericEdgeWorkload.doubleToInt(value);
            case "floatToLong": return CanonicalNumericEdgeWorkload.floatToLong(value);
            case "doubleToLong": return CanonicalNumericEdgeWorkload.doubleToLong(value);
            case "floatCompare": return CanonicalNumericEdgeWorkload.floatCompare(value);
            case "doubleCompare": return CanonicalNumericEdgeWorkload.doubleCompare(value);
            case "mixedSlots": return CanonicalNumericEdgeWorkload.mixedSlots(value);
            case "constructorSlots": return CanonicalNumericEdgeWorkload.constructorSlots(value);
            default: throw new AssertionError(method);
        }
    }
    static int expected(String method, int value) {
        switch (method) {
            case "floatToInt": return FLOAT_INTS[value & 15];
            case "doubleToInt": return DOUBLE_INTS[value & 15];
            case "floatToLong": case "doubleToLong": return 1;
            case "floatCompare": case "doubleCompare": return COMPARES[value & 15];
            case "mixedSlots": return value + 3 * (value & 31) + 18;
            case "constructorSlots": return value + 2 * (value & 31) + 9;
            default: throw new AssertionError(method);
        }
    }
    static void check(String method, int argument) {
        int result = invoke(method, argument);
        if (result != expected(method, argument)) throw new AssertionError(method + ":" + argument + ":" + result);
        System.out.println("{\"method\":\"" + method + "\",\"argument\":" + argument
            + ",\"status\":\"ok\",\"result\":" + result + ",\"admitted\":true}");
    }
    public static void main(String[] arguments) {
        for (String method : new String[] { "floatToInt", "doubleToInt", "floatToLong", "doubleToLong", "floatCompare", "doubleCompare" }) {
            for (int selector = 0; selector < 16; selector++) check(method, selector);
        }
        for (String method : new String[] { "mixedSlots", "constructorSlots" }) {
            for (int value : new int[] { Integer.MIN_VALUE, -513, -1, 0, 1, 17, 31, 32, Integer.MAX_VALUE }) check(method, value);
        }
        for (int value : new int[] { Integer.MIN_VALUE, -1, 0, 1, Integer.MAX_VALUE }) {
            if (CanonicalUnsupportedPop2.run(value) != value || CanonicalUnsupportedDup2.run(value) != value)
                throw new AssertionError("unsupported fixtures must still be valid Java");
        }
    }
}
