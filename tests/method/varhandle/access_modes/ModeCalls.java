import java.lang.invoke.VarHandle;
import java.lang.invoke.VarHandle.AccessMode;

/** Direct signature-polymorphic calls with zero, one, and two coordinates. */
class ModeCalls {
    static Object onStatic(VarHandle handle, AccessMode mode, Object expected, Object value) {
        switch (mode) {
            case GET: return handle.get();
            case SET: handle.set(value); return null;
            case GET_VOLATILE: return handle.getVolatile();
            case SET_VOLATILE: handle.setVolatile(value); return null;
            case GET_ACQUIRE: return handle.getAcquire();
            case SET_RELEASE: handle.setRelease(value); return null;
            case GET_OPAQUE: return handle.getOpaque();
            case SET_OPAQUE: handle.setOpaque(value); return null;
            case COMPARE_AND_SET: return handle.compareAndSet(expected, value);
            case COMPARE_AND_EXCHANGE: return handle.compareAndExchange(expected, value);
            case COMPARE_AND_EXCHANGE_ACQUIRE: return handle.compareAndExchangeAcquire(expected, value);
            case COMPARE_AND_EXCHANGE_RELEASE: return handle.compareAndExchangeRelease(expected, value);
            case WEAK_COMPARE_AND_SET_PLAIN: return handle.weakCompareAndSetPlain(expected, value);
            case WEAK_COMPARE_AND_SET: return handle.weakCompareAndSet(expected, value);
            case WEAK_COMPARE_AND_SET_ACQUIRE: return handle.weakCompareAndSetAcquire(expected, value);
            case WEAK_COMPARE_AND_SET_RELEASE: return handle.weakCompareAndSetRelease(expected, value);
            case GET_AND_SET: return handle.getAndSet(value);
            case GET_AND_SET_ACQUIRE: return handle.getAndSetAcquire(value);
            case GET_AND_SET_RELEASE: return handle.getAndSetRelease(value);
            case GET_AND_ADD: return handle.getAndAdd(value);
            case GET_AND_ADD_ACQUIRE: return handle.getAndAddAcquire(value);
            case GET_AND_ADD_RELEASE: return handle.getAndAddRelease(value);
            case GET_AND_BITWISE_OR: return handle.getAndBitwiseOr(value);
            case GET_AND_BITWISE_OR_ACQUIRE: return handle.getAndBitwiseOrAcquire(value);
            case GET_AND_BITWISE_OR_RELEASE: return handle.getAndBitwiseOrRelease(value);
            case GET_AND_BITWISE_AND: return handle.getAndBitwiseAnd(value);
            case GET_AND_BITWISE_AND_ACQUIRE: return handle.getAndBitwiseAndAcquire(value);
            case GET_AND_BITWISE_AND_RELEASE: return handle.getAndBitwiseAndRelease(value);
            case GET_AND_BITWISE_XOR: return handle.getAndBitwiseXor(value);
            case GET_AND_BITWISE_XOR_ACQUIRE: return handle.getAndBitwiseXorAcquire(value);
            case GET_AND_BITWISE_XOR_RELEASE: return handle.getAndBitwiseXorRelease(value);
            default: throw new AssertionError(mode);
        }
    }

    static Object onInstance(VarHandle handle, Test.Fields receiver, AccessMode mode, Object expected, Object value) {
        switch (mode) {
            case GET: return handle.get(receiver);
            case SET: handle.set(receiver, value); return null;
            case GET_VOLATILE: return handle.getVolatile(receiver);
            case SET_VOLATILE: handle.setVolatile(receiver, value); return null;
            case GET_ACQUIRE: return handle.getAcquire(receiver);
            case SET_RELEASE: handle.setRelease(receiver, value); return null;
            case GET_OPAQUE: return handle.getOpaque(receiver);
            case SET_OPAQUE: handle.setOpaque(receiver, value); return null;
            case COMPARE_AND_SET: return handle.compareAndSet(receiver, expected, value);
            case COMPARE_AND_EXCHANGE: return handle.compareAndExchange(receiver, expected, value);
            case COMPARE_AND_EXCHANGE_ACQUIRE: return handle.compareAndExchangeAcquire(receiver, expected, value);
            case COMPARE_AND_EXCHANGE_RELEASE: return handle.compareAndExchangeRelease(receiver, expected, value);
            case WEAK_COMPARE_AND_SET_PLAIN: return handle.weakCompareAndSetPlain(receiver, expected, value);
            case WEAK_COMPARE_AND_SET: return handle.weakCompareAndSet(receiver, expected, value);
            case WEAK_COMPARE_AND_SET_ACQUIRE: return handle.weakCompareAndSetAcquire(receiver, expected, value);
            case WEAK_COMPARE_AND_SET_RELEASE: return handle.weakCompareAndSetRelease(receiver, expected, value);
            case GET_AND_SET: return handle.getAndSet(receiver, value);
            case GET_AND_SET_ACQUIRE: return handle.getAndSetAcquire(receiver, value);
            case GET_AND_SET_RELEASE: return handle.getAndSetRelease(receiver, value);
            case GET_AND_ADD: return handle.getAndAdd(receiver, value);
            case GET_AND_ADD_ACQUIRE: return handle.getAndAddAcquire(receiver, value);
            case GET_AND_ADD_RELEASE: return handle.getAndAddRelease(receiver, value);
            case GET_AND_BITWISE_OR: return handle.getAndBitwiseOr(receiver, value);
            case GET_AND_BITWISE_OR_ACQUIRE: return handle.getAndBitwiseOrAcquire(receiver, value);
            case GET_AND_BITWISE_OR_RELEASE: return handle.getAndBitwiseOrRelease(receiver, value);
            case GET_AND_BITWISE_AND: return handle.getAndBitwiseAnd(receiver, value);
            case GET_AND_BITWISE_AND_ACQUIRE: return handle.getAndBitwiseAndAcquire(receiver, value);
            case GET_AND_BITWISE_AND_RELEASE: return handle.getAndBitwiseAndRelease(receiver, value);
            case GET_AND_BITWISE_XOR: return handle.getAndBitwiseXor(receiver, value);
            case GET_AND_BITWISE_XOR_ACQUIRE: return handle.getAndBitwiseXorAcquire(receiver, value);
            case GET_AND_BITWISE_XOR_RELEASE: return handle.getAndBitwiseXorRelease(receiver, value);
            default: throw new AssertionError(mode);
        }
    }

    static Object onArray(VarHandle handle, Object array, AccessMode mode, Object expected, Object value) {
        switch (mode) {
            case GET: return handle.get(array, 1);
            case SET: handle.set(array, 1, value); return null;
            case GET_VOLATILE: return handle.getVolatile(array, 1);
            case SET_VOLATILE: handle.setVolatile(array, 1, value); return null;
            case GET_ACQUIRE: return handle.getAcquire(array, 1);
            case SET_RELEASE: handle.setRelease(array, 1, value); return null;
            case GET_OPAQUE: return handle.getOpaque(array, 1);
            case SET_OPAQUE: handle.setOpaque(array, 1, value); return null;
            case COMPARE_AND_SET: return handle.compareAndSet(array, 1, expected, value);
            case COMPARE_AND_EXCHANGE: return handle.compareAndExchange(array, 1, expected, value);
            case COMPARE_AND_EXCHANGE_ACQUIRE: return handle.compareAndExchangeAcquire(array, 1, expected, value);
            case COMPARE_AND_EXCHANGE_RELEASE: return handle.compareAndExchangeRelease(array, 1, expected, value);
            case WEAK_COMPARE_AND_SET_PLAIN: return handle.weakCompareAndSetPlain(array, 1, expected, value);
            case WEAK_COMPARE_AND_SET: return handle.weakCompareAndSet(array, 1, expected, value);
            case WEAK_COMPARE_AND_SET_ACQUIRE: return handle.weakCompareAndSetAcquire(array, 1, expected, value);
            case WEAK_COMPARE_AND_SET_RELEASE: return handle.weakCompareAndSetRelease(array, 1, expected, value);
            case GET_AND_SET: return handle.getAndSet(array, 1, value);
            case GET_AND_SET_ACQUIRE: return handle.getAndSetAcquire(array, 1, value);
            case GET_AND_SET_RELEASE: return handle.getAndSetRelease(array, 1, value);
            case GET_AND_ADD: return handle.getAndAdd(array, 1, value);
            case GET_AND_ADD_ACQUIRE: return handle.getAndAddAcquire(array, 1, value);
            case GET_AND_ADD_RELEASE: return handle.getAndAddRelease(array, 1, value);
            case GET_AND_BITWISE_OR: return handle.getAndBitwiseOr(array, 1, value);
            case GET_AND_BITWISE_OR_ACQUIRE: return handle.getAndBitwiseOrAcquire(array, 1, value);
            case GET_AND_BITWISE_OR_RELEASE: return handle.getAndBitwiseOrRelease(array, 1, value);
            case GET_AND_BITWISE_AND: return handle.getAndBitwiseAnd(array, 1, value);
            case GET_AND_BITWISE_AND_ACQUIRE: return handle.getAndBitwiseAndAcquire(array, 1, value);
            case GET_AND_BITWISE_AND_RELEASE: return handle.getAndBitwiseAndRelease(array, 1, value);
            case GET_AND_BITWISE_XOR: return handle.getAndBitwiseXor(array, 1, value);
            case GET_AND_BITWISE_XOR_ACQUIRE: return handle.getAndBitwiseXorAcquire(array, 1, value);
            case GET_AND_BITWISE_XOR_RELEASE: return handle.getAndBitwiseXorRelease(array, 1, value);
            default: throw new AssertionError(mode);
        }
    }
}
