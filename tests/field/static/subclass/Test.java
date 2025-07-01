/** Test static field access in a subclass. */
public class Test {
    static class A {
        static int x = 99;
    }

    static class B extends A {}

    public static void main(String[] args) {
        System.out.print("B.x = ");
        System.out.println(B.x);
    }
}
