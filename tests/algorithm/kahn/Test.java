import java.util.*;

public class Test {
    private int vertices;
    private List<Integer>[] adjacencyList;

    public Test(int vertices) {
        this.vertices = vertices;
        adjacencyList = new List[vertices];
        for (int i = 0; i < vertices; i++) {
            adjacencyList[i] = new ArrayList<>();
        }
    }

    public void addEdge(int u, int v) {
        adjacencyList[u].add(v);
    }

    public void kahnTopologicalSort() {
        int[] inDegree = new int[vertices];

        // Calculate in-degrees of all vertices
        for (int u = 0; u < vertices; u++) {
            for (int v : adjacencyList[u]) {
                inDegree[v]++;
            }
        }

        Queue<Integer> queue = new LinkedList<>();

        // Enqueue all vertices with in-degree 0
        for (int i = 0; i < vertices; i++) {
            if (inDegree[i] == 0) {
                queue.offer(i);
            }
        }

        List<Integer> topologicalOrder = new ArrayList<>();

        while (!queue.isEmpty()) {
            int u = queue.poll();
            topologicalOrder.add(u);

            // For each neighbor of the dequeued vertex
            for (int v : adjacencyList[u]) {
                inDegree[v]--;
                if (inDegree[v] == 0) {
                    queue.offer(v);
                }
            }
        }

        // Check if there was a cycle
        if (topologicalOrder.size() != vertices) {
            System.out.println("Graph has a cycle! Topological sort not possible.");
        } else {
            System.out.print("Kahn's Topological Sort: ");
            for (int vertex : topologicalOrder) {
                System.out.print(vertex + " ");
            }
            System.out.println();
        }
    }

    public static void main(String[] args) {
        Test graph = new Test(6);
        graph.addEdge(5, 2);
        graph.addEdge(5, 0);
        graph.addEdge(4, 0);
        graph.addEdge(4, 1);
        graph.addEdge(2, 3);
        graph.addEdge(3, 1);

        graph.kahnTopologicalSort();
    }
}
