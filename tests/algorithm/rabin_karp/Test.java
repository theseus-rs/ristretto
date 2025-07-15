public class Test {
    private static final int PRIME = 101;

    public static void rabinKarpSearch(String text, String pattern) {
        int patternLength = pattern.length();
        int textLength = text.length();
        int patternHash = 0;
        int textHash = 0;
        int h = 1;

        // Calculate h = pow(256, patternLength-1) % PRIME
        for (int i = 0; i < patternLength - 1; i++) {
            h = (h * 256) % PRIME;
        }

        // Calculate hash value for pattern and first window of text
        for (int i = 0; i < patternLength; i++) {
            patternHash = (256 * patternHash + pattern.charAt(i)) % PRIME;
            textHash = (256 * textHash + text.charAt(i)) % PRIME;
        }

        System.out.println("Searching for pattern '" + pattern + "' in text '" + text + "'");

        // Slide the pattern over text one by one
        for (int i = 0; i <= textLength - patternLength; i++) {
            // Check if hash values match
            if (patternHash == textHash) {
                // Check characters one by one
                boolean match = true;
                for (int j = 0; j < patternLength; j++) {
                    if (text.charAt(i + j) != pattern.charAt(j)) {
                        match = false;
                        break;
                    }
                }

                if (match) {
                    System.out.println("Pattern found at index " + i);
                }
            }

            // Calculate hash value for next window
            if (i < textLength - patternLength) {
                textHash = (256 * (textHash - text.charAt(i) * h) + text.charAt(i + patternLength)) % PRIME;

                // Convert negative value to positive
                if (textHash < 0) {
                    textHash = textHash + PRIME;
                }
            }
        }
    }

    public static void main(String[] args) {
        String text = "GEEKS FOR GEEKS";
        String pattern = "GEEK";

        rabinKarpSearch(text, pattern);
    }
}

