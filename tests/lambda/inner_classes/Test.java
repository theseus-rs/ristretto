/** Test lambdas in inner and anonymous classes. */
import java.util.function.*;

public class Test {
    private String outerField = "outer";

    // Static nested class with lambdas
    static class StaticNested {
        private String nestedField = "staticNested";

        public void useLambda() {
            Supplier<String> s = () -> nestedField;
            System.out.println("StaticNested lambda: " + s.get());
        }
    }

    // Inner class with lambdas
    class Inner {
        private String innerField = "inner";

        public void useLambda() {
            // Lambda can access outer class
            Supplier<String> s = () -> outerField + " " + innerField;
            System.out.println("Inner lambda: " + s.get());

            // Access outer this
            Supplier<Test> getOuter = () -> Test.this;
            System.out.println("Outer this: " + (getOuter.get().outerField));
        }

        public void shadowingTest() {
            String innerField = "shadowed";
            Supplier<String> s = () -> {
                // this.innerField refers to Inner.innerField
                // local innerField shadows it
                return "Local: " + innerField + ", Instance: " + this.innerField;
            };
            System.out.println("Shadowing: " + s.get());
        }
    }

    // Method with local class using lambdas
    public void localClassTest() {
        String localVar = "localVar";

        class LocalClass {
            private String localClassField = "localClassField";

            public void useLambda() {
                Supplier<String> s = () -> {
                    return outerField + " " + localVar + " " + localClassField;
                };
                System.out.println("LocalClass lambda: " + s.get());
            }
        }

        LocalClass lc = new LocalClass();
        lc.useLambda();
    }

    // Anonymous class using lambdas
    public void anonymousClassTest() {
        String methodLocal = "methodLocal";

        Runnable anon = new Runnable() {
            private String anonField = "anonField";

            @Override
            public void run() {
                // Lambda inside anonymous class
                Supplier<String> s = () -> {
                    return outerField + " " + methodLocal + " " + anonField;
                };
                System.out.println("Anonymous class lambda: " + s.get());

                // Nested lambda in anonymous class
                Consumer<String> c = str -> {
                    Function<String, String> f = t -> t.toUpperCase() + " " + anonField;
                    System.out.println("Nested in anon: " + f.apply(str));
                };
                c.accept("hello");
            }
        };
        anon.run();
    }

    // Lambda returning anonymous class
    public void lambdaReturningAnonymous() {
        Supplier<Runnable> supplier = () -> new Runnable() {
            @Override
            public void run() {
                System.out.println("Anonymous from lambda, outer: " + outerField);
            }
        };
        supplier.get().run();
    }

    // Lambda vs anonymous class comparison
    public void lambdaVsAnonymous() {
        String capturedVar = "captured";

        // Lambda implementation
        Runnable lambda = () -> System.out.println("Lambda: " + capturedVar);

        // Anonymous class implementation
        Runnable anon = new Runnable() {
            @Override
            public void run() {
                System.out.println("Anonymous: " + capturedVar);
            }
        };

        System.out.println("Lambda class: " + lambda.getClass().getSimpleName());
        System.out.println("Anonymous class: " + anon.getClass().getSimpleName());
        lambda.run();
        anon.run();
    }

    // Lambda in enum
    enum Operation {
        ADD((a, b) -> a + b),
        SUBTRACT((a, b) -> a - b),
        MULTIPLY((a, b) -> a * b);

        private final IntBinaryOperator op;

        Operation(IntBinaryOperator op) {
            this.op = op;
        }

        public int apply(int a, int b) {
            return op.applyAsInt(a, b);
        }
    }

    public static void main(String[] args) {
        System.out.println("=== Lambda in Inner/Anonymous Classes Tests ===");

        Test test = new Test();

        // Static nested class
        System.out.println("--- Static Nested Class ---");
        StaticNested sn = new StaticNested();
        sn.useLambda();

        // Inner class
        System.out.println("--- Inner Class ---");
        Test.Inner inner = test.new Inner();
        inner.useLambda();
        inner.shadowingTest();

        // Local class
        System.out.println("--- Local Class ---");
        test.localClassTest();

        // Anonymous class
        System.out.println("--- Anonymous Class ---");
        test.anonymousClassTest();

        // Lambda returning anonymous class
        System.out.println("--- Lambda Returning Anonymous ---");
        test.lambdaReturningAnonymous();

        // Lambda vs anonymous
        System.out.println("--- Lambda vs Anonymous Comparison ---");
        test.lambdaVsAnonymous();

        // Enum with lambdas
        System.out.println("--- Enum with Lambdas ---");
        System.out.println("ADD(5, 3): " + Operation.ADD.apply(5, 3));
        System.out.println("SUBTRACT(5, 3): " + Operation.SUBTRACT.apply(5, 3));
        System.out.println("MULTIPLY(5, 3): " + Operation.MULTIPLY.apply(5, 3));

        // Lambda in anonymous class constructor
        System.out.println("--- Lambda in Anonymous Constructor ---");
        new Object() {
            private String field = "anonymousField";
            {
                Supplier<String> s = () -> field;
                System.out.println("Initializer lambda: " + s.get());
            }
        };

        // Deeply nested
        System.out.println("--- Deeply Nested ---");
        test.deeplyNested();

        System.out.println("=== End Inner/Anonymous Classes Tests ===");
    }

    public void deeplyNested() {
        class Level1 {
            String l1 = "L1";

            class Level2 {
                String l2 = "L2";

                void test() {
                    class Level3 {
                        String l3 = "L3";

                        void test() {
                            Supplier<String> s = () -> {
                                return outerField + " " + l1 + " " + l2 + " " + l3;
                            };
                            System.out.println("Deep: " + s.get());
                        }
                    }
                    new Level3().test();
                }
            }
        }
        new Level1().new Level2().test();
    }
}
