/** Shared Java 8 bytecode. All runtimes execute identical escaping allocations.
 * Results measure the entire runtime, including execution and instrumentation.
 */
public final class GcBench {
    private static final int SLOTS = 2048;
    private static final Object[] roots = new Object[SLOTS];
    private static final int[] marks = new int[SLOTS];
    private static volatile long sink;

    private static final class Node {
        final int value;
        final Node next;
        Node(int value, Node next) { this.value = value; this.next = next; }
    }

    // Exact microseconds to 1024, then 64 bins per doubling; constant memory.
    private static final class Histogram {
        final long[] counts = new long[4096];
        long maxNs;
        void record(long ns) {
            if (ns < 0) throw new AssertionError("negative duration");
            if (ns > maxNs) maxNs = ns;
            long us = ns / 1000 + (ns % 1000 == 0 ? 0 : 1);
            int index;
            if (us <= 1024) index = (int) us;
            else {
                int exponent = 63 - Long.numberOfLeadingZeros(us);
                long width = 1L << (exponent - 6);
                long delta = us - (1L << exponent);
                index = 1024 + (exponent - 10) * 64 + (int) ((delta + width - 1) / width);
            }
            counts[index]++;
        }
        void print() {
            System.out.print("{\"max_ns\":" + maxNs + ",\"buckets_us\":[");
            boolean first = true;
            for (int i = 0; i < counts.length; i++) if (counts[i] != 0) {
                long upper;
                if (i <= 1024) upper = i;
                else {
                    int relative = i - 1024;
                    int exponent = 10 + relative / 64;
                    upper = (1L << exponent) + (relative % 64) * (1L << (exponent - 6));
                }
                if (!first) System.out.print(",");
                first = false;
                System.out.print("[" + upper + "," + counts[i] + "]");
            }
            System.out.print("]}");
        }
    }

    private static int operation(int workload, long sequence, int seed) {
        int mark = (int) sequence & 127;
        int slot = ((int) sequence * 17 + seed) & (SLOTS - 1);
        if (workload == 0) { sink = mark; return mark; }
        if (workload == 2) {
            Node head = null;
            for (int n = 0; n < 16; n++) head = new Node(mark + n, head);
            roots[slot] = head;
            marks[slot] = mark;
            int sum = 0;
            for (Node p = (Node) roots[slot]; p != null; p = p.next) sum += p.value;
            if (sum != 16 * mark + 120) throw new AssertionError("node corruption");
            return sum;
        }
        if (workload == 4) {
            // Bound the retained backing independently of the runtime's reference width.
            slot &= 31;
            Object[] values = new Object[4097];
            if (values[0] != null || values[4096] != null) throw new AssertionError("non-null defaults");
            Node marker = new Node(mark, null);
            values[0] = marker;
            values[4096] = marker;
            roots[slot] = values;
            marks[slot] = mark;
            return checkReferenceArray((Object[]) roots[slot], mark);
        }
        int length = 256;
        if (workload == 3) length = sequence % 4096 == 0 ? 1024 * 1024
            : 32 << (((int) sequence * 31 + seed) & 7);
        byte[] data = new byte[length];
        data[0] = (byte) mark;
        data[length - 1] = (byte) mark;
        roots[slot] = data; // Escapes; old-to-young overwrites exercise barriers.
        marks[slot] = mark;
        byte[] check = (byte[]) roots[slot];
        int sum = check[0] + check[check.length - 1];
        if (sum != 2 * mark) throw new AssertionError("array corruption");
        return sum;
    }

    private static int checkReferenceArray(Object[] values, int mark) {
        Node first = (Node) values[0];
        Node last = (Node) values[values.length - 1];
        if (values.length != 4097 || values[2048] != null || first == null ||
            first != last || first.value != mark) throw new AssertionError("reference array corruption");
        return first.value + last.value;
    }

    private static void audit(int workload) {
        if (workload == 0) return;
        for (int i = 0; i < SLOTS; i++) if (roots[i] != null) {
            if (workload == 2) {
                int count = 0, sum = 0;
                for (Node p = (Node) roots[i]; p != null; p = p.next) { count++; sum += p.value; }
                if (count != 16 || sum != 16 * marks[i] + 120) throw new AssertionError("retained node corruption");
            } else if (workload == 4) {
                checkReferenceArray((Object[]) roots[i], marks[i]);
            } else {
                byte[] p = (byte[]) roots[i];
                if (p[0] != marks[i] || p[p.length - 1] != marks[i]) throw new AssertionError("retained array corruption");
            }
        }
    }

    private static long arrivals(long elapsed, long rate) {
        return elapsed / 1000000000L * rate + elapsed % 1000000000L * rate / 1000000000L;
    }

    public static void main(String[] args) throws Exception {
        String name = args[0];
        int workload = name.equals("control") ? 0 : name.equals("allocation") ? 1
            : name.equals("roots") ? 2 : name.equals("mixed") ? 3 : name.equals("reference-array") ? 4 : -1;
        long duration = Long.parseLong(args[1]) * 1000000L;
        long warmup = Long.parseLong(args[2]) * 1000000L;
        long rate = Long.parseLong(args[3]); // Zero is a closed-loop capacity screen.
        int seed = Integer.parseInt(args[4]);
        if (workload < 0 || duration <= 0 || duration > 86400000000000L
            || warmup < 0 || rate < 0 || rate > 1000000000L) throw new IllegalArgumentException();
        long start = System.nanoTime(), n = 0;
        while (System.nanoTime() - start < warmup) sink = operation(workload, n++, seed);
        audit(workload);
        // Preserve the warmed live set. Fresh allocations and overwrites stay measured.
        Histogram service = new Histogram(), response = new Histogram();
        long[] times = new long[128], completions = new long[128], backlogs = new long[128];
        int samples = 0;
        long count = 0, checksum = 0, peakBacklog = 0;
        long offered = arrivals(duration, rate);
        start = GcTelemetry.beginMeasurement();
        long now = start;
        while (now - start < duration && (rate == 0 || count < offered)) {
            long offset = rate == 0 ? now - start : (count / rate) * 1000000000L + (count % rate) * 1000000000L / rate;
            long due = start + offset;
            while ((now = System.nanoTime()) < due) {
                long remaining = due - now;
                if (remaining >= 1000000L) Thread.sleep(remaining / 1000000L);
                else Thread.yield();
            }
            if (now - start >= duration) break;
            long before = now;
            if (rate == 0) due = before;
            checksum += operation(workload, count, seed);
            now = System.nanoTime();
            service.record(now - before);
            response.record(now - due);
            count++;
            long arrived = rate == 0 ? count : Math.min(offered, arrivals(now - start, rate) + 1);
            long backlog = Math.max(0, arrived - count);
            if (backlog > peakBacklog) peakBacklog = backlog;
            if (samples < 128 && now - start >= duration / 128 * samples) {
                times[samples] = now - start; completions[samples] = count; backlogs[samples] = backlog;
                samples++;
            }
        }
        long elapsed = GcTelemetry.endMeasurement() - start;
        sink = checksum;
        audit(workload);
        System.out.print("GC_RESULT {\"schema\":1,\"workload\":\"" + name + "\",\"duration_ns\":" + duration
            + ",\"elapsed_ns\":" + elapsed + ",\"warmup_ns\":" + warmup + ",\"rate\":" + rate
            + ",\"seed\":" + seed + ",\"completed\":" + count + ",\"offered\":" + offered
            + ",\"unfinished\":" + (rate == 0 ? 0 : Math.max(0, offered - count))
            + ",\"peak_backlog\":" + peakBacklog + ",\"checksum\":" + checksum + ",\"service\":");
        service.print(); System.out.print(",\"response\":"); response.print();
        System.out.print(",\"timeline\":[");
        for (int i = 0; i < samples; i++) {
            if (i != 0) System.out.print(",");
            System.out.print("[" + times[i] + "," + completions[i] + "," + backlogs[i] + "]");
        }
        System.out.println("]}");
    }
}
