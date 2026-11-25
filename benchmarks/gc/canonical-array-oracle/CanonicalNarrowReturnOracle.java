/** Peer-only driver for the patched, verifier-valid ireturn fixture. */
public final class CanonicalNarrowReturnOracle {
    public static void main(String[] arguments) {
        for (int value : new int[] { 2, 3, 257, -1, 65535, 128, -129, 32768, Integer.MIN_VALUE, Integer.MAX_VALUE }) {
            emit("bytes", value, CanonicalNarrowReturnWorkload.bytes(value), (byte) value);
            emit("chars", value, CanonicalNarrowReturnWorkload.chars(value), (char) value);
            emit("shorts", value, CanonicalNarrowReturnWorkload.shorts(value), (short) value);
            emit("booleans", value, CanonicalNarrowReturnWorkload.booleans(value), value & 1);
        }
    }
    static void emit(String method, int argument, int actual, int expected) {
        if (actual != expected) throw new AssertionError(method + ":" + argument + ":" + actual);
        System.out.println("{\"method\":\"" + method + "\",\"argument\":" + argument
            + ",\"status\":\"ok\",\"result\":" + actual + ",\"admitted\":true}");
    }
}
