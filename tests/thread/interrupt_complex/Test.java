public class Test {
    public static void main(String[] args) throws InterruptedException {
        System.out.println("=== Interrupt Complex Test ===");

        testInterruptDuringWait();

        System.out.println("Interrupt Complex Test PASSED");
    }

    private static void testInterruptDuringWait() throws InterruptedException {
        final Object lock = new Object();
        final boolean[] threadStarted = { false };

        Thread t = new Thread(() -> {
            synchronized (lock) {
                try {
                    threadStarted[0] = true;
                    lock.notify(); // signal main thread we are ready
                    System.out.println("Thread: Going to wait forever...");
                    lock.wait();
                    System.out.println("Thread: Woke up normally (unexpected)");
                } catch (InterruptedException e) {
                    System.out.println("Thread: Interrupted as expected");
                }
            }
        });

        synchronized (lock) {
            t.start();
            while (!threadStarted[0]) {
                lock.wait();
            }
        }

        // Wait a bit to ensure t is really in wait()
        Thread.sleep(100);

        System.out.println("Main: Interrupting thread...");
        t.interrupt();
        t.join();
        System.out.println("Main: Thread joined");
    }
}
