/** Ordinary javac bytecode for explicit experimental preparation lifetime checks. */
public final class ExperimentalJitLifecycle {
    static CanonicalGraphWorkload.Node observed;
    static int release;

    public static int hold() {
        CanonicalGraphWorkload.Node live = new CanonicalGraphWorkload.Node(73);
        observed = live;
        while (release == 0) { }
        return live.value;
    }

    public static int nullRead() {
        CanonicalGraphWorkload.Node missing = null;
        return missing.value;
    }

    public static int propagate() { return nullRead(); }

    public static int catchNull() {
        try { return nullRead(); }
        catch (NullPointerException expected) { return 19; }
    }

    public static synchronized int synchronizedMethod() { return 1; }
    public static int arrayRead(int[] values) { return values[0]; }
}
