/** Test synchronized methods and thread safety. */
public class Test {
    private static int staticCounter = 0;
    private int instanceCounter = 0;
    private final Object lock = new Object();

    // Synchronized static method
    public static synchronized void incrementStaticCounter() {
        staticCounter++;
    }

    // Synchronized instance method
    public synchronized void incrementInstanceCounter() {
        instanceCounter++;
    }

    // Method with synchronized block
    public void incrementWithSyncBlock() {
        synchronized (lock) {
            instanceCounter++;
        }
    }

    public static void main(String[] args) throws InterruptedException {
        Test instance = new Test();

        // Test static synchronized method (single thread for determinism)
        System.out.println("Testing static synchronized method:");
        for (int i = 0; i < 5; i++) {
            incrementStaticCounter();
        }
        System.out.println("Static counter after 5 increments: " + staticCounter);

        // Test instance synchronized method (single thread for determinism)
        System.out.println("\nTesting instance synchronized method:");
        for (int i = 0; i < 5; i++) {
            instance.incrementInstanceCounter();
        }
        System.out.println("Instance counter after 5 increments: " + instance.instanceCounter);

        // Test synchronized block (single thread for determinism)
        System.out.println("\nTesting synchronized block:");
        for (int i = 0; i < 5; i++) {
            instance.incrementWithSyncBlock();
        }
        System.out.println("Instance counter after 5 more increments: " + instance.instanceCounter);

        // Test with two threads that run sequentially (join before next)
        System.out.println("\nTesting with sequential threads:");
        Thread t1 = new Thread(() -> {
            for (int i = 0; i < 3; i++) {
                incrementStaticCounter();
            }
        }, "Thread-1");

        Thread t2 = new Thread(() -> {
            for (int i = 0; i < 3; i++) {
                incrementStaticCounter();
            }
        }, "Thread-2");

        t1.start();
        t1.join(); // Wait for t1 to finish before starting t2
        t2.start();
        t2.join();

        System.out.println("Static counter after two sequential threads: " + staticCounter);

        System.out.println("\nSynchronization tests completed");
    }
}

