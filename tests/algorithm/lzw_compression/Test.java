import java.util.*;

public class Test {
    public static List<Integer> compress(String input) {
        Map<String, Integer> dictionary = new HashMap<>();

        // Initialize dictionary with single characters
        for (int i = 0; i < 256; i++) {
            dictionary.put(String.valueOf((char) i), i);
        }

        String current = "";
        List<Integer> result = new ArrayList<>();
        int dictSize = 256;

        for (char c : input.toCharArray()) {
            String combined = current + c;

            if (dictionary.containsKey(combined)) {
                current = combined;
            } else {
                result.add(dictionary.get(current));
                dictionary.put(combined, dictSize++);
                current = String.valueOf(c);
            }
        }

        if (!current.isEmpty()) {
            result.add(dictionary.get(current));
        }

        return result;
    }

    public static String decompress(List<Integer> compressed) {
        Map<Integer, String> dictionary = new HashMap<>();

        // Initialize dictionary with single characters
        for (int i = 0; i < 256; i++) {
            dictionary.put(i, String.valueOf((char) i));
        }

        String current = dictionary.get(compressed.get(0));
        StringBuilder result = new StringBuilder(current);
        int dictSize = 256;

        for (int i = 1; i < compressed.size(); i++) {
            int code = compressed.get(i);
            String entry;

            if (dictionary.containsKey(code)) {
                entry = dictionary.get(code);
            } else if (code == dictSize) {
                entry = current + current.charAt(0);
            } else {
                throw new IllegalArgumentException("Invalid compressed data");
            }

            result.append(entry);
            dictionary.put(dictSize++, current + entry.charAt(0));
            current = entry;
        }

        return result.toString();
    }

    public static void main(String[] args) {
        String input = "ABABCABABA";
        System.out.println("Original: " + input);
        System.out.println("Original size: " + input.length() + " characters");

        List<Integer> compressed = compress(input);
        System.out.println("Compressed: " + compressed);
        System.out.println("Compressed size: " + compressed.size() + " codes");

        String decompressed = decompress(compressed);
        System.out.println("Decompressed: " + decompressed);
        System.out.println("Successful: " + input.equals(decompressed));

        double compressionRatio = (double) compressed.size() / input.length();
        System.out.println("Compression ratio: " + compressionRatio);
    }
}
