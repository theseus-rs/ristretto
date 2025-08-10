public class Test {
    public static void main(String[] args) {
        System.out.println("=== Thread Group Tests ===");

        testBasicThreadGroup();
        testNestedThreadGroups();
        testThreadGroupEnumeration();
        testThreadGroupInterrupt();
        testThreadGroupDaemonStatus();
        testThreadGroupDestruction();

        System.out.println("Thread group tests completed");
    }

    private static void testBasicThreadGroup() {
        System.out.println("Test 1: Basic thread group");
        ThreadGroup group1 = new ThreadGroup("TestGroup1");
        System.out.println("Group name: " + group1.getName());
        System.out.println("Group parent: " + group1.getParent().getName());
        System.out.println("Initial active count: " + group1.activeCount());

        Thread thread1 = new Thread(group1, () -> {
            System.out.println("Thread1 in group: " + Thread.currentThread().getThreadGroup().getName());
            try {
                Thread.sleep(200);
            } catch (InterruptedException e) {
                System.out.println("Thread1 interrupted");
            }
        }, "GroupThread1");

        Thread thread2 = new Thread(group1, () -> {
            System.out.println("Thread2 in group: " + Thread.currentThread().getThreadGroup().getName());
            try {
                Thread.sleep(200);
            } catch (InterruptedException e) {
                System.out.println("Thread2 interrupted");
            }
        }, "GroupThread2");

        thread1.start();
        thread2.start();

        System.out.println("Active count after starting threads: " + group1.activeCount());

        try {
            thread1.join();
            thread2.join();
        } catch (InterruptedException e) {
            System.out.println("Basic thread group join interrupted");
        }

        System.out.println("Final active count in group1: " + group1.activeCount());
    }

    private static void testNestedThreadGroups() {
        System.out.println("Test 2: Nested thread groups");
        ThreadGroup parentGroup = new ThreadGroup("ParentGroup");
        ThreadGroup childGroup = new ThreadGroup(parentGroup, "ChildGroup");

        System.out.println("Child group parent: " + childGroup.getParent().getName());
        System.out.println("Parent group max priority: " + parentGroup.getMaxPriority());
        System.out.println("Child group max priority: " + childGroup.getMaxPriority());

        Thread childThread = new Thread(childGroup, () -> {
            System.out.println("Child thread group: " + Thread.currentThread().getThreadGroup().getName());
            System.out.println("Child thread priority: " + Thread.currentThread().getPriority());
        }, "ChildThread");

        childThread.start();

        try {
            childThread.join();
        } catch (InterruptedException e) {
            System.out.println("Child thread join interrupted");
        }
    }

    private static void testThreadGroupEnumeration() {
        System.out.println("Test 3: Thread group enumeration");
        ThreadGroup enumGroup = new ThreadGroup("EnumGroup");

        Thread[] enumThreads = new Thread[2];
        for (int i = 0; i < 2; i++) {
            final int threadId = i;
            enumThreads[i] = new Thread(enumGroup, () -> {
                try {
                    Thread.sleep(100);
                } catch (InterruptedException e) {
                    System.out.println("EnumThread" + threadId + " interrupted");
                }
            }, "EnumThread" + i);
            enumThreads[i].start();
        }

        Thread[] threads = new Thread[enumGroup.activeCount()];
        int count = enumGroup.enumerate(threads);
        System.out.println("Enumerated " + count + " threads from enumGroup:");
        for (int i = 0; i < count; i++) {
            System.out.println("  Thread: " + threads[i].getName());
        }

        try {
            for (Thread thread : enumThreads) {
                thread.join();
            }
        } catch (InterruptedException e) {
            System.out.println("Enum threads join interrupted");
        }
    }

    private static void testThreadGroupInterrupt() {
        System.out.println("Test 4: Thread group interrupt");
        ThreadGroup interruptGroup = new ThreadGroup("InterruptGroup");

        Thread[] interruptThreads = new Thread[2];
        for (int i = 0; i < 2; i++) {
            final int threadNum = i;
            interruptThreads[i] = new Thread(interruptGroup, () -> {
                try {
                    System.out.println("InterruptThread" + threadNum + ": Starting long operation");
                    Thread.sleep(1000);
                    System.out.println("InterruptThread" + threadNum + ": Completed normally");
                } catch (InterruptedException e) {
                    System.out.println("InterruptThread" + threadNum + ": Was interrupted");
                }
            }, "InterruptThread" + i);
            interruptThreads[i].start();
        }

        try {
            Thread.sleep(100);
            System.out.println("Interrupting entire thread group");
            interruptGroup.interrupt();
        } catch (InterruptedException e) {
            System.out.println("Main thread interrupted during group test");
        }

        try {
            for (Thread t : interruptThreads) {
                t.join();
            }
        } catch (InterruptedException e) {
            System.out.println("Interrupt threads join interrupted");
        }
    }

    private static void testThreadGroupDaemonStatus() {
        System.out.println("Test 5: Thread group daemon status");
        ThreadGroup daemonGroup = new ThreadGroup("DaemonGroup");
        System.out.println("Group daemon status before: " + daemonGroup.isDaemon());
        daemonGroup.setDaemon(true);
        System.out.println("Group daemon status after: " + daemonGroup.isDaemon());
    }

    private static void testThreadGroupDestruction() {
        System.out.println("Test 6: Thread group destruction");
        ThreadGroup destroyGroup = new ThreadGroup("DestroyGroup");
        System.out.println("Group destroyed status before: " + destroyGroup.isDestroyed());

        Thread destroyThread = new Thread(destroyGroup, () -> {
            System.out.println("Destroy thread running");
        });
        destroyThread.start();

        try {
            destroyThread.join();
        } catch (InterruptedException e) {
            System.out.println("Destroy thread join interrupted");
        }
    }
}
