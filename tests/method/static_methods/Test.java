/** Test static method behavior and inheritance. */
public class Test {
    static class Parent {
        public static void staticMethod() {
            System.out.println("Parent static method");
        }

        public static void parentOnlyStaticMethod() {
            System.out.println("Parent only static method");
        }

        public void instanceMethod() {
            System.out.println("Parent instance method");
        }
    }

    static class Child extends Parent {
        public static void staticMethod() {
            System.out.println("Child static method");
        }

        @Override
        public void instanceMethod() {
            System.out.println("Child instance method");
        }
    }

    public static void main(String[] args) {
        Parent.staticMethod();
        Child.staticMethod();

        Parent parentRef = new Child();
        parentRef.staticMethod(); // Calls Parent.staticMethod() - static methods are not overridden
        parentRef.instanceMethod(); // Calls Child.instanceMethod() - instance methods are overridden

        Child child = new Child();
        child.staticMethod();
        child.instanceMethod();

        Parent.parentOnlyStaticMethod();
        Child.parentOnlyStaticMethod();
    }
}
