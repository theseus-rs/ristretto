public class Test {
    public static void main(String[] args) {
        System.out.println("=== Daemon Thread Tests ===");

        testBasicDaemonThread();
        testNonDaemonThread();
        testDaemonThreadInheritance();
        testSettingDaemonAfterStart();

        System.out.println("Daemon thread tests completed");
    }

    private static void testBasicDaemonThread() {
        System.out.println("Test 1: Basic daemon thread");
        Thread daemonThread = new Thread(() -> {
            System.out.println("Daemon thread started");
        });

        daemonThread.setDaemon(true);
        System.out.println("Is daemon before start: " + daemonThread.isDaemon());
        daemonThread.start();

        // Wait for daemon thread to complete its output
        try {
            daemonThread.join(500);
        } catch (InterruptedException e) {
            System.out.println("Daemon thread join interrupted");
        }
    }

    private static void testNonDaemonThread() {
        System.out.println("Test 2: Non-daemon thread");
        Thread normalThread = new Thread(() -> {
            System.out.println("Normal thread running");
        });
        System.out.println("Is daemon for normal thread: " + normalThread.isDaemon());
        normalThread.start();

        try {
            normalThread.join();
        } catch (InterruptedException e) {
            System.out.println("Normal thread join interrupted");
        }
    }

    private static void testDaemonThreadInheritance() {
        System.out.println("Test 3: Daemon thread inheritance");
        Thread parentThread = new Thread(() -> {
            Thread childThread = new Thread(() -> {
                System.out.println("Child thread is daemon: " + Thread.currentThread().isDaemon());
            });
            childThread.start();
            try {
                childThread.join();
            } catch (InterruptedException e) {
                System.out.println("Child thread join interrupted");
            }
        });

        parentThread.setDaemon(true);
        parentThread.start();

        try {
            parentThread.join();
        } catch (InterruptedException e) {
            System.out.println("Parent thread join interrupted");
        }
    }

    private static void testSettingDaemonAfterStart() {
        System.out.println("Test 4: Setting daemon after start (should throw exception)");
        Thread testThread = new Thread(() -> {
            try {
                Thread.sleep(50);
            } catch (InterruptedException e) {
                System.out.println("Test thread interrupted");
            }
        });
        testThread.start();

        try {
            testThread.setDaemon(true);
            System.out.println("ERROR: Should not be able to set daemon after start");
        } catch (IllegalThreadStateException e) {
            System.out.println("Correctly caught exception when setting daemon after start: " + e.getClass().getSimpleName());
        }

        try {
            testThread.join();
        } catch (InterruptedException e) {
            System.out.println("Test thread join interrupted");
        }
    }
}
