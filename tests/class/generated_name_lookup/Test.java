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
        out.writeShort(21);
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
        out.writeByte(1);
        out.writeUTF("marker"); // 13
        out.writeByte(1);
        out.writeUTF("Ljava/lang/Object;"); // 14
        out.writeByte(1);
        out.writeUTF("identity"); // 15
        out.writeByte(1);
        out.writeUTF("(L" + name + ";)L" + name + ";"); // 16
        out.writeByte(1);
        out.writeUTF("<init>"); // 17
        out.writeByte(1);
        out.writeUTF("()V"); // 18
        out.writeByte(12);
        out.writeShort(17);
        out.writeShort(18); // 19
        out.writeByte(10);
        out.writeShort(4);
        out.writeShort(19); // 20: Object.<init>
        out.writeShort(0x21);
        out.writeShort(2);
        out.writeShort(4);
        out.writeShort(0);
        out.writeShort(1);
        out.writeShort(9); // public static marker
        out.writeShort(13);
        out.writeShort(14);
        out.writeShort(0);
        out.writeShort(5);
        method(out, 5, 6, new byte[] { 0x10, (byte) value, (byte) 0xac });
        method(out, 8, 9, new byte[] { 0x12, 2, (byte) 0xb0 });
        method(out, 11, 12, new byte[] { 0x12, 10, (byte) 0xb0 });
        method(out, 15, 16, new byte[] { 0x2a, (byte) 0xb0 }, 1);
        method(out, 17, 18, new byte[] { 0x2a, (byte) 0xb7, 0, 20, (byte) 0xb1 }, 1);
        out.writeShort(0);
        return buffer.toByteArray();
    }

    static void method(DataOutputStream out, int name, int descriptor, byte[] code) throws Exception {
        method(out, name, descriptor, code, 0);
    }

    static void method(DataOutputStream out, int name, int descriptor, byte[] code, int locals) throws Exception {
        out.writeShort(name == 17 ? 1 : 9); // constructor is an instance method
        out.writeShort(name);
        out.writeShort(descriptor);
        out.writeShort(1);
        out.writeShort(7);
        out.writeInt(12 + code.length);
        out.writeShort(1);
        out.writeShort(locals);
        out.writeInt(code.length);
        out.write(code);
        out.writeShort(0);
        out.writeShort(0);
    }

    static void identity(String name, Class<?> type) {
        if (type.isHidden()) throw new AssertionError("ordinary definition marked hidden: " + type.getName());
        if (!name.equals(type.getName())) throw new AssertionError("renamed definition: " + type.getName());
        if (type.getDeclaringClass() != null || type.getEnclosingClass() != null) {
            throw new AssertionError("ordinary name mistaken for an inner class");
        }
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
        Class<?> reflected = (Class<?>) type.getMethod("self").invoke(null);
        if (reflected != type) throw new AssertionError("wrong reflected self reference");
        java.lang.reflect.Method identity = type.getMethod("identity", type);
        if (identity.getReturnType() != type || identity.getParameterTypes()[0] != type) {
            throw new AssertionError("wrong descriptor class identity");
        }
        Object instance = type.getConstructor().newInstance();
        if (instance.getClass() != type || !type.isInstance(instance)) {
            throw new AssertionError("wrong reflected instance class");
        }
        Object constructed = MethodHandles.lookup().findConstructor(type, MethodType.methodType(void.class)).invoke();
        if (constructed.getClass() != type) throw new AssertionError("wrong constructor handle class");
        System.out.println("value: " + actual);
    }

    static void definition(String name, String loaderName) throws Throwable {
        MethodHandles.Lookup lookup = MethodHandles.lookup();
        Class<?> defined = lookup.defineClass(bytes(name, 7));
        identity(name, defined);
        findable(name, Test.class.getClassLoader(), defined);
        check(7, defined);
        System.out.println("reflected literal: " + defined.getMethod("literal").invoke(null));

        Class<?> viaLoader = new DefiningLoader(loaderName, bytes(loaderName, 9)).defined;
        identity(loaderName, viaLoader);
        findable(loaderName, viaLoader.getClassLoader(), viaLoader);
        check(9, viaLoader);
        System.out.println("loader reflected literal: " + viaLoader.getMethod("literal").invoke(null));

        // The binary name is identical, but each defining loader owns a distinct class.
        Class<?> second = new DefiningLoader(loaderName, bytes(loaderName, 13)).defined;
        if (viaLoader == second) throw new AssertionError("shared class mirror");
        if (viaLoader.getClassLoader() == second.getClassLoader()) throw new AssertionError("shared loader");
        identity(loaderName, second);
        check(9, viaLoader);
        check(13, second);
        findable(loaderName, viaLoader.getClassLoader(), viaLoader);
        findable(loaderName, second.getClassLoader(), second);
        MethodHandles.Lookup fields = MethodHandles.lookup();
        fields.findStaticSetter(viaLoader, "marker", Object.class).invokeExact((Object) "first");
        fields.findStaticSetter(second, "marker", Object.class).invokeExact((Object) "second");
        Object firstMarker = (Object) fields.findStaticGetter(viaLoader, "marker", Object.class).invokeExact();
        Object secondMarker = (Object) fields.findStaticGetter(second, "marker", Object.class).invokeExact();
        if (!"first".equals(firstMarker) || !"second".equals(secondMarker)) {
            throw new AssertionError("shared static field storage");
        }
        if (viaLoader.isAssignableFrom(second) || second.isAssignableFrom(viaLoader)) {
            throw new AssertionError("unrelated definitions are assignable");
        }
        if (viaLoader.isInstance(second.getConstructor().newInstance())) {
            throw new AssertionError("instance belongs to the other loader");
        }
        try {
            Class.forName(loaderName, false, Test.class.getClassLoader());
            throw new AssertionError("child definition leaked to parent loader");
        } catch (ClassNotFoundException expected) {
            System.out.println("loader isolation: true");
        }

        // A child may also define a name that its parent has already defined.
        Class<?> shadow = new DefiningLoader(name, bytes(name, 17)).defined;
        check(7, defined);
        check(17, shadow);
        findable(name, Test.class.getClassLoader(), defined);
        findable(name, shadow.getClassLoader(), shadow);
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
