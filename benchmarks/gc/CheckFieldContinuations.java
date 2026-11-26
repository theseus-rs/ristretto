import java.lang.reflect.InvocationTargetException;
import java.lang.reflect.Method;

/** Correctness oracle for exact VM field fixtures; no performance measurements. */
public final class CheckFieldContinuations {
    private static int checks;
    private static void check(boolean condition) {
        if (!condition) throw new AssertionError("check " + checks);
        checks++;
    }
    private static void same(Object actual, Object expected) {
        if (expected instanceof Float) {
            check(Float.floatToRawIntBits((Float) actual) == Float.floatToRawIntBits((Float) expected));
        } else if (expected instanceof Double) {
            check(Double.doubleToRawLongBits((Double) actual) == Double.doubleToRawLongBits((Double) expected));
        } else if (expected == null || expected instanceof int[]) {
            check(actual == expected);
        } else {
            check(expected.equals(actual));
        }
    }
    public static void main(String[] args) throws Exception {
        int[] reference = {99};
        String[] labels = {"Z", "B", "C", "S", "I", "J", "F", "D", "R"};
        Class<?>[] types = {int.class, int.class, int.class, int.class, int.class,
                           long.class, float.class, double.class, int[].class};
        Object[][] inputs = {{42, -1}, {255, 128}, {-1, 0xd800}, {65535, 32768},
            {Integer.MIN_VALUE, Integer.MAX_VALUE}, {Long.MIN_VALUE, Long.MAX_VALUE},
            {Float.intBitsToFloat(0x7fc00042), -0.0f},
            {Double.longBitsToDouble(0x7ff8000000000042L), -0.0d}, {reference, null}};
        Object[][] expected = {{0, 1}, {-1, -128}, {65535, 0xd800}, {-1, -32768},
            inputs[4], inputs[5], inputs[6], inputs[7], inputs[8]};
        FieldContinuations receiver = new FieldContinuations();
        for (int i = 0; i < labels.length; i++) {
            Method stat = FieldContinuations.class.getMethod("static" + labels[i], types[i]);
            Method inst = FieldContinuations.class.getMethod("instance" + labels[i], FieldContinuations.class, types[i]);
            for (int j = 0; j < 2; j++) {
                same(stat.invoke(null, inputs[i][j]), expected[i][j]);
                same(inst.invoke(null, receiver, inputs[i][j]), expected[i][j]);
            }
        }
        // The symbolic child's initializer loops forever. Successful access proves
        // only the actual declaring class was initialized.
        check(FieldResolutionCaller.readInherited() == 7);
        check(FieldResolutionCaller.writeInherited() == 9);
        check(FieldResolutionCaller.readInherited() == 9);
        String[] names = {"wrongStatic", "wrongInstance", "wrongDescriptor", "privateNull", "validNull", "writeFinal"};
        Class<?>[] errors = {IncompatibleClassChangeError.class, IncompatibleClassChangeError.class,
            NoSuchFieldError.class, IllegalAccessError.class, NullPointerException.class, IllegalAccessError.class};
        for (int i = 0; i < names.length; i++) {
            Method method = FieldResolutionCaller.class.getMethod(names[i]);
            for (int attempt = 0; attempt < 2; attempt++) {
                try {
                    method.invoke(null);
                    throw new AssertionError(names[i] + " did not throw");
                } catch (InvocationTargetException error) {
                    check(error.getCause().getClass() == errors[i]);
                }
            }
        }
        check(FieldResolutionBase.immutable == 0);
        System.out.println("field checks passed: " + checks);
    }
}
