import java.lang.reflect.InvocationTargetException;
import java.lang.reflect.Method;

/** Checks the exact class files exported by the exception-match continuation fixture. */
public final class ExceptionMatchOracle {
    private static int checks;

    private static void require(boolean condition, String description) {
        if (!condition) throw new AssertionError(description);
        checks++;
    }

    public static void main(String[] args) throws Exception {
        Throwable[] values = {new NullPointerException(), new ArithmeticException(), new Error()};
        for (String name : new String[] {"OrderedExceptionMatchfalse",
                "SuspendedExceptionMatchfalse", "SuspendedExceptionMatchtrue"}) {
            Method run = Class.forName(name).getMethod("run", Throwable.class, Object.class);
            for (Throwable value : values) {
                Object local = new int[] {77};
                Object result = run.invoke(null, value, local);
                require(result == (value instanceof NullPointerException ? local : null),
                        name + " ordered matching for " + value.getClass().getName());
            }
        }
        Method simple = Class.forName("ExceptionMatchRequest").getMethod("run", Throwable.class);
        for (Throwable value : values) {
            require(simple.invoke(null, value).equals(1), "Throwable superclass match");
        }
        Method noInit = Class.forName("NoInitExceptionMatch").getMethod("run", Throwable.class, Object.class);
        Object local = new int[] {55};
        // DormantExceptionTarget's initializer loops forever. Resolving that catch
        // class must return without initializing it, including on repeated lookup.
        require(noInit.invoke(null, values[0], local) == local, "catch resolution without initialization");
        require(noInit.invoke(null, values[0], local) == local, "repeated no-init lookup");
        for (Throwable value : values) {
            try {
                // HotSpot may resolve catch types during verification. Both early
                // resolution and handler lookup must expose the linkage failure.
                Class.forName("OrderedExceptionMatchtrue")
                        .getMethod("run", Throwable.class, Object.class).invoke(null, value, local);
                throw new AssertionError("missing catch class was swallowed by catch-all");
            } catch (NoClassDefFoundError error) {
                require(true, "early catch resolution failure");
            } catch (InvocationTargetException error) {
                require(error.getCause() instanceof NoClassDefFoundError, "handler resolution failure");
            }
        }
        System.out.println("exception match oracle passed " + checks + " checks");
    }
}
