import java.nio.channels.AsynchronousChannelGroup;
import java.util.Locale;
import java.util.concurrent.ThreadFactory;
import java.util.concurrent.TimeUnit;
import java.util.concurrent.atomic.AtomicInteger;

/**
 * Exercises the lifecycle of the Windows IOCP-backed asynchronous channel group.
 */
public class Test {
    public static void main(String[] args) throws Exception {
        if (!System.getProperty("os.name", "").toLowerCase(Locale.ROOT).contains("windows")) {
            System.out.println("IOCP test skipped on non-Windows host");
            return;
        }

        AtomicInteger threadCount = new AtomicInteger();
        ThreadFactory factory = task -> {
            Thread thread = new Thread(task, "iocp-integration-" + threadCount.incrementAndGet());
            thread.setDaemon(true);
            return thread;
        };

        AsynchronousChannelGroup group =
                AsynchronousChannelGroup.withFixedThreadPool(2, factory);
        System.out.println("Group initially active: " + !group.isShutdown());

        group.shutdown();
        System.out.println("Shutdown requested: " + group.isShutdown());
        System.out.println("Terminated: " + group.awaitTermination(10, TimeUnit.SECONDS));
        System.out.println("Handler threads created: " + threadCount.get());

        // A second shutdown must not close the completion-port handle twice.
        group.shutdown();
        System.out.println("Second shutdown preserved termination: " + group.isTerminated());
    }
}
