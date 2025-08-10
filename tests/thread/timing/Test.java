public class Test {
    public static void main(String[] args) {
        System.out.println("=== Timing Tests ===");

        testThreadExecutionTiming();
        testConcurrentExecutionTiming();
        testThreadSchedulingTiming();
        testSystemTimingPrecision();
        testThreadCreationAndStartupTiming();

        System.out.println("Timing tests completed");
    }

    private static void testThreadExecutionTiming() {
        System.out.println("Test 1: Thread execution timing");
        long mainStart = System.currentTimeMillis();

        Thread timedThread = new Thread(() -> {
            long threadStart = System.currentTimeMillis();
            System.out.println("TimedThread: Started at " + threadStart);

            try {
                // Simulate work
                for (int i = 0; i < 5; i++) {
                    Thread.sleep(50);
                    System.out.println("TimedThread: Iteration " + i + " at " +
                                     (System.currentTimeMillis() - threadStart) + "ms");
                }
            } catch (InterruptedException e) {
                System.out.println("TimedThread interrupted");
            }

            long threadEnd = System.currentTimeMillis();
            System.out.println("TimedThread: Completed in " + (threadEnd - threadStart) + "ms");
        });

        timedThread.start();

        try {
            timedThread.join();
            long mainEnd = System.currentTimeMillis();
            System.out.println("Main: Total time " + (mainEnd - mainStart) + "ms");
        } catch (InterruptedException e) {
            System.out.println("Timing test interrupted");
        }
    }

    private static void testConcurrentExecutionTiming() {
        System.out.println("Test 2: Concurrent execution timing");
        Thread[] concurrentThreads = new Thread[3];
        long concurrentStart = System.currentTimeMillis();

        for (int i = 0; i < 3; i++) {
            final int threadId = i;
            concurrentThreads[i] = new Thread(() -> {
                long start = System.currentTimeMillis();
                try {
                    Thread.sleep(200); // All sleep for same duration
                } catch (InterruptedException e) {
                    System.out.println("Concurrent thread " + threadId + " interrupted");
                }
                long end = System.currentTimeMillis();
                System.out.println("ConcurrentThread" + threadId + ": " + (end - start) + "ms");
            });
        }

        // Start all threads
        for (Thread thread : concurrentThreads) {
            thread.start();
        }

        // Wait for all to complete
        try {
            for (Thread thread : concurrentThreads) {
                thread.join();
            }
            long concurrentEnd = System.currentTimeMillis();
            System.out.println("All concurrent threads completed in " +
                             (concurrentEnd - concurrentStart) + "ms");
        } catch (InterruptedException e) {
            System.out.println("Concurrent timing test interrupted");
        }
    }

    private static void testThreadSchedulingTiming() {
        System.out.println("Test 3: Thread scheduling timing");
        final long[] schedulingTimes = new long[2];

        Thread scheduler1 = new Thread(() -> {
            schedulingTimes[0] = System.currentTimeMillis();
            System.out.println("Scheduler1: Started at " + schedulingTimes[0]);
            Thread.yield(); // Yield to other threads
            try {
                Thread.sleep(10);
            } catch (InterruptedException e) {
                System.out.println("Scheduler1 interrupted");
            }
        });

        Thread scheduler2 = new Thread(() -> {
            schedulingTimes[1] = System.currentTimeMillis();
            System.out.println("Scheduler2: Started at " + schedulingTimes[1]);
            try {
                Thread.sleep(10);
            } catch (InterruptedException e) {
                System.out.println("Scheduler2 interrupted");
            }
        });

        scheduler1.start();
        scheduler2.start();

        try {
            scheduler1.join();
            scheduler2.join();

            if (schedulingTimes[0] != 0 && schedulingTimes[1] != 0) {
                long timeDiff = Math.abs(schedulingTimes[1] - schedulingTimes[0]);
                System.out.println("Thread start time difference: " + timeDiff + "ms");
            }
        } catch (InterruptedException e) {
            System.out.println("Scheduling test interrupted");
        }
    }

    private static void testSystemTimingPrecision() {
        System.out.println("Test 4: System timing precision");
        long[] measurements = new long[5];
        for (int i = 0; i < 5; i++) {
            long start = System.currentTimeMillis();
            try {
                Thread.sleep(10);
            } catch (InterruptedException e) {
                System.out.println("Precision test interrupted");
            }
            long end = System.currentTimeMillis();
            measurements[i] = end - start;
            System.out.println("Measurement " + i + ": " + measurements[i] + "ms");
        }

        // Calculate average
        long sum = 0;
        for (long measurement : measurements) {
            sum += measurement;
        }
        System.out.println("Average sleep time: " + (sum / measurements.length) + "ms");
    }

    private static void testThreadCreationAndStartupTiming() {
        System.out.println("Test 5: Thread creation and startup timing");
        long creationStart = System.currentTimeMillis();

        Thread[] startupThreads = new Thread[10];
        for (int i = 0; i < 10; i++) {
            final int threadNum = i;
            startupThreads[i] = new Thread(() -> {
                long startupTime = System.currentTimeMillis();
                System.out.println("StartupThread" + threadNum + ": Running at " + startupTime);
            });
        }

        long startTime = System.currentTimeMillis();
        for (Thread thread : startupThreads) {
            thread.start();
        }

        try {
            for (Thread thread : startupThreads) {
                thread.join();
            }
            long endTime = System.currentTimeMillis();
            System.out.println("Thread creation phase: " + (startTime - creationStart) + "ms");
            System.out.println("Thread execution phase: " + (endTime - startTime) + "ms");
        } catch (InterruptedException e) {
            System.out.println("Startup timing test interrupted");
        }
    }
}
