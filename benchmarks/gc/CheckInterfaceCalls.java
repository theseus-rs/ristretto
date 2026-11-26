// Executes the exact class files exported by jit_interface_call_tests.rs.
// Correctness checks only; no performance conclusions follow from this driver.
import java.lang.reflect.InvocationTargetException;
import java.lang.reflect.Method;

public final class CheckInterfaceCalls {
    private static int checks;

    private static void check(boolean condition) {
        checks++;
        if (!condition) throw new AssertionError("check " + checks);
    }

    private static void error(String name, Class<?> expected, Object receiver) throws Exception {
        Method caller = Class.forName("InterfaceError" + name + "Caller").getMethod("caller", name.equals("Class") ? Class.forName("InterfaceErrorClass") : Object.class);
        try {
            caller.invoke(null, receiver);
            throw new AssertionError("missing " + expected.getName());
        } catch (InvocationTargetException actual) {
            check(actual.getCause().getClass() == expected);
        }
    }

    public static void main(String[] args) throws Exception {
        InterfaceValue[] receivers = {new InterfaceTarget0(), new InterfaceTarget1(),
            new InterfaceTarget2(), new InterfaceTarget3(), new InterfaceTarget4(),
            new InterfaceTarget5(), new InterfaceTarget6()};
        for (int pass = 0; pass < 2; pass++) {
            for (int i = 0; i < receivers.length; i++) {
                check(InterfaceDispatch.caller(receivers[i]) == Math.min(i, 5));
            }
        }
        check(InterfaceDefaultCaller.caller(new InterfaceDefaultBase()) == 40);
        check(InterfaceDefaultCaller.caller(new InterfaceDefaultChild()) == 41);
        check(InterfaceDefaultCaller.caller(new InterfaceDefaultOverride()) == 42);
        String[] errors = {"Static", "Missing", "Class", "Abstract", "NonPublic", "NonMember"};
        Class<?>[] expected = {IncompatibleClassChangeError.class, NoSuchMethodError.class,
            IncompatibleClassChangeError.class, AbstractMethodError.class,
            IllegalAccessError.class, IncompatibleClassChangeError.class};
        for (int i = 0; i < errors.length; i++) {
            Object receiver = Class.forName("InterfaceError" + errors[i] + "Impl").getConstructor().newInstance();
            error(errors[i], i < 3 ? expected[i] : NullPointerException.class, null);
            error(errors[i], expected[i], receiver);
        }
        check(InterfacePressureCaller.caller(new InterfacePressureImpl(), 8000) == 165);
        check(InterfaceNative.caller(new int[] {77}) == int[].class);
        check(InterfaceNative.caller(new byte[] {11}) == byte[].class);
        InterfaceValues receiver = new InterfaceValuesImpl();
        check(InterfaceValuesCaller.caller0(receiver, Long.MIN_VALUE) == Long.MIN_VALUE);
        check(InterfaceValuesCaller.caller0(receiver, Long.MAX_VALUE) == Long.MAX_VALUE);
        for (int bits : new int[] {0x80000000, 0x7fc00042}) {
            check(Float.floatToRawIntBits(InterfaceValuesCaller.caller1(receiver, Float.intBitsToFloat(bits))) == bits);
        }
        for (long bits : new long[] {0x8000000000000000L, 0x7ff8000000000042L}) {
            check(Double.doubleToRawLongBits(InterfaceValuesCaller.caller2(receiver, Double.longBitsToDouble(bits))) == bits);
        }
        int[] array = {77};
        check(InterfaceValuesCaller.caller3(receiver, array) == array);
        check(InterfaceValuesCaller.caller3(receiver, null) == null);
        RuntimeException throwable = new RuntimeException();
        check(InterfaceThrowCallertrue.caller(new InterfaceThrowImpltrue(), throwable) == throwable);
        try {
            InterfaceThrowCallerfalse.caller(new InterfaceThrowImplfalse(), throwable);
            throw new AssertionError("interface exception swallowed");
        } catch (RuntimeException actual) {
            check(actual == throwable);
        }
        System.out.println("PASS " + checks + " interface-call checks");
    }
}
