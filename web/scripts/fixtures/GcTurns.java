import java.lang.management.GarbageCollectorMXBean;
import java.lang.management.ManagementFactory;

public class GcTurns {
    static long cycles() {
        long total = 0;
        for (GarbageCollectorMXBean bean : ManagementFactory.getGarbageCollectorMXBeans()) {
            long value = bean.getCollectionCount();
            if (value >= 0) total += value;
        }
        return total;
    }

    public static void main(String[] args) {
        int[][] ring = new int[16][];
        long before = cycles();
        System.out.println("GC_TURNS_BEGIN");
        for (int i = 0; i < 50000; i++) {
            ring[i % ring.length] = new int[] { i };
            if (i % 4096 == 0) System.gc();
        }
        int checksum = 0;
        for (int[] slot : ring) checksum += slot[0];
        System.out.println("GC_TURNS_END cycles=" + (cycles() - before) + " checksum=" + checksum);
    }
}
