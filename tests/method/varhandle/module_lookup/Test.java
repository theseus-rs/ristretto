import java.io.IOException;
import java.io.InputStream;
import java.lang.invoke.*;
import java.lang.module.*;
import java.net.URI;
import java.util.*;

public class Test {
    static class Loader extends ClassLoader {
        Class<?> target() throws IOException {
            try (InputStream in = modfixture.Target.class.getResourceAsStream("Target.class")) {
                if (in == null) throw new IOException("missing module target");
                byte[] bytes = in.readAllBytes();
                return defineClass("modfixture.Target", bytes, 0, bytes.length);
            }
        }
    }

    public static void main(String[] args) throws Exception {
        String name = "fixture.varhandle";
        ModuleDescriptor descriptor = ModuleDescriptor.newModule(name).exports("modfixture").build();
        ModuleReference ref = new ModuleReference(descriptor, URI.create("memory:/fixture")) {
            public ModuleReader open() { throw new AssertionError("loader supplies class bytes"); }
        };
        ModuleFinder finder = new ModuleFinder() {
            public Optional<ModuleReference> find(String n) { return name.equals(n) ? Optional.of(ref) : Optional.empty(); }
            public Set<ModuleReference> findAll() { return Set.of(ref); }
        };
        ModuleLayer boot = ModuleLayer.boot();
        Configuration config = boot.configuration().resolve(finder, ModuleFinder.of(), Set.of(name));
        Loader loader = new Loader();
        ModuleLayer.Controller controller = ModuleLayer.defineModules(config, List.of(boot), n -> loader);
        Module module = controller.layer().findModule(name).orElseThrow();
        Class<?> owner = loader.target();
        if (owner.getModule() != module || java.lang.reflect.Array.newInstance(owner, 0).getClass().getModule() != module)
            throw new AssertionError("target defined in wrong module");
        Loader sibling = new Loader();
        ModuleLayer.Controller second = ModuleLayer.defineModules(config, List.of(boot), n -> sibling);
        Module siblingModule = second.layer().findModule(name).orElseThrow();
        if (siblingModule == module || sibling.target().getModule() != siblingModule
                || owner.getModule() != module || modfixture.Target.class.getModule().isNamed())
            throw new AssertionError("modules must be scoped to their defining loaders");
        MethodHandles.Lookup caller = MethodHandles.lookup();
        VarHandle visible = caller.findStaticVarHandle(owner, "visible", int.class);
        if ((int) visible.get() != 3) throw new AssertionError("exported field");
        try {
            MethodHandles.privateLookupIn(owner, caller);
            throw new AssertionError("obtained private lookup before opening package");
        } catch (IllegalAccessException expected) {}
        controller.addOpens(module, "modfixture", Test.class.getModule());
        VarHandle hidden = MethodHandles.privateLookupIn(owner, caller).findStaticVarHandle(owner, "hidden", int.class);
        if ((int) hidden.getAndAdd(1) != 7 || (int) hidden.get() != 8) throw new AssertionError("opened field");
        System.out.println("Named module lookup passed");
    }
}
