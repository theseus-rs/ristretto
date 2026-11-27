/** Finite putstatic wait and cleared-static return ownership fixture. */
public final class CanonicalCompiledStaticHandoff {
    static final class Node {
        int value;
        Node(int value) { this.value = value; }
    }
    static final class Target {
        static Node root;
        static int marker = prepare();
        static int prepare() {
            int sum = 0;
            for (int i = 0; i < 256; i++) sum += i;
            return sum;
        }
    }
    public static int prepare(int ignored) { return Target.marker; }
    public static int store(int value) {
        Target.root = new Node(value);
        return take().value + Target.marker;
    }
    static Node take() {
        Node held = Target.root;
        Target.root = null;
        return held;
    }
    public static int empty(int ignored) { return Target.root == null ? 1 : 0; }
    public static int churn(int count) {
        int sum = 0;
        for (int i = 0; i < count; i++) sum += new Node(i).value;
        return sum;
    }
}
