import type { Target } from '../shared/protocol';
export type Example = { title: string; minimumVersion: number; source: string };
const javaExamples = {
  hello: {
    title: 'Hello, world',
    minimumVersion: 8,
    source: `public class Main {
    public static void main(String[] args) {
        System.out.println("Hello, world! ☕");

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

const script = (title: string, source: string): Example => ({ title, source, minimumVersion: 25 });
const scalaHello = 'println("Hello, world! ☕")\nfor (i <- 1 to 3) println(s"Cup $i of Scala")\n';
const scalaCollections =
  'val coffees = List("Espresso", "Ristretto", "Latte")\ncoffees.sorted.map(_.toUpperCase).foreach(println)\n';
export const examples: Record<Target, Record<string, Example>> = {
  java: javaExamples,
  kotlin: {
    hello: script(
      'Hello, world',
      'println("Hello, world! ☕")\nfor (i in 1..3) println("Cup $i of Kotlin")\n',
    ),
    collections: script(
      'Collections & transformations',
      'val coffees = listOf("Espresso", "Ristretto", "Latte")\ncoffees.sorted().map { it.uppercase() }.forEach { println(it) }\n',
    ),
    features: script(
      'Null safety',
      'val order: String? = null\nprintln(order?.uppercase() ?: "Ristretto")\n',
    ),
  },
  groovy: {
    hello: script(
      'Hello, world',
      'println "Hello, world! ☕"\n(1..3).each { println "Cup $it of Groovy" }\n',
    ),
    collections: script(
      'Collections & transformations',
      'def coffees = ["Espresso", "Ristretto", "Latte"]\ncoffees.sort().collect { it.toUpperCase() }.each { println it }\n',
    ),
    features: script(
      'Closures',
      'def order = { name, shots -> "$name with $shots shots" }\nprintln order("Ristretto", 2)\n',
    ),
  },
  scala2: {
    hello: script('Hello, world', scalaHello),
    collections: script('Collections & transformations', scalaCollections),
    features: script(
      'Case classes & pattern matching',
      'case class Coffee(name: String, shots: Int)\nval order = Coffee("Ristretto", 2)\nprintln(order match {\n  case Coffee(name, shots) => s"$name with $shots shots"\n})\n',
    ),
  },
  scala3: {
    hello: script('Hello, world', scalaHello),
    collections: script('Collections & transformations', scalaCollections),
    features: script(
      'Enums & pattern matching',
      'enum Coffee:\n  case Espresso, Ristretto, Latte\nval order = Coffee.Ristretto\nprintln(order match\n  case Coffee.Ristretto => "Ristretto with 2 shots"\n  case other => other.toString\n)\n',
    ),
  },
  clojure: {
    hello: script(
      'Hello, world',
      '(println "Hello, world! ☕")\n(doseq [i (range 1 4)]\n  (println (str "Cup " i " of Clojure")))\n',
    ),
    collections: script(
      'Collections & transformations',
      '(require \'[clojure.string :as str])\n(doseq [coffee (sort ["Espresso" "Ristretto" "Latte"])]\n  (println (str/upper-case coffee)))\n',
    ),
    features: script(
      'Functions & sequences',
      '(defn order [name shots]\n  (str name " with " shots " shots"))\n(println (order "Ristretto" (reduce + [1 1])))\n',
    ),
  },
};
