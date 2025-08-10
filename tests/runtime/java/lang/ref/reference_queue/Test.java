import java.lang.ref.*;

/**
 * Tests for ReferenceQueue functionality.
 */
public class Test {

    public static void main(String[] args) {
        System.out.println("=== ReferenceQueue Tests ===");
        testBasicReferenceQueue();
        testReferenceQueueOperations();
        testReferenceQueueWithMultipleReferences();
        System.out.println("=== ReferenceQueue Tests Completed ===");
    }

    /**
     * Test basic ReferenceQueue functionality
     */
    public static void testBasicReferenceQueue() {
        System.out.println("\n--- Testing Basic ReferenceQueue ---");

        ReferenceQueue<Object> queue = new ReferenceQueue<>();
        System.out.println("ReferenceQueue created: " + (queue != null));

        // Test poll on empty queue
        Reference<?> polled = queue.poll();
        System.out.println("poll() on empty queue returns null: " + (polled == null));

        // Test multiple polls on empty queue
        Reference<?> polled2 = queue.poll();
        System.out.println("Second poll() on empty queue returns null: " + (polled2 == null));
    }

    /**
     * Test ReferenceQueue operations with different reference types
     */
    public static void testReferenceQueueOperations() {
        System.out.println("\n--- Testing ReferenceQueue Operations ---");

        ReferenceQueue<Object> queue = new ReferenceQueue<>();
        Object obj = new Object();

        WeakReference<Object> weakRef = new WeakReference<>(obj, queue);
        SoftReference<Object> softRef = new SoftReference<>(obj, queue);
        PhantomReference<Object> phantomRef = new PhantomReference<>(obj, queue);

        System.out.println("References with queue created");
        System.out.println("WeakReference created: " + (weakRef != null));
        System.out.println("SoftReference created: " + (softRef != null));
        System.out.println("PhantomReference created: " + (phantomRef != null));

        // Test enqueue operations
        boolean weakEnqueued = weakRef.enqueue();
        boolean softEnqueued = softRef.enqueue();
        boolean phantomEnqueued = phantomRef.enqueue();

        System.out.println("WeakReference enqueue() successful: " + weakEnqueued);
        System.out.println("SoftReference enqueue() successful: " + softEnqueued);
        System.out.println("PhantomReference enqueue() successful: " + phantomEnqueued);

        // Test polling references
        int pollCount = 0;
        Reference<?> polledRef;
        while ((polledRef = queue.poll()) != null) {
            pollCount++;
            System.out.println("Polled reference " + pollCount + ": " + (polledRef != null));

            // Check which type of reference was polled
            if (polledRef instanceof WeakReference) {
                System.out.println("  Type: WeakReference");
            } else if (polledRef instanceof SoftReference) {
                System.out.println("  Type: SoftReference");
            } else if (polledRef instanceof PhantomReference) {
                System.out.println("  Type: PhantomReference");
            }
        }
        System.out.println("Total references polled: " + pollCount);
    }

    /**
     * Test ReferenceQueue with multiple references to same object
     */
    public static void testReferenceQueueWithMultipleReferences() {
        System.out.println("\n--- Testing ReferenceQueue with Multiple References ---");

        ReferenceQueue<Object> queue = new ReferenceQueue<>();
        Object obj = new Object();

        WeakReference<Object> weak1 = new WeakReference<>(obj, queue);
        WeakReference<Object> weak2 = new WeakReference<>(obj, queue);
        WeakReference<Object> weak3 = new WeakReference<>(obj, queue);

        System.out.println("Multiple WeakReferences to same object created");
        System.out.println("All references point to same object: " +
            (weak1.get() == weak2.get() && weak2.get() == weak3.get()));

        // Enqueue all references
        weak1.enqueue();
        weak2.enqueue();
        weak3.enqueue();

        System.out.println("All references enqueued");

        // Poll and count
        int count = 0;
        while (queue.poll() != null) {
            count++;
        }
        System.out.println("Number of references polled: " + count);

        // Verify all are no longer enqueued
        System.out.println("weak1 no longer enqueued: " + !weak1.isEnqueued());
        System.out.println("weak2 no longer enqueued: " + !weak2.isEnqueued());
        System.out.println("weak3 no longer enqueued: " + !weak3.isEnqueued());
    }
}
