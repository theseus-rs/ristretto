import java.lang.management.ManagementFactory;
import com.sun.management.ThreadMXBean;

/** Peer-only launcher; its management calls are outside the shared workload. */
public class CanonicalRingOracle {
    private static volatile int sink;

    private static int run(String workload, int steps) {
        return switch (workload) {
            case "small" -> CanonicalRingWorkload.small(steps);
            case "medium" -> CanonicalRingWorkload.medium(steps);
            case "large" -> CanonicalRingWorkload.large(steps);
            default -> throw new IllegalArgumentException(workload);
        };
    }

    public static void main(String[] args) {
        if (args.length == 0) {
            for (String workload : new String[]{"small", "medium", "large"}) {
                for (int steps : new int[]{-1, 0, 1, 7, 17, 65, 257}) {
                    System.out.println(workload + "\t" + steps + "\t" + run(workload, steps));
                }
            }
            return;
        }
        if (args.length != 4 || !args[0].equals("--allocation-evidence")) {
            throw new IllegalArgumentException("--allocation-evidence small|medium|large steps repetitions");
        }
        String workload = args[1];
        int steps = Integer.parseInt(args[2]);
        int repetitions = Integer.parseInt(args[3]);
        if (steps <= 0 || repetitions <= 0) throw new IllegalArgumentException("positive limits required");
        ThreadMXBean threads = (ThreadMXBean) ManagementFactory.getThreadMXBean();
        if (!threads.isThreadAllocatedMemorySupported()) throw new IllegalStateException("allocation counter unavailable");
        if (!threads.isThreadAllocatedMemoryEnabled()) threads.setThreadAllocatedMemoryEnabled(true);
        for (int i = 0; i < 100; i++) sink = run(workload, steps);
        long id = Thread.currentThread().getId();
        long before = threads.getThreadAllocatedBytes(id);
        int checksum = 0;
        for (int i = 0; i < repetitions; i++) checksum ^= run(workload, steps);
        long after = threads.getThreadAllocatedBytes(id);
        sink = checksum;
        if (before < 0 || after <= before) throw new IllegalStateException("no verified allocation exposure");
        System.out.println("{\"schema\":1,\"workload\":\"" + workload + "\",\"steps\":" + steps
            + ",\"repetitions\":" + repetitions + ",\"allocated_bytes\":" + (after-before)
            + ",\"checksum\":" + checksum + ",\"qualification\":false}");
    }
}
