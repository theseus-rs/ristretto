import java.lang.management.ManagementFactory;

public final class RingSamples {
    public static void main(String[] args) {
        String workload = args[0];
        int steps = Integer.parseInt(args[1]);
        int expected = Integer.parseInt(args[2]);
        int warmup = Integer.parseInt(args[3]);
        int samples = Integer.parseInt(args[4]);
        var collectors = ManagementFactory.getGarbageCollectorMXBeans();
        long[] counts = new long[collectors.size()];
        long[] times = new long[collectors.size()];
        for (var gc : collectors) {
            System.out.println("collector\t" + gc.getName());
        }
        long[] durations = new long[samples];
        int[] results = new int[samples];
        long batchStart = 0;
        for (int i = -warmup; i < samples; i++) {
            if (i == 0) {
                for (int n = 0; n < collectors.size(); n++) {
                    counts[n] = collectors.get(n).getCollectionCount();
                    times[n] = collectors.get(n).getCollectionTime();
                }
            }
            long start = System.nanoTime();
            if (i == 0) batchStart = start;
            int result = switch (workload) {
                case "small" -> CanonicalRingWorkload.small(steps);
                case "medium" -> CanonicalRingWorkload.medium(steps);
                case "large" -> CanonicalRingWorkload.large(steps);
                default -> throw new IllegalArgumentException(workload);
            };
            long elapsed = System.nanoTime() - start;
            if (result != expected) throw new AssertionError("unexpected checksum " + result);
            if (i >= 0) {
                durations[i] = elapsed;
                results[i] = result;
            }
        }
        long batch = System.nanoTime() - batchStart;
        for (int n = 0; n < collectors.size(); n++) {
            var collector = collectors.get(n);
            long count = collector.getCollectionCount();
            long time = collector.getCollectionTime();
            counts[n] = count < 0 || counts[n] < 0 ? -1 : count - counts[n];
            times[n] = time < 0 || times[n] < 0 ? -1 : time - times[n];
        }
        for (int n = 0; n < collectors.size(); n++) {
            System.out.println("gc_delta\t" + collectors.get(n).getName() + "\t" + counts[n] + "\t" + times[n]);
        }
        System.out.println("batch_ns\t" + batch);
        for (int i = 0; i < samples; i++) {
            System.out.println("sample\t" + results[i] + "\t" + durations[i]);
        }
    }
}
