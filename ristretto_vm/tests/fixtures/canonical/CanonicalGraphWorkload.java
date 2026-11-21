/** Shared ordinary-Java fixture for the restricted canonical and normal VM tiers. */
public final class CanonicalGraphWorkload {
    static final class Node {
        int value;
        Node next;
        Node other;

        Node(int value) { this.value = value; }
    }

    static int checksum(Node a, Node b) {
        Node retained = a.next;
        a.next = null;
        if (retained != b || retained.next != a) return -1000000;
        a.other = retained;
        return a.value + a.other.value;
    }

    public static int run(int rounds) {
        int sum = 0;
        for (int i = 0; i < rounds; i++) {
            Node a = new Node(i);
            Node b = new Node(i + 1);
            a.next = b;
            b.next = a;
            sum += checksum(a, b);
        }
        return sum;
    }
}
