import java.lang.management.ManagementFactory;

public final class RingSamples {
    public static void main(String[] args) {
        String workload = args[0];
        int steps = Integer.parseInt(args[1]);
        int expected = Integer.parseInt(args[2]);
        int warmup = Integer.parseInt(args[3]);
        int samples = Integer.parseInt(args[4]);
        for (var gc : ManagementFactory.getGarbageCollectorMXBeans()) {
            System.out.println("collector\t" + gc.getName());
        }
        for (int i = -warmup; i < samples; i++) {
            long start = System.nanoTime();
            int result = switch (workload) {
                case "small" -> CanonicalRingWorkload.small(steps);
                case "medium" -> CanonicalRingWorkload.medium(steps);
                case "large" -> CanonicalRingWorkload.large(steps);
                default -> throw new IllegalArgumentException(workload);
            };
            long elapsed = System.nanoTime() - start;
            if (result != expected) throw new AssertionError("unexpected checksum " + result);
            if (i >= 0) System.out.println("sample\t" + result + "\t" + elapsed);
        }
    }
}
