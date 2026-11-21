/** Persistent int/reference fields without a class initializer. */
public final class CanonicalStaticFields {
    static final class Node {
        int value;
        Node next;
        Node(int value) { this.value = value; }
    }
    static int number;
    static Node root;
    static final int SALT = 7;
    public static int read(int ignored) {
        Node node = root;
        return number + (node == null ? 0 : node.value) + SALT;
    }
    public static int write(int value) {
        number = value;
        Node node = new Node(value);
        node.next = node;
        root = node;
        return read(0);
    }
    public static int clear(int ignored) {
        Node retained = root;
        root = null;
        number = 0;
        return retained == null ? 0 : retained.value;
    }
    static Node take() { Node retained = root; root = null; return retained; }
    public static int returned(int ignored) {
        Node retained = take();
        return retained == null ? 0 : retained.value;
    }
    public static int churn(int count) {
        int sum = 0;
        for (int i = 0; i < count; i++) {
            Node node = new Node(i);
            node.next = node;
            sum += node.value;
        }
        return sum;
    }
}
