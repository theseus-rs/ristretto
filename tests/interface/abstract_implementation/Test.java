/** Test abstract classes implementing interfaces */
interface CompleteInterface {
    void method1();
    void method2();
    int method3(String param);
    default void defaultMethod() {
        System.out.println("CompleteInterface.defaultMethod");
    }
}

abstract class PartialImplementation implements CompleteInterface {
    // Implement some methods
    public void method1() {
        System.out.println("PartialImplementation.method1");
    }

    public int method3(String param) {
        System.out.println("PartialImplementation.method3 with: " + param);
        return param.hashCode();
    }

    // Leave method2 abstract - must be implemented by subclasses

    // Add abstract method of our own
    abstract void abstractMethod();
}

class ConcreteImplementation extends PartialImplementation {
    public void method2() {
        System.out.println("ConcreteImplementation.method2");
    }

    void abstractMethod() {
        System.out.println("ConcreteImplementation.abstractMethod");
    }
}

abstract class AnotherAbstract implements CompleteInterface {
    // Implements all interface methods but adds abstract method
    public void method1() {
        System.out.println("AnotherAbstract.method1");
    }

    public void method2() {
        System.out.println("AnotherAbstract.method2");
    }

    public int method3(String param) {
        System.out.println("AnotherAbstract.method3 with: " + param);
        return 42;
    }

    abstract void additionalAbstractMethod();
}

class FinalImplementation extends AnotherAbstract {
    void additionalAbstractMethod() {
        System.out.println("FinalImplementation.additionalAbstractMethod");
    }
}

public class Test {
    public static void main(String[] args) {
        System.out.println("=== Abstract Classes with Interfaces Test ===");

        ConcreteImplementation concrete = new ConcreteImplementation();
        concrete.method1();
        concrete.method2();
        int result1 = concrete.method3("test");
        System.out.println("method3 returned: " + result1);
        concrete.defaultMethod();
        concrete.abstractMethod();

        FinalImplementation finalImpl = new FinalImplementation();
        finalImpl.method1();
        finalImpl.method2();
        int result2 = finalImpl.method3("final");
        System.out.println("method3 returned: " + result2);
        finalImpl.defaultMethod();
        finalImpl.additionalAbstractMethod();

        // Test instanceof with abstract classes and interfaces
        System.out.println("concrete instanceof CompleteInterface: " + (concrete instanceof CompleteInterface));
        System.out.println("concrete instanceof PartialImplementation: " + (concrete instanceof PartialImplementation));
        System.out.println("finalImpl instanceof CompleteInterface: " + (finalImpl instanceof CompleteInterface));
        System.out.println("finalImpl instanceof AnotherAbstract: " + (finalImpl instanceof AnotherAbstract));

        // Test interface references
        CompleteInterface ref1 = concrete;
        CompleteInterface ref2 = finalImpl;

        ref1.method1();
        ref2.method1();
    }
}
