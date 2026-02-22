public class Test {
    public static void main(String[] args) {
        System.out.println("=== Sleep Tests ===");

        testBasicSleep();
        testSleepWithNanoseconds();
        testSleepInterruption();
        testZeroSleep();
        testNegativeSleep();
        testMultipleThreadsSleeping();

        System.out.println("Sleep tests completed");
    }

    private static void testBasicSleep() {
        System.out.println("Test 1: Basic sleep");
        long startTime = System.currentTimeMillis();
        try {
            System.out.println("Sleeping for 100ms");
            Thread.sleep(100);
            long elapsed = System.currentTimeMillis() - startTime;
            System.out.println("Sleep completed in expected range: " + (elapsed >= 90 && elapsed <= 30000));
        } catch (InterruptedException e) {
            System.out.println("Sleep interrupted: " + e.getMessage());
        }
    }

    private static void testSleepWithNanoseconds() {
        System.out.println("Test 2: Sleep with nanoseconds");
        long startTime = System.currentTimeMillis();
        try {
            System.out.println("Sleeping for 50ms and 500000ns");
            Thread.sleep(50, 500000);
            long elapsed = System.currentTimeMillis() - startTime;
            System.out.println("Nano sleep completed in expected range: " + (elapsed >= 40 && elapsed <= 30000));
        } catch (InterruptedException e) {
            System.out.println("Nano sleep interrupted: " + e.getMessage());
        }
    }

    private static void testSleepInterruption() {
        System.out.println("Test 3: Sleep interruption");
        Thread sleepingThread = new Thread(() -> {
            try {
                System.out.println("Thread: Starting long sleep");
                Thread.sleep(2000);
                System.out.println("Thread: Sleep completed without interruption");
            } catch (InterruptedException e) {
                System.out.println("Thread: Sleep was interrupted");
            }
        });

        sleepingThread.start();

        // Interrupt after 200ms
        try {
            Thread.sleep(200);
            System.out.println("Main: Interrupting sleeping thread");
            sleepingThread.interrupt();
        } catch (InterruptedException e) {
            System.out.println("Main thread sleep interrupted");
        }

        try {
            sleepingThread.join();
        } catch (InterruptedException e) {
            System.out.println("Join interrupted in sleep test");
        }
    }

    private static void testZeroSleep() {
        System.out.println("Test 4: Zero sleep");
        long startTime = System.currentTimeMillis();
        try {
            Thread.sleep(0);
            long elapsed = System.currentTimeMillis() - startTime;
            System.out.println("Zero sleep completed in expected range: " + (elapsed >= 0 && elapsed <= 30000));
        } catch (InterruptedException e) {
            System.out.println("Zero sleep interrupted");
        }
    }

    private static void testNegativeSleep() {
        System.out.println("Test 5: Negative sleep");
        try {
            Thread.sleep(-1);
            System.out.println("ERROR: Negative sleep should throw exception");
        } catch (IllegalArgumentException e) {
            System.out.println("Correctly caught exception for negative sleep: " + e.getClass().getSimpleName());
        } catch (InterruptedException e) {
            System.out.println("Unexpected InterruptedException for negative sleep");
        }
    }

    private static void testMultipleThreadsSleeping() {
        System.out.println("Test 6: Multiple threads sleeping");
        // Run threads sequentially to ensure deterministic output order
        for (int i = 0; i < 3; i++) {
            final int threadNum = i;
            final int sleepTime = (i + 1) * 50;
            Thread sleeper = new Thread(() -> {
                try {
                    long threadStart = System.currentTimeMillis();
                    System.out.println("Sleeper " + threadNum + ": Sleeping");
                    Thread.sleep(sleepTime);
                    long elapsed = System.currentTimeMillis() - threadStart;
                    System.out.println("Sleeper " + threadNum + ": Woke up in expected range: " + (elapsed >= sleepTime - 20 && elapsed <= sleepTime + 30000));
                } catch (InterruptedException e) {
                    System.out.println("Sleeper " + threadNum + " interrupted");
                }
            });
            sleeper.start();
            try {
                sleeper.join();
            } catch (InterruptedException e) {
                System.out.println("Sleeper join interrupted");
            }
        }
    }
}
