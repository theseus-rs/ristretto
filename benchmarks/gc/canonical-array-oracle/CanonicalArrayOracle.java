/** HotSpot-only driver. Never include this class in the canonical Program class set. */
public final class CanonicalArrayOracle {
    private static final String[] METHODS = {
        "booleans", "bytes", "chars", "shorts", "ints", "longs", "floats", "doubles",
        "returned", "fieldReturned", "run"
    };
    private static final int[] ARGUMENTS = {
        -513, -1, 0, 1, 17, 127, 255, 32767, 65535, Integer.MIN_VALUE, Integer.MAX_VALUE
    };

    private static int invoke(String method, int value) {
        switch (method) {
            case "booleans": return CanonicalPrimitiveArrayWorkload.booleans(value);
            case "bytes": return CanonicalPrimitiveArrayWorkload.bytes(value);
            case "chars": return CanonicalPrimitiveArrayWorkload.chars(value);
            case "shorts": return CanonicalPrimitiveArrayWorkload.shorts(value);
            case "ints": return CanonicalPrimitiveArrayWorkload.ints(value);
            case "longs": return CanonicalPrimitiveArrayWorkload.longs(value);
            case "floats": return CanonicalPrimitiveArrayWorkload.floats(value);
            case "doubles": return CanonicalPrimitiveArrayWorkload.doubles(value);
            case "returned": return CanonicalPrimitiveArrayWorkload.returned(value);
            case "fieldReturned": return CanonicalPrimitiveArrayWorkload.fieldReturned(value);
            case "run": return CanonicalPrimitiveArrayWorkload.run(value);
            case "failNegative": return CanonicalPrimitiveArrayWorkload.failNegative(value);
            case "failBounds": return CanonicalPrimitiveArrayWorkload.failBounds(value);
            case "failStoreBounds": return CanonicalPrimitiveArrayWorkload.failStoreBounds(value);
            case "failNull": return CanonicalPrimitiveArrayWorkload.failNull(value);
            case "failNullLength": return CanonicalPrimitiveArrayWorkload.failNullLength(value);
            case "failNullStore": return CanonicalPrimitiveArrayWorkload.failNullStore(value);
            case "rejectedTypeCast": return CanonicalPrimitiveArrayTypeFailure.run(value);
            default: throw new AssertionError(method);
        }
    }

    // Independent scalar formulas; no array operations or calls to fixture helpers.
    private static int expected(String method, int value) {
        switch (method) {
            case "booleans": return value == 0 ? 5 : 4;
            case "bytes": return ((byte) value) * 3 + (byte) (value + 1) + 3;
            case "chars": return ((char) value) * 3 + (char) (value + 1) + 3;
            case "shorts": return ((short) value) * 3 + (short) (value + 1) + 3;
            case "ints": return 3 * value + 3;
            case "longs": return value + 3;
            case "floats": return 8 * ((value & 1023) - 511) + 7;
            case "doubles": return 16 * ((value & 2047) - 1023) + 7;
            case "returned": case "fieldReturned": return 2 * value + 4;
            case "run":
                int sum = 0;
                for (int index = 0; index < METHODS.length - 1; index++) sum += expected(METHODS[index], value);
                return sum;
            default: throw new AssertionError(method);
        }
    }

    private static void success(String method, int argument, int expected) {
        int result = invoke(method, argument);
        if (result != expected) throw new AssertionError(method + "(" + argument + "): " + result + " != " + expected);
        System.out.println("{\"method\":\"" + method + "\",\"argument\":" + argument
            + ",\"status\":\"ok\",\"result\":" + result + ",\"admitted\":true}");
    }

    private static void failure(String method, int argument, Class<?> expected, boolean admitted) {
        try {
            invoke(method, argument);
            throw new AssertionError(method + " unexpectedly returned");
        } catch (RuntimeException exception) {
            if (exception.getClass() != expected) throw new AssertionError(method, exception);
            System.out.println("{\"method\":\"" + method + "\",\"argument\":" + argument
                + ",\"status\":\"java_exception\",\"exception\":\"" + expected.getName()
                + "\",\"admitted\":" + admitted + "}");
        }
    }

    public static void main(String[] ignored) {
        for (String method : METHODS) for (int argument : ARGUMENTS) success(method, argument, expected(method, argument));
        success("failNegative", 0, 0);
        success("failNegative", 2, 2);
        success("failBounds", 0, 11);
        success("failBounds", 1, 0);
        success("failStoreBounds", 0, 17);
        success("failStoreBounds", 1, 0);
        for (int argument : new int[] {-1, -17, Integer.MIN_VALUE}) failure("failNegative", argument, NegativeArraySizeException.class, true);
        for (int argument : new int[] {-1, 2, Integer.MIN_VALUE, Integer.MAX_VALUE}) {
            failure("failBounds", argument, ArrayIndexOutOfBoundsException.class, true);
            failure("failStoreBounds", argument, ArrayIndexOutOfBoundsException.class, true);
        }
        for (int argument : new int[] {-1, 0, Integer.MAX_VALUE}) {
            failure("failNull", argument, NullPointerException.class, true);
            failure("failNullLength", argument, NullPointerException.class, true);
            failure("failNullStore", argument, NullPointerException.class, true);
        }
        failure("rejectedTypeCast", 0, ClassCastException.class, false);
    }
}
