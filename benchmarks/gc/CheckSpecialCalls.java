// Runs the exact class files exported by jit_special_call_tests.rs.
// Correctness qualification only; this is not a performance benchmark.
import java.lang.reflect.Method;
import java.util.Arrays;

public final class CheckSpecialCalls {
    private static int checks;

    private static void check(boolean condition) {
        checks++;
        if (!condition) throw new AssertionError("check " + checks);
    }

    public static void main(String[] args) throws Exception {
        check(SpecialConstructor.caller(77).value == 77);
        check(SpecialPressureConstructor.caller(8000).array[0] == 77);
        SpecialLeaf leaf = new SpecialLeaf();
        check(SpecialLeaf.caller(leaf) == 2);
        check(SpecialLeaf.caller(leaf) == 2);
        try {
            SpecialLeaf.caller(null);
            throw new AssertionError("null receiver accepted");
        } catch (NullPointerException expected) {
            check(true);
        }
        check(SpecialDefaultCaller.caller(new SpecialDefaultCaller()) == 42);
        RuntimeException throwable = new RuntimeException();
        check(SpecialThrowtrue.caller(throwable) == throwable);
        try {
            SpecialThrowfalse.caller(throwable);
            throw new AssertionError("constructor exception swallowed");
        } catch (RuntimeException actual) {
            check(actual == throwable);
        }
        SpecialWide receiver = new SpecialWide();
        check(SpecialWide.caller0(receiver, Long.MIN_VALUE) == Long.MIN_VALUE);
        check(SpecialWide.caller0(receiver, Long.MAX_VALUE) == Long.MAX_VALUE);
        for (int bits : new int[] {0x80000000, 0x7fc00042}) {
            check(Float.floatToRawIntBits(SpecialWide.caller1(receiver, Float.intBitsToFloat(bits))) == bits);
        }
        for (long bits : new long[] {0x8000000000000000L, 0x7ff8000000000042L}) {
            check(Double.doubleToRawLongBits(SpecialWide.caller2(receiver, Double.longBitsToDouble(bits))) == bits);
        }
        int[] array = {77};
        check(SpecialWide.caller3(receiver, array) == array);
        check(SpecialWide.caller3(receiver, null) == null);
        Class<?>[] types = new Class<?>[255];
        Arrays.fill(types, int.class);
        types[0] = SpecialMaximum.class;
        Object[] values = new Object[255];
        values[0] = new SpecialMaximum();
        int expected = 0;
        for (int i = 1; i < values.length; i++) {
            int value = i * 7 - 11;
            values[i] = value;
            expected = expected * 31 + value;
        }
        Method maximum = SpecialMaximum.class.getMethod("caller", types);
        check(((Integer) maximum.invoke(null, values)).intValue() == expected);
        System.out.println("PASS " + checks + " special-call checks");
    }
}
