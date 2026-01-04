/** Test custom functional interfaces with lambdas. */
public class Test {
    // Simple functional interfaces
    @FunctionalInterface
    interface Action {
        void execute();
    }

    @FunctionalInterface
    interface Callback<T> {
        void onComplete(T result);
    }

    @FunctionalInterface
    interface Parser<T> {
        T parse(String input) throws ParseException;
    }

    @FunctionalInterface
    interface TriFunction<A, B, C, R> {
        R apply(A a, B b, C c);
    }

    @FunctionalInterface
    interface QuadFunction<A, B, C, D, R> {
        R apply(A a, B b, C c, D d);
    }

    @FunctionalInterface
    interface Validator<T> {
        ValidationResult validate(T value);
    }

    // Functional interface with default methods
    @FunctionalInterface
    interface Transformer<T, R> {
        R transform(T input);

        default <V> Transformer<T, V> andThen(Transformer<R, V> after) {
            return input -> after.transform(this.transform(input));
        }

        default <V> Transformer<V, R> compose(Transformer<V, T> before) {
            return input -> this.transform(before.transform(input));
        }
    }

    // Functional interface with static methods
    @FunctionalInterface
    interface Processor<T> {
        T process(T input);

        static <T> Processor<T> identity() {
            return t -> t;
        }

        static <T> Processor<T> chain(Processor<T> first, Processor<T> second) {
            return t -> second.process(first.process(t));
        }
    }

    // Helper classes
    static class ParseException extends Exception {
        public ParseException(String message) {
            super(message);
        }
    }

    static class ValidationResult {
        private boolean valid;
        private String message;

        public ValidationResult(boolean valid, String message) {
            this.valid = valid;
            this.message = message;
        }

        @Override
        public String toString() {
            return valid ? "Valid" : "Invalid: " + message;
        }
    }

    public static void main(String[] args) {
        System.out.println("=== Custom Functional Interfaces Tests ===");

        // Simple Action
        System.out.println("--- Action ---");
        Action greet = () -> System.out.println("Hello, World!");
        greet.execute();

        Action multiLine = () -> {
            System.out.println("Line 1");
            System.out.println("Line 2");
        };
        multiLine.execute();

        // Callback
        System.out.println("--- Callback ---");
        Callback<String> stringCallback = result -> System.out.println("Callback received: " + result);
        stringCallback.onComplete("success");

        Callback<Integer> intCallback = result -> System.out.println("Int callback: " + (result * 2));
        intCallback.onComplete(21);

        // Parser with exception
        System.out.println("--- Parser ---");
        Parser<Integer> intParser = input -> {
            try {
                return Integer.parseInt(input);
            } catch (NumberFormatException e) {
                throw new ParseException("Cannot parse: " + input);
            }
        };

        try {
            System.out.println("Parsed '42': " + intParser.parse("42"));
        } catch (ParseException e) {
            System.out.println("Error: " + e.getMessage());
        }

        try {
            System.out.println("Parsed 'abc': " + intParser.parse("abc"));
        } catch (ParseException e) {
            System.out.println("Error: " + e.getMessage());
        }

        // TriFunction
        System.out.println("--- TriFunction ---");
        TriFunction<String, Integer, Boolean, String> formatter = (name, age, active) -> {
            return String.format("%s is %d years old, active: %b", name, age, active);
        };
        System.out.println(formatter.apply("Alice", 30, true));

        TriFunction<Integer, Integer, Integer, Integer> sum3 = (a, b, c) -> a + b + c;
        System.out.println("Sum of 1, 2, 3: " + sum3.apply(1, 2, 3));

        // QuadFunction
        System.out.println("--- QuadFunction ---");
        QuadFunction<Integer, Integer, Integer, Integer, Integer> sum4 = (a, b, c, d) -> a + b + c + d;
        System.out.println("Sum of 1, 2, 3, 4: " + sum4.apply(1, 2, 3, 4));

        // Validator
        System.out.println("--- Validator ---");
        Validator<String> notEmpty = s -> {
            if (s == null || s.isEmpty()) {
                return new ValidationResult(false, "String is empty");
            }
            return new ValidationResult(true, "OK");
        };

        Validator<String> maxLength = s -> {
            if (s != null && s.length() > 10) {
                return new ValidationResult(false, "String too long");
            }
            return new ValidationResult(true, "OK");
        };

        System.out.println("Validate 'hello': " + notEmpty.validate("hello"));
        System.out.println("Validate '': " + notEmpty.validate(""));
        System.out.println("Validate 'hello world!': " + maxLength.validate("hello world!"));

        // Transformer with chaining
        System.out.println("--- Transformer ---");
        Transformer<String, String> trim = String::trim;
        Transformer<String, String> upper = String::toUpperCase;
        Transformer<String, Integer> length = String::length;

        Transformer<String, Integer> trimUpperLength = trim.andThen(upper).andThen(length);
        System.out.println("Transform '  hello  ': " + trimUpperLength.transform("  hello  "));

        Transformer<String, String> upperThenTrim = upper.compose(trim);
        System.out.println("Compose '  hello  ': " + upperThenTrim.transform("  hello  "));

        // Processor with static methods
        System.out.println("--- Processor ---");
        Processor<String> identity = Processor.identity();
        System.out.println("Identity: " + identity.process("unchanged"));

        Processor<String> addPrefix = s -> "PREFIX_" + s;
        Processor<String> addSuffix = s -> s + "_SUFFIX";
        Processor<String> chained = Processor.chain(addPrefix, addSuffix);
        System.out.println("Chained: " + chained.process("value"));

        // Generic interface with wildcard bounds
        System.out.println("--- Bounded Generics ---");
        @FunctionalInterface
        interface NumberFormatter<N extends Number> {
            String format(N number);
        }

        NumberFormatter<Integer> intFormatter = n -> String.format("Int: %d", n);
        NumberFormatter<Double> doubleFormatter = n -> String.format("Double: %.2f", n);

        System.out.println(intFormatter.format(42));
        System.out.println(doubleFormatter.format(3.14159));

        // Currying with functional interfaces
        System.out.println("--- Currying ---");
        java.util.function.Function<Integer, java.util.function.Function<Integer, Integer>> curriedAdd =
            a -> b -> a + b;
        System.out.println("Curried add(5)(3): " + curriedAdd.apply(5).apply(3));

        java.util.function.Function<Integer, java.util.function.Function<Integer, java.util.function.Function<Integer, Integer>>> curriedAdd3 =
            a -> b -> c -> a + b + c;
        System.out.println("Curried add(1)(2)(3): " + curriedAdd3.apply(1).apply(2).apply(3));

        // Functional interface with generic method
        System.out.println("--- Generic Method in Interface ---");
        @FunctionalInterface
        interface GenericMapper {
            <T, R> R map(T input, java.util.function.Function<T, R> mapper);
        }

        GenericMapper mapper = new GenericMapper() {
            @Override
            public <T, R> R map(T input, java.util.function.Function<T, R> mapper) {
                System.out.println("Mapping: " + input);
                return mapper.apply(input);
            }
        };
        System.out.println("Result: " + mapper.map("hello", String::length));

        System.out.println("=== End Custom Functional Interfaces Tests ===");
    }
}
