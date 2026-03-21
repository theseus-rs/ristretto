/** Test MethodHandle-based constructor invocation (NewInvokeSpecial).
 *
 * This exercises the MethodHandle dispatch path for constructors, including
 * findConstructor() with various parameter types and constructor chaining.
 */
import java.lang.invoke.MethodHandle;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.MethodType;

public class Test {
    static class SimpleClass {
        private final String value;

        public SimpleClass() {
            this.value = "default";
        }

        public SimpleClass(String value) {
            this.value = value;
        }

        public SimpleClass(int num) {
            this.value = "int:" + num;
        }

        public SimpleClass(String prefix, int num) {
            this.value = prefix + ":" + num;
        }

        @Override
        public String toString() {
            return "SimpleClass(" + value + ")";
        }
    }

    static class Parent {
        protected String name;

        public Parent(String name) {
            this.name = name;
        }

        @Override
        public String toString() {
            return "Parent(" + name + ")";
        }
    }

    static class Child extends Parent {
        private final int age;

        public Child(String name, int age) {
            super(name);
            this.age = age;
        }

        @Override
        public String toString() {
            return "Child(" + name + ", " + age + ")";
        }
    }

    public static void main(String[] args) throws Throwable {
        MethodHandles.Lookup lookup = MethodHandles.lookup();

        System.out.println("=== MethodHandle Constructor Tests ===");

        // Test no-arg constructor
        MethodHandle noArgCtor = lookup.findConstructor(
            SimpleClass.class, MethodType.methodType(void.class));
        SimpleClass obj1 = (SimpleClass) noArgCtor.invoke();
        System.out.println("No-arg constructor: " + obj1);

        // Test single String arg constructor
        MethodHandle stringCtor = lookup.findConstructor(
            SimpleClass.class, MethodType.methodType(void.class, String.class));
        SimpleClass obj2 = (SimpleClass) stringCtor.invoke("hello");
        System.out.println("String constructor: " + obj2);

        // Test single int arg constructor
        MethodHandle intCtor = lookup.findConstructor(
            SimpleClass.class, MethodType.methodType(void.class, int.class));
        SimpleClass obj3 = (SimpleClass) intCtor.invoke(42);
        System.out.println("Int constructor: " + obj3);

        // Test two-arg constructor
        MethodHandle twoArgCtor = lookup.findConstructor(
            SimpleClass.class, MethodType.methodType(void.class, String.class, int.class));
        SimpleClass obj4 = (SimpleClass) twoArgCtor.invoke("prefix", 99);
        System.out.println("Two-arg constructor: " + obj4);

        // Test inheritance constructor
        MethodHandle childCtor = lookup.findConstructor(
            Child.class, MethodType.methodType(void.class, String.class, int.class));
        Child child = (Child) childCtor.invoke("Alice", 30);
        System.out.println("Child constructor: " + child);

        // Test that constructed objects are distinct
        SimpleClass a = (SimpleClass) noArgCtor.invoke();
        SimpleClass b = (SimpleClass) noArgCtor.invoke();
        System.out.println("Distinct objects: " + (a != b));

        // Test invokeExact with constructor
        MethodHandle exactCtor = lookup.findConstructor(
            SimpleClass.class, MethodType.methodType(void.class, String.class));
        SimpleClass exact = (SimpleClass) exactCtor.invokeExact("exact");
        System.out.println("InvokeExact constructor: " + exact);

        System.out.println("=== MethodHandle Constructor Tests Complete ===");
    }
}
