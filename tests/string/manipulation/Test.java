/** Test String manipulation methods like substring, concat, replace, trim */
public class Test {
    public static void main(String[] args) {
        testSubstring();
        testConcat();
        testReplace();
        testReplaceAll();
        testReplaceFirst();
        testTrim();
        testCaseConversion();
        testInternationalCaseConversion();
        testIntern();
    }

    private static void testSubstring() {
        System.out.println("=== substring Tests ===");
        String str = "Hello World";
        System.out.println("Original: " + str);
        System.out.println("substring(0, 5): " + str.substring(0, 5));
        System.out.println("substring(6): " + str.substring(6));
        System.out.println("substring(0, 0): " + str.substring(0, 0));
        System.out.println("substring(5, 5): " + str.substring(5, 5));
        System.out.println("substring(0, length): " + str.substring(0, str.length()));
    }

    private static void testConcat() {
        System.out.println("\n=== concat Tests ===");
        String a = "Hello";
        String b = " World";
        String c = "";
        System.out.println("'Hello'.concat(' World'): " + a.concat(b));
        System.out.println("'Hello'.concat(''): " + a.concat(c));
        System.out.println("''.concat('World'): " + c.concat("World"));
    }

    private static void testReplace() {
        System.out.println("\n=== replace Tests ===");
        String original = "Hello World Hello";
        System.out.println("Original: " + original);
        System.out.println("replace('l', 'L'): " + original.replace('l', 'L'));
        System.out.println("replace('Hello', 'Hi'): " + original.replace("Hello", "Hi"));
        System.out.println("replace('xyz', 'abc'): " + original.replace("xyz", "abc"));
        System.out.println("replace('', 'X'): " + original.replace("", "X"));
    }

    private static void testReplaceAll() {
        System.out.println("\n=== replaceAll Tests ===");
        String text = "Hello123World456";
        System.out.println("Original: " + text);
        System.out.println("replaceAll('\\\\d+', 'X'): " + text.replaceAll("\\d+", "X"));
        System.out.println("replaceAll('[aeiou]', '*'): " + text.replaceAll("[aeiou]", "*"));
    }

    private static void testReplaceFirst() {
        System.out.println("\n=== replaceFirst Tests ===");
        String text2 = "abc abc abc";
        System.out.println("Original: " + text2);
        System.out.println("replaceFirst('abc', 'XYZ'): " + text2.replaceFirst("abc", "XYZ"));
        System.out.println("replaceFirst('\\\\w+', 'WORD'): " + text2.replaceFirst("\\w+", "WORD"));
    }

    private static void testTrim() {
        System.out.println("\n=== trim Tests ===");
        String[] trimTests = {
            "  Hello World  ",
            "\\t\\tTabbed\\t\\t",
            "\\n\\nNewlines\\n\\n",
            "   ",
            "",
            "NoSpaces",
            " Leading",
            "Trailing "
        };

        for (String test : trimTests) {
            System.out.println("'" + test + "' -> '" + test.trim() + "'");
        }
    }

    private static void testCaseConversion() {
        System.out.println("\n=== Case Conversion Tests ===");
        String mixed = "Hello World 123";
        System.out.println("Original: " + mixed);
        System.out.println("toUpperCase(): " + mixed.toUpperCase());
        System.out.println("toLowerCase(): " + mixed.toLowerCase());
    }

    private static void testInternationalCaseConversion() {
        System.out.println("\n=== International Case Conversion Tests ===");
        String international = "Café München naïve";
        System.out.println("International: " + international);
        System.out.println("toUpperCase(): " + international.toUpperCase());
        System.out.println("toLowerCase(): " + international.toLowerCase());
    }

    private static void testIntern() {
        System.out.println("\n=== intern Tests ===");
        String s1 = new String("Hello");
        String s2 = new String("Hello");
        String s3 = s1.intern();
        String s4 = s2.intern();

        System.out.println("s1 == s2: " + (s1 == s2));
        System.out.println("s3 == s4: " + (s3 == s4));
        System.out.println("s1.equals(s2): " + s1.equals(s2));
        System.out.println("s3.equals(s4): " + s3.equals(s4));
    }
}
