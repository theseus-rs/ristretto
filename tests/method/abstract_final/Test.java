/** Test abstract method behavior and implementation. */
public class Test {
    static abstract class AbstractParent {
        public abstract void abstractMethod();

        public void concreteMethod() {
            System.out.println("Concrete method in abstract class");
        }

        public final void finalMethod() {
            System.out.println("Final method cannot be overridden");
        }
    }

    static class ConcreteChild extends AbstractParent {
        @Override
        public void abstractMethod() {
            System.out.println("Abstract method implemented in concrete class");
        }

        @Override
        public void concreteMethod() {
            System.out.println("Concrete method overridden in child");
        }

        // Cannot override final method - would cause compilation error
        // public void finalMethod() { ... }
    }

    static abstract class AbstractChild extends AbstractParent {
        // Can leave abstract method unimplemented
        public void additionalMethod() {
            System.out.println("Additional method in abstract child");
        }
    }

    static class ConcreteGrandChild extends AbstractChild {
        @Override
        public void abstractMethod() {
            System.out.println("Abstract method implemented in grandchild");
        }
    }

    public static void main(String[] args) {
        ConcreteChild child = new ConcreteChild();
        child.abstractMethod();
        child.concreteMethod();
        child.finalMethod();

        ConcreteGrandChild grandChild = new ConcreteGrandChild();
        grandChild.abstractMethod();
        grandChild.additionalMethod();
        grandChild.finalMethod();

        AbstractParent parentRef = new ConcreteChild();
        parentRef.abstractMethod();
        parentRef.concreteMethod();
        parentRef.finalMethod();
    }
}

