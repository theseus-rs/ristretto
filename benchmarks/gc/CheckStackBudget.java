// Runs the exact exported stack-budget fixtures on HotSpot. VM stack sizes and
// exhaustion depths differ; these checks do not compare their capacity or speed.
import java.lang.reflect.InvocationTargetException;
import java.lang.reflect.Method;

public final class CheckStackBudget {
    private static int checks;

    private static void check(boolean condition) {
        checks++;
        if (!condition) throw new AssertionError("check " + checks);
    }

    private static void exercise(Class<?> target) throws Exception {
        Method even = target.getMethod("even", int.class, int[].class);
        Method caught = target.getMethod("caught", int.class, int[].class);
        int[] array = {77};
        check(even.invoke(null, 0, array) == array);
        check(even.invoke(null, 4, array) == array);
        check(caught.invoke(null, 8, array) == null);
        Throwable overflow = (Throwable) caught.invoke(null, Integer.MAX_VALUE, array);
        check(overflow.getClass() == StackOverflowError.class);
        StackTraceElement[] trace = overflow.getStackTrace();
        check(trace.length > 0);
        check(trace[0].getClassName().equals(target.getName()));
        check(trace[0].getMethodName().equals("even") || trace[0].getMethodName().equals("odd"));
        try {
            even.invoke(null, Integer.MAX_VALUE, array);
            throw new AssertionError("overflow was swallowed");
        } catch (InvocationTargetException error) {
            check(error.getCause().getClass() == StackOverflowError.class);
        }
        // These wrappers check holdsLock inside their explicit monitor region.
        Method nativeEven = target.getMethod("nativeEven", int.class, int[].class);
        Method nativeCaught = target.getMethod("nativeCaught", int.class, int[].class);
        check(nativeEven.invoke(null, 3, array) == array);
        check(!Thread.holdsLock(array));
        Throwable nativeOverflow = (Throwable) nativeCaught.invoke(null, Integer.MAX_VALUE, array);
        check(nativeOverflow.getClass() == StackOverflowError.class);
        check(!Thread.holdsLock(array));
        try {
            nativeEven.invoke(null, Integer.MAX_VALUE, array);
            throw new AssertionError("native wrapper swallowed overflow");
        } catch (InvocationTargetException error) {
            check(error.getCause().getClass() == StackOverflowError.class);
        }
        check(!Thread.holdsLock(array));
        // Synchronized fixtures must have released every recursive monitor entry.
        final Throwable[] failure = new Throwable[1];
        Thread recovery = new Thread(() -> {
            try {
                if (nativeEven.invoke(null, 3, array) != array) throw new AssertionError("reference changed");
            } catch (Throwable error) {
                failure[0] = error;
            }
        });
        recovery.setDaemon(true);
        recovery.start();
        recovery.join(5000);
        check(!recovery.isAlive() && failure[0] == null);
    }

    public static void main(String[] args) throws Exception {
        check(CompiledStackBounds.value() == 7);
        exercise(CompiledStackRecursionfalse.class);
        exercise(CompiledStackRecursiontrue.class);
        check(CompiledStackPressure.even(4, 8000) == 15);
        System.out.println("PASS " + checks + " stack-budget fixture checks");
    }
}
