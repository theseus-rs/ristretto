import java.lang.ref.*;

/**
 * Tests for reference chaining and complex scenarios.
 */
public class Test {

    public static void main(String[] args) {
        System.out.println("=== Reference Chaining Tests ===");
        testReferenceChaining();
        testComplexReferenceScenarios();
        testReferenceLifecycle();
        System.out.println("=== Reference Chaining Tests Completed ===");
    }

    /**
     * Test reference chaining and complex scenarios
     */
    public static void testReferenceChaining() {
        System.out.println("\n--- Testing Reference Chaining ---");

        // Create a chain of objects
        Object obj1 = new Object();
        Object obj2 = new Object();
        Object obj3 = new Object();

        ReferenceQueue<Object> queue = new ReferenceQueue<>();

        WeakReference<Object> ref1 = new WeakReference<>(obj1, queue);
        WeakReference<Object> ref2 = new WeakReference<>(obj2, queue);
        WeakReference<Object> ref3 = new WeakReference<>(obj3, queue);

        System.out.println("Reference chain created");
        System.out.println("ref1 valid: " + (ref1.get() != null));
        System.out.println("ref2 valid: " + (ref2.get() != null));
        System.out.println("ref3 valid: " + (ref3.get() != null));

        // Test clearing in sequence
        ref2.clear();
        System.out.println("After clearing ref2:");
        System.out.println("ref1 still valid: " + (ref1.get() != null));
        System.out.println("ref2 now null: " + (ref2.get() == null));
        System.out.println("ref3 still valid: " + (ref3.get() != null));

        // Test enqueue operations
        ref1.enqueue();
        ref3.enqueue();

        System.out.println("After enqueuing ref1 and ref3:");
        System.out.println("ref1 enqueued: " + ref1.isEnqueued());
        System.out.println("ref2 not enqueued: " + !ref2.isEnqueued());
        System.out.println("ref3 enqueued: " + ref3.isEnqueued());

        // Poll from queue
        int pollCount = 0;
        Reference<?> polled;
        while ((polled = queue.poll()) != null) {
            pollCount++;
            System.out.println("Polled reference " + pollCount + ": " + (polled != null));
        }
        System.out.println("Total references polled: " + pollCount);
    }

    /**
     * Test complex reference scenarios
     */
    public static void testComplexReferenceScenarios() {
        System.out.println("\n--- Testing Complex Reference Scenarios ---");

        Object sharedObj = new Object();
        ReferenceQueue<Object> queue1 = new ReferenceQueue<>();
        ReferenceQueue<Object> queue2 = new ReferenceQueue<>();

        // Create a complex web of references
        WeakReference<Object> weak1 = new WeakReference<>(sharedObj, queue1);
        WeakReference<Object> weak2 = new WeakReference<>(sharedObj, queue2);
        SoftReference<Object> soft1 = new SoftReference<>(sharedObj, queue1);
        PhantomReference<Object> phantom1 = new PhantomReference<>(sharedObj, queue2);

        // Additional objects with their own references
        Object obj2 = new Object();
        Object obj3 = new Object();
        WeakReference<Object> weak3 = new WeakReference<>(obj2, queue1);
        SoftReference<Object> soft2 = new SoftReference<>(obj3, queue2);

        System.out.println("Complex reference web created");
        System.out.println("References to shared object: weak1, weak2, soft1, phantom1");
        System.out.println("Additional isolated references: weak3, soft2");

        // Verify shared object references
        System.out.println("weak1 == weak2 referent: " + (weak1.get() == weak2.get()));
        System.out.println("weak1 == soft1 referent: " + (weak1.get() == soft1.get()));
        System.out.println("phantom1 referent is null: " + (phantom1.get() == null));

        // Test selective clearing
        weak1.clear();
        phantom1.clear();

        System.out.println("After clearing weak1 and phantom1:");
        System.out.println("weak1 null: " + (weak1.get() == null));
        System.out.println("weak2 still valid: " + (weak2.get() == sharedObj));
        System.out.println("soft1 still valid: " + (soft1.get() == sharedObj));
        System.out.println("weak3 still valid: " + (weak3.get() == obj2));
        System.out.println("soft2 still valid: " + (soft2.get() == obj3));

        // Test selective enqueueing
        weak2.enqueue();
        soft1.enqueue();
        weak3.enqueue();

        // Check queue distributions
        int queue1Count = 0;
        while (queue1.poll() != null) queue1Count++;

        int queue2Count = 0;
        while (queue2.poll() != null) queue2Count++;

        System.out.println("References processed by queue1: " + queue1Count);
        System.out.println("References processed by queue2: " + queue2Count);
    }

    /**
     * Test complete reference lifecycle
     */
    public static void testReferenceLifecycle() {
        System.out.println("\n--- Testing Reference Lifecycle ---");

        ReferenceQueue<Object> queue = new ReferenceQueue<>();

        // Create reference
        Object obj = new Object();
        WeakReference<Object> ref = new WeakReference<>(obj, queue);

        System.out.println("Phase 1 - Creation:");
        System.out.println("Reference created: " + (ref != null));
        System.out.println("Referent accessible: " + (ref.get() == obj));
        System.out.println("Not enqueued: " + !ref.isEnqueued());

        // Phase 2 - Active use
        System.out.println("\nPhase 2 - Active Use:");
        Object retrieved = ref.get();
        System.out.println("Retrieved object equals original: " + (retrieved == obj));
        System.out.println("Reference still not enqueued: " + !ref.isEnqueued());

        // Phase 3 - Manual operations
        System.out.println("\nPhase 3 - Manual Operations:");
        boolean enqueued = ref.enqueue();
        System.out.println("Manual enqueue successful: " + enqueued);
        System.out.println("Now enqueued: " + ref.isEnqueued());
        System.out.println("Referent still accessible: " + (ref.get() == obj));

        // Phase 4 - Queue processing
        System.out.println("\nPhase 4 - Queue Processing:");
        Reference<?> polled = queue.poll();
        System.out.println("Polled reference equals original: " + (polled == ref));
        System.out.println("No longer enqueued: " + !ref.isEnqueued());
        System.out.println("Referent still accessible: " + (ref.get() == obj));

        // Phase 5 - Clearing
        System.out.println("\nPhase 5 - Clearing:");
        ref.clear();
        System.out.println("Reference cleared: " + (ref.get() == null));
        System.out.println("Original object still accessible: " + (obj != null));

        // Phase 6 - Post-clear operations
        System.out.println("\nPhase 6 - Post-Clear Operations:");
        boolean secondEnqueue = ref.enqueue();
        System.out.println("Enqueue after clear: " + secondEnqueue);

        ref.clear(); // Should not throw
        System.out.println("Double clear successful");

        Reference<?> secondPoll = queue.poll();
        System.out.println("Queue empty after lifecycle: " + (secondPoll == null));

        System.out.println("\nReference lifecycle completed successfully");
    }
}
