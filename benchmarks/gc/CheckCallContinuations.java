// Exact exported call-loop bytecode; suspension/cancellation are checked in Rust.
public final class CheckCallContinuations {
    public static void main(String[] args) throws Exception {
        Class<?> fixture = Class.forName("ResumableCallLoop");
        java.lang.reflect.Method run = fixture.getMethod("run", int.class);
        java.lang.reflect.Field count = fixture.getField("count");
        int checks = 0;
        for (int iterations : new int[] {0, 1, 3, 10000}) {
            count.setInt(null, 0);
            int value = (Integer) run.invoke(null, iterations);
            if (value != iterations * (iterations + 1) / 2) throw new AssertionError("sum");
            checks++;
            if (count.getInt(null) != iterations) throw new AssertionError("replayed call");
            checks++;
        }
        System.out.println("PASS " + checks + " results from the exact Ristretto call-loop fixture");
    }
}
