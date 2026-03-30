/**
 * Test that the main thread has the correct name.
 *
 * The JVM specification requires that the primordial thread is named "main".
 * Thread.currentThread().getName() must return "main" when called from the
 * main method.
 */
public class Test {
    public static void main(String[] args) {
        testMainThreadName();
        testMainThreadNameFromMethod();
        testChildThreadSeesOwnName();
    }

    /** The main thread must be named "main". */
    static void testMainThreadName() {
        System.out.println("=== Main Thread Name ===");
        Thread current = Thread.currentThread();
        String name = current.getName();
        System.out.println("Main thread name: " + name);
        System.out.println("Is main: " + "main".equals(name));
    }

    /** Thread.currentThread().getName() returns "main" even from a called method. */
    static void testMainThreadNameFromMethod() {
        System.out.println("\n=== Main Thread Name From Method ===");
        String name = getCallingThreadName();
        System.out.println("From method: " + name);
        System.out.println("Is main: " + "main".equals(name));
    }

    /** A child thread should see its own name, not "main". */
    static void testChildThreadSeesOwnName() {
        System.out.println("\n=== Child Thread Name ===");
        final String[] childName = new String[1];
        Thread child = new Thread(() -> {
            childName[0] = Thread.currentThread().getName();
        }, "TestChild");
        child.start();
        try {
            child.join();
        } catch (InterruptedException e) {
            // ignore
        }
        System.out.println("Child thread name: " + childName[0]);
        System.out.println("Is TestChild: " + "TestChild".equals(childName[0]));
    }

    private static String getCallingThreadName() {
        return Thread.currentThread().getName();
    }
}
