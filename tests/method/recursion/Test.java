/** Test recursive methods and stack behavior. */
public class Test {
    // Simple recursive method - factorial
    public static long factorial(int n) {
        System.out.println("Computing factorial of " + n);
        if (n <= 1) {
            return 1;
        }
        return n * factorial(n - 1);
    }

    // Mutual recursion - even/odd check
    public static boolean isEven(int n) {
        if (n == 0) return true;
        return isOdd(n - 1);
    }

    public static boolean isOdd(int n) {
        if (n == 0) return false;
        return isEven(n - 1);
    }

    // Tail recursion example
    public static long factorialTailRecursive(int n, long accumulator) {
        if (n <= 1) {
            return accumulator;
        }
        return factorialTailRecursive(n - 1, n * accumulator);
    }

    public static long factorialTailRecursive(int n) {
        return factorialTailRecursive(n, 1);
    }

    // Tree traversal recursion
    static class TreeNode {
        int value;
        TreeNode left, right;

        TreeNode(int value) {
            this.value = value;
        }
    }

    public static void inorderTraversal(TreeNode node) {
        if (node != null) {
            inorderTraversal(node.left);
            System.out.print(node.value + " ");
            inorderTraversal(node.right);
        }
    }

    // Deep recursion test (controlled to avoid stack overflow)
    public static int deepRecursion(int depth, int maxDepth) {
        if (depth >= maxDepth) {
            System.out.println("Reached maximum depth: " + depth);
            return depth;
        }
        return deepRecursion(depth + 1, maxDepth);
    }

    public static void main(String[] args) {
        System.out.println("=== Testing factorial recursion ===");
        System.out.println("5! = " + factorial(5));

        System.out.println("\n=== Testing mutual recursion ===");
        for (int i = 0; i <= 6; i++) {
            System.out.println(i + " is " + (isEven(i) ? "even" : "odd"));
        }

        System.out.println("\n=== Testing tail recursion ===");
        System.out.println("5! (tail recursive) = " + factorialTailRecursive(5));

        System.out.println("\n=== Testing tree traversal recursion ===");
        TreeNode root = new TreeNode(4);
        root.left = new TreeNode(2);
        root.right = new TreeNode(6);
        root.left.left = new TreeNode(1);
        root.left.right = new TreeNode(3);
        root.right.left = new TreeNode(5);
        root.right.right = new TreeNode(7);

        System.out.print("Inorder traversal: ");
        inorderTraversal(root);
        System.out.println();

        System.out.println("\n=== Testing controlled deep recursion ===");
        int maxDepth = deepRecursion(0, 1000);
        System.out.println("Maximum depth reached: " + maxDepth);
    }
}

