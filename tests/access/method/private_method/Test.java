/** Tests private method access modifier - private methods can only be called within the same class */
public class Test {
    public static void main(String[] args) {
        // Test private method access
        MethodTestClass obj = new MethodTestClass();

        // Private method cannot be called directly from outside the class
        // obj.privateMethod(); // This would cause a compile error

        // Access private method through public method
        obj.callPrivateMethod();

        System.out.println("Private method test passed");
    }
}

class MethodTestClass {
    private void privateMethod() {
        System.out.println("Private method called");
    }

    private static void staticPrivateMethod() {
        System.out.println("Static private method called");
    }

    public void callPrivateMethod() {
        // Private method can be called within the same class
        privateMethod();
        staticPrivateMethod();
        System.out.println("Private methods called from within same class");
    }

    private String privateMethodWithReturn() {
        return "Private method return value";
    }
}
