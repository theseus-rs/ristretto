/** Test String internationalization and locale-specific operations */
public class Test {
    public static void main(String[] args) {
        testUnicodeCategories();
        testMixedScripts();
        testDiacriticsAndCombiningCharacters();
        testEmojiAndSurrogatePairs();
        testNormalization();
        testCaseConversionEdgeCases();
        testInternationalSorting();
        testCrossScriptOperations();
        testCodePointMethods();
        testByteRepresentation();
    }

    private static void testUnicodeCategories() {
        System.out.println("=== Unicode Category Tests ===");

        // Latin characters
        String latin = "Hello Café";
        System.out.println("Latin: " + latin);
        System.out.println("Latin length: " + latin.length());
        System.out.println("Latin toUpperCase(): " + latin.toUpperCase());
        System.out.println("Latin toLowerCase(): " + latin.toLowerCase());

        // Cyrillic characters
        String cyrillic = "Привет мир";
        System.out.println("Cyrillic: " + cyrillic);
        System.out.println("Cyrillic length: " + cyrillic.length());
        System.out.println("Cyrillic toUpperCase(): " + cyrillic.toUpperCase());
        System.out.println("Cyrillic toLowerCase(): " + cyrillic.toLowerCase());

        // Greek characters
        String greek = "Γεια σας κόσμος";
        System.out.println("Greek: " + greek);
        System.out.println("Greek length: " + greek.length());
        System.out.println("Greek toUpperCase(): " + greek.toUpperCase());
        System.out.println("Greek toLowerCase(): " + greek.toLowerCase());

        // Arabic characters (RTL)
        String arabic = "مرحبا بالعالم";
        System.out.println("Arabic: " + arabic);
        System.out.println("Arabic length: " + arabic.length());
        System.out.println("Arabic toUpperCase(): " + arabic.toUpperCase());
        System.out.println("Arabic toLowerCase(): " + arabic.toLowerCase());

        // Hebrew characters (RTL)
        String hebrew = "שלום עולם";
        System.out.println("Hebrew: " + hebrew);
        System.out.println("Hebrew length: " + hebrew.length());
        System.out.println("Hebrew toUpperCase(): " + hebrew.toUpperCase());
        System.out.println("Hebrew toLowerCase(): " + hebrew.toLowerCase());

        // Chinese characters
        String chinese = "你好世界";
        System.out.println("Chinese: " + chinese);
        System.out.println("Chinese length: " + chinese.length());
        System.out.println("Chinese toUpperCase(): " + chinese.toUpperCase());
        System.out.println("Chinese toLowerCase(): " + chinese.toLowerCase());

        // Japanese characters (Hiragana, Katakana, Kanji)
        String japanese = "こんにちは世界 コンニチハ";
        System.out.println("Japanese: " + japanese);
        System.out.println("Japanese length: " + japanese.length());
        System.out.println("Japanese toUpperCase(): " + japanese.toUpperCase());
        System.out.println("Japanese toLowerCase(): " + japanese.toLowerCase());

        // Korean characters
        String korean = "안녕하세요 세계";
        System.out.println("Korean: " + korean);
        System.out.println("Korean length: " + korean.length());
        System.out.println("Korean toUpperCase(): " + korean.toUpperCase());
        System.out.println("Korean toLowerCase(): " + korean.toLowerCase());
    }

    private static void testMixedScripts() {
        System.out.println("\n=== Mixed Script Tests ===");
        String mixed = "Hello 世界 мир עולם";
        System.out.println("Mixed scripts: " + mixed);
        System.out.println("Mixed length: " + mixed.length());
        System.out.println("Mixed toUpperCase(): " + mixed.toUpperCase());
        System.out.println("Mixed toLowerCase(): " + mixed.toLowerCase());
    }

    private static void testDiacriticsAndCombiningCharacters() {
        System.out.println("\n=== Diacritics and Combining Characters Tests ===");
        String diacritics = "café naïve résumé";
        System.out.println("Diacritics: " + diacritics);
        System.out.println("Diacritics length: " + diacritics.length());
        System.out.println("Diacritics toUpperCase(): " + diacritics.toUpperCase());
        System.out.println("Diacritics toLowerCase(): " + diacritics.toLowerCase());

        // Test combining characters
        String combining = "e\\u0301"; // e + combining acute accent = é
        String precomposed = "é";      // precomposed é
        System.out.println("Combining e + acute: " + combining);
        System.out.println("Precomposed é: " + precomposed);
        System.out.println("Combining length: " + combining.length());
        System.out.println("Precomposed length: " + precomposed.length());
        System.out.println("Are they equal? " + combining.equals(precomposed));
    }

    private static void testEmojiAndSurrogatePairs() {
        System.out.println("\n=== Emoji and Surrogate Pair Tests ===");
        String emoji = "😀😃😄😁😊😉😍";
        System.out.println("Emoji: " + emoji);
        System.out.println("Emoji length: " + emoji.length());
        System.out.println("Emoji codePointCount: " + emoji.codePointCount(0, emoji.length()));

        // Test emoji with skin tone modifiers
        String emojiWithModifier = "👋🏻👋🏼👋🏽👋🏾👋🏿";
        System.out.println("Emoji with skin tone: " + emojiWithModifier);
        System.out.println("Emoji with modifier length: " + emojiWithModifier.length());
        System.out.println("Emoji with modifier codePointCount: " + emojiWithModifier.codePointCount(0, emojiWithModifier.length()));

        // Test flag emoji (regional indicator sequences)
        String flags = "🇺🇸🇬🇧🇫🇷🇩🇪🇯🇵";
        System.out.println("Flag emoji: " + flags);
        System.out.println("Flag emoji length: " + flags.length());
        System.out.println("Flag emoji codePointCount: " + flags.codePointCount(0, flags.length()));

        // Test complex emoji sequences
        String complexEmoji = "👨‍👩‍👧‍👦👨‍💻👩‍🚀";
        System.out.println("Complex emoji: " + complexEmoji);
        System.out.println("Complex emoji length: " + complexEmoji.length());
        System.out.println("Complex emoji codePointCount: " + complexEmoji.codePointCount(0, complexEmoji.length()));
    }

    private static void testNormalization() {
        System.out.println("\n=== Normalization Tests ===");
        String str1 = "Åpfel"; // A with ring above
        String str2 = "A\\u030Apfel"; // A + combining ring above
        System.out.println("Precomposed Åpfel: " + str1);
        System.out.println("Decomposed Åpfel: " + str2);
        System.out.println("Precomposed length: " + str1.length());
        System.out.println("Decomposed length: " + str2.length());
        System.out.println("Are they equal? " + str1.equals(str2));
    }

    private static void testCaseConversionEdgeCases() {
        System.out.println("\n=== Case Conversion Edge Cases ===");

        // German ß character
        String german = "Straße";
        System.out.println("German ß: " + german);
        System.out.println("German toUpperCase(): " + german.toUpperCase());
        System.out.println("German toLowerCase(): " + german.toLowerCase());

        // Turkish i/I conversion
        String turkish = "İstanbul";
        System.out.println("Turkish İ: " + turkish);
        System.out.println("Turkish toUpperCase(): " + turkish.toUpperCase());
        System.out.println("Turkish toLowerCase(): " + turkish.toLowerCase());
    }

    private static void testInternationalSorting() {
        System.out.println("\n=== International Sorting Tests ===");
        String[] words = {"apple", "ápple", "café", "zebra", "Zebra", "naïve"};
        System.out.println("Original order:");
        for (String word : words) {
            System.out.println("  " + word);
        }

        // Sort using natural string comparison
        java.util.Arrays.sort(words);
        System.out.println("After Arrays.sort():");
        for (String word : words) {
            System.out.println("  " + word);
        }
    }

    private static void testCrossScriptOperations() {
        System.out.println("\n=== Cross-Script Operations Tests ===");
        String english = "Hello";
        String russian = "Привет";
        String chinese = "你好";  // Renamed to avoid conflict

        System.out.println("English + Russian: " + english + " " + russian);
        System.out.println("Russian + Chinese: " + russian + " " + chinese);
        System.out.println("All three: " + english + " " + russian + " " + chinese);

        // Test indexOf with international characters
        String international = "Hello мир 世界 שלום";
        System.out.println("International string: " + international);
        System.out.println("indexOf('мир'): " + international.indexOf("мир"));
        System.out.println("indexOf('世界'): " + international.indexOf("世界"));
        System.out.println("indexOf('שלום'): " + international.indexOf("שלום"));

        // Test substring with international characters
        System.out.println("substring(0, 5): '" + international.substring(0, 5) + "'");
        System.out.println("substring(6, 9): '" + international.substring(6, 9) + "'");
        System.out.println("substring(10, 12): '" + international.substring(10, 12) + "'");

        // Test replace with international characters
        String replaced = international.replace("мир", "world");
        System.out.println("After replace('мир', 'world'): " + replaced);

        // Test split with international delimiters
        String delimited = "apple•banana•cherry•date";
        String[] parts = delimited.split("•");
        System.out.println("Split by bullet character:");
        for (String part : parts) {
            System.out.println("  '" + part + "'");
        }
    }

    private static void testCodePointMethods() {
        System.out.println("\n=== Code Point Tests ===");
        String codePointTest = "A𝐁𝐂"; // A, Mathematical Bold B, Mathematical Bold C
        System.out.println("Code point test string: " + codePointTest);
        System.out.println("Length: " + codePointTest.length());
        System.out.println("Code point count: " + codePointTest.codePointCount(0, codePointTest.length()));

        for (int i = 0; i < codePointTest.length(); ) {
            int codePoint = codePointTest.codePointAt(i);
            System.out.println("Code point at " + i + ": " + codePoint + " (char: " +
                             Character.toString(codePoint) + ")");
            i += Character.charCount(codePoint);
        }
    }

    private static void testByteRepresentation() {
        System.out.println("\n=== Byte Representation Tests ===");
        String[] testStrings = {"Hello", "café", "世界", "🙂"};

        for (String str : testStrings) {
            byte[] bytes = str.getBytes();
            System.out.print("'" + str + "' bytes: ");
            for (byte b : bytes) {
                System.out.print(String.format("%02X ", b & 0xFF));
            }
            System.out.println("(length: " + bytes.length + ")");
        }
    }
}
