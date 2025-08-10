import java.lang.ref.*;

/**
 * Tests for Java Cleaner functionality.
 * Note: Cleaner was introduced in Java 9 as a replacement for finalize().
 */
public class Test {

    private static volatile boolean cleanupExecuted = false;
    private static volatile int cleanupCount = 0;
    private static volatile String cleanupMessage = "";

    public static void main(String[] args) {
        System.out.println("=== Cleaner Tests ===");
        testBasicCleaner();
        testCleanerWithState();
        testCleanerManualCleanup();
        testMultipleCleaners();
        testCleanerEdgeCases();
        System.out.println("=== Cleaner Tests Completed ===");
    }

    /**
     * Test basic Cleaner functionality
     */
    public static void testBasicCleaner() {
        System.out.println("\n--- Testing Basic Cleaner ---");

        try {
            // Reset state
            cleanupExecuted = false;
            cleanupCount = 0;

            Cleaner cleaner = Cleaner.create();
            System.out.println("Cleaner created: " + (cleaner != null));

            // Create an object that will register for cleanup
            Object obj = new Object();
            Runnable cleanupAction = () -> {
                cleanupExecuted = true;
                cleanupCount++;
                System.out.println("Cleanup action executed");
            };

            Cleaner.Cleanable cleanable = cleaner.register(obj, cleanupAction);
            System.out.println("Object registered with cleaner: " + (cleanable != null));

            // Remove strong reference
            obj = null;

            // Suggest garbage collection
            System.gc();
            Thread.sleep(100); // Give time for cleanup

            System.out.println("Cleanup executed: " + cleanupExecuted);
            System.out.println("Cleanup count: " + cleanupCount);

        } catch (Exception e) {
            System.out.println("Cleaner not supported or error occurred: " + e.getClass().getSimpleName());
            System.out.println("This is expected on Java versions < 9");
        }
    }

    /**
     * Test Cleaner with state object
     */
    public static void testCleanerWithState() {
        System.out.println("\n--- Testing Cleaner with State ---");

        try {
            cleanupExecuted = false;
            cleanupMessage = "";

            Cleaner cleaner = Cleaner.create();

            // State object that holds cleanup information
            final String resourceName = "TestResource";
            final int resourceId = 12345;

            Runnable cleanupAction = () -> {
                cleanupExecuted = true;
                cleanupMessage = "Cleaned up " + resourceName + " with ID " + resourceId;
                System.out.println("State-based cleanup: " + cleanupMessage);
            };

            Object obj = new Object();
            Cleaner.Cleanable cleanable = cleaner.register(obj, cleanupAction);

            System.out.println("Object with state registered");

            // Clear reference
            obj = null;
            cleanable = null;

            // Force garbage collection
            System.gc();
            Thread.sleep(100);

            System.out.println("State cleanup executed: " + cleanupExecuted);
            System.out.println("Cleanup message: " + cleanupMessage);

        } catch (Exception e) {
            System.out.println("Cleaner not supported: " + e.getClass().getSimpleName());
        }
    }

    /**
     * Test manual cleanup with Cleaner
     */
    public static void testCleanerManualCleanup() {
        System.out.println("\n--- Testing Manual Cleaner Cleanup ---");

        try {
            cleanupExecuted = false;
            cleanupCount = 0;

            Cleaner cleaner = Cleaner.create();

            Runnable cleanupAction = () -> {
                cleanupExecuted = true;
                cleanupCount++;
                System.out.println("Manual cleanup action executed");
            };

            Object obj = new Object();
            Cleaner.Cleanable cleanable = cleaner.register(obj, cleanupAction);

            System.out.println("Object registered for manual cleanup");
            System.out.println("Initial cleanup state: " + cleanupExecuted);

            // Manually trigger cleanup
            cleanable.clean();

            System.out.println("After manual clean():");
            System.out.println("Cleanup executed: " + cleanupExecuted);
            System.out.println("Cleanup count: " + cleanupCount);

            // Second call to clean() should be safe but not execute again
            cleanable.clean();
            System.out.println("After second clean() call:");
            System.out.println("Cleanup count (should be same): " + cleanupCount);

            // Object can still be referenced
            System.out.println("Object still accessible: " + (obj != null));

        } catch (Exception e) {
            System.out.println("Cleaner not supported: " + e.getClass().getSimpleName());
        }
    }

    /**
     * Test multiple Cleaners and Cleanables
     */
    public static void testMultipleCleaners() {
        System.out.println("\n--- Testing Multiple Cleaners ---");

        try {
            cleanupCount = 0;

            Cleaner cleaner1 = Cleaner.create();
            Cleaner cleaner2 = Cleaner.create();

            System.out.println("Multiple cleaners created");

            // Register multiple objects with different cleaners
            Object obj1 = new Object();
            Object obj2 = new Object();
            Object obj3 = new Object();

            Cleaner.Cleanable cleanable1 = cleaner1.register(obj1, () -> {
                cleanupCount++;
                System.out.println("Cleaner1 cleanup executed (count: " + cleanupCount + ")");
            });

            Cleaner.Cleanable cleanable2 = cleaner1.register(obj2, () -> {
                cleanupCount++;
                System.out.println("Cleaner1 second cleanup executed (count: " + cleanupCount + ")");
            });

            Cleaner.Cleanable cleanable3 = cleaner2.register(obj3, () -> {
                cleanupCount++;
                System.out.println("Cleaner2 cleanup executed (count: " + cleanupCount + ")");
            });

            System.out.println("Multiple objects registered");
            System.out.println("Initial cleanup count: " + cleanupCount);

            // Manual cleanup of first object
            cleanable1.clean();
            System.out.println("After manual cleanup of obj1: " + cleanupCount);

            // Clear references to trigger automatic cleanup
            obj2 = null;
            obj3 = null;
            cleanable2 = null;
            cleanable3 = null;

            // Suggest GC
            System.gc();
            Thread.sleep(100);

            System.out.println("Final cleanup count: " + cleanupCount);

        } catch (Exception e) {
            System.out.println("Cleaner not supported: " + e.getClass().getSimpleName());
        }
    }

    /**
     * Test edge cases and error conditions
     */
    public static void testCleanerEdgeCases() {
        System.out.println("\n--- Testing Cleaner Edge Cases ---");

        try {
            Cleaner cleaner = Cleaner.create();

            // Test with null action (should throw)
            try {
                Object obj = new Object();
                Cleaner.Cleanable cleanable = cleaner.register(obj, null);
                System.out.println("Null action registration: UNEXPECTED SUCCESS");
            } catch (NullPointerException e) {
                System.out.println("Null action registration correctly throws: " + e.getClass().getSimpleName());
            }

            // Test with exception in cleanup action
            cleanupExecuted = false;
            Object obj2 = new Object();
            Cleaner.Cleanable cleanable2 = cleaner.register(obj2, () -> {
                cleanupExecuted = true;
                System.out.println("Cleanup with exception executing");
                throw new RuntimeException("Test exception in cleanup");
            });

            cleanable2.clean();
            System.out.println("Cleanup with exception executed: " + cleanupExecuted);
            System.out.println("Exception in cleanup handled gracefully");

            // Test cleanup of already cleaned object
            cleanable2.clean(); // Should be safe
            System.out.println("Double cleanup completed safely");

        } catch (Exception e) {
            System.out.println("Cleaner not supported: " + e.getClass().getSimpleName());
        }
    }

    /**
     * Helper class for testing resource cleanup
     */
    static class TestResource {
        private final String name;
        private final Cleaner.Cleanable cleanable;

        public TestResource(String name, Cleaner cleaner) {
            this.name = name;
            this.cleanable = cleaner.register(this, () -> {
                System.out.println("TestResource cleanup: " + name);
                cleanupCount++;
            });
        }

        public void close() {
            cleanable.clean();
        }

        public String getName() {
            return name;
        }
    }
}
