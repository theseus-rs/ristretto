import java.lang.invoke.CallSite;
import java.lang.invoke.ConstantCallSite;
import java.lang.invoke.MethodHandle;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.MethodType;
import java.lang.reflect.InvocationTargetException;
import java.lang.reflect.Method;

/** Stack limits, invocation paths, propagation, unwinding, and recovery (JVMS 2.5.2/2.5.6). */
public class Test {
    private static int entries;
    private static int cleanups;
    private static final Object LOCK = new Object();
    private static Method reflected;
    private static MethodHandle handle;
    private static Runnable lambda;
    private static Action action;

    interface Task { void run() throws Throwable; }
    interface Action { void call(); }
    static class VirtualCall implements Action {
        public void call() { enter(); call(); }

        private void special() { enter(); special(); }

        synchronized void synchronizedCall() { enter(); synchronizedCall(); }
    }
    static class InterfaceCall implements Action {
        public void call() { enter(); action.call(); }
    }
    interface DefaultCall {
        default void recurse() { enter(); recurse(); }
    }
    static class DefaultImplementation implements DefaultCall { }
    static class ConstructorCall {
        ConstructorCall() { enter(); new ConstructorCall(); }
    }
    static class Initializer {
        static { direct(); }
        static void use() { }
    }

    private static native void nativeRecurse();

    private static native void prepareNative();

    public static void enter() {
        if (++entries > 100000) throw new AssertionError("Recursion limit did not produce StackOverflowError");
    }

    public static void direct() { enter(); direct(); }

    private static void mutualA() { enter(); mutualB(); }

    private static void mutualB() { enter(); mutualA(); }

    public static void reflective() throws Exception { enter(); reflected.invoke(null); }

    public static void methodHandle() throws Throwable { enter(); handle.invokeExact(); }

    private static void lambdaCall() { enter(); lambda.run(); }

    public static void nativeCallback() { enter(); nativeRecurse(); }

    private static void withFinally() {
        enter();
        try { withFinally(); } finally { cleanups++; }
    }

    private static void withMonitor() {
        enter();
        synchronized (LOCK) { withMonitor(); }
    }

    private static int finite(int n) { return n == 0 ? 0 : 1 + finite(n - 1); }

    /** Bootstrap for an actual recursive invokedynamic instruction in GeneratedFrames. */
    public static CallSite bootstrap(MethodHandles.Lookup lookup, String name, MethodType type)
            throws Exception {
        return new ConstantCallSite(lookup.findStatic(lookup.lookupClass(), "run", type));
    }

    public static void main(String[] args) throws Throwable {
        if (args.length != 1) throw new IllegalArgumentException("Use cargo test -p ristretto_vm --test resource_errors");
        String name = args[0];
        final Task task = task(name);
        // Repeated overflow verifies stack guard restoration and frame accounting after unwind.
        int repetitions = name.equals("initialization") ? 1 : 2;
        for (int i = 0; i < repetitions; i++) {
            entries = 0;
            cleanups = 0;
            Throwable caught = null;
            try { task.run(); } catch (Throwable error) { caught = error; }
            while (caught instanceof InvocationTargetException) caught = caught.getCause();
            if (caught == null || caught.getClass() != StackOverflowError.class) {
                throw new AssertionError("Expected StackOverflowError: " + name, caught);
            }
            if (entries == 0 && !name.equals("large-frame")) {
                throw new AssertionError("Overflow occurred before the tested method ran");
            }
            if (name.equals("finally") && (cleanups == 0 || cleanups != entries)) {
                throw new AssertionError("finally did not unwind every entered frame");
            }
            if (finite(20) != 20) throw new AssertionError("Stack did not recover");
        }
        if (name.equals("initialization")) {
            try {
                Initializer.use();
                throw new AssertionError("Erroneous class initialized twice");
            } catch (NoClassDefFoundError expected) { }
        }
        if (name.equals("monitor") || name.equals("synchronized")) {
            final boolean[] acquired = {false};
            final Object monitor = name.equals("monitor") ? LOCK : action;
            Thread probe = new Thread(new Runnable() {
                public void run() { synchronized (monitor) { acquired[0] = true; } }
            });
            probe.setDaemon(true);
            probe.start();
            probe.join(5000);
            if (probe.isAlive() || !acquired[0]) throw new AssertionError("Monitor leaked on unwind");
        }
        System.out.println("PASS " + name + " java.lang.StackOverflowError");
    }

    private static Task task(String name) throws Throwable {
        if (name.equals("static")) return new Task() { public void run() { direct(); } };
        if (name.equals("mutual")) return new Task() { public void run() { mutualA(); } };
        if (name.equals("virtual")) {
            final VirtualCall receiver = new VirtualCall();
            return new Task() { public void run() { receiver.call(); } };
        }
        if (name.equals("special")) {
            final VirtualCall receiver = new VirtualCall();
            return new Task() { public void run() { receiver.special(); } };
        }
        if (name.equals("interface")) {
            action = new InterfaceCall();
            return new Task() { public void run() { action.call(); } };
        }
        if (name.equals("default-method")) {
            final DefaultCall receiver = new DefaultImplementation();
            return new Task() { public void run() { receiver.recurse(); } };
        }
        if (name.equals("constructor")) return new Task() { public void run() { new ConstructorCall(); } };
        if (name.equals("initialization")) return new Task() { public void run() { Initializer.use(); } };
        if (name.equals("finally")) return new Task() { public void run() { withFinally(); } };
        if (name.equals("monitor")) return new Task() { public void run() { withMonitor(); } };
        if (name.equals("synchronized")) {
            final VirtualCall receiver = new VirtualCall();
            action = receiver;
            return new Task() { public void run() { receiver.synchronizedCall(); } };
        }
        if (name.equals("reflection")) {
            reflected = Test.class.getMethod("reflective");
            return new Task() { public void run() throws Exception { reflected.invoke(null); } };
        }
        if (name.equals("method-handle")) {
            handle = MethodHandles.lookup().findStatic(Test.class, "methodHandle", MethodType.methodType(void.class));
            return new Task() { public void run() throws Throwable { handle.invokeExact(); } };
        }
        if (name.equals("lambda")) {
            lambda = Test::lambdaCall;
            return new Task() { public void run() { lambda.run(); } };
        }
        if (name.equals("invokedynamic") || name.equals("large-frame") || name.equals("operand-stack")) {
            final Method method = GeneratedFrames.method(name);
            return new Task() { public void run() throws Exception { method.invoke(null); } };
        }
        if (name.equals("jni")) {
            System.load(System.getProperty("resource.library"));
            prepareNative();
            return new Task() { public void run() { nativeCallback(); } };
        }
        if (name.equals("thread") || name.equals("uncaught-thread") || name.equals("virtual-thread")) {
            final boolean uncaught = name.equals("uncaught-thread");
            final Method startVirtual = name.equals("virtual-thread")
                ? Thread.class.getMethod("startVirtualThread", Runnable.class) : null;
            return new Task() {
                public void run() throws Throwable {
                    final Throwable[] failure = new Throwable[1];
                    Runnable body = new Runnable() {
                        public void run() {
                            if (uncaught) direct();
                            else {
                                try { direct(); } catch (Throwable error) { failure[0] = error; }
                            }
                        }
                    };
                    Thread thread;
                    if (startVirtual != null) thread = (Thread) startVirtual.invoke(null, body);
                    else {
                        thread = new Thread(body);
                        thread.setDaemon(true);
                        thread.setUncaughtExceptionHandler(new Thread.UncaughtExceptionHandler() {
                            public void uncaughtException(Thread t, Throwable error) { failure[0] = error; }
                        });
                        thread.start();
                    }
                    thread.join(5000);
                    if (thread.isAlive()) throw new AssertionError("Worker did not terminate");
                    if (failure[0] != null) throw failure[0];
                }
            };
        }
        throw new IllegalArgumentException(name);
    }
}
