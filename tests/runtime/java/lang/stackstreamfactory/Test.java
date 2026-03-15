/** Test java.lang.StackStreamFactory via the StackWalker API. */
public class Test {
    public static void main(String[] args) {
        testStackWalkerCreation();
        testCallerClass();
        testStackWalkHasFrames();
        testStackWalkMethodNames();
    }

    private static void testStackWalkerCreation() {
        System.out.println("=== StackWalker Creation ===");
        StackWalker walker = StackWalker.getInstance();
        System.out.println("Created: " + (walker != null));
    }

    private static void testCallerClass() {
        System.out.println("=== Caller Class ===");
        Class<?> caller = getCallerFromHelper();
        System.out.println("Caller: " + caller.getName());
    }

    private static Class<?> getCallerFromHelper() {
        return StackWalker.getInstance(StackWalker.Option.RETAIN_CLASS_REFERENCE).getCallerClass();
    }

    private static void testStackWalkHasFrames() {
        System.out.println("=== Stack Walk Has Frames ===");
        StackWalker walker = StackWalker.getInstance();
        boolean hasFrames = walker.walk(s -> s.count() > 0);
        System.out.println("Has frames: " + hasFrames);
    }

    private static void testStackWalkMethodNames() {
        System.out.println("=== Stack Walk Methods ===");
        level1();
    }

    private static void level1() {
        level2();
    }

    private static void level2() {
        level3();
    }

    private static void level3() {
        StackWalker walker = StackWalker.getInstance();
        walker.forEach(frame -> {
            if (frame.getClassName().equals("Test") && !frame.getMethodName().startsWith("lambda$")) {
                System.out.println(frame.getMethodName());
            }
        });
    }
}
