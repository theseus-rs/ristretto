/** Test lambdas with Comparator interface. */
import java.util.*;
import java.util.function.*;

public class Test {
    static class Person {
        private String name;
        private int age;
        private String city;

        public Person(String name, int age, String city) {
            this.name = name;
            this.age = age;
            this.city = city;
        }

        public String getName() { return name; }
        public int getAge() { return age; }
        public String getCity() { return city; }

        @Override
        public String toString() {
            return name + "(" + age + ", " + city + ")";
        }
    }

    public static void main(String[] args) {
        System.out.println("=== Comparator Lambda Tests ===");

        // Basic comparator with lambda
        System.out.println("--- Basic Comparator ---");
        List<String> words = new ArrayList<>(Arrays.asList("banana", "apple", "cherry", "date"));
        System.out.println("Original: " + words);

        Collections.sort(words, (a, b) -> a.compareTo(b));
        System.out.println("Natural order: " + words);

        Collections.sort(words, (a, b) -> b.compareTo(a));
        System.out.println("Reverse order: " + words);

        Collections.sort(words, (a, b) -> Integer.compare(a.length(), b.length()));
        System.out.println("By length: " + words);

        // Comparator.comparing
        System.out.println("--- Comparator.comparing ---");
        words = new ArrayList<>(Arrays.asList("banana", "apple", "cherry", "date"));

        words.sort(Comparator.comparing(String::length));
        System.out.println("By length (comparing): " + words);

        words.sort(Comparator.comparing(s -> s.charAt(0)));
        System.out.println("By first char: " + words);

        // Comparator with objects
        System.out.println("--- Object Comparator ---");
        List<Person> people = new ArrayList<>(Arrays.asList(
            new Person("Alice", 30, "NYC"),
            new Person("Bob", 25, "LA"),
            new Person("Charlie", 35, "Chicago"),
            new Person("Diana", 25, "NYC")
        ));
        System.out.println("Original: " + people);

        people.sort(Comparator.comparing(Person::getName));
        System.out.println("By name: " + people);

        people.sort(Comparator.comparingInt(Person::getAge));
        System.out.println("By age: " + people);

        // thenComparing
        System.out.println("--- thenComparing ---");
        people = new ArrayList<>(Arrays.asList(
            new Person("Alice", 30, "NYC"),
            new Person("Bob", 25, "LA"),
            new Person("Charlie", 35, "Chicago"),
            new Person("Alice", 25, "Boston")
        ));

        people.sort(Comparator.comparing(Person::getName).thenComparingInt(Person::getAge));
        System.out.println("By name then age: " + people);

        people.sort(Comparator.comparingInt(Person::getAge).thenComparing(Person::getName));
        System.out.println("By age then name: " + people);

        // reversed
        System.out.println("--- reversed ---");
        people.sort(Comparator.comparing(Person::getName).reversed());
        System.out.println("By name reversed: " + people);

        people.sort(Comparator.comparingInt(Person::getAge).reversed().thenComparing(Person::getName));
        System.out.println("By age desc then name: " + people);

        // nullsFirst and nullsLast
        System.out.println("--- null handling ---");
        List<String> withNulls = new ArrayList<>(Arrays.asList("banana", null, "apple", null, "cherry"));
        System.out.println("With nulls: " + withNulls);

        withNulls.sort(Comparator.nullsFirst(Comparator.naturalOrder()));
        System.out.println("Nulls first: " + withNulls);

        withNulls = new ArrayList<>(Arrays.asList("banana", null, "apple", null, "cherry"));
        withNulls.sort(Comparator.nullsLast(Comparator.naturalOrder()));
        System.out.println("Nulls last: " + withNulls);

        // naturalOrder and reverseOrder
        System.out.println("--- naturalOrder/reverseOrder ---");
        words = new ArrayList<>(Arrays.asList("banana", "apple", "cherry", "date"));
        words.sort(Comparator.naturalOrder());
        System.out.println("Natural: " + words);

        words.sort(Comparator.reverseOrder());
        System.out.println("Reverse: " + words);

        // Custom comparator with complex logic
        System.out.println("--- Complex Comparator ---");
        Comparator<Person> complex = (p1, p2) -> {
            // Sort by city first
            int cityCompare = p1.getCity().compareTo(p2.getCity());
            if (cityCompare != 0) return cityCompare;
            // Then by age descending
            int ageCompare = Integer.compare(p2.getAge(), p1.getAge());
            if (ageCompare != 0) return ageCompare;
            // Then by name
            return p1.getName().compareTo(p2.getName());
        };

        people = new ArrayList<>(Arrays.asList(
            new Person("Alice", 30, "NYC"),
            new Person("Bob", 25, "LA"),
            new Person("Charlie", 35, "NYC"),
            new Person("Diana", 30, "NYC")
        ));
        people.sort(complex);
        System.out.println("Complex sort: " + people);

        // Comparator.comparing with key extractor and comparator
        System.out.println("--- comparing with key comparator ---");
        words = new ArrayList<>(Arrays.asList("banana", "apple", "cherry", "date"));
        words.sort(Comparator.comparing(String::toLowerCase, Comparator.reverseOrder()));
        System.out.println("By lowercase reversed: " + words);

        // Using comparator with streams
        System.out.println("--- Comparator with Streams ---");
        people = Arrays.asList(
            new Person("Alice", 30, "NYC"),
            new Person("Bob", 25, "LA"),
            new Person("Charlie", 35, "Chicago")
        );

        Optional<Person> oldest = people.stream()
            .max(Comparator.comparingInt(Person::getAge));
        System.out.println("Oldest: " + oldest.orElse(null));

        Optional<Person> youngest = people.stream()
            .min(Comparator.comparingInt(Person::getAge));
        System.out.println("Youngest: " + youngest.orElse(null));

        // Sorted stream
        System.out.print("Sorted by name: ");
        people.stream()
            .sorted(Comparator.comparing(Person::getName))
            .forEach(p -> System.out.print(p.getName() + " "));
        System.out.println();

        // Binary search with comparator
        System.out.println("--- Binary Search ---");
        List<String> sorted = Arrays.asList("apple", "banana", "cherry", "date");
        int index = Collections.binarySearch(sorted, "cherry", Comparator.naturalOrder());
        System.out.println("Index of 'cherry': " + index);

        // TreeSet/TreeMap with comparator
        System.out.println("--- TreeSet with Comparator ---");
        TreeSet<String> byLength = new TreeSet<>(Comparator.comparingInt(String::length).thenComparing(Comparator.naturalOrder()));
        byLength.addAll(Arrays.asList("a", "bb", "ccc", "dd", "e"));
        System.out.println("TreeSet by length: " + byLength);

        System.out.println("=== End Comparator Lambda Tests ===");
    }
}
