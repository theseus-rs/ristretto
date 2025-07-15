/** Test constructor methods and object initialization. */
public class Test {
    static class BasicClass {
        private int value;
        private String name;

        // Default constructor
        public BasicClass() {
            this.value = 0;
            this.name = "default";
            System.out.println("Default constructor called: " + this);
        }

        // Parameterized constructor
        public BasicClass(int value) {
            this.value = value;
            this.name = "value_" + value;
            System.out.println("Int constructor called: " + this);
        }

        // Constructor with multiple parameters
        public BasicClass(int value, String name) {
            this.value = value;
            this.name = name;
            System.out.println("Full constructor called: " + this);
        }

        // Copy constructor
        public BasicClass(BasicClass other) {
            this.value = other.value;
            this.name = other.name + "_copy";
            System.out.println("Copy constructor called: " + this);
        }

        @Override
        public String toString() {
            return "BasicClass{value=" + value + ", name='" + name + "'}";
        }
    }

    static class InheritanceClass extends BasicClass {
        private double extra;

        // Constructor calling super()
        public InheritanceClass() {
            super(); // Calls parent default constructor
            this.extra = 0.0;
            System.out.println("Child default constructor called");
        }

        // Constructor calling super with parameters
        public InheritanceClass(int value, double extra) {
            super(value); // Calls parent parameterized constructor
            this.extra = extra;
            System.out.println("Child parameterized constructor called");
        }

        // Constructor with this() call
        public InheritanceClass(double extra) {
            this(); // Calls this class's default constructor
            this.extra = extra;
            System.out.println("Child this() constructor called");
        }

        @Override
        public String toString() {
            return super.toString() + " + extra=" + extra;
        }
    }

    static class InitializationBlocks {
        private int instanceVar;
        private static int staticVar;

        // Static initialization block
        static {
            staticVar = 100;
            System.out.println("Static initialization block executed: staticVar = " + staticVar);
        }

        // Instance initialization block
        {
            instanceVar = 50;
            System.out.println("Instance initialization block executed: instanceVar = " + instanceVar);
        }

        public InitializationBlocks() {
            System.out.println("Constructor executed after instance block");
        }

        public InitializationBlocks(int value) {
            this.instanceVar = value;
            System.out.println("Parameterized constructor, instanceVar set to: " + instanceVar);
        }
    }

    static class ConstructorChaining {
        private int a, b, c;

        public ConstructorChaining(int a) {
            this(a, 0);
            System.out.println("Constructor with 1 parameter");
        }

        public ConstructorChaining(int a, int b) {
            this(a, b, 0);
            System.out.println("Constructor with 2 parameters");
        }

        public ConstructorChaining(int a, int b, int c) {
            this.a = a;
            this.b = b;
            this.c = c;
            System.out.println("Constructor with 3 parameters: a=" + a + ", b=" + b + ", c=" + c);
        }

        @Override
        public String toString() {
            return "ConstructorChaining{a=" + a + ", b=" + b + ", c=" + c + "}";
        }
    }

    public static void main(String[] args) {
        System.out.println("=== Testing basic constructors ===");
        BasicClass obj1 = new BasicClass();
        BasicClass obj2 = new BasicClass(42);
        BasicClass obj3 = new BasicClass(99, "custom");
        BasicClass obj4 = new BasicClass(obj2);

        System.out.println("\n=== Testing inheritance constructors ===");
        InheritanceClass child1 = new InheritanceClass();
        InheritanceClass child2 = new InheritanceClass(10, 3.14);
        InheritanceClass child3 = new InheritanceClass(2.71);

        System.out.println("\n=== Testing initialization blocks ===");
        System.out.println("Creating first InitializationBlocks object:");
        InitializationBlocks init1 = new InitializationBlocks();
        System.out.println("Creating second InitializationBlocks object:");
        InitializationBlocks init2 = new InitializationBlocks(75);

        System.out.println("\n=== Testing constructor chaining ===");
        ConstructorChaining chain1 = new ConstructorChaining(1);
        System.out.println("Result: " + chain1);

        ConstructorChaining chain2 = new ConstructorChaining(1, 2);
        System.out.println("Result: " + chain2);

        ConstructorChaining chain3 = new ConstructorChaining(1, 2, 3);
        System.out.println("Result: " + chain3);
    }
}

