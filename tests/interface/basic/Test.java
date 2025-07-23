/** Test basic interface declaration, implementation, and usage */
interface BasicInterface {
    void method1();
    int method2(String param);
}

class BasicImplementation implements BasicInterface {
    public void method1() {
        System.out.println("BasicImplementation.method1 called");
    }

    public int method2(String param) {
        System.out.println("BasicImplementation.method2 called with: " + param);
        return param.length();
    }
}

public class Test {
    public static void main(String[] args) {
        System.out.println("=== Basic Interface Test ===");

        BasicImplementation impl = new BasicImplementation();
        impl.method1();
        int result = impl.method2("test");
        System.out.println("method2 returned: " + result);

        // Test interface reference
        BasicInterface ref = new BasicImplementation();
        ref.method1();
        int result2 = ref.method2("interface ref");
        System.out.println("method2 via interface returned: " + result2);

        // Test instanceof with interfaces
        System.out.println("impl instanceof BasicInterface: " + (impl instanceof BasicInterface));
        System.out.println("ref instanceof BasicInterface: " + (ref instanceof BasicInterface));
        System.out.println("ref instanceof BasicImplementation: " + (ref instanceof BasicImplementation));
    }
}
