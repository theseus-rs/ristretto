/**
 * Tests that compile-time constants do NOT trigger class initialization.
 *
 * Per JLS §12.4.1:
 * "A reference to a field that is a constant variable (§4.12.4) must not trigger
 * initialization of the class or interface that declares the constant field."
 *
 * A compile-time constant is a static final field of a primitive type or String
 * that is initialized with a constant expression.
 */
public class Test {
    static class A {
        // This is a compile-time constant (primitive + constant expression)
        static final int CONSTANT = 42;

        // This is NOT a compile-time constant (requires method call)
        static final int NOT_CONSTANT = compute();

        static {
            System.out.println("A.<clinit> executed");
        }

        static int compute() {
            System.out.println("A.compute() called");
            return 100;
        }
    }

    static class B {
        // This is a compile-time constant (String literal)
        static final String STRING_CONSTANT = "hello";

        static {
            System.out.println("B.<clinit> executed");
        }
    }

    static class C {
        // This is NOT a compile-time constant (requires method call)
        static final int NOT_CONSTANT = compute();

        static {
            System.out.println("C.<clinit> executed");
        }

        static int compute() {
            System.out.println("C.compute() called");
            return 200;
        }
    }

    public static void main(String[] args) {
        // Accessing compile-time constant should NOT trigger A.<clinit>
        System.out.println("Accessing A.CONSTANT (compile-time constant):");
        System.out.println("A.CONSTANT = " + A.CONSTANT);
        System.out.println();

        // Accessing non-constant static final WILL trigger A.<clinit>
        System.out.println("Accessing A.NOT_CONSTANT (not compile-time constant):");
        System.out.println("A.NOT_CONSTANT = " + A.NOT_CONSTANT);
        System.out.println();

        // Accessing String constant should NOT trigger B.<clinit>
        System.out.println("Accessing B.STRING_CONSTANT (compile-time constant):");
        System.out.println("B.STRING_CONSTANT = " + B.STRING_CONSTANT);
        System.out.println();

        // Accessing non-constant field WILL trigger C.<clinit>
        System.out.println("Accessing C.NOT_CONSTANT (not compile-time constant):");
        System.out.println("C.NOT_CONSTANT = " + C.NOT_CONSTANT);
    }
}

