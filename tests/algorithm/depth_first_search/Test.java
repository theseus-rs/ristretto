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

    public void addEdge(int source, int destination) {
        adjacencyList[source].add(destination);
    }

    public void dfs(int startVertex) {
        boolean[] visited = new boolean[vertices];
        System.out.print("DFS traversal starting from vertex " + startVertex + ": ");
        dfsUtil(startVertex, visited);
        System.out.println();
    }

    private void dfsUtil(int vertex, boolean[] visited) {
        visited[vertex] = true;
        System.out.print(vertex + " ");

        for (int neighbor : adjacencyList[vertex]) {
            if (!visited[neighbor]) {
                dfsUtil(neighbor, visited);
            }
        }
    }

    public static void main(String[] args) {
        Test graph = new Test(6);

        graph.addEdge(0, 1);
        graph.addEdge(0, 2);
        graph.addEdge(1, 3);
        graph.addEdge(1, 4);
        graph.addEdge(2, 5);

        graph.dfs(0);
    }
}
