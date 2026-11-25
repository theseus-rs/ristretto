import java.lang.invoke.MethodHandles;
import java.lang.invoke.VarHandle.AccessMode;
import java.nio.ByteOrder;

public class Test {
    public static void main(String[] args) {
        expect(NullPointerException.class, () -> MethodHandles.arrayElementVarHandle(null));
        expect(IllegalArgumentException.class, () -> MethodHandles.arrayElementVarHandle(String.class));
        for (boolean buffer : new boolean[]{false, true}) {
            expect(NullPointerException.class, () -> view(buffer, null, ByteOrder.BIG_ENDIAN));
            expect(NullPointerException.class, () -> view(buffer, int[].class, null));
            expect(IllegalArgumentException.class, () -> view(buffer, int.class, ByteOrder.BIG_ENDIAN));
            for (Class<?> unsupported : new Class<?>[]{byte[].class, boolean[].class, Object[].class, int[][].class}) {
                expect(UnsupportedOperationException.class, () -> view(buffer, unsupported, ByteOrder.BIG_ENDIAN));
            }
        }
        java.lang.invoke.VarHandle array = MethodHandles.arrayElementVarHandle(int[].class);
        expect(UnsupportedOperationException.class, () -> array.coordinateTypes().set(1, long.class));
        expect(NullPointerException.class, () -> array.accessModeType(null));
        expect(NullPointerException.class, () -> array.isAccessModeSupported(null));
        expect(NullPointerException.class, () -> array.toMethodHandle(null));
        for (AccessMode mode : AccessMode.values()) {
            if (AccessMode.valueFromMethodName(mode.methodName()) != mode) throw new AssertionError(mode);
        }
        expect(IllegalArgumentException.class, () -> AccessMode.valueFromMethodName("GET"));
        expect(IllegalArgumentException.class, () -> AccessMode.valueFromMethodName("getAndMultiply"));
        System.out.println("VarHandle factory and mode arguments passed");
    }

    static void view(boolean buffer, Class<?> type, ByteOrder order) {
        if (buffer) MethodHandles.byteBufferViewVarHandle(type, order);
        else MethodHandles.byteArrayViewVarHandle(type, order);
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
}
