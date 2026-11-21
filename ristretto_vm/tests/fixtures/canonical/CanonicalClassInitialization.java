/** Closed class initialization, recursion, errors and handoff. */
public final class CanonicalClassInitialization {
    static final class Node { int value; Node next; Node(int value) { this.value = value; } }
    static final class Slow {
        static final Node ROOT = new Node(17);
        static Node extra;
        static int value = prepare();
        static int prepare() {
            int sum = 0;
            for (int i = 0; i < 2048; i++) sum += i;
            return sum + ROOT.value;
        }
    }
    static final class Bad { static Node absent; static int value = absent.value; }
    static final class Nested { static int value = Slow.value + 1; }
    static final class NestedBad { static int value = Bad.value; }
    static final class CycleA { static int value = CycleB.value + 1; }
    static final class CycleB { static int value = CycleA.value + 2; }
    public static int slow(int value) { return Slow.value + value; }
    public static int storeSlow(int value) { Slow.extra = new Node(value); return Slow.extra.value; }
    static Node take() { Node node = Slow.extra; Slow.extra = null; return node; }
    public static int returned(int ignored) { Node node = take(); return node == null ? 0 : node.value; }
    public static int bad(int ignored) { return Bad.value; }
    public static int nested(int ignored) { return Nested.value; }
    public static int nestedBad(int ignored) { return NestedBad.value; }
    public static int cycleA(int ignored) { return CycleA.value; }
    public static int cycleB(int ignored) { return CycleB.value; }
    public static int churn(int count) {
        int sum = 0;
        for (int i = 0; i < count; i++) { Node node = new Node(i); node.next = node; sum += node.value; }
        return sum;
    }
}
