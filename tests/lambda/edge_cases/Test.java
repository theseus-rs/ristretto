/** Test lambda edge cases and corner cases. */
import java.util.function.*;

public class Test {
    // Test with Object methods
    @FunctionalInterface
    interface FunctionalWithObjectMethods {
        void doSomething();

        // These don't count - they're from Object
        boolean equals(Object o);
        int hashCode();
        String toString();
    }

    public static void main(String[] args) {
        System.out.println("=== Lambda Edge Cases Tests ===");

        // Empty lambda body
        System.out.println("--- Empty Lambda ---");
        Runnable empty = () -> {};
        empty.run();
        System.out.println("Empty lambda executed");

        // Lambda with only comment (still valid)
        Runnable withComment = () -> {
            // This is a comment
        };
        withComment.run();
        System.out.println("Comment-only lambda executed");

        // Void method expression
        System.out.println("--- Void Expressions ---");
        Consumer<String> voidExpr = s -> System.out.println(s);
        voidExpr.accept("Void expression");

        // Non-void expression in void context
        Consumer<String> nonVoidInVoid = s -> s.length(); // result discarded
        nonVoidInVoid.accept("Result discarded");
        System.out.println("Non-void in void context: OK");

        // Lambda returning null
        System.out.println("--- Returning Null ---");
        Supplier<String> returnsNull = () -> null;
        System.out.println("Returns null: " + returnsNull.get());

        Function<String, String> maybeNull = s -> s.isEmpty() ? null : s;
        System.out.println("Maybe null (empty): " + maybeNull.apply(""));
        System.out.println("Maybe null (hello): " + maybeNull.apply("hello"));

        // Lambda with unused parameter
        System.out.println("--- Unused Parameter ---");
        BiFunction<String, Integer, String> unusedSecond = (s, n) -> s;
        System.out.println("Unused param: " + unusedSecond.apply("value", 999));

        // Lambda parameter shadowing is NOT allowed - can't test directly
        // but we can test that lambda params are distinct from outer scope
        System.out.println("--- Parameter Scoping ---");
        String outerVar = "outer";
        Consumer<String> consumer = outerVar2 -> System.out.println("Lambda param: " + outerVar2 + ", outer: " + outerVar);
        consumer.accept("inner");

        // Lambda with this reference
        System.out.println("--- This in Lambda ---");
        Test test = new Test();
        test.testThisInLambda();

        // Recursive via array holder
        System.out.println("--- Recursive Lambda ---");
        final IntUnaryOperator[] fib = new IntUnaryOperator[1];
        fib[0] = n -> n <= 1 ? n : fib[0].applyAsInt(n - 1) + fib[0].applyAsInt(n - 2);
        System.out.println("Fibonacci(10): " + fib[0].applyAsInt(10));

        // Lambda with array creation
        System.out.println("--- Array Creation ---");
        IntFunction<int[]> createArray = int[]::new;
        int[] arr = createArray.apply(5);
        System.out.println("Created array length: " + arr.length);

        IntFunction<String[][]> create2DArray = String[][]::new;
        String[][] arr2d = create2DArray.apply(3);
        System.out.println("Created 2D array length: " + arr2d.length);

        // Lambda with varargs (through method reference)
        System.out.println("--- Varargs ---");
        Function<String[], String> joinVarargs = parts -> String.join(", ", parts);
        System.out.println("Joined: " + joinVarargs.apply(new String[]{"a", "b", "c"}));

        // Lambda with auto-boxing/unboxing
        System.out.println("--- Auto-boxing ---");
        Function<Integer, Integer> boxed = n -> n * 2;
        IntUnaryOperator primitive = n -> n * 2;

        // Auto-box int to Integer
        Integer result1 = boxed.apply(5);
        // Auto-unbox for primitive
        int result2 = primitive.applyAsInt(5);
        System.out.println("Boxed result: " + result1);
        System.out.println("Primitive result: " + result2);

        // Very long lambda chain
        System.out.println("--- Long Chain ---");
        Function<Integer, Integer> chain = Function.identity();
        for (int i = 0; i < 10; i++) {
            final int increment = i;
            chain = chain.andThen(n -> n + increment);
        }
        System.out.println("Chain result (0+0+1+...+9): " + chain.apply(0));

        // Lambda in ternary
        System.out.println("--- Lambda in Ternary ---");
        boolean condition = true;
        Runnable r1 = condition ? () -> System.out.println("True branch") : () -> System.out.println("False branch");
        r1.run();

        // Lambda with diamond operator
        System.out.println("--- Diamond Operator ---");
        java.util.Map<String, Function<String, String>> map = new java.util.HashMap<>();
        map.put("upper", String::toUpperCase);
        map.put("lower", String::toLowerCase);
        System.out.println("From map: " + map.get("upper").apply("hello"));

        // Lambda with Object methods interface
        System.out.println("--- Object Methods Interface ---");
        FunctionalWithObjectMethods fom = () -> System.out.println("FunctionalWithObjectMethods");
        fom.doSomething();

        // Lambda assigned to Object
        System.out.println("--- Lambda as Object ---");
        Object obj = (Runnable) () -> System.out.println("Lambda as Object");
        ((Runnable) obj).run();

        // Lambda type from method
        System.out.println("--- Lambda from Method ---");
        Runnable fromMethod = getRunnable();
        fromMethod.run();

        // Lambda in static initializer
        System.out.println("--- Static Initializer ---");
        StaticInitTest.run();

        // Lambda with exception in expression
        System.out.println("--- Exception Expression ---");
        Supplier<Integer> mayThrow = () -> {
            throw new RuntimeException("Expected");
        };
        try {
            mayThrow.get();
        } catch (RuntimeException e) {
            System.out.println("Caught: " + e.getMessage());
        }

        // Lambda returning itself (infinite type avoided)
        System.out.println("--- Self-referential ---");
        final Supplier<Object>[] self = new Supplier[1];
        self[0] = () -> self[0];
        System.out.println("Self-reference works: " + (self[0].get() == self[0]));

        System.out.println("=== End Lambda Edge Cases Tests ===");
    }

    private void testThisInLambda() {
        Supplier<Test> getThis = () -> this;
        System.out.println("This is same: " + (getThis.get() == this));
    }

    private static Runnable getRunnable() {
        return () -> System.out.println("From method lambda");
    }

    static class StaticInitTest {
        private static final Runnable INIT_LAMBDA;

        static {
            String message = "Static initializer lambda";
            INIT_LAMBDA = () -> System.out.println(message);
        }

        public static void run() {
            INIT_LAMBDA.run();
        }
    }
}
