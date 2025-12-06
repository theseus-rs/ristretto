/** Test String edge cases and error handling */
public class Test {
    public static void main(String[] args) {
        testNullHandling();
        testBoundaryConditions();
        testLargeStrings();
        testEmptyStringEdgeCases();
        testSingleCharacterStrings();
        testSpecialCharacters();
        testWhitespaceHandling();
        testSurrogatePairs();
        testMinMaxValues();
        testStringPoolBehavior();
        testStringBuilderEdgeCases();
    }

    private static void testNullHandling() {
        System.out.println("=== Null Handling Tests ===");
        String nullStr = null;
        String normalStr = "Hello";

        // Test String.valueOf with null
        System.out.println("String.valueOf(null): " + String.valueOf(nullStr));

        // Test concat with null - should throw NullPointerException
        try {
            String result = normalStr.concat(null);
            System.out.println("concat(null) succeeded: " + result);
        } catch (NullPointerException e) {
            System.out.println("concat(null) threw NullPointerException");
        }

        // Test equals with null
        System.out.println("'Hello'.equals(null): " + normalStr.equals(null));

        // Test null.equals() - should throw NullPointerException
        try {
            boolean result = nullStr.equals("Hello");
            System.out.println("null.equals('Hello'): " + result);
        } catch (NullPointerException e) {
            System.out.println("null.equals('Hello') threw NullPointerException");
        }

        // Test indexOf with null
        try {
            int result = normalStr.indexOf(null);
            System.out.println("indexOf(null): " + result);
        } catch (NullPointerException e) {
            System.out.println("indexOf(null) threw NullPointerException");
        }
    }

    private static void testBoundaryConditions() {
        System.out.println("\n=== Boundary Condition Tests ===");
        String test = "Hello";

        // Test charAt with invalid indices
        try {
            char c = test.charAt(-1);
            System.out.println("charAt(-1): " + c);
        } catch (StringIndexOutOfBoundsException e) {
            System.out.println("charAt(-1) threw StringIndexOutOfBoundsException");
        }

        try {
            char c = test.charAt(test.length());
            System.out.println("charAt(length): " + c);
        } catch (StringIndexOutOfBoundsException e) {
            System.out.println("charAt(length) threw StringIndexOutOfBoundsException");
        }

        // Test substring with invalid indices
        try {
            String result = test.substring(-1);
            System.out.println("substring(-1): " + result);
        } catch (StringIndexOutOfBoundsException e) {
            System.out.println("substring(-1) threw StringIndexOutOfBoundsException");
        }

        try {
            String result = test.substring(0, -1);
            System.out.println("substring(0, -1): " + result);
        } catch (StringIndexOutOfBoundsException e) {
            System.out.println("substring(0, -1) threw StringIndexOutOfBoundsException");
        }

        try {
            String result = test.substring(2, 1);
            System.out.println("substring(2, 1): " + result);
        } catch (StringIndexOutOfBoundsException e) {
            System.out.println("substring(2, 1) threw StringIndexOutOfBoundsException");
        }

        try {
            String result = test.substring(0, test.length() + 1);
            System.out.println("substring(0, length+1): " + result);
        } catch (StringIndexOutOfBoundsException e) {
            System.out.println("substring(0, length+1) threw StringIndexOutOfBoundsException");
        }
    }

    private static void testLargeStrings() {
        System.out.println("\n=== Large String Tests ===");
        StringBuilder largeBuilder = new StringBuilder();
        for (int i = 0; i < 10000; i++) {
            largeBuilder.append("X");
        }
        String largeString = largeBuilder.toString();
        System.out.println("Large string length: " + largeString.length());
        System.out.println("Large string charAt(5000): " + largeString.charAt(5000));
        System.out.println("Large string substring(4999, 5001): '" + largeString.substring(4999, 5001) + "'");
    }

    private static void testEmptyStringEdgeCases() {
        System.out.println("\n=== Empty String Edge Cases ===");
        String empty = "";
        System.out.println("Empty string length: " + empty.length());
        System.out.println("Empty string isEmpty(): " + empty.isEmpty());
        System.out.println("Empty string equals(''): " + empty.equals(""));
        System.out.println("Empty string compareTo(''): " + empty.compareTo(""));
        System.out.println("Empty string concat('test'): '" + empty.concat("test") + "'");
        System.out.println("Empty string indexOf(''): " + empty.indexOf(""));
        System.out.println("Empty string contains(''): " + empty.contains(""));
        System.out.println("Empty string startsWith(''): " + empty.startsWith(""));
        System.out.println("Empty string endsWith(''): " + empty.endsWith(""));
    }

    private static void testSingleCharacterStrings() {
        System.out.println("\n=== Single Character Tests ===");
        String single = "A";
        System.out.println("Single char length: " + single.length());
        System.out.println("Single char charAt(0): " + single.charAt(0));
        System.out.println("Single char substring(0, 1): '" + single.substring(0, 1) + "'");
        System.out.println("Single char substring(1): '" + single.substring(1) + "'");
        System.out.println("Single char indexOf('A'): " + single.indexOf('A'));
        System.out.println("Single char indexOf('B'): " + single.indexOf('B'));
    }

    private static void testSpecialCharacters() {
        System.out.println("\n=== Special Character Tests ===");
        String special = "\\t\\n\\r\\f\\b\\\"\\'\\\\\\/";
        System.out.println("Special chars: " + special);
        System.out.println("Special chars length: " + special.length());
    }

    private static void testWhitespaceHandling() {
        String whitespace = " \\t\\n\\r\\f";
        System.out.println("Whitespace string: '" + whitespace + "'");
        System.out.println("Whitespace length: " + whitespace.length());
        System.out.println("Whitespace trim(): '" + whitespace.trim() + "'");
        System.out.println("Whitespace trim() length: " + whitespace.trim().length());
    }

    private static void testSurrogatePairs() {
        System.out.println("\n=== Surrogate Pair Edge Cases ===");
        String emoji = "🙂";
        System.out.println("Emoji: " + emoji);
        System.out.println("Emoji length: " + emoji.length());
        System.out.println("Emoji charAt(0): " + emoji.charAt(0) + " (code: " + (int)emoji.charAt(0) + ")");
        System.out.println("Emoji charAt(1): " + emoji.charAt(1) + " (code: " + (int)emoji.charAt(1) + ")");

        // Test broken surrogate pairs
        String brokenSurrogate = "\\uD800"; // High surrogate without low surrogate
        System.out.println("Broken surrogate: " + brokenSurrogate);
        System.out.println("Broken surrogate length: " + brokenSurrogate.length());
    }

    private static void testMinMaxValues() {
        System.out.println("\n=== Min/Max Value Tests ===");
        String maxChar = String.valueOf(Character.MAX_VALUE);
        String minChar = String.valueOf(Character.MIN_VALUE);
        System.out.println("Max char value: " + (int)Character.MAX_VALUE);
        System.out.println("Min char value: " + (int)Character.MIN_VALUE);
        System.out.println("Max char string: '" + maxChar + "'");
        System.out.println("Min char string: '" + minChar + "'");
    }

    private static void testStringPoolBehavior() {
        System.out.println("\n=== String Pool Tests ===");
        String literal1 = "Hello";
        String literal2 = "Hello";
        String constructed = new String("Hello");
        String interned = constructed.intern();

        System.out.println("literal1 == literal2: " + (literal1 == literal2));
        System.out.println("literal1 == constructed: " + (literal1 == constructed));
        System.out.println("literal1 == interned: " + (literal1 == interned));
        System.out.println("literal1.equals(constructed): " + literal1.equals(constructed));
        System.out.println("literal1.equals(interned): " + literal1.equals(interned));
    }

    private static void testStringBuilderEdgeCases() {
        System.out.println("\n=== StringBuilder/StringBuffer Edge Cases ===");
        StringBuilder sb = new StringBuilder();

        // Test with capacity 0
        try {
            StringBuilder sb0 = new StringBuilder(0);
            System.out.println("StringBuilder(0) capacity: " + sb0.capacity());
        } catch (Exception e) {
            System.out.println("StringBuilder(0) threw exception: " + e.getClass().getSimpleName());
        }

        // Test with negative capacity
        try {
            StringBuilder sbNeg = new StringBuilder(-1);
            System.out.println("StringBuilder(-1) capacity: " + sbNeg.capacity());
        } catch (Exception e) {
            System.out.println("StringBuilder(-1) threw exception: " + e.getClass().getSimpleName());
        }

        // Test setLength with negative value
        try {
            sb.append("Hello");
            sb.setLength(-1);
            System.out.println("setLength(-1) succeeded");
        } catch (Exception e) {
            System.out.println("setLength(-1) threw exception: " + e.getClass().getSimpleName());
        }

        // Test insert at invalid position
        try {
            StringBuilder sb2 = new StringBuilder("Hello");
            sb2.insert(-1, "X");
            System.out.println("insert(-1, 'X') succeeded");
        } catch (Exception e) {
            System.out.println("insert(-1, 'X') threw exception: " + e.getClass().getSimpleName());
        }

        try {
            StringBuilder sb3 = new StringBuilder("Hello");
            sb3.insert(10, "X");
            System.out.println("insert(10, 'X') succeeded");
        } catch (Exception e) {
            System.out.println("insert(10, 'X') threw exception: " + e.getClass().getSimpleName());
        }
    }
}
