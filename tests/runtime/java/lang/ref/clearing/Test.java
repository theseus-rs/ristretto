import java.lang.ref.*;

/**
 * Tests for Reference clearing behavior.
 */
public class Test {

    public static void main(String[] args) {
        System.out.println("=== Reference Clearing Tests ===");
        testBasicClearing();
        testClearingWithQueue();
        testClearingEdgeCases();
        System.out.println("=== Reference Clearing Tests Completed ===");
    }

    /**
     * Test basic reference clearing behavior
     */
    public static void testBasicClearing() {
        System.out.println("\n--- Testing Basic Reference Clearing ---");

        Object obj = new Object();

        WeakReference<Object> weakRef = new WeakReference<>(obj);
        SoftReference<Object> softRef = new SoftReference<>(obj);

        System.out.println("Before clear - weak.get() not null: " + (weakRef.get() != null));
        System.out.println("Before clear - soft.get() not null: " + (softRef.get() != null));

        weakRef.clear();
        softRef.clear();

        System.out.println("After clear - weak.get() is null: " + (weakRef.get() == null));
        System.out.println("After clear - soft.get() is null: " + (softRef.get() == null));

        // Original object should still be reachable
        System.out.println("Original object still exists: " + (obj != null));
    }

    /**
     * Test clearing behavior with ReferenceQueue
     */
    public static void testClearingWithQueue() {
        System.out.println("\n--- Testing Clearing with ReferenceQueue ---");

        Object obj = new Object();
        ReferenceQueue<Object> queue = new ReferenceQueue<>();

        WeakReference<Object> weakRef = new WeakReference<>(obj, queue);
        SoftReference<Object> softRef = new SoftReference<>(obj, queue);
        PhantomReference<Object> phantomRef = new PhantomReference<>(obj, queue);

        System.out.println("References with queue created");
        System.out.println("Initially not enqueued - weak: " + !weakRef.isEnqueued());
        System.out.println("Initially not enqueued - soft: " + !softRef.isEnqueued());
        System.out.println("Initially not enqueued - phantom: " + !phantomRef.isEnqueued());

        // Clear references
        weakRef.clear();
        softRef.clear();
        phantomRef.clear();

        System.out.println("After clearing:");
        System.out.println("weak.get() is null: " + (weakRef.get() == null));
        System.out.println("soft.get() is null: " + (softRef.get() == null));
        System.out.println("phantom.get() is null: " + (phantomRef.get() == null));

        // Clearing should not automatically enqueue
        System.out.println("Clear does not auto-enqueue weak: " + !weakRef.isEnqueued());
        System.out.println("Clear does not auto-enqueue soft: " + !softRef.isEnqueued());
        System.out.println("Clear does not auto-enqueue phantom: " + !phantomRef.isEnqueued());

        // Queue should still be empty
        Reference<?> polled = queue.poll();
        System.out.println("Queue remains empty after clear: " + (polled == null));
    }

    /**
     * Test edge cases for clearing
     */
    public static void testClearingEdgeCases() {
        System.out.println("\n--- Testing Clearing Edge Cases ---");

        // Test clearing already cleared reference
        Object obj = new Object();
        WeakReference<Object> ref = new WeakReference<>(obj);

        ref.clear();
        System.out.println("First clear successful");

        ref.clear(); // Should not throw
        System.out.println("Second clear on same reference succeeds");
        System.out.println("Reference still null after double clear: " + (ref.get() == null));

        // Test clearing null reference
        WeakReference<Object> nullRef = new WeakReference<>(null);
        System.out.println("Null reference initially null: " + (nullRef.get() == null));

        nullRef.clear(); // Should not throw
        System.out.println("Clearing null reference succeeds");
        System.out.println("Null reference still null after clear: " + (nullRef.get() == null));

        // Test clearing after enqueue
        ReferenceQueue<Object> queue = new ReferenceQueue<>();
        WeakReference<Object> queueRef = new WeakReference<>(new Object(), queue);

        queueRef.enqueue();
        System.out.println("Reference enqueued: " + queueRef.isEnqueued());

        queueRef.clear();
        System.out.println("Clear after enqueue succeeds");
        System.out.println("Reference null after clear: " + (queueRef.get() == null));
        System.out.println("Reference still enqueued: " + queueRef.isEnqueued());
    }
}
