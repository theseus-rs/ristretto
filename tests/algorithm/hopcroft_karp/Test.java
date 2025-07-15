import java.util.*;

public class Test {
    private static final int NIL = 0;
    private static final int INF = Integer.MAX_VALUE;

    private int n, m;
    private List<Integer>[] adj;
    private int[] pairU, pairV, dist;

    public Test(int n, int m) {
        this.n = n;
        this.m = m;
        adj = new List[n + 1];
        for (int i = 0; i <= n; i++) {
            adj[i] = new ArrayList<>();
        }
        pairU = new int[n + 1];
        pairV = new int[m + 1];
        dist = new int[n + 1];
    }

    public void addEdge(int u, int v) {
        adj[u].add(v);
    }

    public int hopcroftKarp() {
        Arrays.fill(pairU, NIL);
        Arrays.fill(pairV, NIL);

        int matching = 0;

        while (bfs()) {
            for (int u = 1; u <= n; u++) {
                if (pairU[u] == NIL && dfs(u)) {
                    matching++;
                }
            }
        }

        return matching;
    }

    private boolean bfs() {
        Queue<Integer> queue = new LinkedList<>();

        for (int u = 1; u <= n; u++) {
            if (pairU[u] == NIL) {
                dist[u] = 0;
                queue.offer(u);
            } else {
                dist[u] = INF;
            }
        }

        dist[NIL] = INF;

        while (!queue.isEmpty()) {
            int u = queue.poll();

            if (dist[u] < dist[NIL]) {
                for (int v : adj[u]) {
                    if (dist[pairV[v]] == INF) {
                        dist[pairV[v]] = dist[u] + 1;
                        queue.offer(pairV[v]);
                    }
                }
            }
        }

        return dist[NIL] != INF;
    }

    private boolean dfs(int u) {
        if (u != NIL) {
            for (int v : adj[u]) {
                if (dist[pairV[v]] == dist[u] + 1) {
                    if (dfs(pairV[v])) {
                        pairV[v] = u;
                        pairU[u] = v;
                        return true;
                    }
                }
            }
            dist[u] = INF;
            return false;
        }
        return true;
    }

    public void printMatching() {
        System.out.println("Matching pairs:");
        for (int u = 1; u <= n; u++) {
            if (pairU[u] != NIL) {
                System.out.println("U" + u + " - V" + pairU[u]);
            }
        }
    }

    public static void main(String[] args) {
        Test hk = new Test(4, 4);

        hk.addEdge(1, 1);
        hk.addEdge(1, 2);
        hk.addEdge(2, 1);
        hk.addEdge(3, 2);
        hk.addEdge(4, 2);
        hk.addEdge(4, 4);

        int maxMatching = hk.hopcroftKarp();
        System.out.println("Maximum bipartite matching: " + maxMatching);
        hk.printMatching();
    }
}
