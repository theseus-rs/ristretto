public class Test {
    public static void main(String[] args) {
        System.out.println("=== Basic Threading Tests ===");

        testBasicThreadCreation();
        testThreadStates();
        testThreadPriority();

        System.out.println("Basic threading tests completed");
    }

    private static void testBasicThreadCreation() {
        System.out.println("Test 1: Basic thread creation");
        Thread thread1 = new Thread(() -> {
            System.out.println("Thread running: " + Thread.currentThread().getName());
        });
        thread1.setName("BasicThread");
        thread1.start();

        try {
            thread1.join();
        } catch (InterruptedException e) {
            System.out.println("Interrupted: " + e.getMessage());
        }
    }

    private static void testThreadStates() {
        System.out.println("Test 2: Thread states");
        Thread stateThread = new Thread(() -> {
            try {
                Thread.sleep(100);
            } catch (InterruptedException e) {
                System.out.println("Sleep interrupted");
            }
        });

        System.out.println("State before start: " + stateThread.getState());
        stateThread.start();
        System.out.println("State after start: " + stateThread.getState());

        try {
            stateThread.join();
            System.out.println("State after completion: " + stateThread.getState());
        } catch (InterruptedException e) {
            System.out.println("Join interrupted: " + e.getMessage());
        }
    }

    private static void testThreadPriority() {
        System.out.println("Test 3: Thread priority");
        Thread lowPriority = new Thread(() -> {
            System.out.println("Low priority thread: " + Thread.currentThread().getPriority());
        });
        Thread highPriority = new Thread(() -> {
            System.out.println("High priority thread: " + Thread.currentThread().getPriority());
        });

        lowPriority.setPriority(Thread.MIN_PRIORITY);
        highPriority.setPriority(Thread.MAX_PRIORITY);

        lowPriority.start();
        highPriority.start();

        try {
            lowPriority.join();
            highPriority.join();
        } catch (InterruptedException e) {
            System.out.println("Priority test interrupted: " + e.getMessage());
        }
    }
}
