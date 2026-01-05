/** Test method references - all four kinds. */
import java.util.function.*;

public class Test {
    private String prefix = "Instance: ";

    // Instance method
    public String format(String input) {
        return prefix + input;
    }

    // Static method
    public static String staticFormat(String input) {
        return "Static: " + input;
    }

    public static int staticAdd(int a, int b) {
        return a + b;
    }

    // Method for arbitrary object reference
    public boolean startsWith(String other) {
        return prefix.startsWith(other);
    }

    static class Person {
        private String name;
        private int age;

        public Person() {
            this.name = "Unknown";
            this.age = 0;
        }

        public Person(String name) {
            this.name = name;
            this.age = 0;
        }

        public Person(String name, int age) {
            this.name = name;
            this.age = age;
        }

        @Override
        public String toString() {
            return name + " (age " + age + ")";
        }

        public int compareTo(Person other) {
            return this.name.compareTo(other.name);
        }
    }

    public static void main(String[] args) {
        System.out.println("=== Method Reference Tests ===");

        Test instance = new Test();

        // 1. Reference to a static method: ClassName::staticMethod
        System.out.println("--- Static Method Reference ---");
        Function<String, String> staticRef = Test::staticFormat;
        System.out.println(staticRef.apply("hello"));

        BinaryOperator<Integer> addRef = Test::staticAdd;
        System.out.println("Static add: " + addRef.apply(5, 3));

        // Math static methods
        UnaryOperator<Double> absRef = Math::abs;
        System.out.println("Math.abs(-5.5): " + absRef.apply(-5.5));

        BinaryOperator<Integer> maxRef = Math::max;
        System.out.println("Math.max(10, 20): " + maxRef.apply(10, 20));

        // 2. Reference to an instance method of a particular object: instance::method
        System.out.println("--- Instance Method Reference (Particular Object) ---");
        Function<String, String> instanceRef = instance::format;
        System.out.println(instanceRef.apply("world"));

        // StringBuilder instance method
        StringBuilder sb = new StringBuilder("Base: ");
        Consumer<String> appendRef = sb::append;
        appendRef.accept("one");
        appendRef.accept(", two");
        System.out.println(sb.toString());

        // 3. Reference to an instance method of an arbitrary object: ClassName::instanceMethod
        System.out.println("--- Instance Method Reference (Arbitrary Object) ---");
        Function<String, String> toUpperRef = String::toUpperCase;
        System.out.println("toUpper: " + toUpperRef.apply("hello"));

        Function<String, String> toLowerRef = String::toLowerCase;
        System.out.println("toLower: " + toLowerRef.apply("HELLO"));

        Function<String, Integer> lengthRef = String::length;
        System.out.println("length: " + lengthRef.apply("hello"));

        Predicate<String> isEmptyRef = String::isEmpty;
        System.out.println("isEmpty(''): " + isEmptyRef.test(""));
        System.out.println("isEmpty('hello'): " + isEmptyRef.test("hello"));

        // BiFunction using arbitrary object
        BiFunction<String, String, Boolean> startsWithRef = String::startsWith;
        System.out.println("'hello'.startsWith('he'): " + startsWithRef.apply("hello", "he"));
        System.out.println("'hello'.startsWith('wo'): " + startsWithRef.apply("hello", "wo"));

        // 4. Reference to a constructor: ClassName::new
        System.out.println("--- Constructor Reference ---");
        Supplier<Person> noArgConstructor = Person::new;
        Person p1 = noArgConstructor.get();
        System.out.println("No-arg constructor: " + p1);

        Function<String, Person> singleArgConstructor = Person::new;
        Person p2 = singleArgConstructor.apply("Alice");
        System.out.println("Single-arg constructor: " + p2);

        BiFunction<String, Integer, Person> twoArgConstructor = Person::new;
        Person p3 = twoArgConstructor.apply("Bob", 25);
        System.out.println("Two-arg constructor: " + p3);

        // Array constructor reference
        System.out.println("--- Array Constructor Reference ---");
        IntFunction<String[]> arrayConstructor = String[]::new;
        String[] arr = arrayConstructor.apply(5);
        System.out.println("Created array length: " + arr.length);

        IntFunction<int[]> intArrayConstructor = int[]::new;
        int[] intArr = intArrayConstructor.apply(3);
        System.out.println("Created int array length: " + intArr.length);

        // Method reference comparison
        System.out.println("--- Method Reference vs Lambda ---");
        Function<String, Integer> lambdaLen = s -> s.length();
        Function<String, Integer> refLen = String::length;
        System.out.println("Lambda length: " + lambdaLen.apply("test"));
        System.out.println("Reference length: " + refLen.apply("test"));

        // Chaining method references with andThen
        System.out.println("--- Method Reference Chaining ---");
        Function<String, String> trimRef = String::trim;
        Function<String, String> combined = trimRef.andThen(String::toUpperCase);
        System.out.println("Trim and upper: " + combined.apply("  hello  "));

        System.out.println("=== End Method Reference Tests ===");
    }
}
