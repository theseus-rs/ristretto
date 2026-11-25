package bridge;

/** Admitted target: cycle plus primitive array, returning only an integer. */
public final class Target {
    static final class Node {
        int value;
        Node next;
        Node(int value) { this.value = value; }
    }
    public static int compute(int value) {
        if (value < 0) {
            int[] invalid = new int[value];
            return invalid.length;
        }
        Node left = new Node(value);
        Node right = new Node(value + 1);
        left.next = right;
        right.next = left;
        int[] payload = new int[3];
        payload[0] = left.value;
        payload[1] = right.value;
        payload[2] = 3;
        return left.next.next.value + payload[0] + payload[1] + payload[2];
    }
    public int instance(int value) { return value; }
}
