// Exact exported reference-continuation bytecode, executed independently by HotSpot.
// Correctness only: native suspension and root lifetimes are checked by the Rust VM tests.
import java.lang.reflect.InvocationTargetException;
import java.lang.reflect.Method;

public final class CheckReferenceContinuations {
    public static void main(String[] args) throws Exception {
        int checks = 0;
        for (String name : new String[] {"SuspendedLocal", "SuspendedOperand"}) {
            Method run = Class.forName(name).getMethod("run", int.class);
            for (int count : new int[] {0, 1, 3, 8000}) {
                int[] value = (int[]) run.invoke(null, count);
                if (value.length != 1 || value[0] != 77) throw new AssertionError(name);
                checks++;
            }
        }
        Class.forName("ConsumedReference").getMethod("run").invoke(null);
        checks++;
        Method failing = Class.forName("SuspendedException").getMethod("run", int.class);
        for (int count : new int[] {0, 1, 8}) {
            try {
                failing.invoke(null, count);
                throw new AssertionError("missing NegativeArraySizeException");
            } catch (InvocationTargetException error) {
                if (!(error.getCause() instanceof NegativeArraySizeException)) throw error;
            }
            checks++;
        }
        if (checks != 12) throw new AssertionError("unexpected case count: " + checks);
        System.out.println("PASS " + checks + " results from 4 exact Ristretto fixture classes");
    }
}
