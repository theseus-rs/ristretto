import java.util.*;
import java.util.concurrent.*;

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
        ConcurrentLinkedQueue<String> messages = new ConcurrentLinkedQueue<>();

        for (int i = 0; i < 3; i++) {
            final int threadId = i;
            yieldThreads[i] = new Thread(() -> {
                for (int j = 0; j < 5; j++) {
                    messages.add("YieldThread" + threadId + ": Iteration " + j);
                    Thread.yield();
                }
                messages.add("YieldThread" + threadId + ": Completed");
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

        List<String> sorted = new ArrayList<>(messages);
        Collections.sort(sorted);
        for (String msg : sorted) {
            System.out.println(msg);
        }
    }

    private static void testYieldVsNoYieldComparison() {
        System.out.println("Test 2: Yield vs no yield comparison");

        // Test without yield
        ConcurrentLinkedQueue<String> noYieldMessages = new ConcurrentLinkedQueue<>();
        Thread noYield1 = new Thread(() -> {
            for (int i = 0; i < 3; i++) {
                noYieldMessages.add("NoYield1: " + i);
            }
        });
        Thread noYield2 = new Thread(() -> {
            for (int i = 0; i < 3; i++) {
                noYieldMessages.add("NoYield2: " + i);
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

        List<String> sortedNoYield = new ArrayList<>(noYieldMessages);
        Collections.sort(sortedNoYield);
        for (String msg : sortedNoYield) {
            System.out.println(msg);
        }

        // Test with yield
        ConcurrentLinkedQueue<String> yieldMessages = new ConcurrentLinkedQueue<>();
        Thread withYield1 = new Thread(() -> {
            for (int i = 0; i < 3; i++) {
                yieldMessages.add("WithYield1: " + i);
                Thread.yield();
            }
        });
        Thread withYield2 = new Thread(() -> {
            for (int i = 0; i < 3; i++) {
                yieldMessages.add("WithYield2: " + i);
                Thread.yield();
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

        List<String> sortedYield = new ArrayList<>(yieldMessages);
        Collections.sort(sortedYield);
        for (String msg : sortedYield) {
            System.out.println(msg);
        }
    }

    private static void testYieldInProducerConsumer() {
        System.out.println("Test 3: Yield in producer-consumer");

        // Use a simple sequential test to verify yield doesn't break things
        Thread producer = new Thread(() -> {
            for (int i = 0; i < 5; i++) {
                System.out.println("Producer: Item " + i);
                Thread.yield();
            }
        });

        producer.start();
        try {
            producer.join();
        } catch (InterruptedException e) {
            System.out.println("Producer interrupted");
        }
        System.out.println("Producer-consumer completed");
    }

    private static void testYieldWithThreadPriorities() {
        System.out.println("Test 4: Yield with thread priorities");
        ConcurrentLinkedQueue<String> messages = new ConcurrentLinkedQueue<>();

        Thread highPriority = new Thread(() -> {
            for (int i = 0; i < 3; i++) {
                messages.add("HighPriority: " + i);
                Thread.yield();
            }
        });

        Thread lowPriority = new Thread(() -> {
            for (int i = 0; i < 3; i++) {
                messages.add("LowPriority: " + i);
                Thread.yield();
            }
        });

        highPriority.setPriority(Thread.MAX_PRIORITY);
        lowPriority.setPriority(Thread.MIN_PRIORITY);

        highPriority.start();
        lowPriority.start();

        try {
            highPriority.join();
            lowPriority.join();
        } catch (InterruptedException e) {
            System.out.println("Priority yield test interrupted");
        }

        List<String> sorted = new ArrayList<>(messages);
        Collections.sort(sorted);
        for (String msg : sorted) {
            System.out.println(msg);
        }
    }

    private static void testCooperativeMultitasking() {
        System.out.println("Test 5: Cooperative multitasking");
        ConcurrentLinkedQueue<String> messages = new ConcurrentLinkedQueue<>();
        Thread[] taskThreads = new Thread[3];

        for (int i = 0; i < 3; i++) {
            final int taskId = i;
            taskThreads[i] = new Thread(() -> {
                for (int w = 0; w < 5; w++) {
                    messages.add("CooperativeTask" + taskId + ": Work unit " + w);
                    Thread.yield();
                }
                messages.add("CooperativeTask" + taskId + ": Completed all work");
            });
            taskThreads[i].start();
        }

        try {
            for (Thread thread : taskThreads) {
                thread.join();
            }
        } catch (InterruptedException e) {
            System.out.println("Cooperative multitasking test interrupted");
        }

        List<String> sorted = new ArrayList<>(messages);
        Collections.sort(sorted);
        for (String msg : sorted) {
            System.out.println(msg);
        }
    }
}
