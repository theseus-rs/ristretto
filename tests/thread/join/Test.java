public class Test {
    public static void main(String[] args) {
        System.out.println("=== Join Tests ===");

        testBasicJoin();
        testJoinWithTimeout();
        testMultipleThreadJoin();
        testJoinOnCompletedThread();

        System.out.println("Join tests completed");
    }

    private static void testBasicJoin() {
        System.out.println("Test 1: Basic join");
        final boolean[] done = {false};
        Thread worker = new Thread(() -> {
            // No output from worker thread
            long start = System.currentTimeMillis();
            while (System.currentTimeMillis() - start < 100) {
                // Busy wait
            }
            done[0] = true;
        });

        worker.start();

        try {
            System.out.println("Main: Waiting for worker");
            worker.join();
            System.out.println("Main: Worker joined, done=" + done[0]);
        } catch (InterruptedException e) {
            System.out.println("Main interrupted");
        }
    }

    private static void testJoinWithTimeout() {
        System.out.println("Test 2: Join with timeout");
        final boolean[] done = {false};
        Thread longWorker = new Thread(() -> {
            // No output from worker thread
            long start = System.currentTimeMillis();
            while (System.currentTimeMillis() - start < 200) {
                // Busy wait
            }
            done[0] = true;
        });

        longWorker.start();

        try {
            System.out.println("Main: Waiting with timeout");
            longWorker.join(50);
            System.out.println("Main: Join returned");
        } catch (InterruptedException e) {
            System.out.println("Main interrupted");
        }

        try {
            longWorker.join();
            System.out.println("Main: Final join, done=" + done[0]);
        } catch (InterruptedException e) {
            System.out.println("Final join interrupted");
        }
    }

    private static void testMultipleThreadJoin() {
        System.out.println("Test 3: Multiple thread join");
        for (int i = 0; i < 2; i++) {
            final int workerId = i;
            final boolean[] done = {false};
            Thread worker = new Thread(() -> {
                done[0] = true;
            });

            System.out.println("Main: Starting worker " + workerId);
            worker.start();
            try {
                worker.join();
                System.out.println("Main: Worker " + workerId + " joined, done=" + done[0]);
            } catch (InterruptedException e) {
                System.out.println("Worker join interrupted");
            }
        }
    }

    private static void testJoinOnCompletedThread() {
        System.out.println("Test 4: Join on completed thread");
        Thread completedWorker = new Thread(() -> {
            // No output
        });

        completedWorker.start();
        try {
            completedWorker.join();
            System.out.println("First join completed");
            completedWorker.join();
            System.out.println("Second join completed");
        } catch (InterruptedException e) {
            System.out.println("Join interrupted");
        }
    }
}
