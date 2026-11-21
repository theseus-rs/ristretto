public class CanonicalInitializerFaults {
    static class Node { int value; }
    static class Other { }
    static class Negative { static int value = new int[-1].length; }
    static class NegativeReference { static int value = new Object[-1].length; }
    static class Bounds { static int value = fail(); static int fail() { int[] a = new int[1]; return a[1]; } }
    static class BoundsStore { static int value = fail(); static int fail() { int[] a = new int[1]; a[1] = 2; return 0; } }
    static class Store { static int value = fail(); static int fail() { Object[] a = new Node[1]; a[0] = new Other(); return 0; } }
    static class Cast { static int value = fail(); static int fail() { Object a = new Other(); return ((Node) a).value; } }
    static class NullArray { static int value = fail(); static int fail() { int[] a = null; return a.length; } }
    static class NestedNegative { static int value = new int[-1].length; }
    static class Nested { static int value = NestedNegative.value; }
    static class Slow { static int value = sum(); static int sum() { int total = 0; for (int i = 0; i < 4096; i++) total += i; return total; } }
    public static int negative(int ignored) { return Negative.value; }
    public static int negativeReference(int ignored) { return NegativeReference.value; }
    public static int bounds(int ignored) { return Bounds.value; }
    public static int boundsStore(int ignored) { return BoundsStore.value; }
    public static int store(int ignored) { return Store.value; }
    public static int cast(int ignored) { return Cast.value; }
    public static int nullArray(int ignored) { return NullArray.value; }
    public static int nested(int ignored) { return Nested.value; }
    public static int slow(int ignored) { return Slow.value; }
}
