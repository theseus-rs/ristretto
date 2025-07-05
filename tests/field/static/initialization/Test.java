/** Tests static field initialization in a class. */
public class Test {
    static class A {
        static int x = init();

        static int init() {
            System.out.println("A.x initialized");
            return 17;
        }
    }

    public static void main(String[] args) {
        System.out.println("A.x = ");
        System.out.println(A.x);
    }
}
