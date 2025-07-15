import java.util.*;

public class Test {
    public static int edmondsKarp(int[][] capacity, int source, int sink) {
        int n = capacity.length;
        int[][] flow = new int[n][n];
        int maxFlow = 0;

        while (true) {
            int[] parent = new int[n];
            Arrays.fill(parent, -1);

            // BFS to find augmenting path
            Queue<Integer> queue = new LinkedList<>();
            queue.offer(source);
            parent[source] = source;

            while (!queue.isEmpty() && parent[sink] == -1) {
                int u = queue.poll();

                for (int v = 0; v < n; v++) {
                    if (parent[v] == -1 && capacity[u][v] - flow[u][v] > 0) {
                        parent[v] = u;
                        queue.offer(v);
                    }
                }
            }

            // If no augmenting path found, we're done
            if (parent[sink] == -1) break;

            // Find minimum capacity along the path
            int pathFlow = Integer.MAX_VALUE;
            for (int v = sink; v != source; v = parent[v]) {
                int u = parent[v];
                pathFlow = Math.min(pathFlow, capacity[u][v] - flow[u][v]);
            }

            // Update flow along the path
            for (int v = sink; v != source; v = parent[v]) {
                int u = parent[v];
                flow[u][v] += pathFlow;
                flow[v][u] -= pathFlow;
            }

            maxFlow += pathFlow;
        }

        return maxFlow;
    }

    public static void main(String[] args) {
        int[][] capacity = {
            {0, 3, 3, 0, 0, 0},
            {0, 0, 2, 3, 0, 0},
            {0, 0, 0, 0, 2, 0},
            {0, 0, 0, 0, 4, 2},
            {0, 0, 0, 0, 0, 2},
            {0, 0, 0, 0, 0, 0}
        };

        int maxFlow = edmondsKarp(capacity, 0, 5);
        System.out.println("Maximum flow (Edmonds-Karp): " + maxFlow);
    }
}
