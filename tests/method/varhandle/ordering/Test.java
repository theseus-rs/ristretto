import java.lang.invoke.MethodHandle;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.VarHandle;
import java.util.concurrent.CountDownLatch;
import java.util.concurrent.CyclicBarrier;
import java.util.concurrent.TimeUnit;

public class Test {
    int count;
    volatile int ready;

    public static void main(String[] args) throws Exception {
        VarHandle field = MethodHandles.lookup().findVarHandle(Test.class, "count", int.class);
        VarHandle element = MethodHandles.arrayElementVarHandle(int[].class);
        contention(field, element);
        volatileStoreLoad(element);
        publication(element, false);
        publication(element, true);
        adjacentElements();
        opaqueOrdering();
        aliasedBitwiseUpdates();

        VarHandle ready = MethodHandles.lookup().findVarHandle(Test.class, "ready", int.class);
        fieldPublication(message -> message.ready = 1, message -> (int) ready.getVolatile(message));
        fieldPublication(message -> ready.setVolatile(message, 1), message -> message.ready);
        // The expected value -1 never matches. Its acquire witness still observes the publication.
        fieldPublication(message -> ready.getAndSetRelease(message, 1),
                message -> (int) ready.compareAndExchangeAcquire(message, -1, 2));
        fencedPublication(ready, VarHandle::storeStoreFence, VarHandle::loadLoadFence);
        fencedPublication(ready, VarHandle::releaseFence, VarHandle::acquireFence);
        fencedPublication(ready, VarHandle::fullFence, VarHandle::fullFence);
        System.out.println("Atomic updates, publication and fences passed");
    }

    static void opaqueOrdering() throws InterruptedException {
        VarHandle word = MethodHandles.arrayElementVarHandle(long[].class);
        long first = 0x55555555aaaaaaaaL, second = 0xaaaaaaaa55555555L;
        long[] pattern = {first}, sequence = {0};
        CyclicBarrier round = new CyclicBarrier(2);
        runWorkers(2, worker -> {
            long previous = 0;
            for (int batch = 0; batch < 8; batch++) {
                round.await(30, TimeUnit.SECONDS);
                for (int i = 0; i < 64; i++) {
                    if (worker == 0) {
                        word.setOpaque(pattern, 0, (i & 1) == 0 ? first : second);
                        word.setOpaque(sequence, 0, 1L + batch * 64 + i);
                    } else {
                        long sampled = (long) word.getOpaque(pattern, 0);
                        long observed = (long) word.getOpaque(sequence, 0);
                        if (sampled != first && sampled != second) throw new AssertionError("torn opaque long");
                        if (observed < previous || observed > 512) throw new AssertionError("incoherent opaque read");
                        previous = observed;
                    }
                }
            }
        });
        if (sequence[0] != 512 || pattern[0] != second) throw new AssertionError("writer did not finish");
    }

    static void aliasedBitwiseUpdates() throws InterruptedException {
        VarHandle left = MethodHandles.arrayElementVarHandle(int[].class);
        VarHandle right = MethodHandles.arrayElementVarHandle(int[].class);
        int[] words = new int[32];
        runWorkers(4, worker -> {
            VarHandle target = (worker & 1) == 0 ? left : right;
            int mask = 1 << worker;
            for (int i = 0; i < words.length; i++) {
                int old = (int) target.getAndBitwiseOr(words, i, mask);
                if ((old & mask) != 0) throw new AssertionError("bit already set");
            }
        });
        for (int word : words) if (word != 15) throw new AssertionError("lost disjoint bit");
    }

    static void adjacentElements() throws InterruptedException {
        VarHandle bytes = MethodHandles.arrayElementVarHandle(byte[].class);
        VarHandle shorts = MethodHandles.arrayElementVarHandle(short[].class);
        byte[] b = {-7, 100, 17, -3};
        short[] s = {-6, 32000, -1800, -2};
        CountDownLatch start = new CountDownLatch(1);
        Thread[] workers = new Thread[2];
        for (int i = 0; i < workers.length; i++) {
            final int index = i;
            workers[i] = new Thread(() -> {
                try {
                    start.await();
                } catch (InterruptedException e) {
                    throw new AssertionError(e);
                }
                for (int round = 0; round < 96; round++) {
                    bytes.getAndAdd(b, index, (byte) 1);
                    shorts.getAndAdd(s, index, (short) 13);
                }
            });
            workers[i].start();
        }
        start.countDown();
        for (Thread worker : workers) worker.join();
        if (b[0] != 89 || b[1] != (byte) (100 + 96) || b[2] != 17 || b[3] != -3
                || s[0] != -6 + 96 * 13 || s[1] != (short) (32000 + 96 * 13) || s[2] != -1800 || s[3] != -2) {
            throw new AssertionError("adjacent subword updates");
        }
    }

    interface Publish { void set(Test message); }
    interface Observe { int get(Test message); }

    static void fencedPublication(VarHandle ready, Runnable storeFence, Runnable loadFence) throws InterruptedException {
        fieldPublication(message -> {
            storeFence.run();
            ready.setOpaque(message, 1);
        }, message -> {
            int observed = (int) ready.getOpaque(message);
            loadFence.run();
            return observed;
        });
    }

    static void fieldPublication(Publish publish, Observe observe) throws InterruptedException {
        Test[] messages = new Test[32];
        for (int i = 0; i < messages.length; i++) messages[i] = new Test();
        Thread writer = new Thread(() -> {
            for (int i = 0; i < messages.length; i++) {
                messages[i].count = i + 1;
                publish.set(messages[i]);
            }
        });
        writer.start();
        long start = System.nanoTime();
        try {
            for (int i = 0; i < messages.length; i++) {
                while (observe.get(messages[i]) != 1) {
                    if (System.nanoTime() - start > 30_000_000_000L) throw new AssertionError("field publication timed out");
                    Thread.yield();
                }
                if (messages[i].count != i + 1) throw new AssertionError("published stale payload " + i);
            }
        } finally {
            writer.join();
        }
        for (int i = 0; i < messages.length; i++) {
            if (messages[i].ready != 1 || messages[i].count != i + 1) throw new AssertionError("incomplete writer");
        }
    }

    static void contention(VarHandle field, VarHandle element) throws InterruptedException {
        Test receiver = new Test();
        MethodHandle boundAdd = field.toMethodHandle(VarHandle.AccessMode.GET_AND_ADD);
        int[] array = {0, 23};
        int rounds = 64;
        int workers = 4;
        int[][] fieldWitnesses = new int[workers][rounds];
        int[][] arrayWitnesses = new int[workers][rounds];
        runWorkers(workers, worker -> {
            for (int j = 0; j < rounds; j++) {
                fieldWitnesses[worker][j] = worker % 2 == 0
                        ? (int) field.getAndAdd(receiver, 1)
                        : (int) boundAdd.invokeExact(receiver, 1);
                arrayWitnesses[worker][j] = (int) element.getAndAdd(array, 0, 1);
            }
        });
        int total = workers * rounds;
        if (receiver.count != total || array[0] != total || array[1] != 23) throw new AssertionError("lost update");
        checkWitnesses(fieldWitnesses, total);
        checkWitnesses(arrayWitnesses, total);
    }

    static void volatileStoreLoad(VarHandle element) throws InterruptedException {
        int rounds = 64;
        int[][] flags = new int[2][rounds];
        int[][] seen = new int[2][rounds];
        CyclicBarrier startRound = new CyclicBarrier(2);
        runWorkers(2, worker -> {
            for (int round = 0; round < rounds; round++) {
                startRound.await(30, TimeUnit.SECONDS);
                element.setVolatile(flags[worker], round, 1);
                seen[worker][round] = (int) element.getVolatile(flags[1 - worker], round);
            }
        });
        for (int round = 0; round < rounds; round++) {
            int left = seen[0][round], right = seen[1][round];
            if ((left != 0 && left != 1) || (right != 0 && right != 1) || (left == 0 && right == 0)) {
                throw new AssertionError("volatile store/load round " + round + " observed " + left + ", " + right);
            }
        }
    }

    interface Worker { void run(int index) throws Throwable; }

    static void runWorkers(int count, Worker action) throws InterruptedException {
        CountDownLatch start = new CountDownLatch(1);
        Thread[] workers = new Thread[count];
        Throwable[] failures = new Throwable[count];
        for (int i = 0; i < count; i++) {
            final int index = i;
            workers[i] = new Thread(() -> {
                try {
                    start.await();
                    action.run(index);
                } catch (Throwable failure) {
                    failures[index] = failure;
                }
            });
            workers[i].start();
        }
        start.countDown();
        for (Thread worker : workers) worker.join();
        for (Throwable failure : failures) {
            if (failure != null) throw new AssertionError("worker failed", failure);
        }
    }

    static void checkWitnesses(int[][] witnesses, int total) {
        boolean[] seen = new boolean[total];
        for (int[] worker : witnesses) {
            for (int previous : worker) {
                if (previous < 0 || previous >= total || seen[previous]) throw new AssertionError("invalid witness " + previous);
                seen[previous] = true;
            }
        }
    }

    static void publication(VarHandle element, boolean useVolatile) throws InterruptedException {
        int[] payload = new int[32];
        int[] ready = new int[payload.length];
        Thread writer = new Thread(() -> {
            for (int i = 0; i < payload.length; i++) {
                payload[i] = i + 1;
                if (useVolatile) element.setVolatile(ready, i, 1);
                else element.setRelease(ready, i, 1);
            }
        });
        writer.start();
        // Read the payload before joining. Only the flag should publish the writer's stores.
        long start = System.nanoTime();
        for (int i = 0; i < ready.length; i++) {
            while ((useVolatile ? (int) element.getVolatile(ready, i) : (int) element.getAcquire(ready, i)) == 0) {
                if (System.nanoTime() - start > 30_000_000_000L) throw new AssertionError("publication timed out");
                Thread.yield();
            }
            if (payload[i] != i + 1) throw new AssertionError("observed flag before payload");
        }
        writer.join();
    }
}
