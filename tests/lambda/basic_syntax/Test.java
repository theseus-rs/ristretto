/** Test basic lambda syntax and variations. */
public class Test {
    @FunctionalInterface
    interface NoParam {
        void run();
    }

    @FunctionalInterface
    interface SingleParam {
        void accept(String s);
    }

    @FunctionalInterface
    interface TwoParams {
        int combine(int a, int b);
    }

    @FunctionalInterface
    interface WithReturn {
        String transform(String input);
    }

    public static void main(String[] args) {
        System.out.println("=== Basic Lambda Syntax Tests ===");

        // No parameters, empty body
        NoParam noOp = () -> {};
        noOp.run();
        System.out.println("No-op lambda executed");

        // No parameters, single expression
        NoParam simple = () -> System.out.println("Simple lambda");
        simple.run();

        // No parameters, block body
        NoParam block = () -> {
            System.out.println("Block body line 1");
            System.out.println("Block body line 2");
        };
        block.run();

        // Single parameter without parentheses
        SingleParam withoutParens = s -> System.out.println("Value: " + s);
        withoutParens.accept("no parens");

        // Single parameter with parentheses
        SingleParam withParens = (s) -> System.out.println("Value: " + s);
        withParens.accept("with parens");

        // Single parameter with explicit type
        SingleParam explicitType = (String s) -> System.out.println("Value: " + s);
        explicitType.accept("explicit type");

        // Two parameters
        TwoParams add = (a, b) -> a + b;
        System.out.println("Add result: " + add.combine(5, 3));

        // Two parameters with explicit types
        TwoParams multiply = (int a, int b) -> a * b;
        System.out.println("Multiply result: " + multiply.combine(5, 3));

        // Two parameters with block body
        TwoParams complexOp = (a, b) -> {
            int temp = a * 2;
            int result = temp + b;
            return result;
        };
        System.out.println("Complex op result: " + complexOp.combine(5, 3));

        // Lambda with return
        WithReturn upper = input -> input.toUpperCase();
        System.out.println("Upper: " + upper.transform("hello"));

        // Lambda with return and block
        WithReturn wrap = input -> {
            return "[" + input + "]";
        };
        System.out.println("Wrapped: " + wrap.transform("hello"));

        // Nested lambdas
        NoParam outer = () -> {
            NoParam inner = () -> System.out.println("Inner lambda");
            System.out.println("Outer lambda");
            inner.run();
        };
        outer.run();

        System.out.println("=== End Basic Lambda Syntax Tests ===");
    }
}
