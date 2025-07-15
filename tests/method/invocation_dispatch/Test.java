/** Test method invocation and dispatch behavior. */
public class Test {
    static class Parent {
        public void virtualMethod() {
            System.out.println("Parent virtual method");
        }

        public static void staticMethod() {
            System.out.println("Parent static method");
        }

        public final void finalMethod() {
            System.out.println("Parent final method");
        }

        private void privateMethod() {
            System.out.println("Parent private method");
        }

        public void callPrivate() {
            privateMethod();
        }
    }

    static class Child extends Parent {
        @Override
        public void virtualMethod() {
            System.out.println("Child virtual method");
            super.virtualMethod(); // Call parent implementation
        }

        public static void staticMethod() {
            System.out.println("Child static method");
        }

        // Cannot override final method
        // public void finalMethod() { ... } // Would cause compilation error

        // This is a new method, not an override
        private void privateMethod() {
            System.out.println("Child private method (not override)");
        }

        public void callChildPrivate() {
            privateMethod(); // Calls child's private method
        }
    }

    static class GrandChild extends Child {
        @Override
        public void virtualMethod() {
            System.out.println("GrandChild virtual method");
            super.virtualMethod(); // Calls Child's implementation
        }
    }

    // Interface for testing interface dispatch
    interface TestInterface {
        void interfaceMethod();

        default void defaultMethod() {
            System.out.println("Interface default method");
        }
    }

    static class InterfaceImpl implements TestInterface {
        @Override
        public void interfaceMethod() {
            System.out.println("Interface method implementation");
        }

        @Override
        public void defaultMethod() {
            System.out.println("Overridden default method");
            TestInterface.super.defaultMethod(); // Call interface default
        }
    }

    // Test method dispatch with different call types
    public static void testMethodDispatch() {
        System.out.println("=== Virtual Method Dispatch ===");
        Parent parent = new Parent();
        Child child = new Child();
        GrandChild grandChild = new GrandChild();

        // Direct calls
        parent.virtualMethod();
        child.virtualMethod();
        grandChild.virtualMethod();

        System.out.println("\n=== Polymorphic Dispatch ===");
        Parent[] objects = {new Parent(), new Child(), new GrandChild()};
        for (Parent obj : objects) {
            obj.virtualMethod(); // Calls the most derived implementation
        }

        System.out.println("\n=== Static Method Dispatch ===");
        Parent.staticMethod();
        Child.staticMethod();

        // Static methods are not polymorphic
        Parent parentRef = new Child();
        parentRef.staticMethod(); // Calls Parent.staticMethod()

        System.out.println("\n=== Final Method Dispatch ===");
        parentRef.finalMethod(); // Final methods cannot be overridden
        child.finalMethod();

        System.out.println("\n=== Private Method Behavior ===");
        parent.callPrivate();
        child.callPrivate(); // Calls Parent's private method
        child.callChildPrivate(); // Calls Child's private method
    }

    public static void testInterfaceDispatch() {
        System.out.println("\n=== Interface Method Dispatch ===");
        TestInterface impl = new InterfaceImpl();
        impl.interfaceMethod();
        impl.defaultMethod();

        InterfaceImpl concrete = new InterfaceImpl();
        concrete.interfaceMethod();
        concrete.defaultMethod();
    }

    // Test method resolution with overloading
    static class OverloadTest {
        public void method(Object obj) {
            System.out.println("method(Object): " + obj);
        }

        public void method(String str) {
            System.out.println("method(String): " + str);
        }

        public void method(Integer num) {
            System.out.println("method(Integer): " + num);
        }

        public void method(int num) {
            System.out.println("method(int): " + num);
        }
    }

    public static void testMethodResolution() {
        System.out.println("\n=== Method Resolution and Overloading ===");
        OverloadTest test = new OverloadTest();

        test.method("Hello");           // Calls method(String)
        test.method(42);                // Calls method(int)
        test.method(Integer.valueOf(42)); // Calls method(Integer)
        test.method((Object)"Hello");   // Calls method(Object)
        test.method((Object)42);        // Calls method(Object) - boxing then upcast
    }

    public static void main(String[] args) {
        testMethodDispatch();
        testInterfaceDispatch();
        testMethodResolution();
    }
}

