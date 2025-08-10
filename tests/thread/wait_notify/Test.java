public class Test {
    private static final Object lock = new Object();
    private static boolean condition = false;
    private static int waitingThreads = 0;
    private static int notifiedThreads = 0;

    public static void main(String[] args) {
        System.out.println("=== Wait/Notify/NotifyAll Tests ===");

        testBasicWaitNotify();
        testNotifyAllWithMultipleWaiters();
        testWaitWithTimeout();
        testProducerConsumerPattern();

        System.out.println("Wait/Notify/NotifyAll tests completed");
    }

    private static void testBasicWaitNotify() {
        System.out.println("Test 1: Basic wait/notify");
        Thread waiter = new Thread(() -> {
            synchronized (lock) {
                try {
                    System.out.println("Waiter: Going to wait");
                    waitingThreads++;
                    lock.wait();
                    System.out.println("Waiter: Woke up from wait");
                } catch (InterruptedException e) {
                    System.out.println("Waiter interrupted: " + e.getMessage());
                }
            }
        });

        Thread notifier = new Thread(() -> {
            try {
                Thread.sleep(100); // Ensure waiter goes to wait first
            } catch (InterruptedException e) {
                System.out.println("Notifier sleep interrupted");
            }
            synchronized (lock) {
                System.out.println("Notifier: Sending notify");
                lock.notify();
            }
        });

        waiter.start();
        notifier.start();

        try {
            waiter.join();
            notifier.join();
        } catch (InterruptedException e) {
            System.out.println("Join interrupted in test 1");
        }
    }

    private static void testNotifyAllWithMultipleWaiters() {
        System.out.println("Test 2: NotifyAll with multiple waiters");
        waitingThreads = 0;
        notifiedThreads = 0;

        Thread[] waiters = new Thread[3];
        for (int i = 0; i < 3; i++) {
            final int threadNum = i;
            waiters[i] = new Thread(() -> {
                synchronized (lock) {
                    try {
                        System.out.println("Waiter " + threadNum + ": Going to wait");
                        waitingThreads++;
                        lock.wait();
                        notifiedThreads++;
                        System.out.println("Waiter " + threadNum + ": Woke up from wait");
                    } catch (InterruptedException e) {
                        System.out.println("Waiter " + threadNum + " interrupted");
                    }
                }
            });
            waiters[i].start();
        }

        // Wait for all threads to enter wait state
        try {
            Thread.sleep(200);
        } catch (InterruptedException e) {
            System.out.println("Main sleep interrupted");
        }

        Thread notifyAllThread = new Thread(() -> {
            synchronized (lock) {
                System.out.println("NotifyAll: Waking up all waiters");
                lock.notifyAll();
            }
        });
        notifyAllThread.start();

        try {
            for (Thread waiter : waiters) {
                waiter.join();
            }
            notifyAllThread.join();
        } catch (InterruptedException e) {
            System.out.println("Join interrupted in test 2");
        }

        System.out.println("Threads that were waiting: " + waitingThreads);
        System.out.println("Threads that were notified: " + notifiedThreads);
    }

    private static void testWaitWithTimeout() {
        System.out.println("Test 3: Wait with timeout");
        Thread timeoutWaiter = new Thread(() -> {
            synchronized (lock) {
                try {
                    long startTime = System.currentTimeMillis();
                    System.out.println("Timeout waiter: Starting wait with 500ms timeout");
                    lock.wait(500);
                    long endTime = System.currentTimeMillis();
                    System.out.println("Timeout waiter: Woke up after " + (endTime - startTime) + "ms");
                } catch (InterruptedException e) {
                    System.out.println("Timeout waiter interrupted");
                }
            }
        });

        timeoutWaiter.start();
        try {
            timeoutWaiter.join();
        } catch (InterruptedException e) {
            System.out.println("Timeout waiter join interrupted");
        }
    }

    private static void testProducerConsumerPattern() {
        System.out.println("Test 4: Producer-Consumer pattern");
        ProducerConsumerTest.run();
    }

    static class ProducerConsumerTest {
        private static final Object buffer = new Object();
        private static String data = null;

        public static void run() {
            Thread producer = new Thread(() -> {
                synchronized (buffer) {
                    try {
                        Thread.sleep(100);
                        data = "Produced Data";
                        System.out.println("Producer: Data produced");
                        buffer.notify();
                    } catch (InterruptedException e) {
                        System.out.println("Producer interrupted");
                    }
                }
            });

            Thread consumer = new Thread(() -> {
                synchronized (buffer) {
                    try {
                        while (data == null) {
                            System.out.println("Consumer: Waiting for data");
                            buffer.wait();
                        }
                        System.out.println("Consumer: Consumed " + data);
                    } catch (InterruptedException e) {
                        System.out.println("Consumer interrupted");
                    }
                }
            });

            consumer.start();
            producer.start();

            try {
                producer.join();
                consumer.join();
            } catch (InterruptedException e) {
                System.out.println("Producer-Consumer join interrupted");
            }
        }
    }
}
