/** Tests public method access modifier - public methods can be called from anywhere */
public class Test {
    public static void main(String[] args) {
        // Test public method access
        MethodTestClass obj = new MethodTestClass();

        // Public method can be called from anywhere
        obj.publicMethod();

        // Public static method can be called without instance
        MethodTestClass.staticPublicMethod();

        System.out.println("Public method test passed");
    }
}

class MethodTestClass {
    public void publicMethod() {
        System.out.println("Public method called");
    }

    public static void staticPublicMethod() {
        System.out.println("Static public method called");
    }

    public String publicMethodWithReturn() {
        return "Public method return value";
    }
}
