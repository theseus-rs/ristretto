/** Test interface constants and nested interfaces */
interface ConstantInterface {
    // All fields in interfaces are implicitly public, static, and final
    String STRING_CONSTANT = "Interface Constant";
    int INT_CONSTANT = 42;
    double DOUBLE_CONSTANT = 3.14159;

    // Nested interface
    interface NestedInterface {
        String NESTED_CONSTANT = "Nested Interface Constant";
        void nestedMethod();
    }

    // Nested static class (implicitly static in interfaces)
    class NestedClass {
        public void nestedClassMethod() {
            System.out.println("NestedClass.nestedClassMethod");
            System.out.println("Accessing outer constant: " + STRING_CONSTANT);
        }
    }

    void method();
}

class ConstantImplementation implements ConstantInterface {
    public void method() {
        System.out.println("ConstantImplementation.method");
        System.out.println("STRING_CONSTANT: " + STRING_CONSTANT);
        System.out.println("INT_CONSTANT: " + INT_CONSTANT);
        System.out.println("DOUBLE_CONSTANT: " + DOUBLE_CONSTANT);
    }
}

class NestedImplementation implements ConstantInterface.NestedInterface {
    public void nestedMethod() {
        System.out.println("NestedImplementation.nestedMethod");
        System.out.println("NESTED_CONSTANT: " + NESTED_CONSTANT);
        System.out.println("Outer constant: " + ConstantInterface.STRING_CONSTANT);
    }
}

interface AnotherInterface {
    String STRING_CONSTANT = "Another Constant"; // Same name, different value

    interface NestedInterface {
        String NESTED_CONSTANT = "Another Nested Constant";
        void anotherNestedMethod();
    }
}

class MultipleConstantUser implements ConstantInterface, AnotherInterface {
    public void method() {
        System.out.println("MultipleConstantUser.method");
        // Need to qualify constants when there's ambiguity
        System.out.println("ConstantInterface.STRING_CONSTANT: " + ConstantInterface.STRING_CONSTANT);
        System.out.println("AnotherInterface.STRING_CONSTANT: " + AnotherInterface.STRING_CONSTANT);
    }
}

public class Test {
    public static void main(String[] args) {
        System.out.println("=== Interface Constants and Nested Interfaces Test ===");

        // Test constants access
        ConstantImplementation impl = new ConstantImplementation();
        impl.method();

        // Direct access to constants
        System.out.println("Direct access to ConstantInterface.STRING_CONSTANT: " + ConstantInterface.STRING_CONSTANT);
        System.out.println("Direct access to ConstantInterface.INT_CONSTANT: " + ConstantInterface.INT_CONSTANT);

        // Test nested interface implementation
        NestedImplementation nested = new NestedImplementation();
        nested.nestedMethod();

        // Test nested class
        ConstantInterface.NestedClass nestedClass = new ConstantInterface.NestedClass();
        nestedClass.nestedClassMethod();

        // Test multiple constant interfaces
        MultipleConstantUser multiUser = new MultipleConstantUser();
        multiUser.method();

        // Test instanceof with nested types
        System.out.println("nested instanceof ConstantInterface.NestedInterface: " +
                          (nested instanceof ConstantInterface.NestedInterface));
        System.out.println("nestedClass instanceof ConstantInterface.NestedClass: " +
                          (nestedClass instanceof ConstantInterface.NestedClass));

        // Test constant immutability (they are final)
        System.out.println("Constants are final and cannot be changed");
    }
}
