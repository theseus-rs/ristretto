import java.io.InputStream;

/** ClassLoader metadata must read the parent declared by ClassLoader itself. */
public class Test {
    static class Loader extends ClassLoader {
        private final String parent = "not a class loader";

        Loader(ClassLoader actualParent) { super(actualParent); }

        Class<?> define() throws Exception {
            try (InputStream in = Test.class.getResourceAsStream("/ParentFieldFixture.class")) {
                if (in == null) throw new AssertionError("missing fixture");
                byte[] bytes = in.readAllBytes();
                return defineClass("ParentFieldFixture", bytes, 0, bytes.length);
            }
        }
    }

    public static void main(String[] args) throws Exception {
        for (ClassLoader actualParent : new ClassLoader[] {null, Test.class.getClassLoader()}) {
            Loader loader = new Loader(actualParent);
            Class<?> defined = loader.define();
            if (defined.getClassLoader() != loader || loader.getParent() != actualParent) {
                throw new AssertionError("incorrect loader metadata");
            }
            if (Class.forName("ParentFieldFixture", false, loader) != defined) {
                throw new AssertionError("incorrect defined class");
            }
            System.out.println("shadowed parent preserved: " + loader.parent);
        }
    }
}

class ParentFieldFixture {}
