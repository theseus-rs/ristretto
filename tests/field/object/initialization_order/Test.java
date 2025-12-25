/**
 * Tests instance field initialization order during object construction.
 *
 * Per JLS §12.5, the order of instance initialization is:
 * 1. Call to super(...) (recursively up to Object.<init>)
 * 2. Instance field initializers (in textual order)
 * 3. Instance initializer blocks (in textual order)
 * 4. Constructor body
 *
 * This test verifies:
 * - Zero initialization before constructors
 * - Correct superclass -> subclass ordering
 * - Field initializers execute in textual order
 * - Instance initializer blocks execute at the right time
 */
public class Test {
    static class A {
        int a1 = print("A.a1 field initializer");
        int a2;

        {
            System.out.println("A instance initializer block");
            a2 = 20;
        }

        A() {
            System.out.println("A constructor");
        }

        int print(String msg) {
            System.out.println(msg);
            return 10;
        }
    }

    static class B extends A {
        int b1 = print("B.b1 field initializer");
        int b2;

        {
            System.out.println("B instance initializer block");
            b2 = 40;
        }

        B() {
            // Implicit super() call happens first
            System.out.println("B constructor");
        }
    }

    static class C extends B {
        int c1 = print("C.c1 field initializer");

        C() {
            // Implicit super() call happens first
            System.out.println("C constructor");
        }
    }

    public static void main(String[] args) {
        System.out.println("Creating new C():");
        C obj = new C();

        System.out.println();
        System.out.println("Final field values:");
        System.out.println("obj.a1 = " + obj.a1);
        System.out.println("obj.a2 = " + obj.a2);
        System.out.println("obj.b1 = " + obj.b1);
        System.out.println("obj.b2 = " + obj.b2);
        System.out.println("obj.c1 = " + obj.c1);
    }
}

