import java.io.ByteArrayOutputStream;
import java.io.DataOutputStream;
import java.lang.reflect.Array;
import java.lang.reflect.Field;
import java.nio.ByteBuffer;
import sun.misc.Unsafe;

/** Real allocation failures, run one case per bounded JVM by the Rust resource_errors harness. */
public class Test {
    private static volatile Object retained;
    private static byte[] reserve;
    private static final int HUGE = Integer.MAX_VALUE;
    private static RuntimeException initializationFailure;
    private static final OutOfMemoryError[] pressureFailures = new OutOfMemoryError[16];
    private static boolean exhaustedBeforeInitializationFailure;

    interface Operation { void run() throws Exception; }

    static class Node {
        Node next;
        long a, b, c, d, e, f, g, h;
        Node(Node next) { this.next = next; }
    }

    static class Loader extends ClassLoader {
        Loader() { super(null); }
        void load(byte[] bytes) {
            Class<?> type = defineClass(null, bytes, 0, bytes.length);
            resolveClass(type);
        }
    }

    static class FailedInitialization {
        static final byte[] VALUE = new byte[HUGE];
    }

    static class FailedErrorWrapper {
        static { fillHeapThenThrow(); }
        static void use() { }
    }

    private static void fillHeapThenThrow() {
        Object[] objects = (Object[]) retained;
        int next = 0;
        for (int attempt = 0; attempt < pressureFailures.length; attempt++) {
            try {
                while (next < objects.length) objects[next++] = new Object();
                throw new AssertionError("Heap did not fill");
            } catch (OutOfMemoryError expected) {
                // Retain the VM's preallocated OOM errors and their backtraces as well.
                pressureFailures[attempt] = expected;
            }
        }
        exhaustedBeforeInitializationFailure = true;
        // The application throws a non-Error; JVMS 5.5 must attempt to allocate EIIE.
        throw initializationFailure;
    }

    private static native boolean limitAddressSpace();
    private static native boolean restoreAddressSpace();

    public static void main(String[] args) throws Exception {
        if (args.length != 1) throw new IllegalArgumentException("Use cargo test -p ristretto_vm --test resource_errors");
        String name = args[0];
        // Allocate reporting space before applying pressure. Do not allocate in the catch path.
        reserve = new byte[1024 * 1024];
        Operation operation = operation(name);
        OutOfMemoryError caught = null;
        try {
            operation.run();
        } catch (OutOfMemoryError error) {
            caught = error;
        } finally {
            retained = null;
            reserve = null;
        }
        System.gc();
        if (caught == null) throw new AssertionError("No OutOfMemoryError: " + name);
        if (caught.getClass() != OutOfMemoryError.class) throw new AssertionError(caught);
        checkMessage(name, caught.getMessage());
        if (name.equals("initialization-wrapper") && !exhaustedBeforeInitializationFailure) {
            throw new AssertionError("Failure preceded the tested initialization step");
        }
        if (name.equals("initialization")) {
            try {
                retained = FailedInitialization.VALUE;
                throw new AssertionError("Erroneous class initialized twice");
            } catch (NoClassDefFoundError expected) {
                // JVMS 5.5: the first Error escapes unchanged; subsequent use fails.
            }
        }
        byte[] recovery = new byte[1024];
        recovery[0] = 42;
        if (recovery[0] != 42) throw new AssertionError("Heap recovery failed");
        System.out.println("PASS " + name + " java.lang.OutOfMemoryError");
    }

    private static Operation operation(final String name) throws Exception {
        if (name.startsWith("limit-") || name.startsWith("heap-primitive-")) {
            final boolean limit = name.startsWith("limit-");
            final String type = name.substring(limit ? 6 : 15);
            return new Operation() {
                public void run() {
                    // Each branch emits its own newarray opcode (all eight atype values).
                    int size = limit ? HUGE : 32 * 1024 * 1024;
                    if (type.equals("boolean")) retained = new boolean[size];
                    else if (type.equals("byte")) retained = new byte[size];
                    else if (type.equals("char")) retained = new char[size];
                    else if (type.equals("short")) retained = new short[size];
                    else if (type.equals("int")) retained = new int[size];
                    else if (type.equals("long")) retained = new long[size];
                    else if (type.equals("float")) retained = new float[size];
                    else if (type.equals("double")) retained = new double[size];
                    else if (type.equals("reference")) retained = new Object[size];
                    else throw new IllegalArgumentException(type);
                }
            };
        }
        if (name.equals("heap-object") || name.equals("gc-overhead")) {
            // Preload the class before the test; failure must occur in new, not class loading.
            retained = new Node(null);
            return new Operation() {
                public void run() {
                    for (int i = 0; i < 2000000; i++) retained = new Node((Node) retained);
                }
            };
        }
        if (name.equals("heap-reference")) return new Operation() {
            public void run() { retained = new Object[8 * 1024 * 1024]; }
        };
        if (name.equals("heap-multi-primitive")) return new Operation() {
            public void run() { retained = new byte[1024][32768]; }
        };
        if (name.equals("heap-multi-reference")) return new Operation() {
            public void run() { retained = new Object[1024][8192]; }
        };
        if (name.equals("multi-outer-limit")) return new Operation() {
            public void run() { retained = new byte[HUGE][1]; }
        };
        if (name.equals("multi-inner-limit")) return new Operation() {
            public void run() { retained = new byte[1][HUGE]; }
        };
        if (name.equals("multi-reference-limit")) return new Operation() {
            public void run() { retained = new Object[1][HUGE]; }
        };
        if (name.equals("multi-limit-before-negative")) return new Operation() {
            public void run() { retained = new byte[HUGE][-1]; }
        };
        if (name.equals("reflect-array")) return new Operation() {
            public void run() { retained = Array.newInstance(long.class, HUGE); }
        };
        if (name.equals("reflect-multi")) return new Operation() {
            public void run() { retained = Array.newInstance(Object.class, 1, HUGE); }
        };
        if (name.equals("clone")) {
            final byte[] original = new byte[10 * 1024 * 1024];
            return new Operation() {
                public void run() { retained = original.clone(); }
            };
        }
        if (name.equals("string-growth")) return new Operation() {
            public void run() { new StringBuilder().ensureCapacity(32 * 1024 * 1024); }
        };
        if (name.equals("direct-buffer")) {
            // Warm the NIO implementation before applying its independent 1 MiB limit.
            retained = ByteBuffer.allocateDirect(1);
            return new Operation() {
                public void run() { retained = ByteBuffer.allocateDirect(2 * 1024 * 1024); }
            };
        }
        if (name.equals("native-allocate") || name.equals("native-reallocate")) {
            Field field = Unsafe.class.getDeclaredField("theUnsafe");
            field.setAccessible(true);
            final Unsafe unsafe = (Unsafe) field.get(null);
            return new Operation() {
                public void run() {
                    long address = name.equals("native-reallocate") ? unsafe.allocateMemory(8) : 0;
                    try {
                        // Impossible address-space request, not a loop consuming physical RAM.
                        if (address == 0) address = unsafe.allocateMemory(1L << 62);
                        else {
                            unsafe.putLong(address, 42);
                            try {
                                address = unsafe.reallocateMemory(address, 1L << 62);
                            } catch (OutOfMemoryError error) {
                                if (unsafe.getLong(address) != 42) {
                                    throw new AssertionError("Failed realloc corrupted original block");
                                }
                                throw error;
                            }
                        }
                    } finally {
                        if (address != 0) unsafe.freeMemory(address);
                    }
                }
            };
        }
        if (name.equals("metaspace") || name.equals("constant-pool")
                || name.equals("compressed-class-space")) {
            final byte[] bytes = classBytes(name.equals("constant-pool") ? 8192 : 0);
            return new Operation() {
                public void run() {
                    Loader loader = new Loader();
                    retained = loader; // Keep every defined class alive until failure.
                    for (int i = 0; i < 200000; i++) {
                        // The first UTF8 entry is GeneratedXXXXXXXX; edit only the fixed-width suffix.
                        for (int digit = 0; digit < 8; digit++) {
                            int nibble = (i >>> (digit * 4)) & 15;
                            bytes[29 - digit] = (byte) (nibble < 10 ? '0' + nibble : 'a' + nibble - 10);
                        }
                        loader.load(bytes);
                    }
                }
            };
        }
        if (name.equals("initialization")) return new Operation() {
            public void run() { retained = FailedInitialization.VALUE; }
        };
        if (name.equals("initialization-wrapper")) {
            initializationFailure = new RuntimeException("preallocated initialization failure");
            new ExceptionInInitializerError(initializationFailure); // Resolve the wrapper class first.
            retained = new Object[2 * 1024 * 1024];
            // Preload (but do not initialize) the class before heap exhaustion.
            Class.forName("Test$FailedErrorWrapper", false, Test.class.getClassLoader());
            return new Operation() {
                public void run() { FailedErrorWrapper.use(); }
            };
        }
        if (name.equals("native-thread")) {
            System.load(System.getProperty("resource.library"));
            // A single 64 MiB stack cannot fit in the process-local 16 MiB address-space headroom.
            // No thread storm, RLIMIT_NPROC changes, or pressure on other processes is needed.
            final Thread thread = new Thread(null, new Runnable() {
                public void run() { }
            }, "bounded-stack-allocation", 64L * 1024 * 1024);
            thread.setDaemon(true);
            return new Operation() {
                public void run() throws Exception {
                    if (!limitAddressSpace()) throw new AssertionError("Could not set RLIMIT_AS");
                    try {
                        thread.start();
                    } finally {
                        if (!restoreAddressSpace()) throw new AssertionError("Could not restore RLIMIT_AS");
                    }
                    thread.join();
                }
            };
        }
        throw new IllegalArgumentException(name);
    }

    private static void checkMessage(String name, String message) {
        String expected;
        if (name.startsWith("limit-") || name.endsWith("-limit") || name.equals("multi-limit-before-negative")
                || name.startsWith("reflect-") || name.equals("initialization")) {
            expected = "Requested array size exceeds VM limit";
        } else if (name.equals("metaspace") || name.equals("constant-pool")) {
            expected = "Metaspace";
        } else if (name.equals("compressed-class-space")) {
            expected = "Compressed class space";
        } else if (name.equals("gc-overhead")) {
            expected = "GC overhead limit exceeded";
        } else if (name.equals("direct-buffer")) {
            // JDK 8/11 and newer JDKs use different wording.
            if (message != null && message.toLowerCase(java.util.Locale.ROOT).contains("direct buffer memory")) return;
            throw new AssertionError("Unexpected direct-memory failure: " + message);
        } else if (name.equals("native-thread")) {
            if (message != null && (message.contains("unable to create native thread")
                    || message.contains("unable to create new native thread"))) return;
            throw new AssertionError("Unexpected native-thread failure: " + message);
        } else if (name.startsWith("native-")) {
            return; // Unsafe's native-allocation message is not stable across releases.
        } else {
            expected = "Java heap space";
        }
        if (message == null || !message.contains(expected)) {
            throw new AssertionError("Wrong allocation failed: expected " + expected + ", got " + message);
        }
    }

    /** A valid Java 8 class with optional integer constants occupying its run-time pool. */
    private static byte[] classBytes(int constants) throws Exception {
        ByteArrayOutputStream buffer = new ByteArrayOutputStream();
        DataOutputStream out = new DataOutputStream(buffer);
        out.writeInt(0xcafebabe);
        out.writeShort(0);
        out.writeShort(52);
        out.writeShort(5 + constants);
        out.writeByte(1); out.writeUTF("Generated00000000");
        out.writeByte(7); out.writeShort(1);
        out.writeByte(1); out.writeUTF("java/lang/Object");
        out.writeByte(7); out.writeShort(3);
        for (int i = 0; i < constants; i++) { out.writeByte(3); out.writeInt(i); }
        out.writeShort(0x21); // ACC_PUBLIC | ACC_SUPER
        out.writeShort(2); out.writeShort(4);
        out.writeShort(0); out.writeShort(0); out.writeShort(0); out.writeShort(0);
        return buffer.toByteArray();
    }
}
