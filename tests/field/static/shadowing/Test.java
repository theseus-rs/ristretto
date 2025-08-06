/** Test static field shadowing in subclasses. */
public class Test {
    static class A {
        static int x = 1;
    }

    static class B extends A {
        static int x = 2;
    }

    public static void main(String[] args) {
        System.out.println("A.x = " + A.x);
        System.out.println("B.x = " + B.x);
    }
}
