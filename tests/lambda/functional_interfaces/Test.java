/** Test built-in functional interfaces from java.util.function. */
import java.util.function.*;

public class Test {
    public static void main(String[] args) {
        System.out.println("=== Built-in Functional Interfaces Tests ===");

        // Predicate<T> - takes T, returns boolean
        System.out.println("--- Predicate ---");
        Predicate<String> isEmpty = String::isEmpty;
        Predicate<String> isLong = s -> s.length() > 5;
        System.out.println("isEmpty(''): " + isEmpty.test(""));
        System.out.println("isEmpty('hello'): " + isEmpty.test("hello"));
        System.out.println("isLong('hello'): " + isLong.test("hello"));
        System.out.println("isLong('hello world'): " + isLong.test("hello world"));

        // Predicate combinators
        Predicate<String> notEmpty = isEmpty.negate();
        System.out.println("notEmpty('hello'): " + notEmpty.test("hello"));

        Predicate<String> notEmptyAndLong = notEmpty.and(isLong);
        System.out.println("notEmptyAndLong('hi'): " + notEmptyAndLong.test("hi"));
        System.out.println("notEmptyAndLong('hello world'): " + notEmptyAndLong.test("hello world"));

        Predicate<String> emptyOrLong = isEmpty.or(isLong);
        System.out.println("emptyOrLong(''): " + emptyOrLong.test(""));
        System.out.println("emptyOrLong('hello world'): " + emptyOrLong.test("hello world"));
        System.out.println("emptyOrLong('hi'): " + emptyOrLong.test("hi"));

        // BiPredicate<T, U>
        System.out.println("--- BiPredicate ---");
        BiPredicate<String, String> startsWith = String::startsWith;
        System.out.println("startsWith('hello', 'he'): " + startsWith.test("hello", "he"));
        System.out.println("startsWith('hello', 'wo'): " + startsWith.test("hello", "wo"));

        // Function<T, R> - takes T, returns R
        System.out.println("--- Function ---");
        Function<String, Integer> length = String::length;
        Function<String, String> toUpper = String::toUpperCase;
        System.out.println("length('hello'): " + length.apply("hello"));
        System.out.println("toUpper('hello'): " + toUpper.apply("hello"));

        // Function composition
        Function<String, Integer> upperThenLength = toUpper.andThen(length);
        System.out.println("upperThenLength('hello'): " + upperThenLength.apply("hello"));

        Function<String, String> trimFirst = s -> s.trim();
        Function<String, String> trimThenUpper = trimFirst.andThen(toUpper);
        System.out.println("trimThenUpper('  hello  '): " + trimThenUpper.apply("  hello  "));

        // BiFunction<T, U, R>
        System.out.println("--- BiFunction ---");
        BiFunction<String, String, String> concat = (a, b) -> a + b;
        System.out.println("concat('hello', ' world'): " + concat.apply("hello", " world"));

        BiFunction<Integer, Integer, Integer> add = (a, b) -> a + b;
        System.out.println("add(5, 3): " + add.apply(5, 3));

        // Consumer<T> - takes T, returns void
        System.out.println("--- Consumer ---");
        Consumer<String> print = System.out::println;
        print.accept("Consumer output");

        Consumer<String> printUpper = s -> System.out.println(s.toUpperCase());
        Consumer<String> printBoth = print.andThen(printUpper);
        printBoth.accept("test");

        // BiConsumer<T, U>
        System.out.println("--- BiConsumer ---");
        BiConsumer<String, Integer> printWithCount = (s, n) -> {
            for (int i = 0; i < n; i++) {
                System.out.println("  " + i + ": " + s);
            }
        };
        printWithCount.accept("repeated", 3);

        // Supplier<T> - returns T
        System.out.println("--- Supplier ---");
        Supplier<String> hello = () -> "Hello, World!";
        System.out.println("supplier: " + hello.get());

        Supplier<Long> timestamp = System::currentTimeMillis;
        long t1 = timestamp.get();
        System.out.println("timestamp obtained: " + (t1 > 0));

        // UnaryOperator<T> - extends Function<T, T>
        System.out.println("--- UnaryOperator ---");
        UnaryOperator<String> addBrackets = s -> "[" + s + "]";
        System.out.println("addBrackets('hello'): " + addBrackets.apply("hello"));

        UnaryOperator<Integer> double_ = n -> n * 2;
        System.out.println("double(5): " + double_.apply(5));

        // BinaryOperator<T> - extends BiFunction<T, T, T>
        System.out.println("--- BinaryOperator ---");
        BinaryOperator<Integer> sum = (a, b) -> a + b;
        BinaryOperator<Integer> max = (a, b) -> a > b ? a : b;
        System.out.println("sum(5, 3): " + sum.apply(5, 3));
        System.out.println("max(5, 3): " + max.apply(5, 3));

        // Primitive specializations
        System.out.println("--- Primitive Specializations ---");

        // IntPredicate
        IntPredicate isPositive = n -> n > 0;
        System.out.println("isPositive(5): " + isPositive.test(5));
        System.out.println("isPositive(-3): " + isPositive.test(-3));

        // IntFunction
        IntFunction<String> intToString = n -> "Number: " + n;
        System.out.println(intToString.apply(42));

        // IntConsumer
        System.out.println("IntConsumer output:");
        IntConsumer printInt = n -> System.out.println("  Int: " + n);
        printInt.accept(123);

        // IntSupplier
        IntSupplier randomInt = () -> 42;
        System.out.println("IntSupplier: " + randomInt.getAsInt());

        // IntUnaryOperator
        IntUnaryOperator square = n -> n * n;
        System.out.println("square(7): " + square.applyAsInt(7));

        // IntBinaryOperator
        IntBinaryOperator multiply = (a, b) -> a * b;
        System.out.println("multiply(4, 5): " + multiply.applyAsInt(4, 5));

        // ToIntFunction
        ToIntFunction<String> toLength = String::length;
        System.out.println("toLength('hello'): " + toLength.applyAsInt("hello"));

        // LongPredicate, DoublePredicate
        LongPredicate isLongPositive = n -> n > 0L;
        System.out.println("isLongPositive(100L): " + isLongPositive.test(100L));

        DoublePredicate isFinite = Double::isFinite;
        System.out.println("isFinite(3.14): " + isFinite.test(3.14));
        System.out.println("isFinite(Infinity): " + isFinite.test(Double.POSITIVE_INFINITY));

        // DoubleUnaryOperator
        DoubleUnaryOperator sqrt = Math::sqrt;
        System.out.println("sqrt(16.0): " + sqrt.applyAsDouble(16.0));

        // ObjIntConsumer
        System.out.println("--- ObjIntConsumer ---");
        ObjIntConsumer<String> printStringNTimes = (s, n) -> {
            for (int i = 0; i < n; i++) {
                System.out.println("  " + s);
            }
        };
        printStringNTimes.accept("repeat", 2);

        System.out.println("=== End Built-in Functional Interfaces Tests ===");
    }
}
