import java.io.IOException;
import java.lang.invoke.MethodHandle;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.MethodType;
import java.lang.invoke.VarHandle;
import java.util.Arrays;

public class Test {
    int number = 7;
    final int fixed = 7;
    static int decode(String value) { return Integer.parseInt(value); }
    static int checkedDecode(String value) throws IOException { return decode(value); }
    static String encode(int value) { return Integer.toString(value); }
    static int index(int base, int delta) { return base + delta; }
    static int parses, formats, visits;
    static int parse(int radix, String value) { parses++; return Integer.parseInt(value, radix); }
    static String format(int radix, int value) { formats++; return Integer.toString(value, radix); }
    static void visit(int index) {
        visits++;
        if (index < 0) throw new IllegalArgumentException("negative index");
    }
    static String encodePositive(int value) {
        if (value < 0) throw new IllegalStateException("negative witness");
        return encode(value);
    }

    public static void main(String[] args) throws Exception {
        MethodHandles.Lookup lookup = MethodHandles.lookup();
        MethodHandle decode = lookup.findStatic(Test.class, "decode", MethodType.methodType(int.class, String.class));
        MethodHandle encode = lookup.findStatic(Test.class, "encode", MethodType.methodType(String.class, int.class));
        MethodHandle index = lookup.findStatic(Test.class, "index", MethodType.methodType(int.class, int.class, int.class));
        VarHandle array = MethodHandles.arrayElementVarHandle(int[].class);
        try {
            array.compareAndSet(new int[1], (Object) null, (Object) true, (Object) false);
            throw new AssertionError("invalid compareAndSet operands accepted");
        } catch (ClassCastException expected) {
        } catch (NullPointerException wrongOrder) {
            throw new AssertionError("index unboxed before replacement", wrongOrder);
        }
        try {
            array.compareAndSet(new int[1], (Object) false, (Object) true, (Object) null);
            throw new AssertionError("null replacement accepted");
        } catch (NullPointerException expected) {}
        try {
            MethodHandles.insertCoordinates(array, 3, 0);
            throw new AssertionError("accepted invalid coordinate position");
        } catch (IllegalArgumentException expected) {}
        try {
            MethodHandles.insertCoordinates(array, 0, "wrong receiver");
            throw new AssertionError("accepted invalid bound receiver");
        } catch (ClassCastException expected) {}
        VarHandle strings = MethodHandles.filterValue(array, decode, encode);
        VarHandle indexed = MethodHandles.filterCoordinates(strings, 1, decode);
        VarHandle swapped = MethodHandles.permuteCoordinates(indexed, Arrays.asList(String.class, int[].class), 1, 0);
        int[] left = {1, 11};
        int[] right = {3, 11};
        VarHandle first = bind(swapped, left);
        VarHandle second = bind(swapped, right);
        for (int i = 0; i < 3; i++) {
            exchange(first, i + 11);
            exchange(second, i + 11);
            if (left[1] != i + 12 || right[1] != i + 12) throw new AssertionError("bound receivers");
        }
        VarHandle collected = MethodHandles.collectCoordinates(array, 1, index);
        if ((int) collected.getAndAdd(left, -1, 1, 4) != 1 || left[0] != 5 || right[0] != 3) {
            throw new AssertionError("collected coordinates");
        }
        try {
            first.set(0L, "1", "invalid");
            throw new AssertionError("value filter was skipped");
        } catch (NumberFormatException expected) {
            if (left[1] != 14) throw new AssertionError("invalid input changed storage");
        }
        try {
            MethodHandles.permuteCoordinates(array, array.coordinateTypes(), 1, 0);
            throw new AssertionError("swapped incompatible coordinate types");
        } catch (IllegalArgumentException expected) {}
        try {
            MethodHandles.filterValue(array, decode, decode);
            throw new AssertionError("incompatible result filter");
        } catch (IllegalArgumentException expected) {}

        MethodHandle positive = lookup.findStatic(Test.class, "encodePositive", encode.type());
        VarHandle checked = MethodHandles.filterValue(array, decode, positive);
        left[0] = -1;
        try {
            String old = (String) checked.getAndSet(left, 0, "23");
            throw new AssertionError("unexpected outgoing value " + old);
        } catch (IllegalStateException expected) {
            if (!"negative witness".equals(expected.getMessage()) || left[0] != 23) {
                throw new AssertionError("outgoing filter must run after the exchange", expected);
            }
        }
        filteredCas(lookup, array);
        contextualFilters(lookup, array);
        duplicateCoordinate(lookup, collected);
        filteredFieldModes(lookup, decode, encode);
        System.out.println("VarHandle adapters passed");
    }

    static void filteredFieldModes(MethodHandles.Lookup lookup, MethodHandle decode, MethodHandle encode) throws Exception {
        Test receiver = new Test();
        VarHandle field = lookup.findVarHandle(Test.class, "number", int.class);
        VarHandle fixed = lookup.findVarHandle(Test.class, "fixed", int.class);
        VarHandle writable = MethodHandles.filterValue(field, decode, encode);
        VarHandle readOnly = MethodHandles.filterValue(fixed, decode, encode);
        if (!writable.isAccessModeSupported(VarHandle.AccessMode.GET_AND_ADD)
                || readOnly.isAccessModeSupported(VarHandle.AccessMode.SET)) {
            throw new AssertionError("filtered update support");
        }
        for (VarHandle.AccessMode mode : VarHandle.AccessMode.values()) {
            if (writable.isAccessModeSupported(mode) != field.isAccessModeSupported(mode)
                    || readOnly.isAccessModeSupported(mode) != fixed.isAccessModeSupported(mode)) {
                throw new AssertionError("filtered field support for " + mode);
            }
        }
        String previous = (String) writable.getAndAdd(receiver, "2");
        if (!previous.equals("7") || receiver.number != 9) throw new AssertionError("filtered addition");
        try {
            readOnly.set(receiver, "11");
            throw new AssertionError("filtered final field accepted a write");
        } catch (UnsupportedOperationException expected) {
            if ((int) fixed.get(receiver) != 7) throw new AssertionError("filtered final field changed");
        }
        MethodHandle checked = lookup.findStatic(Test.class, "checkedDecode", decode.type());
        try {
            MethodHandles.filterValue(field, checked, encode);
            throw new AssertionError("accepted a filter declaring IOException");
        } catch (IllegalArgumentException expected) {}
    }

    static void filteredCas(MethodHandles.Lookup lookup, VarHandle array) throws Exception {
        MethodHandle parse = lookup.findStatic(Test.class, "parse", MethodType.methodType(int.class, int.class, String.class));
        MethodHandle format = lookup.findStatic(Test.class, "format", MethodType.methodType(String.class, int.class, int.class));
        VarHandle hex = MethodHandles.filterValue(array, parse, format).withInvokeExactBehavior();
        int[] values = {26};
        parses = formats = 0;
        try {
            boolean ignored = hex.compareAndSet((Object) values, 0, 16, "1a", "2f");
            throw new AssertionError("adapted an exact receiver");
        } catch (java.lang.invoke.WrongMethodTypeException expected) {}
        if (parses != 0 || formats != 0 || values[0] != 26) throw new AssertionError("filters ran before type check");
        if (hex.compareAndSet(values, 0, 16, "ff", "2f") || values[0] != 26 || parses != 2 || formats != 0) {
            throw new AssertionError("mismatching filtered CAS");
        }
        MethodHandle cas = hex.toMethodHandle(VarHandle.AccessMode.COMPARE_AND_SET);
        try {
            if (!(boolean) cas.invokeExact(values, 0, 16, "1a", "2f")) throw new AssertionError("matching filtered CAS");
        } catch (Throwable failure) {
            throw new AssertionError("bound CAS", failure);
        }
        if (values[0] != 47 || parses != 4 || formats != 0) throw new AssertionError("CAS filter counts");
        parses = formats = 0;
    }

    static void contextualFilters(MethodHandles.Lookup lookup, VarHandle array) throws Exception {
        MethodHandle parse = lookup.findStatic(Test.class, "parse", MethodType.methodType(int.class, int.class, String.class));
        MethodHandle format = lookup.findStatic(Test.class, "format", MethodType.methodType(String.class, int.class, int.class));
        VarHandle hex = MethodHandles.filterValue(array, parse, format);
        if (!hex.coordinateTypes().equals(Arrays.asList(int[].class, int.class, int.class)) || hex.varType() != String.class) {
            throw new AssertionError("radix coordinate or value type");
        }
        int[] values = {26};
        String old = (String) hex.compareAndExchange(values, 0, 16, "1a", "2f");
        if (!old.equals("1a") || values[0] != 47 || parses != 2 || formats != 1) {
            throw new AssertionError("contextual exchange filters");
        }
        try {
            hex.set(values, 0, 8, "9");
            throw new AssertionError("invalid octal input accepted");
        } catch (NumberFormatException expected) {
            if (values[0] != 47 || parses != 3 || formats != 1) throw new AssertionError("failed input filter");
        }
    }

    static void duplicateCoordinate(MethodHandles.Lookup lookup, VarHandle collected) throws Exception {
        MethodHandle visit = lookup.findStatic(Test.class, "visit", MethodType.methodType(void.class, int.class));
        // collected indexes by base + delta. Supply the same index to both, and to the side-effect filter.
        VarHandle observed = MethodHandles.collectCoordinates(collected, 1, visit);
        if (!observed.coordinateTypes().equals(Arrays.asList(int[].class, int.class, int.class, int.class))) {
            throw new AssertionError("void collector coordinates");
        }
        VarHandle twice = MethodHandles.permuteCoordinates(observed, Arrays.asList(int[].class, int.class), 0, 1, 1, 1);
        int[] values = {3, 5, 7};
        if (!twice.coordinateTypes().equals(Arrays.asList(int[].class, int.class))
                || (int) twice.getAndSet(values, 1, 11) != 7 || !Arrays.equals(values, new int[]{3, 5, 11}) || visits != 1) {
            throw new AssertionError("duplicated index or side effect");
        }
        try {
            twice.set(values, -1, 13);
            throw new AssertionError("void filter did not reject the index");
        } catch (IllegalArgumentException expected) {
            if (visits != 2 || !Arrays.equals(values, new int[]{3, 5, 11})) throw new AssertionError("failed void filter");
        }
    }

    static VarHandle bind(VarHandle target, int[] receiver) {
        VarHandle bound = MethodHandles.insertCoordinates(target, 1, (Object) receiver);
        return MethodHandles.dropCoordinates(bound, 0, long.class);
    }

    static void exchange(VarHandle handle, int old) {
        String witness = (String) handle.compareAndExchange(99L, "1", encode(old), encode(old + 1));
        if (!witness.equals(encode(old)) || !((String) handle.get(0L, "1")).equals(encode(old + 1))) {
            throw new AssertionError("adapted compareAndExchange");
        }
    }
}
