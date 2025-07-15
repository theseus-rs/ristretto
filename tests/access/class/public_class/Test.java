/** Tests public class access modifier - public classes can be accessed from anywhere */
public class Test {
    public static void main(String[] args) {
        // Test public class access
        PublicClass pub = new PublicClass();
        pub.testMethod();
        System.out.println("Public class test passed");
    }
}
