/** Test lambdas with generic types. */
import java.util.function.*;

public class Test {
    // Generic functional interface
    @FunctionalInterface
    interface Transformer<T, R> {
        R transform(T input);
    }

    // Generic functional interface with multiple type params
    @FunctionalInterface
    interface BiTransformer<T, U, R> {
        R combine(T first, U second);
    }

    // Bounded type parameter
    @FunctionalInterface
    interface NumberProcessor<T extends Number> {
        double process(T number);
    }

    // Generic method that accepts functional interface
    public static <T, R> R apply(T input, Transformer<T, R> transformer) {
        return transformer.transform(input);
    }

    // Generic method with multiple type params
    public static <T, U, R> R combine(T first, U second, BiTransformer<T, U, R> combiner) {
        return combiner.combine(first, second);
    }

    // Generic holder class
    static class Box<T> {
        private T value;

        public Box(T value) {
            this.value = value;
        }

        public T get() {
            return value;
        }

        public <R> Box<R> map(Function<T, R> mapper) {
            return new Box<>(mapper.apply(value));
        }

        public void ifPresent(Consumer<T> consumer) {
            if (value != null) {
                consumer.accept(value);
            }
        }

        @Override
        public String toString() {
            return "Box[" + value + "]";
        }
    }

    public static void main(String[] args) {
        System.out.println("=== Lambda with Generics Tests ===");

        // Basic generic lambda
        System.out.println("--- Basic Generic Lambda ---");
        Transformer<String, Integer> stringToLength = s -> s.length();
        System.out.println("'hello' length: " + stringToLength.transform("hello"));

        Transformer<Integer, String> intToString = n -> "Number: " + n;
        System.out.println(intToString.transform(42));

        Transformer<String, String> identity = s -> s;
        System.out.println("Identity: " + identity.transform("unchanged"));

        // BiTransformer
        System.out.println("--- BiTransformer ---");
        BiTransformer<String, Integer, String> repeat = (s, n) -> {
            StringBuilder sb = new StringBuilder();
            for (int i = 0; i < n; i++) {
                sb.append(s);
            }
            return sb.toString();
        };
        System.out.println("Repeat 'ab' 3 times: " + repeat.combine("ab", 3));

        BiTransformer<Integer, Integer, Double> divide = (a, b) -> (double) a / b;
        System.out.println("10 / 3 = " + divide.combine(10, 3));

        // Generic method usage
        System.out.println("--- Generic Method Usage ---");
        String result1 = apply(123, n -> "Result: " + n);
        System.out.println(result1);

        Integer result2 = apply("hello world", s -> s.split(" ").length);
        System.out.println("Word count: " + result2);

        String combined = combine("Hello", 42, (s, n) -> s + " " + n);
        System.out.println("Combined: " + combined);

        // Bounded type parameter
        System.out.println("--- Bounded Type Parameter ---");
        NumberProcessor<Integer> intProcessor = n -> n.doubleValue() * 2;
        System.out.println("Int processed: " + intProcessor.process(5));

        NumberProcessor<Double> doubleProcessor = n -> n.doubleValue() * 2;
        System.out.println("Double processed: " + doubleProcessor.process(3.14));

        NumberProcessor<Long> longProcessor = n -> Math.sqrt(n.doubleValue());
        System.out.println("Long sqrt: " + longProcessor.process(16L));

        // Generic Box with lambdas
        System.out.println("--- Generic Box ---");
        Box<String> stringBox = new Box<>("hello");
        System.out.println("String box: " + stringBox);

        Box<Integer> lengthBox = stringBox.map(String::length);
        System.out.println("Mapped to length: " + lengthBox);

        Box<String> doubledBox = lengthBox.map(n -> "Length: " + (n * 2));
        System.out.println("Doubled: " + doubledBox);

        stringBox.ifPresent(s -> System.out.println("Present: " + s));

        // Chained generic operations
        System.out.println("--- Chained Operations ---");
        Box<String> chainResult = new Box<>("  HELLO WORLD  ")
            .map(String::trim)
            .map(String::toLowerCase)
            .map(s -> s.replace(" ", "_"));
        System.out.println("Chained: " + chainResult);

        // Wildcard type inference
        System.out.println("--- Type Inference ---");
        Function<Object, String> toString = Object::toString;
        System.out.println("toString(42): " + toString.apply(42));
        System.out.println("toString(true): " + toString.apply(true));

        // Generic lambda with explicit types
        BiFunction<String, String, Integer> compareLen =
            (String a, String b) -> Integer.compare(a.length(), b.length());
        System.out.println("Compare 'hello' and 'hi': " + compareLen.apply("hello", "hi"));

        // Nested generics
        System.out.println("--- Nested Generics ---");
        Function<Box<String>, Box<Integer>> boxMapper = box -> box.map(String::length);
        Box<Integer> mapped = boxMapper.apply(new Box<>("test"));
        System.out.println("Box mapped: " + mapped);

        // Generic array operations
        System.out.println("--- Generic Array ---");
        Transformer<String[], Integer> arrayLength = arr -> arr.length;
        String[] testArray = {"a", "b", "c"};
        System.out.println("Array length: " + arrayLength.transform(testArray));

        System.out.println("=== End Lambda with Generics Tests ===");
    }
}
