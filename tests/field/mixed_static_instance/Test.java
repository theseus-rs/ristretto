/**
 * Tests that static and instance field initialization are completely separate.
 *
 * Key behaviors verified:
 * - Static fields: initialized at class initialization (<clinit>)
 * - Instance fields: initialized at object creation (<init>)
 * - Static fields are NOT initialized during object allocation
 * - Instance fields are NOT initialized during class loading
 */
public class Test {
    static class A {
        static int staticField = initStatic("A.staticField");
        int instanceField = initInstance("A.instanceField");

        static int initStatic(String name) {
            System.out.println("Static init: " + name);
            return 1;
        }

        int initInstance(String name) {
            System.out.println("Instance init: " + name);
            return 2;
        }
    }

    public static void main(String[] args) {
        // First, access the class to trigger class initialization
        // This should initialize static fields but NOT instance fields
        System.out.println("Step 1: Accessing class (triggers <clinit>):");
        System.out.println("A.staticField = " + A.staticField);
        System.out.println();

        // Creating first object - instance fields initialized here
        System.out.println("Step 2: Creating first A object:");
        A obj1 = new A();
        System.out.println("obj1.instanceField = " + obj1.instanceField);
        System.out.println();

        // Creating second object - instance fields initialized again
        System.out.println("Step 3: Creating second A object:");
        A obj2 = new A();
        System.out.println("obj2.instanceField = " + obj2.instanceField);
        System.out.println();

        // Static field should still have the same value
        System.out.println("Step 4: Static field value unchanged:");
        System.out.println("A.staticField = " + A.staticField);
    }
}

