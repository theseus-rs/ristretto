public class Test {
    public static void main(String[] args) {
        System.out.println("=== Concurrent Collections Tests ===");

        testThreadSafeVsUnsafeCollections();
        testSynchronizedCollections();
        testVectorOperations();
        testHashtableOperations();
        testStringBufferVsStringBuilder();
        testCustomThreadSafeCollection();

        System.out.println("Concurrent collections tests completed");
    }

    private static void testThreadSafeVsUnsafeCollections() {
        System.out.println("Test 1: Thread-safe vs unsafe collections");

        // Unsafe ArrayList test
        java.util.List<Integer> unsafeList = new java.util.ArrayList<>();
        Thread[] unsafeWriters = new Thread[3];

        for (int i = 0; i < 3; i++) {
            final int threadId = i;
            unsafeWriters[i] = new Thread(() -> {
                for (int j = 0; j < 100; j++) {
                    unsafeList.add(threadId * 100 + j);
                }
                System.out.println("UnsafeWriter" + threadId + " completed");
            });
        }

        for (Thread writer : unsafeWriters) {
            writer.start();
        }

        try {
            for (Thread writer : unsafeWriters) {
                writer.join();
            }
        } catch (InterruptedException e) {
            System.out.println("Unsafe writers interrupted");
        }

        System.out.println("Unsafe list final size: " + unsafeList.size() + " (expected: 300)");
    }

    private static void testSynchronizedCollections() {
        System.out.println("Test 2: Synchronized collections");
        java.util.List<Integer> syncList = java.util.Collections.synchronizedList(new java.util.ArrayList<>());
        Thread[] syncWriters = new Thread[3];

        for (int i = 0; i < 3; i++) {
            final int threadId = i;
            syncWriters[i] = new Thread(() -> {
                for (int j = 0; j < 100; j++) {
                    syncList.add(threadId * 100 + j);
                }
                System.out.println("SyncWriter" + threadId + " completed");
            });
        }

        for (Thread writer : syncWriters) {
            writer.start();
        }

        try {
            for (Thread writer : syncWriters) {
                writer.join();
            }
        } catch (InterruptedException e) {
            System.out.println("Sync writers interrupted");
        }

        System.out.println("Sync list final size: " + syncList.size());
    }

    private static void testVectorOperations() {
        System.out.println("Test 3: Vector operations");
        java.util.Vector<String> vector = new java.util.Vector<>();
        Thread[] vectorThreads = new Thread[2];

        vectorThreads[0] = new Thread(() -> {
            for (int i = 0; i < 50; i++) {
                vector.add("Writer: " + i);
                if (i % 10 == 0) {
                    System.out.println("VectorWriter: Added " + (i + 1) + " elements");
                }
            }
        });

        vectorThreads[1] = new Thread(() -> {
            try {
                Thread.sleep(100); // Let writer add some elements first
            } catch (InterruptedException e) {
                System.out.println("VectorReader interrupted during sleep");
            }

            for (int i = 0; i < 10; i++) {
                if (!vector.isEmpty()) {
                    String element = vector.get(0);
                    System.out.println("VectorReader: Read " + element);
                }
                try {
                    Thread.sleep(20);
                } catch (InterruptedException e) {
                    System.out.println("VectorReader interrupted");
                    break;
                }
            }
        });

        vectorThreads[0].start();
        vectorThreads[1].start();

        try {
            for (Thread thread : vectorThreads) {
                thread.join();
            }
        } catch (InterruptedException e) {
            System.out.println("Vector test interrupted");
        }

        System.out.println("Vector final size: " + vector.size());
    }

    private static void testHashtableOperations() {
        System.out.println("Test 4: Hashtable operations");
        java.util.Hashtable<String, Integer> hashtable = new java.util.Hashtable<>();
        Thread[] hashtableThreads = new Thread[3];

        for (int i = 0; i < 3; i++) {
            final int threadId = i;
            hashtableThreads[i] = new Thread(() -> {
                for (int j = 0; j < 20; j++) {
                    String key = "Thread" + threadId + "_Key" + j;
                    hashtable.put(key, threadId * 100 + j);
                }
                System.out.println("HashtableThread" + threadId + " completed");
            });
        }

        for (Thread thread : hashtableThreads) {
            thread.start();
        }

        try {
            for (Thread thread : hashtableThreads) {
                thread.join();
            }
        } catch (InterruptedException e) {
            System.out.println("Hashtable test interrupted");
        }

        System.out.println("Hashtable final size: " + hashtable.size());
    }

    private static void testStringBufferVsStringBuilder() {
        System.out.println("Test 5: StringBuffer vs StringBuilder");
        StringBuffer stringBuffer = new StringBuffer();
        StringBuilder stringBuilder = new StringBuilder();

        Thread[] bufferThreads = new Thread[2];
        bufferThreads[0] = new Thread(() -> {
            for (int i = 0; i < 1000; i++) {
                stringBuffer.append("A");
            }
            System.out.println("StringBuffer thread completed");
        });

        bufferThreads[1] = new Thread(() -> {
            for (int i = 0; i < 1000; i++) {
                stringBuffer.append("B");
            }
            System.out.println("StringBuffer thread 2 completed");
        });

        Thread[] builderThreads = new Thread[2];
        builderThreads[0] = new Thread(() -> {
            for (int i = 0; i < 1000; i++) {
                stringBuilder.append("X");
            }
            System.out.println("StringBuilder thread completed");
        });

        builderThreads[1] = new Thread(() -> {
            for (int i = 0; i < 1000; i++) {
                stringBuilder.append("Y");
            }
            System.out.println("StringBuilder thread 2 completed");
        });

        // Start buffer threads
        for (Thread thread : bufferThreads) {
            thread.start();
        }

        // Start builder threads
        for (Thread thread : builderThreads) {
            thread.start();
        }

        try {
            for (Thread thread : bufferThreads) {
                thread.join();
            }
            for (Thread thread : builderThreads) {
                thread.join();
            }
        } catch (InterruptedException e) {
            System.out.println("String buffer/builder test interrupted");
        }

        System.out.println("StringBuffer final length: " + stringBuffer.length() + " (expected: 2000)");
        System.out.println("StringBuilder final length: " + stringBuilder.length() + " (may vary due to race conditions)");
    }

    private static void testCustomThreadSafeCollection() {
        System.out.println("Test 6: Custom thread-safe collection");
        ThreadSafeCounter counter = new ThreadSafeCounter();
        Thread[] counterThreads = new Thread[5];

        for (int i = 0; i < 5; i++) {
            final int threadId = i;
            counterThreads[i] = new Thread(() -> {
                for (int j = 0; j < 200; j++) {
                    counter.increment();
                }
                System.out.println("CounterThread" + threadId + " completed, current count: " + counter.getValue());
            });
        }

        for (Thread thread : counterThreads) {
            thread.start();
        }

        try {
            for (Thread thread : counterThreads) {
                thread.join();
            }
        } catch (InterruptedException e) {
            System.out.println("Counter test interrupted");
        }

        System.out.println("Final counter value: " + counter.getValue() + " (expected: 1000)");
    }

    static class ThreadSafeCounter {
        private int count = 0;
        private final Object lock = new Object();

        public void increment() {
            synchronized (lock) {
                count++;
            }
        }

        public void decrement() {
            synchronized (lock) {
                count--;
            }
        }

        public int getValue() {
            synchronized (lock) {
                return count;
            }
        }
    }
}
