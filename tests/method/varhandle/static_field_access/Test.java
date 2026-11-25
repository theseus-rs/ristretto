import java.lang.invoke.MethodHandle;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.VarHandle;

public class Test {
    static String text = "initial";
    static Number number;
    static Number[] numbers;
    static Runnable[] tasks;
    static Cloneable[][] arrays;
    static final Object CONSTANT = new Object();
    static byte b = Byte.MAX_VALUE;
    static short s = Short.MIN_VALUE;
    static char c = Character.MAX_VALUE;
    static Integer counter = 0;
    static int attempts;

    // Looking up the handles here exercises LazyInitializingVarHandle on Java 22+.
    static final VarHandle TEXT = handle("text", String.class);
    static final VarHandle NUMBERS = handle("numbers", Number[].class);
    static final VarHandle ARRAYS = handle("arrays", Cloneable[][].class);
    static final VarHandle FINAL = handle("CONSTANT", Object.class);
    static final VarHandle BYTE = handle("b", byte.class);
    static final VarHandle SHORT = handle("s", short.class);
    static final VarHandle CHAR = handle("c", char.class);
    static final VarHandle COUNTER = handle("counter", Integer.class);
    static final VarHandle ATTEMPTS = handle("attempts", int.class);

    static VarHandle handle(String name, Class<?> type) {
        try {
            return MethodHandles.lookup().findStaticVarHandle(Test.class, name, type);
        } catch (ReflectiveOperationException e) {
            throw new ExceptionInInitializerError(e);
        }
    }

    static void checkThrows(Class<? extends RuntimeException> expected, Runnable operation) {
        try {
            operation.run();
        } catch (RuntimeException e) {
            if (expected.isInstance(e)) return;
            throw e;
        }
        throw new AssertionError("Expected " + expected.getName());
    }

    static class Worker extends Thread {
        int wins;

        public void run() {
            for (int i = 0; i < 400; i++) {
                Integer old = (Integer) COUNTER.getVolatile();
                if (COUNTER.compareAndSet(old, Integer.valueOf(old + 1))) {
                    wins++;
                }
                ATTEMPTS.getAndAdd(1);
            }
        }
    }

    static class Subwords {
        static int neighbor = 0x12121212;
        static byte small = 0x12;
        static short medium = 0x1212;
    }

    static void subwordExchanges() throws Throwable {
        MethodHandles.Lookup lookup = MethodHandles.lookup();
        VarHandle.AccessMode mode = VarHandle.AccessMode.COMPARE_AND_EXCHANGE;
        MethodHandle small = lookup.findStaticVarHandle(Subwords.class, "small", byte.class).toMethodHandle(mode);
        MethodHandle medium = lookup.findStaticVarHandle(Subwords.class, "medium", short.class).toMethodHandle(mode);
        byte b = (byte) small.invokeExact((byte) 0x12, (byte) -7);
        short s = (short) medium.invokeExact((short) 0x1212, (short) -300);
        if (b != 0x12 || s != 0x1212 || Subwords.small != -7 || Subwords.medium != -300
                || Subwords.neighbor != 0x12121212) throw new AssertionError("subword exchange changed the wrong field");
        if ((byte) small.invokeExact((byte) 0, (byte) 1) != -7
                || (short) medium.invokeExact((short) 0, (short) 1) != -300) throw new AssertionError("subword witness");
    }

    public static void main(String[] args) throws Throwable {
        subwordExchanges();
        Object wrong = Integer.valueOf(42);
        checkThrows(ClassCastException.class, () -> TEXT.set(wrong));
        checkThrows(ClassCastException.class, () -> TEXT.compareAndSet(wrong, "new"));
        checkThrows(ClassCastException.class, () -> TEXT.compareAndExchange("unmatched", wrong));
        System.out.println("text preserved: " + "initial".equals(text));
        checkThrows(UnsupportedOperationException.class, () -> TEXT.getAndAdd(wrong));
        VarHandle numberHandle = handle("number", Number.class);
        if (numberHandle.compareAndSet(3, Integer.valueOf(5)) || !numberHandle.compareAndSet(null, (short) 5)
                || !(number instanceof Short) || number.intValue() != 5) {
            throw new AssertionError("primitive reference operands");
        }
        Runnable[] tasksValue = new I[1];
        handle("tasks", Runnable[].class).set((Object) tasksValue);
        if (tasks != tasksValue) throw new AssertionError("reference array component named I");
        NUMBERS.setRelease((Object) new Integer[1]);
        checkThrows(ClassCastException.class, () -> NUMBERS.getAndSet((Object) new String[1]));
        System.out.println("array witness: " + (NUMBERS.getAndSet(null) instanceof Integer[]));
        int[][][] matrix = new int[1][][];
        Object candidate = matrix;
        if (!Cloneable[][].class.isInstance(candidate) || !(candidate instanceof Cloneable[][])
                || ((Cloneable[][]) candidate) != candidate) {
            throw new AssertionError("nested array type checks");
        }
        ARRAYS.set(candidate);
        System.out.println("nested array preserved: " + (arrays == matrix));

        Object original = FINAL.get();
        System.out.println("final set supported: " + FINAL.isAccessModeSupported(VarHandle.AccessMode.SET));
        checkThrows(UnsupportedOperationException.class, () -> FINAL.set(new Object()));
        checkThrows(UnsupportedOperationException.class, () -> FINAL.compareAndSet(original, null));
        System.out.println("final preserved: " + (CONSTANT == original));

        System.out.println("byte: " + (byte) BYTE.getAndAdd((byte) 1) + " -> " + (int) b);
        System.out.println("short: " + (short) SHORT.getAndAddAcquire((short) -1) + " -> " + (int) s);
        System.out.println("char: " + (int) (char) CHAR.getAndAddRelease((char) 1) + " -> " + (int) c);

        Worker[] workers = {new Worker(), new Worker(), new Worker(), new Worker()};
        for (Worker worker : workers) worker.start();
        int wins = 0;
        for (Worker worker : workers) {
            worker.join();
            wins += worker.wins;
        }
        System.out.println("CAS count matches: " + (wins > 0 && counter == wins));
        System.out.println("increments: " + attempts);
    }
}

class I implements Runnable {
    public void run() {}
}
