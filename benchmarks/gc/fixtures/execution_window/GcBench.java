/** Count-bounded execution-window probe. This is not the performance GcBench.
 * The ordinary runtime executes these Java calls; compiled kernels have exact
 * independently checkable entries/backedges, with calls outside both boundaries.
 */
public final class GcBench {
    static volatile int sink;
    static int kernel(int n) {
        int sum = 0;
        for (int i = 0; i < n; i++) sum += i;
        return sum;
    }
    static int outside(int x) { return x + 1; }
    public static void main(String[] args) throws InterruptedException {
        int warmup = 0;
        for (int i = 0; i < 9; i++) warmup += kernel(5) + outside(i);
        // Give the ordinary background compiler a bounded settling opportunity.
        // Readiness is independently checked; this delay is not proof of readiness.
        Thread.sleep(1000);
        long start = GcTelemetry.beginMeasurement();
        int checksum = 0;
        for (int i = 0; i < 7; i++) checksum += kernel(5);
        long elapsed = GcTelemetry.endMeasurement() - start;
        int after = 0;
        for (int i = 0; i < 11; i++) after += kernel(5) + outside(i);
        sink = warmup + checksum + after;
        System.out.println("GC_WINDOW_PROBE {\"schema_version\":1,\"warmup\":" + warmup
            + ",\"completed\":7,\"checksum\":" + checksum + ",\"after\":" + after
            + ",\"elapsed_ns\":" + elapsed + "}");
    }
}
