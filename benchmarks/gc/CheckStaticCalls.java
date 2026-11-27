// Executes the class files exported by jit_static_call_tests.rs without rewriting them.
// Correctness qualification only; this is not a performance benchmark.
import java.lang.reflect.Method;
import java.util.Arrays;

public final class CheckStaticCalls {
    private static int checks;

    private static void check(boolean condition) {
        checks++;
        if (!condition) throw new AssertionError("check " + checks);
    }

    private static void initializeFailingClass(String name) throws Exception {
        try {
            Class.forName(name, true, CheckStaticCalls.class.getClassLoader());
            throw new AssertionError("initializer did not run: " + name);
        } catch (ExceptionInInitializerError expected) {
            check(expected.getCause() instanceof NullPointerException);
        }
    }

    public static void main(String[] args) throws Exception {
        int[] array = {77};
        check(StaticReferenceCall.caller(Long.MIN_VALUE, -0.0f, Double.POSITIVE_INFINITY, array) == array);
        check(StaticReferenceCall.caller(0, 0, 0, null) == null);
        check(StaticPrimitive0.caller(Integer.MIN_VALUE) == Integer.MIN_VALUE);
        check(StaticPrimitive0.caller(Integer.MAX_VALUE) == Integer.MAX_VALUE);
        check(StaticPrimitive1.caller(Long.MIN_VALUE) == Long.MIN_VALUE);
        check(StaticPrimitive1.caller(Long.MAX_VALUE) == Long.MAX_VALUE);
        for (int bits : new int[] {0x80000000, 0x7fc00042}) {
            check(Float.floatToRawIntBits(StaticPrimitive2.caller(Float.intBitsToFloat(bits))) == bits);
        }
        for (long bits : new long[] {0x8000000000000000L, 0x7ff8000000000042L}) {
            check(Double.doubleToRawLongBits(StaticPrimitive3.caller(Double.longBitsToDouble(bits))) == bits);
        }
        check(StaticInitCaller.caller() == 42);
        initializeFailingClass("StaticInitChild");
        check(StaticKindCallertrue.caller() == 42);
        try {
            StaticKindCallerfalse.caller();
            throw new AssertionError("instance target was accepted");
        } catch (IncompatibleClassChangeError expected) {
            check(true);
        }
        initializeFailingClass("InstanceTarget");
        check(StaticNativeCaller.caller(array) == System.identityHashCode(array));
        check(StaticNativeCaller.caller(null) == 0);
        RuntimeException throwable = new RuntimeException();
        check(StaticThrowfalsetrue.caller(throwable) == throwable);
        check(StaticThrowtruetrue.caller(throwable) == throwable);
        try {
            StaticThrowfalsefalse.caller(throwable);
            throw new AssertionError("missing exception");
        } catch (RuntimeException actual) {
            check(actual == throwable);
        }
        try {
            StaticThrowtruefalse.caller(throwable);
            throw new AssertionError("missing exception");
        } catch (RuntimeException actual) {
            check(actual == throwable);
        }
        check(StaticPressureCall.caller(8000)[0] == 77);
        Class<?>[] types = new Class<?>[255];
        Arrays.fill(types, int.class);
        Object[] values = new Object[255];
        int expected = 0;
        for (int i = 0; i < values.length; i++) {
            int value = i * 7 - 11;
            values[i] = value;
            expected = expected * 31 + value;
        }
        Method maximum = StaticMaximumArguments.class.getMethod("caller", types);
        check(((Integer) maximum.invoke(null, values)).intValue() == expected * 2);
        System.out.println("PASS " + checks + " static-call checks");
    }
}
