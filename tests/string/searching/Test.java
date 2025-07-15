/** Test String searching methods like indexOf, lastIndexOf, contains, startsWith, endsWith */
public class Test {
    public static void main(String[] args) {
        testIndexOf();
        testLastIndexOf();
        testContains();
        testStartsWith();
        testEndsWith();
        testUnicodeSearching();
        testSearchingEdgeCases();
        testCaseSensitiveSearching();
    }

    private static void testIndexOf() {
        System.out.println("=== indexOf Tests ===");
        String text = "Hello World Hello";
        System.out.println("Text: " + text);
        System.out.println("indexOf('l'): " + text.indexOf('l'));
        System.out.println("indexOf('l', 3): " + text.indexOf('l', 3));
        System.out.println("indexOf('l', 10): " + text.indexOf('l', 10));
        System.out.println("indexOf('Hello'): " + text.indexOf("Hello"));
        System.out.println("indexOf('Hello', 1): " + text.indexOf("Hello", 1));
        System.out.println("indexOf('xyz'): " + text.indexOf("xyz"));
        System.out.println("indexOf(''): " + text.indexOf(""));
        System.out.println("indexOf('', 5): " + text.indexOf("", 5));
    }

    private static void testLastIndexOf() {
        System.out.println("\n=== lastIndexOf Tests ===");
        String text = "Hello World Hello";
        System.out.println("lastIndexOf('l'): " + text.lastIndexOf('l'));
        System.out.println("lastIndexOf('l', 10): " + text.lastIndexOf('l', 10));
        System.out.println("lastIndexOf('l', 2): " + text.lastIndexOf('l', 2));
        System.out.println("lastIndexOf('Hello'): " + text.lastIndexOf("Hello"));
        System.out.println("lastIndexOf('Hello', 10): " + text.lastIndexOf("Hello", 10));
        System.out.println("lastIndexOf('xyz'): " + text.lastIndexOf("xyz"));
        System.out.println("lastIndexOf(''): " + text.lastIndexOf(""));
        System.out.println("lastIndexOf('', 5): " + text.lastIndexOf("", 5));
    }

    private static void testContains() {
        System.out.println("\n=== contains Tests ===");
        String text = "Hello World Hello";
        System.out.println("contains('World'): " + text.contains("World"));
        System.out.println("contains('world'): " + text.contains("world"));
        System.out.println("contains('xyz'): " + text.contains("xyz"));
        System.out.println("contains(''): " + text.contains(""));
        System.out.println("contains('Hello World Hello'): " + text.contains("Hello World Hello"));
    }

    private static void testStartsWith() {
        System.out.println("\n=== startsWith Tests ===");
        String text = "Hello World Hello";
        System.out.println("startsWith('Hello'): " + text.startsWith("Hello"));
        System.out.println("startsWith('World'): " + text.startsWith("World"));
        System.out.println("startsWith('World', 6): " + text.startsWith("World", 6));
        System.out.println("startsWith('Hello', 12): " + text.startsWith("Hello", 12));
        System.out.println("startsWith(''): " + text.startsWith(""));
        System.out.println("startsWith('', 5): " + text.startsWith("", 5));
    }

    private static void testEndsWith() {
        System.out.println("\n=== endsWith Tests ===");
        String text = "Hello World Hello";
        System.out.println("endsWith('Hello'): " + text.endsWith("Hello"));
        System.out.println("endsWith('World'): " + text.endsWith("World"));
        System.out.println("endsWith('o'): " + text.endsWith("o"));
        System.out.println("endsWith(''): " + text.endsWith(""));
        System.out.println("endsWith('Hello World Hello'): " + text.endsWith("Hello World Hello"));
    }

    private static void testUnicodeSearching() {
        System.out.println("\n=== Unicode Search Tests ===");
        String unicode = "Hello 世界 World 世界";
        System.out.println("Unicode text: " + unicode);
        System.out.println("indexOf('世'): " + unicode.indexOf('世'));
        System.out.println("indexOf('世界'): " + unicode.indexOf("世界"));
        System.out.println("lastIndexOf('世界'): " + unicode.lastIndexOf("世界"));
        System.out.println("contains('世界'): " + unicode.contains("世界"));
        System.out.println("startsWith('Hello'): " + unicode.startsWith("Hello"));
        System.out.println("endsWith('世界'): " + unicode.endsWith("世界"));
    }

    private static void testSearchingEdgeCases() {
        System.out.println("\n=== Edge Cases ===");
        String empty = "";
        System.out.println("Empty string tests:");
        System.out.println("''.indexOf('a'): " + empty.indexOf('a'));
        System.out.println("''.contains(''): " + empty.contains(""));
        System.out.println("''.startsWith(''): " + empty.startsWith(""));
        System.out.println("''.endsWith(''): " + empty.endsWith(""));

        // Test with null character
        String withNull = "Hello\\u0000World";
        System.out.println("String with null char: " + withNull);
        System.out.println("indexOf('\\u0000'): " + withNull.indexOf('\u0000'));
        System.out.println("contains('\\u0000'): " + withNull.contains("\u0000"));
    }

    private static void testCaseSensitiveSearching() {
        System.out.println("\n=== Case Sensitivity Tests ===");
        String caseTest = "Hello WORLD hello";
        System.out.println("Text: " + caseTest);
        System.out.println("indexOf('hello'): " + caseTest.indexOf("hello"));
        System.out.println("indexOf('HELLO'): " + caseTest.indexOf("HELLO"));
        System.out.println("lastIndexOf('hello'): " + caseTest.lastIndexOf("hello"));
        System.out.println("contains('WORLD'): " + caseTest.contains("WORLD"));
        System.out.println("contains('world'): " + caseTest.contains("world"));
    }
}
