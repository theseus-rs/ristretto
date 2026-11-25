import java.lang.invoke.MethodHandle;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.MethodType;
import java.lang.invoke.VarHandle;
import java.lang.invoke.VarHandle.AccessMode;
import java.lang.reflect.Array;
import java.lang.reflect.Field;
import java.util.Arrays;

public class Test {
    static int failures;

    public static class Fields {
        public boolean flag;
        public byte b;
        public short s;
        public char c;
        public int i;
        public long l;
        public float f;
        public double d;
        public String text;
        public final boolean fixed_flag = true;
        public final byte fixed_b = 122;
        public final short fixed_s = Short.MIN_VALUE;
        public final char fixed_c = Character.MAX_VALUE;
        public final int fixed_i = 10;
        public final long fixed_l = 0x10000000aL;
        public final float fixed_f = 1.5f;
        public final double fixed_d = -3.5;
        public final String fixed_text = "before";
    }

    public static class Statics {
        public static boolean flag;
        public static byte b;
        public static short s;
        public static char c;
        public static int i;
        public static long l;
        public static float f;
        public static double d;
        public static String text;
        public static final boolean fixed_flag = true;
        public static final byte fixed_b = 122;
        public static final short fixed_s = Short.MIN_VALUE;
        public static final char fixed_c = Character.MAX_VALUE;
        public static final int fixed_i = 10;
        public static final long fixed_l = 0x10000000aL;
        public static final float fixed_f = 1.5f;
        public static final double fixed_d = -3.5;
        public static final String fixed_text = "before";
    }

    interface Call {
        Object apply(AccessMode mode, Object expected, Object value) throws Throwable;
    }

    interface Read {
        Object get() throws Exception;
    }

    interface Reset {
        void set(Object initial) throws Exception;
    }

    // A null arithmetic or bitwise result marks an unsupported operation.
    static class Values {
        public final String field;
        final Object initial, operand, sum, or, and, xor;

        Values(String field, Object initial, Object operand, Object sum, Object or, Object and, Object xor) {
            this.field = field;
            this.initial = initial;
            this.operand = operand;
            this.sum = sum;
            this.or = or;
            this.and = and;
            this.xor = xor;
        }
    }

    public static void main(String[] args) throws Throwable {
        Route[] routes = Route.values();
        String carrier = null;
        int firstRoute = 0;
        if (args.length >= 2 && args[0].equals("--carrier")) {
            carrier = args[1];
            firstRoute = 2;
        }
        if (args.length > firstRoute) {
            routes = new Route[args.length - firstRoute];
            for (int i = firstRoute; i < args.length; i++) routes[i - firstRoute] = Route.valueOf(args[i]);
        }
        Values[] cases = {
            new Values("i", 10, 6, 16, 14, 2, 12),
            new Values("l", 0x10000000aL, 6L, 0x100000010L, 0x10000000eL, 2L, 0x10000000cL),
            new Values("i", Integer.MAX_VALUE, 1, Integer.MIN_VALUE, Integer.MAX_VALUE, 1, Integer.MAX_VALUE - 1),
            new Values("l", Long.MAX_VALUE, 1L, Long.MIN_VALUE, Long.MAX_VALUE, 1L, Long.MAX_VALUE - 1L),
            new Values("b", (byte) 122, (byte) 6, Byte.MIN_VALUE, (byte) 126, (byte) 2, (byte) 124),
            new Values("s", Short.MIN_VALUE, (short) -1, Short.MAX_VALUE, (short) -1, Short.MIN_VALUE, Short.MAX_VALUE),
            new Values("c", Character.MAX_VALUE, (char) 1, (char) 0, Character.MAX_VALUE, (char) 1, (char) 65534),
            new Values("flag", true, false, null, true, false, true),
            new Values("f", 1.5f, 0.25f, 1.75f, null, null, null),
            new Values("d", -3.5, 0.25, -3.25, null, null, null),
            new Values("text", "before", "after", null, null, null, null)
        };

        Fields receiver = new Fields();
        MethodHandles.Lookup lookup = MethodHandles.lookup();
        // The extra overflow vectors reuse field names. Final storage uses the first vector for each carrier.
        java.util.Set<String> testedFinalFields = new java.util.HashSet<>();
        for (Values values : cases) {
            Field field = Fields.class.getDeclaredField(values.field);
            Class<?> type = field.getType();
            if (carrier != null && !carrier.equals(type.getSimpleName())) continue;

            VarHandle instance = lookup.findVarHandle(Fields.class, values.field, type);
            verify(routes, "instance", values, type, instance,
                (mode, expected, value) -> ModeCalls.onInstance(instance, receiver, mode, expected, value),
                () -> field.get(receiver), value -> field.set(receiver, value), receiver);

            Field staticField = Statics.class.getDeclaredField(values.field);
            VarHandle staticHandle = lookup.findStaticVarHandle(Statics.class, values.field, type);
            verify(routes, "static", values, type, staticHandle,
                (mode, expected, value) -> ModeCalls.onStatic(staticHandle, mode, expected, value),
                () -> staticField.get(null), value -> staticField.set(null, value));

            Object array = Array.newInstance(type, 3);
            VarHandle element = MethodHandles.arrayElementVarHandle(array.getClass());
            verify(routes, "array", values, type, element,
                (mode, expected, value) -> ModeCalls.onArray(element, array, mode, expected, value),
                () -> Array.get(array, 1), value -> Array.set(array, 1, value), array, 1);

            if (testedFinalFields.add(values.field)) {
                Field fixed = Fields.class.getField("fixed_" + values.field);
                VarHandle finalInstance = lookup.unreflectVarHandle(fixed);
                verify(routes, "final instance", values, type, finalInstance,
                    (mode, expected, value) -> ModeCalls.onInstance(finalInstance, receiver, mode, expected, value),
                    () -> fixed.get(receiver), null, receiver);
                Field fixedStatic = Statics.class.getField(fixed.getName());
                VarHandle finalStatic = lookup.unreflectVarHandle(fixedStatic);
                verify(routes, "final static", values, type, finalStatic,
                    (mode, expected, value) -> ModeCalls.onStatic(finalStatic, mode, expected, value),
                    () -> fixedStatic.get(null), null);
            }
        }
        if (testedFinalFields.isEmpty()) throw new AssertionError("Unknown carrier: " + carrier);
        if (failures != 0) throw new AssertionError(failures + " field/array mode checks failed");
        System.out.println("Field and array access modes passed " + Arrays.toString(routes)
                + (carrier == null ? "" : " for " + carrier));
    }

    static void verify(Route[] routes, String storage, Values v, Class<?> type, VarHandle handle, Call call,
                       Read read, Reset reset, Object... boundCoordinates) throws Throwable {
        Class<?>[] coordinates;
        if (boundCoordinates.length == 0) coordinates = new Class<?>[0];
        else if (boundCoordinates.length == 1) coordinates = new Class<?>[]{Fields.class};
        else coordinates = new Class<?>[]{boundCoordinates[0].getClass(), int.class};
        check(storage + " variable type", type, handle.varType());
        check(storage + " coordinates", Arrays.asList(coordinates), handle.coordinateTypes());
        for (AccessMode mode : AccessMode.values()) {
            String label = storage + " " + type.getSimpleName() + " " + mode.methodName();
            String name = mode.methodName();
            boolean load = name.startsWith("get") && !name.startsWith("getAnd");
            boolean store = name.startsWith("set");
            boolean weak = name.startsWith("weakCompareAndSet");
            boolean cas = weak || mode == AccessMode.COMPARE_AND_SET;
            boolean exchange = name.startsWith("compareAndExchange");
            Object updated = v.operand;
            if (load) updated = v.initial;
            else if (name.startsWith("getAndAdd")) updated = v.sum;
            else if (name.startsWith("getAndBitwiseOr")) updated = v.or;
            else if (name.startsWith("getAndBitwiseAnd")) updated = v.and;
            else if (name.startsWith("getAndBitwiseXor")) updated = v.xor;
            boolean supported = (reset != null || load) && updated != null;

            Class<?> resultType = store ? void.class : cas ? boolean.class : type;
            MethodType signature = MethodType.methodType(resultType, coordinates);
            if (!load) signature = signature.appendParameterTypes(type);
            if (cas || exchange) signature = signature.appendParameterTypes(type);
            check(label + " signature", signature, handle.accessModeType(mode));
            check(label + " supported", supported, handle.isAccessModeSupported(mode));
            for (Route route : routes) {
                String condition = label + " " + route;
                try {
                    Call operation = call;
                    if (route != Route.DIRECT) {
                        MethodHandle target = route.bind(handle, mode, signature);
                        int operands = signature.parameterCount() - coordinates.length;
                        operation = (ignored, expected, operand) ->
                                invoke(target, boundCoordinates, operands, expected, operand);
                    }
                    checkCall(operation, mode, condition, supported, signature, updated, v, read, reset);
                } catch (Exception | AssertionError | LinkageError | InternalError failure) {
                    // Each writable case resets its own storage, so later cases can still report their results.
                    failures++;
                    System.err.println(condition + ": " + failure);
                }
            }
        }
    }

    // Keep the receiver and index at the real invocation site. Capturing them in freshly
    // generated method handles would also exercise a different adapter on every iteration.
    static Object invoke(MethodHandle target, Object[] coordinates, int operands, Object expected, Object value)
            throws Throwable {
        switch (coordinates.length) {
            case 0:
                if (operands == 0) return target.invoke();
                if (operands == 1) return target.invoke(value);
                return target.invoke(expected, value);
            case 1:
                Object receiver = coordinates[0];
                if (operands == 0) return target.invoke(receiver);
                if (operands == 1) return target.invoke(receiver, value);
                return target.invoke(receiver, expected, value);
            case 2:
                Object array = coordinates[0];
                int index = (int) coordinates[1];
                if (operands == 0) return target.invoke(array, index);
                if (operands == 1) return target.invoke(array, index, value);
                return target.invoke(array, index, expected, value);
            default: throw new AssertionError("unexpected coordinate count");
        }
    }

    enum Route {
        DIRECT, TO_METHOD_HANDLE, FIND_VIRTUAL, INVOKER, EXACT_INVOKER;

        MethodHandle bind(VarHandle handle, AccessMode mode, MethodType type) throws ReflectiveOperationException {
            switch (this) {
                case TO_METHOD_HANDLE: return handle.toMethodHandle(mode);
                case FIND_VIRTUAL: return MethodHandles.publicLookup().findVirtual(VarHandle.class, mode.methodName(), type).bindTo(handle);
                case INVOKER: return MethodHandles.varHandleInvoker(mode, type).bindTo(handle);
                case EXACT_INVOKER: return MethodHandles.varHandleExactInvoker(mode, type).bindTo(handle);
                default: throw new AssertionError(this);
            }
        }
    }

    static void checkCall(Call call, AccessMode mode, String label, boolean supported, MethodType signature,
                          Object updated, Values v, Read read, Reset reset) throws Throwable {
        boolean weak = mode.methodName().startsWith("weakCompareAndSet");
        boolean cas = weak || mode == AccessMode.COMPARE_AND_SET;
        boolean exchange = mode.methodName().startsWith("compareAndExchange");
        if (reset != null) reset.set(v.initial);
        if (!supported) {
            try {
                call.apply(mode, v.initial, v.operand);
                throw new AssertionError(label + " accepted unsupported operation");
            } catch (UnsupportedOperationException expected) {
                check(label + " after rejection", v.initial, read.get());
            }
            return;
        }

        if (cas || exchange) {
            check(label + " mismatch", cas ? Boolean.FALSE : v.initial, call.apply(mode, v.operand, v.operand));
            check(label + " after mismatch", v.initial, read.get());
        }
        Object result = call.apply(mode, v.initial, v.operand);
        // A weak CAS may fail spuriously even with no competing writer.
        if (weak) {
            for (int retry = 0; Boolean.FALSE.equals(result) && retry < 10; retry++) {
                check(label + " after spurious failure", v.initial, read.get());
                result = call.apply(mode, v.initial, v.operand);
            }
            if (Boolean.FALSE.equals(result)) {
                check(label + " after final retry", v.initial, read.get());
                return;
            }
        }
        check(label + " return", cas ? Boolean.TRUE : signature.returnType() == void.class ? null : v.initial, result);
        check(label + " storage", updated, read.get());
    }

    static void check(String label, Object expected, Object actual) {
        if (!java.util.Objects.equals(expected, actual)) {
            throw new AssertionError(label + " expected " + expected + ", got " + actual);
        }
    }
}
