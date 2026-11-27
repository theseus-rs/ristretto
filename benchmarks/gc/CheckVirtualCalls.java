// Executes exactly the class files exported by jit_virtual_call_tests.rs.
// Correctness qualification only; this is not a performance benchmark.
import java.lang.reflect.InvocationTargetException;
import java.lang.reflect.Method;

public final class CheckVirtualCalls {
    private static int checks;

    private static void check(boolean condition) {
        checks++;
        if (!condition) throw new AssertionError("check " + checks);
    }

    private static void error(Class<?> expected, Class<?> target, Object receiver) throws Exception {
        Method caller = target.getMethod("caller", target);
        try {
            caller.invoke(null, receiver);
            throw new AssertionError("missing " + expected.getName());
        } catch (InvocationTargetException actual) {
            check(actual.getCause().getClass() == expected);
        }
    }

    public static void main(String[] args) throws Exception {
        VirtualTarget0[] receivers = {new VirtualTarget0(), new VirtualTarget1(),
            new VirtualTarget2(), new VirtualTarget3(), new VirtualTarget4(),
            new VirtualTarget5(), new VirtualTarget6()};
        for (int pass = 0; pass < 2; pass++) {
            for (int i = 0; i < receivers.length; i++) {
                check(VirtualTarget0.caller(receivers[i]) == Math.min(i, 5));
            }
        }
        error(NullPointerException.class, VirtualTarget0.class, null);
        check(VirtualPressureBase.caller(new VirtualPressureChild(), 8000) == 165);
        check(VirtualPrivateBase.caller(new VirtualPrivateBase()) == 7);
        check(VirtualPrivateBase.caller(new VirtualPrivateChild()) == 7);
        error(IncompatibleClassChangeError.class, VirtualErrorStatic.class, null);
        error(IncompatibleClassChangeError.class, VirtualErrorStatic.class, new VirtualErrorStatic());
        error(NoSuchMethodError.class, VirtualErrorMissing.class, null);
        error(NoSuchMethodError.class, VirtualErrorMissing.class, new VirtualErrorMissing());
        error(NullPointerException.class, VirtualErrorAbstract.class, null);
        error(AbstractMethodError.class, VirtualErrorAbstract.class, new VirtualMissingImplementation());
        check(VirtualNative.caller(new Object()) == Object.class);
        check(VirtualNative.caller(new int[] {77}) == int[].class);
        check(VirtualNative.caller(new byte[] {11}) == byte[].class);
        check(VirtualDefaultBase.caller(new VirtualDefaultBase()) == 40);
        check(VirtualDefaultBase.caller(new VirtualDefaultChild()) == 41);
        check(VirtualDefaultBase.caller(new VirtualDefaultOverride()) == 42);
        VirtualValuesBase receiver = new VirtualValuesChild();
        check(VirtualValuesBase.caller0(receiver, Long.MIN_VALUE) == Long.MIN_VALUE);
        check(VirtualValuesBase.caller0(receiver, Long.MAX_VALUE) == Long.MAX_VALUE);
        for (int bits : new int[] {0x80000000, 0x7fc00042}) {
            check(Float.floatToRawIntBits(VirtualValuesBase.caller1(receiver, Float.intBitsToFloat(bits))) == bits);
        }
        for (long bits : new long[] {0x8000000000000000L, 0x7ff8000000000042L}) {
            check(Double.doubleToRawLongBits(VirtualValuesBase.caller2(receiver, Double.longBitsToDouble(bits))) == bits);
        }
        int[] array = {77};
        check(VirtualValuesBase.caller3(receiver, array) == array);
        check(VirtualValuesBase.caller3(receiver, null) == null);
        RuntimeException throwable = new RuntimeException();
        check(VirtualThrowBasetrue.caller(new VirtualThrowChildtrue(), throwable) == throwable);
        try {
            VirtualThrowBasefalse.caller(new VirtualThrowChildfalse(), throwable);
            throw new AssertionError("virtual exception swallowed");
        } catch (RuntimeException actual) {
            check(actual == throwable);
        }
        System.out.println("PASS " + checks + " virtual-call checks");
    }
}
