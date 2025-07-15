public class Test {
    public static int editDistance(String str1, String str2) {
        int m = str1.length();
        int n = str2.length();
        int[][] dp = new int[m + 1][n + 1];

        // Initialize base cases
        for (int i = 0; i <= m; i++) {
            dp[i][0] = i;
        }
        for (int j = 0; j <= n; j++) {
            dp[0][j] = j;
        }

        // Fill the DP table
        for (int i = 1; i <= m; i++) {
            for (int j = 1; j <= n; j++) {
                if (str1.charAt(i - 1) == str2.charAt(j - 1)) {
                    dp[i][j] = dp[i - 1][j - 1];
                } else {
                    dp[i][j] = 1 + Math.min(Math.min(
                        dp[i - 1][j],    // deletion
                        dp[i][j - 1]),   // insertion
                        dp[i - 1][j - 1] // substitution
                    );
                }
            }
        }

        return dp[m][n];
    }

    public static void printOperations(String str1, String str2) {
        int m = str1.length();
        int n = str2.length();
        int[][] dp = new int[m + 1][n + 1];

        for (int i = 0; i <= m; i++) {
            dp[i][0] = i;
        }
        for (int j = 0; j <= n; j++) {
            dp[0][j] = j;
        }

        for (int i = 1; i <= m; i++) {
            for (int j = 1; j <= n; j++) {
                if (str1.charAt(i - 1) == str2.charAt(j - 1)) {
                    dp[i][j] = dp[i - 1][j - 1];
                } else {
                    dp[i][j] = 1 + Math.min(Math.min(dp[i - 1][j], dp[i][j - 1]), dp[i - 1][j - 1]);
                }
            }
        }

        System.out.println("Edit distance between '" + str1 + "' and '" + str2 + "': " + dp[m][n]);
    }

    public static void main(String[] args) {
        String str1 = "kitten";
        String str2 = "sitting";

        int distance = editDistance(str1, str2);
        System.out.println("String 1: " + str1);
        System.out.println("String 2: " + str2);
        System.out.println("Edit Distance: " + distance);

        printOperations(str1, str2);
    }
}

