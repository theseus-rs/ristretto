import java.util.*;
import java.util.concurrent.*;

public class Test {
    public static void main(String[] args) {
        System.out.println("=== Virtual Thread Tests ===");

        testVirtualThreadSupportCheck();
        testPlatformThreadCharacteristics();
        testMultipleThreadCreation();

        System.out.println("Virtual thread tests completed");
    }

    private static void testVirtualThreadSupportCheck() {
        try {
            System.out.println("Test 1: Virtual thread support check");

            Class<?> threadClass = Thread.class;
            java.lang.reflect.Method ofVirtualMethod = null;

            try {
                ofVirtualMethod = threadClass.getMethod("ofVirtual");
                System.out.println("Virtual thread support: Available");

                Object builderObj = ofVirtualMethod.invoke(null);
                java.lang.reflect.Method startMethod = builderObj.getClass().getMethod("start", Runnable.class);

                Thread virtualThread = (Thread) startMethod.invoke(builderObj, (Runnable) () -> {
                    System.out.println("VirtualThread: Running");
                    System.out.println("VirtualThread: Is virtual: " + Thread.currentThread().isVirtual());
                });

                virtualThread.join();

            } catch (NoSuchMethodException e) {
                System.out.println("Virtual thread support: Not available (requires Java 19+)");
                runPlatformThreadTests();
            }

        } catch (Exception e) {
            System.out.println("Virtual thread test error: " + e.getClass().getSimpleName());
            runPlatformThreadTests();
        }
    }

    private static void testPlatformThreadCharacteristics() {
        System.out.println("Test 2: Platform thread characteristics");
        Thread platformThread = new Thread(() -> {
            System.out.println("PlatformThread: Name: " + Thread.currentThread().getName());
            System.out.println("PlatformThread: Priority: " + Thread.currentThread().getPriority());
            System.out.println("PlatformThread: ThreadGroup: " + Thread.currentThread().getThreadGroup().getName());

            try {
                java.lang.reflect.Method isVirtualMethod = Thread.class.getMethod("isVirtual");
                boolean isVirtual = (Boolean) isVirtualMethod.invoke(Thread.currentThread());
                System.out.println("PlatformThread: Is virtual: " + isVirtual);
            } catch (Exception e) {
                System.out.println("PlatformThread: Is virtual: false (method not available)");
            }
        });

        platformThread.start();
        try {
            platformThread.join();
        } catch (InterruptedException e) {
            System.out.println("Platform thread test interrupted");
        }
    }

    private static void testMultipleThreadCreation() {
        System.out.println("Test 3: Multiple thread creation");
        int threadCount = 10;
        Thread[] threads = new Thread[threadCount];
        ConcurrentLinkedQueue<String> messages = new ConcurrentLinkedQueue<>();

        for (int i = 0; i < threadCount; i++) {
            final int threadId = i;
            threads[i] = new Thread(() -> {
                messages.add("Thread" + threadId + ": Starting work");
                try {
                    Thread.sleep(10);
                } catch (InterruptedException e) {
                    messages.add("Thread" + threadId + " interrupted");
                }
                messages.add("Thread" + threadId + ": Work completed");
            });
        }

        for (Thread thread : threads) {
            thread.start();
        }

        try {
            for (Thread thread : threads) {
                thread.join();
            }
        } catch (InterruptedException e) {
            System.out.println("Multiple thread test interrupted");
        }

        List<String> sorted = new ArrayList<>(messages);
        Collections.sort(sorted);
        for (String msg : sorted) {
            System.out.println(msg);
        }
        System.out.println("All " + threadCount + " threads completed");
    }

    private static void runPlatformThreadTests() {
        System.out.println("Running enhanced platform thread tests:");

        int workerCount = 5;
        Thread[] workerPool = new Thread[workerCount];
        ConcurrentLinkedQueue<String> messages = new ConcurrentLinkedQueue<>();

        for (int i = 0; i < workerPool.length; i++) {
            final int workerId = i;
            workerPool[i] = new Thread(() -> {
                for (int j = 0; j < 3; j++) {
                    messages.add("Worker" + workerId + ": Task " + j);
                    try {
                        Thread.sleep(10);
                    } catch (InterruptedException e) {
                        messages.add("Worker" + workerId + " interrupted");
                        break;
                    }
                }
            });
        }

        for (Thread worker : workerPool) {
            worker.start();
        }

        try {
            for (Thread worker : workerPool) {
                worker.join();
            }
        } catch (InterruptedException e) {
            System.out.println("Worker pool test interrupted");
        }

        List<String> sorted = new ArrayList<>(messages);
        Collections.sort(sorted);
        for (String msg : sorted) {
            System.out.println(msg);
        }
    }
}
