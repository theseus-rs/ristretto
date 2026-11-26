/** Semantic oracle for the exact exported monitor bytecode; no timing claims. */
public final class CheckMonitorContinuations {
    private static int checks;
    private static void check(boolean value) {
        if (!value) throw new AssertionError("check " + checks);
        checks++;
    }
    public static void main(String[] args) throws Exception {
        Object lock = new Object();
        int[] array = new int[] {42};
        check(MonitorContinuations.balanced(lock) == 2);
        check(!Thread.holdsLock(lock));
        check(MonitorContinuations.nullEnter() == 7);
        check(MonitorContinuations.nullExit() == 8);
        check(MonitorContinuations.illegalExit(lock) == 9);
        check(MonitorContinuations.retain(lock, array) == array);
        check(!Thread.holdsLock(lock));
        MonitorContinuations.spin(array);
        check(!Thread.holdsLock(array));
        int[] empty = new int[0];
        try {
            MonitorContinuations.spin(empty);
            throw new AssertionError("missing array error");
        } catch (ArrayIndexOutOfBoundsException expected) {
            check(!Thread.holdsLock(empty));
        }
        final Throwable[] error = new Throwable[1];
        Thread contender = new Thread(() -> {
            try {
                if (MonitorContinuations.balanced(lock) != 2) throw new AssertionError();
            } catch (Throwable failure) { error[0] = failure; }
        });
        synchronized (lock) {
            contender.start();
            long deadline = System.nanoTime() + 5_000_000_000L;
            while (contender.getState() != Thread.State.BLOCKED && System.nanoTime() < deadline) {
                Thread.yield();
            }
            check(contender.getState() == Thread.State.BLOCKED);
            check(Thread.holdsLock(lock));
        }
        contender.join(5000);
        check(!contender.isAlive());
        check(error[0] == null);
        check(!Thread.holdsLock(lock));
        System.out.println("monitor checks passed: " + checks);
    }
}
