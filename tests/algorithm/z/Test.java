public class Test {
    public static void zAlgorithmSearch(String text, String pattern) {
        String combined = pattern + "$" + text;
        int[] z = zArray(combined);

        System.out.println("Searching for pattern '" + pattern + "' in text '" + text + "'");

        for (int i = 0; i < combined.length(); i++) {
            if (z[i] == pattern.length()) {
                System.out.println("Pattern found at index " + (i - pattern.length() - 1));
            }
        }
    }

    private static int[] zArray(String str) {
        int n = str.length();
        int[] z = new int[n];
        int l = 0, r = 0;

        for (int i = 1; i < n; i++) {
            if (i <= r) {
                z[i] = Math.min(r - i + 1, z[i - l]);
            }

            while (i + z[i] < n && str.charAt(z[i]) == str.charAt(i + z[i])) {
                z[i]++;
            }

            if (i + z[i] - 1 > r) {
                l = i;
                r = i + z[i] - 1;
            }
        }

        return z;
    }

    public static void main(String[] args) {
        String text = "AABAACAADAABAABA";
        String pattern = "AABA";

        zAlgorithmSearch(text, pattern);
    }
}

