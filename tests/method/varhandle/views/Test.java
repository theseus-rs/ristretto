import java.lang.invoke.MethodHandles;
import java.lang.invoke.VarHandle;
import java.nio.ByteBuffer;
import java.nio.ByteOrder;
import java.nio.ReadOnlyBufferException;

/** Exercise cold and cached access handles without replacing their JDK semantics. */
public class Test {
    public static void main(String[] args) throws Throwable {
        for (ByteOrder order : new ByteOrder[]{ByteOrder.LITTLE_ENDIAN, ByteOrder.BIG_ENDIAN}) {
            arrays(order);
            buffers(order);
        }
        receiverIdentity();
        errors();
        System.out.println("VarHandle view checks passed");
    }

    static void arrays(ByteOrder order) {
        byte[] bytes = new byte[24];
        VarHandle ints = MethodHandles.byteArrayViewVarHandle(int[].class, order);
        VarHandle longs = MethodHandles.byteArrayViewVarHandle(long[].class, order);
        VarHandle floats = MethodHandles.byteArrayViewVarHandle(float[].class, order);
        VarHandle doubles = MethodHandles.byteArrayViewVarHandle(double[].class, order);
        VarHandle shorts = MethodHandles.byteArrayViewVarHandle(short[].class, order);
        VarHandle chars = MethodHandles.byteArrayViewVarHandle(char[].class, order);
        ByteBuffer expected = ByteBuffer.wrap(bytes).order(order);
        for (int round = 0; round < 3; round++) {
            // Plain access permits unaligned byte offsets. Signed and floating
            // values exercise the argument and result types of the cached handle.
            ints.set(bytes, 1, 0x89abcdef + round);
            check((int) ints.get(bytes, 1) == expected.getInt(1));
            longs.set(bytes, 3, 0xfedcba9876543210L + round);
            check((long) longs.get(bytes, 3) == expected.getLong(3));
            floats.set(bytes, 1, -1.25f - round);
            check((float) floats.get(bytes, 1) == expected.getFloat(1));
            doubles.set(bytes, 3, -3.5 - round);
            check((double) doubles.get(bytes, 3) == expected.getDouble(3));
            shorts.set(bytes, 1, (short) (-1234 - round));
            check((short) shorts.get(bytes, 1) == expected.getShort(1));
            chars.set(bytes, 1, (char) (0xfedc + round));
            check((char) chars.get(bytes, 1) == expected.getChar(1));
            floats.set(bytes, 1, Float.intBitsToFloat(0xffc01234));
            check(Float.floatToRawIntBits((float) floats.get(bytes, 1)) == 0xffc01234);
            doubles.set(bytes, 1, -0.0);
            check(Double.doubleToRawLongBits((double) doubles.get(bytes, 1)) == Long.MIN_VALUE);

            // JDK 25 removed ordered access to byte-array views.
            if (Runtime.version().feature() < 25) {
                ints.setVolatile(bytes, 0, 17);
                check((int) ints.getVolatile(bytes, 0) == 17);
                ints.setRelease(bytes, 0, 37);
                check((int) ints.getAcquire(bytes, 0) == 37);
                ints.setOpaque(bytes, 0, 41);
                check((int) ints.getOpaque(bytes, 0) == 41);
            }
        }
        System.out.println("array " + order);
    }

    static void buffers(ByteOrder order) {
        VarHandle ints = MethodHandles.byteBufferViewVarHandle(int[].class, order);
        for (ByteBuffer buffer : new ByteBuffer[]{ByteBuffer.allocate(24), ByteBuffer.allocateDirect(24)}) {
            buffer.order(order);
            for (int round = 0; round < 3; round++) {
                ints.set(buffer, 1, 0x89abcdef + round);
                check((int) ints.get(buffer, 1) == buffer.getInt(1));
                ints.set(buffer, 0, 53);
                // Newer JDKs restrict ordered byte-buffer views to native memory.
                if (buffer.isDirect()) {
                    ints.setVolatile(buffer, 0, 53);
                    check((int) ints.getVolatile(buffer, 0) == 53);
                }
                ByteBuffer readOnly = buffer.asReadOnlyBuffer();
                check((int) ints.get(readOnly, 0) == 53);
                expect(ReadOnlyBufferException.class, () -> ints.set(readOnly, 0, 1));
            }
        }
        System.out.println("buffer " + order);
    }

    static void receiverIdentity() throws Throwable {
        VarHandle little = MethodHandles.byteArrayViewVarHandle(int[].class, ByteOrder.LITTLE_ENDIAN);
        VarHandle big = MethodHandles.byteArrayViewVarHandle(int[].class, ByteOrder.BIG_ENDIAN);
        byte[] bytes = {1, 2, 3, 4};
        for (int round = 0; round < 3; round++) {
            check((int) little.get(bytes, 0) == 0x04030201);
            check((int) big.get(bytes, 0) == 0x01020304);
            check((int) little.toMethodHandle(VarHandle.AccessMode.GET).invokeExact(bytes, 0) == 0x04030201);
        }
    }

    static void errors() {
        VarHandle ints = MethodHandles.byteArrayViewVarHandle(int[].class, ByteOrder.nativeOrder());
        byte[] bytes = new byte[8];
        for (int round = 0; round < 3; round++) {
            check((int) ints.get(bytes, 0) == 0);
            expect(NullPointerException.class, () -> { int v = (int) ints.get((byte[]) null, 0); });
            expect(ClassCastException.class, () -> { int v = (int) ints.get(new int[8], 0); });
            expect(IndexOutOfBoundsException.class, () -> { int v = (int) ints.get(bytes, -1); });
            expect(IndexOutOfBoundsException.class, () -> { int v = (int) ints.get(bytes, 5); });
            if (Runtime.version().feature() < 25) {
                check((int) ints.getVolatile(bytes, 0) == 0);
                expect(IllegalStateException.class, () -> { int v = (int) ints.getVolatile(bytes, 1); });
            }
            expect(IndexOutOfBoundsException.class, () -> ints.set(bytes, 5, 1));
            expect(IndexOutOfBoundsException.class, () -> { int v = (int) ints.get(new byte[3], 0); });
            expect(IndexOutOfBoundsException.class, () -> { int v = (int) ints.get(bytes, Integer.MAX_VALUE); });
            check((int) ints.get(bytes, 4) == 0);
            try {
                int value = (int) ints.get(bytes, 5);
                throw new AssertionError("missing bounds check");
            } catch (IndexOutOfBoundsException failure) {
                check(failure.getMessage().contains("length 5"));
                System.out.println("bounds: " + failure.getClass().getSimpleName());
            }
        }
    }

    static void check(boolean condition) {
        if (!condition) throw new AssertionError("incorrect VarHandle result");
    }

    static void expect(Class<? extends Throwable> type, Runnable action) {
        try {
            action.run();
        } catch (Throwable failure) {
            if (type.isInstance(failure)) return;
            throw new AssertionError("expected " + type + " but got " + failure, failure);
        }
        throw new AssertionError("missing " + type);
    }
}
