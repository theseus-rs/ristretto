public class Test {
    private static int counter = 0;
    private static final Object lock1 = new Object();
    private static final Object lock2 = new Object();

    public static void main(String[] args) {
        System.out.println("=== Synchronization Tests ===");

        testBasicSynchronization();
        testSynchronizedMethods();
        testDeadlockPrevention();
        testReentrantSynchronization();
        testStaticSynchronization();
        testSynchronizationWithInheritance();

        System.out.println("Synchronization tests completed");
    }

    private static void testBasicSynchronization() {
        System.out.println("Test 1: Basic synchronization");
        counter = 0;
        Thread[] incrementers = new Thread[5];

        for (int i = 0; i < 5; i++) {
            final int threadId = i;
            incrementers[i] = new Thread(() -> {
                for (int j = 0; j < 100; j++) {
                    synchronized (Test.class) {
                        counter++;
                    }
                }
                System.out.println("Incrementer" + threadId + " completed");
            });
        }

        for (Thread thread : incrementers) {
            thread.start();
        }

        try {
            for (Thread thread : incrementers) {
                thread.join();
            }
        } catch (InterruptedException e) {
            System.out.println("Incrementer join interrupted");
        }

        System.out.println("Final counter value: " + counter);
        System.out.println("Expected value: 500");
    }

    private static void testSynchronizedMethods() {
        System.out.println("Test 2: Synchronized methods");
        SynchronizedCounter syncCounter = new SynchronizedCounter();
        Thread[] syncIncrementers = new Thread[3];

        for (int i = 0; i < 3; i++) {
            final int threadId = i;
            syncIncrementers[i] = new Thread(() -> {
                for (int j = 0; j < 50; j++) {
                    syncCounter.increment();
                }
                System.out.println("SyncIncrementer" + threadId + " completed, count: " + syncCounter.getCount());
            });
        }

        for (Thread thread : syncIncrementers) {
            thread.start();
        }

        try {
            for (Thread thread : syncIncrementers) {
                thread.join();
            }
        } catch (InterruptedException e) {
            System.out.println("Sync incrementer join interrupted");
        }

        System.out.println("Final synchronized counter: " + syncCounter.getCount());
    }

    private static void testDeadlockPrevention() {
        System.out.println("Test 3: Deadlock prevention");
        Thread thread1 = new Thread(() -> {
            synchronized (lock1) {
                System.out.println("Thread1: Acquired lock1");
                try {
                    Thread.sleep(100);
                } catch (InterruptedException e) {
                    System.out.println("Thread1 interrupted");
                }
                synchronized (lock2) {
                    System.out.println("Thread1: Acquired lock2");
                }
            }
        });

        Thread thread2 = new Thread(() -> {
            synchronized (lock1) { // Same order to prevent deadlock
                System.out.println("Thread2: Acquired lock1");
                try {
                    Thread.sleep(100);
                } catch (InterruptedException e) {
                    System.out.println("Thread2 interrupted");
                }
                synchronized (lock2) {
                    System.out.println("Thread2: Acquired lock2");
                }
            }
        });

        thread1.start();
        thread2.start();

        try {
            thread1.join();
            thread2.join();
        } catch (InterruptedException e) {
            System.out.println("Deadlock prevention test join interrupted");
        }
    }

    private static void testReentrantSynchronization() {
        System.out.println("Test 4: Reentrant synchronization");
        ReentrantClass reentrant = new ReentrantClass();
        Thread reentrantThread = new Thread(() -> {
            reentrant.method1();
        });

        reentrantThread.start();
        try {
            reentrantThread.join();
        } catch (InterruptedException e) {
            System.out.println("Reentrant test join interrupted");
        }
    }

    private static void testStaticSynchronization() {
        System.out.println("Test 5: Static synchronization");
        Thread[] staticSyncThreads = new Thread[3];

        for (int i = 0; i < 3; i++) {
            final int threadId = i;
            staticSyncThreads[i] = new Thread(() -> {
                StaticSyncClass.staticMethod(threadId);
            });
        }

        for (Thread thread : staticSyncThreads) {
            thread.start();
        }

        try {
            for (Thread thread : staticSyncThreads) {
                thread.join();
            }
        } catch (InterruptedException e) {
            System.out.println("Static sync test join interrupted");
        }
    }

    private static void testSynchronizationWithInheritance() {
        System.out.println("Test 6: Synchronization with inheritance");
        ChildSyncClass childSync = new ChildSyncClass();
        Thread[] inheritanceThreads = new Thread[2];

        inheritanceThreads[0] = new Thread(() -> {
            childSync.parentMethod();
        });

        inheritanceThreads[1] = new Thread(() -> {
            childSync.childMethod();
        });

        for (Thread thread : inheritanceThreads) {
            thread.start();
        }

        try {
            for (Thread thread : inheritanceThreads) {
                thread.join();
            }
        } catch (InterruptedException e) {
            System.out.println("Inheritance sync test join interrupted");
        }
    }

    static class SynchronizedCounter {
        private int count = 0;

        public synchronized void increment() {
            count++;
        }

        public synchronized int getCount() {
            return count;
        }
    }

    static class ReentrantClass {
        public synchronized void method1() {
            System.out.println("ReentrantClass: In method1");
            method2();
        }

        public synchronized void method2() {
            System.out.println("ReentrantClass: In method2");
            method3();
        }

        public synchronized void method3() {
            System.out.println("ReentrantClass: In method3");
        }
    }

    static class StaticSyncClass {
        public static synchronized void staticMethod(int threadId) {
            System.out.println("StaticSyncClass: Thread " + threadId + " entering static method");
            try {
                Thread.sleep(100);
            } catch (InterruptedException e) {
                System.out.println("StaticSyncClass: Thread " + threadId + " interrupted");
            }
            System.out.println("StaticSyncClass: Thread " + threadId + " exiting static method");
        }
    }

    static class ParentSyncClass {
        public synchronized void parentMethod() {
            System.out.println("ParentSyncClass: Parent method start");
            try {
                Thread.sleep(200);
            } catch (InterruptedException e) {
                System.out.println("ParentSyncClass: Parent method interrupted");
            }
            System.out.println("ParentSyncClass: Parent method end");
        }
    }

    static class ChildSyncClass extends ParentSyncClass {
        public synchronized void childMethod() {
            System.out.println("ChildSyncClass: Child method start");
            try {
                Thread.sleep(200);
            } catch (InterruptedException e) {
                System.out.println("ChildSyncClass: Child method interrupted");
            }
            System.out.println("ChildSyncClass: Child method end");
        }
    }
}
