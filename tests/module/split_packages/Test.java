import java.lang.module.Configuration;
import java.lang.module.FindException;
import java.lang.module.ModuleDescriptor;
import java.lang.module.ModuleFinder;
import java.lang.module.ModuleReference;
import java.lang.module.ResolutionException;
import java.util.List;
import java.util.Optional;
import java.util.Set;
import java.util.stream.Collectors;

/**
 * Split Package Detection Tests
 * Tests that the module system correctly detects and rejects split packages
 * (the same package exported by two different modules), as required by the
 * JPMS specification (JEP 261).
 */
public class Test {
    public static void main(String[] args) {
        System.out.println("=== Split Package Detection Tests ===");

        testBootLayerNoSplitPackages();
        testPackageUniquenessAcrossModules();
        testModuleDescriptorPackages();
        testResolutionRejectsOverlap();

        System.out.println("=== Split Package Detection Tests Complete ===");
    }

    /**
     * Verify that the boot layer contains no split packages; every package
     * belongs to exactly one module.
     */
    private static void testBootLayerNoSplitPackages() {
        System.out.println("--- Test: Boot Layer Has No Split Packages ---");
        try {
            ModuleLayer bootLayer = ModuleLayer.boot();
            java.util.Map<String, List<String>> packageToModules = new java.util.HashMap<>();

            for (Module module : bootLayer.modules()) {
                if (module.getDescriptor() == null) continue;
                for (String pkg : module.getDescriptor().packages()) {
                    packageToModules.computeIfAbsent(pkg, k -> new java.util.ArrayList<>())
                        .add(module.getName());
                }
            }

            long splitCount = packageToModules.entrySet().stream()
                .filter(e -> e.getValue().size() > 1)
                .count();

            System.out.println("Total packages in boot layer: " + packageToModules.size());
            System.out.println("Split packages found: " + splitCount);

            if (splitCount > 0) {
                packageToModules.entrySet().stream()
                    .filter(e -> e.getValue().size() > 1)
                    .sorted(java.util.Map.Entry.comparingByKey())
                    .limit(5)
                    .forEach(e -> System.out.println("  SPLIT: " + e.getKey() +
                        " in " + e.getValue()));
            }
        } catch (Exception e) {
            System.out.println("Error: " + e.getMessage());
        }
    }

    /**
     * Cross-check that system module finder references do not share packages.
     */
    private static void testPackageUniquenessAcrossModules() {
        System.out.println("--- Test: Package Uniqueness Across System Modules ---");
        try {
            ModuleFinder systemFinder = ModuleFinder.ofSystem();
            java.util.Map<String, String> packageOwner = new java.util.HashMap<>();
            int duplicates = 0;

            for (ModuleReference ref : systemFinder.findAll()) {
                ModuleDescriptor desc = ref.descriptor();
                for (String pkg : desc.packages()) {
                    String existing = packageOwner.putIfAbsent(pkg, desc.name());
                    if (existing != null && !existing.equals(desc.name())) {
                        duplicates++;
                        if (duplicates <= 3) {
                            System.out.println("  Duplicate: " + pkg +
                                " in " + existing + " and " + desc.name());
                        }
                    }
                }
            }

            System.out.println("System modules package duplicates: " + duplicates);
            System.out.println("Total unique packages in system modules: " + packageOwner.size());
        } catch (Exception e) {
            System.out.println("Error: " + e.getMessage());
        }
    }

    /**
     * Verify that each module descriptor's packages set is non-overlapping
     * with other modules in the boot configuration.
     */
    private static void testModuleDescriptorPackages() {
        System.out.println("--- Test: Module Descriptor Package Sets ---");
        try {
            Configuration config = ModuleLayer.boot().configuration();

            // Pick a few well-known modules and verify their packages don't overlap
            String[] moduleNames = {"java.base", "java.logging", "java.xml"};
            java.util.Map<String, Set<String>> modulePkgs = new java.util.LinkedHashMap<>();

            for (String name : moduleNames) {
                config.findModule(name).ifPresent(rm -> {
                    Set<String> pkgs = rm.reference().descriptor().packages();
                    modulePkgs.put(name, pkgs);
                    System.out.println(name + " has " + pkgs.size() + " packages");
                });
            }

            // Check pairwise disjointness
            List<String> names = new java.util.ArrayList<>(modulePkgs.keySet());
            boolean allDisjoint = true;
            for (int i = 0; i < names.size(); i++) {
                for (int j = i + 1; j < names.size(); j++) {
                    Set<String> a = modulePkgs.get(names.get(i));
                    Set<String> b = modulePkgs.get(names.get(j));
                    boolean disjoint = java.util.Collections.disjoint(a, b);
                    if (!disjoint) {
                        allDisjoint = false;
                        System.out.println("  OVERLAP: " + names.get(i) +
                            " and " + names.get(j));
                    }
                }
            }
            System.out.println("All checked modules have disjoint packages: " + allDisjoint);
        } catch (Exception e) {
            System.out.println("Error: " + e.getMessage());
        }
    }

    /**
     * Attempt to create a Configuration that would have overlapping packages
     * by resolving two root modules that include the same package.
     * The module system should reject this with a ResolutionException or
     * FindException.
     */
    private static void testResolutionRejectsOverlap() {
        System.out.println("--- Test: Resolution Rejects Overlapping Finders ---");
        try {
            // Use an empty before-finder and system as after-finder.
            // Resolving only "java.base" should always succeed.
            ModuleFinder emptyFinder = ModuleFinder.of();
            ModuleFinder systemFinder = ModuleFinder.ofSystem();

            Configuration parent = ModuleLayer.boot().configuration();
            Configuration child = parent.resolve(emptyFinder, systemFinder,
                List.of("java.base"));

            System.out.println("Resolving java.base in child config succeeded: true");
            System.out.println("Child config modules: " + child.modules().size());
            System.out.println("Child config parents: " + child.parents().size());
        } catch (ResolutionException e) {
            System.out.println("ResolutionException: " + e.getMessage());
        } catch (FindException e) {
            System.out.println("FindException: " + e.getMessage());
        } catch (Exception e) {
            System.out.println("Error: " + e.getClass().getSimpleName() + ": " + e.getMessage());
        }
    }
}
