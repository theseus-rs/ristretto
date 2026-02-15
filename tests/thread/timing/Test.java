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
            System.out.println("TimedThread: Started");

            try {
                // Simulate work
                for (int i = 0; i < 5; i++) {
                    Thread.sleep(20);
                    System.out.println("TimedThread: Iteration " + i);
                }
            } catch (InterruptedException e) {
                System.out.println("TimedThread interrupted");
            }

            long elapsed = System.currentTimeMillis() - threadStart;
            System.out.println("TimedThread: Completed in expected range: " + (elapsed >= 80 && elapsed <= 30000));
        });

        timedThread.start();

        try {
            timedThread.join();
            long mainElapsed = System.currentTimeMillis() - mainStart;
            System.out.println("Main: Completed in expected range: " + (mainElapsed >= 80 && mainElapsed <= 30000));
        } catch (InterruptedException e) {
            System.out.println("Timing test interrupted");
        }
    }

    private static void testConcurrentExecutionTiming() {
        System.out.println("Test 2: Concurrent execution timing");
        long concurrentStart = System.currentTimeMillis();

        // Run threads sequentially for deterministic output
        for (int i = 0; i < 3; i++) {
            final int threadId = i;
            Thread thread = new Thread(() -> {
                long start = System.currentTimeMillis();
                try {
                    Thread.sleep(50);
                } catch (InterruptedException e) {
                    System.out.println("Concurrent thread " + threadId + " interrupted");
                }
                long elapsed = System.currentTimeMillis() - start;
                System.out.println("ConcurrentThread" + threadId + " completed in expected range: " + (elapsed >= 40 && elapsed <= 30000));
            });
            thread.start();
            try {
                thread.join();
            } catch (InterruptedException e) {
                System.out.println("Join interrupted");
            }
        }

        long concurrentEnd = System.currentTimeMillis();
        System.out.println("All concurrent threads completed in expected range: " + ((concurrentEnd - concurrentStart) >= 120 && (concurrentEnd - concurrentStart) <= 60000));
    }

    private static void testThreadSchedulingTiming() {
        System.out.println("Test 3: Thread scheduling timing");

        // Run threads sequentially for deterministic output
        Thread scheduler1 = new Thread(() -> {
            System.out.println("Scheduler1: Started");
            Thread.yield();
            try {
                Thread.sleep(10);
            } catch (InterruptedException e) {
                System.out.println("Scheduler1 interrupted");
            }
        });

        scheduler1.start();
        try {
            scheduler1.join();
        } catch (InterruptedException e) {
            System.out.println("Join interrupted");
        }

        Thread scheduler2 = new Thread(() -> {
            System.out.println("Scheduler2: Started");
            try {
                Thread.sleep(10);
            } catch (InterruptedException e) {
                System.out.println("Scheduler2 interrupted");
            }
        });

        scheduler2.start();
        try {
            scheduler2.join();
            System.out.println("Both schedulers completed");
        } catch (InterruptedException e) {
            System.out.println("Scheduling test interrupted");
        }
    }

    private static void testSystemTimingPrecision() {
        System.out.println("Test 4: System timing precision");
        int successCount = 0;
        for (int i = 0; i < 5; i++) {
            long start = System.currentTimeMillis();
            try {
                Thread.sleep(10);
            } catch (InterruptedException e) {
                System.out.println("Precision test interrupted");
            }
            long elapsed = System.currentTimeMillis() - start;
            boolean inRange = elapsed >= 5 && elapsed <= 30000;
            System.out.println("Measurement " + i + " in expected range: " + inRange);
            if (inRange) successCount++;
        }
        System.out.println("Timing precision acceptable: " + (successCount >= 3));
    }

    private static void testThreadCreationAndStartupTiming() {
        System.out.println("Test 5: Thread creation and startup timing");

        // Run threads sequentially for deterministic output
        for (int i = 0; i < 3; i++) {
            final int threadNum = i;
            Thread thread = new Thread(() -> {
                System.out.println("StartupThread" + threadNum + ": Running");
            });
            thread.start();
            try {
                thread.join();
            } catch (InterruptedException e) {
                System.out.println("Join interrupted");
            }
        }
        System.out.println("Thread startup test completed");
    }
}
