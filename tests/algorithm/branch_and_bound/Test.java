import java.util.*;

public class Test {
    static class Node {
        int level, profit, weight;
        double bound;
        boolean[] taken;

        Node(int level, int profit, int weight, boolean[] taken) {
            this.level = level;
            this.profit = profit;
            this.weight = weight;
            this.taken = taken.clone();
        }
    }

    static class Item {
        int weight, value;
        double ratio;

        Item(int weight, int value) {
            this.weight = weight;
            this.value = value;
            this.ratio = (double) value / weight;
        }
    }

    public static int knapsackBranchAndBound(Item[] items, int capacity) {
        // Sort items by value-to-weight ratio in descending order
        Arrays.sort(items, (a, b) -> Double.compare(b.ratio, a.ratio));

        Queue<Node> queue = new PriorityQueue<>((a, b) -> Double.compare(b.bound, a.bound));

        Node root = new Node(-1, 0, 0, new boolean[items.length]);
        root.bound = calculateBound(root, items, capacity);
        queue.offer(root);

        int maxProfit = 0;
        boolean[] bestSolution = new boolean[items.length];

        while (!queue.isEmpty()) {
            Node current = queue.poll();

            if (current.bound > maxProfit && current.level < items.length - 1) {
                int nextLevel = current.level + 1;

                // Include the next item
                if (current.weight + items[nextLevel].weight <= capacity) {
                    Node include = new Node(nextLevel,
                        current.profit + items[nextLevel].value,
                        current.weight + items[nextLevel].weight,
                        current.taken);
                    include.taken[nextLevel] = true;
                    include.bound = calculateBound(include, items, capacity);

                    if (include.profit > maxProfit) {
                        maxProfit = include.profit;
                        bestSolution = include.taken.clone();
                    }

                    if (include.bound > maxProfit) {
                        queue.offer(include);
                    }
                }

                // Exclude the next item
                Node exclude = new Node(nextLevel, current.profit, current.weight, current.taken);
                exclude.bound = calculateBound(exclude, items, capacity);

                if (exclude.bound > maxProfit) {
                    queue.offer(exclude);
                }
            }
        }

        System.out.println("Branch and Bound Knapsack Solution:");
        System.out.println("Maximum profit: " + maxProfit);
        System.out.print("Items included: ");
        for (int i = 0; i < bestSolution.length; i++) {
            if (bestSolution[i]) {
                System.out.print(i + " ");
            }
        }
        System.out.println();

        return maxProfit;
    }

    private static double calculateBound(Node node, Item[] items, int capacity) {
        if (node.weight >= capacity) {
            return 0;
        }

        double bound = node.profit;
        int weight = node.weight;
        int level = node.level + 1;

        while (level < items.length && weight + items[level].weight <= capacity) {
            weight += items[level].weight;
            bound += items[level].value;
            level++;
        }

        if (level < items.length) {
            bound += (capacity - weight) * items[level].ratio;
        }

        return bound;
    }

    public static void main(String[] args) {
        Item[] items = {
            new Item(10, 60),
            new Item(20, 100),
            new Item(30, 120)
        };
        int capacity = 50;

        int maxProfit = knapsackBranchAndBound(items, capacity);
        System.out.println("Final result: " + maxProfit);
    }
}
