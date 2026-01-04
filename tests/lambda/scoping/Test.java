/** Test lambda scoping and shadowing rules. */
import java.util.function.*;

public class Test {
    private static String staticField = "static";
    private String instanceField = "instance";
    private int value = 100;

    // Method with parameter that could be shadowed
    public void testScoping(String param) {
        String local = "local";
        int value = 50; // Shadows instance field

        // Lambda can access all enclosing scopes
        Runnable r = () -> {
            System.out.println("Static field: " + staticField);
            System.out.println("Instance field: " + instanceField);
            System.out.println("Method parameter: " + param);
            System.out.println("Local variable: " + local);
            System.out.println("Shadowed value: " + value);
            System.out.println("Instance value via this: " + this.value);
        };
        r.run();
    }

    public static void main(String[] args) {
        System.out.println("=== Lambda Scoping Tests ===");

        // Basic scoping
        System.out.println("--- Basic Scoping ---");
        Test test = new Test();
        test.testScoping("paramValue");

        // Lambda parameter scoping
        System.out.println("--- Lambda Parameter Scoping ---");
        Consumer<String> c1 = s -> System.out.println("Lambda param: " + s);
        c1.accept("value1");

        BiConsumer<String, Integer> c2 = (s, n) -> {
            System.out.println("First param: " + s);
            System.out.println("Second param: " + n);
        };
        c2.accept("hello", 42);

        // Nested lambda scoping
        System.out.println("--- Nested Lambda Scoping ---");
        int outer = 10;
        Function<Integer, Function<Integer, Integer>> nested = a -> {
            // 'a' is in scope here
            return b -> {
                // both 'a', 'b', and 'outer' are in scope here
                return outer + a + b;
            };
        };
        Function<Integer, Integer> partial = nested.apply(5);
        System.out.println("Nested result: " + partial.apply(3)); // 10 + 5 + 3 = 18

        // Variable scoping from enclosing method
        System.out.println("--- Enclosing Method Scope ---");
        testMethodScope();

        // Static context scoping
        System.out.println("--- Static Context ---");
        Supplier<String> staticAccess = () -> staticField;
        System.out.println("Static access: " + staticAccess.get());

        // Instance context scoping
        System.out.println("--- Instance Context ---");
        test.instanceContextTest();

        // Lambda in loop with scoping
        System.out.println("--- Loop Scoping ---");
        Runnable[] runners = new Runnable[3];
        for (int i = 0; i < 3; i++) {
            final int captured = i;
            runners[i] = () -> System.out.println("Loop iteration: " + captured);
        }
        for (Runnable r : runners) {
            r.run();
        }

        // Scoping with enhanced for
        System.out.println("--- Enhanced For Scoping ---");
        String[] items = {"a", "b", "c"};
        java.util.List<Supplier<String>> suppliers = new java.util.ArrayList<>();
        for (String item : items) {
            suppliers.add(() -> item);
        }
        for (Supplier<String> s : suppliers) {
            System.out.println("Enhanced for captured: " + s.get());
        }

        // Effectively final in different scopes
        System.out.println("--- Effectively Final ---");
        String effectivelyFinal = "unchanged";
        Consumer<String> usesEffectivelyFinal = s ->
            System.out.println(effectivelyFinal + " " + s);
        usesEffectivelyFinal.accept("suffix");

        // Array allows modification while being effectively final
        System.out.println("--- Array Modification ---");
        int[] counter = {0};
        Runnable incrementer = () -> {
            counter[0]++;
            System.out.println("Counter: " + counter[0]);
        };
        incrementer.run();
        incrementer.run();
        incrementer.run();

        // Object allows modification while being effectively final
        System.out.println("--- Object Modification ---");
        StringBuilder sb = new StringBuilder();
        Consumer<String> appender = s -> sb.append(s);
        appender.accept("hello");
        appender.accept(" ");
        appender.accept("world");
        System.out.println("StringBuilder: " + sb.toString());

        // Lambda in constructor scope
        System.out.println("--- Constructor Scope ---");
        new ScopedClass("constructorValue");

        // Lambda in initializer scope
        System.out.println("--- Initializer Scope ---");
        new InitializerClass();

        // This reference in lambda
        System.out.println("--- This Reference ---");
        test.testThisReference();

        System.out.println("=== End Lambda Scoping Tests ===");
    }

    private static void testMethodScope() {
        int methodLocal = 42;
        String methodString = "methodScope";

        Consumer<Integer> c = n -> {
            System.out.println("Method local: " + methodLocal);
            System.out.println("Method string: " + methodString);
            System.out.println("Lambda param: " + n);
        };
        c.accept(100);
    }

    private void instanceContextTest() {
        Supplier<String> s = () -> {
            return "Instance field: " + instanceField + ", value: " + value;
        };
        System.out.println(s.get());
    }

    private void testThisReference() {
        Supplier<Test> getThis = () -> this;
        System.out.println("This reference: " + (getThis.get() == this));

        Consumer<String> useThis = s -> {
            this.instanceField = s;
            System.out.println("Modified instanceField: " + this.instanceField);
        };
        useThis.accept("modified");
    }

    static class ScopedClass {
        private String field;
        private Supplier<String> supplier;

        public ScopedClass(String param) {
            this.field = param;
            // Lambda captures constructor parameter
            this.supplier = () -> "Constructor captured: " + param;
            System.out.println(supplier.get());
        }
    }

    static class InitializerClass {
        private String initField = "initValue";
        private Supplier<String> supplier;

        {
            // Instance initializer
            String initLocal = "initLocal";
            supplier = () -> initField + " " + initLocal;
        }

        public InitializerClass() {
            System.out.println("From initializer: " + supplier.get());
        }
    }
}
