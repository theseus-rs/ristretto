public final class CanonicalReturnWorkload {
    static final class Node {
        int value;
        Node next;
        Node(int value) { this.value = value; }
    }
    static Node identity(Node value) { return value; }
    public static int run(int value) {
        Node a = new Node(value);
        a.next = a;
        Node b = identity(a.next);
        a.next = null;
        if (b != a) return -1000000;
        return b.value;
    }
}
