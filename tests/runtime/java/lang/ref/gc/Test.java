import java.lang.ref.*;

/**
 * Tests for garbage collection behavior with references.
 */
public class Test {

    public static void main(String[] args) {
        System.out.println("=== Garbage Collection Tests ===");
        testGarbageCollectionBehavior();
        testGCWithQueue();
        testMemoryPressure();
        System.out.println("=== Garbage Collection Tests Completed ===");
    }

    /**
     * Test garbage collection behavior (limited without forced GC)
     */
    public static void testGarbageCollectionBehavior() {
        System.out.println("\n--- Testing Garbage Collection Behavior ---");

        ReferenceQueue<Object> queue = new ReferenceQueue<>();

        // Create references in a limited scope
        WeakReference<Object> weakRef = createWeakReference(queue);
        SoftReference<Object> softRef = createSoftReference(queue);
        PhantomReference<Object> phantomRef = createPhantomReference(queue);

        System.out.println("References created with limited scope objects");
        System.out.println("WeakReference initially not null: " + (weakRef.get() != null));
        System.out.println("SoftReference initially not null: " + (softRef.get() != null));
        System.out.println("PhantomReference always null: " + (phantomRef.get() == null));

        // Suggest garbage collection (no guarantee it will run)
        System.gc();

        // Check if objects were collected (results may vary)
        System.out.println("After gc() suggestion - checking states...");
        System.out.println("WeakReference state: " + (weakRef.get() == null ? "cleared" : "still_referenced"));
        System.out.println("SoftReference state: " + (softRef.get() == null ? "cleared" : "still_referenced"));

        // Check queue for enqueued references
        int queueCount = 0;
        while (queue.poll() != null) {
            queueCount++;
        }
        System.out.println("References found in queue after GC: " + queueCount);
    }

    /**
     * Test GC behavior with queue notifications
     */
    public static void testGCWithQueue() {
        System.out.println("\n--- Testing GC with Queue Notifications ---");

        ReferenceQueue<Object> queue = new ReferenceQueue<>();

        // Create multiple references that should be eligible for GC
        WeakReference<Object> weak1 = new WeakReference<>(new Object(), queue);
        WeakReference<Object> weak2 = new WeakReference<>(new Object(), queue);
        SoftReference<Object> soft1 = new SoftReference<>(new Object(), queue);

        System.out.println("Multiple references created");
        System.out.println("Queue initially empty: " + (queue.poll() == null));

        // Force references to lose their strong references
        System.gc();

        // Give some time for potential GC processing
        try {
            Thread.sleep(10);
        } catch (InterruptedException e) {
            // Ignore
        }

        System.out.println("After GC and brief wait:");
        System.out.println("weak1 state: " + (weak1.get() == null ? "cleared" : "still_referenced"));
        System.out.println("weak2 state: " + (weak2.get() == null ? "cleared" : "still_referenced"));
        System.out.println("soft1 state: " + (soft1.get() == null ? "cleared" : "still_referenced"));

        // Check queue contents
        int enqueuedCount = 0;
        Reference<?> polled;
        while ((polled = queue.poll()) != null) {
            enqueuedCount++;
            System.out.println("Found enqueued reference " + enqueuedCount);
        }
        System.out.println("Total enqueued references: " + enqueuedCount);
    }

    /**
     * Test behavior under simulated memory pressure
     */
    public static void testMemoryPressure() {
        System.out.println("\n--- Testing Memory Pressure Simulation ---");

        ReferenceQueue<Object> queue = new ReferenceQueue<>();

        // Create references to objects of different sizes
        WeakReference<Object> smallRef = new WeakReference<>(new Object(), queue);
        WeakReference<byte[]> largeRef = new WeakReference<>(new byte[1000], queue);
        SoftReference<byte[]> softLargeRef = new SoftReference<>(new byte[1000], queue);

        System.out.println("References to different sized objects created");
        System.out.println("Small object reference valid: " + (smallRef.get() != null));
        System.out.println("Large object weak reference valid: " + (largeRef.get() != null));
        System.out.println("Large object soft reference valid: " + (softLargeRef.get() != null));

        // Multiple GC suggestions
        for (int i = 0; i < 3; i++) {
            System.gc();
            try {
                Thread.sleep(5);
            } catch (InterruptedException e) {
                // Ignore
            }
        }

        System.out.println("After multiple GC suggestions:");
        System.out.println("Small object state: " + (smallRef.get() == null ? "cleared" : "still_referenced"));
        System.out.println("Large weak object state: " + (largeRef.get() == null ? "cleared" : "still_referenced"));
        System.out.println("Large soft object state: " + (softLargeRef.get() == null ? "cleared" : "still_referenced"));

        // Note: SoftReferences are typically cleared only under memory pressure
        System.out.println("Note: SoftReferences typically survive GC unless memory pressure exists");
    }

    private static WeakReference<Object> createWeakReference(ReferenceQueue<Object> queue) {
        Object obj = new Object(); // This object should be eligible for GC after method returns
        return new WeakReference<>(obj, queue);
    }

    private static SoftReference<Object> createSoftReference(ReferenceQueue<Object> queue) {
        Object obj = new Object(); // This object should be eligible for GC after method returns
        return new SoftReference<>(obj, queue);
    }

    private static PhantomReference<Object> createPhantomReference(ReferenceQueue<Object> queue) {
        Object obj = new Object(); // This object should be eligible for GC after method returns
        return new PhantomReference<>(obj, queue);
    }
}
