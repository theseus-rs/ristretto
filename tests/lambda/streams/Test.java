/** Test lambdas with streams API. */
import java.util.*;
import java.util.stream.*;

public class Test {
    public static void main(String[] args) {
        System.out.println("=== Lambda with Streams Tests ===");

        // Basic stream operations
        System.out.println("--- filter ---");
        List<Integer> numbers = Arrays.asList(1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
        System.out.println("Original: " + numbers);

        List<Integer> evens = numbers.stream()
            .filter(n -> n % 2 == 0)
            .collect(Collectors.toList());
        System.out.println("Evens: " + evens);

        List<Integer> greaterThan5 = numbers.stream()
            .filter(n -> n > 5)
            .collect(Collectors.toList());
        System.out.println("Greater than 5: " + greaterThan5);

        // map
        System.out.println("--- map ---");
        List<String> words = Arrays.asList("hello", "world", "java", "stream");
        System.out.println("Original words: " + words);

        List<String> upperWords = words.stream()
            .map(String::toUpperCase)
            .collect(Collectors.toList());
        System.out.println("Uppercase: " + upperWords);

        List<Integer> lengths = words.stream()
            .map(String::length)
            .collect(Collectors.toList());
        System.out.println("Lengths: " + lengths);

        // flatMap
        System.out.println("--- flatMap ---");
        List<List<Integer>> nested = Arrays.asList(
            Arrays.asList(1, 2),
            Arrays.asList(3, 4, 5),
            Arrays.asList(6)
        );
        List<Integer> flat = nested.stream()
            .flatMap(List::stream)
            .collect(Collectors.toList());
        System.out.println("Flattened: " + flat);

        // reduce
        System.out.println("--- reduce ---");
        int sum = numbers.stream()
            .reduce(0, (a, b) -> a + b);
        System.out.println("Sum: " + sum);

        int product = numbers.stream()
            .filter(n -> n <= 5)
            .reduce(1, (a, b) -> a * b);
        System.out.println("Product of 1-5: " + product);

        Optional<Integer> max = numbers.stream()
            .reduce((a, b) -> a > b ? a : b);
        System.out.println("Max: " + max.orElse(-1));

        // sorted
        System.out.println("--- sorted ---");
        List<String> unsorted = Arrays.asList("banana", "apple", "cherry", "date");
        List<String> sorted = unsorted.stream()
            .sorted()
            .collect(Collectors.toList());
        System.out.println("Sorted: " + sorted);

        List<String> sortedByLength = unsorted.stream()
            .sorted((a, b) -> Integer.compare(a.length(), b.length()))
            .collect(Collectors.toList());
        System.out.println("Sorted by length: " + sortedByLength);

        List<String> sortedReverse = unsorted.stream()
            .sorted(Comparator.reverseOrder())
            .collect(Collectors.toList());
        System.out.println("Sorted reverse: " + sortedReverse);

        // distinct
        System.out.println("--- distinct ---");
        List<Integer> withDupes = Arrays.asList(1, 2, 2, 3, 3, 3, 4, 4, 4, 4);
        List<Integer> unique = withDupes.stream()
            .distinct()
            .collect(Collectors.toList());
        System.out.println("Unique: " + unique);

        // limit and skip
        System.out.println("--- limit and skip ---");
        List<Integer> first3 = numbers.stream()
            .limit(3)
            .collect(Collectors.toList());
        System.out.println("First 3: " + first3);

        List<Integer> skip3 = numbers.stream()
            .skip(3)
            .collect(Collectors.toList());
        System.out.println("Skip 3: " + skip3);

        List<Integer> middle = numbers.stream()
            .skip(3)
            .limit(4)
            .collect(Collectors.toList());
        System.out.println("Middle (skip 3, limit 4): " + middle);

        // forEach
        System.out.println("--- forEach ---");
        System.out.print("ForEach: ");
        numbers.stream()
            .limit(5)
            .forEach(n -> System.out.print(n + " "));
        System.out.println();

        // anyMatch, allMatch, noneMatch
        System.out.println("--- match operations ---");
        boolean anyEven = numbers.stream().anyMatch(n -> n % 2 == 0);
        System.out.println("Any even: " + anyEven);

        boolean allPositive = numbers.stream().allMatch(n -> n > 0);
        System.out.println("All positive: " + allPositive);

        boolean noneNegative = numbers.stream().noneMatch(n -> n < 0);
        System.out.println("None negative: " + noneNegative);

        // findFirst, findAny
        System.out.println("--- find operations ---");
        Optional<Integer> first = numbers.stream()
            .filter(n -> n > 5)
            .findFirst();
        System.out.println("First > 5: " + first.orElse(-1));

        Optional<Integer> any = numbers.stream()
            .filter(n -> n > 5)
            .findAny();
        System.out.println("Any > 5: " + any.orElse(-1));

        // count
        System.out.println("--- count ---");
        long count = numbers.stream()
            .filter(n -> n % 2 == 0)
            .count();
        System.out.println("Count of evens: " + count);

        // min and max
        System.out.println("--- min and max ---");
        Optional<Integer> minVal = numbers.stream().min(Integer::compareTo);
        Optional<Integer> maxVal = numbers.stream().max(Integer::compareTo);
        System.out.println("Min: " + minVal.orElse(-1));
        System.out.println("Max: " + maxVal.orElse(-1));

        // collect to different types
        System.out.println("--- collect ---");
        String joined = words.stream().collect(Collectors.joining(", "));
        System.out.println("Joined: " + joined);

        String joinedWithBrackets = words.stream()
            .collect(Collectors.joining(", ", "[", "]"));
        System.out.println("Joined with brackets: " + joinedWithBrackets);

        // groupingBy
        System.out.println("--- groupingBy ---");
        Map<Integer, List<String>> byLength = words.stream()
            .collect(Collectors.groupingBy(String::length));
        for (Map.Entry<Integer, List<String>> entry : byLength.entrySet()) {
            System.out.println("  Length " + entry.getKey() + ": " + entry.getValue());
        }

        // partitioningBy
        System.out.println("--- partitioningBy ---");
        Map<Boolean, List<Integer>> partitioned = numbers.stream()
            .collect(Collectors.partitioningBy(n -> n % 2 == 0));
        System.out.println("Even partition: " + partitioned.get(true));
        System.out.println("Odd partition: " + partitioned.get(false));

        // mapToInt and primitive streams
        System.out.println("--- primitive streams ---");
        int sumInts = words.stream()
            .mapToInt(String::length)
            .sum();
        System.out.println("Sum of lengths: " + sumInts);

        OptionalDouble avg = words.stream()
            .mapToInt(String::length)
            .average();
        System.out.println("Average length: " + avg.orElse(0.0));

        IntStream.range(1, 6).forEach(n -> System.out.print(n + " "));
        System.out.println();

        // peek (for debugging)
        System.out.println("--- peek ---");
        List<String> peeked = words.stream()
            .peek(s -> System.out.println("  Processing: " + s))
            .map(String::toUpperCase)
            .limit(2)
            .collect(Collectors.toList());
        System.out.println("Peeked result: " + peeked);

        System.out.println("=== End Lambda with Streams Tests ===");
    }
}
