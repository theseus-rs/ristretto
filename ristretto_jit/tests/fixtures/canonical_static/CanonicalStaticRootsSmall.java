/** Closed canonical static-root correctness fixture; no benchmark timing. */
public final class CanonicalStaticRootsSmall {
    static final int SLOTS = 9;
    static final Object[] roots = new Object[SLOTS];
    static final int[] marks = new int[SLOTS];
    static int initialized = 1;
    static int rewrites;
    static Node extra;
    static final class Node {
        int value;
        Node next;
        Node(int value) { this.value = value; this.next = this; }
    }
    static void fill(int seed) {
        for (int i = 0; i < SLOTS; i++) {
            roots[i] = new Node(seed + i);
            marks[i] = seed + i;
        }
    }
    static int audit() {
        int sum = 0;
        for (int i = 0; i < SLOTS; i++) {
            Node node = (Node) roots[i];
            if (node.next != node || node.value != marks[i]) return -999;
            sum += node.value;
        }
        return sum;
    }
    static Node detach(int index) {
        Node node = (Node) roots[index];
        roots[index] = null;
        return node;
    }
    public static int run(int seed) {
        fill(seed);
        int first = audit();
        Node held = detach(SLOTS - 1);
        roots[SLOTS - 1] = new Node(held.value + 7);
        marks[SLOTS - 1] = held.value + 7;
        rewrites++;
        return first + audit() + initialized + rewrites;
    }
    public static int referenceOverwrite(int seed) {
        extra = new Node(seed);
        Node previous = extra;
        extra = new Node(seed + 1);
        previous.next = extra;
        extra = null;
        return previous.next.value + (extra == null ? 0 : 999);
    }
    public static int prepared(int ignored) {
        return roots.length + marks.length + initialized;
    }
}
