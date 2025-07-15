public class Test {
    public static void main(String[] args) {
        int[] arr = {64, 34, 25, 12, 22, 11, 90};
        int target = 22;

        int result = linearSearch(arr, target);
        System.out.println("Linear Search Result: " + result);

        // Test with element not in array
        int notFound = linearSearch(arr, 100);
        System.out.println("Element not found: " + notFound);
    }

    public static int linearSearch(int[] arr, int target) {
        for (int i = 0; i < arr.length; i++) {
            if (arr[i] == target) {
                return i;
            }
        }
        return -1; // Element not found
    }
}

