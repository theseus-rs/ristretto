import java.lang.invoke.MethodHandle;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.VarHandle;
import java.util.Arrays;

public class Test {
    public static void main(String[] args) throws Throwable {
        subWordExchanges();
        int[] values = {3, 5, 7};
        VarHandle ints = MethodHandles.arrayElementVarHandle(int[].class);
        ints.set(values, 0, 11);
        check("last element", (int) ints.getAndAdd(values, 2, 6) == 7);
        for (int index : new int[]{-1, values.length, Integer.MAX_VALUE}) {
            expect(ArrayIndexOutOfBoundsException.class, () -> { int v = (int) ints.get(values, index); });
            expect(ArrayIndexOutOfBoundsException.class, () -> ints.set(values, index, 13));
            expect(ArrayIndexOutOfBoundsException.class, () -> ints.compareAndSet(values, index, 5, 13));
        }
        expect(ArrayIndexOutOfBoundsException.class, () -> { int v = (int) ints.get(new int[0], 0); });
        expect(NullPointerException.class, () -> { int v = (int) ints.get((int[]) null, 0); });
        expect(ClassCastException.class, () -> { int v = (int) ints.get((Object) new long[3], 0); });
        check("bounds and neighbors", Arrays.equals(values, new int[]{11, 5, 13}));

        VarHandle refs = MethodHandles.arrayElementVarHandle(Object[].class);
        String[] strings = {"left", "middle", "right"};
        Object wrong = Integer.valueOf(42);
        VarHandle elements = MethodHandles.arrayElementVarHandle(String[].class);
        expect(ArrayIndexOutOfBoundsException.class, () -> elements.setVolatile(strings, 42, wrong));
        expect(NullPointerException.class, () -> elements.setRelease((String[]) null, 0, wrong));
        expect(ArrayIndexOutOfBoundsException.class, () -> elements.compareAndSet(strings, -1, wrong, wrong));
        expect(ClassCastException.class, () -> elements.setVolatile(strings, 0, wrong));
        // Plain array assignment evaluates the value cast before the array store instruction.
        expect(ClassCastException.class, () -> elements.set(strings, 42, wrong));
        expect(ArrayStoreException.class, () -> refs.set(strings, 1, wrong));
        expect(ArrayStoreException.class, () -> refs.getAndSet(strings, 1, wrong));
        expect(ArrayStoreException.class, () -> refs.compareAndSet(strings, 1, "middle", wrong));
        Object witness = refs.compareAndExchange(strings, 1, wrong, "valid");
        check("incompatible expected value returns current reference", witness == strings[1] && witness == "middle");
        expect(ArrayStoreException.class, () -> refs.compareAndExchange(strings, 1, "mismatch", wrong));
        check("covariant array unchanged", Arrays.equals(strings, new String[]{"left", "middle", "right"}));
        check("null replacement", refs.compareAndSet(strings, 1, "middle", null) && strings[1] == null);

        VarHandle doubles = MethodHandles.arrayElementVarHandle(double[].class);
        double nan = Double.longBitsToDouble(0x7ff8000000000123L);
        double other = Double.longBitsToDouble(0x7ff8000000000456L);
        double[] numbers = {-0.0, nan};
        check("opposite signed zeros", !doubles.compareAndSet(numbers, 0, 0.0, 3.0));
        check("signed zero witness", Double.doubleToRawLongBits((double) doubles.compareAndExchange(numbers, 0, -0.0, 3.0)) == Long.MIN_VALUE);
        check("NaN payload mismatch", !doubles.compareAndSet(numbers, 1, other, 5.0));
        check("NaN payload match", doubles.compareAndSet(numbers, 1, nan, 5.0));
        check("double array storage", numbers[0] == 3.0 && numbers[1] == 5.0);
        System.out.println("Array element access passed");
    }

    static void subWordExchanges() throws Throwable {
        MethodHandle bytes = MethodHandles.arrayElementVarHandle(byte[].class)
                .toMethodHandle(VarHandle.AccessMode.COMPARE_AND_EXCHANGE);
        MethodHandle shorts = MethodHandles.arrayElementVarHandle(short[].class)
                .toMethodHandle(VarHandle.AccessMode.COMPARE_AND_EXCHANGE);
        for (int length = 1; length <= 7; length++) {
            byte[] data = new byte[length];
            short[] words = new short[length];
            Arrays.fill(data, (byte) 7);
            Arrays.fill(words, (short) -91);
            int last = length - 1;
            check("byte miss", (byte) bytes.invokeExact(data, last, (byte) 1, (byte) 2) == 7);
            check("byte hit", (byte) bytes.invokeExact(data, last, (byte) 7, (byte) -5) == 7);
            check("short miss", (short) shorts.invokeExact(words, last, (short) 2, (short) 3) == -91);
            check("short hit", (short) shorts.invokeExact(words, last, (short) -91, (short) 4321) == -91);
            for (int i = 0; i < length; i++) {
                check("byte contents", data[i] == (i == last ? -5 : 7));
                check("short contents", words[i] == (i == last ? 4321 : -91));
            }
        }
    }

    static void expect(Class<? extends RuntimeException> type, Runnable action) {
        try {
            action.run();
        } catch (RuntimeException failure) {
            if (type == failure.getClass()) return;
            throw failure;
        }
        throw new AssertionError("expected " + type.getName());
    }

    static void check(String label, boolean condition) {
        if (!condition) throw new AssertionError(label);
    }
}
