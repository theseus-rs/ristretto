import java.util.*;

public class Test {
    static class Edge {
        int source, destination, weight;

        Edge(int source, int destination, int weight) {
            this.source = source;
            this.destination = destination;
            this.weight = weight;
        }
    }

    public static void bellmanFord(List<Edge> edges, int V, int src) {
        int[] dist = new int[V];
        Arrays.fill(dist, Integer.MAX_VALUE);
        dist[src] = 0;

        // Relax all edges V-1 times
        for (int i = 0; i < V - 1; i++) {
            for (Edge edge : edges) {
                if (dist[edge.source] != Integer.MAX_VALUE &&
                    dist[edge.source] + edge.weight < dist[edge.destination]) {
                    dist[edge.destination] = dist[edge.source] + edge.weight;
                }
            }
        }

        // Check for negative cycles
        for (Edge edge : edges) {
            if (dist[edge.source] != Integer.MAX_VALUE &&
                dist[edge.source] + edge.weight < dist[edge.destination]) {
                System.out.println("Graph contains negative weight cycle");
                return;
            }
        }

        printSolution(dist, src);
    }

    public static void printSolution(int[] dist, int src) {
        System.out.println("Vertex Distance from Source " + src);
        for (int i = 0; i < dist.length; i++) {
            System.out.println(i + " \t\t " + dist[i]);
        }
    }

    public static void main(String[] args) {
        int V = 5;
        List<Edge> edges = new ArrayList<>();

        edges.add(new Edge(0, 1, -1));
        edges.add(new Edge(0, 2, 4));
        edges.add(new Edge(1, 2, 3));
        edges.add(new Edge(1, 3, 2));
        edges.add(new Edge(1, 4, 2));
        edges.add(new Edge(3, 2, 5));
        edges.add(new Edge(3, 1, 1));
        edges.add(new Edge(4, 3, -3));

        bellmanFord(edges, V, 0);
    }
}
