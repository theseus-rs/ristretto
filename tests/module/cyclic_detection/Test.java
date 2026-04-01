import java.lang.module.Configuration;
import java.lang.module.ModuleDescriptor;
import java.lang.module.ModuleFinder;
import java.lang.module.ModuleReference;
import java.lang.module.ResolutionException;
import java.util.List;
import java.util.Optional;
import java.util.Set;

/**
 * Cyclic Dependency Detection Tests
 * Tests that the module system correctly handles or rejects cyclic module
 * dependencies. Per the JPMS spec, direct cycles in requires are not allowed
 * (except for the implicit java.base requirement), and the resolver must
 * detect them.
 */
public class Test {
    public static void main(String[] args) {
        System.out.println("=== Cyclic Dependency Detection Tests ===");

        testBootLayerNoCycles();
        testReadEdgesAcyclic();
        testSelfReadability();
        testJavaBaseImplicitRequirement();
        testResolvedModuleReads();

        System.out.println("=== Cyclic Dependency Detection Tests Complete ===");
    }

    /**
     * Verify that the boot layer configuration has no cyclic requires chains.
     * Walk the read graph via BFS and confirm no back-edges.
     */
    private static void testBootLayerNoCycles() {
        System.out.println("--- Test: Boot Layer No Requires Cycles ---");
        try {
            Configuration config = ModuleLayer.boot().configuration();
            java.util.Map<String, Set<String>> requiresGraph = new java.util.HashMap<>();

            for (var rm : config.modules()) {
                Set<String> deps = new java.util.HashSet<>();
                for (var req : rm.reference().descriptor().requires()) {
                    deps.add(req.name());
                }
                requiresGraph.put(rm.name(), deps);
            }

            // Detect cycles using DFS with coloring
            // WHITE=unvisited, GRAY=in-progress, BLACK=done
            java.util.Map<String, Integer> color = new java.util.HashMap<>();
            final int WHITE = 0, GRAY = 1, BLACK = 2;
            for (String mod : requiresGraph.keySet()) {
                color.put(mod, WHITE);
            }

            boolean hasCycle = false;
            for (String mod : requiresGraph.keySet()) {
                if (color.get(mod) == WHITE) {
                    if (dfsCycleCheck(mod, requiresGraph, color, WHITE, GRAY, BLACK)) {
                        hasCycle = true;
                        break;
                    }
                }
            }

            System.out.println("Modules in requires graph: " + requiresGraph.size());
            System.out.println("Cycle detected in requires graph: " + hasCycle);
        } catch (Exception e) {
            System.out.println("Error: " + e.getMessage());
        }
    }

    private static boolean dfsCycleCheck(String node,
            java.util.Map<String, Set<String>> graph,
            java.util.Map<String, Integer> color,
            int WHITE, int GRAY, int BLACK) {
        color.put(node, GRAY);
        Set<String> neighbors = graph.getOrDefault(node, Set.of());
        for (String neighbor : neighbors) {
            Integer neighborColor = color.get(neighbor);
            if (neighborColor == null) continue; // external module not in graph
            if (neighborColor == GRAY) return true; // back edge = cycle
            if (neighborColor == WHITE && dfsCycleCheck(neighbor, graph, color, WHITE, GRAY, BLACK)) {
                return true;
            }
        }
        color.put(node, BLACK);
        return false;
    }

    /**
     * Verify that read edges (which include transitive reads) do not form
     * problematic cycles. Note: read edges CAN be bidirectional (module A
     * reads B and B reads A via transitive), but the requires graph should not cycle.
     */
    private static void testReadEdgesAcyclic() {
        System.out.println("--- Test: Read Edges Analysis ---");
        try {
            Configuration config = ModuleLayer.boot().configuration();

            int totalReadEdges = 0;
            int bidirectionalPairs = 0;

            java.util.Map<String, Set<String>> readGraph = new java.util.HashMap<>();
            for (var rm : config.modules()) {
                Set<String> reads = new java.util.HashSet<>();
                for (var readModule : rm.reads()) {
                    reads.add(readModule.name());
                    totalReadEdges++;
                }
                readGraph.put(rm.name(), reads);
            }

            // Count bidirectional read pairs
            for (var entry : readGraph.entrySet()) {
                String from = entry.getKey();
                for (String to : entry.getValue()) {
                    if (!from.equals(to) && readGraph.containsKey(to) &&
                        readGraph.get(to).contains(from)) {
                        bidirectionalPairs++;
                    }
                }
            }
            // Each pair counted twice
            bidirectionalPairs /= 2;

            System.out.println("Total read edges: " + totalReadEdges);
            System.out.println("Bidirectional read pairs: " + bidirectionalPairs);

            // java.base should be read by many modules
            long readJavaBase = readGraph.values().stream()
                .filter(reads -> reads.contains("java.base"))
                .count();
            System.out.println("Modules that read java.base: " + readJavaBase);
        } catch (Exception e) {
            System.out.println("Error: " + e.getMessage());
        }
    }

    /**
     * Test that every module can read itself (self-readability is implicit).
     */
    private static void testSelfReadability() {
        System.out.println("--- Test: Self Readability ---");
        try {
            ModuleLayer bootLayer = ModuleLayer.boot();
            int selfReadable = 0;
            int notSelfReadable = 0;

            for (Module mod : bootLayer.modules()) {
                if (mod.canRead(mod)) {
                    selfReadable++;
                } else {
                    notSelfReadable++;
                    System.out.println("  NOT self-readable: " + mod.getName());
                }
            }

            System.out.println("Self-readable modules: " + selfReadable);
            System.out.println("Not self-readable: " + notSelfReadable);
            System.out.println("All modules are self-readable: " + (notSelfReadable == 0));
        } catch (Exception e) {
            System.out.println("Error: " + e.getMessage());
        }
    }

    /**
     * Verify that java.base has the MANDATED requires modifier, confirming
     * the implicit java.base dependency is correctly modeled.
     */
    private static void testJavaBaseImplicitRequirement() {
        System.out.println("--- Test: java.base Implicit Requirement ---");
        try {
            ModuleLayer bootLayer = ModuleLayer.boot();

            // Check a few modules for their java.base requires
            String[] moduleNames = {"java.logging", "java.xml", "java.sql"};
            for (String name : moduleNames) {
                Module mod = bootLayer.findModule(name).orElse(null);
                if (mod == null || mod.getDescriptor() == null) continue;

                Optional<ModuleDescriptor.Requires> javaBaseReq =
                    mod.getDescriptor().requires().stream()
                        .filter(r -> r.name().equals("java.base"))
                        .findFirst();

                if (javaBaseReq.isPresent()) {
                    ModuleDescriptor.Requires req = javaBaseReq.get();
                    boolean isMandated = req.modifiers().contains(
                        ModuleDescriptor.Requires.Modifier.MANDATED);
                    System.out.println(name + " requires java.base: mandated=" + isMandated);
                } else {
                    System.out.println(name + " does not require java.base (unexpected)");
                }
            }

            // java.base itself should NOT require java.base
            Module javaBase = bootLayer.findModule("java.base").orElse(null);
            if (javaBase != null) {
                boolean requiresSelf = javaBase.getDescriptor().requires().stream()
                    .anyMatch(r -> r.name().equals("java.base"));
                System.out.println("java.base requires itself: " + requiresSelf);
            }
        } catch (Exception e) {
            System.out.println("Error: " + e.getMessage());
        }
    }

    /**
     * Verify resolved module read sets are consistent with transitive
     * dependency resolution.
     */
    private static void testResolvedModuleReads() {
        System.out.println("--- Test: Resolved Module Reads ---");
        try {
            Configuration config = ModuleLayer.boot().configuration();

            // Every resolved module should read java.base (except java.base itself)
            int readJavaBase = 0;
            int dontReadJavaBase = 0;
            for (var rm : config.modules()) {
                boolean readsBase = rm.reads().stream()
                    .anyMatch(r -> r.name().equals("java.base"));
                if (rm.name().equals("java.base")) {
                    // java.base reads itself
                    continue;
                }
                if (readsBase) {
                    readJavaBase++;
                } else {
                    dontReadJavaBase++;
                    System.out.println("  Does not read java.base: " + rm.name());
                }
            }

            System.out.println("Non-base modules reading java.base: " + readJavaBase);
            System.out.println("Non-base modules NOT reading java.base: " + dontReadJavaBase);
            System.out.println("All non-base modules read java.base: " + (dontReadJavaBase == 0));
        } catch (Exception e) {
            System.out.println("Error: " + e.getMessage());
        }
    }
}
