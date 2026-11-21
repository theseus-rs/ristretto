/** Correctness fixture for owner-scoped fields, methods and overload descriptors. */
public class CanonicalLinkedWorkload {
    public static class Node {
        public int value;
        public Node(int value) { this.value = value; }
        public Node(long value) { this.value = (int) value + 5; }
        public static int helper(int value) { return value + 37; }
    }

    public static int helper(int value) { return value + 11; }
    public static int helper(long value) { return (int) value + 23; }

    public static int run(int value) {
        Node first = new Node(value);
        Node second = new Node((long) value);
        return helper(value) + helper((long) value) + Node.helper(value)
                + first.value + second.value;
    }
}
