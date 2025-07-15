public class Test {
    private static final int NO_OF_CHARS = 256;

    public static void boyerMooreSearch(String text, String pattern) {
        int[] badChar = new int[NO_OF_CHARS];

        // Fill the bad character array
        badCharHeuristic(pattern, badChar);

        int shift = 0;
        System.out.println("Searching for pattern '" + pattern + "' in text '" + text + "'");

        while (shift <= (text.length() - pattern.length())) {
            int j = pattern.length() - 1;

            // Keep reducing index j of pattern while characters match
            while (j >= 0 && pattern.charAt(j) == text.charAt(shift + j)) {
                j--;
            }

            // If pattern is present at current shift
            if (j < 0) {
                System.out.println("Pattern found at index " + shift);
                shift += (shift + pattern.length() < text.length()) ?
                    pattern.length() - badChar[text.charAt(shift + pattern.length())] : 1;
            } else {
                // Shift the pattern so that the bad character aligns with the last occurrence
                shift += Math.max(1, j - badChar[text.charAt(shift + j)]);
            }
        }
    }

    private static void badCharHeuristic(String pattern, int[] badChar) {
        // Initialize all occurrences as -1
        for (int i = 0; i < NO_OF_CHARS; i++) {
            badChar[i] = -1;
        }

        // Fill the actual value of last occurrence of a character
        for (int i = 0; i < pattern.length(); i++) {
            badChar[(int) pattern.charAt(i)] = i;
        }
    }

    public static void main(String[] args) {
        String text = "ABAAABCDABCDABCABCDABDE";
        String pattern = "ABCAB";

        boyerMooreSearch(text, pattern);
    }
}

