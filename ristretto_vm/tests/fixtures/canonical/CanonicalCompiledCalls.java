public final class CanonicalCompiledCalls {
    private static long wide(long left, int right) { return left + right; }
    private static int twice(int value) { return value * 2; }
    public static int run(int rounds) {
        int sum = 0;
        for (int i = 0; i < rounds; i++) sum += twice(i);
        return sum + (int) wide(1L, rounds);
    }
    public static int depth(int value) {
        if (value <= 0) return 0;
        return depth(value - 1) + 1;
    }
}
