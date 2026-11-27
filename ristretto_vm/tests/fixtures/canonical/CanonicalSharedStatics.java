/** Finite shared-world class initialization and persistent-root fixture. */
public final class CanonicalSharedStatics {
    static final class Node {
        int value;
        Node(int value) { this.value = value; }
    }
    static final class Holder {
        static final int SALT = 7;
        static int value;
        static Object root;
    }
    static final class Slow {
        static final int[] data = new int[9];
        static int value = prepare();
        static int prepare() {
            data[0] = 17;
            int sum = 0;
            for (int i = 0; i < 256; i++) sum += i;
            return sum + data[0];
        }
    }
    static final class Bad {
        static int[] data = new int[size()];
        static int size() { return -1; }
    }
    static final class Nested {
        static int value = Slow.value + 1;
    }
    static final class CycleA { static int value = CycleB.value + 1; }
    static final class CycleB { static int value = CycleA.value + 2; }
    public static int cycleA(int ignored) { return CycleA.value; }
    public static int cycleB(int ignored) { return CycleB.value; }
    public static int read(int ignored) {
        Node node = (Node)Holder.root;
        return Holder.value + (node == null ? 0 : node.value) + Holder.SALT;
    }
    public static int write(int value) {
        Holder.value = value;
        Holder.root = new Node(value);
        return read(0);
    }
    public static int clear(int ignored) {
        Holder.root = null;
        Holder.value = 0;
        return read(0);
    }
    public static int slow(int value) { return Slow.value + value; }
    public static int waitWithRoot(int value) { return consume(new Node(value)); }
    static int consume(Node node) { return Slow.value + node.value; }
    public static int bad(int ignored) { return Bad.data.length; }
    public static int nested(int ignored) { return Nested.value; }
}
