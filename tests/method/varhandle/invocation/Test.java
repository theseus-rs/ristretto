import java.lang.invoke.MethodHandle;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.MethodType;
import java.lang.invoke.VarHandle;
import java.lang.invoke.VarHandle.AccessMode;
import java.lang.invoke.WrongMethodTypeException;
import java.util.Collections;

public class Test {
    public int number = 7;
    public static int count = 11;
    public Object value = Integer.valueOf(23);
    public final int fixed = 31;

    interface Action {
        void run() throws Throwable;
    }

    public static void main(String[] args) throws Throwable {
        MethodHandles.Lookup lookup = MethodHandles.lookup();
        VarHandle field = lookup.findVarHandle(Test.class, "number", int.class);
        VarHandle staticField = lookup.findStaticVarHandle(Test.class, "count", int.class);
        Test receiver = new Test();
        MethodType get = MethodType.methodType(int.class, Test.class);
        MethodType set = MethodType.methodType(void.class, Test.class, int.class);
        MethodType cas = MethodType.methodType(boolean.class, Test.class, int.class, int.class);
        check("metadata", field.varType() == int.class && field.coordinateTypes().equals(Collections.singletonList(Test.class)));
        check("static coordinates", staticField.coordinateTypes().isEmpty());
        check("GET type", field.accessModeType(VarHandle.AccessMode.GET).equals(get));
        check("SET type", field.accessModeType(VarHandle.AccessMode.SET).equals(set));
        check("CAS type", field.accessModeType(VarHandle.AccessMode.COMPARE_AND_SET).equals(cas));

        field.set((Object) receiver, (short) 13);
        Object boxed = field.get(receiver);
        check("boxing and widening", boxed.equals(13) && (long) field.get(receiver) == 13L);
        field.set(receiver, (Object) Integer.valueOf(17));
        check("unboxing", receiver.number == 17);
        expect(NullPointerException.class, () -> field.set(receiver, (Integer) null));
        expect(NullPointerException.class, () -> { int v = (int) field.get((Test) null); });
        expect(ClassCastException.class, () -> { int v = (int) field.get((Object) "wrong"); });
        expect(WrongMethodTypeException.class, () -> { int v = (int) field.get(); });
        expect(WrongMethodTypeException.class, () -> field.set(receiver, 15L));
        expect(WrongMethodTypeException.class, () -> { boolean v = (boolean) field.get(receiver); });
        check("failed writes preserve value", receiver.number == 17);

        MethodHandle bound = field.toMethodHandle(VarHandle.AccessMode.GET);
        check("bound GET", bound.type().equals(get) && (int) bound.invokeExact(receiver) == 17);
        MethodHandle exact = MethodHandles.varHandleExactInvoker(VarHandle.AccessMode.COMPARE_AND_SET, cas);
        check("exact invoker", (boolean) exact.invokeExact(field, receiver, 17, 19) && receiver.number == 19);
        MethodType adapted = MethodType.methodType(long.class, Object.class);
        MethodHandle flexible = MethodHandles.varHandleInvoker(VarHandle.AccessMode.GET, adapted);
        check("adapting invoker", (long) flexible.invokeExact(field, (Object) receiver) == 19L);
        MethodHandle incompatible = MethodHandles.varHandleExactInvoker(VarHandle.AccessMode.GET, adapted);
        expect(WrongMethodTypeException.class, () -> { long v = (long) incompatible.invokeExact(field, (Object) receiver); });

        VarHandle value = lookup.findVarHandle(Test.class, "value", Object.class);
        expect(ClassCastException.class, () -> { String old = (String) value.getAndSet(receiver, "new"); });
        check("return cast follows the update", "new".equals(receiver.value));

        VarHandle fixed = lookup.findVarHandle(Test.class, "fixed", int.class);
        MethodHandle writeFinal = fixed.toMethodHandle(VarHandle.AccessMode.SET);
        check("bound final SET type", writeFinal.type().equals(set));
        expect(UnsupportedOperationException.class, () -> { writeFinal.invokeExact(receiver, 37); });
        check("rejected bound SET", (int) fixed.get(receiver) == 31);
        arrayInvokers();
        allInvokerModes(field, fixed, receiver);
        System.out.println("VarHandle invocation passed");
    }

    enum Invocation {
        BOUND, EXACT_INVOKER, INVOKER, FIND_VIRTUAL;

        MethodHandle bind(VarHandle target, AccessMode mode) throws ReflectiveOperationException {
            return bind(target, mode, target.accessModeType(mode));
        }

        MethodHandle bind(VarHandle target, AccessMode mode, MethodType type) throws ReflectiveOperationException {
            MethodHandle unbound;
            switch (this) {
                case BOUND: return target.toMethodHandle(mode).asType(type);
                case FIND_VIRTUAL: unbound = MethodHandles.lookup().findVirtual(VarHandle.class, mode.methodName(), type); break;
                case EXACT_INVOKER: unbound = MethodHandles.varHandleExactInvoker(mode, type); break;
                case INVOKER: unbound = MethodHandles.varHandleInvoker(mode, type); break;
                default: throw new AssertionError(this);
            }
            check(this + " receiver", unbound.type().equals(type.insertParameterTypes(0, VarHandle.class)));
            check(this + " revealed type", MethodHandles.lookup().revealDirect(unbound).getMethodType().equals(type));
            return unbound.bindTo(target);
        }
    }

    static void allInvokerModes(VarHandle field, VarHandle fixed, Test receiver) throws Throwable {
        for (Invocation route : Invocation.values()) {
            MethodType adapted = MethodType.methodType(long.class, Object.class);
            MethodHandle get = route.bind(field, AccessMode.GET, adapted);
            if (route == Invocation.EXACT_INVOKER) {
                expect(WrongMethodTypeException.class, () -> { long v = (long) get.invokeExact((Object) receiver); });
            } else {
                check("adapted " + route, (long) get.invokeExact((Object) receiver) == receiver.number);
            }
            for (AccessMode mode : AccessMode.values()) {
                MethodHandle call = route.bind(field, mode);
                String label = route + " " + mode;
                String name = mode.methodName();
                receiver.number = 10;
                int updated = 6;
                if (name.startsWith("set")) {
                    call.invokeExact(receiver, 6);
                } else if (name.startsWith("compareAndSet") || name.startsWith("weakCompareAndSet")) {
                    check(label + " mismatch", !(boolean) call.invokeExact(receiver, 6, 6) && receiver.number == 10);
                    if (!(boolean) call.invokeExact(receiver, 10, 6)) {
                        check(label + " strong CAS", name.startsWith("weak"));
                        updated = 10;
                    }
                } else if (name.startsWith("compareAndExchange")) {
                    check(label + " mismatch", (int) call.invokeExact(receiver, 6, 6) == 10 && receiver.number == 10);
                    check(label + " witness", (int) call.invokeExact(receiver, 10, 6) == 10);
                } else if (name.startsWith("getAnd")) {
                    check(label + " witness", (int) call.invokeExact(receiver, 6) == 10);
                    if (name.startsWith("getAndAdd")) updated = 16;
                    if (name.startsWith("getAndBitwiseOr")) updated = 14;
                    if (name.startsWith("getAndBitwiseAnd")) updated = 2;
                    if (name.startsWith("getAndBitwiseXor")) updated = 12;
                } else {
                    check(label, (int) call.invokeExact(receiver) == 10);
                    updated = 10;
                }
                check(label + " storage", receiver.number == updated);
            }
            MethodHandle write = route.bind(fixed, AccessMode.SET);
            expect(UnsupportedOperationException.class, () -> { write.invokeExact(receiver, 5); });
            check(route + " final field", (int) fixed.get(receiver) == 31);
        }
    }

    static void arrayInvokers() throws Throwable {
        VarHandle element = MethodHandles.arrayElementVarHandle(long[].class);
        long high = 1L << 40;
        for (Invocation route : Invocation.values()) {
            long[] array = {11, 0};
            MethodHandle get = route.bind(element, AccessMode.GET);
            MethodHandle set = route.bind(element, AccessMode.SET);
            MethodHandle exchange = route.bind(element, AccessMode.COMPARE_AND_EXCHANGE);
            set.invokeExact(array, 1, high);
            check(route + " read", (long) get.invokeExact(array, 1) == high);
            check(route + " mismatch", (long) exchange.invokeExact(array, 1, 0L, 9L) == high && array[1] == high);
            check(route + " match", (long) exchange.invokeExact(array, 1, high, high + 1) == high);
            check(route + " storage", array[1] == high + 1 && array[0] == 11);
        }
    }

    static void expect(Class<? extends Throwable> type, Action action) throws Throwable {
        try {
            action.run();
        } catch (Throwable t) {
            if (t.getClass() == type) return;
            throw t;
        }
        throw new AssertionError("expected " + type.getName());
    }

    static void check(String label, boolean condition) {
        if (!condition) throw new AssertionError(label);
    }
}
