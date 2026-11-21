public class CanonicalCastUnsupportedInstanceOf {
    public static int run(int ignored) { Object value = new int[1]; return value instanceof int[] ? 1 : 0; }
}
