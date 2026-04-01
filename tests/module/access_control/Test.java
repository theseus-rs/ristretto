/**
 * Module Access Control Tests
 * Tests JPMS access control: qualified vs unqualified opens/exports,
 * Module.isOpen/isExported checks, boot layer loader association,
 * and ServiceLoader provider discovery.
 */
public class Test {
    public static void main(String[] args) {
        System.out.println("=== Module Access Control Tests ===");

        testQualifiedVsUnqualifiedExports();
        testQualifiedVsUnqualifiedOpens();
        testModuleIsExported();
        testModuleIsOpen();
        testBootLayerLoaderAssociation();
        testServiceLoaderDiscovery();
        testModuleReadsConsistency();

        System.out.println("=== Module Access Control Tests Complete ===");
    }

    /**
     * Test that qualified exports are properly restricted to target modules,
     * while unqualified exports are visible to all.
     */
    private static void testQualifiedVsUnqualifiedExports() {
        System.out.println("--- Test: Qualified vs Unqualified Exports ---");
        try {
            Module javaBase = ModuleLayer.boot().findModule("java.base").orElse(null);
            if (javaBase == null || javaBase.getDescriptor() == null) {
                System.out.println("java.base not found");
                return;
            }

            long unqualifiedCount = javaBase.getDescriptor().exports().stream()
                .filter(e -> !e.isQualified())
                .count();
            long qualifiedCount = javaBase.getDescriptor().exports().stream()
                .filter(e -> e.isQualified())
                .count();

            System.out.println("java.base unqualified exports: " + (unqualifiedCount > 0));
            System.out.println("java.base qualified exports: " + (qualifiedCount > 0));

            // java.lang should be an unqualified export
            boolean javaLangExported = javaBase.getDescriptor().exports().stream()
                .anyMatch(e -> e.source().equals("java.lang") && !e.isQualified());
            System.out.println("java.lang is unqualified export: " + javaLangExported);

            // Verify a qualified export has specific targets
            javaBase.getDescriptor().exports().stream()
                .filter(e -> e.isQualified())
                .sorted(java.util.Comparator.comparing(e -> e.source()))
                .limit(3)
                .forEach(e -> {
                    System.out.println("Qualified export: " + e.source() +
                        " to " + e.targets().stream().sorted().collect(
                            java.util.stream.Collectors.joining(", ")));
                });
        } catch (Exception e) {
            System.out.println("Error: " + e.getMessage());
        }
    }

    /**
     * Test that qualified opens are properly restricted to target modules,
     * while unqualified opens grant access to all.
     */
    private static void testQualifiedVsUnqualifiedOpens() {
        System.out.println("--- Test: Qualified vs Unqualified Opens ---");
        try {
            ModuleLayer bootLayer = ModuleLayer.boot();
            boolean foundQualified = false;
            boolean foundUnqualified = false;

            // Sort modules by name for deterministic iteration order
            java.util.List<Module> sortedModules = bootLayer.modules().stream()
                .filter(m -> m.getDescriptor() != null)
                .sorted(java.util.Comparator.comparing(Module::getName))
                .collect(java.util.stream.Collectors.toList());

            for (Module m : sortedModules) {
                // Sort opens by source package name for deterministic order
                java.util.List<java.lang.module.ModuleDescriptor.Opens> sortedOpens =
                    m.getDescriptor().opens().stream()
                        .sorted(java.util.Comparator.comparing(
                            java.lang.module.ModuleDescriptor.Opens::source))
                        .collect(java.util.stream.Collectors.toList());
                for (java.lang.module.ModuleDescriptor.Opens opens : sortedOpens) {
                    if (opens.isQualified() && !foundQualified) {
                        foundQualified = true;
                        System.out.println("Qualified open found: " + m.getName() +
                            " opens " + opens.source() + " to " +
                            opens.targets().stream().sorted().collect(
                                java.util.stream.Collectors.joining(", ")));
                    }
                    if (!opens.isQualified() && !foundUnqualified) {
                        foundUnqualified = true;
                        System.out.println("Unqualified open found: " + m.getName() +
                            " opens " + opens.source() + " to all");
                    }
                    if (foundQualified && foundUnqualified) break;
                }
                if (foundQualified && foundUnqualified) break;
            }

            System.out.println("Has qualified opens: " + foundQualified);
            System.out.println("Has unqualified opens: " + foundUnqualified);
        } catch (Exception e) {
            System.out.println("Error: " + e.getMessage());
        }
    }

    /**
     * Test Module.isExported() behavior for qualified and unqualified exports.
     */
    private static void testModuleIsExported() {
        System.out.println("--- Test: Module.isExported ---");
        try {
            Module javaBase = ModuleLayer.boot().findModule("java.base").orElse(null);
            if (javaBase == null) {
                System.out.println("java.base not found");
                return;
            }

            // java.lang should be exported (unqualified)
            System.out.println("java.lang exported: " + javaBase.isExported("java.lang"));

            // java.lang should be exported to any module
            Module javaLogging = ModuleLayer.boot().findModule("java.logging").orElse(null);
            if (javaLogging != null) {
                System.out.println("java.lang exported to java.logging: " +
                    javaBase.isExported("java.lang", javaLogging));
            }

            // An internal package should not be exported to everyone
            // (sun.nio.cs is a qualified export in java.base)
            boolean sunNioCsExported = javaBase.isExported("sun.nio.cs");
            System.out.println("sun.nio.cs exported (unqualified): " + sunNioCsExported);
        } catch (Exception e) {
            System.out.println("Error: " + e.getMessage());
        }
    }

    /**
     * Test Module.isOpen() behavior for qualified and unqualified opens.
     */
    private static void testModuleIsOpen() {
        System.out.println("--- Test: Module.isOpen ---");
        try {
            Module javaBase = ModuleLayer.boot().findModule("java.base").orElse(null);
            if (javaBase == null) {
                System.out.println("java.base not found");
                return;
            }

            // java.base is not an open module (check via descriptor)
            System.out.println("java.base is open: " + javaBase.getDescriptor().isOpen());

            // java.lang should NOT be open (unqualified) in java.base
            System.out.println("java.lang open (unqualified): " + javaBase.isOpen("java.lang"));

            // Test isOpen with a target module
            Module javaLogging = ModuleLayer.boot().findModule("java.logging").orElse(null);
            if (javaLogging != null) {
                // java.lang is not opened to java.logging
                System.out.println("java.lang open to java.logging: " +
                    javaBase.isOpen("java.lang", javaLogging));
            }
        } catch (Exception e) {
            System.out.println("Error: " + e.getMessage());
        }
    }

    /**
     * Test that the boot layer is properly associated with class loaders.
     */
    private static void testBootLayerLoaderAssociation() {
        System.out.println("--- Test: Boot Layer Loader Association ---");
        try {
            ModuleLayer bootLayer = ModuleLayer.boot();

            // Boot layer should not be null
            System.out.println("Boot layer exists: " + (bootLayer != null));

            // Boot layer should have modules
            System.out.println("Boot layer has modules: " + !bootLayer.modules().isEmpty());

            // java.base should be findable in boot layer
            boolean hasJavaBase = bootLayer.findModule("java.base").isPresent();
            System.out.println("Boot layer has java.base: " + hasJavaBase);

            // java.logging should be findable
            boolean hasJavaLogging = bootLayer.findModule("java.logging").isPresent();
            System.out.println("Boot layer has java.logging: " + hasJavaLogging);

            // The boot layer's configuration should have modules
            System.out.println("Configuration has modules: " +
                !bootLayer.configuration().modules().isEmpty());

            // Boot layer should find a loader for java.base
            ClassLoader baseLoader = bootLayer.findLoader("java.base");
            System.out.println("java.base loader is bootstrap: " + (baseLoader == null));
        } catch (Exception e) {
            System.out.println("Error: " + e.getMessage());
        }
    }

    /**
     * Test that ServiceLoader can discover providers through the module system.
     */
    private static void testServiceLoaderDiscovery() {
        System.out.println("--- Test: ServiceLoader Discovery ---");
        try {
            // FileSystemProvider should be discoverable
            java.util.ServiceLoader<java.nio.file.spi.FileSystemProvider> fsLoader =
                java.util.ServiceLoader.load(java.nio.file.spi.FileSystemProvider.class);
            java.util.List<String> schemes = new java.util.ArrayList<>();
            for (java.nio.file.spi.FileSystemProvider provider : fsLoader) {
                schemes.add(provider.getScheme());
            }
            java.util.Collections.sort(schemes);
            for (int i = 0; i < Math.min(3, schemes.size()); i++) {
                System.out.println("FileSystem provider: " + schemes.get(i));
            }
            System.out.println("FileSystem providers found: " + !schemes.isEmpty());

            // Charset provider should be discoverable
            java.util.ServiceLoader<java.nio.charset.spi.CharsetProvider> csLoader =
                java.util.ServiceLoader.load(java.nio.charset.spi.CharsetProvider.class);
            int csCount = 0;
            for (java.nio.charset.spi.CharsetProvider provider : csLoader) {
                csCount++;
            }
            System.out.println("Charset providers found: " + csCount);
        } catch (Exception e) {
            System.out.println("Error: " + e.getMessage());
        }
    }

    /**
     * Test that module reads are consistent with the resolved configuration.
     */
    private static void testModuleReadsConsistency() {
        System.out.println("--- Test: Module Reads Consistency ---");
        try {
            Module javaBase = ModuleLayer.boot().findModule("java.base").orElse(null);
            Module javaLogging = ModuleLayer.boot().findModule("java.logging").orElse(null);
            if (javaBase == null || javaLogging == null) {
                System.out.println("Required modules not found");
                return;
            }

            // java.logging requires java.base, so it should read java.base
            System.out.println("java.logging reads java.base: " + javaLogging.canRead(javaBase));

            // Every module implicitly reads java.base
            boolean allReadBase = true;
            for (Module m : ModuleLayer.boot().modules()) {
                if (!m.canRead(javaBase) && !m.getName().equals("java.base")) {
                    allReadBase = false;
                    System.out.println("Module " + m.getName() + " does NOT read java.base");
                    break;
                }
            }
            System.out.println("All modules read java.base: " + allReadBase);

            // Check transitive reads: java.sql requires transitive java.logging
            Module javaSql = ModuleLayer.boot().findModule("java.sql").orElse(null);
            if (javaSql != null) {
                // Any module reading java.sql should also read java.logging (transitive)
                System.out.println("java.sql reads java.logging: " +
                    javaSql.canRead(javaLogging));

                // Verify the transitive dependency is in the descriptor
                boolean hasTransitiveLogging = javaSql.getDescriptor().requires().stream()
                    .anyMatch(r -> r.name().equals("java.logging") &&
                        r.modifiers().contains(
                            java.lang.module.ModuleDescriptor.Requires.Modifier.TRANSITIVE));
                System.out.println("java.sql requires transitive java.logging: " +
                    hasTransitiveLogging);
            }
        } catch (Exception e) {
            System.out.println("Error: " + e.getMessage());
        }
    }
}
