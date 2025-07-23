/** Test static methods in interfaces */
interface StaticMethodInterface {
    static void staticMethod() {
        System.out.println("StaticMethodInterface.staticMethod");
    }

    static String staticWithReturn(String input) {
        System.out.println("StaticMethodInterface.staticWithReturn called with: " + input);
        return "Static: " + input;
    }

    static void staticCallsOtherStatic() {
        System.out.println("staticCallsOtherStatic calling staticMethod:");
        staticMethod();
    }

    // Instance method for comparison
    void instanceMethod();
}

interface AnotherStaticInterface {
    static void staticMethod() {
        System.out.println("AnotherStaticInterface.staticMethod");
    }
}

class StaticImplementation implements StaticMethodInterface, AnotherStaticInterface {
    public void instanceMethod() {
        System.out.println("StaticImplementation.instanceMethod");
    }

    public static void classStaticMethod() {
        System.out.println("StaticImplementation.classStaticMethod");
    }
}

public class Test {
    public static void main(String[] args) {
        System.out.println("=== Interface Static Methods Test ===");

        // Call static methods directly on interfaces
        StaticMethodInterface.staticMethod();
        String result = StaticMethodInterface.staticWithReturn("test");
        System.out.println("Returned: " + result);
        StaticMethodInterface.staticCallsOtherStatic();

        // Call static method on another interface
        AnotherStaticInterface.staticMethod();

        // Static methods are not inherited by implementing classes
        StaticImplementation impl = new StaticImplementation();
        impl.instanceMethod();
        StaticImplementation.classStaticMethod();

        // Test that static methods are not accessible via interface references
        StaticMethodInterface ref = impl;
        ref.instanceMethod();
        // ref.staticMethod(); // This would be a compile error

        // But can still call via interface name
        StaticMethodInterface.staticMethod();

        System.out.println("Static methods called successfully");
    }
}

