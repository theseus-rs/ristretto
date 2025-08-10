import java.lang.ref.*;

/**
 * Tests for WeakReference functionality.
 */
public class Test {

    public static void main(String[] args) {
        System.out.println("=== WeakReference Tests ===");
        testBasicWeakReference();
        testWeakReferenceWithQueue();
        testWeakReferenceClearing();
        System.out.println("=== WeakReference Tests Completed ===");
    }

    /**
     * Test basic WeakReference functionality
     */
    public static void testBasicWeakReference() {
        System.out.println("\n--- Testing Basic WeakReference ---");

        Object obj = new Object();
        WeakReference<Object> weakRef = new WeakReference<>(obj);

        System.out.println("WeakReference created: " + (weakRef != null));
        System.out.println("WeakReference.get() returns object: " + (weakRef.get() == obj));
        System.out.println("WeakReference.get() not null: " + (weakRef.get() != null));

        // Test with null
        WeakReference<Object> nullWeakRef = new WeakReference<>(null);
        System.out.println("WeakReference with null: " + (nullWeakRef.get() == null));

        // Test isEnqueued before adding to queue
        System.out.println("isEnqueued() without queue: " + weakRef.isEnqueued());
    }

    /**
     * Test WeakReference with ReferenceQueue
     */
    public static void testWeakReferenceWithQueue() {
        System.out.println("\n--- Testing WeakReference with Queue ---");

        Object obj = new Object();
        ReferenceQueue<Object> queue = new ReferenceQueue<>();
        WeakReference<Object> weakRef = new WeakReference<>(obj, queue);

        System.out.println("WeakReference with queue created: " + (weakRef != null));
        System.out.println("WeakReference.get() returns object: " + (weakRef.get() == obj));
        System.out.println("Initially not enqueued: " + !weakRef.isEnqueued());

        // Test manual enqueue
        boolean enqueued = weakRef.enqueue();
        System.out.println("Manual enqueue() successful: " + enqueued);
        System.out.println("isEnqueued() after manual enqueue: " + weakRef.isEnqueued());

        // Test poll from queue
        Reference<?> polledRef = queue.poll();
        System.out.println("poll() returns the reference: " + (polledRef == weakRef));
        System.out.println("isEnqueued() after poll: " + weakRef.isEnqueued());

        // Test second enqueue attempt
        boolean secondEnqueue = weakRef.enqueue();
        System.out.println("Second enqueue() attempt fails: " + !secondEnqueue);
    }

    /**
     * Test WeakReference clearing
     */
    public static void testWeakReferenceClearing() {
        System.out.println("\n--- Testing WeakReference Clearing ---");

        Object obj = new Object();
        WeakReference<Object> weakRef = new WeakReference<>(obj);

        System.out.println("Before clear - get() not null: " + (weakRef.get() != null));

        // Test clear
        weakRef.clear();
        System.out.println("After clear(), get() returns null: " + (weakRef.get() == null));

        // Test clearing already cleared reference
        weakRef.clear(); // Should not throw
        System.out.println("Clearing already cleared reference succeeds");

        // Test clearing null reference
        WeakReference<Object> nullRef = new WeakReference<>(null);
        nullRef.clear(); // Should not throw
        System.out.println("Clearing null reference succeeds");
    }
}
