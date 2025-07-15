/** Test String conversion and formatting methods like valueOf, format, toString */
public class Test {
    public static void main(String[] args) {
        testStringValueOf();
        testStringFormat();
        testStringFormatAdvanced();
        testGetBytes();
        testToCharArray();
        testCopyValueOf();
        testStringConstructors();
        testMatches();
        testRegionMatches();
    }

    private static void testStringValueOf() {
        System.out.println("=== String.valueOf Tests ===");
        System.out.println("valueOf(true): " + String.valueOf(true));
        System.out.println("valueOf(false): " + String.valueOf(false));
        System.out.println("valueOf('A'): " + String.valueOf('A'));
        System.out.println("valueOf(42): " + String.valueOf(42));
        System.out.println("valueOf(42L): " + String.valueOf(42L));
        System.out.println("valueOf(3.14f): " + String.valueOf(3.14f));
        System.out.println("valueOf(3.14159): " + String.valueOf(3.14159));

        // Test valueOf with char array
        char[] charArray = {'H', 'e', 'l', 'l', 'o'};
        System.out.println("valueOf(char[]): " + String.valueOf(charArray));
        System.out.println("valueOf(char[], 1, 3): " + String.valueOf(charArray, 1, 3));

        // Test valueOf with Object
        Object obj = new Object();
        System.out.println("valueOf(Object): " + String.valueOf(obj));
        System.out.println("valueOf(null): " + String.valueOf((Object)null));
    }

    private static void testStringFormat() {
        System.out.println("\n=== String.format Tests ===");
        System.out.println("format('%s', 'Hello'): " + String.format("%s", "Hello"));
        System.out.println("format('%d', 42): " + String.format("%d", 42));
        System.out.println("format('%f', 3.14): " + String.format("%f", 3.14));
        System.out.println("format('%.2f', 3.14159): " + String.format("%.2f", 3.14159));
        System.out.println("format('%x', 255): " + String.format("%x", 255));
        System.out.println("format('%o', 8): " + String.format("%o", 8));
        System.out.println("format('%c', 65): " + String.format("%c", 65));
        System.out.println("format('%b', true): " + String.format("%b", true));
    }

    private static void testStringFormatAdvanced() {
        System.out.println("\n=== Advanced String.format Tests ===");
        // Test format with multiple arguments
        System.out.println("format('%s %d %.2f', 'Hello', 42, 3.14159): " +
                          String.format("%s %d %.2f", "Hello", 42, 3.14159));

        // Test format with positional arguments
        System.out.println("format('%2$s %1$d', 42, 'Hello'): " +
                          String.format("%2$s %1$d", 42, "Hello"));

        // Test format with width and alignment
        System.out.println("format('%10s', 'Hello'): '" + String.format("%10s", "Hello") + "'");
        System.out.println("format('%-10s', 'Hello'): '" + String.format("%-10s", "Hello") + "'");
        System.out.println("format('%05d', 42): " + String.format("%05d", 42));
    }

    private static void testGetBytes() {
        System.out.println("\n=== getBytes Tests ===");
        String text = "Hello World";
        byte[] bytes = text.getBytes();
        System.out.print("getBytes(): ");
        for (byte b : bytes) {
            System.out.print(b + " ");
        }
        System.out.println();

        // Test getBytes with unicode
        String unicode = "Hello 世界";
        byte[] unicodeBytes = unicode.getBytes();
        System.out.print("Unicode getBytes(): ");
        for (byte b : unicodeBytes) {
            System.out.print(b + " ");
        }
        System.out.println();
    }

    private static void testToCharArray() {
        System.out.println("\n=== toCharArray Tests ===");
        String str = "Hello";
        char[] chars = str.toCharArray();
        System.out.print("toCharArray(): ");
        for (char c : chars) {
            System.out.print(c + " ");
        }
        System.out.println();

        // Test with emoji
        String emoji = "😀😃😄";
        char[] emojiChars = emoji.toCharArray();
        System.out.print("Emoji toCharArray(): ");
        for (char c : emojiChars) {
            System.out.print("\\u" + Integer.toHexString(c) + " ");
        }
        System.out.println();
    }

    private static void testCopyValueOf() {
        System.out.println("\n=== copyValueOf Tests ===");
        char[] data = {'H', 'e', 'l', 'l', 'o', ' ', 'W', 'o', 'r', 'l', 'd'};
        System.out.println("copyValueOf(data): " + String.copyValueOf(data));
        System.out.println("copyValueOf(data, 0, 5): " + String.copyValueOf(data, 0, 5));
        System.out.println("copyValueOf(data, 6, 5): " + String.copyValueOf(data, 6, 5));
    }

    private static void testStringConstructors() {
        System.out.println("\n=== String Constructor Tests ===");
        char[] charArray = {'H', 'e', 'l', 'l', 'o'};
        String text = "Hello World";
        byte[] bytes = text.getBytes();

        String s1 = new String();
        System.out.println("new String(): '" + s1 + "'");

        String s2 = new String("Hello");
        System.out.println("new String('Hello'): " + s2);

        String s3 = new String(charArray);
        System.out.println("new String(charArray): " + s3);

        String s4 = new String(charArray, 1, 3);
        System.out.println("new String(charArray, 1, 3): " + s4);

        String s5 = new String(bytes);
        System.out.println("new String(bytes): " + s5);
    }

    private static void testMatches() {
        System.out.println("\n=== matches Tests ===");
        String pattern = "Hello";
        System.out.println("'Hello'.matches('Hello'): " + pattern.matches("Hello"));
        System.out.println("'Hello'.matches('H.*o'): " + pattern.matches("H.*o"));
        System.out.println("'Hello123'.matches('.*\\\\d+'): " + "Hello123".matches(".*\\d+"));
        System.out.println("'abc'.matches('[a-z]+'): " + "abc".matches("[a-z]+"));
    }

    private static void testRegionMatches() {
        System.out.println("\n=== regionMatches Tests ===");
        String s = "Hello World";
        String other = "hello world";
        System.out.println("regionMatches(0, 'Hello', 0, 5): " + s.regionMatches(0, "Hello", 0, 5));
        System.out.println("regionMatches(6, 'World', 0, 5): " + s.regionMatches(6, "World", 0, 5));
        System.out.println("regionMatches(true, 0, 'hello', 0, 5): " + s.regionMatches(true, 0, "hello", 0, 5));
        System.out.println("regionMatches(false, 0, 'hello', 0, 5): " + s.regionMatches(false, 0, "hello", 0, 5));
    }
}
