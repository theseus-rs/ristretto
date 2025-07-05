/**
 * Test static initializer in a class.
 *
 * This is a simple test to ensure that static initializers are executed when the class is loaded, and that they can
 * initialize static fields.
 */
public class Test {
    static class A {
        static int x;
        static {
            x = 123;
            System.out.println("A.<clinit>");
        }
    }

    public static void main(String[] args) {
        System.out.print("A.x = ");
        System.out.println(A.x);
    }
}
