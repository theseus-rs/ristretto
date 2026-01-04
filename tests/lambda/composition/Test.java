/** Test lambda composition and chaining. */
import java.util.function.*;

public class Test {
    public static void main(String[] args) {
        System.out.println("=== Lambda Composition Tests ===");

        // Function andThen
        System.out.println("--- Function andThen ---");
        Function<String, String> trim = String::trim;
        Function<String, String> toUpper = String::toUpperCase;
        Function<String, String> addBrackets = s -> "[" + s + "]";

        Function<String, String> composed1 = trim.andThen(toUpper);
        System.out.println("trim.andThen(toUpper): " + composed1.apply("  hello  "));

        Function<String, String> composed2 = trim.andThen(toUpper).andThen(addBrackets);
        System.out.println("trim.andThen(toUpper).andThen(addBrackets): " + composed2.apply("  hello  "));

        // Function compose (opposite direction)
        System.out.println("--- Function compose ---");
        Function<String, String> composed3 = addBrackets.compose(toUpper).compose(trim);
        System.out.println("addBrackets.compose(toUpper).compose(trim): " + composed3.apply("  hello  "));

        // Predicate and/or/negate
        System.out.println("--- Predicate Combinators ---");
        Predicate<String> notEmpty = s -> !s.isEmpty();
        Predicate<String> notNull = s -> s != null;
        Predicate<String> isLong = s -> s.length() > 5;
        Predicate<String> startsWithH = s -> s.startsWith("H") || s.startsWith("h");

        Predicate<String> validString = notNull.and(notEmpty);
        System.out.println("validString('hello'): " + validString.test("hello"));
        System.out.println("validString(''): " + validString.test(""));

        Predicate<String> startsWithHOrLong = startsWithH.or(isLong);
        System.out.println("startsWithHOrLong('hello'): " + startsWithHOrLong.test("hello"));
        System.out.println("startsWithHOrLong('world!'): " + startsWithHOrLong.test("world!"));
        System.out.println("startsWithHOrLong('abc'): " + startsWithHOrLong.test("abc"));

        Predicate<String> isEmpty = notEmpty.negate();
        System.out.println("isEmpty(''): " + isEmpty.test(""));
        System.out.println("isEmpty('hello'): " + isEmpty.test("hello"));

        // Complex predicate chain
        Predicate<String> complex = notNull.and(notEmpty).and(isLong.or(startsWithH));
        System.out.println("complex('hi'): " + complex.test("hi"));
        System.out.println("complex('hello world'): " + complex.test("hello world"));
        System.out.println("complex('abcdef'): " + complex.test("abcdef"));

        // Consumer andThen
        System.out.println("--- Consumer andThen ---");
        Consumer<String> print = s -> System.out.println("  Print: " + s);
        Consumer<String> printUpper = s -> System.out.println("  Upper: " + s.toUpperCase());
        Consumer<String> printLength = s -> System.out.println("  Length: " + s.length());

        Consumer<String> combinedConsumer = print.andThen(printUpper).andThen(printLength);
        combinedConsumer.accept("hello");

        // BiFunction andThen
        System.out.println("--- BiFunction andThen ---");
        BiFunction<Integer, Integer, Integer> add = (a, b) -> a + b;
        Function<Integer, String> toStr = n -> "Result: " + n;

        BiFunction<Integer, Integer, String> addThenStr = add.andThen(toStr);
        System.out.println(addThenStr.apply(5, 3));

        // UnaryOperator chaining
        System.out.println("--- UnaryOperator Chaining ---");
        UnaryOperator<Integer> double_ = n -> n * 2;
        UnaryOperator<Integer> addTen = n -> n + 10;
        UnaryOperator<Integer> square = n -> n * n;

        Function<Integer, Integer> chain = double_.andThen(addTen).andThen(square);
        System.out.println("double(5), addTen, square: " + chain.apply(5)); // ((5*2)+10)^2 = 400

        // Building pipeline
        System.out.println("--- Pipeline Building ---");
        Function<String, String> pipeline = Function.identity();
        pipeline = pipeline.andThen(String::trim);
        pipeline = pipeline.andThen(String::toLowerCase);
        pipeline = pipeline.andThen(s -> s.replace(" ", "_"));
        pipeline = pipeline.andThen(s -> "[" + s + "]");
        System.out.println("Pipeline result: " + pipeline.apply("  Hello World  "));

        // Identity function
        System.out.println("--- Identity Function ---");
        Function<String, String> id = Function.identity();
        System.out.println("Identity: " + id.apply("unchanged"));

        // Composing with identity
        Function<String, String> withId = id.andThen(toUpper);
        System.out.println("Id then upper: " + withId.apply("hello"));

        // IntUnaryOperator chaining
        System.out.println("--- IntUnaryOperator Chaining ---");
        IntUnaryOperator intDouble = n -> n * 2;
        IntUnaryOperator intAddOne = n -> n + 1;

        IntUnaryOperator intChain = intDouble.andThen(intAddOne);
        System.out.println("intDouble(5).andThen(intAddOne): " + intChain.applyAsInt(5));

        IntUnaryOperator intComposeChain = intAddOne.compose(intDouble);
        System.out.println("intAddOne.compose(intDouble)(5): " + intComposeChain.applyAsInt(5));

        // BiConsumer andThen
        System.out.println("--- BiConsumer andThen ---");
        BiConsumer<String, Integer> biPrint = (s, n) -> System.out.println("  Value: " + s + ", Count: " + n);
        BiConsumer<String, Integer> biPrintReverse = (s, n) -> System.out.println("  Reverse: " + new StringBuilder(s).reverse() + ", Double: " + (n * 2));

        BiConsumer<String, Integer> biCombined = biPrint.andThen(biPrintReverse);
        biCombined.accept("hello", 3);

        // Composing different types
        System.out.println("--- Type-changing Composition ---");
        Function<String, Integer> strToLen = String::length;
        Function<Integer, Double> intToDouble = n -> n * 1.5;
        Function<Double, String> doubleToStr = d -> "Final: " + d;

        Function<String, String> fullPipeline = strToLen.andThen(intToDouble).andThen(doubleToStr);
        System.out.println(fullPipeline.apply("hello"));

        System.out.println("=== End Lambda Composition Tests ===");
    }
}
