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

        // Run threads sequentially for deterministic output
        java.util.List<Integer> unsafeList = new java.util.ArrayList<>();

        for (int i = 0; i < 3; i++) {
            final int threadId = i;
            Thread writer = new Thread(() -> {
                for (int j = 0; j < 100; j++) {
                    unsafeList.add(threadId * 100 + j);
                }
                System.out.println("UnsafeWriter" + threadId + " completed");
            });
            writer.start();
            try {
                writer.join();
            } catch (InterruptedException e) {
                System.out.println("Writer join interrupted");
            }
        }

        System.out.println("Unsafe list final size: " + unsafeList.size() + " (expected: 300)");
    }

    private static void testSynchronizedCollections() {
        System.out.println("Test 2: Synchronized collections");
        java.util.List<Integer> syncList = java.util.Collections.synchronizedList(new java.util.ArrayList<>());

        // Run threads sequentially for deterministic output
        for (int i = 0; i < 3; i++) {
            final int threadId = i;
            Thread writer = new Thread(() -> {
                for (int j = 0; j < 100; j++) {
                    syncList.add(threadId * 100 + j);
                }
                System.out.println("SyncWriter" + threadId + " completed");
            });
            writer.start();
            try {
                writer.join();
            } catch (InterruptedException e) {
                System.out.println("Writer join interrupted");
            }
        }

        System.out.println("Sync list final size: " + syncList.size());
    }

    private static void testVectorOperations() {
        System.out.println("Test 3: Vector operations");
        java.util.Vector<String> vector = new java.util.Vector<>();

        // Run sequentially for deterministic output
        Thread writer = new Thread(() -> {
            for (int i = 0; i < 50; i++) {
                vector.add("Writer: " + i);
            }
            System.out.println("VectorWriter: Added 50 elements");
        });
        writer.start();
        try {
            writer.join();
        } catch (InterruptedException e) {
            System.out.println("Writer join interrupted");
        }

        Thread reader = new Thread(() -> {
            if (!vector.isEmpty()) {
                String element = vector.get(0);
                System.out.println("VectorReader: Read first element");
            }
        });
        reader.start();
        try {
            reader.join();
        } catch (InterruptedException e) {
            System.out.println("Reader join interrupted");
        }

        System.out.println("Vector final size: " + vector.size());
    }

    private static void testHashtableOperations() {
        System.out.println("Test 4: Hashtable operations");
        java.util.Hashtable<String, Integer> hashtable = new java.util.Hashtable<>();

        // Run threads sequentially for deterministic output
        for (int i = 0; i < 3; i++) {
            final int threadId = i;
            Thread thread = new Thread(() -> {
                for (int j = 0; j < 20; j++) {
                    String key = "Thread" + threadId + "_Key" + j;
                    hashtable.put(key, threadId * 100 + j);
                }
                System.out.println("HashtableThread" + threadId + " completed");
            });
            thread.start();
            try {
                thread.join();
            } catch (InterruptedException e) {
                System.out.println("Hashtable thread join interrupted");
            }
        }

        System.out.println("Hashtable final size: " + hashtable.size());
    }

    private static void testStringBufferVsStringBuilder() {
        System.out.println("Test 5: StringBuffer vs StringBuilder");
        StringBuffer stringBuffer = new StringBuffer();

        // Run sequentially for deterministic output
        Thread bufferThread1 = new Thread(() -> {
            for (int i = 0; i < 1000; i++) {
                stringBuffer.append("A");
            }
            System.out.println("StringBuffer thread 1 completed");
        });
        bufferThread1.start();
        try {
            bufferThread1.join();
        } catch (InterruptedException e) {
            System.out.println("Buffer thread join interrupted");
        }

        Thread bufferThread2 = new Thread(() -> {
            for (int i = 0; i < 1000; i++) {
                stringBuffer.append("B");
            }
            System.out.println("StringBuffer thread 2 completed");
        });
        bufferThread2.start();
        try {
            bufferThread2.join();
        } catch (InterruptedException e) {
            System.out.println("Buffer thread join interrupted");
        }

        System.out.println("StringBuffer final length: " + stringBuffer.length());
    }

    private static void testCustomThreadSafeCollection() {
        System.out.println("Test 6: Custom thread-safe collection");
        ThreadSafeCounter counter = new ThreadSafeCounter();

        // Run threads sequentially for deterministic output
        for (int i = 0; i < 5; i++) {
            final int threadId = i;
            Thread thread = new Thread(() -> {
                for (int j = 0; j < 200; j++) {
                    counter.increment();
                }
                System.out.println("CounterThread" + threadId + " completed, current count: " + counter.getValue());
            });
            thread.start();
            try {
                thread.join();
            } catch (InterruptedException e) {
                System.out.println("Counter thread join interrupted");
            }
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
