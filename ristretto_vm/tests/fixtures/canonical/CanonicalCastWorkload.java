public class CanonicalCastWorkload {
    public static class Node {
        public int value;
        public Object owner;
        public Node(int value) { this.value = value; }
    }
    public static class Other { public Other() {} }
    public static Object identity(Object value) { return value; }
    public static Node nodeIdentity(Node value) { return value; }
    public static Node castNode(Object value) { return (Node) value; }
    public static int objects(int value) {
        Object[] roots = new Object[9];
        Node node = new Node(value);
        node.owner = roots;
        roots[8] = node;
        Node cast = (Node) identity(roots[8]);
        if (cast != node) return -9001;
        roots[8] = null;
        return nodeIdentity(castNode(identity(cast))).value;
    }
    public static int nullable(int ignored) {
        Object value = null;
        Node node = (Node) value;
        byte[] bytes = (byte[]) value;
        Node[] nodes = (Node[]) value;
        Object[] objects = (Object[]) value;
        return node == null && bytes == null && nodes == null && objects == null ? 0 : -9002;
    }
    public static int booleans(int value) {
        boolean[] data = new boolean[3]; data[1] = (value & 1) != 0;
        Object boxed = identity(data); boolean[] cast = (boolean[]) boxed;
        if (cast != data) return -9003;
        return cast[1] ? 1 : 0;
    }
    public static int bytes(int value) {
        byte[] data = new byte[3]; data[1] = (byte) value;
        Object boxed = identity(data); byte[] cast = (byte[]) boxed;
        if (cast != data) return -9003;
        return cast[1];
    }
    public static int chars(int value) {
        char[] data = new char[3]; data[1] = (char) value;
        Object boxed = identity(data); char[] cast = (char[]) boxed;
        if (cast != data) return -9003;
        return cast[1];
    }
    public static int shorts(int value) {
        short[] data = new short[3]; data[1] = (short) value;
        Object boxed = identity(data); short[] cast = (short[]) boxed;
        if (cast != data) return -9003;
        return cast[1];
    }
    public static int ints(int value) {
        int[] data = new int[3]; data[1] = value;
        Object boxed = identity(data); int[] cast = (int[]) boxed;
        if (cast != data) return -9003;
        return cast[1];
    }
    public static int longs(int value) {
        long[] data = new long[3]; data[1] = value;
        Object boxed = identity(data); long[] cast = (long[]) boxed;
        if (cast != data) return -9003;
        return (int) cast[1];
    }
    public static int floats(int value) {
        float[] data = new float[3]; data[1] = value;
        Object boxed = identity(data); float[] cast = (float[]) boxed;
        if (cast != data) return -9003;
        return (int) cast[1];
    }
    public static int doubles(int value) {
        double[] data = new double[3]; data[1] = value;
        Object boxed = identity(data); double[] cast = (double[]) boxed;
        if (cast != data) return -9003;
        return (int) cast[1];
    }
    public static Object createNodes(int value) {
        Node[] nodes = new Node[9]; nodes[8] = new Node(value); return nodes;
    }
    public static int referenceArrays(int value) {
        Object boxed = createNodes(value);
        Object[] broad = (Object[]) identity(boxed);
        Node[] narrow = (Node[]) broad;
        if (narrow != boxed || narrow != broad) return -9004;
        Node holder = new Node(0); holder.owner = narrow;
        boxed = holder.owner; holder.owner = null;
        return ((Node[]) identity(boxed))[8].value;
    }
    public static int objectArrays(int value) {
        Object[] values = new Object[9]; values[8] = new Node(value);
        Object boxed = identity(values); Object[] cast = (Object[]) boxed;
        if (cast != values) return -9005;
        return ((Node) cast[8]).value;
    }
    public static int failClass(int ignored) {
        Object boxed = new Other(); return ((Node) boxed).value;
    }
    public static int failPrimitive(int ignored) {
        Object boxed = new int[1]; return ((byte[]) boxed).length;
    }
    public static int failObjectArray(int ignored) {
        Object boxed = new byte[1]; return ((Object[]) boxed).length;
    }
    public static int failCovariance(int ignored) {
        Object boxed = new Object[1]; return ((Node[]) boxed).length;
    }
    public static int failNodeFromArray(int ignored) {
        Object boxed = new Node[1]; return ((Node) boxed).value;
    }
    public static int failArrayFromNode(int value) {
        Object boxed = new Node(value); return ((Node[]) boxed).length;
    }
    public static int failArrayStore(int ignored) {
        Object boxed = new Node[1]; Object[] broad = (Object[]) boxed;
        broad[0] = new Other(); return 99;
    }
    public static int failArrayBounds(int index) {
        Object boxed = new Node[1]; Object[] broad = (Object[]) boxed;
        broad[index] = new Other(); return 99;
    }
    public static int failNullArray(int index) {
        Object boxed = null; Object[] broad = (Object[]) boxed;
        broad[index] = new Other(); return 99;
    }
}
