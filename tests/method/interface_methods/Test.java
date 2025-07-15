/** Test interface method behavior including default and static methods. */
public class Test {
    interface BasicInterface {
        void abstractMethod();

        default void defaultMethod() {
            System.out.println("Default method in interface");
        }

        static void staticMethod() {
            System.out.println("Static method in interface");
        }
    }

    interface ExtendedInterface extends BasicInterface {
        void additionalMethod();

        @Override
        default void defaultMethod() {
            System.out.println("Overridden default method in extended interface");
        }
    }

    static class Implementation implements ExtendedInterface {
        @Override
        public void abstractMethod() {
            System.out.println("Abstract method implemented");
        }

        @Override
        public void additionalMethod() {
            System.out.println("Additional method implemented");
        }

        // Can override default method
        @Override
        public void defaultMethod() {
            System.out.println("Default method overridden in class");
            ExtendedInterface.super.defaultMethod(); // Call interface default
        }
    }

    static class MinimalImplementation implements BasicInterface {
        @Override
        public void abstractMethod() {
            System.out.println("Minimal implementation");
        }
        // Uses default method from interface
    }

    public static void main(String[] args) {
        Implementation impl = new Implementation();
        impl.abstractMethod();
        impl.additionalMethod();
        impl.defaultMethod();

        MinimalImplementation minimal = new MinimalImplementation();
        minimal.abstractMethod();
        minimal.defaultMethod(); // Uses interface default

        BasicInterface.staticMethod(); // Call static method on interface

        BasicInterface ref = new Implementation();
        ref.abstractMethod();
        ref.defaultMethod();
    }
}

