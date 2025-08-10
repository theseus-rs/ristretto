import java.lang.ref.*;

/**
 * Tests for PhantomReference functionality.
 */
public class Test {

    public static void main(String[] args) {
        System.out.println("=== PhantomReference Tests ===");
        testBasicPhantomReference();
        testPhantomReferenceWithQueue();
        testPhantomReferenceClearing();
        System.out.println("=== PhantomReference Tests Completed ===");
    }

    /**
     * Test basic PhantomReference functionality
     */
    public static void testBasicPhantomReference() {
        System.out.println("\n--- Testing Basic PhantomReference ---");

        Object obj = new Object();
        ReferenceQueue<Object> queue = new ReferenceQueue<>();
        PhantomReference<Object> phantomRef = new PhantomReference<>(obj, queue);

        System.out.println("PhantomReference created: " + (phantomRef != null));
        // PhantomReference.get() should always return null
        System.out.println("PhantomReference.get() returns null: " + (phantomRef.get() == null));

        // Test with null
        PhantomReference<Object> nullPhantomRef = new PhantomReference<>(null, queue);
        System.out.println("PhantomReference with null: " + (nullPhantomRef.get() == null));

        // Test isEnqueued
        System.out.println("isEnqueued() initially: " + phantomRef.isEnqueued());
    }

    /**
     * Test PhantomReference with ReferenceQueue operations
     */
    public static void testPhantomReferenceWithQueue() {
        System.out.println("\n--- Testing PhantomReference with Queue ---");

        Object obj = new Object();
        ReferenceQueue<Object> queue = new ReferenceQueue<>();
        PhantomReference<Object> phantomRef = new PhantomReference<>(obj, queue);

        System.out.println("PhantomReference with queue created: " + (phantomRef != null));
        System.out.println("PhantomReference.get() always returns null: " + (phantomRef.get() == null));
        System.out.println("Initially not enqueued: " + !phantomRef.isEnqueued());

        // Test manual enqueue
        boolean enqueued = phantomRef.enqueue();
        System.out.println("Manual enqueue() successful: " + enqueued);
        System.out.println("isEnqueued() after manual enqueue: " + phantomRef.isEnqueued());

        // Test poll from queue
        Reference<?> polledRef = queue.poll();
        System.out.println("poll() returns the reference: " + (polledRef == phantomRef));
        System.out.println("isEnqueued() after poll: " + phantomRef.isEnqueued());

        // Test second enqueue attempt
        boolean secondEnqueue = phantomRef.enqueue();
        System.out.println("Second enqueue() attempt fails: " + !secondEnqueue);
    }

    /**
     * Test PhantomReference clearing
     */
    public static void testPhantomReferenceClearing() {
        System.out.println("\n--- Testing PhantomReference Clearing ---");

        Object obj = new Object();
        ReferenceQueue<Object> queue = new ReferenceQueue<>();
        PhantomReference<Object> phantomRef = new PhantomReference<>(obj, queue);

        System.out.println("Before clear - get() returns null: " + (phantomRef.get() == null));

        // Test clear
        phantomRef.clear();
        System.out.println("After clear(), get() still returns null: " + (phantomRef.get() == null));

        // Test clearing already cleared reference
        phantomRef.clear(); // Should not throw
        System.out.println("Clearing already cleared reference succeeds");

        // Test clearing null reference
        PhantomReference<Object> nullRef = new PhantomReference<>(null, queue);
        nullRef.clear(); // Should not throw
        System.out.println("Clearing null reference succeeds");
    }
}
