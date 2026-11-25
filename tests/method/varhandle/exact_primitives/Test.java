import java.lang.invoke.*;
import java.nio.*;

public class Test {
    boolean booleanField;
    static boolean booleanStatic;
    byte byteField;
    static byte byteStatic;
    short shortField;
    static short shortStatic;
    char charField;
    static char charStatic;
    int intField;
    static int intStatic;
    long longField;
    static long longStatic;
    float floatField;
    static float floatStatic;
    double doubleField;
    static double doubleStatic;

    public static void main(String[] args) throws Exception {
        Test receiver = new Test();
        checkBoolean(receiver);
        checkByte(receiver);
        checkShort(receiver);
        checkChar(receiver);
        checkInt(receiver);
        checkLong(receiver);
        checkFloat(receiver);
        checkDouble(receiver);
        exactBuffer();
        System.out.println("Exact primitive descriptors passed");
    }

    static void checkBoolean(Test receiver) throws Exception {
        VarHandle field = MethodHandles.lookup().findVarHandle(Test.class, "booleanField", boolean.class).withInvokeExactBehavior();
        VarHandle statics = MethodHandles.lookup().findStaticVarHandle(Test.class, "booleanStatic", boolean.class).withInvokeExactBehavior();
        VarHandle element = MethodHandles.arrayElementVarHandle(boolean[].class).withInvokeExactBehavior();
        boolean[] array = new boolean[2];
        boolean value = true;
        field.set(receiver, value);
        statics.set(value);
        element.set(array, 1, value);
        mismatch(() -> field.set((Object) receiver, value));
        mismatch(() -> statics.set((Object) value));
        mismatch(() -> { Object ignored = element.get(array, 1); });
        if ((boolean) field.get(receiver) != value || (boolean) statics.get() != value || (boolean) element.get(array, 1) != value
                || receiver.booleanField != value || booleanStatic != value || array[1] != value) throw new AssertionError("boolean");
    }

    static void checkByte(Test receiver) throws Exception {
        VarHandle field = MethodHandles.lookup().findVarHandle(Test.class, "byteField", byte.class).withInvokeExactBehavior();
        VarHandle statics = MethodHandles.lookup().findStaticVarHandle(Test.class, "byteStatic", byte.class).withInvokeExactBehavior();
        VarHandle element = MethodHandles.arrayElementVarHandle(byte[].class).withInvokeExactBehavior();
        byte[] array = new byte[2];
        byte value = (byte) -7;
        field.set(receiver, value);
        statics.set(value);
        element.set(array, 1, value);
        mismatch(() -> field.set((Object) receiver, value));
        mismatch(() -> statics.set((Object) value));
        mismatch(() -> { Object ignored = element.get(array, 1); });
        if ((byte) field.get(receiver) != value || (byte) statics.get() != value || (byte) element.get(array, 1) != value
                || receiver.byteField != value || byteStatic != value || array[1] != value) throw new AssertionError("byte");
    }

    static void checkShort(Test receiver) throws Exception {
        VarHandle field = MethodHandles.lookup().findVarHandle(Test.class, "shortField", short.class).withInvokeExactBehavior();
        VarHandle statics = MethodHandles.lookup().findStaticVarHandle(Test.class, "shortStatic", short.class).withInvokeExactBehavior();
        VarHandle element = MethodHandles.arrayElementVarHandle(short[].class).withInvokeExactBehavior();
        short[] array = new short[2];
        short value = (short) -300;
        field.set(receiver, value);
        statics.set(value);
        element.set(array, 1, value);
        mismatch(() -> field.set((Object) receiver, value));
        mismatch(() -> statics.set((Object) value));
        mismatch(() -> { Object ignored = element.get(array, 1); });
        if ((short) field.get(receiver) != value || (short) statics.get() != value || (short) element.get(array, 1) != value
                || receiver.shortField != value || shortStatic != value || array[1] != value) throw new AssertionError("short");
    }

    static void checkChar(Test receiver) throws Exception {
        VarHandle field = MethodHandles.lookup().findVarHandle(Test.class, "charField", char.class).withInvokeExactBehavior();
        VarHandle statics = MethodHandles.lookup().findStaticVarHandle(Test.class, "charStatic", char.class).withInvokeExactBehavior();
        VarHandle element = MethodHandles.arrayElementVarHandle(char[].class).withInvokeExactBehavior();
        char[] array = new char[2];
        char value = '\ufedc';
        field.set(receiver, value);
        statics.set(value);
        element.set(array, 1, value);
        mismatch(() -> field.set((Object) receiver, value));
        mismatch(() -> statics.set((Object) value));
        mismatch(() -> { Object ignored = element.get(array, 1); });
        if ((char) field.get(receiver) != value || (char) statics.get() != value || (char) element.get(array, 1) != value
                || receiver.charField != value || charStatic != value || array[1] != value) throw new AssertionError("char");
    }

    static void checkInt(Test receiver) throws Exception {
        VarHandle field = MethodHandles.lookup().findVarHandle(Test.class, "intField", int.class).withInvokeExactBehavior();
        VarHandle statics = MethodHandles.lookup().findStaticVarHandle(Test.class, "intStatic", int.class).withInvokeExactBehavior();
        VarHandle element = MethodHandles.arrayElementVarHandle(int[].class).withInvokeExactBehavior();
        int[] array = new int[2];
        int value = 0x12345678;
        field.set(receiver, value);
        statics.set(value);
        element.set(array, 1, value);
        mismatch(() -> field.set((Object) receiver, value));
        mismatch(() -> statics.set((Object) value));
        mismatch(() -> { Object ignored = element.get(array, 1); });
        if ((int) field.get(receiver) != value || (int) statics.get() != value || (int) element.get(array, 1) != value
                || receiver.intField != value || intStatic != value || array[1] != value) throw new AssertionError("int");
    }

    static void checkLong(Test receiver) throws Exception {
        VarHandle field = MethodHandles.lookup().findVarHandle(Test.class, "longField", long.class).withInvokeExactBehavior();
        VarHandle statics = MethodHandles.lookup().findStaticVarHandle(Test.class, "longStatic", long.class).withInvokeExactBehavior();
        VarHandle element = MethodHandles.arrayElementVarHandle(long[].class).withInvokeExactBehavior();
        long[] array = new long[2];
        long value = 0x123456789L;
        field.set(receiver, value);
        statics.set(value);
        element.set(array, 1, value);
        mismatch(() -> field.set((Object) receiver, value));
        mismatch(() -> statics.set((Object) value));
        mismatch(() -> { Object ignored = element.get(array, 1); });
        if ((long) field.get(receiver) != value || (long) statics.get() != value || (long) element.get(array, 1) != value
                || receiver.longField != value || longStatic != value || array[1] != value) throw new AssertionError("long");
    }

    static void checkFloat(Test receiver) throws Exception {
        VarHandle field = MethodHandles.lookup().findVarHandle(Test.class, "floatField", float.class).withInvokeExactBehavior();
        VarHandle statics = MethodHandles.lookup().findStaticVarHandle(Test.class, "floatStatic", float.class).withInvokeExactBehavior();
        VarHandle element = MethodHandles.arrayElementVarHandle(float[].class).withInvokeExactBehavior();
        float[] array = new float[2];
        float value = -1.25f;
        field.set(receiver, value);
        statics.set(value);
        element.set(array, 1, value);
        mismatch(() -> field.set((Object) receiver, value));
        mismatch(() -> statics.set((Object) value));
        mismatch(() -> { Object ignored = element.get(array, 1); });
        if ((float) field.get(receiver) != value || (float) statics.get() != value || (float) element.get(array, 1) != value
                || receiver.floatField != value || floatStatic != value || array[1] != value) throw new AssertionError("float");
    }

    static void checkDouble(Test receiver) throws Exception {
        VarHandle field = MethodHandles.lookup().findVarHandle(Test.class, "doubleField", double.class).withInvokeExactBehavior();
        VarHandle statics = MethodHandles.lookup().findStaticVarHandle(Test.class, "doubleStatic", double.class).withInvokeExactBehavior();
        VarHandle element = MethodHandles.arrayElementVarHandle(double[].class).withInvokeExactBehavior();
        double[] array = new double[2];
        double value = 2.75d;
        field.set(receiver, value);
        statics.set(value);
        element.set(array, 1, value);
        mismatch(() -> field.set((Object) receiver, value));
        mismatch(() -> statics.set((Object) value));
        mismatch(() -> { Object ignored = element.get(array, 1); });
        if ((double) field.get(receiver) != value || (double) statics.get() != value || (double) element.get(array, 1) != value
                || receiver.doubleField != value || doubleStatic != value || array[1] != value) throw new AssertionError("double");
    }

    static void exactBuffer() {
        ByteBuffer buffer = ByteBuffer.allocateDirect(8).order(ByteOrder.BIG_ENDIAN);
        VarHandle view = MethodHandles.byteBufferViewVarHandle(int[].class, buffer.order()).withInvokeExactBehavior();
        view.set(buffer, 0, 19);
        mismatch(() -> view.set(buffer, 0L, 21));
        if ((int) view.get(buffer, 0) != 19 || buffer.getInt(0) != 19) throw new AssertionError("buffer");
    }

    static void mismatch(Runnable operation) {
        try {
            operation.run();
            throw new AssertionError("accepted an inexact descriptor");
        } catch (WrongMethodTypeException expected) {}
    }
}
