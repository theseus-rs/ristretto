// Executes the exact class files exported by jit_primitive_control_flow_tests.rs.
// Correctness evidence only; this is not a performance benchmark.
public final class CheckPrimitiveControlFlow {
    private static int checks;

    private static Object call(String name, Class<?>[] types, Object... args) throws Exception {
        return Class.forName(name).getMethod("run", types).invoke(null, args);
    }

    private static void check(Object actual, Object expected) {
        if (!actual.equals(expected)) throw new AssertionError(actual + " != " + expected);
        checks++;
    }

    private static boolean predicate(int index, int a, int b) {
        switch (index) {
            case 0: return a == b;
            case 1: return a != b;
            case 2: return a < b;
            case 3: return a >= b;
            case 4: return a > b;
            default: return a <= b;
        }
    }

    public static void main(String[] args) throws Exception {
        int[] ints = {Integer.MIN_VALUE, -1, 0, 1, Integer.MAX_VALUE};
        Class<?>[] unary = {int.class}, binary = {int.class, int.class};
        for (int index = 0; index < 6; index++) {
            for (int a : ints) {
                check(call("ForwardZeroCondition" + index, unary, a), predicate(index, a, 0) ? 1 : 0);
                for (int b : ints) {
                    check(call("ForwardPairCondition" + index, binary, a, b), predicate(index, a, b) ? 1 : 0);
                }
            }
        }
        for (String name : new String[] {"ForwardGoto", "ForwardWideGoto"}) {
            for (int branch : new int[] {-1, 0, 1}) {
                for (int value : ints) {
                    check(call(name, binary, branch, value), branch == 0 ? value : -value);
                }
            }
        }
        int[] lows = {Integer.MIN_VALUE, -1, Integer.MAX_VALUE - 2};
        for (int index = 0; index < lows.length; index++) {
            int low = lows[index];
            for (int key : new int[] {Integer.MIN_VALUE, -2, -1, 0, 1, 2, Integer.MAX_VALUE, low, low + 1, low + 2}) {
                check(call("ForwardTable" + index, unary, key), key >= low && key <= low + 2 ? 6 + (key - low) : 9);
            }
        }
        for (int key : new int[] {Integer.MIN_VALUE, -2, -1, 0, 1, 2, Integer.MAX_VALUE}) {
            check(call("ForwardLookup", unary, key), key == Integer.MIN_VALUE || key == Integer.MAX_VALUE ? 6 : key == -1 ? 7 : 8);
            check(call("ForwardEmptyLookup", unary, key), 6);
        }
        for (int key : new int[] {-1, 0, 1}) {
            check(call("ForwardLongMerge", unary, key), key == 0 ? 2L : 1L);
            check(call("ForwardDoubleMerge", unary, key), key == 0 ? 2.0 : 1.0);
            check(call("ForwardLocalReuse", unary, key), key == 0 ? 2 : 1);
        }
        long[] longs = {Long.MIN_VALUE, -1, 0, 1, Long.MAX_VALUE};
        for (long a : longs) {
            for (long b : longs) {
                check(call("ForwardLongCompare", new Class<?>[] {long.class, long.class}, a, b), a >= b ? 1 : 0);
            }
        }
        double[] values = {Double.NEGATIVE_INFINITY, -1.0, -0.0, 0.0, 1.0, Double.POSITIVE_INFINITY, Double.NaN};
        for (double a : values) {
            for (double b : values) {
                check(call("ForwardFloatCompare", new Class<?>[] {float.class, float.class}, (float) a, (float) b), a < b ? 1 : 0);
                check(call("ForwardDoubleCompare", new Class<?>[] {double.class, double.class}, a, b), a > b ? 1 : 0);
            }
        }
        for (int input : new int[] {0, 100000}) check(call("PrimitiveLoop", unary, input), 0);
        if (checks != 388) throw new AssertionError("unexpected case count: " + checks);
        System.out.println("PASS " + checks + " results from 26 exact Ristretto fixture classes");
    }
}
