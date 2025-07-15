import java.util.*;

public class Test {
    // Activity Selection Problem
    static class Activity {
        int start, finish;

        Activity(int start, int finish) {
            this.start = start;
            this.finish = finish;
        }
    }

    public static void activitySelection(Activity[] activities) {
        Arrays.sort(activities, (a, b) -> Integer.compare(a.finish, b.finish));

        System.out.println("Selected activities:");
        int lastFinish = 0;
        int count = 0;

        for (int i = 0; i < activities.length; i++) {
            if (activities[i].start >= lastFinish) {
                System.out.println("Activity: " + activities[i].start + " - " + activities[i].finish);
                lastFinish = activities[i].finish;
                count++;
            }
        }
        System.out.println("Total activities selected: " + count);
    }

    // Fractional Knapsack
    static class Item {
        int value, weight;
        double ratio;

        Item(int value, int weight) {
            this.value = value;
            this.weight = weight;
            this.ratio = (double) value / weight;
        }
    }

    public static double fractionalKnapsack(Item[] items, int capacity) {
        Arrays.sort(items, (a, b) -> Double.compare(b.ratio, a.ratio));

        double totalValue = 0;
        int currentWeight = 0;

        for (Item item : items) {
            if (currentWeight + item.weight <= capacity) {
                currentWeight += item.weight;
                totalValue += item.value;
                System.out.println("Take full item: value=" + item.value + ", weight=" + item.weight);
            } else {
                int remainingCapacity = capacity - currentWeight;
                totalValue += item.value * ((double) remainingCapacity / item.weight);
                System.out.println("Take partial item: value=" + (item.value * ((double) remainingCapacity / item.weight)) + ", weight=" + remainingCapacity);
                break;
            }
        }

        return totalValue;
    }

    public static void main(String[] args) {
        // Activity Selection
        Activity[] activities = {
            new Activity(1, 4),
            new Activity(3, 5),
            new Activity(0, 6),
            new Activity(5, 7),
            new Activity(3, 9),
            new Activity(5, 9),
            new Activity(6, 10),
            new Activity(8, 11),
            new Activity(8, 12),
            new Activity(2, 14),
            new Activity(12, 16)
        };

        activitySelection(activities);
        System.out.println();

        // Fractional Knapsack
        Item[] items = {
            new Item(60, 10),
            new Item(100, 20),
            new Item(120, 30)
        };
        int capacity = 50;

        double maxValue = fractionalKnapsack(items, capacity);
        System.out.println("Maximum value in fractional knapsack: " + maxValue);
    }
}
