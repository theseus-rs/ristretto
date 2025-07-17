/** Test instanceof behavior with interfaces and inheritance */
interface TestInterface {
    void testMethod();
}

interface ExtendedInterface extends TestInterface {
    void extendedMethod();
}

interface AnotherInterface {
    void anotherMethod();
}

class BaseClass {
    public void baseMethod() {}
}

class ImplementingClass extends BaseClass implements TestInterface {
    public void testMethod() {}
}

class ExtendedImplementingClass extends ImplementingClass implements ExtendedInterface {
    public void extendedMethod() {}
}

class MultipleInterfaceClass implements TestInterface, AnotherInterface {
    public void testMethod() {}
    public void anotherMethod() {}
}

public class Test {
    public static void main(String[] args) {
        // Test interface instanceof
        ImplementingClass impl = new ImplementingClass();
        System.out.println("ImplementingClass instanceof TestInterface: " + (impl instanceof TestInterface));
        System.out.println("ImplementingClass instanceof ExtendedInterface: " + (impl instanceof ExtendedInterface));
        System.out.println("ImplementingClass instanceof AnotherInterface: " + (impl instanceof AnotherInterface));
        System.out.println("ImplementingClass instanceof BaseClass: " + (impl instanceof BaseClass));
        System.out.println("ImplementingClass instanceof Object: " + (impl instanceof Object));

        // Test extended interface implementation
        ExtendedImplementingClass extImpl = new ExtendedImplementingClass();
        System.out.println("ExtendedImplementingClass instanceof TestInterface: " + (extImpl instanceof TestInterface));
        System.out.println("ExtendedImplementingClass instanceof ExtendedInterface: " + (extImpl instanceof ExtendedInterface));
        System.out.println("ExtendedImplementingClass instanceof AnotherInterface: " + (extImpl instanceof AnotherInterface));
        System.out.println("ExtendedImplementingClass instanceof BaseClass: " + (extImpl instanceof BaseClass));
        System.out.println("ExtendedImplementingClass instanceof ImplementingClass: " + (extImpl instanceof ImplementingClass));

        // Test multiple interface implementation
        MultipleInterfaceClass multi = new MultipleInterfaceClass();
        System.out.println("MultipleInterfaceClass instanceof TestInterface: " + (multi instanceof TestInterface));
        System.out.println("MultipleInterfaceClass instanceof AnotherInterface: " + (multi instanceof AnotherInterface));
        System.out.println("MultipleInterfaceClass instanceof ExtendedInterface: " + (multi instanceof ExtendedInterface));
        System.out.println("MultipleInterfaceClass instanceof Object: " + (multi instanceof Object));

        // Test through interface references
        TestInterface interfaceRef = new ImplementingClass();
        System.out.println("TestInterface ref instanceof TestInterface: " + (interfaceRef instanceof TestInterface));
        System.out.println("TestInterface ref instanceof ImplementingClass: " + (interfaceRef instanceof ImplementingClass));
        System.out.println("TestInterface ref instanceof BaseClass: " + (interfaceRef instanceof BaseClass));
        System.out.println("TestInterface ref instanceof Object: " + (interfaceRef instanceof Object));

        // Test interface hierarchy
        ExtendedInterface extInterfaceRef = new ExtendedImplementingClass();
        System.out.println("ExtendedInterface ref instanceof TestInterface: " + (extInterfaceRef instanceof TestInterface));
        System.out.println("ExtendedInterface ref instanceof ExtendedInterface: " + (extInterfaceRef instanceof ExtendedInterface));
        System.out.println("ExtendedInterface ref instanceof ExtendedImplementingClass: " + (extInterfaceRef instanceof ExtendedImplementingClass));

        // Test base class doesn't implement interface
        BaseClass base = new BaseClass();
        System.out.println("BaseClass instanceof TestInterface: " + (base instanceof TestInterface));
        System.out.println("BaseClass instanceof Object: " + (base instanceof Object));

        // Test null interface references
        TestInterface nullInterface = null;
        System.out.println("null TestInterface instanceof TestInterface: " + (nullInterface instanceof TestInterface));
        System.out.println("null TestInterface instanceof Object: " + (nullInterface instanceof Object));
    }
}
