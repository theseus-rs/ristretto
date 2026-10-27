import java.lang.management.ManagementFactory;
import java.lang.management.RuntimeMXBean;

/** Exercises the sun.management runtime MXBean backed by the VMManagementImpl natives. */
public class Test {

    public static void main(String[] args) {
        RuntimeMXBean bean = ManagementFactory.getRuntimeMXBean();
        System.out.println("bean present: " + (bean != null));

        System.out.println("pid positive: " + (bean.getPid() > 0));
        System.out.println("start time positive: " + (bean.getStartTime() > 0));
        System.out.println("uptime non-negative: " + (bean.getUptime() >= 0));
        System.out.println("start time consistent with uptime: " + (Math.abs(System.currentTimeMillis() - bean.getStartTime() - bean.getUptime()) < 60000L));

        System.out.println("vm name matches property: " + bean.getVmName().equals(System.getProperty("java.vm.name")));
        System.out.println("vm version matches property: " + bean.getVmVersion().equals(System.getProperty("java.vm.version")));
    }
}
