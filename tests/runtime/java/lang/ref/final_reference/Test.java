import java.lang.ref.*;

/**
 * Tests for FinalReference functionality.
 * Note: FinalReference is an internal class used by the JVM for finalization.
 * These tests verify behavior related to final references and finalization queue.
 */
public class Test {

    private static volatile int finalReferenceCount = 0;
    private static volatile boolean finalReferenceProcessed = false;
    private static volatile String lastProcessedObject = "";

    public static void main(String[] args) {
        System.out.println("=== FinalReference Tests ===");
        testFinalReferenceCreation();
        testFinalReferenceQueue();
        testFinalReferenceProcessing();
        testFinalReferenceWithFinalization();
        testFinalReferenceEdgeCases();
        System.out.println("=== FinalReference Tests Completed ===");
    }

    /**
     * Test FinalReference creation and basic behavior
     */
    public static void testFinalReferenceCreation() {
        System.out.println("\n--- Testing FinalReference Creation ---");

        try {
            // Since FinalReference is package-private, we test through finalization
            finalReferenceCount = 0;

            FinalizableTestObject obj = new FinalizableTestObject("FinalRefTest");

            System.out.println("Object with finalizer created: " + (obj != null));
            System.out.println("Object name: " + obj.getName());
            System.out.println("Initial final reference count: " + finalReferenceCount);

            // The JVM internally creates FinalReference for objects with finalizers
            obj = null; // Remove strong reference

            System.gc();
            System.runFinalization();

            try {
                Thread.sleep(100);
            } catch (InterruptedException e) {
                // Ignore
            }

            System.out.println("After GC and finalization:");
            System.out.println("Final reference processed: " + finalReferenceProcessed);
            System.out.println("Final reference count: " + finalReferenceCount);

        } catch (Exception e) {
            System.out.println("Error in final reference creation test: " + e.getMessage());
        }
    }

    /**
     * Test FinalReference queue behavior
     */
    public static void testFinalReferenceQueue() {
        System.out.println("\n--- Testing FinalReference Queue ---");

        finalReferenceCount = 0;
        finalReferenceProcessed = false;

        // Create multiple objects with finalizers
        FinalizableTestObject obj1 = new FinalizableTestObject("QueueTest1");
        FinalizableTestObject obj2 = new FinalizableTestObject("QueueTest2");
        FinalizableTestObject obj3 = new FinalizableTestObject("QueueTest3");

        System.out.println("Multiple finalizable objects created");
        System.out.println("obj1: " + obj1.getName());
        System.out.println("obj2: " + obj2.getName());
        System.out.println("obj3: " + obj3.getName());

        // Clear references
        obj1 = null;
        obj2 = null;
        obj3 = null;

        System.out.println("Strong references cleared");
        System.out.println("Pre-GC final reference count: " + finalReferenceCount);

        // Force GC multiple times to ensure finalization
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
        System.out.println("Final reference count: " + finalReferenceCount);
        System.out.println("Any final references processed: " + finalReferenceProcessed);
    }

    /**
     * Test FinalReference processing with different object types
     */
    public static void testFinalReferenceProcessing() {
        System.out.println("\n--- Testing FinalReference Processing ---");

        finalReferenceCount = 0;
        lastProcessedObject = "";

        // Test with different types of finalizable objects
        createFinalizableObject("SimpleObject");
        createFinalizableObjectWithResources("ResourceObject");
        createFinalizableObjectWithException("ExceptionObject");

        System.out.println("Different types of finalizable objects created");
        System.out.println("Initial state - count: " + finalReferenceCount);

        // Process finalization
        System.gc();
        System.runFinalization();

        try {
            Thread.sleep(150);
        } catch (InterruptedException e) {
            // Ignore
        }

        System.out.println("After finalization processing:");
        System.out.println("Total processed: " + finalReferenceCount);
        System.out.println("Last processed object: " + lastProcessedObject);
        System.out.println("Processing completed: " + finalReferenceProcessed);
    }

    /**
     * Test FinalReference interaction with standard finalization
     */
    public static void testFinalReferenceWithFinalization() {
        System.out.println("\n--- Testing FinalReference with Finalization ---");

        finalReferenceCount = 0;

        // Create object that tracks finalization steps
        TrackingFinalizableObject tracker = new TrackingFinalizableObject("Tracker");

        System.out.println("Tracking finalizable object created");
        System.out.println("Tracker name: " + tracker.getName());
        System.out.println("Initial finalization step: " + tracker.getFinalizationStep());

        // Clear reference
        tracker = null;

        // Force finalization
        System.gc();
        System.runFinalization();

        try {
            Thread.sleep(100);
        } catch (InterruptedException e) {
            // Ignore
        }

        System.out.println("After finalization:");
        System.out.println("Final reference count: " + finalReferenceCount);
        System.out.println("Finalization tracking completed");

        // Check if any tracking data was preserved
        System.out.println("Last processed: " + lastProcessedObject);
    }

    /**
     * Test edge cases and error conditions
     */
    public static void testFinalReferenceEdgeCases() {
        System.out.println("\n--- Testing FinalReference Edge Cases ---");

        finalReferenceCount = 0;

        // Test with object that throws in finalizer
        try {
            ExceptionFinalizableObject excObj = new ExceptionFinalizableObject("ExceptionTest");
            System.out.println("Exception finalizable object created");

            excObj = null;
            System.gc();
            System.runFinalization();

            Thread.sleep(100);

            System.out.println("Exception in finalizer handled");
            System.out.println("Final reference count after exception: " + finalReferenceCount);

        } catch (Exception e) {
            System.out.println("Unexpected exception: " + e.getMessage());
        }

        // Test with object that resurrects itself
        ResurrectionTestObject.resurrected = null;
        ResurrectionTestObject resurrection = new ResurrectionTestObject("Resurrection");

        System.out.println("Resurrection object created");
        resurrection = null;

        System.gc();
        System.runFinalization();

        try {
            Thread.sleep(100);
        } catch (InterruptedException e) {
            // Ignore
        }

        System.out.println("After resurrection test:");
        System.out.println("Object resurrected: " + (ResurrectionTestObject.resurrected != null));
        System.out.println("Final reference count: " + finalReferenceCount);

        // Clear resurrection and finalize again
        ResurrectionTestObject.resurrected = null;
        System.gc();
        System.runFinalization();

        try {
            Thread.sleep(100);
        } catch (InterruptedException e) {
            // Ignore
        }

        System.out.println("After final cleanup:");
        System.out.println("Final reference count: " + finalReferenceCount);
    }

    // Helper methods
    private static void createFinalizableObject(String name) {
        new FinalizableTestObject(name);
    }

    private static void createFinalizableObjectWithResources(String name) {
        new ResourceFinalizableObject(name);
    }

    private static void createFinalizableObjectWithException(String name) {
        new ExceptionFinalizableObject(name);
    }

    /**
     * Basic finalizable test object
     */
    static class FinalizableTestObject {
        private String name;

        public FinalizableTestObject(String name) {
            this.name = name;
        }

        public String getName() {
            return name;
        }

        @Override
        protected void finalize() throws Throwable {
            try {
                finalReferenceCount++;
                finalReferenceProcessed = true;
                lastProcessedObject = name;
                System.out.println("Finalizing: " + name + " (count: " + finalReferenceCount + ")");
            } finally {
                super.finalize();
            }
        }
    }

    /**
     * Finalizable object with resource cleanup
     */
    static class ResourceFinalizableObject {
        private String name;
        private byte[] resource;

        public ResourceFinalizableObject(String name) {
            this.name = name;
            this.resource = new byte[1024]; // Simulate resource
        }

        @Override
        protected void finalize() throws Throwable {
            try {
                finalReferenceCount++;
                finalReferenceProcessed = true;
                lastProcessedObject = name + "_resource";
                System.out.println("Finalizing resource object: " + name);
                resource = null; // Clean up resource
            } finally {
                super.finalize();
            }
        }
    }

    /**
     * Finalizable object that throws exception
     */
    static class ExceptionFinalizableObject {
        private String name;

        public ExceptionFinalizableObject(String name) {
            this.name = name;
        }

        @Override
        protected void finalize() throws Throwable {
            try {
                finalReferenceCount++;
                finalReferenceProcessed = true;
                lastProcessedObject = name + "_exception";
                System.out.println("Finalizing with exception: " + name);
                throw new RuntimeException("Finalization exception for " + name);
            } finally {
                super.finalize();
            }
        }
    }

    /**
     * Tracking finalizable object
     */
    static class TrackingFinalizableObject {
        private String name;
        private int finalizationStep = 0;

        public TrackingFinalizableObject(String name) {
            this.name = name;
        }

        public String getName() {
            return name;
        }

        public int getFinalizationStep() {
            return finalizationStep;
        }

        @Override
        protected void finalize() throws Throwable {
            try {
                finalizationStep = 1;
                finalReferenceCount++;
                finalReferenceProcessed = true;
                lastProcessedObject = name + "_tracked";
                System.out.println("Tracking finalization: " + name);
                finalizationStep = 2;
            } finally {
                super.finalize();
            }
        }
    }

    /**
     * Object that can resurrect itself
     */
    static class ResurrectionTestObject {
        public static volatile ResurrectionTestObject resurrected;
        private String name;
        private boolean hasBeenFinalized = false;

        public ResurrectionTestObject(String name) {
            this.name = name;
        }

        @Override
        protected void finalize() throws Throwable {
            try {
                finalReferenceCount++;
                if (!hasBeenFinalized) {
                    hasBeenFinalized = true;
                    resurrected = this; // Resurrect
                    System.out.println("Resurrecting: " + name);
                } else {
                    System.out.println("Final finalization: " + name);
                    finalReferenceProcessed = true;
                    lastProcessedObject = name + "_final";
                }
            } finally {
                super.finalize();
            }
        }
    }
}
