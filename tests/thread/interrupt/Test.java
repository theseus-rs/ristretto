public class Test {
    public static void main(String[] args) {
        System.out.println("=== Interrupt Tests ===");

        testBasicInterrupt();
        testInterruptFlagChecking();
        testInterruptDuringWait();
        testInterruptDuringJoin();
        testMultipleInterrupts();
        testInterruptStatusPreservation();

        System.out.println("Interrupt tests completed");
    }

    private static void testBasicInterrupt() {
        System.out.println("Test 1: Basic interrupt");
        Thread interruptedThread = new Thread(() -> {
            try {
                System.out.println("InterruptedThread: Starting work");
                Thread.sleep(1000);
                System.out.println("InterruptedThread: Work completed normally");
            } catch (InterruptedException e) {
                System.out.println("InterruptedThread: Caught InterruptedException");
                System.out.println("InterruptedThread: Interrupt status after catch: " +
                                 Thread.currentThread().isInterrupted());
            }
        });

        interruptedThread.start();

        try {
            Thread.sleep(200);
            System.out.println("Main: Interrupting thread");
            interruptedThread.interrupt();
        } catch (InterruptedException e) {
            System.out.println("Main thread interrupted");
        }

        try {
            interruptedThread.join();
        } catch (InterruptedException e) {
            System.out.println("Join interrupted");
        }
    }

    private static void testInterruptFlagChecking() {
        System.out.println("Test 2: Interrupt flag checking");
        Thread flagThread = new Thread(() -> {
            System.out.println("FlagThread: Starting loop");
            int count = 0;
            while (!Thread.currentThread().isInterrupted() && count < 10) {
                System.out.println("FlagThread: Iteration " + count);
                count++;
                try {
                    Thread.sleep(100);
                } catch (InterruptedException e) {
                    System.out.println("FlagThread: Sleep interrupted, setting interrupt flag");
                    Thread.currentThread().interrupt(); // Restore interrupt status
                    break;
                }
            }
            System.out.println("FlagThread: Loop ended, interrupt status: " +
                             Thread.currentThread().isInterrupted());
        });

        flagThread.start();

        try {
            Thread.sleep(300);
            System.out.println("Main: Interrupting flag thread");
            flagThread.interrupt();
        } catch (InterruptedException e) {
            System.out.println("Main thread interrupted during flag test");
        }

        try {
            flagThread.join();
        } catch (InterruptedException e) {
            System.out.println("Flag thread join interrupted");
        }
    }

    private static void testInterruptDuringWait() {
        System.out.println("Test 3: Interrupt during wait");
        final Object waitLock = new Object();

        Thread waitingThread = new Thread(() -> {
            synchronized (waitLock) {
                try {
                    System.out.println("WaitingThread: Going to wait");
                    waitLock.wait();
                    System.out.println("WaitingThread: Wait completed normally");
                } catch (InterruptedException e) {
                    System.out.println("WaitingThread: Wait was interrupted");
                }
            }
        });

        waitingThread.start();

        try {
            Thread.sleep(200);
            System.out.println("Main: Interrupting waiting thread");
            waitingThread.interrupt();
        } catch (InterruptedException e) {
            System.out.println("Main thread interrupted during wait test");
        }

        try {
            waitingThread.join();
        } catch (InterruptedException e) {
            System.out.println("Waiting thread join interrupted");
        }
    }

    private static void testInterruptDuringJoin() {
        System.out.println("Test 4: Interrupt during join");
        Thread longRunningThread = new Thread(() -> {
            try {
                System.out.println("LongRunningThread: Starting long work");
                Thread.sleep(2000);
                System.out.println("LongRunningThread: Work completed");
            } catch (InterruptedException e) {
                System.out.println("LongRunningThread: Interrupted");
            }
        });

        Thread joiningThread = new Thread(() -> {
            try {
                System.out.println("JoiningThread: Waiting for long running thread");
                longRunningThread.join();
                System.out.println("JoiningThread: Join completed normally");
            } catch (InterruptedException e) {
                System.out.println("JoiningThread: Join was interrupted");
            }
        });

        longRunningThread.start();
        joiningThread.start();

        try {
            Thread.sleep(300);
            System.out.println("Main: Interrupting joining thread");
            joiningThread.interrupt();

            Thread.sleep(100);
            System.out.println("Main: Interrupting long running thread");
            longRunningThread.interrupt();
        } catch (InterruptedException e) {
            System.out.println("Main thread interrupted during join test");
        }

        try {
            joiningThread.join();
            longRunningThread.join();
        } catch (InterruptedException e) {
            System.out.println("Final join interrupted");
        }
    }

    private static void testMultipleInterrupts() {
        System.out.println("Test 5: Multiple interrupts");
        Thread multiInterruptThread = new Thread(() -> {
            int interruptCount = 0;
            for (int i = 0; i < 3; i++) {
                try {
                    System.out.println("MultiInterruptThread: Sleep attempt " + i);
                    Thread.sleep(200);
                } catch (InterruptedException e) {
                    interruptCount++;
                    System.out.println("MultiInterruptThread: Interrupt " + interruptCount);
                    if (interruptCount >= 2) {
                        System.out.println("MultiInterruptThread: Too many interrupts, exiting");
                        break;
                    }
                }
            }
        });

        multiInterruptThread.start();

        try {
            Thread.sleep(100);
            multiInterruptThread.interrupt();
            Thread.sleep(150);
            multiInterruptThread.interrupt();
        } catch (InterruptedException e) {
            System.out.println("Main thread interrupted during multi-interrupt test");
        }

        try {
            multiInterruptThread.join();
        } catch (InterruptedException e) {
            System.out.println("Multi-interrupt thread join interrupted");
        }
    }

    private static void testInterruptStatusPreservation() {
        System.out.println("Test 6: Interrupt status preservation");
        Thread statusThread = new Thread(() -> {
            System.out.println("StatusThread: Initial interrupt status: " +
                             Thread.currentThread().isInterrupted());

            // Set interrupt status
            Thread.currentThread().interrupt();
            System.out.println("StatusThread: After setting interrupt: " +
                             Thread.currentThread().isInterrupted());

            // Check and clear
            boolean wasInterrupted = Thread.interrupted();
            System.out.println("StatusThread: Thread.interrupted() returned: " + wasInterrupted);
            System.out.println("StatusThread: Status after Thread.interrupted(): " +
                             Thread.currentThread().isInterrupted());
        });

        statusThread.start();
        try {
            statusThread.join();
        } catch (InterruptedException e) {
            System.out.println("Status thread join interrupted");
        }
    }
}
