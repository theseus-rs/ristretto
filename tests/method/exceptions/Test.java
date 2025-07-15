/** Test method exception handling and throws declarations. */
public class Test {
    // Method that throws checked exception
    public static void throwsCheckedException() throws Exception {
        System.out.println("About to throw checked exception");
        throw new Exception("This is a checked exception");
    }

    // Method that throws specific checked exceptions
    public static void throwsSpecificExceptions(int type) throws IllegalArgumentException, NumberFormatException {
        System.out.println("Testing specific exceptions with type: " + type);
        switch (type) {
            case 1:
                throw new IllegalArgumentException("Invalid argument provided");
            case 2:
                throw new NumberFormatException("Invalid number format");
            default:
                System.out.println("No exception thrown");
        }
    }

    // Method that throws runtime exception (unchecked)
    public static void throwsRuntimeException() {
        System.out.println("About to throw runtime exception");
        throw new RuntimeException("This is a runtime exception");
    }

    // Method with try-catch handling
    public static void handleExceptions() {
        try {
            throwsCheckedException();
        } catch (Exception e) {
            System.out.println("Caught checked exception: " + e.getMessage());
        }

        try {
            throwsSpecificExceptions(1);
        } catch (NumberFormatException e) {
            System.out.println("Caught NumberFormatException: " + e.getMessage());
        } catch (IllegalArgumentException e) {
            System.out.println("Caught IllegalArgumentException: " + e.getMessage());
        }

        try {
            throwsRuntimeException();
        } catch (RuntimeException e) {
            System.out.println("Caught runtime exception: " + e.getMessage());
        }
    }

    // Method with finally block
    public static void methodWithFinally() {
        try {
            System.out.println("In try block");
            throwsRuntimeException();
        } catch (RuntimeException e) {
            System.out.println("In catch block: " + e.getMessage());
        } finally {
            System.out.println("In finally block - always executes");
        }
    }

    // Method with multiple catch blocks and rethrow
    public static void complexExceptionHandling() throws Exception {
        try {
            throwsSpecificExceptions(2);
        } catch (NumberFormatException e) {
            System.out.println("Rethrowing NumberFormatException as generic Exception");
            throw new Exception("Wrapped exception", e);
        } catch (IllegalArgumentException e) {
            System.out.println("Handling IllegalArgumentException locally");
        }
    }

    // Method demonstrating exception inheritance
    static class CustomException extends Exception {
        public CustomException(String message) {
            super(message);
        }
    }

    public static void throwsCustomException() throws CustomException {
        throw new CustomException("This is a custom exception");
    }

    public static void main(String[] args) {
        System.out.println("=== Testing basic exception handling ===");
        handleExceptions();

        System.out.println("\n=== Testing finally block ===");
        methodWithFinally();

        System.out.println("\n=== Testing complex exception handling ===");
        try {
            complexExceptionHandling();
        } catch (Exception e) {
            System.out.println("Caught rethrown exception: " + e.getMessage());
            if (e.getCause() != null) {
                System.out.println("Caused by: " + e.getCause().getMessage());
            }
        }

        System.out.println("\n=== Testing custom exception ===");
        try {
            throwsCustomException();
        } catch (CustomException e) {
            System.out.println("Caught custom exception: " + e.getMessage());
        }

        System.out.println("\n=== Testing uncaught exception ===");
        try {
            throwsSpecificExceptions(1);
        } catch (NumberFormatException e) {
            System.out.println("This won't catch IllegalArgumentException");
        } catch (IllegalArgumentException e) {
            System.out.println("Caught the right exception: " + e.getMessage());
        }
    }
}

