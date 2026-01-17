public class Test {
    public static void main(String[] args) {
        System.out.println("=== Interrupt Tests ===");

        testInterruptStatusPreservation();
        testInterruptFlagChecking();

        System.out.println("Interrupt tests completed");
    }

    private static void testInterruptStatusPreservation() {
        System.out.println("Test 1: Interrupt status preservation");
        Thread statusThread = new Thread(() -> {
            System.out.println("StatusThread: Initial interrupt status: " +
                             Thread.currentThread().isInterrupted());
            Thread.currentThread().interrupt();
            System.out.println("StatusThread: After setting interrupt: " +
                             Thread.currentThread().isInterrupted());
        });

        statusThread.start();
        try {
            statusThread.join();
        } catch (InterruptedException e) {
            System.out.println("Status thread join interrupted");
        }
    }

    private static void testInterruptFlagChecking() {
        System.out.println("Test 2: Interrupt flag checking");
        final boolean[] done = {false};
        Thread flagThread = new Thread(() -> {
            System.out.println("FlagThread: Starting loop");
            int count = 0;
            while (!Thread.currentThread().isInterrupted() && count < 3) {
                System.out.println("FlagThread: Iteration " + count);
                count++;
            }
            System.out.println("FlagThread: Loop ended, count=" + count);
            done[0] = true;
        });

        flagThread.start();

        try {
            flagThread.join();
            System.out.println("Main: Thread completed, done=" + done[0]);
        } catch (InterruptedException e) {
            System.out.println("Flag thread join interrupted");
        }
    }
}
