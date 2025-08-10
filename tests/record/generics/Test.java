import java.util.*;

public class Test {

    // Generic record with single type parameter
    record Container<T>(T value) {}

    // Generic record with multiple type parameters
    record Pair<K, V>(K key, V value) {}

    // Generic record with bounded type parameters
    record NumberPair<T extends Number>(T first, T second) {}

    // Generic record with wildcard usage
    record ListContainer<T>(List<T> items) {}

    public static void main(String[] args) {
        System.out.println("=== Generics Tests ===");

        testGenericRecord();
        testMultipleTypeParameters();
        testBoundedTypeParameters();
        testGenericMethods();
        testWildcards();
        testTypeErasure();
    }

    public static void testGenericRecord() {
        System.out.println("--- Test Generic Record ---");
        Container<String> stringContainer = new Container<>("Hello World");
        Container<Integer> intContainer = new Container<>(42);
        Container<Boolean> boolContainer = new Container<>(true);

        System.out.println("String container: " + stringContainer);
        System.out.println("String value: " + stringContainer.value());
        System.out.println("String value type: " + stringContainer.value().getClass().getSimpleName());

        System.out.println("Integer container: " + intContainer);
        System.out.println("Integer value: " + intContainer.value());
        System.out.println("Integer value type: " + intContainer.value().getClass().getSimpleName());

        System.out.println("Boolean container: " + boolContainer);
        System.out.println("Boolean value: " + boolContainer.value());
    }

    public static void testMultipleTypeParameters() {
        System.out.println("--- Test Multiple Type Parameters ---");
        Pair<String, Integer> nameAge = new Pair<>("Alice", 30);
        Pair<Integer, String> idName = new Pair<>(123, "Bob");
        Pair<String, String> coordinates = new Pair<>("latitude", "longitude");

        System.out.println("Name-Age pair: " + nameAge);
        System.out.println("Key: " + nameAge.key() + ", Value: " + nameAge.value());

        System.out.println("ID-Name pair: " + idName);
        System.out.println("Key: " + idName.key() + ", Value: " + idName.value());

        System.out.println("Coordinates pair: " + coordinates);
        System.out.println("Key: " + coordinates.key() + ", Value: " + coordinates.value());
    }

    public static void testBoundedTypeParameters() {
        System.out.println("--- Test Bounded Type Parameters ---");
        NumberPair<Integer> intPair = new NumberPair<>(10, 20);
        NumberPair<Double> doublePair = new NumberPair<>(3.14, 2.71);
        NumberPair<Float> floatPair = new NumberPair<>(1.5f, 2.5f);

        System.out.println("Integer pair: " + intPair);
        System.out.println("Double pair: " + doublePair);
        System.out.println("Float pair: " + floatPair);

        // This would cause compilation error:
        // NumberPair<String> stringPair = new NumberPair<>("a", "b");
    }

    public static void testGenericMethods() {
        System.out.println("--- Test Generic Methods ---");
        Container<String> container1 = new Container<>("test");
        Container<String> container2 = new Container<>("test");
        Container<String> container3 = new Container<>("different");

        System.out.println("container1.equals(container2): " + container1.equals(container2));
        System.out.println("container1.equals(container3): " + container1.equals(container3));
        System.out.println("container1.hashCode(): " + container1.hashCode());
        System.out.println("container2.hashCode(): " + container2.hashCode());
    }

    public static void testWildcards() {
        System.out.println("--- Test Wildcards ---");
        List<String> stringList = Arrays.asList("apple", "banana", "cherry");
        List<Integer> intList = Arrays.asList(1, 2, 3, 4, 5);

        ListContainer<String> stringListContainer = new ListContainer<>(stringList);
        ListContainer<Integer> intListContainer = new ListContainer<>(intList);

        System.out.println("String list container: " + stringListContainer);
        System.out.println("String list size: " + stringListContainer.items().size());

        System.out.println("Integer list container: " + intListContainer);
        System.out.println("Integer list size: " + intListContainer.items().size());

        // Test with nested generics
        Container<List<String>> nestedContainer = new Container<>(stringList);
        System.out.println("Nested container: " + nestedContainer);
    }

    public static void testTypeErasure() {
        System.out.println("--- Test Type Erasure ---");
        Container<String> stringContainer = new Container<>("hello");
        Container<Integer> intContainer = new Container<>(42);

        System.out.println("String container class: " + stringContainer.getClass().getName());
        System.out.println("Integer container class: " + intContainer.getClass().getName());
        System.out.println("Same class: " + (stringContainer.getClass() == intContainer.getClass()));

        // Test raw types
        @SuppressWarnings("rawtypes")
        Container rawContainer = new Container("raw value");
        System.out.println("Raw container: " + rawContainer);
        System.out.println("Raw container value: " + rawContainer.value());
    }
}
