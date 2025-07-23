/** Test interface exception handling and throws clauses */
import java.io.*;

interface ThrowsInterface {
    void mayThrowChecked() throws IOException;
    void mayThrowMultiple() throws IOException, ClassNotFoundException;
    void mayThrowUnchecked() throws RuntimeException;
    void noThrowsClause();

    default void defaultWithThrows() throws Exception {
        System.out.println("ThrowsInterface.defaultWithThrows");
        throw new Exception("Default method exception");
    }

    static void staticWithThrows() throws IllegalStateException {
        System.out.println("ThrowsInterface.staticWithThrows");
        throw new IllegalStateException("Static method exception");
    }
}

interface ExceptionHierarchy {
    void throwsParent() throws Exception;
    void throwsChild() throws IOException;
}

class ThrowsImplementation implements ThrowsInterface, ExceptionHierarchy {
    private boolean shouldThrow = false;

    public void setShouldThrow(boolean shouldThrow) {
        this.shouldThrow = shouldThrow;
    }

    // Can throw same or fewer exceptions than interface declares
    public void mayThrowChecked() throws IOException {
        System.out.println("ThrowsImplementation.mayThrowChecked");
        if (shouldThrow) {
            throw new IOException("Implementation IOException");
        }
    }

    // Can throw subset of declared exceptions
    public void mayThrowMultiple() throws IOException {
        System.out.println("ThrowsImplementation.mayThrowMultiple");
        if (shouldThrow) {
            throw new IOException("Implementation IOException from multiple");
        }
    }

    // Can throw subtype of declared exception
    public void mayThrowUnchecked() throws IllegalArgumentException {
        System.out.println("ThrowsImplementation.mayThrowUnchecked");
        if (shouldThrow) {
            throw new IllegalArgumentException("Implementation IllegalArgumentException");
        }
    }

    // Can add unchecked exceptions even if not declared
    public void noThrowsClause() {
        System.out.println("ThrowsImplementation.noThrowsClause");
        if (shouldThrow) {
            throw new RuntimeException("Unchecked exception from no-throws method");
        }
    }

    // Exception hierarchy implementation
    public void throwsParent() throws Exception {
        System.out.println("ThrowsImplementation.throwsParent");
        if (shouldThrow) {
            throw new Exception("Parent exception");
        }
    }

    // Must throw same or subtype of interface declaration
    public void throwsChild() throws FileNotFoundException {
        System.out.println("ThrowsImplementation.throwsChild");
        if (shouldThrow) {
            throw new FileNotFoundException("Child exception - FileNotFoundException");
        }
    }
}

class NoThrowsImplementation implements ThrowsInterface, ExceptionHierarchy {
    // Implementation can choose not to throw any exceptions
    public void mayThrowChecked() {
        System.out.println("NoThrowsImplementation.mayThrowChecked - no exceptions");
    }

    public void mayThrowMultiple() {
        System.out.println("NoThrowsImplementation.mayThrowMultiple - no exceptions");
    }

    public void mayThrowUnchecked() {
        System.out.println("NoThrowsImplementation.mayThrowUnchecked - no exceptions");
    }

    public void noThrowsClause() {
        System.out.println("NoThrowsImplementation.noThrowsClause - no exceptions");
    }

    public void throwsParent() {
        System.out.println("NoThrowsImplementation.throwsParent - no exceptions");
    }

    public void throwsChild() {
        System.out.println("NoThrowsImplementation.throwsChild - no exceptions");
    }
}

// Test with lambda and functional interface exceptions
@FunctionalInterface
interface ThrowingFunctional {
    void execute() throws Exception;

    default void safeExecute() {
        try {
            execute();
        } catch (Exception e) {
            System.out.println("Exception caught in safeExecute: " + e.getMessage());
        }
    }
}

public class Test {
    public static void main(String[] args) {
        System.out.println("=== Interface Exception Handling Test ===");

        ThrowsImplementation throwingImpl = new ThrowsImplementation();
        NoThrowsImplementation noThrowImpl = new NoThrowsImplementation();

        // Test when implementation doesn't throw
        System.out.println("Testing implementations that don't throw:");
        testInterface(throwingImpl, false);
        testInterface(noThrowImpl, false);

        // Test when implementation does throw
        System.out.println("\nTesting implementations that do throw:");
        testInterface(throwingImpl, true);

        // Test exception hierarchy
        System.out.println("\nTesting exception hierarchy:");
        testExceptionHierarchy(throwingImpl, false);
        testExceptionHierarchy(throwingImpl, true);

        // Test static method with exceptions
        System.out.println("\nTesting static method exceptions:");
        try {
            ThrowsInterface.staticWithThrows();
        } catch (IllegalStateException e) {
            System.out.println("Caught static method exception: " + e.getMessage());
        }

        // Test default method with exceptions
        System.out.println("\nTesting default method exceptions:");
        try {
            throwingImpl.defaultWithThrows();
        } catch (Exception e) {
            System.out.println("Caught default method exception: " + e.getMessage());
        }

        // Test functional interface with exceptions
        System.out.println("\nTesting functional interface exceptions:");
        ThrowingFunctional throwingLambda = () -> {
            System.out.println("Lambda about to throw");
            throw new RuntimeException("Lambda exception");
        };

        ThrowingFunctional safeLambda = () -> System.out.println("Safe lambda execution");

        // Test safe execution
        safeLambda.safeExecute();

        // Test exception in lambda
        throwingLambda.safeExecute();

        // Test direct lambda exception
        try {
            throwingLambda.execute();
        } catch (Exception e) {
            System.out.println("Caught lambda exception: " + e.getMessage());
        }

        // Test exception propagation through interface references
        System.out.println("\nTesting exception propagation through interface references:");
        ((ThrowsImplementation) throwingImpl).setShouldThrow(true);

        try {
            throwingImpl.mayThrowChecked();
        } catch (IOException e) {
            System.out.println("Caught via interface reference: " + e.getMessage());
        }

        System.out.println("Exception handling tests completed");
    }

    private static void testInterface(ThrowsInterface iface, boolean shouldThrow) {
        if (iface instanceof ThrowsImplementation) {
            ((ThrowsImplementation) iface).setShouldThrow(shouldThrow);
        }

        try {
            iface.mayThrowChecked();
        } catch (IOException e) {
            System.out.println("Caught IOException from mayThrowChecked: " + e.getMessage());
        }

        try {
            iface.mayThrowMultiple();
        } catch (IOException e) {
            System.out.println("Caught IOException from mayThrowMultiple: " + e.getMessage());
        } catch (ClassNotFoundException e) {
            System.out.println("Caught ClassNotFoundException from mayThrowMultiple: " + e.getMessage());
        }

        try {
            iface.mayThrowUnchecked();
        } catch (RuntimeException e) {
            System.out.println("Caught RuntimeException from mayThrowUnchecked: " + e.getMessage());
        }

        try {
            iface.noThrowsClause();
        } catch (RuntimeException e) {
            System.out.println("Caught RuntimeException from noThrowsClause: " + e.getMessage());
        }
    }

    private static void testExceptionHierarchy(ExceptionHierarchy hierarchy, boolean shouldThrow) {
        if (hierarchy instanceof ThrowsImplementation) {
            ((ThrowsImplementation) hierarchy).setShouldThrow(shouldThrow);
        }

        try {
            hierarchy.throwsParent();
        } catch (Exception e) {
            System.out.println("Caught from throwsParent: " + e.getClass().getSimpleName() + " - " + e.getMessage());
        }

        try {
            hierarchy.throwsChild();
        } catch (IOException e) {
            System.out.println("Caught from throwsChild: " + e.getClass().getSimpleName() + " - " + e.getMessage());
        }
    }
}
