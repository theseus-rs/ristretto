public class Test {
    private static final int THREAD_COUNT = 10;
    private static final int INCREMENTS_PER_THREAD = 1000;
    private static int sharedCounter = 0;
    private static final Object lock = new Object();

    public static void main(String[] args) throws InterruptedException {
        System.out.println("=== Contention Test ===");

        Thread[] threads = new Thread[THREAD_COUNT];

        for (int i = 0; i < THREAD_COUNT; i++) {
            threads[i] = new Thread(() -> {
                for (int j = 0; j < INCREMENTS_PER_THREAD; j++) {
                    synchronized (lock) {
                        sharedCounter++;
                    }
                    // Small yield to increase contention probability vs sequential execution
                    Thread.yield();
                }
            });
            threads[i].start();
        }

        for (Thread t : threads) {
            t.join();
        }

        int expected = THREAD_COUNT * INCREMENTS_PER_THREAD;
        System.out.println("Expected: " + expected);
        System.out.println("Actual:   " + sharedCounter);

        if (sharedCounter == expected) {
            System.out.println("Contention Test PASSED");
        } else {
            System.out.println("Contention Test FAILED");
        }
    }
}
