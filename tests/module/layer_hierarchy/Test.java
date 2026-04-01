import java.lang.module.Configuration;
import java.lang.module.ModuleFinder;
import java.lang.module.ResolvedModule;
import java.util.List;
import java.util.Optional;
import java.util.Set;

/**
 * Multi-Parent Layer Hierarchy Tests
 * Tests ModuleLayer hierarchies including multi-parent layers, the empty layer,
 * and layer composition as defined in the JPMS specification.
 */
public class Test {
    public static void main(String[] args) {
        System.out.println("=== Multi-Parent Layer Hierarchy Tests ===");

        testEmptyLayer();
        testBootLayerParents();
        testChildLayerCreation();
        testMultiParentLayer();
        testLayerModuleLookup();
        testLayerFindLoader();

        System.out.println("=== Multi-Parent Layer Hierarchy Tests Complete ===");
    }

    /**
     * Test the empty layer; the root of all layer hierarchies.
     */
    private static void testEmptyLayer() {
        System.out.println("--- Test: Empty Layer ---");
        try {
            ModuleLayer empty = ModuleLayer.empty();
            System.out.println("Empty layer modules: " + empty.modules().size());
            System.out.println("Empty layer parents: " + empty.parents().size());

            // Empty layer's configuration should be the empty configuration
            Configuration emptyConfig = empty.configuration();
            System.out.println("Empty config modules: " + emptyConfig.modules().size());
            System.out.println("Empty config parents: " + emptyConfig.parents().size());

            // The empty layer is the parent of the boot layer
            ModuleLayer bootLayer = ModuleLayer.boot();
            boolean bootHasEmptyParent = bootLayer.parents().contains(empty);
            System.out.println("Boot layer has empty layer as parent: " + bootHasEmptyParent);
        } catch (Exception e) {
            System.out.println("Error: " + e.getMessage());
        }
    }

    /**
     * Test boot layer parent relationships.
     */
    private static void testBootLayerParents() {
        System.out.println("--- Test: Boot Layer Parents ---");
        try {
            ModuleLayer bootLayer = ModuleLayer.boot();
            System.out.println("Boot layer parent count: " + bootLayer.parents().size());

            for (int i = 0; i < bootLayer.parents().size(); i++) {
                ModuleLayer parent = bootLayer.parents().get(i);
                System.out.println("  Parent " + i + " modules: " + parent.modules().size());
                System.out.println("  Parent " + i + " is empty layer: " +
                    (parent == ModuleLayer.empty()));
            }

            // Boot configuration parents
            Configuration bootConfig = bootLayer.configuration();
            System.out.println("Boot config parent count: " + bootConfig.parents().size());
            for (int i = 0; i < bootConfig.parents().size(); i++) {
                Configuration parentConfig = bootConfig.parents().get(i);
                System.out.println("  Config parent " + i + " modules: " +
                    parentConfig.modules().size());
            }
        } catch (Exception e) {
            System.out.println("Error: " + e.getMessage());
        }
    }

    /**
     * Create a child layer from the boot layer with a resolved Configuration.
     */
    private static void testChildLayerCreation() {
        System.out.println("--- Test: Child Layer Creation ---");
        try {
            ModuleLayer bootLayer = ModuleLayer.boot();

            // Create a child configuration that resolves java.logging (already in boot)
            ModuleFinder emptyFinder = ModuleFinder.of();
            ModuleFinder systemFinder = ModuleFinder.ofSystem();

            // Resolve java.sql which may bring in dependencies
            Configuration childConfig = bootLayer.configuration().resolve(
                emptyFinder, systemFinder, List.of("java.sql"));

            System.out.println("Child config modules: " + childConfig.modules().size());
            System.out.println("Child config parents: " + childConfig.parents().size());
            childConfig.modules().stream()
                .sorted(java.util.Comparator.comparing(ResolvedModule::name))
                .forEach(rm -> System.out.println("  Resolved: " + rm.name()));

            // Create a layer from the child configuration
            ModuleLayer childLayer = bootLayer.defineModulesWithOneLoader(
                childConfig, ClassLoader.getSystemClassLoader());

            System.out.println("Child layer modules: " + childLayer.modules().size());
            System.out.println("Child layer parent count: " + childLayer.parents().size());
            System.out.println("Child layer parent is boot: " +
                (childLayer.parents().get(0) == bootLayer));
        } catch (Exception e) {
            System.out.println("Error: " + e.getClass().getSimpleName() + ": " + e.getMessage());
        }
    }

    /**
     * Test creation of a layer with multiple parents.
     */
    private static void testMultiParentLayer() {
        System.out.println("--- Test: Multi-Parent Layer ---");
        try {
            ModuleLayer bootLayer = ModuleLayer.boot();
            ModuleLayer emptyLayer = ModuleLayer.empty();

            // Create Configuration with multiple parent configurations
            ModuleFinder emptyFinder = ModuleFinder.of();
            Configuration multiParentConfig = Configuration.resolve(
                emptyFinder, List.of(bootLayer.configuration(), Configuration.empty()),
                emptyFinder, List.of());

            System.out.println("Multi-parent config parents: " +
                multiParentConfig.parents().size());
            System.out.println("Multi-parent config modules: " +
                multiParentConfig.modules().size());

            // Create a layer with multiple parents
            ModuleLayer multiParent = ModuleLayer.defineModulesWithOneLoader(
                multiParentConfig, List.of(bootLayer, emptyLayer),
                ClassLoader.getSystemClassLoader()).layer();

            System.out.println("Multi-parent layer parent count: " +
                multiParent.parents().size());
            System.out.println("Multi-parent layer parent 0 is boot: " +
                (multiParent.parents().get(0) == bootLayer));
            System.out.println("Multi-parent layer parent 1 is empty: " +
                (multiParent.parents().get(1) == emptyLayer));

            // Should be able to find boot layer modules from the multi-parent layer
            Optional<Module> javaBase = multiParent.findModule("java.base");
            System.out.println("Found java.base through multi-parent: " + javaBase.isPresent());

        } catch (Exception e) {
            System.out.println("Error: " + e.getClass().getSimpleName() + ": " + e.getMessage());
        }
    }

    /**
     * Test module lookup through the layer hierarchy.
     */
    private static void testLayerModuleLookup() {
        System.out.println("--- Test: Layer Module Lookup ---");
        try {
            ModuleLayer bootLayer = ModuleLayer.boot();

            // Find module in boot layer
            Optional<Module> javaBase = bootLayer.findModule("java.base");
            System.out.println("java.base in boot layer: " + javaBase.isPresent());
            if (javaBase.isPresent()) {
                System.out.println("java.base layer is boot: " +
                    (javaBase.get().getLayer() == bootLayer));
            }

            // Find module in empty layer (should be empty)
            ModuleLayer emptyLayer = ModuleLayer.empty();
            Optional<Module> javaBaseInEmpty = emptyLayer.findModule("java.base");
            System.out.println("java.base in empty layer: " + javaBaseInEmpty.isPresent());

            // Find non-existent module
            Optional<Module> nonExistent = bootLayer.findModule("non.existent.module");
            System.out.println("non.existent.module in boot layer: " + nonExistent.isPresent());
        } catch (Exception e) {
            System.out.println("Error: " + e.getMessage());
        }
    }

    /**
     * Test finding class loaders for modules in a layer.
     */
    private static void testLayerFindLoader() {
        System.out.println("--- Test: Layer Find Loader ---");
        try {
            ModuleLayer bootLayer = ModuleLayer.boot();

            // java.base should have the bootstrap (null) class loader
            ClassLoader baseLoader = bootLayer.findLoader("java.base");
            System.out.println("java.base loader is null (bootstrap): " + (baseLoader == null));

            // Try finding loader for a non-existent module (should throw)
            try {
                ClassLoader loader = bootLayer.findLoader("non.existent.module");
                System.out.println("non.existent.module loader: " + loader);
            } catch (IllegalArgumentException e) {
                System.out.println("non.existent.module findLoader: " +
                    e.getClass().getSimpleName());
            }
        } catch (Exception e) {
            System.out.println("Error: " + e.getMessage());
        }
    }
}
