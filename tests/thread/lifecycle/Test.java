public class Test {
    public static void main(String[] args) {
        System.out.println("=== Lifecycle Tests ===");

        testThreadStateTransitions();
        testThreadPriorityLifecycle();
        testThreadNameLifecycle();
        testThreadWithRunnableLifecycle();
        testMultipleStartAttempts();
        testThreadWithThreadGroupLifecycle();
        testUncaughtExceptionHandlerLifecycle();

        System.out.println("Lifecycle tests completed");
    }

    private static void testThreadStateTransitions() {
        System.out.println("Test 1: Thread state transitions");
        Thread lifecycleThread = new Thread(() -> {
            try {
                System.out.println("LifecycleThread: Running state");
                Thread.sleep(200);

                synchronized (Test.class) {
                    System.out.println("LifecycleThread: In synchronized block");
                    Thread.sleep(100);
                }
            } catch (InterruptedException e) {
                System.out.println("LifecycleThread: Interrupted");
            }
        });

        System.out.println("State NEW: " + lifecycleThread.getState());
        System.out.println("Is alive before start: " + lifecycleThread.isAlive());

        lifecycleThread.start();
        System.out.println("State after start: " + lifecycleThread.getState());
        System.out.println("Is alive after start: " + lifecycleThread.isAlive());

        try {
            Thread.sleep(50);
            System.out.println("State during execution: " + lifecycleThread.getState());

            lifecycleThread.join();
            System.out.println("State after completion: " + lifecycleThread.getState());
            System.out.println("Is alive after completion: " + lifecycleThread.isAlive());
        } catch (InterruptedException e) {
            System.out.println("Lifecycle test interrupted");
        }
    }

    private static void testThreadPriorityLifecycle() {
        System.out.println("Test 2: Thread priority lifecycle");
        Thread[] priorityThreads = new Thread[3];
        int[] priorities = {Thread.MIN_PRIORITY, Thread.NORM_PRIORITY, Thread.MAX_PRIORITY};

        for (int i = 0; i < 3; i++) {
            final int threadId = i;
            priorityThreads[i] = new Thread(() -> {
                System.out.println("PriorityThread" + threadId + ": Priority " +
                                 Thread.currentThread().getPriority() + " running");
                try {
                    Thread.sleep(100);
                } catch (InterruptedException e) {
                    System.out.println("PriorityThread" + threadId + " interrupted");
                }
            });
            priorityThreads[i].setPriority(priorities[i]);
            System.out.println("Set priority " + priorities[i] + " for thread " + i);
        }

        for (Thread thread : priorityThreads) {
            thread.start();
        }

        try {
            for (Thread thread : priorityThreads) {
                thread.join();
            }
        } catch (InterruptedException e) {
            System.out.println("Priority lifecycle test interrupted");
        }
    }

    private static void testThreadNameLifecycle() {
        System.out.println("Test 3: Thread name lifecycle");
        Thread namedThread = new Thread(() -> {
            System.out.println("Initial name: " + Thread.currentThread().getName());
            Thread.currentThread().setName("RuntimeName");
            System.out.println("Changed name: " + Thread.currentThread().getName());
        }, "InitialName");

        System.out.println("Name before start: " + namedThread.getName());
        namedThread.start();

        try {
            namedThread.join();
            System.out.println("Name after completion: " + namedThread.getName());
        } catch (InterruptedException e) {
            System.out.println("Named thread test interrupted");
        }
    }

    private static void testThreadWithRunnableLifecycle() {
        System.out.println("Test 4: Thread with Runnable lifecycle");
        TestRunnable runnable = new TestRunnable();
        Thread runnableThread = new Thread(runnable, "RunnableThread");

        System.out.println("Runnable execution count before: " + runnable.getExecutionCount());
        runnableThread.start();

        try {
            runnableThread.join();
            System.out.println("Runnable execution count after: " + runnable.getExecutionCount());
        } catch (InterruptedException e) {
            System.out.println("Runnable thread test interrupted");
        }
    }

    private static void testMultipleStartAttempts() {
        System.out.println("Test 5: Multiple start attempts");
        Thread multiStartThread = new Thread(() -> {
            System.out.println("MultiStartThread: Running once");
        });

        multiStartThread.start();
        try {
            multiStartThread.join();
        } catch (InterruptedException e) {
            System.out.println("Multi-start thread interrupted");
        }

        try {
            multiStartThread.start(); // Should throw exception
            System.out.println("ERROR: Should not be able to start thread twice");
        } catch (IllegalThreadStateException e) {
            System.out.println("Correctly caught exception on second start: " + e.getClass().getSimpleName());
        }
    }

    private static void testThreadWithThreadGroupLifecycle() {
        System.out.println("Test 6: Thread with ThreadGroup lifecycle");
        ThreadGroup lifecycleGroup = new ThreadGroup("LifecycleGroup");
        Thread groupThread = new Thread(lifecycleGroup, () -> {
            System.out.println("GroupThread: In group " + Thread.currentThread().getThreadGroup().getName());
            try {
                Thread.sleep(100);
            } catch (InterruptedException e) {
                System.out.println("GroupThread interrupted");
            }
        }, "GroupThread");

        System.out.println("Group active count before start: " + lifecycleGroup.activeCount());
        groupThread.start();

        try {
            Thread.sleep(50);
            System.out.println("Group active count during execution: " + lifecycleGroup.activeCount());

            groupThread.join();
            System.out.println("Group active count after completion: " + lifecycleGroup.activeCount());
        } catch (InterruptedException e) {
            System.out.println("Group thread test interrupted");
        }
    }

    private static void testUncaughtExceptionHandlerLifecycle() {
        System.out.println("Test 7: Uncaught exception handler lifecycle");
        Thread.UncaughtExceptionHandler handler = new Thread.UncaughtExceptionHandler() {
            public void uncaughtException(Thread t, Throwable e) {
                System.out.println("Uncaught exception in thread " + t.getName() + ": " + e.getClass().getSimpleName());
            }
        };

        Thread exceptionThread = new Thread(() -> {
            System.out.println("ExceptionThread: About to throw exception");
            throw new RuntimeException("Test exception");
        });

        exceptionThread.setUncaughtExceptionHandler(handler);
        exceptionThread.start();

        try {
            exceptionThread.join();
            System.out.println("Exception thread completed (state: " + exceptionThread.getState() + ")");
        } catch (InterruptedException e) {
            System.out.println("Exception thread test interrupted");
        }
    }

    static class TestRunnable implements Runnable {
        private int executionCount = 0;

        public void run() {
            executionCount++;
            System.out.println("TestRunnable: Execution " + executionCount);
            try {
                Thread.sleep(50);
            } catch (InterruptedException e) {
                System.out.println("TestRunnable interrupted");
            }
        }

        public int getExecutionCount() {
            return executionCount;
        }
    }
}
