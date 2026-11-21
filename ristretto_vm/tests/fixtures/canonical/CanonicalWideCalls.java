public final class CanonicalWideCalls {
    public static int combine(int value) {
        long wide = CanonicalWideScalar.wide(value);
        double fraction = CanonicalWideScalar.fraction(value);
        float single = CanonicalWideScalar.single(value);
        long saturated = CanonicalWideScalar.saturate(value);
        return (int) wide ^ (int) (long) fraction ^ (int) single ^ (int) saturated;
    }
}
