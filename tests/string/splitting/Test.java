/** Test String splitting and joining operations like split, join */
public class Test {
    public static void main(String[] args) {
        testBasicSplit();
        testSplitWithLimit();
        testSplitWithRegex();
        testSplitWithSpecialCharacters();
        testSplitWithEmptyStrings();
        testSplitEdgeCases();
        testStringJoin();
        testJoinWithEmptyStrings();
        testUnicodeSplitJoin();
        testEmojiSplitJoin();
    }

    private static void testBasicSplit() {
        System.out.println("=== Basic split Tests ===");
        String csv = "apple,banana,cherry,date";
        String[] fruits = csv.split(",");
        System.out.println("Original: " + csv);
        System.out.print("split(','): ");
        for (String fruit : fruits) {
            System.out.print("'" + fruit + "' ");
        }
        System.out.println();
    }

    private static void testSplitWithLimit() {
        System.out.println("\n=== split with limit Tests ===");
        String text = "one,two,three,four,five";
        System.out.println("Original: " + text);

        String[] parts1 = text.split(",", 3);
        System.out.print("split(',', 3): ");
        for (String part : parts1) {
            System.out.print("'" + part + "' ");
        }
        System.out.println();

        String[] parts2 = text.split(",", 0);
        System.out.print("split(',', 0): ");
        for (String part : parts2) {
            System.out.print("'" + part + "' ");
        }
        System.out.println();

        String[] parts3 = text.split(",", -1);
        System.out.print("split(',', -1): ");
        for (String part : parts3) {
            System.out.print("'" + part + "' ");
        }
        System.out.println();
    }

    private static void testSplitWithRegex() {
        System.out.println("\n=== split with regex Tests ===");
        String mixed = "word1 word2\\tword3\\nword4";
        String[] words = mixed.split("\\s+");
        System.out.println("Original: " + mixed);
        System.out.print("split('\\\\s+'): ");
        for (String word : words) {
            System.out.print("'" + word + "' ");
        }
        System.out.println();

        // Test split with digit pattern
        String alphanumeric = "abc123def456ghi";
        String[] alphaParts = alphanumeric.split("\\d+");
        System.out.println("Original: " + alphanumeric);
        System.out.print("split('\\\\d+'): ");
        for (String part : alphaParts) {
            System.out.print("'" + part + "' ");
        }
        System.out.println();
    }

    private static void testSplitWithSpecialCharacters() {
        System.out.println("\n=== split with special characters Tests ===");
        String special = "a.b.c.d";
        String[] dotParts = special.split("\\.");
        System.out.println("Original: " + special);
        System.out.print("split('\\\\.'): ");
        for (String part : dotParts) {
            System.out.print("'" + part + "' ");
        }
        System.out.println();

        String pipes = "a|b|c|d";
        String[] pipeParts = pipes.split("\\|");
        System.out.println("Original: " + pipes);
        System.out.print("split('\\\\|'): ");
        for (String part : pipeParts) {
            System.out.print("'" + part + "' ");
        }
        System.out.println();
    }

    private static void testSplitWithEmptyStrings() {
        System.out.println("\n=== split with empty strings Tests ===");
        String withEmpty = "a,,b,c,";
        String[] emptyParts = withEmpty.split(",");
        System.out.println("Original: " + withEmpty);
        System.out.print("split(','): ");
        for (String part : emptyParts) {
            System.out.print("'" + part + "' ");
        }
        System.out.println();

        String[] emptyPartsWithLimit = withEmpty.split(",", -1);
        System.out.print("split(',', -1): ");
        for (String part : emptyPartsWithLimit) {
            System.out.print("'" + part + "' ");
        }
        System.out.println();
    }

    private static void testSplitEdgeCases() {
        System.out.println("\n=== split edge cases Tests ===");
        String empty = "";
        String[] emptyResult = empty.split(",");
        System.out.println("''.split(','): length=" + emptyResult.length);

        String single = "hello";
        String[] singleResult = single.split(",");
        System.out.println("'hello'.split(','): length=" + singleResult.length + ", content='" + singleResult[0] + "'");

        String onlyDelim = ",,,";
        String[] delimResult = onlyDelim.split(",");
        System.out.println("',,,'.split(','): length=" + delimResult.length);

        String[] delimResultWithLimit = onlyDelim.split(",", -1);
        System.out.println("',,,'.split(',', -1): length=" + delimResultWithLimit.length);
    }

    private static void testStringJoin() {
        System.out.println("\n=== String.join Tests ===");
        String[] elements = {"apple", "banana", "cherry"};
        String joined = String.join(", ", elements);
        System.out.println("join(', ', [apple, banana, cherry]): " + joined);

        String joinedColon = String.join(":", elements);
        System.out.println("join(':', [apple, banana, cherry]): " + joinedColon);

        String[] singleElement = {"hello"};
        String joinedSingle = String.join(",", singleElement);
        System.out.println("join(',', [hello]): " + joinedSingle);

        String[] emptyElements = {};
        String joinedEmpty = String.join(",", emptyElements);
        System.out.println("join(',', []): '" + joinedEmpty + "'");
    }

    private static void testJoinWithEmptyStrings() {
        System.out.println("\n=== join with empty strings Tests ===");
        String[] withEmptyStrings = {"a", "", "b", "", "c"};
        String joinedWithEmpty = String.join(",", withEmptyStrings);
        System.out.println("join(',', [a, '', b, '', c]): " + joinedWithEmpty);
    }

    private static void testUnicodeSplitJoin() {
        System.out.println("\n=== Unicode split/join Tests ===");
        String unicodeText = "Hello,世界,World,测试";
        String[] unicodeParts = unicodeText.split(",");
        System.out.println("Unicode split: " + unicodeText);
        System.out.print("Parts: ");
        for (String part : unicodeParts) {
            System.out.print("'" + part + "' ");
        }
        System.out.println();

        String rejoinedUnicode = String.join(" | ", unicodeParts);
        System.out.println("Rejoined with ' | ': " + rejoinedUnicode);
    }

    private static void testEmojiSplitJoin() {
        System.out.println("\n=== Emoji split/join Tests ===");
        String emojiText = "😀,😃,😄,😁";
        String[] emojiParts = emojiText.split(",");
        System.out.println("Emoji split: " + emojiText);
        System.out.print("Parts: ");
        for (String part : emojiParts) {
            System.out.print("'" + part + "' ");
        }
        System.out.println();

        String rejoinedEmoji = String.join(" -> ", emojiParts);
        System.out.println("Rejoined emoji: " + rejoinedEmoji);
    }
}
