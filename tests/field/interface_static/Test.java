/**
 * Tests static field initialization in interfaces.
 *
 * Per JLS §12.4.1:
 * - Interface static fields are always public static final
 * - Interface initialization does NOT initialize implementing classes
 * - Accessing a compile-time constant does NOT trigger interface initialization
 */
public class Test {
    interface I {
        // Compile-time constant - accessing this should NOT trigger initialization
        int CONSTANT = 42;

        // Not a compile-time constant - accessing this WILL trigger initialization
        int NOT_CONSTANT = compute();

        static int compute() {
            System.out.println("I.compute() called - interface being initialized");
            return 100;
        }
    }

    static class Impl implements I {
        static {
            System.out.println("Impl.<clinit> executed");
        }
    }

    public static void main(String[] args) {
        // Accessing compile-time constant - should NOT trigger I's initialization
        System.out.println("Accessing I.CONSTANT (compile-time constant):");
        System.out.println("I.CONSTANT = " + I.CONSTANT);
        System.out.println();

        // Accessing non-constant - WILL trigger I's initialization
        System.out.println("Accessing I.NOT_CONSTANT:");
        System.out.println("I.NOT_CONSTANT = " + I.NOT_CONSTANT);
        System.out.println();

        // Creating Impl should NOT re-initialize I
        System.out.println("Creating Impl instance:");
        Impl impl = new Impl();

        // Accessing constant through impl should NOT trigger anything special
        System.out.println();
        System.out.println("Accessing through impl:");
        System.out.println("impl.CONSTANT = " + Impl.CONSTANT);
    }
}

