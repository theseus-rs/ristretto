/** Template: patch_narrow_return_fixture.py replaces only four helper Code attributes. */
public final class CanonicalNarrowReturnWorkload {
    static byte byteValue(int value) { return (byte) value; }
    static char charValue(int value) { return (char) value; }
    static short shortValue(int value) { return (short) value; }
    static boolean booleanValue(int value) { return (value & 1) != 0; }

    public static int bytes(int value) { return byteValue(value); }
    public static int chars(int value) { return charValue(value); }
    public static int shorts(int value) { return shortValue(value); }
    public static int booleans(int value) { return booleanValue(value) ? 1 : 0; }
}
