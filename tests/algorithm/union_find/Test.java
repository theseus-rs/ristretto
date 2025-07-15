public class Test {
    static class UnionFind {
        private int[] parent;
        private int[] rank;
        private int components;

        public UnionFind(int n) {
            parent = new int[n];
            rank = new int[n];
            components = n;

            for (int i = 0; i < n; i++) {
                parent[i] = i;
                rank[i] = 0;
            }
        }

        public int find(int x) {
            if (parent[x] != x) {
                parent[x] = find(parent[x]); // Path compression
            }
            return parent[x];
        }

        public boolean union(int x, int y) {
            int rootX = find(x);
            int rootY = find(y);

            if (rootX == rootY) {
                return false; // Already in same set
            }

            // Union by rank
            if (rank[rootX] < rank[rootY]) {
                parent[rootX] = rootY;
            } else if (rank[rootX] > rank[rootY]) {
                parent[rootY] = rootX;
            } else {
                parent[rootY] = rootX;
                rank[rootX]++;
            }

            components--;
            return true;
        }

        public boolean connected(int x, int y) {
            return find(x) == find(y);
        }

        public int getComponents() {
            return components;
        }
    }

    public static void main(String[] args) {
        UnionFind uf = new UnionFind(10);

        System.out.println("Initial components: " + uf.getComponents());

        uf.union(1, 2);
        uf.union(2, 5);
        uf.union(5, 6);
        uf.union(6, 7);
        uf.union(3, 8);
        uf.union(8, 9);

        System.out.println("After unions, components: " + uf.getComponents());
        System.out.println("Are 1 and 5 connected? " + uf.connected(1, 5));
        System.out.println("Are 5 and 7 connected? " + uf.connected(5, 7));
        System.out.println("Are 4 and 9 connected? " + uf.connected(4, 9));
        System.out.println("Are 9 and 8 connected? " + uf.connected(9, 8));
    }
}

