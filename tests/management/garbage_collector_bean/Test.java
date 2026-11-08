import java.lang.management.GarbageCollectorMXBean;
import java.lang.management.ManagementFactory;

public class Test {

    public static void main(String[] args) {
        boolean beansPresent = false;
        boolean namesPresent = true;
        boolean countersNonNegative = true;
        for (GarbageCollectorMXBean bean : ManagementFactory.getGarbageCollectorMXBeans()) {
            beansPresent = true;
            namesPresent &= bean.getName() != null;
            countersNonNegative &= bean.getCollectionCount() >= 0;
            countersNonNegative &= bean.getCollectionTime() >= 0;
        }
        System.out.println("gc beans present: " + beansPresent);
        System.out.println("gc bean names present: " + namesPresent);
        System.out.println("gc bean counters non-negative: " + countersNonNegative);
    }
}
