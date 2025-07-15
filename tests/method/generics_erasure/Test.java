/** Test method generics, type erasure, and type safety. */
import java.util.*;

public class Test {
    // Generic method with single type parameter
    public static <T> T identity(T input) {
        System.out.println("Identity method called with: " + input + " (type: " + input.getClass().getSimpleName() + ")");
        return input;
    }

    // Generic method with multiple type parameters
    public static <T, U> String combine(T first, U second) {
        System.out.println("Combining " + first + " and " + second);
        return first.toString() + second.toString();
    }

    // Generic method with bounded type parameters
    public static <T extends Number> double sum(T a, T b) {
        System.out.println("Summing numbers: " + a + " + " + b);
        return a.doubleValue() + b.doubleValue();
    }

    // Generic method with wildcard bounds
    public static void printNumbers(List<? extends Number> numbers) {
        System.out.print("Numbers: ");
        for (Number num : numbers) {
            System.out.print(num + " ");
        }
        System.out.println();
    }

    public static void addNumbers(List<? super Integer> numbers) {
        numbers.add(42);
        numbers.add(100);
        System.out.println("Added numbers to list");
    }

    // Generic method with recursive bounds
    public static <T extends Comparable<T>> T max(T a, T b) {
        System.out.println("Finding max of " + a + " and " + b);
        return a.compareTo(b) > 0 ? a : b;
    }

    // Method demonstrating type erasure
    public static void processStrings(List<String> strings) {
        System.out.println("Processing string list: " + strings);
    }

    public static void processIntegers(List<Integer> integers) {
        System.out.println("Processing integer list: " + integers);
    }

    // Cannot have both methods below due to type erasure - they have same signature after erasure
    // public static void process(List<String> list) { }
    // public static void process(List<Integer> list) { }

    // Generic class for testing method inheritance
    static class GenericContainer<T> {
        protected T value;

        public GenericContainer(T value) {
            this.value = value;
        }

        public T getValue() {
            return value;
        }

        public void setValue(T value) {
            this.value = value;
        }

        // Generic method within generic class
        public <U> U transform(U input) {
            System.out.println("Transforming " + input + " in container with " + value);
            return input;
        }

        // Method with same type parameter name as class (shadows class parameter)
        public <T> T shadowingMethod(T input) {
            System.out.println("Shadowing method: " + input + " (class T is: " + value + ")");
            return input;
        }
    }

    // Raw type usage (generates warnings)
    @SuppressWarnings("rawtypes")
    public static void useRawTypes() {
        System.out.println("\n=== Testing Raw Types ===");
        GenericContainer rawContainer = new GenericContainer("raw");
        Object value = rawContainer.getValue(); // Returns Object due to type erasure
        System.out.println("Raw container value: " + value);
    }

    // Method overloading with generics
    public static void process(String item) {
        System.out.println("Processing string: " + item);
    }

    public static <T> void process(T item) {
        System.out.println("Processing generic item: " + item);
    }

    public static void main(String[] args) {
        System.out.println("=== Testing Basic Generic Methods ===");
        String str = identity("Hello");
        Integer num = identity(42);
        Double d = identity(3.14);

        System.out.println("\n=== Testing Multiple Type Parameters ===");
        String combined = combine("Hello", 123);
        System.out.println("Combined result: " + combined);

        System.out.println("\n=== Testing Bounded Type Parameters ===");
        double result = sum(10, 20.5);
        System.out.println("Sum result: " + result);

        String maxStr = max("apple", "zebra");
        System.out.println("Max string: " + maxStr);

        Integer maxInt = max(10, 5);
        System.out.println("Max integer: " + maxInt);

        System.out.println("\n=== Testing Wildcard Bounds ===");
        List<Integer> integers = Arrays.asList(1, 2, 3, 4, 5);
        List<Double> doubles = Arrays.asList(1.1, 2.2, 3.3);

        printNumbers(integers);
        printNumbers(doubles);

        List<Number> numbers = new ArrayList<>();
        addNumbers(numbers);
        System.out.println("Numbers list after adding: " + numbers);

        System.out.println("\n=== Testing Generic Container ===");
        GenericContainer<String> stringContainer = new GenericContainer<>("Hello");
        System.out.println("String container value: " + stringContainer.getValue());

        Integer transformed = stringContainer.transform(42);
        System.out.println("Transformed value: " + transformed);

        String shadowed = stringContainer.shadowingMethod("shadowed");
        System.out.println("Shadowed method result: " + shadowed);

        System.out.println("\n=== Testing Method Overloading with Generics ===");
        process("specific"); // Calls process(String)
        process(123); // Calls process(T)

        useRawTypes();

        System.out.println("\n=== Testing Type Erasure Effects ===");
        processStrings(Arrays.asList("a", "b", "c"));
        processIntegers(Arrays.asList(1, 2, 3));
    }
}
