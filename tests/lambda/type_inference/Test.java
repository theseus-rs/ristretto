/** Test lambda type inference and target typing. */
import java.util.function.*;
import java.util.*;

public class Test {
    // Overloaded methods to test target typing
    public static void accept(Consumer<String> c) {
        System.out.println("Consumer version");
        c.accept("test");
    }

    public static void accept(Function<String, String> f) {
        System.out.println("Function version");
        System.out.println("Result: " + f.apply("test"));
    }

    public static void accept(Runnable r) {
        System.out.println("Runnable version");
        r.run();
    }

    // Generic method for type inference
    public static <T> T process(Supplier<T> supplier) {
        return supplier.get();
    }

    public static <T, R> R transform(T input, Function<T, R> transformer) {
        return transformer.apply(input);
    }

    public static void main(String[] args) {
        System.out.println("=== Lambda Type Inference Tests ===");

        // Basic type inference
        System.out.println("--- Basic Type Inference ---");
        Function<String, Integer> f1 = s -> s.length();
        Function<String, Integer> f2 = (String s) -> s.length();
        BiFunction<Integer, Integer, Integer> f3 = (a, b) -> a + b;
        BiFunction<Integer, Integer, Integer> f4 = (Integer a, Integer b) -> a + b;

        System.out.println("f1: " + f1.apply("hello"));
        System.out.println("f2: " + f2.apply("hello"));
        System.out.println("f3: " + f3.apply(5, 3));
        System.out.println("f4: " + f4.apply(5, 3));

        // Target typing with assignment
        System.out.println("--- Target Typing Assignment ---");
        Comparator<String> comp1 = (a, b) -> a.length() - b.length();
        ToIntFunction<String> len = s -> s.length();
        Predicate<String> empty = s -> s.isEmpty();

        System.out.println("comp1.compare('hi', 'hello'): " + comp1.compare("hi", "hello"));
        System.out.println("len.applyAsInt('hello'): " + len.applyAsInt("hello"));
        System.out.println("empty.test(''): " + empty.test(""));

        // Target typing with method argument
        System.out.println("--- Target Typing Method Argument ---");
        accept((Consumer<String>) s -> System.out.println("Consumed: " + s));
        accept((Function<String, String>) s -> s.toUpperCase());

        // Same lambda, different types
        System.out.println("--- Same Lambda Different Types ---");
        Object o1 = (Runnable) () -> System.out.println("I'm a Runnable");
        Object o2 = (Supplier<String>) () -> "I'm a Supplier";

        ((Runnable) o1).run();
        System.out.println(((Supplier<?>) o2).get());

        // Type inference with generics
        System.out.println("--- Generic Type Inference ---");
        String result1 = process(() -> "Hello from supplier");
        Integer result2 = process(() -> 42);
        List<String> result3 = process(() -> Arrays.asList("a", "b", "c"));

        System.out.println("Supplier String: " + result1);
        System.out.println("Supplier Integer: " + result2);
        System.out.println("Supplier List: " + result3);

        // Type inference with transform
        Integer len1 = transform("hello", String::length);
        String upper = transform("hello", String::toUpperCase);
        Double parsed = transform("3.14", Double::parseDouble);

        System.out.println("Transformed length: " + len1);
        System.out.println("Transformed upper: " + upper);
        System.out.println("Transformed parsed: " + parsed);

        // Diamond with lambdas
        System.out.println("--- Diamond with Lambdas ---");
        Map<String, Function<String, String>> operations = new HashMap<>();
        operations.put("upper", String::toUpperCase);
        operations.put("lower", String::toLowerCase);
        operations.put("reverse", s -> new StringBuilder(s).reverse().toString());

        for (Map.Entry<String, Function<String, String>> entry : operations.entrySet()) {
            System.out.println(entry.getKey() + "(hello): " + entry.getValue().apply("hello"));
        }

        // Intersection types (casting)
        System.out.println("--- Intersection Types ---");
        Runnable r = (Runnable & java.io.Serializable) () -> System.out.println("Serializable Runnable");
        r.run();
        System.out.println("Is Serializable: " + (r instanceof java.io.Serializable));

        // Void compatibility
        System.out.println("--- Void Compatibility ---");
        Consumer<String> consumer = s -> s.toUpperCase(); // result discarded
        consumer.accept("hello");
        System.out.println("Consumer with discarded result: OK");

        // Expression vs statement lambdas
        System.out.println("--- Expression vs Statement ---");
        Function<Integer, Integer> expr = n -> n * 2;
        Function<Integer, Integer> stmt = n -> { return n * 2; };
        System.out.println("Expression: " + expr.apply(5));
        System.out.println("Statement: " + stmt.apply(5));

        // Poly expressions
        System.out.println("--- Poly Expressions ---");
        // Lambda is a poly expression - its type depends on context
        Object lambda1 = (Predicate<String>) s -> s.isEmpty();
        Object lambda2 = (Function<String, Boolean>) s -> s.isEmpty();

        System.out.println("As Predicate: " + ((Predicate<String>) lambda1).test(""));
        System.out.println("As Function: " + ((Function<String, Boolean>) lambda2).apply(""));

        // Ternary with lambdas
        System.out.println("--- Ternary with Lambdas ---");
        boolean condition = true;
        Function<String, String> chosen = condition
            ? s -> s.toUpperCase()
            : s -> s.toLowerCase();
        System.out.println("Chosen (true): " + chosen.apply("Test"));

        condition = false;
        chosen = condition
            ? s -> s.toUpperCase()
            : s -> s.toLowerCase();
        System.out.println("Chosen (false): " + chosen.apply("Test"));

        // Method return type inference
        System.out.println("--- Return Type Inference ---");
        Function<String, Integer> fromMethod = createLengthFunction();
        System.out.println("From method: " + fromMethod.apply("hello"));

        System.out.println("=== End Lambda Type Inference Tests ===");
    }

    private static Function<String, Integer> createLengthFunction() {
        return s -> s.length();
    }
}
