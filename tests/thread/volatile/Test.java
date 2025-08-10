public class Test {
    public static void main(String[] args) {
        System.out.println("=== Volatile Tests ===");

        testBasicVolatileBehavior();
        testVolatileVsNonVolatileComparison();
        testVolatileInDoubleCheckedLocking();
        testVolatileArrayBehavior();
        testVolatileWithReferenceTypes();

        System.out.println("Volatile tests completed");
    }

    private static void testBasicVolatileBehavior() {
        System.out.println("Test 1: Basic volatile behavior");
        VolatileTest volatileTest = new VolatileTest();

        Thread writer = new Thread(() -> {
            for (int i = 0; i < 5; i++) {
                volatileTest.setValue(i);
                System.out.println("Writer: Set value to " + i);
                try {
                    Thread.sleep(100);
                } catch (InterruptedException e) {
                    System.out.println("Writer interrupted");
                    break;
                }
            }
            volatileTest.setDone(true);
            System.out.println("Writer: Set done flag");
        });

        Thread reader = new Thread(() -> {
            int lastValue = -1;
            while (!volatileTest.isDone()) {
                int currentValue = volatileTest.getValue();
                if (currentValue != lastValue) {
                    System.out.println("Reader: Saw new value " + currentValue);
                    lastValue = currentValue;
                }
                try {
                    Thread.sleep(50);
                } catch (InterruptedException e) {
                    System.out.println("Reader interrupted");
                    break;
                }
            }
            System.out.println("Reader: Done flag detected, final value: " + volatileTest.getValue());
        });

        writer.start();
        reader.start();

        try {
            writer.join();
            reader.join();
        } catch (InterruptedException e) {
            System.out.println("Volatile test interrupted");
        }
    }

    private static void testVolatileVsNonVolatileComparison() {
        System.out.println("Test 2: Volatile vs non-volatile comparison");
        VolatileComparison comparison = new VolatileComparison();

        Thread modifier = new Thread(() -> {
            try {
                Thread.sleep(200);
                System.out.println("Modifier: Changing values");
                comparison.setVolatileValue(42);
                comparison.setNonVolatileValue(42);
                comparison.setVolatileFlag(true);
                comparison.setNonVolatileFlag(true);
            } catch (InterruptedException e) {
                System.out.println("Modifier interrupted");
            }
        });

        Thread observer = new Thread(() -> {
            while (!comparison.getVolatileFlag()) {
                // Busy wait on volatile flag
            }
            System.out.println("Observer: Volatile flag detected");
            System.out.println("Observer: Volatile value: " + comparison.getVolatileValue());
            System.out.println("Observer: Non-volatile value: " + comparison.getNonVolatileValue());
            System.out.println("Observer: Non-volatile flag: " + comparison.getNonVolatileFlag());
        });

        modifier.start();
        observer.start();

        try {
            modifier.join();
            observer.join();
        } catch (InterruptedException e) {
            System.out.println("Volatile comparison test interrupted");
        }
    }

    private static void testVolatileInDoubleCheckedLocking() {
        System.out.println("Test 3: Volatile in double-checked locking");
        SingletonWithVolatile singleton = SingletonWithVolatile.getInstance();
        Thread[] singletonThreads = new Thread[5];

        for (int i = 0; i < 5; i++) {
            final int threadId = i;
            singletonThreads[i] = new Thread(() -> {
                SingletonWithVolatile instance = SingletonWithVolatile.getInstance();
                System.out.println("SingletonThread" + threadId + ": Got instance " + instance.getId());
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
    }

    private static void testVolatileArrayBehavior() {
        System.out.println("Test 4: Volatile array behavior");
        VolatileArrayTest arrayTest = new VolatileArrayTest();

        Thread arrayWriter = new Thread(() -> {
            for (int i = 0; i < 5; i++) {
                arrayTest.setFlag(true);
                arrayTest.setIndex(i);
                arrayTest.updateArray(i, i * 10);
                System.out.println("ArrayWriter: Updated index " + i + " with value " + (i * 10));
                arrayTest.setFlag(false);
                try {
                    Thread.sleep(100);
                } catch (InterruptedException e) {
                    System.out.println("ArrayWriter interrupted");
                    break;
                }
            }
        });

        Thread arrayReader = new Thread(() -> {
            for (int i = 0; i < 5; i++) {
                while (!arrayTest.getFlag()) {
                    Thread.yield();
                }
                int index = arrayTest.getIndex();
                int value = arrayTest.getArrayValue(index);
                System.out.println("ArrayReader: Read index " + index + " with value " + value);
                while (arrayTest.getFlag()) {
                    Thread.yield();
                }
            }
        });

        arrayWriter.start();
        arrayReader.start();

        try {
            arrayWriter.join();
            arrayReader.join();
        } catch (InterruptedException e) {
            System.out.println("Volatile array test interrupted");
        }
    }

    private static void testVolatileWithReferenceTypes() {
        System.out.println("Test 5: Volatile reference types");
        VolatileReference refTest = new VolatileReference();

        Thread refWriter = new Thread(() -> {
            for (int i = 0; i < 3; i++) {
                DataObject obj = new DataObject("Object" + i, i * 100);
                refTest.setData(obj);
                System.out.println("RefWriter: Set data to " + obj);
                try {
                    Thread.sleep(150);
                } catch (InterruptedException e) {
                    System.out.println("RefWriter interrupted");
                    break;
                }
            }
        });

        Thread refReader = new Thread(() -> {
            DataObject lastSeen = null;
            for (int i = 0; i < 10; i++) {
                DataObject current = refTest.getData();
                if (current != lastSeen) {
                    System.out.println("RefReader: Saw new data " + current);
                    lastSeen = current;
                }
                try {
                    Thread.sleep(50);
                } catch (InterruptedException e) {
                    System.out.println("RefReader interrupted");
                    break;
                }
            }
        });

        refWriter.start();
        refReader.start();

        try {
            refWriter.join();
            refReader.join();
        } catch (InterruptedException e) {
            System.out.println("Volatile reference test interrupted");
        }
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
        private final int id;

        private SingletonWithVolatile() {
            this.id = (int) (System.currentTimeMillis() % 1000);
            System.out.println("Singleton: Created instance with id " + id);
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

        public int getId() { return id; }
    }

    static class VolatileArrayTest {
        private volatile boolean flag = false;
        private volatile int index = 0;
        private final int[] array = new int[10];

        public void setFlag(boolean flag) { this.flag = flag; }
        public boolean getFlag() { return flag; }
        public void setIndex(int index) { this.index = index; }
        public int getIndex() { return index; }
        public void updateArray(int idx, int value) { array[idx] = value; }
        public int getArrayValue(int idx) { return array[idx]; }
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
