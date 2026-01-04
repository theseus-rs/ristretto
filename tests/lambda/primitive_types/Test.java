/** Test primitive types with lambdas. */
import java.util.function.*;

public class Test {
    public static void main(String[] args) {
        System.out.println("=== Primitive Lambdas Tests ===");

        // IntFunction family
        System.out.println("--- IntFunction Family ---");
        IntFunction<String> intToString = n -> "Number: " + n;
        System.out.println(intToString.apply(42));

        IntUnaryOperator square = n -> n * n;
        System.out.println("Square(7): " + square.applyAsInt(7));

        IntBinaryOperator add = (a, b) -> a + b;
        System.out.println("Add(5, 3): " + add.applyAsInt(5, 3));

        IntPredicate isEven = n -> n % 2 == 0;
        System.out.println("isEven(4): " + isEven.test(4));
        System.out.println("isEven(5): " + isEven.test(5));

        IntConsumer printInt = n -> System.out.println("IntConsumer: " + n);
        printInt.accept(123);

        IntSupplier randomInt = () -> 42;
        System.out.println("IntSupplier: " + randomInt.getAsInt());

        // LongFunction family
        System.out.println("--- LongFunction Family ---");
        LongFunction<String> longToString = n -> "Long: " + n;
        System.out.println(longToString.apply(123456789L));

        LongUnaryOperator doubleLong = n -> n * 2;
        System.out.println("DoubleLong(100L): " + doubleLong.applyAsLong(100L));

        LongBinaryOperator multiplyLong = (a, b) -> a * b;
        System.out.println("MultiplyLong(1000L, 1000L): " + multiplyLong.applyAsLong(1000L, 1000L));

        LongPredicate isPositiveLong = n -> n > 0;
        System.out.println("isPositiveLong(100L): " + isPositiveLong.test(100L));
        System.out.println("isPositiveLong(-100L): " + isPositiveLong.test(-100L));

        LongConsumer printLong = n -> System.out.println("LongConsumer: " + n);
        printLong.accept(999999999L);

        LongSupplier currentTime = System::currentTimeMillis;
        System.out.println("LongSupplier isPositive: " + (currentTime.getAsLong() > 0));

        // DoubleFunction family
        System.out.println("--- DoubleFunction Family ---");
        DoubleFunction<String> doubleToString = d -> String.format("Double: %.2f", d);
        System.out.println(doubleToString.apply(3.14159));

        DoubleUnaryOperator squareRoot = Math::sqrt;
        System.out.println("Sqrt(16.0): " + squareRoot.applyAsDouble(16.0));

        DoubleBinaryOperator power = Math::pow;
        System.out.println("Pow(2.0, 10.0): " + power.applyAsDouble(2.0, 10.0));

        DoublePredicate isFinite = Double::isFinite;
        System.out.println("isFinite(3.14): " + isFinite.test(3.14));
        System.out.println("isFinite(Infinity): " + isFinite.test(Double.POSITIVE_INFINITY));

        DoubleConsumer printDouble = d -> System.out.println("DoubleConsumer: " + d);
        printDouble.accept(2.71828);

        DoubleSupplier pi = () -> Math.PI;
        System.out.println("DoubleSupplier (PI): " + pi.getAsDouble());

        // ToXxxFunction
        System.out.println("--- ToXxxFunction ---");
        ToIntFunction<String> stringLength = String::length;
        System.out.println("Length('hello'): " + stringLength.applyAsInt("hello"));

        ToLongFunction<String> stringHash = s -> (long) s.hashCode();
        System.out.println("Hash('hello'): " + stringHash.applyAsLong("hello"));

        ToDoubleFunction<String> parseDouble = Double::parseDouble;
        System.out.println("Parse('3.14'): " + parseDouble.applyAsDouble("3.14"));

        // ToXxxBiFunction
        System.out.println("--- ToXxxBiFunction ---");
        ToIntBiFunction<String, String> compareLen = (a, b) -> a.length() - b.length();
        System.out.println("CompareLen('hello', 'hi'): " + compareLen.applyAsInt("hello", "hi"));

        ToLongBiFunction<Integer, Integer> multiplyToLong = (a, b) -> (long) a * b;
        System.out.println("MultiplyToLong(100000, 100000): " + multiplyToLong.applyAsLong(100000, 100000));

        ToDoubleBiFunction<Integer, Integer> divideToDouble = (a, b) -> (double) a / b;
        System.out.println("DivideToDouble(10, 3): " + divideToDouble.applyAsDouble(10, 3));

        // ObjXxxConsumer
        System.out.println("--- ObjXxxConsumer ---");
        ObjIntConsumer<String> printWithInt = (s, n) -> System.out.println(s + ": " + n);
        printWithInt.accept("Value", 42);

        ObjLongConsumer<String> printWithLong = (s, n) -> System.out.println(s + ": " + n);
        printWithLong.accept("Long value", 123456789L);

        ObjDoubleConsumer<String> printWithDouble = (s, d) -> System.out.println(s + ": " + d);
        printWithDouble.accept("Double value", 3.14159);

        // XxxToYyyFunction
        System.out.println("--- XxxToYyyFunction ---");
        IntToLongFunction intToLong = n -> (long) n * 1000000000L;
        System.out.println("IntToLong(5): " + intToLong.applyAsLong(5));

        IntToDoubleFunction intToDouble = n -> n / 2.0;
        System.out.println("IntToDouble(5): " + intToDouble.applyAsDouble(5));

        LongToIntFunction longToInt = n -> (int) (n % Integer.MAX_VALUE);
        System.out.println("LongToInt(123456789L): " + longToInt.applyAsInt(123456789L));

        LongToDoubleFunction longToDouble = n -> n / 1000.0;
        System.out.println("LongToDouble(5000L): " + longToDouble.applyAsDouble(5000L));

        DoubleToIntFunction doubleToInt = d -> (int) Math.round(d);
        System.out.println("DoubleToInt(3.7): " + doubleToInt.applyAsInt(3.7));

        DoubleToLongFunction doubleToLong = d -> (long) (d * 1000);
        System.out.println("DoubleToLong(3.14159): " + doubleToLong.applyAsLong(3.14159));

        // Chaining primitive operators
        System.out.println("--- Chaining Primitive Operators ---");
        IntUnaryOperator addOne = n -> n + 1;
        IntUnaryOperator timesTen = n -> n * 10;
        IntUnaryOperator combined = addOne.andThen(timesTen);
        System.out.println("(5 + 1) * 10 = " + combined.applyAsInt(5));

        IntUnaryOperator composed = timesTen.compose(addOne);
        System.out.println("(5 + 1) * 10 (compose) = " + composed.applyAsInt(5));

        // Predicate operations
        System.out.println("--- Predicate Operations ---");
        IntPredicate positive = n -> n > 0;
        IntPredicate lessThan100 = n -> n < 100;

        IntPredicate positiveAndSmall = positive.and(lessThan100);
        System.out.println("positiveAndSmall(50): " + positiveAndSmall.test(50));
        System.out.println("positiveAndSmall(150): " + positiveAndSmall.test(150));
        System.out.println("positiveAndSmall(-10): " + positiveAndSmall.test(-10));

        IntPredicate notPositive = positive.negate();
        System.out.println("notPositive(-5): " + notPositive.test(-5));

        IntPredicate positiveOrSmall = positive.or(lessThan100);
        System.out.println("positiveOrSmall(-50): " + positiveOrSmall.test(-50));

        // Boxing and unboxing
        System.out.println("--- Boxing/Unboxing ---");
        Function<Integer, Integer> boxedSquare = n -> n * n;
        IntFunction<Integer> mixedSquare = n -> n * n;
        IntUnaryOperator primitiveSquare = n -> n * n;

        System.out.println("Boxed: " + boxedSquare.apply(5));
        System.out.println("Mixed: " + mixedSquare.apply(5));
        System.out.println("Primitive: " + primitiveSquare.applyAsInt(5));

        System.out.println("=== End Primitive Lambdas Tests ===");
    }
}
