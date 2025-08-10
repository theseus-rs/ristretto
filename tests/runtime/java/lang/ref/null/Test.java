import java.lang.ref.*;

/**
 * Tests for null reference handling.
 */
public class Test {

    public static void main(String[] args) {
        System.out.println("=== Null References Tests ===");
        testNullReferenceCreation();
        testNullReferenceOperations();
        testNullReferenceWithQueue();
        System.out.println("=== Null References Tests Completed ===");
    }

    /**
     * Test creating references with null referents
     */
    public static void testNullReferenceCreation() {
        System.out.println("\n--- Testing Null Reference Creation ---");

        WeakReference<Object> weakNull = new WeakReference<>(null);
        SoftReference<Object> softNull = new SoftReference<>(null);

        ReferenceQueue<Object> queue = new ReferenceQueue<>();
        PhantomReference<Object> phantomNull = new PhantomReference<>(null, queue);
        WeakReference<Object> weakNullQueue = new WeakReference<>(null, queue);
        SoftReference<Object> softNullQueue = new SoftReference<>(null, queue);

        System.out.println("Null references created successfully");
        System.out.println("weakNull.get() == null: " + (weakNull.get() == null));
        System.out.println("softNull.get() == null: " + (softNull.get() == null));
        System.out.println("phantomNull.get() == null: " + (phantomNull.get() == null));
        System.out.println("weakNullQueue.get() == null: " + (weakNullQueue.get() == null));
        System.out.println("softNullQueue.get() == null: " + (softNullQueue.get() == null));

        // All should return the same null value
        System.out.println("All null references return same null: " +
            (weakNull.get() == softNull.get() &&
             softNull.get() == phantomNull.get() &&
             phantomNull.get() == weakNullQueue.get()));
    }

    /**
     * Test operations on null references
     */
    public static void testNullReferenceOperations() {
        System.out.println("\n--- Testing Null Reference Operations ---");

        WeakReference<Object> weakNull = new WeakReference<>(null);
        SoftReference<Object> softNull = new SoftReference<>(null);
        ReferenceQueue<Object> queue = new ReferenceQueue<>();
        PhantomReference<Object> phantomNull = new PhantomReference<>(null, queue);

        System.out.println("Testing clear() operations on null references");

        // Test clear operations - should not throw
        weakNull.clear();
        softNull.clear();
        phantomNull.clear();

        System.out.println("clear() operations completed successfully");
        System.out.println("weakNull.get() still null: " + (weakNull.get() == null));
        System.out.println("softNull.get() still null: " + (softNull.get() == null));
        System.out.println("phantomNull.get() still null: " + (phantomNull.get() == null));

        // Test isEnqueued on null references
        System.out.println("weakNull.isEnqueued(): " + weakNull.isEnqueued());
        System.out.println("softNull.isEnqueued(): " + softNull.isEnqueued());
        System.out.println("phantomNull.isEnqueued(): " + phantomNull.isEnqueued());

        // Multiple clears should not cause issues
        weakNull.clear();
        softNull.clear();
        phantomNull.clear();
        System.out.println("Multiple clear() operations succeeded");
    }

    /**
     * Test null references with ReferenceQueue
     */
    public static void testNullReferenceWithQueue() {
        System.out.println("\n--- Testing Null References with Queue ---");

        ReferenceQueue<Object> queue = new ReferenceQueue<>();

        WeakReference<Object> weakNullQueue = new WeakReference<>(null, queue);
        SoftReference<Object> softNullQueue = new SoftReference<>(null, queue);
        PhantomReference<Object> phantomNullQueue = new PhantomReference<>(null, queue);

        System.out.println("Null references with queue created");
        System.out.println("Queue initially empty: " + (queue.poll() == null));

        // Test enqueue operations on null references
        boolean weakEnqueued = weakNullQueue.enqueue();
        boolean softEnqueued = softNullQueue.enqueue();
        boolean phantomEnqueued = phantomNullQueue.enqueue();

        System.out.println("weakNullQueue.enqueue(): " + weakEnqueued);
        System.out.println("softNullQueue.enqueue(): " + softEnqueued);
        System.out.println("phantomNullQueue.enqueue(): " + phantomEnqueued);

        // Check isEnqueued status
        System.out.println("weakNullQueue.isEnqueued(): " + weakNullQueue.isEnqueued());
        System.out.println("softNullQueue.isEnqueued(): " + softNullQueue.isEnqueued());
        System.out.println("phantomNullQueue.isEnqueued(): " + phantomNullQueue.isEnqueued());

        // Count enqueued references
        int enqueuedCount = 0;
        Reference<?> polled;
        while ((polled = queue.poll()) != null) {
            enqueuedCount++;
            System.out.println("Polled null reference " + enqueuedCount + ": " + (polled != null));

            // Verify it's one of our null references
            if (polled == weakNullQueue) {
                System.out.println("  - WeakReference with null referent");
            } else if (polled == softNullQueue) {
                System.out.println("  - SoftReference with null referent");
            } else if (polled == phantomNullQueue) {
                System.out.println("  - PhantomReference with null referent");
            }
        }
        System.out.println("Total null references enqueued: " + enqueuedCount);

        // Test second enqueue attempts
        boolean secondWeakEnqueue = weakNullQueue.enqueue();
        boolean secondSoftEnqueue = softNullQueue.enqueue();
        boolean secondPhantomEnqueue = phantomNullQueue.enqueue();

        System.out.println("Second enqueue attempts:");
        System.out.println("weakNullQueue second enqueue: " + secondWeakEnqueue);
        System.out.println("softNullQueue second enqueue: " + secondSoftEnqueue);
        System.out.println("phantomNullQueue second enqueue: " + secondPhantomEnqueue);
    }
}
