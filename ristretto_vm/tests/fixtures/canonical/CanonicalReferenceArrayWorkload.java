public class CanonicalReferenceArrayWorkload {
    public static class Node {
        public int value;
        public Object owner;
        public Node(int value) { this.value = value; }
    }
    public static class Other { public Other() {} }
    public static class Holder { public Node[] values; public Holder() {} }

    public static Node[] create(int length) { return new Node[length]; }
    public static Object[] covariant(Node[] values) { return values; }
    public static Object identity(Object value) { return value; }
    public static Node[] detach(Holder holder) {
        Node[] values = holder.values;
        holder.values = null;
        return values;
    }
    public static int nodes(int length) {
        Node[] values = create(length);
        for (int i = 0; i < length; i++) {
            Node node = new Node(i + 3);
            node.owner = values;
            values[i] = node;
        }
        if (covariant(values) != values || identity(values) != values) return -9001;
        int sum = 0;
        for (int i = 0; i < length; i++) sum = sum * 31 + values[i].value;
        if (length > 0) {
            Node retained = values[length - 1];
            values[length - 1] = null;
            if (values[length - 1] != null) return -9002;
            sum += retained.value;
        }
        return sum;
    }
    public static int mixed(int length) {
        Object[] values = new Object[length];
        Node node = new Node(17);
        byte[] bytes = new byte[9];
        node.owner = values;
        for (int i = 0; i < length; i++) {
            if ((i & 1) == 0) values[i] = bytes;
            else values[i] = node;
        }
        int sum = length;
        for (int i = 0; i < length; i++) {
            if ((i & 1) == 0) {
                if (values[i] != bytes) return -9010;
                sum += bytes.length;
            } else {
                if (values[i] != node) return -9011;
                sum += node.value;
            }
        }
        if (length > 0) {
            values[0] = values;
            if (values[0] != values) return -9012;
        }
        return sum;
    }
    public static int returned(int length) {
        Holder holder = new Holder();
        holder.values = create(length);
        Node[] values = detach(holder);
        Object[] envelope = new Object[1];
        envelope[0] = values;
        if (envelope[0] != values || holder.values != null) return -9020;
        return values.length;
    }
    public static int failNegative(int length) { return new Object[length].length; }
    public static int failBounds(int index) { Object[] values = new Object[1]; return values[index] == null ? 0 : 1; }
    public static int failStoreBounds(int index) {
        Object[] values = new Node[1]; values[index] = new Other(); return 99;
    }
    public static int failType(int ignored) {
        Object[] values = new Node[1]; values[0] = new Other(); return 99;
    }
    public static int failNull(int index) {
        Object[] values = null; return values[index] == null ? 0 : 1;
    }
    public static int failNullStore(int index) {
        Object[] values = null; values[index] = new Other(); return 99;
    }
    public static int failNullLength(int ignored) { Object[] values = null; return values.length; }
}
