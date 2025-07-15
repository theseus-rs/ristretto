public class Test {
    public static double[] pageRank(double[][] adjacencyMatrix, double dampingFactor, int maxIterations, double tolerance) {
        int n = adjacencyMatrix.length;
        double[] pageRank = new double[n];
        double[] newPageRank = new double[n];

        // Initialize PageRank values
        for (int i = 0; i < n; i++) {
            pageRank[i] = 1.0 / n;
        }

        // Calculate out-degree for each node
        int[] outDegree = new int[n];
        for (int i = 0; i < n; i++) {
            for (int j = 0; j < n; j++) {
                if (adjacencyMatrix[i][j] > 0) {
                    outDegree[i]++;
                }
            }
        }

        // Iterative calculation
        for (int iter = 0; iter < maxIterations; iter++) {
            for (int i = 0; i < n; i++) {
                newPageRank[i] = (1 - dampingFactor) / n;

                for (int j = 0; j < n; j++) {
                    if (adjacencyMatrix[j][i] > 0 && outDegree[j] > 0) {
                        newPageRank[i] += dampingFactor * pageRank[j] / outDegree[j];
                    }
                }
            }

            // Check for convergence
            double diff = 0;
            for (int i = 0; i < n; i++) {
                diff += Math.abs(newPageRank[i] - pageRank[i]);
            }

            if (diff < tolerance) {
                System.out.println("Converged after " + (iter + 1) + " iterations");
                break;
            }

            // Copy new values
            System.arraycopy(newPageRank, 0, pageRank, 0, n);
        }

        return newPageRank;
    }

    public static void main(String[] args) {
        // Example graph adjacency matrix
        double[][] graph = {
            {0, 1, 1, 0},
            {1, 0, 1, 1},
            {1, 0, 0, 1},
            {0, 1, 1, 0}
        };

        double dampingFactor = 0.85;
        int maxIterations = 100;
        double tolerance = 1e-6;

        System.out.println("PageRank Algorithm");
        System.out.println("Damping factor: " + dampingFactor);

        double[] pageRankValues = pageRank(graph, dampingFactor, maxIterations, tolerance);

        System.out.println("\nPageRank values:");
        for (int i = 0; i < pageRankValues.length; i++) {
            System.out.printf("Node %d: %.6f%n", i, pageRankValues[i]);
        }

        // Find the node with highest PageRank
        int maxIndex = 0;
        for (int i = 1; i < pageRankValues.length; i++) {
            if (pageRankValues[i] > pageRankValues[maxIndex]) {
                maxIndex = i;
            }
        }

        System.out.println("\nHighest PageRank: Node " + maxIndex + " with value " + pageRankValues[maxIndex]);
    }
}

