/** Valid Java cast mismatch, executed by the later closed-layout cast tier. */
public final class CanonicalPrimitiveArrayTypeFailure {
    public static int run(int ignored) {
        Object values = new int[1];
        return ((long[]) values).length;
    }
}
