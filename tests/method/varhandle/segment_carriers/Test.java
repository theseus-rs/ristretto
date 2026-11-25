import java.lang.foreign.*;
import java.lang.invoke.*;
import java.lang.invoke.VarHandle.AccessMode;
import java.nio.*;
import java.util.Arrays;

public class Test {
    static final ValueLayout[] LAYOUTS = {ValueLayout.JAVA_BOOLEAN, ValueLayout.JAVA_BYTE, ValueLayout.JAVA_SHORT,
            ValueLayout.JAVA_CHAR, ValueLayout.JAVA_INT, ValueLayout.JAVA_FLOAT, ValueLayout.JAVA_LONG,
            ValueLayout.JAVA_DOUBLE, ValueLayout.ADDRESS};

    public static void main(String[] args) throws Throwable {
        try (Arena arena = Arena.ofConfined()) {
            for (MemorySegment memory : new MemorySegment[]{MemorySegment.ofArray(new long[3]), arena.allocate(24, 8)}) {
                for (ValueLayout layout : LAYOUTS) {
                    for (ByteOrder order : new ByteOrder[]{ByteOrder.LITTLE_ENDIAN, ByteOrder.BIG_ENDIAN}) {
                        allModes(memory, layout.withOrder(order));
                        if (layout.byteSize() > 1) allModes(memory, layout.withOrder(order).withByteAlignment(1));
                    }
                }
            }
        }
        System.out.println("All scalar segment carriers passed");
    }

    // Address carriers represent numerical pointer values here. They are never dereferenced.
    static Object value(Class<?> type, int n) {
        if (type == boolean.class) return n != 0;
        if (type == byte.class) return (byte) n;
        if (type == char.class) return (char) n;
        if (type == short.class) return (short) n;
        if (type == int.class) return n;
        if (type == float.class) return (float) n;
        if (type == long.class) return (long) n;
        if (type == double.class) return (double) n;
        if (type == MemorySegment.class) return MemorySegment.ofAddress(n);
        throw new AssertionError(type);
    }

    static boolean same(Object actual, Object expected) {
        if (expected instanceof MemorySegment address) {
            return actual instanceof MemorySegment observed && observed.address() == address.address();
        }
        return expected.equals(actual);
    }

    static void encode(byte[] bytes, ValueLayout layout, int n) {
        ByteBuffer view = ByteBuffer.wrap(bytes).order(layout.order());
        Class<?> type = layout.carrier();
        if (type == float.class) view.putFloat(8, n);
        else if (type == double.class) view.putDouble(8, n);
        else switch ((int) layout.byteSize()) {
            case 1 -> view.put(8, (byte) n);
            case 2 -> view.putShort(8, (short) n);
            case 4 -> view.putInt(8, n);
            case 8 -> view.putLong(8, n);
            default -> throw new AssertionError(layout);
        }
    }

    static void allModes(MemorySegment memory, ValueLayout layout) throws Throwable {
        Class<?> type = layout.carrier();
        VarHandle handle = layout.varHandle();
        boolean aligned = layout.byteAlignment() >= layout.byteSize();
        boolean integral = type == int.class || type == long.class || type == MemorySegment.class;
        boolean atomic = integral || type == float.class || type == double.class;
        int initial = type == boolean.class ? 0 : 10, operand = type == boolean.class ? 1 : 6;
        Object oldValue = value(type, initial), next = value(type, operand);
        byte[] expected = new byte[24];
        for (AccessMode mode : AccessMode.values()) {
            String label = layout + " " + mode + (memory.isNative() ? " native" : " heap");
            String name = mode.methodName();
            boolean store = name.startsWith("set");
            boolean read = name.startsWith("get") && !name.startsWith("getAnd");
            boolean cas = name.startsWith("weak") || mode == AccessMode.COMPARE_AND_SET;
            boolean exchange = name.startsWith("compareAndExchange");
            boolean swap = name.startsWith("getAndSet");
            boolean supported = mode == AccessMode.GET || mode == AccessMode.SET
                    || aligned && (read || store || ((cas || exchange || swap) ? atomic : integral));
            if (handle.isAccessModeSupported(mode) != supported) throw new AssertionError(label + " support");
            Arrays.fill(expected, (byte) 0x5a);
            encode(expected, layout, initial);
            memory.copyFrom(MemorySegment.ofArray(expected));
            MethodHandle call = handle.toMethodHandle(mode);
            int updated = initial;
            try {
                if (read) {
                    if (!same(call.invoke(memory, 8L), oldValue)) throw new AssertionError(label + " read");
                } else if (store) {
                    call.invoke(memory, 8L, next);
                    updated = operand;
                } else if (cas || exchange) {
                    Object miss = call.invoke(memory, 8L, next, next);
                    if (!same(miss, cas ? Boolean.FALSE : oldValue) || memory.mismatch(MemorySegment.ofArray(expected)) != -1) {
                        throw new AssertionError(label + " mismatch");
                    }
                    Object hit = call.invoke(memory, 8L, oldValue, next);
                    if (cas && Boolean.FALSE.equals(hit) && name.startsWith("weak")) {
                        updated = initial;
                    } else {
                        if (!same(hit, cas ? Boolean.TRUE : oldValue)) throw new AssertionError(label + " witness");
                        updated = operand;
                    }
                } else {
                    if (!same(call.invoke(memory, 8L, next), oldValue)) throw new AssertionError(label + " witness");
                    if (swap) updated = operand;
                    else if (name.startsWith("getAndAdd")) updated = initial + operand;
                    else if (name.startsWith("getAndBitwiseOr")) updated = initial | operand;
                    else if (name.startsWith("getAndBitwiseAnd")) updated = initial & operand;
                    else if (name.startsWith("getAndBitwiseXor")) updated = initial ^ operand;
                    else throw new AssertionError(mode);
                }
                if (!supported) throw new AssertionError(label + " accepted unsupported mode");
            } catch (UnsupportedOperationException failure) {
                if (supported) throw new AssertionError(label, failure);
            }
            encode(expected, layout, updated);
            if (memory.mismatch(MemorySegment.ofArray(expected)) != -1) throw new AssertionError(label + " storage");
        }
    }
}
