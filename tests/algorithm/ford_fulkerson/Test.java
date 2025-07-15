import java.util.*;

public class Test {
    private static final int INF = Integer.MAX_VALUE;

    public static int fordFulkerson(int[][] graph, int source, int sink) {
        int[][] residualGraph = new int[graph.length][graph[0].length];

        // Create residual graph
        for (int u = 0; u < graph.length; u++) {
            for (int v = 0; v < graph[0].length; v++) {
                residualGraph[u][v] = graph[u][v];
            }
        }

        int[] parent = new int[graph.length];
        int maxFlow = 0;

        // Augment the flow while there exists a path from source to sink
        while (bfs(residualGraph, source, sink, parent)) {
            // Find minimum residual capacity of the edges along the path
            int pathFlow = INF;
            for (int v = sink; v != source; v = parent[v]) {
                int u = parent[v];
                pathFlow = Math.min(pathFlow, residualGraph[u][v]);
            }

            // Add path flow to overall flow
            maxFlow += pathFlow;

            // Update residual capacities of the edges and reverse edges
            for (int v = sink; v != source; v = parent[v]) {
                int u = parent[v];
                residualGraph[u][v] -= pathFlow;
                residualGraph[v][u] += pathFlow;
            }
        }

        return maxFlow;
    }

    private static boolean bfs(int[][] residualGraph, int source, int sink, int[] parent) {
        boolean[] visited = new boolean[residualGraph.length];
        Queue<Integer> queue = new LinkedList<>();
        queue.offer(source);
        visited[source] = true;
        parent[source] = -1;

        while (!queue.isEmpty()) {
            int u = queue.poll();

            for (int v = 0; v < residualGraph.length; v++) {
                if (!visited[v] && residualGraph[u][v] > 0) {
                    queue.offer(v);
                    parent[v] = u;
                    visited[v] = true;
                    if (v == sink) {
                        return true;
                    }
                }
            }
        }

        return false;
    }

    public static void main(String[] args) {
        int[][] graph = {
            {0, 16, 13, 0, 0, 0},
            {0, 0, 10, 12, 0, 0},
            {0, 4, 0, 0, 14, 0},
            {0, 0, 9, 0, 0, 20},
            {0, 0, 0, 7, 0, 4},
            {0, 0, 0, 0, 0, 0}
        };

        int maxFlow = fordFulkerson(graph, 0, 5);
        System.out.println("Maximum flow from source to sink: " + maxFlow);
    }
}
