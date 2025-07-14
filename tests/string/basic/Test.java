/** Test basic String operations like length, charAt, equals, compareTo */
public class Test {
    public static void main(String[] args) {
        testStringLength();
        testCharAt();
        testStringEquality();
        testEqualsIgnoreCase();
        testCompareTo();
        testCompareToIgnoreCase();
        testIsEmpty();
    }

    private static void testStringLength() {
        System.out.println("=== String Length Tests ===");
        String empty = "";
        String single = "A";
        String normal = "Hello";
        String unicode = "Hello 世界";

        System.out.println("Empty string length: " + empty.length());
        System.out.println("Single char length: " + single.length());
        System.out.println("Normal string length: " + normal.length());
        System.out.println("Unicode string length: " + unicode.length());
    }

    private static void testCharAt() {
        System.out.println("\n=== charAt Tests ===");
        String test = "Hello";
        for (int i = 0; i < test.length(); i++) {
            System.out.println("charAt(" + i + "): " + test.charAt(i));
        }

        // Test charAt with unicode
        String unicode2 = "A🙂B";
        System.out.println("Unicode charAt test: " + unicode2);
        for (int i = 0; i < unicode2.length(); i++) {
            System.out.println("charAt(" + i + "): " + unicode2.charAt(i) + " (code: " + (int)unicode2.charAt(i) + ")");
        }
    }

    private static void testStringEquality() {
        System.out.println("\n=== String Equality Tests ===");
        String s1 = "Hello";
        String s2 = "Hello";
        String s3 = "hello";
        String s4 = new String("Hello");

        System.out.println("s1.equals(s2): " + s1.equals(s2));
        System.out.println("s1.equals(s3): " + s1.equals(s3));
        System.out.println("s1.equals(s4): " + s1.equals(s4));
        System.out.println("s1 == s2: " + (s1 == s2));
        System.out.println("s1 == s4: " + (s1 == s4));
    }

    private static void testEqualsIgnoreCase() {
        System.out.println("\n=== equalsIgnoreCase Tests ===");
        String s1 = "Hello";
        String s3 = "hello";
        System.out.println("s1.equalsIgnoreCase(s3): " + s1.equalsIgnoreCase(s3));
        System.out.println("'HELLO'.equalsIgnoreCase('hello'): " + "HELLO".equalsIgnoreCase("hello"));
    }

    private static void testCompareTo() {
        System.out.println("\n=== compareTo Tests ===");
        System.out.println("'a'.compareTo('b'): " + "a".compareTo("b"));
        System.out.println("'b'.compareTo('a'): " + "b".compareTo("a"));
        System.out.println("'a'.compareTo('a'): " + "a".compareTo("a"));
        System.out.println("'Apple'.compareTo('apple'): " + "Apple".compareTo("apple"));
        System.out.println("'abc'.compareTo('ab'): " + "abc".compareTo("ab"));
    }

    private static void testCompareToIgnoreCase() {
        System.out.println("\n=== compareToIgnoreCase Tests ===");
        System.out.println("'Apple'.compareToIgnoreCase('apple'): " + "Apple".compareToIgnoreCase("apple"));
        System.out.println("'HELLO'.compareToIgnoreCase('hello'): " + "HELLO".compareToIgnoreCase("hello"));
    }

    private static void testIsEmpty() {
        System.out.println("\n=== isEmpty Tests ===");
        System.out.println("''.isEmpty(): " + "".isEmpty());
        System.out.println("' '.isEmpty(): " + " ".isEmpty());
        System.out.println("'Hello'.isEmpty(): " + "Hello".isEmpty());
    }
}
