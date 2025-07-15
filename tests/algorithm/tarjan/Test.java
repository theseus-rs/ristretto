import java.util.*;

public class Test {
    private int vertices;
    private List<Integer>[] adjacencyList;
    private int time;

    public Test(int vertices) {
        this.vertices = vertices;
        adjacencyList = new List[vertices];
        for (int i = 0; i < vertices; i++) {
            adjacencyList[i] = new ArrayList<>();
        }
        this.time = 0;
    }

    public void addEdge(int u, int v) {
        adjacencyList[u].add(v);
    }

    public void tarjanSCC() {
        int[] disc = new int[vertices];
        int[] low = new int[vertices];
        boolean[] onStack = new boolean[vertices];
        Stack<Integer> stack = new Stack<>();

        Arrays.fill(disc, -1);
        Arrays.fill(low, -1);

        for (int i = 0; i < vertices; i++) {
            if (disc[i] == -1) {
                tarjanSCCUtil(i, disc, low, onStack, stack);
            }
        }
    }

    private void tarjanSCCUtil(int u, int[] disc, int[] low, boolean[] onStack, Stack<Integer> stack) {
        disc[u] = low[u] = ++time;
        stack.push(u);
        onStack[u] = true;

        for (int v : adjacencyList[u]) {
            if (disc[v] == -1) {
                tarjanSCCUtil(v, disc, low, onStack, stack);
                low[u] = Math.min(low[u], low[v]);
            } else if (onStack[v]) {
                low[u] = Math.min(low[u], disc[v]);
            }
        }

        // If u is a root node, pop the stack and print an SCC
        if (low[u] == disc[u]) {
            System.out.print("SCC: ");
            int w;
            do {
                w = stack.pop();
                onStack[w] = false;
                System.out.print(w + " ");
            } while (w != u);
            System.out.println();
        }
    }

    public static void main(String[] args) {
        Test graph = new Test(5);
        graph.addEdge(1, 0);
        graph.addEdge(0, 2);
        graph.addEdge(2, 1);
        graph.addEdge(0, 3);
        graph.addEdge(3, 4);

        System.out.println("Strongly Connected Components (Tarjan's Algorithm):");
        graph.tarjanSCC();
    }
}
