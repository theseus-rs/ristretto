/** Test lambdas with Optional. */
import java.util.*;
import java.util.function.*;

public class Test {
    static class Person {
        private String name;
        private Integer age;
        private String email;

        public Person(String name, Integer age, String email) {
            this.name = name;
            this.age = age;
            this.email = email;
        }

        public Optional<String> getName() {
            return Optional.ofNullable(name);
        }

        public Optional<Integer> getAge() {
            return Optional.ofNullable(age);
        }

        public Optional<String> getEmail() {
            return Optional.ofNullable(email);
        }

        @Override
        public String toString() {
            return "Person[" + name + "]";
        }
    }

    public static void main(String[] args) {
        System.out.println("=== Lambda with Optional Tests ===");

        // Creating Optional
        System.out.println("--- Creating Optional ---");
        Optional<String> present = Optional.of("hello");
        Optional<String> empty = Optional.empty();
        Optional<String> nullable = Optional.ofNullable(null);

        System.out.println("present.isPresent(): " + present.isPresent());
        System.out.println("empty.isPresent(): " + empty.isPresent());
        System.out.println("nullable.isPresent(): " + nullable.isPresent());

        // ifPresent with lambda
        System.out.println("--- ifPresent ---");
        present.ifPresent(s -> System.out.println("Present value: " + s));
        empty.ifPresent(s -> System.out.println("This won't print"));

        // map with lambda
        System.out.println("--- map ---");
        Optional<Integer> length = present.map(String::length);
        System.out.println("Mapped length: " + length.orElse(-1));

        Optional<Integer> emptyLength = empty.map(String::length);
        System.out.println("Empty mapped: " + emptyLength.orElse(-1));

        Optional<String> upper = present.map(String::toUpperCase);
        System.out.println("Mapped upper: " + upper.orElse("N/A"));

        // Chained map
        Optional<String> result = present
            .map(String::trim)
            .map(String::toUpperCase)
            .map(s -> "[" + s + "]");
        System.out.println("Chained map: " + result.orElse("N/A"));

        // flatMap with lambda
        System.out.println("--- flatMap ---");
        Person person1 = new Person("Alice", 30, "alice@example.com");
        Person person2 = new Person("Bob", null, null);

        Optional<Person> optPerson1 = Optional.of(person1);
        Optional<Person> optPerson2 = Optional.of(person2);

        Optional<String> email1 = optPerson1.flatMap(Person::getEmail);
        Optional<String> email2 = optPerson2.flatMap(Person::getEmail);

        System.out.println("Person1 email: " + email1.orElse("no email"));
        System.out.println("Person2 email: " + email2.orElse("no email"));

        // Chained flatMap
        Optional<Integer> age = optPerson1
            .flatMap(Person::getAge)
            .map(a -> a + 1);
        System.out.println("Person1 age + 1: " + age.orElse(-1));

        // filter with lambda
        System.out.println("--- filter ---");
        Optional<String> filtered1 = present.filter(s -> s.length() > 3);
        Optional<String> filtered2 = present.filter(s -> s.length() > 10);

        System.out.println("Filtered (> 3): " + filtered1.orElse("filtered out"));
        System.out.println("Filtered (> 10): " + filtered2.orElse("filtered out"));

        Optional<Integer> evenAge = optPerson1
            .flatMap(Person::getAge)
            .filter(a -> a % 2 == 0);
        System.out.println("Even age: " + evenAge.orElse(-1));

        // orElseGet with lambda
        System.out.println("--- orElseGet ---");
        String value1 = present.orElseGet(() -> "default from supplier");
        String value2 = empty.orElseGet(() -> "default from supplier");
        System.out.println("Present orElseGet: " + value1);
        System.out.println("Empty orElseGet: " + value2);

        // Lazy evaluation demo
        String lazyValue = empty.orElseGet(() -> {
            System.out.println("  Computing default...");
            return "computed";
        });
        System.out.println("Lazy value: " + lazyValue);

        // orElseThrow with lambda
        System.out.println("--- orElseThrow ---");
        try {
            String val = present.orElseThrow(() -> new RuntimeException("Not found"));
            System.out.println("orElseThrow present: " + val);
        } catch (RuntimeException e) {
            System.out.println("Exception: " + e.getMessage());
        }

        try {
            String val = empty.orElseThrow(() -> new RuntimeException("Value is empty"));
            System.out.println("This won't print");
        } catch (RuntimeException e) {
            System.out.println("Caught: " + e.getMessage());
        }

        // or() with lambda (Java 9+)
        System.out.println("--- or ---");
        Optional<String> or1 = present.or(() -> Optional.of("alternative"));
        Optional<String> or2 = empty.or(() -> Optional.of("alternative"));
        System.out.println("Present or: " + or1.orElse("N/A"));
        System.out.println("Empty or: " + or2.orElse("N/A"));

        // ifPresentOrElse (Java 9+)
        System.out.println("--- ifPresentOrElse ---");
        present.ifPresentOrElse(
            s -> System.out.println("Present: " + s),
            () -> System.out.println("Empty")
        );
        empty.ifPresentOrElse(
            s -> System.out.println("Present: " + s),
            () -> System.out.println("Empty")
        );

        // Complex optional chain
        System.out.println("--- Complex Chain ---");
        Optional<String> complexResult = Optional.of("  Hello World  ")
            .map(String::trim)
            .filter(s -> s.length() > 5)
            .map(String::toLowerCase)
            .map(s -> s.replace(" ", "_"));
        System.out.println("Complex chain: " + complexResult.orElse("N/A"));

        // Optional with stream
        System.out.println("--- Optional Stream ---");
        List<Optional<String>> optionals = Arrays.asList(
            Optional.of("one"),
            Optional.empty(),
            Optional.of("three"),
            Optional.empty(),
            Optional.of("five")
        );

        optionals.stream()
            .filter(Optional::isPresent)
            .map(Optional::get)
            .forEach(s -> System.out.println("  " + s));

        // stream() on Optional (Java 9+)
        System.out.println("--- Optional.stream ---");
        present.stream().forEach(s -> System.out.println("Stream present: " + s));
        empty.stream().forEach(s -> System.out.println("This won't print"));

        long count = optionals.stream()
            .flatMap(Optional::stream)
            .count();
        System.out.println("Non-empty count: " + count);

        System.out.println("=== End Lambda with Optional Tests ===");
    }
}
