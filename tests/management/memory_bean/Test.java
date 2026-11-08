import java.lang.management.ManagementFactory;
import java.lang.management.MemoryMXBean;
import java.lang.management.MemoryUsage;

/** Exercises the sun.management memory MXBean backed by the MemoryImpl natives. */
public class Test {

    public static void main(String[] args) {
        MemoryMXBean bean = ManagementFactory.getMemoryMXBean();
        System.out.println("bean present: " + (bean != null));

        MemoryUsage heap = bean.getHeapMemoryUsage();
        MemoryUsage nonHeap = bean.getNonHeapMemoryUsage();
        System.out.println("heap present: " + (heap != null));
        System.out.println("non-heap present: " + (nonHeap != null));

        System.out.println("heap used non-negative: " + (heap.getUsed() >= 0));
        System.out.println("heap committed covers used: " + (heap.getCommitted() >= heap.getUsed()));

        System.out.println("non-heap used non-negative: " + (nonHeap.getUsed() >= 0));
        System.out.println("non-heap committed covers used: " + (nonHeap.getCommitted() >= nonHeap.getUsed()));
    }
}
