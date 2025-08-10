public class Test {
    public static void main(String[] args) {
        System.out.println("=== Yield Tests ===");

        testBasicYieldBehavior();
        testYieldVsNoYieldComparison();
        testYieldInProducerConsumer();
        testYieldWithThreadPriorities();
        testCooperativeMultitasking();

        System.out.println("Yield tests completed");
    }

    private static void testBasicYieldBehavior() {
        System.out.println("Test 1: Basic yield behavior");
        Thread[] yieldThreads = new Thread[3];

        for (int i = 0; i < 3; i++) {
            final int threadId = i;
            yieldThreads[i] = new Thread(() -> {
                for (int j = 0; j < 5; j++) {
                    System.out.println("YieldThread" + threadId + ": Iteration " + j);
                    Thread.yield(); // Hint to scheduler to yield execution

                    // Small work to make yielding more observable
                    for (int k = 0; k < 1000000; k++) {
                        Math.sin(k); // CPU work
                    }
                }
                System.out.println("YieldThread" + threadId + ": Completed");
            });
        }

        for (Thread thread : yieldThreads) {
            thread.start();
        }

        try {
            for (Thread thread : yieldThreads) {
                thread.join();
            }
        } catch (InterruptedException e) {
            System.out.println("Yield test interrupted");
        }
    }

    private static void testYieldVsNoYieldComparison() {
        System.out.println("Test 2: Yield vs no yield comparison");
        final int[] executionOrder = new int[6];
        final int[] orderIndex = {0};

        // Threads without yield
        Thread noYield1 = new Thread(() -> {
            for (int i = 0; i < 3; i++) {
                synchronized (executionOrder) {
                    executionOrder[orderIndex[0]++] = 1;
                }
                System.out.println("NoYield1: " + i);
                // Busy work
                for (int j = 0; j < 500000; j++) {
                    Math.sqrt(j);
                }
            }
        });

        Thread noYield2 = new Thread(() -> {
            for (int i = 0; i < 3; i++) {
                synchronized (executionOrder) {
                    executionOrder[orderIndex[0]++] = 2;
                }
                System.out.println("NoYield2: " + i);
                // Busy work
                for (int j = 0; j < 500000; j++) {
                    Math.sqrt(j);
                }
            }
        });

        noYield1.start();
        noYield2.start();

        try {
            noYield1.join();
            noYield2.join();
        } catch (InterruptedException e) {
            System.out.println("No yield test interrupted");
        }

        System.out.print("Execution order (no yield): ");
        for (int i = 0; i < orderIndex[0]; i++) {
            System.out.print(executionOrder[i] + " ");
        }
        System.out.println();

        // Reset for yield test
        orderIndex[0] = 0;
        for (int i = 0; i < executionOrder.length; i++) {
            executionOrder[i] = 0;
        }

        // Threads with yield
        Thread withYield1 = new Thread(() -> {
            for (int i = 0; i < 3; i++) {
                synchronized (executionOrder) {
                    executionOrder[orderIndex[0]++] = 3;
                }
                System.out.println("WithYield1: " + i);
                Thread.yield();
                // Busy work
                for (int j = 0; j < 500000; j++) {
                    Math.sqrt(j);
                }
            }
        });

        Thread withYield2 = new Thread(() -> {
            for (int i = 0; i < 3; i++) {
                synchronized (executionOrder) {
                    executionOrder[orderIndex[0]++] = 4;
                }
                System.out.println("WithYield2: " + i);
                Thread.yield();
                // Busy work
                for (int j = 0; j < 500000; j++) {
                    Math.sqrt(j);
                }
            }
        });

        withYield1.start();
        withYield2.start();

        try {
            withYield1.join();
            withYield2.join();
        } catch (InterruptedException e) {
            System.out.println("With yield test interrupted");
        }

        System.out.print("Execution order (with yield): ");
        for (int i = 0; i < orderIndex[0]; i++) {
            System.out.print(executionOrder[i] + " ");
        }
        System.out.println();
    }

    private static void testYieldInProducerConsumer() {
        System.out.println("Test 3: Yield in producer-consumer");
        final StringBuilder buffer = new StringBuilder();
        final boolean[] done = {false};

        Thread producer = new Thread(() -> {
            for (int i = 0; i < 5; i++) {
                synchronized (buffer) {
                    buffer.append("P").append(i).append(" ");
                    System.out.println("Producer: Added P" + i);
                }
                Thread.yield(); // Give consumer a chance
            }
            done[0] = true;
        });

        Thread consumer = new Thread(() -> {
            while (!done[0] || buffer.length() > 0) {
                synchronized (buffer) {
                    if (buffer.length() > 0) {
                        System.out.println("Consumer: Buffer content: " + buffer.toString().trim());
                        buffer.setLength(0); // Clear buffer
                    }
                }
                Thread.yield(); // Give producer a chance
                try {
                    Thread.sleep(50); // Prevent tight loop
                } catch (InterruptedException e) {
                    System.out.println("Consumer interrupted");
                    break;
                }
            }
        });

        producer.start();
        consumer.start();

        try {
            producer.join();
            consumer.join();
        } catch (InterruptedException e) {
            System.out.println("Producer-consumer yield test interrupted");
        }
    }

    private static void testYieldWithThreadPriorities() {
        System.out.println("Test 4: Yield with thread priorities");
        Thread highPriorityYield = new Thread(() -> {
            for (int i = 0; i < 3; i++) {
                System.out.println("HighPriority: " + i + " (priority: " + Thread.currentThread().getPriority() + ")");
                Thread.yield();
                try {
                    Thread.sleep(10);
                } catch (InterruptedException e) {
                    System.out.println("HighPriority interrupted");
                }
            }
        });

        Thread lowPriorityYield = new Thread(() -> {
            for (int i = 0; i < 3; i++) {
                System.out.println("LowPriority: " + i + " (priority: " + Thread.currentThread().getPriority() + ")");
                Thread.yield();
                try {
                    Thread.sleep(10);
                } catch (InterruptedException e) {
                    System.out.println("LowPriority interrupted");
                }
            }
        });

        highPriorityYield.setPriority(Thread.MAX_PRIORITY);
        lowPriorityYield.setPriority(Thread.MIN_PRIORITY);

        highPriorityYield.start();
        lowPriorityYield.start();

        try {
            highPriorityYield.join();
            lowPriorityYield.join();
        } catch (InterruptedException e) {
            System.out.println("Priority yield test interrupted");
        }
    }

    private static void testCooperativeMultitasking() {
        System.out.println("Test 5: Cooperative multitasking");
        CooperativeTask[] tasks = new CooperativeTask[3];
        Thread[] taskThreads = new Thread[3];

        for (int i = 0; i < 3; i++) {
            tasks[i] = new CooperativeTask(i);
            taskThreads[i] = new Thread(tasks[i]);
            taskThreads[i].start();
        }

        try {
            for (Thread thread : taskThreads) {
                thread.join();
            }
        } catch (InterruptedException e) {
            System.out.println("Cooperative multitasking test interrupted");
        }
    }

    static class CooperativeTask implements Runnable {
        private final int taskId;
        private int workDone = 0;

        public CooperativeTask(int taskId) {
            this.taskId = taskId;
        }

        public void run() {
            while (workDone < 5) {
                System.out.println("CooperativeTask" + taskId + ": Work unit " + workDone);
                workDone++;

                // Do some work
                for (int i = 0; i < 100000; i++) {
                    Math.random();
                }

                // Cooperatively yield to other tasks
                Thread.yield();

                try {
                    Thread.sleep(20); // Brief pause
                } catch (InterruptedException e) {
                    System.out.println("CooperativeTask" + taskId + " interrupted");
                    break;
                }
            }
            System.out.println("CooperativeTask" + taskId + ": Completed all work");
        }
    }
}
