import java.lang.ref.*;

/**
 * Tests for Java Finalizer functionality.
 * Note: finalize() is deprecated since Java 9 and removed in newer versions.
 * These tests verify the traditional finalization behavior.
 */
public class Test {

    private static volatile int finalizeCount = 0;
    private static volatile boolean finalizeExecuted = false;
    private static volatile String lastFinalizedObject = "";

    public static void main(String[] args) {
        System.out.println("=== Finalizer Tests ===");
        testBasicFinalization();
        testFinalizationOrder();
        testFinalizationWithReferences();
        testFinalizationEdgeCases();
        testFinalizationTiming();
        System.out.println("=== Finalizer Tests Completed ===");
    }

    /**
     * Test basic finalization behavior
     */
    public static void testBasicFinalization() {
        System.out.println("\n--- Testing Basic Finalization ---");

        finalizeCount = 0;
        finalizeExecuted = false;

        // Create object with finalizer
        createFinalizableObject("BasicTest");

        System.out.println("Finalizable object created");
        System.out.println("Initial finalize count: " + finalizeCount);

        // Suggest garbage collection
        System.gc();
        System.runFinalization();

        // Give time for finalization
        try {
            Thread.sleep(100);
        } catch (InterruptedException e) {
            // Ignore
        }

        System.out.println("After GC and runFinalization:");
        System.out.println("Finalize executed: " + finalizeExecuted);
        System.out.println("Finalize count: " + finalizeCount);
        System.out.println("Last finalized object: " + lastFinalizedObject);
    }

    /**
     * Test finalization order with multiple objects
     */
    public static void testFinalizationOrder() {
        System.out.println("\n--- Testing Finalization Order ---");

        finalizeCount = 0;

        // Create multiple finalizable objects
        createFinalizableObject("Object1");
        createFinalizableObject("Object2");
        createFinalizableObject("Object3");

        System.out.println("Multiple finalizable objects created");
        System.out.println("Initial finalize count: " + finalizeCount);

        // Force garbage collection and finalization
        for (int i = 0; i < 3; i++) {
            System.gc();
            System.runFinalization();
            try {
                Thread.sleep(50);
            } catch (InterruptedException e) {
                // Ignore
            }
        }

        System.out.println("After multiple GC cycles:");
        System.out.println("Total finalize count: " + finalizeCount);
        System.out.println("Note: Finalization order is not guaranteed");
    }

    /**
     * Test finalization with weak references
     */
    public static void testFinalizationWithReferences() {
        System.out.println("\n--- Testing Finalization with References ---");

        finalizeCount = 0;
        ReferenceQueue<FinalizableObject> queue = new ReferenceQueue<>();

        FinalizableObject obj = new FinalizableObject("RefTest");
        WeakReference<FinalizableObject> weakRef = new WeakReference<>(obj, queue);

        System.out.println("Object with finalizer and weak reference created");
        System.out.println("WeakReference valid: " + (weakRef.get() != null));
        System.out.println("Initial finalize count: " + finalizeCount);

        // Clear strong reference
        obj = null;

        // Force GC and finalization
        System.gc();
        System.runFinalization();

        try {
            Thread.sleep(100);
        } catch (InterruptedException e) {
            // Ignore
        }

        System.out.println("After clearing strong reference:");
        System.out.println("WeakReference cleared: " + (weakRef.get() == null));
        System.out.println("Finalize count: " + finalizeCount);

        // Check if reference was enqueued
        Reference<?> polled = queue.poll();
        System.out.println("Reference enqueued: " + (polled != null));
    }

    /**
     * Test finalization edge cases
     */
    public static void testFinalizationEdgeCases() {
        System.out.println("\n--- Testing Finalization Edge Cases ---");

        finalizeCount = 0;

        // Test object resurrection in finalizer
        ResurrectableObject.resurrectCount = 0;
        ResurrectableObject.holder = null;

        createResurrectableObject("Resurrection");

        System.out.println("Resurrectable object created");
        System.gc();
        System.runFinalization();

        try {
            Thread.sleep(100);
        } catch (InterruptedException e) {
            // Ignore
        }

        System.out.println("After first GC:");
        System.out.println("Resurrect count: " + ResurrectableObject.resurrectCount);
        System.out.println("Object resurrected: " + (ResurrectableObject.holder != null));

        // Clear resurrection holder and try again
        ResurrectableObject.holder = null;
        System.gc();
        System.runFinalization();

        try {
            Thread.sleep(100);
        } catch (InterruptedException e) {
            // Ignore
        }

        System.out.println("After second GC:");
        System.out.println("Final resurrect count: " + ResurrectableObject.resurrectCount);
        System.out.println("Object finally collected: " + (ResurrectableObject.holder == null));
    }

    /**
     * Test finalization timing and behavior
     */
    public static void testFinalizationTiming() {
        System.out.println("\n--- Testing Finalization Timing ---");

        finalizeCount = 0;

        // Create objects in a loop to test finalization behavior
        for (int i = 0; i < 5; i++) {
            createFinalizableObject("Batch" + i);
        }

        System.out.println("Batch of 5 finalizable objects created");
        System.out.println("Pre-GC finalize count: " + finalizeCount);

        // Test immediate finalization call
        System.runFinalization();
        System.out.println("After runFinalization() only: " + finalizeCount);

        // Test with GC
        System.gc();
        System.out.println("After gc() only: " + finalizeCount);

        // Test with both
        System.gc();
        System.runFinalization();

        try {
            Thread.sleep(100);
        } catch (InterruptedException e) {
            // Ignore
        }

        System.out.println("After gc() + runFinalization(): " + finalizeCount);

        // Multiple rounds to catch any delayed finalization
        for (int i = 0; i < 3; i++) {
            System.gc();
            System.runFinalization();
            try {
                Thread.sleep(50);
            } catch (InterruptedException e) {
                // Ignore
            }
        }

        System.out.println("Final finalize count after multiple rounds: " + finalizeCount);
    }

    /**
     * Helper method to create finalizable objects
     */
    private static void createFinalizableObject(String name) {
        new FinalizableObject(name);
    }

    /**
     * Helper method to create resurrectable objects
     */
    private static void createResurrectableObject(String name) {
        new ResurrectableObject(name);
    }

    /**
     * Test class with finalizer
     */
    static class FinalizableObject {
        private String name;

        public FinalizableObject(String name) {
            this.name = name;
        }

        @Override
        protected void finalize() throws Throwable {
            try {
                finalizeCount++;
                finalizeExecuted = true;
                lastFinalizedObject = name;
                System.out.println("Finalizing object: " + name + " (count: " + finalizeCount + ")");
            } finally {
                super.finalize();
            }
        }

        public String getName() {
            return name;
        }
    }

    /**
     * Test class that can resurrect itself in finalizer
     */
    static class ResurrectableObject {
        public static volatile ResurrectableObject holder;
        public static volatile int resurrectCount = 0;

        private String name;

        public ResurrectableObject(String name) {
            this.name = name;
        }

        @Override
        protected void finalize() throws Throwable {
            try {
                resurrectCount++;
                if (resurrectCount == 1) {
                    // Resurrect on first finalization
                    holder = this;
                    System.out.println("Resurrecting object: " + name);
                } else {
                    System.out.println("Final finalization of: " + name);
                }
            } finally {
                super.finalize();
            }
        }

        public String getName() {
            return name;
        }
    }
}
