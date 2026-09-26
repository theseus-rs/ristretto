import java.lang.invoke.MethodHandles;
import java.lang.reflect.Method;
import java.io.ByteArrayOutputStream;
import java.io.InputStream;

/** Matching names do not override a package-private method across package boundaries. */
public class Test {
    static class Loader extends ClassLoader {
        Class<?> define() throws Exception {
            // Rename q.Child to p.Child without changing its superclass or method. It now has
            // the same package name as Base, but a different defining loader (and runtime package).
            ByteArrayOutputStream buffer = new ByteArrayOutputStream();
            try (InputStream input = Test.class.getResourceAsStream("/q/Child.class")) {
                byte[] chunk = new byte[1024];
                int count;
                while ((count = input.read(chunk)) != -1) buffer.write(chunk, 0, count);
            }
            byte[] bytes = buffer.toByteArray();
            byte[] name = new byte[]{'q', '/', 'C', 'h', 'i', 'l', 'd'};
            for (int i = 0; i <= bytes.length - name.length; i++) {
                boolean match = true;
                for (int j = 0; j < name.length; j++) match &= bytes[i + j] == name[j];
                if (match) bytes[i] = 'p';
            }
            return defineClass("p.Child", bytes, 0, bytes.length);
        }
    }
    public static void main(String[] args) throws Throwable {
        p.Base child = new q.Child();
        p.Base transitive = new q.Transitive();
        System.out.println("virtual: " + child.call());
        System.out.println("transitive: " + transitive.call());
        Method method = p.Base.class.getDeclaredMethod("value");
        method.setAccessible(true);
        System.out.println("reflection: " + method.invoke(child));
        System.out.println("reflection transitive: " + method.invoke(transitive));
        System.out.println("handle: " + (String) MethodHandles.lookup().unreflect(method).invoke(child));
        System.out.println("handle transitive: " + (String) MethodHandles.lookup().unreflect(method).invoke(transitive));
        p.Base foreign = (p.Base) new Loader().define().getConstructor().newInstance();
        System.out.println("different loader virtual: " + foreign.call());
        System.out.println("different loader reflection: " + method.invoke(foreign));
        System.out.println("different loader handle: " + (String) MethodHandles.lookup().unreflect(method).invoke(foreign));
    }
}
