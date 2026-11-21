public final class CanonicalIntObject {
    int value;
    public CanonicalIntObject(int value) { this.value = value; }
    public static int read(int value) { return new CanonicalIntObject(value).value; }
    public static int fault(int ignored) { CanonicalIntObject absent = null; return absent.value; }
}
