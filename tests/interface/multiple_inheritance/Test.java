/** Test multiple interface inheritance and implementation */
interface InterfaceA {
    void methodA();
    default void defaultA() {
        System.out.println("InterfaceA.defaultA");
    }
}

interface InterfaceB {
    void methodB();
    default void defaultB() {
        System.out.println("InterfaceB.defaultB");
    }
}

interface InterfaceC extends InterfaceA, InterfaceB {
    void methodC();
    default void defaultC() {
        System.out.println("InterfaceC.defaultC");
    }
}

class MultipleImplementation implements InterfaceA, InterfaceB {
    public void methodA() {
        System.out.println("MultipleImplementation.methodA");
    }

    public void methodB() {
        System.out.println("MultipleImplementation.methodB");
    }
}

class ExtendedImplementation implements InterfaceC {
    public void methodA() {
        System.out.println("ExtendedImplementation.methodA");
    }

    public void methodB() {
        System.out.println("ExtendedImplementation.methodB");
    }

    public void methodC() {
        System.out.println("ExtendedImplementation.methodC");
    }
}

public class Test {
    public static void main(String[] args) {
        System.out.println("=== Multiple Interface Inheritance Test ===");

        MultipleImplementation multi = new MultipleImplementation();
        multi.methodA();
        multi.methodB();
        multi.defaultA();
        multi.defaultB();

        System.out.println("multi instanceof InterfaceA: " + (multi instanceof InterfaceA));
        System.out.println("multi instanceof InterfaceB: " + (multi instanceof InterfaceB));
        System.out.println("multi instanceof InterfaceC: " + (multi instanceof InterfaceC));

        ExtendedImplementation extended = new ExtendedImplementation();
        extended.methodA();
        extended.methodB();
        extended.methodC();
        extended.defaultA();
        extended.defaultB();
        extended.defaultC();

        System.out.println("extended instanceof InterfaceA: " + (extended instanceof InterfaceA));
        System.out.println("extended instanceof InterfaceB: " + (extended instanceof InterfaceB));
        System.out.println("extended instanceof InterfaceC: " + (extended instanceof InterfaceC));

        // Test interface references
        InterfaceA refA = extended;
        InterfaceB refB = extended;
        InterfaceC refC = extended;

        refA.methodA();
        refB.methodB();
        refC.methodC();
    }
}
