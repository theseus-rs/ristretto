import java.util.*;
import java.util.concurrent.*;

public class Test {
    public static void main(String[] args) {
        System.out.println("=== Volatile Tests ===");

        testBasicVolatileBehavior();
        testVolatileVsNonVolatileComparison();
        testVolatileInDoubleCheckedLocking();
        testVolatileWithReferenceTypes();

        System.out.println("Volatile tests completed");
    }

    private static void testBasicVolatileBehavior() {
        System.out.println("Test 1: Basic volatile behavior");
        VolatileTest volatileTest = new VolatileTest();

        Thread writer = new Thread(() -> {
            for (int i = 0; i < 5; i++) {
                volatileTest.setValue(i);
                try {
                    Thread.sleep(10);
                } catch (InterruptedException e) {
                    break;
                }
            }
            volatileTest.setDone(true);
        });

        writer.start();
        try {
            writer.join();
        } catch (InterruptedException e) {
            System.out.println("Volatile test interrupted");
        }

        System.out.println("Writer: Final value is " + volatileTest.getValue());
        System.out.println("Writer: Done flag is " + volatileTest.isDone());
    }

    private static void testVolatileVsNonVolatileComparison() {
        System.out.println("Test 2: Volatile vs non-volatile comparison");
        VolatileComparison comparison = new VolatileComparison();

        Thread modifier = new Thread(() -> {
            comparison.setVolatileValue(42);
            comparison.setNonVolatileValue(42);
            comparison.setVolatileFlag(true);
            comparison.setNonVolatileFlag(true);
        });

        modifier.start();
        try {
            modifier.join();
        } catch (InterruptedException e) {
            System.out.println("Volatile comparison test interrupted");
        }

        System.out.println("Observer: Volatile value: " + comparison.getVolatileValue());
        System.out.println("Observer: Non-volatile value: " + comparison.getNonVolatileValue());
        System.out.println("Observer: Volatile flag: " + comparison.getVolatileFlag());
        System.out.println("Observer: Non-volatile flag: " + comparison.getNonVolatileFlag());
    }

    private static void testVolatileInDoubleCheckedLocking() {
        System.out.println("Test 3: Volatile in double-checked locking");
        Thread[] singletonThreads = new Thread[5];
        ConcurrentLinkedQueue<String> messages = new ConcurrentLinkedQueue<>();

        for (int i = 0; i < 5; i++) {
            final int threadId = i;
            singletonThreads[i] = new Thread(() -> {
                SingletonWithVolatile instance = SingletonWithVolatile.getInstance();
                messages.add("SingletonThread" + threadId + ": Got instance");
            });
        }

        for (Thread thread : singletonThreads) {
            thread.start();
        }

        try {
            for (Thread thread : singletonThreads) {
                thread.join();
            }
        } catch (InterruptedException e) {
            System.out.println("Singleton test interrupted");
        }

        List<String> sorted = new ArrayList<>(messages);
        Collections.sort(sorted);
        for (String msg : sorted) {
            System.out.println(msg);
        }
        System.out.println("All threads got same instance: " + (SingletonWithVolatile.getInstance() != null));
    }

    private static void testVolatileWithReferenceTypes() {
        System.out.println("Test 4: Volatile reference types");
        VolatileReference refTest = new VolatileReference();

        Thread refWriter = new Thread(() -> {
            for (int i = 0; i < 3; i++) {
                DataObject obj = new DataObject("Object" + i, i * 100);
                refTest.setData(obj);
            }
        });

        refWriter.start();
        try {
            refWriter.join();
        } catch (InterruptedException e) {
            System.out.println("Volatile reference test interrupted");
        }

        DataObject finalData = refTest.getData();
        System.out.println("RefReader: Final data " + finalData);
    }

    static class VolatileTest {
        private volatile int value = 0;
        private volatile boolean done = false;

        public void setValue(int value) { this.value = value; }
        public int getValue() { return value; }
        public void setDone(boolean done) { this.done = done; }
        public boolean isDone() { return done; }
    }

    static class VolatileComparison {
        private volatile int volatileValue = 0;
        private int nonVolatileValue = 0;
        private volatile boolean volatileFlag = false;
        private boolean nonVolatileFlag = false;

        public void setVolatileValue(int value) { this.volatileValue = value; }
        public int getVolatileValue() { return volatileValue; }
        public void setNonVolatileValue(int value) { this.nonVolatileValue = value; }
        public int getNonVolatileValue() { return nonVolatileValue; }
        public void setVolatileFlag(boolean flag) { this.volatileFlag = flag; }
        public boolean getVolatileFlag() { return volatileFlag; }
        public void setNonVolatileFlag(boolean flag) { this.nonVolatileFlag = flag; }
        public boolean getNonVolatileFlag() { return nonVolatileFlag; }
    }

    static class SingletonWithVolatile {
        private static volatile SingletonWithVolatile instance = null;

        private SingletonWithVolatile() {
        }

        public static SingletonWithVolatile getInstance() {
            if (instance == null) {
                synchronized (SingletonWithVolatile.class) {
                    if (instance == null) {
                        instance = new SingletonWithVolatile();
                    }
                }
            }
            return instance;
        }
    }

    static class VolatileReference {
        private volatile DataObject data = null;

        public void setData(DataObject data) { this.data = data; }
        public DataObject getData() { return data; }
    }

    static class DataObject {
        private final String name;
        private final int value;

        public DataObject(String name, int value) {
            this.name = name;
            this.value = value;
        }

        @Override
        public String toString() {
            return "DataObject{name='" + name + "', value=" + value + "}";
        }
    }
}
