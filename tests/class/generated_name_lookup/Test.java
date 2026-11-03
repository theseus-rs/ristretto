import java.io.ByteArrayOutputStream;
import java.io.DataOutputStream;
import java.lang.invoke.MethodHandle;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.MethodType;

/**
 * Class-file names containing "$$" or "+0x" are legal ordinary names: {@code MethodHandles.Lookup.defineClass} and {@code ClassLoader.defineClass} define them under their own names, and the definitions must be findable by those names afterwards.
 *
 * Spring CGLIB relies on this: it defines {@code X$$SpringCGLIB$$0} and then asserts that {@code Class.forName("X$$SpringCGLIB$$0")} resolves the class it just defined.
 */
public class Test {

    static byte[] bytes(String name, int value) throws Exception {
        ByteArrayOutputStream buffer = new ByteArrayOutputStream();
        DataOutputStream out = new DataOutputStream(buffer);
        out.writeInt(0xcafebabe);
        out.writeShort(0);
        out.writeShort(52);
        out.writeShort(13);
        out.writeByte(1);
        out.writeUTF(name); // 1
        out.writeByte(7);
        out.writeShort(1); // 2
        out.writeByte(1);
        out.writeUTF("java/lang/Object"); // 3
        out.writeByte(7);
        out.writeShort(3); // 4
        out.writeByte(1);
        out.writeUTF("value"); // 5
        out.writeByte(1);
        out.writeUTF("()I"); // 6
        out.writeByte(1);
        out.writeUTF("Code"); // 7
        out.writeByte(1);
        out.writeUTF("self"); // 8
        out.writeByte(1);
        out.writeUTF("()Ljava/lang/Class;"); // 9
        out.writeByte(8);
        out.writeShort(1); // 10: literal shares the class-name UTF-8
        out.writeByte(1);
        out.writeUTF("literal"); // 11
        out.writeByte(1);
        out.writeUTF("()Ljava/lang/String;"); // 12
        out.writeShort(0x21);
        out.writeShort(2);
        out.writeShort(4);
        out.writeShort(0);
        out.writeShort(0);
        out.writeShort(3);
        method(out, 5, 6, new byte[] { 0x10, (byte) value, (byte) 0xac });
        method(out, 8, 9, new byte[] { 0x12, 2, (byte) 0xb0 });
        method(out, 11, 12, new byte[] { 0x12, 10, (byte) 0xb0 });
        out.writeShort(0);
        return buffer.toByteArray();
    }

    static void method(DataOutputStream out, int name, int descriptor, byte[] code) throws Exception {
        out.writeShort(9);
        out.writeShort(name);
        out.writeShort(descriptor);
        out.writeShort(1);
        out.writeShort(7);
        out.writeInt(12 + code.length);
        out.writeShort(1);
        out.writeShort(0);
        out.writeInt(code.length);
        out.write(code);
        out.writeShort(0);
        out.writeShort(0);
    }

    static void identity(String name, Class<?> type) {
        if (type.isHidden()) throw new AssertionError("ordinary definition marked hidden: " + type.getName());
        if (!name.equals(type.getName())) throw new AssertionError("renamed definition: " + type.getName());
        System.out.println("name: " + type.getName());
    }

    /** Resolve the definition by name through the loader that defined it. */
    static void findable(String name, ClassLoader loader, Class<?> defined) throws Exception {
        Class<?> found = Class.forName(name, false, loader);
        if (found != defined) throw new AssertionError("definition not resolvable by name: " + found.getName());
        System.out.println("findable: " + found.getName());
    }

    static void check(int expected, Class<?> type) throws Throwable {
        MethodHandle value = MethodHandles.lookup().findStatic(type, "value", MethodType.methodType(int.class));
        int actual = (int) value.invokeExact();
        if (actual != expected) throw new AssertionError("wrong definition: " + actual);
        MethodHandle self = MethodHandles.lookup().findStatic(type, "self", MethodType.methodType(Class.class));
        if ((Class<?>) self.invokeExact() != type) throw new AssertionError("wrong self reference");
        System.out.println("value: " + actual);
    }

    static void definition(String name, String loaderName) throws Throwable {
        MethodHandles.Lookup lookup = MethodHandles.lookup();
        Class<?> defined = lookup.defineClass(bytes(name, 7));
        identity(name, defined);
        findable(name, Test.class.getClassLoader(), defined);
        check(7, defined);
        System.out.println("reflected value: " + defined.getMethod("value").invoke(null));

        Class<?> viaLoader = new DefiningLoader(loaderName, bytes(loaderName, 9)).defined;
        identity(loaderName, viaLoader);
        findable(loaderName, viaLoader.getClassLoader(), viaLoader);
        System.out.println("loader reflected value: " + viaLoader.getMethod("value").invoke(null));
    }

    public static void main(String[] args) throws Throwable {
        definition("Generated$$Ordinary$$0", "Generated$$Loader$$0");
        definition("Generated+0xbar", "GeneratedLoader+0xbar");
        // Even a suffix matching the VM's hidden-name format is an ordinary name.
        definition("Generated+0x000000000000002a", "GeneratedLoader+0x000000000000002a");
    }

    static class DefiningLoader extends ClassLoader {

        Class<?> defined;

        DefiningLoader(String name, byte[] bytes) {
            super(Test.class.getClassLoader());
            defined = defineClass(name, bytes, 0, bytes.length);
        }
    }
}
