/** Test method initialization order and class loading behavior. */
public class Test {
    static {
        System.out.println("Test class static initializer");
    }

    {
        System.out.println("Test class instance initializer");
    }

    static class Parent {
        static {
            System.out.println("Parent static initializer");
        }

        {
            System.out.println("Parent instance initializer");
        }

        public Parent() {
            System.out.println("Parent constructor");
            virtualMethod(); // Call virtual method from constructor
        }

        public void virtualMethod() {
            System.out.println("Parent virtualMethod");
        }

        public static void staticMethod() {
            System.out.println("Parent staticMethod");
        }
    }

    static class Child extends Parent {
        private int childField = initializeField();

        static {
            System.out.println("Child static initializer");
        }

        {
            System.out.println("Child instance initializer");
        }

        public Child() {
            System.out.println("Child constructor");
        }

        @Override
        public void virtualMethod() {
            System.out.println("Child virtualMethod (field value: " + childField + ")");
        }

        public static void staticMethod() {
            System.out.println("Child staticMethod");
        }

        private int initializeField() {
            System.out.println("Child field initializer");
            return 42;
        }
    }

    // Class with complex initialization order
    static class ComplexInit {
        private static final int STATIC_FINAL = initStaticFinal();
        private static int staticField = initStaticField();

        private final int instanceFinal = initInstanceFinal();
        private int instanceField = initInstanceField();

        static {
            System.out.println("ComplexInit static block 1");
            staticField = 200;
        }

        {
            System.out.println("ComplexInit instance block 1");
            instanceField = 300;
        }

        static {
            System.out.println("ComplexInit static block 2");
        }

        {
            System.out.println("ComplexInit instance block 2");
        }

        public ComplexInit() {
            System.out.println("ComplexInit constructor (static: " + staticField + ", instance: " + instanceField + ", final: " + instanceFinal + ")");
        }

        private static int initStaticFinal() {
            System.out.println("Initializing static final field");
            return 100;
        }

        private static int initStaticField() {
            System.out.println("Initializing static field");
            return 150;
        }

        private int initInstanceFinal() {
            System.out.println("Initializing instance final field");
            return 250;
        }

        private int initInstanceField() {
            System.out.println("Initializing instance field");
            return 350;
        }
    }

    // Class demonstrating lazy loading
    static class LazyLoaded {
        static {
            System.out.println("LazyLoaded class initialized");
        }

        public static void doSomething() {
            System.out.println("LazyLoaded.doSomething() called");
        }
    }

    // Method that triggers class loading
    public static void triggerClassLoading() {
        System.out.println("\n=== Testing Class Loading Order ===");
        System.out.println("About to reference Child class...");
        Child.staticMethod(); // This triggers loading of Child and Parent classes

        System.out.println("\nAbout to create Child instance...");
        Child child = new Child();

        System.out.println("\n=== Testing Complex Initialization ===");
        ComplexInit complex = new ComplexInit();

        System.out.println("\n=== Testing Lazy Class Loading ===");
        System.out.println("LazyLoaded class not yet loaded...");
        LazyLoaded.doSomething(); // This triggers loading
    }

    // Method demonstrating method lookup during initialization
    public static void testMethodLookupDuringInit() {
        System.out.println("\n=== Testing Method Lookup During Initialization ===");
        System.out.println("Creating Child - note virtual method call from Parent constructor:");
        new Child();
    }

    public static void main(String[] args) {
        System.out.println("=== Method Initialization and Class Loading Test ===");

        triggerClassLoading();
        testMethodLookupDuringInit();

        System.out.println("\n=== Summary of Initialization Order ===");
        System.out.println("1. Static final fields (in declaration order)");
        System.out.println("2. Static field initializers and static blocks (in declaration order)");
        System.out.println("3. Instance final fields (in declaration order)");
        System.out.println("4. Instance field initializers and instance blocks (in declaration order)");
        System.out.println("5. Constructor body");
        System.out.println("6. Parent class initialization happens before child class");
        System.out.println("7. Virtual method dispatch works even during construction");
    }
}

