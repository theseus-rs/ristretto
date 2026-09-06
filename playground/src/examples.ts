export const examples = {
  hello: {
    title: 'Hello, world',
    minimumVersion: 8,
    source: `public class Main {
    public static void main(String[] args) {
        System.out.println("Hello, world! ☕");

        // Make yourself a little something.
        String language = "Java";
        int cups = 3;

        for (int i = 1; i <= cups; i++) {
            System.out.println("Cup " + i + " of " + language);
        }
    }
}
`,
  },
  collections: {
    title: 'Collections & streams',
    minimumVersion: 8,
    source: `import java.util.Arrays;
import java.util.List;

public class Main {
    public static void main(String[] args) {
        List<String> coffees = Arrays.asList("Espresso", "Ristretto", "Latte");

        coffees.stream()
            .sorted()
            .map(String::toUpperCase)
            .forEach(System.out::println);
    }
}
`,
  },
  records: {
    title: 'Records & pattern matching',
    minimumVersion: 21,
    source: `public class Main {
    record Coffee(String name, int shots) {}

    public static void main(String[] args) {
        Object order = new Coffee("Ristretto", 2);

        String description = switch (order) {
            case Coffee(var name, var shots) ->
                name + " with " + shots + " shots";
            default -> "Something new";
        };

        System.out.println(description);
    }
}
`,
  },
} as const;
