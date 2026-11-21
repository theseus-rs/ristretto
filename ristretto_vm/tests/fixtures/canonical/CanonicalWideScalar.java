public final class CanonicalWideScalar {
    public static long wide(int value) { long local = (long)value * 3000000000L; return local + 17L; }
    public static float single(int value) { return (float)value; }
    public static double fraction(int value) { double local = value * 0.25; return local - 1.0; }
    public static byte narrow(int value) { return (byte)value; }
    public static char unsigned(int value) { return (char)value; }
    public static long saturate(int value) { return (long)(value * 1.0e40); }
}
