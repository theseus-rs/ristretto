import java.lang.invoke.MethodHandle;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.VarHandle;
import java.lang.invoke.VarHandle.AccessMode;
import java.nio.ByteBuffer;
import java.nio.ByteOrder;
import java.util.Objects;

public class Test {
    enum Carrier {
        SHORT(short[].class, 2), CHAR(char[].class, 2), INT(int[].class, 4),
        LONG(long[].class, 8), FLOAT(float[].class, 4), DOUBLE(double[].class, 8);

        final Class<?> arrayType;
        final int width;

        Carrier(Class<?> arrayType, int width) {
            this.arrayType = arrayType;
            this.width = width;
        }

        Object value(long n) {
            switch (this) {
                case SHORT: return (short) n;
                case CHAR: return (char) n;
                case INT: return (int) n;
                case LONG: return n;
                case FLOAT: return (float) n + 0.5f;
                case DOUBLE: return (double) n + 0.5;
                default: throw new AssertionError(this);
            }
        }

        void put(ByteBuffer buffer, int index, Object value) {
            switch (this) {
                case SHORT: buffer.putShort(index, (Short) value); break;
                case CHAR: buffer.putChar(index, (Character) value); break;
                case INT: buffer.putInt(index, (Integer) value); break;
                case LONG: buffer.putLong(index, (Long) value); break;
                case FLOAT: buffer.putInt(index, Float.floatToRawIntBits((Float) value)); break;
                case DOUBLE: buffer.putLong(index, Double.doubleToRawLongBits((Double) value)); break;
                default: throw new AssertionError(this);
            }
        }
    }

    enum Operation {
        READ, WRITE, CAS, EXCHANGE, SWAP, ADD, OR, AND, XOR;

        static Operation of(AccessMode mode) {
            String name = mode.methodName();
            if (name.startsWith("compareAndExchange")) return EXCHANGE;
            if (name.startsWith("weakCompareAndSet") || mode == AccessMode.COMPARE_AND_SET) return CAS;
            if (name.startsWith("getAndSet")) return SWAP;
            if (name.startsWith("getAndAdd")) return ADD;
            if (name.startsWith("getAndBitwiseOr")) return OR;
            if (name.startsWith("getAndBitwiseAnd")) return AND;
            if (name.startsWith("getAndBitwiseXor")) return XOR;
            if (name.startsWith("set")) return WRITE;
            if (name.startsWith("get")) return READ;
            throw new AssertionError(mode);
        }

        boolean supported(Carrier type) {
            switch (this) {
                case READ: case WRITE: return true;
                case CAS: case EXCHANGE: case SWAP: return type.width >= 4;
                default: return type == Carrier.INT || type == Carrier.LONG;
            }
        }

        Object invoke(MethodHandle call, Object receiver, int index, Object initial, Object operand) throws Throwable {
            switch (this) {
                case READ: return call.invoke(receiver, index);
                case CAS: case EXCHANGE: return call.invoke(receiver, index, initial, operand);
                default: return call.invoke(receiver, index, operand);
            }
        }
    }

    interface ViewCall {
        Object invoke(int index, Object expected, Object value) throws Throwable;
    }

    public static void main(String[] args) throws Throwable {
        exerciseViews(true);
        exerciseViews(false);
        misalignedSlice();
        System.out.println("All byte view access modes passed");
    }

    static void exerciseViews(boolean directCalls) throws Throwable {
        for (ByteOrder order : new ByteOrder[]{ByteOrder.LITTLE_ENDIAN, ByteOrder.BIG_ENDIAN}) {
            for (Carrier type : Carrier.values()) {
                byte[] bytes = new byte[24];
                VarHandle arrayView = MethodHandles.byteArrayViewVarHandle(type.arrayType, order);
                exercise(type, arrayView, bytes, ByteBuffer.wrap(bytes).order(order), "array", directCalls);

                VarHandle bufferView = MethodHandles.byteBufferViewVarHandle(type.arrayType, order);
                ByteBuffer heap = ByteBuffer.allocate(24).order(order);
                exercise(type, bufferView, heap, heap, "heap", directCalls);
                ByteBuffer direct = ByteBuffer.allocateDirect(32).alignedSlice(8).order(order);
                exercise(type, bufferView, direct, direct, "direct", directCalls);
            }
        }
    }

    static void exercise(Carrier type, VarHandle view, Object receiver, ByteBuffer bytes, String backing,
                         boolean directCalls) throws Throwable {
        ByteBuffer expected = ByteBuffer.allocate(bytes.capacity()).order(bytes.order());
        boolean restrictedHeap = Runtime.version().feature() >= 25 && !bytes.isDirect();
        long start = type == Carrier.LONG ? 0x1_2345_678aL : 0x123a;
        long delta = 0x146;
        Object initial = type.value(start);
        Object operand = type.value(delta);
        for (AccessMode mode : AccessMode.values()) {
            Operation op = Operation.of(mode);
            boolean plain = mode == AccessMode.GET || mode == AccessMode.SET;
            boolean supported = op.supported(type) && (plain || !restrictedHeap || receiver instanceof ByteBuffer);
            String label = (directCalls ? "direct call " : "method handle ") + backing + " " + bytes.order() + " " + type + " " + mode;
            if (view.isAccessModeSupported(mode) != supported) throw new AssertionError(label + " support");
            ViewCall call;
            if (directCalls) {
                call = (index, old, value) -> direct(view, mode, receiver, index, old, value);
            } else {
                MethodHandle bound = view.toMethodHandle(mode);
                call = (index, old, value) -> op.invoke(bound, receiver, index, old, value);
            }
            for (int index : new int[]{8, 9}) {
                type.put(bytes, index, initial);
                type.put(expected, index, initial);
                boolean misaligned = !plain && (restrictedHeap || index % type.width != 0);
                try {
                    if (supported && !misaligned && (op == Operation.CAS || op == Operation.EXCHANGE)) {
                        Object miss = call.invoke(index, operand, operand);
                        Object expectedMiss = op == Operation.CAS ? Boolean.FALSE : initial;
                        if (!Objects.equals(miss, expectedMiss) || !bytes.equals(expected)) {
                            throw new AssertionError(label + " mismatch witness or backing bytes at " + index);
                        }
                    }
                    Object witness = call.invoke(index, initial, operand);
                    if (!supported || misaligned) throw new AssertionError(label + " accepted offset " + index);
                    Object expectedWitness = initial;
                    Object updated = operand;
                    switch (op) {
                        case READ: updated = initial; break;
                        case WRITE: expectedWitness = null; break;
                        case CAS:
                            expectedWitness = Boolean.TRUE;
                            if (mode != AccessMode.COMPARE_AND_SET && Boolean.FALSE.equals(witness)) {
                                expectedWitness = Boolean.FALSE;
                                updated = initial;
                            }
                            break;
                        case ADD: updated = type.value(start + delta); break;
                        case OR: updated = type.value(start | delta); break;
                        case AND: updated = type.value(start & delta); break;
                        case XOR: updated = type.value(start ^ delta); break;
                        default: break; // Exchange and swap return the previous value.
                    }
                    if (!Objects.equals(witness, expectedWitness)) throw new AssertionError(label + " witness " + witness);
                    type.put(expected, index, updated);
                } catch (UnsupportedOperationException e) {
                    if (supported) throw new AssertionError(label + " unexpectedly unsupported", e);
                } catch (IllegalStateException e) {
                    if (!supported || !misaligned) throw new AssertionError(label + " unexpected alignment failure", e);
                }
                if (!bytes.equals(expected)) throw new AssertionError(label + " backing bytes at offset " + index);
            }
        }
        if (!restrictedHeap && (type == Carrier.FLOAT || type == Carrier.DOUBLE)) {
            floatingComparison(type, view, receiver, bytes);
        }
    }

    static Object direct(VarHandle handle, AccessMode mode, Object receiver, int index, Object expected, Object value) {
        switch (mode) {
            case GET: return handle.get(receiver, index);
            case SET: handle.set(receiver, index, value); return null;
            case GET_VOLATILE: return handle.getVolatile(receiver, index);
            case SET_VOLATILE: handle.setVolatile(receiver, index, value); return null;
            case GET_ACQUIRE: return handle.getAcquire(receiver, index);
            case SET_RELEASE: handle.setRelease(receiver, index, value); return null;
            case GET_OPAQUE: return handle.getOpaque(receiver, index);
            case SET_OPAQUE: handle.setOpaque(receiver, index, value); return null;
            case COMPARE_AND_SET: return handle.compareAndSet(receiver, index, expected, value);
            case COMPARE_AND_EXCHANGE: return handle.compareAndExchange(receiver, index, expected, value);
            case COMPARE_AND_EXCHANGE_ACQUIRE: return handle.compareAndExchangeAcquire(receiver, index, expected, value);
            case COMPARE_AND_EXCHANGE_RELEASE: return handle.compareAndExchangeRelease(receiver, index, expected, value);
            case WEAK_COMPARE_AND_SET_PLAIN: return handle.weakCompareAndSetPlain(receiver, index, expected, value);
            case WEAK_COMPARE_AND_SET: return handle.weakCompareAndSet(receiver, index, expected, value);
            case WEAK_COMPARE_AND_SET_ACQUIRE: return handle.weakCompareAndSetAcquire(receiver, index, expected, value);
            case WEAK_COMPARE_AND_SET_RELEASE: return handle.weakCompareAndSetRelease(receiver, index, expected, value);
            case GET_AND_SET: return handle.getAndSet(receiver, index, value);
            case GET_AND_SET_ACQUIRE: return handle.getAndSetAcquire(receiver, index, value);
            case GET_AND_SET_RELEASE: return handle.getAndSetRelease(receiver, index, value);
            case GET_AND_ADD: return handle.getAndAdd(receiver, index, value);
            case GET_AND_ADD_ACQUIRE: return handle.getAndAddAcquire(receiver, index, value);
            case GET_AND_ADD_RELEASE: return handle.getAndAddRelease(receiver, index, value);
            case GET_AND_BITWISE_OR: return handle.getAndBitwiseOr(receiver, index, value);
            case GET_AND_BITWISE_OR_ACQUIRE: return handle.getAndBitwiseOrAcquire(receiver, index, value);
            case GET_AND_BITWISE_OR_RELEASE: return handle.getAndBitwiseOrRelease(receiver, index, value);
            case GET_AND_BITWISE_AND: return handle.getAndBitwiseAnd(receiver, index, value);
            case GET_AND_BITWISE_AND_ACQUIRE: return handle.getAndBitwiseAndAcquire(receiver, index, value);
            case GET_AND_BITWISE_AND_RELEASE: return handle.getAndBitwiseAndRelease(receiver, index, value);
            case GET_AND_BITWISE_XOR: return handle.getAndBitwiseXor(receiver, index, value);
            case GET_AND_BITWISE_XOR_ACQUIRE: return handle.getAndBitwiseXorAcquire(receiver, index, value);
            case GET_AND_BITWISE_XOR_RELEASE: return handle.getAndBitwiseXorRelease(receiver, index, value);
            default: throw new AssertionError(mode);
        }
    }

    static void misalignedSlice() {
        VarHandle ints = MethodHandles.byteBufferViewVarHandle(int[].class, ByteOrder.BIG_ENDIAN);
        ByteBuffer base = ByteBuffer.allocateDirect(24).alignedSlice(8);
        base.position(1);
        ByteBuffer slice = base.slice();
        try {
            int ignored = (int) ints.getVolatile(slice, 0);
            throw new AssertionError("ordered read at misaligned slice base");
        } catch (IllegalStateException expected) {}
        base.putInt(4, 0x12345678);
        int old = (int) ints.getAndAdd(slice, 3, 1);
        if (old != 0x12345678 || base.getInt(4) != 0x12345679 || base.get(3) != 0 || base.get(8) != 0) {
            throw new AssertionError("slice-relative index did not restore alignment");
        }
    }

    static void floatingComparison(Carrier type, VarHandle view, Object receiver, ByteBuffer bytes) {
        int index = 8;
        if (type == Carrier.FLOAT) {
            float nan = Float.intBitsToFloat(0x7fc01234);
            float other = Float.intBitsToFloat(0x7fc05678);
            bytes.putFloat(index, -0.0f);
            if (view.compareAndSet(receiver, index, 0.0f, nan) || !view.compareAndSet(receiver, index, -0.0f, nan)
                    || bytes.getInt(index) != 0x7fc01234 || view.compareAndSet(receiver, index, other, 1.0f)
                    || !view.compareAndSet(receiver, index, nan, 1.0f) || bytes.getFloat(index) != 1.0f) {
                throw new AssertionError("float comparison with " + bytes.order());
            }
        } else {
            double nan = Double.longBitsToDouble(0x7ff8000000001234L);
            double other = Double.longBitsToDouble(0x7ff8000000005678L);
            bytes.putDouble(index, -0.0);
            if (view.compareAndSet(receiver, index, 0.0, nan) || !view.compareAndSet(receiver, index, -0.0, nan)
                    || bytes.getLong(index) != 0x7ff8000000001234L || view.compareAndSet(receiver, index, other, 1.0)
                    || !view.compareAndSet(receiver, index, nan, 1.0) || bytes.getDouble(index) != 1.0) {
                throw new AssertionError("double comparison with " + bytes.order());
            }
        }
    }
}
