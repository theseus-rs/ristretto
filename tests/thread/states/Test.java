public class Test {
    public static void main(String[] args) throws Exception {
        System.out.println("=== Thread State Tests ===");

        testWaitingState();
        testTimedWaitingViaWait();
        testTimedWaitingViaSleep();
        testBlockedState();

        System.out.println("Thread state tests completed");
    }

    private static void testWaitingState() throws Exception {
        System.out.println("Test 1: WAITING state via Object.wait()");
        final Object lock = new Object();
        final boolean[] ready = {false};

        Thread waiter = new Thread(() -> {
            synchronized (lock) {
                ready[0] = true;
                lock.notifyAll();
                try {
                    lock.wait();
                } catch (InterruptedException e) {
                    // expected
                }
            }
        });

        synchronized (lock) {
            waiter.start();
            while (!ready[0]) {
                lock.wait();
            }
        }
        // waiter now owns lock and is in wait()
        Thread.sleep(100);
        System.out.println("Waiter state: " + waiter.getState());

        synchronized (lock) {
            lock.notifyAll();
        }
        waiter.join();
        System.out.println("Waiter final state: " + waiter.getState());
    }

    private static void testTimedWaitingViaWait() throws Exception {
        System.out.println("Test 2: TIMED_WAITING state via Object.wait(millis)");
        final Object lock = new Object();
        final boolean[] ready = {false};

        Thread timedWaiter = new Thread(() -> {
            synchronized (lock) {
                ready[0] = true;
                lock.notifyAll();
                try {
                    lock.wait(10000);
                } catch (InterruptedException e) {
                    // expected
                }
            }
        });

        synchronized (lock) {
            timedWaiter.start();
            while (!ready[0]) {
                lock.wait();
            }
        }
        Thread.sleep(100);
        System.out.println("TimedWaiter state: " + timedWaiter.getState());

        synchronized (lock) {
            lock.notifyAll();
        }
        timedWaiter.join();
        System.out.println("TimedWaiter final state: " + timedWaiter.getState());
    }

    private static void testTimedWaitingViaSleep() throws Exception {
        System.out.println("Test 3: TIMED_WAITING state via Thread.sleep()");
        final Object ready = new Object();
        final boolean[] started = {false};

        Thread sleeper = new Thread(() -> {
            synchronized (ready) {
                started[0] = true;
                ready.notifyAll();
            }
            try {
                Thread.sleep(10000);
            } catch (InterruptedException e) {
                // expected
            }
        });

        synchronized (ready) {
            sleeper.start();
            while (!started[0]) {
                ready.wait();
            }
        }
        Thread.sleep(100);
        System.out.println("Sleeper state: " + sleeper.getState());

        sleeper.interrupt();
        sleeper.join();
        System.out.println("Sleeper final state: " + sleeper.getState());
    }

    private static void testBlockedState() throws Exception {
        System.out.println("Test 4: BLOCKED state via synchronized");
        final Object lock = new Object();
        final boolean[] ready = {false};

        synchronized (lock) {
            Thread blocked = new Thread(() -> {
                synchronized (lock) {
                    // Will block until main thread releases
                }
            });

            blocked.start();
            Thread.sleep(100);
            System.out.println("Blocked thread state: " + blocked.getState());
            // Release the lock so blocked thread can proceed
            lock.notifyAll();
            // Need to release lock for blocked to enter
        }
        // lock released, blocked thread can now enter
        Thread.sleep(50);
        System.out.println("Test 4 done");
    }
}
