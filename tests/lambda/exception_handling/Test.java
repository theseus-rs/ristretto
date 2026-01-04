/** Test exception handling in lambdas. */
import java.util.function.*;
import java.io.*;

public class Test {
    // Functional interface that throws checked exception
    @FunctionalInterface
    interface ThrowingSupplier<T, E extends Exception> {
        T get() throws E;
    }

    @FunctionalInterface
    interface ThrowingFunction<T, R, E extends Exception> {
        R apply(T t) throws E;
    }

    @FunctionalInterface
    interface ThrowingRunnable<E extends Exception> {
        void run() throws E;
    }

    // Wrapper method to convert throwing function to regular function
    public static <T, R> Function<T, R> wrap(ThrowingFunction<T, R, Exception> f) {
        return t -> {
            try {
                return f.apply(t);
            } catch (Exception e) {
                throw new RuntimeException(e);
            }
        };
    }

    // Custom exception
    static class CustomException extends Exception {
        public CustomException(String message) {
            super(message);
        }
    }

    public static void main(String[] args) {
        System.out.println("=== Exception Handling in Lambdas Tests ===");

        // Runtime exceptions in lambdas
        System.out.println("--- Runtime Exceptions ---");
        Function<String, Integer> parser = s -> {
            try {
                return Integer.parseInt(s);
            } catch (NumberFormatException e) {
                System.out.println("Caught in lambda: " + e.getMessage());
                return -1;
            }
        };
        System.out.println("Parse '42': " + parser.apply("42"));
        System.out.println("Parse 'abc': " + parser.apply("abc"));

        // Propagating runtime exceptions
        System.out.println("--- Propagating Runtime Exception ---");
        Function<Integer, Integer> divider = n -> {
            if (n == 0) {
                throw new ArithmeticException("Division by zero in lambda");
            }
            return 100 / n;
        };
        try {
            System.out.println("100 / 5 = " + divider.apply(5));
            System.out.println("100 / 0 = " + divider.apply(0));
        } catch (ArithmeticException e) {
            System.out.println("Caught outside: " + e.getMessage());
        }

        // Null pointer in lambda
        System.out.println("--- NullPointerException ---");
        Function<String, String> upper = s -> s.toUpperCase();
        try {
            System.out.println("Upper 'hello': " + upper.apply("hello"));
            System.out.println("Upper null: " + upper.apply(null));
        } catch (NullPointerException e) {
            System.out.println("Caught NPE: " + (e.getMessage() != null ? e.getMessage() : "null"));
        }

        // Using throwing functional interface
        System.out.println("--- Throwing Functional Interface ---");
        ThrowingSupplier<String, IOException> fileReader = () -> {
            throw new IOException("Simulated IO error");
        };
        try {
            String content = fileReader.get();
            System.out.println("Content: " + content);
        } catch (IOException e) {
            System.out.println("Caught IOException: " + e.getMessage());
        }

        // Wrapping checked exceptions
        System.out.println("--- Wrapped Checked Exceptions ---");
        ThrowingFunction<String, Integer, Exception> riskyParser = s -> {
            if (s == null) throw new NullPointerException("Null input");
            if (s.isEmpty()) throw new IllegalArgumentException("Empty input");
            return Integer.parseInt(s);
        };

        Function<String, Integer> safeParser = wrap(riskyParser);
        try {
            System.out.println("Safe parse '123': " + safeParser.apply("123"));
            System.out.println("Safe parse '': " + safeParser.apply(""));
        } catch (RuntimeException e) {
            System.out.println("Caught wrapped: " + e.getCause().getMessage());
        }

        // Custom exception in lambda
        System.out.println("--- Custom Exception ---");
        ThrowingRunnable<CustomException> customThrower = () -> {
            throw new CustomException("Custom error occurred");
        };
        try {
            customThrower.run();
        } catch (CustomException e) {
            System.out.println("Caught custom: " + e.getMessage());
        }

        // Exception in method reference
        System.out.println("--- Exception in Method Reference ---");
        Function<String, Integer> parseRef = Integer::parseInt;
        try {
            System.out.println("Parse 'xyz': " + parseRef.apply("xyz"));
        } catch (NumberFormatException e) {
            System.out.println("Method ref exception: " + e.getClass().getSimpleName());
        }

        // Finally equivalent in lambda
        System.out.println("--- Finally in Lambda ---");
        Supplier<String> withFinally = () -> {
            try {
                System.out.println("  Try block");
                return "success";
            } finally {
                System.out.println("  Finally block");
            }
        };
        System.out.println("Result: " + withFinally.get());

        // Exception with resources pattern
        System.out.println("--- Resource Pattern ---");
        Consumer<String> resourceUser = name -> {
            System.out.println("  Opening resource: " + name);
            try {
                System.out.println("  Using resource: " + name);
                if (name.equals("bad")) {
                    throw new RuntimeException("Resource error");
                }
            } finally {
                System.out.println("  Closing resource: " + name);
            }
        };
        resourceUser.accept("good");
        try {
            resourceUser.accept("bad");
        } catch (RuntimeException e) {
            System.out.println("Caught: " + e.getMessage());
        }

        // Nested try-catch in lambda
        System.out.println("--- Nested Try-Catch ---");
        Function<String, String> nestedHandler = s -> {
            try {
                try {
                    return s.substring(0, 10);
                } catch (IndexOutOfBoundsException e) {
                    System.out.println("  Inner catch: substring failed");
                    return s.toUpperCase();
                }
            } catch (NullPointerException e) {
                System.out.println("  Outer catch: null string");
                return "NULL";
            }
        };
        System.out.println("Result for 'hello': " + nestedHandler.apply("hello"));
        System.out.println("Result for 'hello world extended': " + nestedHandler.apply("hello world extended"));

        // Multiple exception types
        System.out.println("--- Multiple Exception Types ---");
        BiFunction<String, Integer, Character> multiExcept = (s, i) -> {
            try {
                return s.charAt(i);
            } catch (NullPointerException | IndexOutOfBoundsException e) {
                System.out.println("  Caught multi: " + e.getClass().getSimpleName());
                return '?';
            }
        };
        System.out.println("charAt('hello', 1): " + multiExcept.apply("hello", 1));
        System.out.println("charAt('hello', 10): " + multiExcept.apply("hello", 10));

        System.out.println("=== End Exception Handling Tests ===");
    }
}
