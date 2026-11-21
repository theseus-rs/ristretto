public class CanonicalCastUnsupportedDimensions {
    public static int run(int ignored) { Object value = new Object[1]; return ((Object[][]) value).length; }
}
