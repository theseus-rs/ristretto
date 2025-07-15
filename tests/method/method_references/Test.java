/** Test method references and lambda expressions. */
import java.util.Arrays;
import java.util.List;
import java.util.function.*;

public class Test {
    // Static method for method reference
    public static int multiply(int a, int b) {
        return a * b;
    }

    // Instance method for method reference
    public int add(int a, int b) {
        return a + b;
    }

    // Method that accepts functional interfaces
    public static void testBinaryOperator(BinaryOperator<Integer> operator, int a, int b) {
        int result = operator.apply(a, b);
        System.out.println("Operation result: " + result);
    }

    public static void testPredicate(Predicate<String> predicate, String input) {
        boolean result = predicate.test(input);
        System.out.println("Predicate test for '" + input + "': " + result);
    }

    public static void testConsumer(Consumer<String> consumer, String input) {
        consumer.accept(input);
    }

    public static void testFunction(Function<String, Integer> function, String input) {
        Integer result = function.apply(input);
        System.out.println("Function result for '" + input + "': " + result);
    }

    // Constructor for constructor reference
    static class Person {
        private String name;
        private int age;

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
            return name + " (" + age + ")";
        }
    }

    public static void main(String[] args) {
        Test test = new Test();

        System.out.println("=== Testing method references ===");

        // Static method reference
        testBinaryOperator(Test::multiply, 5, 3);

        // Instance method reference
        testBinaryOperator(test::add, 5, 3);

        // Method reference to existing method
        testBinaryOperator(Integer::sum, 5, 3);

        System.out.println("\n=== Testing lambda expressions ===");

        // Lambda expressions
        testBinaryOperator((a, b) -> a - b, 10, 3);
        testBinaryOperator((a, b) -> a * a + b * b, 3, 4);

        System.out.println("\n=== Testing predicates ===");

        // Predicate method references and lambdas
        testPredicate(String::isEmpty, "");
        testPredicate(String::isEmpty, "hello");
        testPredicate(s -> s.length() > 5, "hello");
        testPredicate(s -> s.length() > 5, "hello world");

        System.out.println("\n=== Testing consumers ===");

        // Consumer method references and lambdas
        testConsumer(System.out::println, "Method reference consumer");
        testConsumer(s -> System.out.println("Lambda consumer: " + s), "test");

        System.out.println("\n=== Testing functions ===");

        // Function method references and lambdas
        testFunction(String::length, "hello");
        testFunction(s -> s.hashCode(), "hello");

        System.out.println("\n=== Testing constructor references ===");

        // Constructor references
        Function<String, Person> personConstructor1 = Person::new;
        BiFunction<String, Integer, Person> personConstructor2 = Person::new;

        Person person1 = personConstructor1.apply("Alice");
        Person person2 = personConstructor2.apply("Bob", 25);

        System.out.println("Created person 1: " + person1);
        System.out.println("Created person 2: " + person2);

        System.out.println("\n=== Testing with collections ===");

        List<String> words = Arrays.asList("hello", "world", "java", "method", "reference");

        // Using method references with streams
        System.out.println("Original list: " + words);

        words.stream()
             .filter(s -> s.length() > 4)
             .map(String::toUpperCase)
             .forEach(System.out::println);
    }
}
