import java.util.*;
import java.util.concurrent.*;

public class Test {
    public static void main(String[] args) {
        System.out.println("=== Exception Tests ===");

        testUncaughtExceptionHandler();
        testDefaultUncaughtExceptionHandler();
        testExceptionInSynchronizedBlock();
        testExceptionDuringWait();
        testMultipleExceptionsInThreadGroup();
        testExceptionPropagationAndTermination();
        testErrorVsExceptionHandling();

        System.out.println("Exception tests completed");
    }

    private static void testUncaughtExceptionHandler() {
        System.out.println("Test 1: Uncaught exception handler");
        Thread.UncaughtExceptionHandler customHandler = new Thread.UncaughtExceptionHandler() {
            public void uncaughtException(Thread t, Throwable e) {
                System.out.println("CustomHandler: Caught " + e.getClass().getSimpleName() +
                                 " in thread " + t.getName() + ": " + e.getMessage());
            }
        };

        Thread exceptionThread = new Thread(() -> {
            System.out.println("ExceptionThread: About to throw RuntimeException");
            throw new RuntimeException("Test runtime exception");
        }, "ExceptionThread");

        exceptionThread.setUncaughtExceptionHandler(customHandler);
        exceptionThread.start();

        try {
            exceptionThread.join();
            System.out.println("ExceptionThread final state: " + exceptionThread.getState());
        } catch (InterruptedException e) {
            System.out.println("Exception thread join interrupted");
        }
    }

    private static void testDefaultUncaughtExceptionHandler() {
        System.out.println("Test 2: Default uncaught exception handler");
        Thread.UncaughtExceptionHandler originalDefault = Thread.getDefaultUncaughtExceptionHandler();

        Thread.UncaughtExceptionHandler defaultHandler = new Thread.UncaughtExceptionHandler() {
            public void uncaughtException(Thread t, Throwable e) {
                System.out.println("DefaultHandler: Thread " + t.getName() +
                                 " threw " + e.getClass().getSimpleName());
            }
        };

        Thread.setDefaultUncaughtExceptionHandler(defaultHandler);

        Thread defaultExceptionThread = new Thread(() -> {
            System.out.println("DefaultExceptionThread: Throwing exception");
            throw new IllegalStateException("Default handler test");
        }, "DefaultExceptionThread");

        defaultExceptionThread.start();

        try {
            defaultExceptionThread.join();
        } catch (InterruptedException e) {
            System.out.println("Default exception thread join interrupted");
        }

        // Restore original handler
        Thread.setDefaultUncaughtExceptionHandler(originalDefault);
    }

    private static void testExceptionInSynchronizedBlock() {
        System.out.println("Test 3: Exception in synchronized block");
        final Object syncLock = new Object();

        Thread syncExceptionThread = new Thread(() -> {
            try {
                synchronized (syncLock) {
                    System.out.println("SyncExceptionThread: In synchronized block");
                    throw new RuntimeException("Exception in synchronized block");
                }
            } catch (RuntimeException e) {
                System.out.println("SyncExceptionThread: Caught exception: " + e.getMessage());
                System.out.println("SyncExceptionThread: Lock should be released automatically");
            }
        });

        syncExceptionThread.start();

        try {
            syncExceptionThread.join();

            // Test that lock was released
            synchronized (syncLock) {
                System.out.println("Main: Successfully acquired lock after exception");
            }
        } catch (InterruptedException e) {
            System.out.println("Sync exception thread join interrupted");
        }
    }

    private static void testExceptionDuringWait() {
        System.out.println("Test 4: Exception during wait");
        final Object waitLock = new Object();

        Thread waitExceptionThread = new Thread(() -> {
            synchronized (waitLock) {
                try {
                    System.out.println("WaitExceptionThread: Going to wait");
                    waitLock.wait(500);
                    System.out.println("WaitExceptionThread: Wait completed");
                } catch (InterruptedException e) {
                    System.out.println("WaitExceptionThread: Wait interrupted");
                }
            }
        });

        waitExceptionThread.start();

        try {
            Thread.sleep(50);
            waitExceptionThread.interrupt();
            waitExceptionThread.join();
        } catch (InterruptedException e) {
            System.out.println("Wait exception test interrupted");
        }
    }

    private static void testMultipleExceptionsInThreadGroup() {
        System.out.println("Test 5: Multiple exceptions in thread group");
        ConcurrentLinkedQueue<String> messages = new ConcurrentLinkedQueue<>();
        ThreadGroup exceptionGroup = new ThreadGroup("ExceptionGroup") {
            public void uncaughtException(Thread t, Throwable e) {
                messages.add("ExceptionGroup: Thread " + t.getName() +
                                 " had uncaught " + e.getClass().getSimpleName());
            }
        };

        Thread[] exceptionThreads = new Thread[3];
        String[] exceptionTypes = {"RuntimeException", "IllegalArgumentException", "NullPointerException"};

        for (int i = 0; i < 3; i++) {
            final int threadId = i;
            exceptionThreads[i] = new Thread(exceptionGroup, () -> {
                messages.add("GroupExceptionThread" + threadId + ": About to throw " + exceptionTypes[threadId]);
                switch (threadId) {
                    case 0:
                        throw new RuntimeException("Group runtime exception");
                    case 1:
                        throw new IllegalArgumentException("Group illegal argument");
                    case 2:
                        throw new NullPointerException("Group null pointer");
                }
            }, "GroupExceptionThread" + i);
        }

        for (Thread thread : exceptionThreads) {
            thread.start();
        }

        try {
            for (Thread thread : exceptionThreads) {
                thread.join();
            }
        } catch (InterruptedException e) {
            System.out.println("Group exception test interrupted");
        }

        List<String> sorted = new ArrayList<>(messages);
        Collections.sort(sorted);
        for (String msg : sorted) {
            System.out.println(msg);
        }
    }

    private static void testExceptionPropagationAndTermination() {
        System.out.println("Test 6: Exception propagation and thread termination");
        Thread terminationThread = new Thread(() -> {
            try {
                System.out.println("TerminationThread: Starting work");
                for (int i = 0; i < 5; i++) {
                    if (i == 3) {
                        throw new RuntimeException("Termination at iteration " + i);
                    }
                    System.out.println("TerminationThread: Iteration " + i);
                    Thread.sleep(50);
                }
                System.out.println("TerminationThread: Work completed normally");
            } catch (RuntimeException e) {
                System.out.println("TerminationThread: Caught RuntimeException: " + e.getMessage());
                System.out.println("TerminationThread: Thread will terminate");
                throw e;
            } catch (InterruptedException e) {
                System.out.println("TerminationThread: Interrupted");
            }
        });

        terminationThread.setUncaughtExceptionHandler((t, e) -> {
            System.out.println("TerminationHandler: Thread " + t.getName() + " terminated due to " + e.getClass().getSimpleName());
        });

        terminationThread.start();

        try {
            terminationThread.join();
            System.out.println("TerminationThread final state: " + terminationThread.getState());
            System.out.println("TerminationThread is alive: " + terminationThread.isAlive());
        } catch (InterruptedException e) {
            System.out.println("Termination test interrupted");
        }
    }

    private static void testErrorVsExceptionHandling() {
        System.out.println("Test 7: Error vs Exception handling");
        Thread errorThread = new Thread(() -> {
            try {
                System.out.println("ErrorThread: About to cause StackOverflowError");
                recursiveMethod(0);
            } catch (StackOverflowError e) {
                System.out.println("ErrorThread: Caught StackOverflowError");
            } catch (Throwable t) {
                System.out.println("ErrorThread: Caught other Throwable: " + t.getClass().getSimpleName());
            }
        });

        errorThread.start();

        try {
            errorThread.join();
        } catch (InterruptedException e) {
            System.out.println("Error thread join interrupted");
        }
    }

    private static void recursiveMethod(int depth) {
        recursiveMethod(depth + 1);
    }
}
