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
        final boolean[] done = {false};
        
        Thread lifecycleThread = new Thread(() -> {
            // Busy wait long enough for main thread to check state
            long start = System.currentTimeMillis();
            while (System.currentTimeMillis() - start < 500) {
                // Busy wait
            }
            synchronized (Test.class) {
                done[0] = true;
            }
        });

        System.out.println("State NEW: " + lifecycleThread.getState());
        System.out.println("Is alive before start: " + lifecycleThread.isAlive());

        lifecycleThread.start();
        
        // Brief sleep to let thread start
        try {
            Thread.sleep(50);
        } catch (InterruptedException e) {
            // ignore
        }
        
        System.out.println("State after start: " + lifecycleThread.getState());
        System.out.println("Is alive after start: " + lifecycleThread.isAlive());
        System.out.println("State during execution: " + lifecycleThread.getState());

        try {
            lifecycleThread.join();
            System.out.println("State after completion: " + lifecycleThread.getState());
            System.out.println("Is alive after completion: " + lifecycleThread.isAlive());
            System.out.println("Lifecycle done: " + done[0]);
        } catch (InterruptedException e) {
            System.out.println("Lifecycle test interrupted");
        }
    }

    private static void testThreadPriorityLifecycle() {
        System.out.println("Test 2: Thread priority lifecycle");
        int[] priorities = {Thread.MIN_PRIORITY, Thread.NORM_PRIORITY, Thread.MAX_PRIORITY};

        // Run threads sequentially for deterministic output
        for (int i = 0; i < 3; i++) {
            final int threadId = i;
            Thread priorityThread = new Thread(() -> {
                System.out.println("PriorityThread" + threadId + ": Priority " +
                                 Thread.currentThread().getPriority() + " running");
            });
            priorityThread.setPriority(priorities[i]);
            System.out.println("Set priority " + priorities[i] + " for thread " + i);
            priorityThread.start();
            try {
                priorityThread.join();
            } catch (InterruptedException e) {
                System.out.println("PriorityThread" + threadId + " interrupted");
            }
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
        final boolean[] ran = {false};
        Thread multiStartThread = new Thread(() -> {
            ran[0] = true;
        });

        multiStartThread.start();
        try {
            multiStartThread.join();
            System.out.println("Thread ran: " + ran[0]);
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
        System.out.println("Test 7: Exception in thread");

        Thread exceptionThread = new Thread(() -> {
            System.out.println("ExceptionThread: Testing exception handling");
            try {
                throw new RuntimeException("Test exception");
            } catch (RuntimeException e) {
                System.out.println("ExceptionThread: Caught exception: " + e.getClass().getSimpleName());
            }
        });

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
        }

        public int getExecutionCount() {
            return executionCount;
        }
    }
}
