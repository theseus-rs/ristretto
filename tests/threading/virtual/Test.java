public class Test {
    public static void main(String[] args) {
        System.out.println("=== Virtual Thread Tests ===");

        testVirtualThreadSupportCheck();
        testPlatformThreadCharacteristics();
        testMultipleThreadCreation();

        System.out.println("Virtual thread tests completed");
    }

    private static void testVirtualThreadSupportCheck() {
        // Note: Virtual threads were introduced in Java 19 as preview and finalized in Java 21
        // These tests will work on compatible JVMs, otherwise they'll show compatibility info

        try {
            System.out.println("Test 1: Virtual thread support check");

            // Try to create a virtual thread using reflection for compatibility
            Class<?> threadClass = Thread.class;
            java.lang.reflect.Method ofVirtualMethod = null;

            try {
                ofVirtualMethod = threadClass.getMethod("ofVirtual");
                System.out.println("Virtual thread support: Available");

                // Test virtual thread creation and execution
                Object builderObj = ofVirtualMethod.invoke(null);
                java.lang.reflect.Method startMethod = builderObj.getClass().getMethod("start", Runnable.class);

                Thread virtualThread = (Thread) startMethod.invoke(builderObj, (Runnable) () -> {
                    System.out.println("VirtualThread: Running on " + Thread.currentThread());
                    System.out.println("VirtualThread: Is virtual: " + Thread.currentThread().isVirtual());
                    try {
                        Thread.sleep(100);
                    } catch (InterruptedException e) {
                        System.out.println("VirtualThread interrupted");
                    }
                });

                virtualThread.join();

            } catch (NoSuchMethodException e) {
                System.out.println("Virtual thread support: Not available (requires Java 19+)");
                System.out.println("Running alternative tests with platform threads");
                runPlatformThreadTests();
            }

        } catch (Exception e) {
            System.out.println("Virtual thread test error: " + e.getClass().getSimpleName());
            runPlatformThreadTests();
        }
    }

    private static void testPlatformThreadCharacteristics() {
        System.out.println("Test 2: Platform thread characteristics");
        Thread platformThread = new Thread(() -> {
            System.out.println("PlatformThread: Running on " + Thread.currentThread());
            System.out.println("PlatformThread: Name: " + Thread.currentThread().getName());
            System.out.println("PlatformThread: Priority: " + Thread.currentThread().getPriority());
            System.out.println("PlatformThread: ThreadGroup: " + Thread.currentThread().getThreadGroup().getName());

            try {
                // Check if isVirtual method exists
                java.lang.reflect.Method isVirtualMethod = Thread.class.getMethod("isVirtual");
                boolean isVirtual = (Boolean) isVirtualMethod.invoke(Thread.currentThread());
                System.out.println("PlatformThread: Is virtual: " + isVirtual);
            } catch (Exception e) {
                System.out.println("PlatformThread: Is virtual: false (method not available)");
            }
        });

        platformThread.start();
        try {
            platformThread.join();
        } catch (InterruptedException e) {
            System.out.println("Platform thread test interrupted");
        }
    }

    private static void testMultipleThreadCreation() {
        System.out.println("Test 3: Multiple thread creation");
        int threadCount = 10; // Would be much higher with virtual threads
        Thread[] threads = new Thread[threadCount];

        for (int i = 0; i < threadCount; i++) {
            final int threadId = i;
            threads[i] = new Thread(() -> {
                System.out.println("Thread" + threadId + ": Starting work");
                try {
                    Thread.sleep(50 + (threadId * 10)); // Staggered timing
                } catch (InterruptedException e) {
                    System.out.println("Thread" + threadId + " interrupted");
                }
                System.out.println("Thread" + threadId + ": Work completed");
            });
        }

        long startTime = System.currentTimeMillis();
        for (Thread thread : threads) {
            thread.start();
        }

        try {
            for (Thread thread : threads) {
                thread.join();
            }
            long endTime = System.currentTimeMillis();
            System.out.println("All " + threadCount + " threads completed in " + (endTime - startTime) + "ms");
        } catch (InterruptedException e) {
            System.out.println("Multiple thread test interrupted");
        }
    }

    private static void runPlatformThreadTests() {
        System.out.println("Running enhanced platform thread tests:");

        // Test platform thread pools simulation
        Thread[] workerPool = new Thread[5];
        for (int i = 0; i < workerPool.length; i++) {
            final int workerId = i;
            workerPool[i] = new Thread(() -> {
                for (int j = 0; j < 3; j++) {
                    System.out.println("Worker" + workerId + ": Task " + j);
                    try {
                        Thread.sleep(100);
                    } catch (InterruptedException e) {
                        System.out.println("Worker" + workerId + " interrupted");
                        break;
                    }
                }
            });
        }

        for (Thread worker : workerPool) {
            worker.start();
        }

        try {
            for (Thread worker : workerPool) {
                worker.join();
            }
        } catch (InterruptedException e) {
            System.out.println("Worker pool test interrupted");
        }
    }
}
