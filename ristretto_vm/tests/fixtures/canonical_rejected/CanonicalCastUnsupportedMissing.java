public class CanonicalCastUnsupportedMissing {
    public static int run(int ignored) { Object value = null; return (String) value == null ? 0 : 1; }
}
