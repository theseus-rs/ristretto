import java.lang.reflect.Array;

/** Non-exhausting controls shared by OpenJDK and the ordinary Ristretto compatibility suite. */
public class Test {
    interface Operation { void run(); }
    private static volatile Object result;

    public static void main(String[] args) {
        negative("newarray", new Operation() {
            public void run() { result = new long[-1]; }
        });
        negative("anewarray", new Operation() {
            public void run() { result = new Object[-1]; }
        });
        negative("multianewarray outer", new Operation() {
            public void run() { result = new byte[-1][1]; }
        });
        negative("multianewarray inner", new Operation() {
            public void run() { result = new Object[1][-1]; }
        });
        negative("negative after zero", new Operation() {
            public void run() { result = new byte[0][-1]; }
        });
        negative("reflection", new Operation() {
            public void run() { result = Array.newInstance(int.class, -1); }
        });
        negative("reflection multidimensional", new Operation() {
            public void run() { result = Array.newInstance(Object.class, 0, -1); }
        });
        // Later dimensions are not allocated if an earlier dimension has length zero.
        byte[][] empty = new byte[0][Integer.MAX_VALUE];
        Object reflected = Array.newInstance(Object.class, 0, Integer.MAX_VALUE);
        if (empty.length != 0 || Array.getLength(reflected) != 0) throw new AssertionError();
        System.out.println("zero outer dimension: no allocation of inner arrays");
        // These explicitly constructed errors test athrow/catch semantics, not exhaustion.
        errorContract(new OutOfMemoryError("test"));
        errorContract(new StackOverflowError("test"));
        System.out.println("PASS controls");
    }

    private static void negative(String label, Operation operation) {
        try {
            operation.run();
            throw new AssertionError("Expected NegativeArraySizeException: " + label);
        } catch (NegativeArraySizeException expected) {
            System.out.println(label + ": NegativeArraySizeException");
        }
    }

    private static void errorContract(VirtualMachineError error) {
        boolean unwound = false;
        try {
            try { throw error; } finally { unwound = true; }
        } catch (Exception wrong) {
            throw new AssertionError("VM errors must not be caught as Exception", wrong);
        } catch (Error caught) {
            if (caught != error || !unwound || !"test".equals(caught.getMessage())) {
                throw new AssertionError("Error identity, message, or finally was lost");
            }
            System.out.println(caught.getClass().getName() + ": Error identity and finally preserved");
        }
    }
}
