import java.lang.invoke.MethodHandles;
import java.lang.invoke.VarHandle;

/** Primitive bytecode descriptors for every mode, including boolean and void results. */
public class Test {
    int field;
    static int count;

    public static void main(String[] args) throws Exception {
        VarHandle instance = MethodHandles.lookup().findVarHandle(Test.class, "field", int.class);
        VarHandle statics = MethodHandles.lookup().findStaticVarHandle(Test.class, "count", int.class);
        VarHandle element = MethodHandles.arrayElementVarHandle(int[].class);
        Test receiver = new Test();
        int[] array = {0, 7};
        instance.set(receiver, 11);
        equal(11, (int) instance.get(receiver));
        statics.setVolatile(13);
        equal(13, (int) statics.getVolatile());
        element.setRelease(array, 1, 17);
        equal(17, (int) element.getAcquire(array, 1));
        instance.setOpaque(receiver, 19);
        equal(19, (int) instance.getOpaque(receiver));

        if (!statics.compareAndSet(13, 23)) throw new AssertionError("compareAndSet");
        equal(19, (int) instance.compareAndExchange(receiver, 19, 29));
        equal(23, (int) statics.compareAndExchangeAcquire(0, 31));
        equal(17, (int) element.compareAndExchangeRelease(array, 1, 17, 37));
        // Mismatches must fail even for weak modes and leave the original values in place.
        if (instance.weakCompareAndSet(receiver, 0, 1) || statics.weakCompareAndSetPlain(0, 1)
                || element.weakCompareAndSetAcquire(array, 1, 0, 1) || instance.weakCompareAndSetRelease(receiver, 0, 1)) {
            throw new AssertionError("weak CAS accepted a mismatch");
        }

        equal(29, (int) instance.getAndSet(receiver, 10));
        equal(23, (int) statics.getAndSetAcquire(10));
        equal(37, (int) element.getAndSetRelease(array, 1, 10));
        equal(10, (int) instance.getAndAdd(receiver, 4));
        equal(10, (int) statics.getAndAddAcquire(4));
        equal(10, (int) element.getAndAddRelease(array, 1, 4));
        equal(14, (int) instance.getAndBitwiseOr(receiver, 3));
        equal(14, (int) statics.getAndBitwiseOrAcquire(3));
        equal(14, (int) element.getAndBitwiseOrRelease(array, 1, 3));
        equal(15, (int) instance.getAndBitwiseAnd(receiver, 10));
        equal(15, (int) statics.getAndBitwiseAndAcquire(10));
        equal(15, (int) element.getAndBitwiseAndRelease(array, 1, 10));
        equal(10, (int) instance.getAndBitwiseXor(receiver, 6));
        equal(10, (int) statics.getAndBitwiseXorAcquire(6));
        equal(10, (int) element.getAndBitwiseXorRelease(array, 1, 6));
        equal(12, receiver.field);
        equal(12, count);
        equal(12, array[1]);
        equal(0, array[0]);
        // This long lies just above a float rounding midpoint.
        long midpoint = (1L << 62) + (1L << 38) + 1;
        VarHandle longs = MethodHandles.arrayElementVarHandle(long[].class);
        long[] wide = {midpoint};
        if ((float) longs.get(wide, 0) != (float) midpoint || (double) longs.get(wide, 0) != (double) midpoint) {
            throw new AssertionError("long widening");
        }
        VarHandle floats = MethodHandles.arrayElementVarHandle(float[].class);
        float[] decimal = {-0.0f};
        if (Double.doubleToRawLongBits((double) floats.get(decimal, 0)) != Long.MIN_VALUE) {
            throw new AssertionError("float widening lost signed zero");
        }
        System.out.println("Primitive call sites passed");
    }

    static void equal(int expected, int actual) {
        if (expected != actual) throw new AssertionError("expected " + expected + ", got " + actual);
    }
}
