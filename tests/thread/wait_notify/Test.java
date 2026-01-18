public class Test {
    public static void main(String[] args) {
        System.out.println("=== Wait/Notify/NotifyAll Tests ===");

        testBasicWaitNotify();
        testWaitWithTimeout();
        testProducerConsumerPattern();

        System.out.println("Wait/Notify/NotifyAll tests completed");
    }

    private static void testBasicWaitNotify() {
        System.out.println("Test 1: Basic wait with timeout");
        final Object lock = new Object();

        Thread waiter = new Thread(() -> {
            synchronized (lock) {
                try {
                    System.out.println("Waiter: Going to wait with timeout");
                    lock.wait(200);
                    System.out.println("Waiter: Woke up from wait");
                } catch (InterruptedException e) {
                    System.out.println("Waiter interrupted");
                }
            }
        });

        waiter.start();
        try {
            waiter.join();
        } catch (InterruptedException e) {
            System.out.println("Join interrupted");
        }
    }

    private static void testWaitWithTimeout() {
        System.out.println("Test 2: Wait with explicit timeout");
        final Object lock = new Object();

        Thread timeoutWaiter = new Thread(() -> {
            synchronized (lock) {
                try {
                    long startTime = System.currentTimeMillis();
                    System.out.println("TimeoutWaiter: Starting wait with 100ms timeout");
                    lock.wait(100);
                    long elapsed = System.currentTimeMillis() - startTime;
                    System.out.println("TimeoutWaiter: Woke up, elapsed in expected range: " + (elapsed >= 50 && elapsed <= 500));
                } catch (InterruptedException e) {
                    System.out.println("TimeoutWaiter interrupted");
                }
            }
        });

        timeoutWaiter.start();
        try {
            timeoutWaiter.join();
        } catch (InterruptedException e) {
            System.out.println("Join interrupted");
        }
    }

    private static void testProducerConsumerPattern() {
        System.out.println("Test 3: Simple producer-consumer pattern");
        final Object lock = new Object();
        final boolean[] dataReady = {false};

        Thread producer = new Thread(() -> {
            System.out.println("Producer: Producing data");
            try {
                Thread.sleep(50);
            } catch (InterruptedException e) {
                System.out.println("Producer interrupted");
            }
            synchronized (lock) {
                dataReady[0] = true;
                System.out.println("Producer: Data ready, notifying");
                lock.notify();
            }
        });

        Thread consumer = new Thread(() -> {
            synchronized (lock) {
                try {
                    System.out.println("Consumer: Waiting for data with timeout");
                    if (!dataReady[0]) {
                        lock.wait(500);
                    }
                    System.out.println("Consumer: Data ready status: " + dataReady[0]);
                } catch (InterruptedException e) {
                    System.out.println("Consumer interrupted");
                }
            }
        });

        producer.start();
        consumer.start();

        try {
            producer.join();
            consumer.join();
        } catch (InterruptedException e) {
            System.out.println("Join interrupted");
        }
    }
}
