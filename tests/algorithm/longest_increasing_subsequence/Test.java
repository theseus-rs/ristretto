public class Test {
    public static int longestIncreasingSubsequence(int[] nums) {
        if (nums.length == 0) return 0;

        int[] dp = new int[nums.length];
        int length = 0;

        for (int num : nums) {
            int left = 0, right = length;

            // Binary search
            while (left < right) {
                int mid = left + (right - left) / 2;
                if (dp[mid] < num) {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            }

            dp[left] = num;
            if (left == length) {
                length++;
            }
        }

        return length;
    }

    public static int[] getLIS(int[] nums) {
        if (nums.length == 0) return new int[0];

        int[] dp = new int[nums.length];
        int[] parent = new int[nums.length];
        int[] indices = new int[nums.length];
        int length = 0;

        for (int i = 0; i < nums.length; i++) {
            int left = 0, right = length;

            while (left < right) {
                int mid = left + (right - left) / 2;
                if (dp[mid] < nums[i]) {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            }

            dp[left] = nums[i];
            indices[left] = i;
            parent[i] = left > 0 ? indices[left - 1] : -1;

            if (left == length) {
                length++;
            }
        }

        // Reconstruct LIS
        int[] result = new int[length];
        int k = indices[length - 1];
        for (int i = length - 1; i >= 0; i--) {
            result[i] = nums[k];
            k = parent[k];
        }

        return result;
    }

    public static void main(String[] args) {
        int[] nums = {10, 9, 2, 5, 3, 7, 101, 18};

        int lisLength = longestIncreasingSubsequence(nums);
        int[] lis = getLIS(nums);

        System.out.print("Array: ");
        for (int num : nums) {
            System.out.print(num + " ");
        }
        System.out.println();

        System.out.println("LIS Length: " + lisLength);
        System.out.print("LIS: ");
        for (int num : lis) {
            System.out.print(num + " ");
        }
        System.out.println();
    }
}

