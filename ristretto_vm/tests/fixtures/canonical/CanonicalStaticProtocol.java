/** Initialization triggers, recursion, once-only effects and Java failure propagation. */
public final class CanonicalStaticProtocol {
    static final class Journal {
        static int code;
        static void append(int digit) { code = code * 10 + digit; }
    }
    static final class A {
        static int x = prepare();
        static int prepare() { Journal.append(1); int b = B.y; Journal.append(3); return b + 10; }
    }
    static final class B {
        static int y = prepare();
        static int prepare() { Journal.append(2); return A.x + 2; }
    }
    static final class Lazy {
        static int value = prepare();
        static int prepare() { Journal.append(4); return 7; }
        static int read() { return value; }
    }
    static final class Fieldless {
        static { Journal.append(5); }
    }
    static final class Bad {
        static int[] data = new int[size()];
        static int size() { return -1; }
        public static int run(int ignored) { return data.length; }
    }
    static final class NestedBad {
        static int[] data = Bad.data;
    }
    public static int recursive(int ignored) { int x = A.x; return x + A.x + B.y + Journal.code * 100; }
    public static int newTrigger(int ignored) { new Lazy(); return Journal.code; }
    public static int getTrigger(int ignored) { return Lazy.value + Journal.code * 10; }
    public static int putTrigger(int ignored) { Lazy.value = 9; return Lazy.value + Journal.code * 10; }
    public static int callTrigger(int ignored) { int n = Lazy.read(); return n + Journal.code * 10; }
    public static int noTrigger(int ignored) { Lazy[] a = new Lazy[1]; Object n = null; Lazy v = (Lazy)n; return a.length + Journal.code + (v == null ? 0 : 100); }
    public static int fieldless(int ignored) { new Fieldless(); return Journal.code; }
    public static int failure(int ignored) { return Bad.data.length; }
    public static int nestedFailure(int ignored) { return NestedBad.data.length; }
}
