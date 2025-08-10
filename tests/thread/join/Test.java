public class Test {
    public static void main(String[] args) {
        System.out.println("=== Join Tests ===");

        testBasicJoin();
        testJoinWithTimeout();
        testJoinWithNanoseconds();
        testMultipleThreadJoin();
        testJoinOnCompletedThread();
        testSelfJoinPrevention();

        System.out.println("Join tests completed");
    }

    private static void testBasicJoin() {
        System.out.println("Test 1: Basic join");
        Thread worker = new Thread(() -> {
            try {
                System.out.println("Worker: Starting work");
                Thread.sleep(200);
                System.out.println("Worker: Work completed");
            } catch (InterruptedException e) {
                System.out.println("Worker interrupted");
            }
        });

        long startTime = System.currentTimeMillis();
        worker.start();

        try {
            System.out.println("Main: Waiting for worker to complete");
            worker.join();
            long endTime = System.currentTimeMillis();
            System.out.println("Main: Worker completed after " + (endTime - startTime) + "ms");
        } catch (InterruptedException e) {
            System.out.println("Main thread interrupted during join");
        }
    }

    private static void testJoinWithTimeout() {
        System.out.println("Test 2: Join with timeout");
        Thread longWorker = new Thread(() -> {
            try {
                System.out.println("LongWorker: Starting long work (1000ms)");
                Thread.sleep(1000);
                System.out.println("LongWorker: Work completed");
            } catch (InterruptedException e) {
                System.out.println("LongWorker interrupted");
            }
        });

        long startTime = System.currentTimeMillis();
        longWorker.start();

        try {
            System.out.println("Main: Waiting for long worker with 300ms timeout");
            longWorker.join(300);
            long endTime = System.currentTimeMillis();
            System.out.println("Main: Join returned after " + (endTime - startTime) + "ms");
            System.out.println("LongWorker alive status: " + longWorker.isAlive());
        } catch (InterruptedException e) {
            System.out.println("Main thread interrupted during timeout join");
        }

        // Wait for the long worker to complete
        try {
            longWorker.join();
        } catch (InterruptedException e) {
            System.out.println("Final join interrupted");
        }
    }

    private static void testJoinWithNanoseconds() {
        System.out.println("Test 3: Join with nanoseconds");
        Thread nanoWorker = new Thread(() -> {
            try {
                System.out.println("NanoWorker: Starting nano work");
                Thread.sleep(150);
                System.out.println("NanoWorker: Work completed");
            } catch (InterruptedException e) {
                System.out.println("NanoWorker interrupted");
            }
        });

        long startTime = System.currentTimeMillis();
        nanoWorker.start();

        try {
            System.out.println("Main: Waiting with 100ms and 500000ns timeout");
            nanoWorker.join(100, 500000);
            long endTime = System.currentTimeMillis();
            System.out.println("Main: Nano join returned after " + (endTime - startTime) + "ms");
            System.out.println("NanoWorker alive status: " + nanoWorker.isAlive());
        } catch (InterruptedException e) {
            System.out.println("Main thread interrupted during nano join");
        }

        try {
            nanoWorker.join();
        } catch (InterruptedException e) {
            System.out.println("Nano worker final join interrupted");
        }
    }

    private static void testMultipleThreadJoin() {
        System.out.println("Test 4: Multiple thread join");
        Thread[] workers = new Thread[3];
        for (int i = 0; i < 3; i++) {
            final int workerId = i;
            final int workTime = (i + 1) * 100;
            workers[i] = new Thread(() -> {
                try {
                    System.out.println("Worker" + workerId + ": Starting " + workTime + "ms work");
                    Thread.sleep(workTime);
                    System.out.println("Worker" + workerId + ": Work completed");
                } catch (InterruptedException e) {
                    System.out.println("Worker" + workerId + " interrupted");
                }
            });
        }

        // Start all workers
        long startTime = System.currentTimeMillis();
        for (Thread worker : workers) {
            worker.start();
        }

        // Join all workers
        try {
            for (int i = 0; i < workers.length; i++) {
                System.out.println("Main: Joining worker " + i);
                workers[i].join();
                long currentTime = System.currentTimeMillis();
                System.out.println("Main: Worker " + i + " joined after " + (currentTime - startTime) + "ms");
            }
        } catch (InterruptedException e) {
            System.out.println("Multiple worker join interrupted");
        }
    }

    private static void testJoinOnCompletedThread() {
        System.out.println("Test 5: Join on completed thread");
        Thread completedWorker = new Thread(() -> {
            System.out.println("CompletedWorker: Quick work");
        });

        completedWorker.start();
        try {
            completedWorker.join();
            System.out.println("First join completed");

            // Try to join again
            long startTime = System.currentTimeMillis();
            completedWorker.join();
            long endTime = System.currentTimeMillis();
            System.out.println("Second join on completed thread took " + (endTime - startTime) + "ms");
        } catch (InterruptedException e) {
            System.out.println("Completed worker join interrupted");
        }
    }

    private static void testSelfJoinPrevention() {
        System.out.println("Test 6: Self-join prevention");
        Thread selfJoinThread = new Thread(() -> {
            try {
                System.out.println("SelfJoinThread: Attempting self-join with timeout");
                Thread.currentThread().join(100);
                System.out.println("SelfJoinThread: Self-join returned (expected with timeout)");
            } catch (InterruptedException e) {
                System.out.println("SelfJoinThread: Self-join interrupted");
            }
        });

        selfJoinThread.start();
        try {
            selfJoinThread.join();
        } catch (InterruptedException e) {
            System.out.println("Self-join thread join interrupted");
        }
    }
}
