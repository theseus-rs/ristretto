/** Shared boundary bytecode. Peers execute these normal Java implementations.
 * Experimental Ristretto instrumentation intercepts only these explicit calls.
 * No observer is invoked from the per-operation loop.
 */
public final class GcTelemetry {
    private GcTelemetry() { }
    public static long beginMeasurement() { return System.nanoTime(); }
    public static long endMeasurement() { return System.nanoTime(); }
}
