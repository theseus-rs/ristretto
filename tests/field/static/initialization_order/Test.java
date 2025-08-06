/** Test the order of static field initialization in a class hierarchy. */
public class Test {
    static class A {
        static int x = print("A.x");
    }
    static class B extends A {
        static int y = print("B.y");
    }

    static int print(String message) {
        System.out.println(message + " initialized");
        return 0;
    }

    public static void main(String[] args) {
        System.out.println("B.y = " + B.y);
    }
}
