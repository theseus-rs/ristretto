public final class CanonicalEntryInitialization {
    static final int VALUE = prepare();
    static int prepare() { return 19; }
    public static int run(int value) { return VALUE + value; }
    public static int identity(int value) { return value; }
}
