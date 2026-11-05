import java.io.InputStream;
import java.lang.reflect.RecordComponent;

/** Reflection metadata must resolve types through the class's defining loader. */
public class Test {
    static class DefiningLoader extends ClassLoader {
        DefiningLoader() {
            super(Test.class.getClassLoader());
        }

        Class<?> define(String name) throws Exception {
            try (InputStream in = Test.class.getResourceAsStream("/" + name + ".class")) {
                if (in == null) throw new AssertionError("missing fixture: " + name);
                byte[] bytes = in.readAllBytes();
                return defineClass(name, bytes, 0, bytes.length);
            }
        }
    }

    static void check(DefiningLoader loader) throws Exception {
        Class<?> outer = loader.define("Metadata$$Fixture");
        Class<?> member = loader.define("Metadata$$Fixture$Member");
        Class<?> pair = loader.define("Metadata$$Fixture$Pair");
        Class<?> local = loader.define("Metadata$$Fixture$1Local");
        if (outer == Metadata$$Fixture.class) throw new AssertionError("shared parent definition");
        if (member.getDeclaringClass() != outer || pair.getDeclaringClass() != outer) {
            throw new AssertionError("wrong declaring class");
        }
        for (Class<?> declared : outer.getDeclaredClasses()) {
            if (declared != member && declared != pair) throw new AssertionError("wrong declared class");
        }
        if (outer.getDeclaredClasses().length != 2) throw new AssertionError("missing declared classes");
        if (member.getNestHost() != outer || pair.getNestHost() != outer) {
            throw new AssertionError("wrong nest host");
        }
        Class<?>[] nest = outer.getNestMembers();
        if (nest.length != 4) throw new AssertionError("missing nest members");
        for (Class<?> entry : nest) {
            if (entry.getClassLoader() != loader) throw new AssertionError("wrong nest member loader");
        }
        if (local.getDeclaringClass() != null || local.getEnclosingClass() != outer
                || local.getEnclosingMethod().getDeclaringClass() != outer) {
            throw new AssertionError("wrong enclosing class");
        }
        RecordComponent component = pair.getRecordComponents()[0];
        if (component.getType() != outer || component.getAccessor().getReturnType() != outer) {
            throw new AssertionError("wrong record component type");
        }
        System.out.println("reflection loader identity: true");
    }

    public static void main(String[] args) throws Exception {
        check(new DefiningLoader());
        check(new DefiningLoader());
    }
}

class Metadata$$Fixture {
    public static class Member {}
    public record Pair(Metadata$$Fixture value) {}

    public static Class<?> local() {
        class Local {}
        return Local.class;
    }
}
