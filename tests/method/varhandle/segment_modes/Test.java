import java.lang.foreign.Arena;
import java.lang.foreign.MemoryLayout;
import java.lang.foreign.MemorySegment;
import java.lang.foreign.ValueLayout;
import java.lang.invoke.MethodHandle;
import java.lang.invoke.VarHandle;
import java.lang.invoke.VarHandle.AccessMode;
import java.nio.ByteBuffer;
import java.nio.ByteOrder;
import java.util.Arrays;

public class Test {
    public static void main(String[] args) throws Throwable {
        try (Arena arena = Arena.ofConfined()) {
            scalar(arena, ValueLayout.JAVA_BOOLEAN, false, true, MemorySegment.ofArray(new byte[]{1}));
            scalar(arena, ValueLayout.JAVA_BYTE, (byte) 3, (byte) -7, MemorySegment.ofArray(new byte[]{-7}));
            scalar(arena, ValueLayout.JAVA_CHAR, 'a', '\ufedc', MemorySegment.ofArray(new char[]{'\ufedc'}));
            scalar(arena, ValueLayout.JAVA_SHORT, (short) 5, (short) -300, MemorySegment.ofArray(new short[]{-300}));
            scalar(arena, ValueLayout.JAVA_INT, 7, -12345, MemorySegment.ofArray(new int[]{-12345}));
            scalar(arena, ValueLayout.JAVA_LONG, 9L, 1L << 42, MemorySegment.ofArray(new long[]{1L << 42}));
            scalar(arena, ValueLayout.JAVA_FLOAT, 1.25f, -0.0f, MemorySegment.ofArray(new float[]{-0.0f}));
            scalar(arena, ValueLayout.JAVA_DOUBLE, -2.5, 3.75, MemorySegment.ofArray(new double[]{3.75}));
            addresses(arena);
            exactSegment(arena);
            unaligned(arena);
            for (MemorySegment segment : new MemorySegment[]{MemorySegment.ofArray(new long[3]), arena.allocate(24, 8)}) {
                for (ByteOrder order : new ByteOrder[]{ByteOrder.BIG_ENDIAN, ByteOrder.LITTLE_ENDIAN}) {
                    wordModes(segment, ValueLayout.JAVA_LONG.withOrder(order));
                    wordModes(segment, ValueLayout.JAVA_LONG_UNALIGNED.withOrder(order));
                    floatingComparisons(segment, order);
                }
            }
        }
        stridedPath();
        signedHeapByte();
        System.out.println("Segment access modes passed");
    }

    static void wordModes(MemorySegment segment, ValueLayout.OfLong layout) throws Throwable {
        VarHandle handle = layout.varHandle();
        ByteBuffer bytes = ByteBuffer.allocate(24).order(layout.order());
        MemorySegment initialBytes = MemorySegment.ofArray(bytes.array());
        long initial = 0x1122334455667788L;
        long operand = 0x0f0f030305050707L;
        long offset = 8;
        bytes.putLong(0, 19L);
        bytes.putLong(16, 23L);
        for (AccessMode mode : AccessMode.values()) {
            String label = (segment.isNative() ? "native " : "heap ") + layout + " " + mode;
            MethodHandle call = handle.toMethodHandle(mode);
            bytes.putLong(8, initial);
            segment.copyFrom(initialBytes);
            long updated = initial;
            boolean supported = layout.byteAlignment() == Long.BYTES || mode == AccessMode.GET || mode == AccessMode.SET;
            try {
                switch (mode) {
                    case GET, GET_OPAQUE, GET_ACQUIRE, GET_VOLATILE ->
                            equalWord(label, initial, (long) call.invokeExact(segment, offset));
                    case SET, SET_OPAQUE, SET_RELEASE, SET_VOLATILE -> {
                        call.invokeExact(segment, offset, operand);
                        updated = operand;
                    }
                    case COMPARE_AND_SET, WEAK_COMPARE_AND_SET, WEAK_COMPARE_AND_SET_PLAIN,
                            WEAK_COMPARE_AND_SET_ACQUIRE, WEAK_COMPARE_AND_SET_RELEASE -> {
                        if ((boolean) call.invokeExact(segment, offset, operand, operand)) throw new AssertionError(label + " mismatch");
                        checkBytes(label + " after mismatch", bytes, segment);
                        boolean swapped = (boolean) call.invokeExact(segment, offset, initial, operand);
                        // A weak CAS can fail spuriously with a matching expected value.
                        if (swapped) updated = operand;
                        else if (mode == AccessMode.COMPARE_AND_SET) throw new AssertionError(label + " failed to match");
                    }
                    case COMPARE_AND_EXCHANGE, COMPARE_AND_EXCHANGE_ACQUIRE, COMPARE_AND_EXCHANGE_RELEASE -> {
                        equalWord(label, initial, (long) call.invokeExact(segment, offset, operand, operand));
                        checkBytes(label + " after mismatch", bytes, segment);
                        equalWord(label, initial, (long) call.invokeExact(segment, offset, initial, operand));
                        updated = operand;
                    }
                    case GET_AND_SET, GET_AND_SET_ACQUIRE, GET_AND_SET_RELEASE -> {
                        equalWord(label, initial, (long) call.invokeExact(segment, offset, operand));
                        updated = operand;
                    }
                    case GET_AND_ADD, GET_AND_ADD_ACQUIRE, GET_AND_ADD_RELEASE -> {
                        equalWord(label, initial, (long) call.invokeExact(segment, offset, operand));
                        updated = initial + operand;
                    }
                    case GET_AND_BITWISE_AND, GET_AND_BITWISE_AND_ACQUIRE, GET_AND_BITWISE_AND_RELEASE -> {
                        equalWord(label, initial, (long) call.invokeExact(segment, offset, operand));
                        updated = initial & operand;
                    }
                    case GET_AND_BITWISE_OR, GET_AND_BITWISE_OR_ACQUIRE, GET_AND_BITWISE_OR_RELEASE -> {
                        equalWord(label, initial, (long) call.invokeExact(segment, offset, operand));
                        updated = initial | operand;
                    }
                    case GET_AND_BITWISE_XOR, GET_AND_BITWISE_XOR_ACQUIRE, GET_AND_BITWISE_XOR_RELEASE -> {
                        equalWord(label, initial, (long) call.invokeExact(segment, offset, operand));
                        updated = initial ^ operand;
                    }
                    default -> throw new AssertionError(mode);
                }
                if (!supported) throw new AssertionError(label + " accepted on an unaligned layout");
            } catch (UnsupportedOperationException failure) {
                if (supported) throw new AssertionError(label, failure);
            }
            bytes.putLong(8, updated);
            checkBytes(label, bytes, segment);
        }
    }

    static void floatingComparisons(MemorySegment segment, ByteOrder order) {
        ByteBuffer bytes = ByteBuffer.allocate(24).order(order);
        MemorySegment initialBytes = MemorySegment.ofArray(bytes.array());
        VarHandle floats = ValueLayout.JAVA_FLOAT.withOrder(order).varHandle();
        int floatBits = 0x7fc01234;
        bytes.putInt(8, floatBits);
        segment.copyFrom(initialBytes);
        float miss = (float) floats.compareAndExchange(segment, 8L, Float.intBitsToFloat(floatBits + 1), 1.0f);
        equalWord("float NaN witness", floatBits, Float.floatToRawIntBits(miss));
        checkBytes("float after mismatch", bytes, segment);
        if (!floats.compareAndSet(segment, 8L, Float.intBitsToFloat(floatBits), -0.0f)) throw new AssertionError("float NaN match");
        bytes.putInt(8, Integer.MIN_VALUE);
        checkBytes("float after match", bytes, segment);

        VarHandle doubles = ValueLayout.JAVA_DOUBLE.withOrder(order).varHandle();
        long doubleBits = 0x7ff8000000001234L;
        bytes.putLong(8, doubleBits);
        segment.copyFrom(initialBytes);
        double previous = (double) doubles.compareAndExchange(segment, 8L, Double.longBitsToDouble(doubleBits + 1), 1.0);
        equalWord("double NaN witness", doubleBits, Double.doubleToRawLongBits(previous));
        checkBytes("double after mismatch", bytes, segment);
        if (!doubles.compareAndSet(segment, 8L, Double.longBitsToDouble(doubleBits), -0.0)) throw new AssertionError("double NaN match");
        bytes.putLong(8, Long.MIN_VALUE);
        checkBytes("double after match", bytes, segment);
    }

    static void checkBytes(String label, ByteBuffer expected, MemorySegment actual) {
        for (int i = 0; i < expected.capacity(); i++) {
            int wanted = Byte.toUnsignedInt(expected.get(i));
            int observed = Byte.toUnsignedInt(actual.get(ValueLayout.JAVA_BYTE, i));
            if (wanted != observed) throw new AssertionError(label + " byte " + i + " expected " + wanted + ", got " + observed);
        }
    }

    static void equalWord(String label, long expected, long actual) {
        if (expected != actual) throw new AssertionError(label + " expected " + expected + ", got " + actual);
    }

    static void checkSupport(ValueLayout layout) {
        VarHandle handle = layout.varHandle();
        Class<?> type = layout.carrier();
        if (handle.varType() != type || !handle.coordinateTypes().equals(Arrays.asList(MemorySegment.class, long.class))) {
            throw new AssertionError("access type for " + layout);
        }
        boolean integral = type == int.class || type == long.class || type == MemorySegment.class;
        boolean atomic = integral || type == float.class || type == double.class;
        boolean aligned = layout.byteAlignment() >= layout.byteSize();
        for (AccessMode mode : AccessMode.values()) {
            boolean supported = switch (mode) {
                case GET, SET -> true;
                case GET_ACQUIRE, SET_RELEASE, GET_OPAQUE, SET_OPAQUE, GET_VOLATILE, SET_VOLATILE -> aligned;
                case COMPARE_AND_SET, COMPARE_AND_EXCHANGE, COMPARE_AND_EXCHANGE_ACQUIRE, COMPARE_AND_EXCHANGE_RELEASE,
                        WEAK_COMPARE_AND_SET, WEAK_COMPARE_AND_SET_PLAIN, WEAK_COMPARE_AND_SET_ACQUIRE,
                        WEAK_COMPARE_AND_SET_RELEASE, GET_AND_SET, GET_AND_SET_ACQUIRE, GET_AND_SET_RELEASE -> aligned && atomic;
                case GET_AND_ADD, GET_AND_ADD_ACQUIRE, GET_AND_ADD_RELEASE, GET_AND_BITWISE_OR, GET_AND_BITWISE_OR_ACQUIRE,
                        GET_AND_BITWISE_OR_RELEASE, GET_AND_BITWISE_AND, GET_AND_BITWISE_AND_ACQUIRE, GET_AND_BITWISE_AND_RELEASE,
                        GET_AND_BITWISE_XOR, GET_AND_BITWISE_XOR_ACQUIRE, GET_AND_BITWISE_XOR_RELEASE -> aligned && integral;
            };
            if (handle.isAccessModeSupported(mode) != supported) throw new AssertionError(layout + " " + mode);
        }
    }

    static void scalar(Arena arena, ValueLayout layout, Object initial, Object next, MemorySegment expectedBytes) {
        checkSupport(layout);
        VarHandle handle = layout.varHandle();
        MemorySegment slot = arena.allocate(layout);
        handle.setRelease(slot, 0L, initial);
        if (!initial.equals(handle.getAcquire(slot, 0L))) throw new AssertionError("published " + layout);
        if (handle.isAccessModeSupported(AccessMode.COMPARE_AND_EXCHANGE)) {
            Object miss = handle.compareAndExchange(slot, 0L, next, next);
            if (!initial.equals(miss) || !initial.equals(handle.get(slot, 0L))) throw new AssertionError("miss " + layout);
            Object hit = handle.compareAndExchange(slot, 0L, initial, next);
            if (!initial.equals(hit)) throw new AssertionError("exchange witness " + layout);
        } else {
            try {
                handle.compareAndExchange(slot, 0L, initial, next);
                throw new AssertionError("unsupported exchange accepted for " + layout);
            } catch (UnsupportedOperationException expected) {
                if (!initial.equals(handle.get(slot, 0L))) throw new AssertionError("rejected exchange " + layout);
            }
            handle.setOpaque(slot, 0L, next);
        }
        if (!next.equals(handle.get(slot, 0L))) throw new AssertionError("updated value " + layout);
        for (long offset = 0; offset < layout.byteSize(); offset++) {
            int actual = Byte.toUnsignedInt(slot.get(ValueLayout.JAVA_BYTE, offset));
            int expected = Byte.toUnsignedInt(expectedBytes.get(ValueLayout.JAVA_BYTE, offset));
            if (actual != expected) throw new AssertionError(layout + " byte " + offset + " expected " + expected + ", got " + actual);
        }
    }

    static void exactSegment(Arena arena) {
        VarHandle handle = ValueLayout.JAVA_DOUBLE.varHandle().withInvokeExactBehavior();
        MemorySegment memory = arena.allocate(ValueLayout.JAVA_DOUBLE);
        handle.set(memory, 0L, 2.75);
        try {
            handle.set(memory, 0L, 3.0f);
            throw new AssertionError("accepted float operand");
        } catch (java.lang.invoke.WrongMethodTypeException expected) {}
        if ((double) handle.get(memory, 0L) != 2.75 || memory.get(ValueLayout.JAVA_DOUBLE, 0) != 2.75) {
            throw new AssertionError("exact segment storage");
        }
    }

    static void addresses(Arena arena) {
        checkSupport(ValueLayout.ADDRESS);
        VarHandle addresses = ValueLayout.ADDRESS.varHandle();
        MemorySegment slot = arena.allocate(ValueLayout.ADDRESS);
        MemorySegment value = arena.allocate(8);
        MemorySegment alias = value.asSlice(0, 1);
        addresses.set(slot, 0L, value);
        MemorySegment miss = (MemorySegment) addresses.compareAndExchange(slot, 0L, MemorySegment.NULL, MemorySegment.NULL);
        if (miss.address() != value.address() || alias == value) throw new AssertionError("address witness or alias");
        if (!addresses.compareAndSet(slot, 0L, alias, MemorySegment.NULL) || slot.get(ValueLayout.ADDRESS, 0L).address() != 0) {
            throw new AssertionError("CAS by address");
        }
    }

    static void unaligned(Arena arena) {
        checkSupport(ValueLayout.JAVA_INT_UNALIGNED);
        MemorySegment bytes = arena.allocate(8, 4);
        VarHandle unaligned = ValueLayout.JAVA_INT_UNALIGNED.varHandle();
        unaligned.set(bytes, 1L, 0x12345678);
        try {
            // Even a physically aligned address has only plain access through this layout.
            unaligned.setRelease(bytes, 0L, 17);
            throw new AssertionError("ordered access through unaligned layout");
        } catch (UnsupportedOperationException expected) {
            if ((int) unaligned.get(bytes, 1L) != 0x12345678) throw new AssertionError("unaligned storage");
        }
    }

    static void stridedPath() {
        int[] words = {11, 12, 13, 14, 15, 16};
        MemorySegment segment = MemorySegment.ofArray(words);
        VarHandle odd = MemoryLayout.sequenceLayout(6, ValueLayout.JAVA_INT)
                .varHandle(MemoryLayout.PathElement.sequenceElement(1, 2));
        int old = (int) odd.getAndAdd(segment, 0L, 1L, 20);
        if (old != 14 || !Arrays.equals(words, new int[]{11, 12, 13, 34, 15, 16})) {
            throw new AssertionError("strided sequence update");
        }
        try {
            int ignored = (int) odd.get(segment, 0L, 3L);
            throw new AssertionError("invalid stride index accepted");
        } catch (IndexOutOfBoundsException expected) {}
    }

    static void signedHeapByte() {
        byte[] bytes = {11, -7};
        MemorySegment heap = MemorySegment.ofArray(bytes);
        VarHandle element = ValueLayout.JAVA_BYTE.varHandle();
        // Widen the byte returned at the call site so an incorrect zero extension is observable.
        int value = (byte) element.get(heap, 1L);
        if (value != bytes[1]) throw new AssertionError("signed heap byte expected -7, got " + value);
        Byte boxed = (Byte) element.get(heap, 1L);
        if (boxed.intValue() != -7) throw new AssertionError("boxed heap byte " + boxed);
    }
}
