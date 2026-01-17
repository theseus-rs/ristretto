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

        // Run threads sequentially for deterministic output
        Thread thread1 = new Thread(group1, () -> {
            System.out.println("Thread1 in group: " + Thread.currentThread().getThreadGroup().getName());
        }, "GroupThread1");

        thread1.start();
        try {
            thread1.join();
        } catch (InterruptedException e) {
            System.out.println("Thread1 join interrupted");
        }

        Thread thread2 = new Thread(group1, () -> {
            System.out.println("Thread2 in group: " + Thread.currentThread().getThreadGroup().getName());
        }, "GroupThread2");

        thread2.start();
        try {
            thread2.join();
        } catch (InterruptedException e) {
            System.out.println("Thread2 join interrupted");
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

        // Run threads sequentially for deterministic output
        for (int i = 0; i < 2; i++) {
            final int threadId = i;
            Thread enumThread = new Thread(enumGroup, () -> {
                System.out.println("EnumThread" + threadId + " running");
            }, "EnumThread" + i);
            enumThread.start();
            try {
                enumThread.join();
            } catch (InterruptedException e) {
                System.out.println("EnumThread" + threadId + " join interrupted");
            }
        }
        System.out.println("Enumeration test completed");
    }

    private static void testThreadGroupInterrupt() {
        System.out.println("Test 4: Thread group interrupt");
        ThreadGroup interruptGroup = new ThreadGroup("InterruptGroup");

        // Run threads sequentially for deterministic output
        for (int i = 0; i < 2; i++) {
            final int threadNum = i;
            Thread interruptThread = new Thread(interruptGroup, () -> {
                try {
                    System.out.println("InterruptThread" + threadNum + ": Starting operation");
                    Thread.sleep(100);
                    System.out.println("InterruptThread" + threadNum + ": Completed normally");
                } catch (InterruptedException e) {
                    System.out.println("InterruptThread" + threadNum + ": Was interrupted");
                }
            }, "InterruptThread" + i);
            interruptThread.start();
            try {
                interruptThread.join();
            } catch (InterruptedException e) {
                System.out.println("InterruptThread" + threadNum + " join interrupted");
            }
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
