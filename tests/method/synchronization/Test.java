/** Test synchronized methods and thread safety. */
public class Test {
    private static int staticCounter = 0;
    private int instanceCounter = 0;
    private final Object lock = new Object();

    // Synchronized static method
    public static synchronized void incrementStaticCounter() {
        staticCounter++;
        System.out.println("Static counter: " + staticCounter + " (Thread: " + Thread.currentThread().getName() + ")");
        try {
            Thread.sleep(10); // Simulate some work
        } catch (InterruptedException e) {
            Thread.currentThread().interrupt();
        }
    }

    // Synchronized instance method
    public synchronized void incrementInstanceCounter() {
        instanceCounter++;
        System.out.println("Instance counter: " + instanceCounter + " (Thread: " + Thread.currentThread().getName() + ")");
        try {
            Thread.sleep(10); // Simulate some work
        } catch (InterruptedException e) {
            Thread.currentThread().interrupt();
        }
    }

    // Method with synchronized block
    public void incrementWithSyncBlock() {
        synchronized (lock) {
            instanceCounter++;
            System.out.println("Sync block counter: " + instanceCounter + " (Thread: " + Thread.currentThread().getName() + ")");
            try {
                Thread.sleep(10); // Simulate some work
            } catch (InterruptedException e) {
                Thread.currentThread().interrupt();
            }
        }
    }

    // Method demonstrating different synchronization behaviors
    public void demonstrateSynchronization() throws InterruptedException {
        Test instance1 = new Test();
        Test instance2 = new Test();

        // Test static synchronization - all threads synchronize on the class
        Thread[] staticThreads = new Thread[3];
        for (int i = 0; i < 3; i++) {
            staticThreads[i] = new Thread(() -> {
                for (int j = 0; j < 3; j++) {
                    incrementStaticCounter();
                }
            }, "StaticThread-" + i);
            staticThreads[i].start();
        }

        // Test instance synchronization - threads synchronize per instance
        Thread[] instanceThreads = new Thread[4];
        for (int i = 0; i < 2; i++) {
            final Test instance = (i == 0) ? instance1 : instance2;
            instanceThreads[i] = new Thread(() -> {
                for (int j = 0; j < 3; j++) {
                    instance.incrementInstanceCounter();
                }
            }, "InstanceThread-" + i);
            instanceThreads[i].start();
        }

        // Test synchronized block
        for (int i = 2; i < 4; i++) {
            instanceThreads[i] = new Thread(() -> {
                for (int j = 0; j < 3; j++) {
                    instance1.incrementWithSyncBlock();
                }
            }, "SyncBlockThread-" + (i-2));
            instanceThreads[i].start();
        }

        // Wait for all threads to complete
        for (Thread t : staticThreads) t.join();
        for (Thread t : instanceThreads) t.join();

        System.out.println("Final static counter: " + staticCounter);
        System.out.println("Final instance1 counter: " + instance1.instanceCounter);
        System.out.println("Final instance2 counter: " + instance2.instanceCounter);
    }

    public static void main(String[] args) throws InterruptedException {
        Test test = new Test();
        test.demonstrateSynchronization();
    }
}

