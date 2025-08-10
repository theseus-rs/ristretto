public class Test {
    public static void main(String[] args) {
        System.out.println("=== Thread Local Tests ===");

        testBasicThreadLocalUsage();
        testInheritableThreadLocal();
        testThreadLocalCleanup();
        testMultipleThreadLocalVariables();
        testThreadLocalInThreadPoolSimulation();
        testThreadLocalWithCustomObjects();

        System.out.println("ThreadLocal tests completed");
    }

    private static void testBasicThreadLocalUsage() {
        System.out.println("Test 1: Basic ThreadLocal usage");
        ThreadLocal<String> threadLocal = new ThreadLocal<String>() {
            @Override
            protected String initialValue() {
                return "Initial-" + Thread.currentThread().getName();
            }
        };

        Thread[] localThreads = new Thread[3];
        for (int i = 0; i < 3; i++) {
            final int threadId = i;
            localThreads[i] = new Thread(() -> {
                String initial = threadLocal.get();
                System.out.println("LocalThread" + threadId + " initial value: " + initial);

                threadLocal.set("Modified-" + threadId);
                String modified = threadLocal.get();
                System.out.println("LocalThread" + threadId + " modified value: " + modified);

                try {
                    Thread.sleep(100);
                } catch (InterruptedException e) {
                    System.out.println("LocalThread" + threadId + " interrupted");
                }

                String final_value = threadLocal.get();
                System.out.println("LocalThread" + threadId + " final value: " + final_value);
            }, "LocalThread" + i);
        }

        for (Thread thread : localThreads) {
            thread.start();
        }

        try {
            for (Thread thread : localThreads) {
                thread.join();
            }
        } catch (InterruptedException e) {
            System.out.println("ThreadLocal test interrupted");
        }
    }

    private static void testInheritableThreadLocal() {
        System.out.println("Test 2: InheritableThreadLocal");
        InheritableThreadLocal<Integer> inheritableLocal = new InheritableThreadLocal<Integer>() {
            @Override
            protected Integer initialValue() {
                return 0;
            }

            @Override
            protected Integer childValue(Integer parentValue) {
                return parentValue + 100; // Child gets parent value + 100
            }
        };

        Thread parentThread = new Thread(() -> {
            inheritableLocal.set(42);
            System.out.println("ParentThread: Set value to " + inheritableLocal.get());

            Thread childThread = new Thread(() -> {
                Integer inherited = inheritableLocal.get();
                System.out.println("ChildThread: Inherited value: " + inherited);

                inheritableLocal.set(inherited + 1);
                System.out.println("ChildThread: Modified value: " + inheritableLocal.get());
            });

            childThread.start();
            try {
                childThread.join();
                System.out.println("ParentThread: After child completion, value: " + inheritableLocal.get());
            } catch (InterruptedException e) {
                System.out.println("Parent thread interrupted waiting for child");
            }
        });

        parentThread.start();
        try {
            parentThread.join();
        } catch (InterruptedException e) {
            System.out.println("Inheritable ThreadLocal test interrupted");
        }
    }

    private static void testThreadLocalCleanup() {
        System.out.println("Test 3: ThreadLocal cleanup");
        ThreadLocal<String> cleanupLocal = new ThreadLocal<>();

        Thread cleanupThread = new Thread(() -> {
            cleanupLocal.set("CleanupValue");
            System.out.println("CleanupThread: Set value: " + cleanupLocal.get());

            // Explicitly remove the value
            cleanupLocal.remove();
            String afterRemove = cleanupLocal.get();
            System.out.println("CleanupThread: After remove: " +
                             (afterRemove == null ? "null" : afterRemove));
        });

        cleanupThread.start();
        try {
            cleanupThread.join();
        } catch (InterruptedException e) {
            System.out.println("Cleanup test interrupted");
        }
    }

    private static void testMultipleThreadLocalVariables() {
        System.out.println("Test 4: Multiple ThreadLocal variables");
        ThreadLocal<String> stringLocal = ThreadLocal.withInitial(() -> "DefaultString");
        ThreadLocal<Integer> intLocal = ThreadLocal.withInitial(() -> 0);
        ThreadLocal<Boolean> boolLocal = ThreadLocal.withInitial(() -> false);

        Thread multiLocalThread = new Thread(() -> {
            System.out.println("MultiLocalThread: String: " + stringLocal.get());
            System.out.println("MultiLocalThread: Integer: " + intLocal.get());
            System.out.println("MultiLocalThread: Boolean: " + boolLocal.get());

            stringLocal.set("ModifiedString");
            intLocal.set(123);
            boolLocal.set(true);

            System.out.println("MultiLocalThread: Modified String: " + stringLocal.get());
            System.out.println("MultiLocalThread: Modified Integer: " + intLocal.get());
            System.out.println("MultiLocalThread: Modified Boolean: " + boolLocal.get());
        });

        multiLocalThread.start();
        try {
            multiLocalThread.join();
        } catch (InterruptedException e) {
            System.out.println("Multi ThreadLocal test interrupted");
        }
    }

    private static void testThreadLocalInThreadPoolSimulation() {
        System.out.println("Test 5: ThreadLocal in thread pool simulation");
        ThreadLocal<Integer> poolLocal = new ThreadLocal<Integer>() {
            @Override
            protected Integer initialValue() {
                return 1;
            }
        };

        // Simulate thread reuse
        Thread poolThread = new Thread(() -> {
            // First task
            Integer value1 = poolLocal.get();
            poolLocal.set(value1 * 2);
            System.out.println("PoolThread: Task 1, value: " + poolLocal.get());

            // Clear for next task
            poolLocal.remove();

            // Second task (should get initial value again)
            Integer value2 = poolLocal.get();
            poolLocal.set(value2 * 3);
            System.out.println("PoolThread: Task 2, value: " + poolLocal.get());
        });

        poolThread.start();
        try {
            poolThread.join();
        } catch (InterruptedException e) {
            System.out.println("Pool ThreadLocal test interrupted");
        }
    }

    private static void testThreadLocalWithCustomObjects() {
        System.out.println("Test 6: ThreadLocal with custom objects");
        ThreadLocal<UserSession> sessionLocal = new ThreadLocal<UserSession>() {
            @Override
            protected UserSession initialValue() {
                return new UserSession("Anonymous", 0);
            }
        };

        Thread[] sessionThreads = new Thread[2];
        String[] userNames = {"Alice", "Bob"};

        for (int i = 0; i < 2; i++) {
            final int userId = i;
            sessionThreads[i] = new Thread(() -> {
                UserSession session = sessionLocal.get();
                System.out.println("SessionThread" + userId + " initial: " + session);

                session.setUsername(userNames[userId]);
                session.setUserId(userId + 1);

                System.out.println("SessionThread" + userId + " modified: " + sessionLocal.get());
            });
        }

        for (Thread thread : sessionThreads) {
            thread.start();
        }

        try {
            for (Thread thread : sessionThreads) {
                thread.join();
            }
        } catch (InterruptedException e) {
            System.out.println("Session ThreadLocal test interrupted");
        }
    }

    static class UserSession {
        private String username;
        private int userId;

        public UserSession(String username, int userId) {
            this.username = username;
            this.userId = userId;
        }

        public String getUsername() { return username; }
        public void setUsername(String username) { this.username = username; }
        public int getUserId() { return userId; }
        public void setUserId(int userId) { this.userId = userId; }

        @Override
        public String toString() {
            return "UserSession{username='" + username + "', userId=" + userId + "}";
        }
    }
}
