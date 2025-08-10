import java.lang.ref.*;

/**
 * Tests for SoftReference functionality.
 */
public class Test {

    public static void main(String[] args) {
        System.out.println("=== SoftReference Tests ===");
        testBasicSoftReference();
        testSoftReferenceWithQueue();
        testSoftReferenceClearing();
        System.out.println("=== SoftReference Tests Completed ===");
    }

    /**
     * Test basic SoftReference functionality
     */
    public static void testBasicSoftReference() {
        System.out.println("\n--- Testing Basic SoftReference ---");

        Object obj = new Object();
        SoftReference<Object> softRef = new SoftReference<>(obj);

        System.out.println("SoftReference created: " + (softRef != null));
        System.out.println("SoftReference.get() returns object: " + (softRef.get() == obj));
        System.out.println("SoftReference.get() not null: " + (softRef.get() != null));

        // Test with null
        SoftReference<Object> nullSoftRef = new SoftReference<>(null);
        System.out.println("SoftReference with null: " + (nullSoftRef.get() == null));

        // Test isEnqueued before adding to queue
        System.out.println("isEnqueued() without queue: " + softRef.isEnqueued());
    }

    /**
     * Test SoftReference with ReferenceQueue
     */
    public static void testSoftReferenceWithQueue() {
        System.out.println("\n--- Testing SoftReference with Queue ---");

        Object obj = new Object();
        ReferenceQueue<Object> queue = new ReferenceQueue<>();
        SoftReference<Object> softRef = new SoftReference<>(obj, queue);

        System.out.println("SoftReference with queue created: " + (softRef != null));
        System.out.println("SoftReference.get() returns object: " + (softRef.get() == obj));
        System.out.println("Initially not enqueued: " + !softRef.isEnqueued());

        // Test manual enqueue
        boolean enqueued = softRef.enqueue();
        System.out.println("Manual enqueue() successful: " + enqueued);
        System.out.println("isEnqueued() after manual enqueue: " + softRef.isEnqueued());

        // Test poll from queue
        Reference<?> polledRef = queue.poll();
        System.out.println("poll() returns the reference: " + (polledRef == softRef));
        System.out.println("isEnqueued() after poll: " + softRef.isEnqueued());

        // Test second enqueue attempt
        boolean secondEnqueue = softRef.enqueue();
        System.out.println("Second enqueue() attempt fails: " + !secondEnqueue);
    }

    /**
     * Test SoftReference clearing
     */
    public static void testSoftReferenceClearing() {
        System.out.println("\n--- Testing SoftReference Clearing ---");

        Object obj = new Object();
        SoftReference<Object> softRef = new SoftReference<>(obj);

        System.out.println("Before clear - get() not null: " + (softRef.get() != null));

        // Test clear
        softRef.clear();
        System.out.println("After clear(), get() returns null: " + (softRef.get() == null));

        // Test clearing already cleared reference
        softRef.clear(); // Should not throw
        System.out.println("Clearing already cleared reference succeeds");

        // Test clearing null reference
        SoftReference<Object> nullRef = new SoftReference<>(null);
        nullRef.clear(); // Should not throw
        System.out.println("Clearing null reference succeeds");
    }
}
