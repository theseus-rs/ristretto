import java.lang.ref.*;

/**
 * Tests for multiple references to the same object.
 */
public class Test {

    public static void main(String[] args) {
        System.out.println("=== Multiple References Tests ===");
        testMultipleReferencesToSameObject();
        testMixedReferenceTypes();
        testMultipleQueues();
        System.out.println("=== Multiple References Tests Completed ===");
    }

    /**
     * Test multiple references to the same object
     */
    public static void testMultipleReferencesToSameObject() {
        System.out.println("\n--- Testing Multiple References to Same Object ---");

        Object obj = new Object();
        ReferenceQueue<Object> queue = new ReferenceQueue<>();

        WeakReference<Object> weak1 = new WeakReference<>(obj, queue);
        WeakReference<Object> weak2 = new WeakReference<>(obj, queue);
        WeakReference<Object> weak3 = new WeakReference<>(obj, queue);

        System.out.println("Multiple WeakReferences to same object created");
        System.out.println("weak1.get() == weak2.get(): " + (weak1.get() == weak2.get()));
        System.out.println("weak2.get() == weak3.get(): " + (weak2.get() == weak3.get()));
        System.out.println("All point to original object: " + (weak1.get() == obj));

        // Clear one reference
        weak1.clear();
        System.out.println("After clearing weak1:");
        System.out.println("weak1.get() is null: " + (weak1.get() == null));
        System.out.println("weak2.get() still valid: " + (weak2.get() == obj));
        System.out.println("weak3.get() still valid: " + (weak3.get() == obj));

        // Clear another reference
        weak2.clear();
        System.out.println("After clearing weak2:");
        System.out.println("weak1.get() is null: " + (weak1.get() == null));
        System.out.println("weak2.get() is null: " + (weak2.get() == null));
        System.out.println("weak3.get() still valid: " + (weak3.get() == obj));

        // Original object should still be reachable through weak3 and strong reference
        System.out.println("Original object still accessible: " + (obj != null));
    }

    /**
     * Test mixed reference types to same object
     */
    public static void testMixedReferenceTypes() {
        System.out.println("\n--- Testing Mixed Reference Types ---");

        Object obj = new Object();
        ReferenceQueue<Object> queue1 = new ReferenceQueue<>();
        ReferenceQueue<Object> queue2 = new ReferenceQueue<>();

        WeakReference<Object> weak1 = new WeakReference<>(obj, queue1);
        WeakReference<Object> weak2 = new WeakReference<>(obj, queue1);
        SoftReference<Object> soft1 = new SoftReference<>(obj, queue2);
        PhantomReference<Object> phantom1 = new PhantomReference<>(obj, queue2);

        System.out.println("Mixed reference types created");
        System.out.println("weak1.get() == weak2.get(): " + (weak1.get() == weak2.get()));
        System.out.println("weak1.get() == soft1.get(): " + (weak1.get() == soft1.get()));
        System.out.println("phantom1.get() is null: " + (phantom1.get() == null));

        // Test clearing different types
        weak1.clear();
        soft1.clear();

        System.out.println("After clearing weak1 and soft1:");
        System.out.println("weak1.get() is null: " + (weak1.get() == null));
        System.out.println("weak2.get() still valid: " + (weak2.get() == obj));
        System.out.println("soft1.get() is null: " + (soft1.get() == null));
        System.out.println("phantom1.get() still null: " + (phantom1.get() == null));

        // Test enqueuing
        boolean weak2Enqueued = weak2.enqueue();
        boolean phantomEnqueued = phantom1.enqueue();

        System.out.println("weak2 enqueue successful: " + weak2Enqueued);
        System.out.println("phantom1 enqueue successful: " + phantomEnqueued);

        // Check queues
        Reference<?> fromQueue1 = queue1.poll();
        Reference<?> fromQueue2 = queue2.poll();

        System.out.println("Reference polled from queue1: " + (fromQueue1 == weak2));
        System.out.println("Reference polled from queue2: " + (fromQueue2 == phantom1));
    }

    /**
     * Test multiple references with multiple queues
     */
    public static void testMultipleQueues() {
        System.out.println("\n--- Testing Multiple Queues ---");

        Object obj1 = new Object();
        Object obj2 = new Object();

        ReferenceQueue<Object> queue1 = new ReferenceQueue<>();
        ReferenceQueue<Object> queue2 = new ReferenceQueue<>();
        ReferenceQueue<Object> queue3 = new ReferenceQueue<>();

        WeakReference<Object> weak1 = new WeakReference<>(obj1, queue1);
        WeakReference<Object> weak2 = new WeakReference<>(obj1, queue2);
        WeakReference<Object> weak3 = new WeakReference<>(obj2, queue1);
        WeakReference<Object> weak4 = new WeakReference<>(obj2, queue3);

        System.out.println("References distributed across multiple queues");
        System.out.println("weak1 and weak2 reference same object: " + (weak1.get() == weak2.get()));
        System.out.println("weak3 and weak4 reference same object: " + (weak3.get() == weak4.get()));
        System.out.println("obj1 and obj2 are different: " + (weak1.get() != weak3.get()));

        // Enqueue all references
        weak1.enqueue();
        weak2.enqueue();
        weak3.enqueue();
        weak4.enqueue();

        System.out.println("All references enqueued");

        // Check each queue
        int queue1Count = 0;
        while (queue1.poll() != null) queue1Count++;

        int queue2Count = 0;
        while (queue2.poll() != null) queue2Count++;

        int queue3Count = 0;
        while (queue3.poll() != null) queue3Count++;

        System.out.println("References in queue1: " + queue1Count);
        System.out.println("References in queue2: " + queue2Count);
        System.out.println("References in queue3: " + queue3Count);
        System.out.println("Total references processed: " + (queue1Count + queue2Count + queue3Count));
    }
}
