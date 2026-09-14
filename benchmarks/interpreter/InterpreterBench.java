import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;

/** Identical Java 8 bytecode for both interpreters; no benchmark dependencies. */
public final class InterpreterBench {
    private static volatile long sink;
    private static final int SIZE = 1024;
    private static final int[] DATA = new int[SIZE];
    private static final int[] INDEX = new int[SIZE];
    private static final int[] COPY = new int[SIZE];
    private static final int[] MATRIX_A = new int[16 * 16];
    private static final int[] MATRIX_B = new int[16 * 16];
    private static final int[] MATRIX_C = new int[16 * 16];
    private static final String TEXT = "The quick brown fox jumps over the lazy dog. 0123456789";
    private static final Object LOCK = new Object();
    private static final RuntimeException REUSED_EXCEPTION = new RuntimeException("benchmark");
    private static final Node FIELD = new Node(7, null);
    private static final Operation[] OPERATIONS = {new Add(), new Xor()};
    private static final Base[] VIRTUAL = {new Add(), new Xor()};

    interface Operation { int apply(int value); }
    abstract static class Base implements Operation { public abstract int apply(int value); }
    static final class Add extends Base { public int apply(int value) { return value + 17; } }
    static final class Xor extends Base { public int apply(int value) { return value ^ 0x5a5a; } }
    static final class Node {
        int value;
        Node next;
        Node(int value, Node next) { this.value = value; this.next = next; }
    }

    private static void initialize(int seed) {
        int state = seed;
        for (int i = 0; i < SIZE; i++) {
            state = state * 1664525 + 1013904223;
            DATA[i] = state;
            INDEX[i] = (state >>> 16) & (SIZE - 1);
        }
        for (int i = 0; i < MATRIX_A.length; i++) {
            MATRIX_A[i] = DATA[i] & 15;
            MATRIX_B[i] = DATA[i + 256] & 15;
        }
    }

    private static int mix(int x) { return (x * 1664525 + 1013904223) ^ (x >>> 13); }
    private static int fib(int n) { return n < 2 ? n : fib(n - 1) + fib(n - 2); }
    private static void throwReused() { throw REUSED_EXCEPTION; }
    private static void throwFresh() { throw new IllegalArgumentException("benchmark"); }

    // Each unit is the fixed amount of work documented in src/workloads.rs and README.md.
    private static long run(int id, int units, int seed) {
        long result = 0;
        switch (id) {
            case 0: return seed; // Protocol/timing control; deliberately does no kernel work.
            case 1: {
                int x = seed;
                for (int u = 0; u < units; u++) for (int i = 0; i < 256; i++)
                    x = (x * 1664525 + 1013904223) ^ (x >>> 13);
                return x;
            }
            case 2: {
                long x = seed;
                for (int u = 0; u < units; u++) for (int i = 0; i < 256; i++)
                    x = (x * 6364136223846793005L + 1442695040888963407L) ^ (x >>> 29);
                return x;
            }
            case 3: {
                float x = (seed & 255) + 0.25f;
                for (int u = 0; u < units; u++) for (int i = 0; i < 256; i++)
                    x = (x + 1.25f) * 0.999f - 0.125f;
                return Float.floatToIntBits(x);
            }
            case 4: {
                double x = (seed & 255) + 0.25;
                for (int u = 0; u < units; u++) for (int i = 0; i < 256; i++)
                    x = (x + 1.25) * 0.999 - 0.125;
                return Double.doubleToLongBits(x);
            }
            case 5: {
                int x = seed;
                for (int u = 0; u < units; u++) for (int i = 0; i < 256; i++) {
                    x = (int) ((double) (long) (x & 65535) * 1.01) + (int) (float) i;
                    result += (short) x + (byte) x + (char) x;
                }
                return result;
            }
            case 6:
                for (int u = 0; u < units; u++) for (int i = 0; i < SIZE; i++)
                    if (i < 1000) result += DATA[i]; else result -= DATA[i];
                return result;
            case 7:
                for (int u = 0; u < units; u++) for (int i = 0; i < SIZE; i++)
                    if ((DATA[i] & 65536) == 0) result += DATA[i]; else result -= DATA[i];
                return result;
            case 8:
                for (int u = 0; u < units; u++) for (int i = 0; i < 256; i++) {
                    switch (DATA[i] & 7) {
                        case 0: result += 3; break;
                        case 1: result ^= 17; break;
                        case 2: result -= 5; break;
                        case 3: result += 23; break;
                        case 4: result ^= 31; break;
                        case 5: result -= 11; break;
                        case 6: result += 7; break;
                        default: result ^= 13;
                    }
                }
                return result;
            case 9:
                for (int u = 0; u < units; u++) for (int i = 0; i < 256; i++) {
                    switch ((DATA[i] & 7) * 1009) {
                        case 0: result += 3; break;
                        case 1009: result ^= 17; break;
                        case 2018: result -= 5; break;
                        case 3027: result += 23; break;
                        case 4036: result ^= 31; break;
                        case 5045: result -= 11; break;
                        case 6054: result += 7; break;
                        default: result ^= 13;
                    }
                }
                return result;
            case 10: {
                int x = seed;
                for (int u = 0; u < units; u++) for (int i = 0; i < 256; i++) x = mix(x);
                return x;
            }
            case 11: {
                int x = seed;
                for (int u = 0; u < units; u++) for (int i = 0; i < 256; i++)
                    x = VIRTUAL[i & 1].apply(x);
                return x;
            }
            case 12: {
                int x = seed;
                for (int u = 0; u < units; u++) for (int i = 0; i < 256; i++)
                    x = OPERATIONS[i & 1].apply(x);
                return x;
            }
            case 13:
                for (int u = 0; u < units; u++) result += fib(12 + (seed & 1));
                return result;
            case 14:
                for (int u = 0; u < units; u++) for (int i = 0; i < SIZE; i++) result += DATA[i];
                return result;
            case 15:
                for (int u = 0; u < units; u++) for (int i = 0; i < SIZE; i++) result += DATA[INDEX[i]];
                return result;
            case 16:
                FIELD.value = seed;
                for (int u = 0; u < units; u++) for (int i = 0; i < 256; i++) {
                    FIELD.value = FIELD.value * 31 + i;
                    result += FIELD.value;
                }
                return result;
            case 17:
                for (int u = 0; u < units; u++) {
                    Node head = null;
                    for (int i = 0; i < 64; i++) head = new Node(seed + i, head);
                    for (Node node = head; node != null; node = node.next) result += node.value;
                }
                return result;
            case 18:
                for (int u = 0; u < units; u++) {
                    int[] values = new int[256];
                    for (int i = 0; i < values.length; i++) values[i] = seed + i;
                    for (int value : values) result += value;
                }
                return result;
            case 19:
                for (int u = 0; u < units; u++) for (int i = 0; i < 16; i++) {
                    try { throwReused(); } catch (RuntimeException e) { result += i + seed; }
                }
                return result;
            case 20:
                for (int u = 0; u < units; u++) for (int i = 0; i < 16; i++) {
                    try { throwFresh(); } catch (IllegalArgumentException e) { result += i + seed; }
                }
                return result;
            case 21:
                for (int u = 0; u < units; u++) for (int i = 0; i < 256; i++) {
                    synchronized (LOCK) { result += i ^ seed; }
                }
                return result;
            case 22:
                for (int u = 0; u < units; u++) for (int i = 0; i < TEXT.length(); i++)
                    result = result * 31 + TEXT.charAt(i);
                return result;
            case 23:
                for (int u = 0; u < units; u++) {
                    StringBuilder builder = new StringBuilder();
                    for (int i = 0; i < 16; i++) builder.append(TEXT).append(seed + i);
                    String value = builder.toString();
                    result += value.length() + value.charAt(value.length() - 1);
                }
                return result;
            case 24:
                for (int u = 0; u < units; u++) {
                    System.arraycopy(DATA, 0, COPY, 0, SIZE);
                    result += COPY[u & (SIZE - 1)];
                }
                return result;
            case 25:
                for (int u = 0; u < units; u++) {
                    int[] values = DATA.clone();
                    Arrays.sort(values);
                    for (int value : values) result = result * 31 + value;
                }
                return result;
            case 26:
                for (int u = 0; u < units; u++) {
                    HashMap<Integer, Integer> map = new HashMap<Integer, Integer>();
                    for (int i = 0; i < 64; i++) map.put(seed + i, i * 3);
                    for (int i = 0; i < 64; i++) result += map.get(seed + i);
                }
                return result;
            case 27:
                for (int u = 0; u < units; u++) {
                    ArrayList<Integer> list = new ArrayList<Integer>();
                    for (int i = 0; i < 256; i++) list.add(seed + i);
                    for (int i = 0; i < list.size(); i++) result += list.get(i);
                }
                return result;
            case 28:
                for (int u = 0; u < units; u++) {
                    boolean[] composite = new boolean[4096];
                    for (int p = 2; p * p < composite.length; p++) if (!composite[p])
                        for (int i = p * p; i < composite.length; i += p) composite[i] = true;
                    for (int i = 2; i < composite.length; i++) if (!composite[i]) result += i;
                }
                return result;
            case 29:
                for (int u = 0; u < units; u++) {
                    for (int i = 0; i < 16; i++) for (int j = 0; j < 16; j++) {
                        int value = 0;
                        for (int k = 0; k < 16; k++) value += MATRIX_A[i * 16 + k] * MATRIX_B[k * 16 + j];
                        MATRIX_C[i * 16 + j] = value;
                    }
                    for (int value : MATRIX_C) result += value;
                }
                return result;
            default: throw new IllegalArgumentException("Unknown benchmark: " + id);
        }
    }

    public static void main(String[] args) throws Exception {
        if (args.length == 1 && args[0].equals("info")) {
            System.out.println("java.home=" + System.getProperty("java.home"));
            System.out.println("java.version=" + System.getProperty("java.version"));
            System.out.println("java.vm.name=" + System.getProperty("java.vm.name"));
            System.out.println("java.vm.version=" + System.getProperty("java.vm.version"));
            System.out.println("java.vm.info=" + System.getProperty("java.vm.info"));
            return;
        }
        int id = Integer.parseInt(args[0]);
        int units = Integer.parseInt(args[1]);
        int warmups = Integer.parseInt(args[2]);
        int samples = Integer.parseInt(args[3]);
        int seed = Integer.parseInt(args[4]);
        if (units < 1 || warmups < 0 || samples < 1) throw new IllegalArgumentException("Invalid counts");
        initialize(seed);
        // Exercise the I/O handshake before measuring the first kernel.
        System.out.println("H");
        System.out.flush();
        if (System.in.read() != 'G' || System.in.read() != '\n') throw new IllegalStateException("Expected handshake");
        for (int i = 0; i < warmups; i++) sink = run(id, units, seed);
        for (int i = 0; i < samples; i++) {
            System.out.println("R");
            System.out.flush();
            if (System.in.read() != 'G' || System.in.read() != '\n') throw new IllegalStateException("Expected start");
            long value = run(id, units, seed);
            sink = value;
            // The host stops its clock at this marker, before checksum formatting.
            System.out.println("D");
            System.out.flush();
            System.out.println(value);
            System.out.flush();
        }
    }
}
