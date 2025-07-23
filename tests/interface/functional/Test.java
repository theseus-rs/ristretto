/** Test functional interfaces and lambda expressions */
@FunctionalInterface
interface SimpleFunctional {
    void execute();

    // Default methods don't count toward the single abstract method
    default void defaultMethod() {
        System.out.println("SimpleFunctional.defaultMethod");
    }

    // Static methods don't count either
    static void staticMethod() {
        System.out.println("SimpleFunctional.staticMethod");
    }
}

@FunctionalInterface
interface FunctionalWithParams {
    int calculate(int a, int b);
}

@FunctionalInterface
interface FunctionalWithReturn {
    String process(String input);
}

// Not a functional interface - has multiple abstract methods
interface NotFunctional {
    void method1();
    void method2();
}

class LambdaUser {
    public void useLambdas() {
        // Simple lambda
        SimpleFunctional simple = () -> System.out.println("Lambda executed");
        simple.execute();
        simple.defaultMethod();

        // Lambda with parameters
        FunctionalWithParams calc = (a, b) -> {
            System.out.println("Calculating: " + a + " + " + b);
            return a + b;
        };
        int result = calc.calculate(5, 3);
        System.out.println("Result: " + result);

        // Lambda with return value
        FunctionalWithReturn processor = input -> {
            System.out.println("Processing: " + input);
            return input.toUpperCase();
        };
        String processed = processor.process("hello");
        System.out.println("Processed: " + processed);

        // Method reference
        FunctionalWithReturn methodRef = String::toLowerCase;
        String lower = methodRef.process("WORLD");
        System.out.println("Method ref result: " + lower);
    }
}

class TraditionalImplementation implements SimpleFunctional {
    public void execute() {
        System.out.println("Traditional implementation executed");
    }
}

public class Test {
    public static void main(String[] args) {
        System.out.println("=== Functional Interfaces and Lambdas Test ===");

        LambdaUser user = new LambdaUser();
        user.useLambdas();

        // Traditional implementation
        TraditionalImplementation traditional = new TraditionalImplementation();
        traditional.execute();
        traditional.defaultMethod();

        // Static method call
        SimpleFunctional.staticMethod();

        // Assign traditional implementation to functional interface
        SimpleFunctional funcRef = traditional;
        funcRef.execute();

        // Test instanceof with functional interfaces
        SimpleFunctional lambda = () -> System.out.println("Instance test lambda");
        System.out.println("lambda instanceof SimpleFunctional: " + (lambda instanceof SimpleFunctional));
        System.out.println("traditional instanceof SimpleFunctional: " + (traditional instanceof SimpleFunctional));

        // Test with built-in functional interfaces
        java.util.function.Predicate<String> predicate = s -> s.length() > 3;
        System.out.println("Test 'hello' length > 3: " + predicate.test("hello"));
        System.out.println("Test 'hi' length > 3: " + predicate.test("hi"));
    }
}

