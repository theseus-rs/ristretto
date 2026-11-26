import java.lang.reflect.InvocationTargetException;
import java.lang.reflect.Method;

/** Independent semantics for the exact generated reference-store fixtures. */
public final class ReferenceStoreOracle {
    private static int checks;
    private static void check(boolean condition, String label) {
        if (!condition) throw new AssertionError(label);
        checks++;
    }
    private static void store(Method method, Object[] array, int index, Object value,
                              Class<? extends Throwable> expected) throws Exception {
        Object before = array == null ? null : array[0];
        try {
            method.invoke(null, array, index, value);
            check(expected == null, "missing " + expected);
            check(array[index] == value, "wrong stored identity");
        } catch (InvocationTargetException failure) {
            check(expected != null && failure.getCause().getClass() == expected,
                    "expected " + expected + ", received " + failure.getCause());
            check(array == null || array[0] == before, "failed store changed destination");
        }
    }
    public static void main(String[] args) throws Exception {
        Class<?> fixture = Class.forName("ReferenceStoreContinuations");
        Method store = fixture.getMethod("store", Object[].class, int.class, Object.class);
        String[] strings = new String[1];
        Object wrong = new Object();
        store(store, null, -1, wrong, NullPointerException.class);
        store(store, strings, -1, wrong, ArrayIndexOutOfBoundsException.class);
        store(store, strings, 1, wrong, ArrayIndexOutOfBoundsException.class);
        store(store, strings, 0, wrong, ArrayStoreException.class);
        store(store, strings, 0, "value", null);
        store(store, strings, 0, null, null);
        store(store, new Object[1], 0, new int[] {77}, null);
        store(store, new Object[1][], 0, new String[1], null);
        store(store, new Object[1][], 0, new int[1], ArrayStoreException.class);
        store(store, new Cloneable[1][], 0, new int[1][], null);
        store(store, new java.io.Serializable[1][], 0, new int[1][], null);
        store(store, new Object[1][][], 0, new int[1][][], null);
        store(store, new int[1][], 0, new long[1], ArrayStoreException.class);
        Method wide = fixture.getMethod("wide", Object[].class, int.class, Object.class, long.class);
        Object[] target = new Object[1];
        for (long bits : new long[] {Long.MIN_VALUE, -1, 0, 42, Long.MAX_VALUE}) {
            check(wide.invoke(null, target, 0, wrong, bits).equals(bits * 2), "wide state");
            check(target[0] == wrong, "wide method lost its store");
        }
        Method caught = fixture.getMethod("caught", Object[].class, int.class, Object.class);
        check(caught.invoke(null, new String[1], 0, wrong).equals(0), "caught type error");
        check(caught.invoke(null, target, 0, wrong).equals(1), "successful protected store");
        System.out.println("reference-store oracle passed " + checks + " checks");
    }
}
