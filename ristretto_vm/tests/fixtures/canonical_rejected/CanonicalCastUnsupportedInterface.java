public class CanonicalCastUnsupportedInterface {
    public static int run(int ignored) { Object value = new int[1]; return (Cloneable) value == null ? 0 : 1; }
}
