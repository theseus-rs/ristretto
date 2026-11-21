/** Shared loop-carried allocation/reference-update workload. No timing in this class. */
public class CanonicalRingWorkload {
    public static class Node {
        public int generation;
        public byte[] payload;
        public Node next;
        public Node previous;

        public Node(int generation, int base, int mask) {
            this.generation = generation;
            this.payload = new byte[base + (generation & mask)];
            this.payload[0] = (byte) generation;
            this.payload[this.payload.length - 1] = (byte) (generation * 17 + 3);
        }
    }

    public static int small(int steps) { return execute(steps, 8, 64, 63); }
    public static int medium(int steps) { return execute(steps, 32, 1024, 255); }
    public static int large(int steps) { return execute(steps, 128, 4096, 1023); }

    public static int inspect(byte[] payload, int salt) {
        return salt * 31 + payload.length + payload[0] * 7 + payload[payload.length - 1];
    }

    public static int execute(int steps, int retained, int base, int mask) {
        if (steps < 0) return -1;
        Node head = new Node(0, base, mask);
        head.next = head;
        head.previous = head;
        for (int i = 1; i < retained; i++) {
            Node node = new Node(i, base, mask);
            Node tail = head.previous;
            node.previous = tail;
            node.next = head;
            tail.next = node;
            head.previous = node;
        }
        Node cursor = head;
        int checksum = 1;
        for (int i = 0; i < steps; i++) {
            Node old = cursor.next;
            Node after = old.next;
            Node replacement = new Node(i + retained, base, mask);
            replacement.previous = cursor;
            replacement.next = after;
            cursor.next = replacement;
            after.previous = replacement;
            if (head == old) head = replacement;
            byte[] alias = old.payload;
            old.payload = null;
            old.next = old;
            old.previous = old;
            // The removed node becomes cyclic garbage; its former array stays rooted
            // through this call after the last catalog edge to the array was removed.
            checksum = inspect(alias, checksum + old.generation);
            cursor = replacement.next;
        }
        cursor = head;
        for (int i = 0; i < retained; i++) {
            if (cursor.next.previous != cursor) return -19088743;
            if (cursor.previous.next != cursor) return -19088744;
            checksum = inspect(cursor.payload, checksum + cursor.generation);
            cursor = cursor.next;
        }
        if (cursor != head) return -19088745;
        return checksum;
    }
}
