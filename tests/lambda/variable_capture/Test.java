/** Test variable capture in lambda expressions. */
public class Test {
    private static int staticField = 100;
    private int instanceField = 200;

    @FunctionalInterface
    interface IntSupplier {
        int get();
    }

    @FunctionalInterface
    interface StringSupplier {
        String get();
    }

    @FunctionalInterface
    interface Runnable {
        void run();
    }

    public void testInstanceCapture() {
        // Capture instance field
        IntSupplier captureInstance = () -> instanceField;
        System.out.println("Captured instance field: " + captureInstance.get());

        // Modify instance field and check lambda sees the change
        instanceField = 250;
        System.out.println("After modification: " + captureInstance.get());

        // Capture 'this'
        StringSupplier captureThis = () -> "Instance: " + this.toString();
        System.out.println("Captured this: " + captureThis.get());
    }

    public static void testLocalCapture() {
        // Capture effectively final local variable
        int localVar = 42;
        IntSupplier captureLocal = () -> localVar;
        System.out.println("Captured local: " + captureLocal.get());

        // Capture multiple local variables
        int a = 10;
        int b = 20;
        IntSupplier captureMultiple = () -> a + b;
        System.out.println("Captured multiple: " + captureMultiple.get());

        // Capture effectively final String
        String message = "Hello";
        StringSupplier captureString = () -> message;
        System.out.println("Captured string: " + captureString.get());

        // Capture array (arrays can be modified even if effectively final)
        int[] array = {1, 2, 3};
        IntSupplier captureArray = () -> array[0];
        System.out.println("Captured array[0] before: " + captureArray.get());
        array[0] = 999;
        System.out.println("Captured array[0] after: " + captureArray.get());

        // Capture final variable
        final int finalVar = 100;
        IntSupplier captureFinal = () -> finalVar;
        System.out.println("Captured final: " + captureFinal.get());
    }

    public static void testStaticCapture() {
        // Capture static field
        IntSupplier captureStatic = () -> staticField;
        System.out.println("Captured static: " + captureStatic.get());

        // Modify static and check lambda sees the change
        staticField = 150;
        System.out.println("After static modification: " + captureStatic.get());
    }

    public static void testCaptureInLoop() {
        System.out.println("Capture in loop:");
        for (int i = 0; i < 3; i++) {
            final int captured = i;
            Runnable r = () -> System.out.println("  Loop iteration: " + captured);
            r.run();
        }
    }

    public static void testNestedCapture() {
        int outer = 100;
        IntSupplier outerLambda = () -> {
            int inner = 50;
            IntSupplier innerLambda = () -> outer + inner;
            return innerLambda.get();
        };
        System.out.println("Nested capture: " + outerLambda.get());
    }

    public static void testCaptureFromMethod() {
        IntSupplier supplier = createSupplier(42);
        System.out.println("Captured from method: " + supplier.get());
    }

    private static IntSupplier createSupplier(int value) {
        return () -> value * 2;
    }

    @Override
    public String toString() {
        return "Test[field=" + instanceField + "]";
    }

    public static void main(String[] args) {
        System.out.println("=== Variable Capture Tests ===");

        Test test = new Test();

        System.out.println("--- Instance Field Capture ---");
        test.testInstanceCapture();

        System.out.println("--- Local Variable Capture ---");
        testLocalCapture();

        System.out.println("--- Static Field Capture ---");
        testStaticCapture();

        System.out.println("--- Capture in Loop ---");
        testCaptureInLoop();

        System.out.println("--- Nested Capture ---");
        testNestedCapture();

        System.out.println("--- Capture from Method ---");
        testCaptureFromMethod();

        System.out.println("=== End Variable Capture Tests ===");
    }
}
