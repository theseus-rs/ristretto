import java.io.ByteArrayOutputStream;
import java.io.DataOutputStream;
import java.io.InputStream;
import java.lang.invoke.CallSite;
import java.lang.invoke.ConstantCallSite;
import java.lang.invoke.MethodHandle;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.MethodType;

/** Bootstrap owners, Class/MethodType arguments, and method/field handles must all use
 * the referring class's loader even when its parent has the same definitions. */
public class Test {
    static final String NAME = "LoaderDynamic";
    public static int bootstrapInitializations;

    public static class Bootstrap {
        static { bootstrapInitializations++; }
        public static CallSite bootstrap(MethodHandles.Lookup caller, String name, MethodType site,
                Class<?> type, MethodType signature, MethodHandle identity, MethodHandle getter)
                throws Exception {
            check(type == caller.lookupClass(), "bootstrap Class argument");
            check(Bootstrap.class.getClassLoader() == type.getClassLoader(), "bootstrap defining loader");
            check(site.returnType() == type && site.parameterCount() == 0, "call-site descriptor");
            check(signature.returnType() == type && signature.parameterType(0) == type, "MethodType argument");
            check(identity.type().equals(signature), "method handle descriptor");
            check(caller.revealDirect(identity).getDeclaringClass() == type, "method handle owner");
            check(getter.type().returnType() == type, "field handle descriptor");
            check(caller.revealDirect(getter).getDeclaringClass() == type, "field handle owner");
            System.out.println("bootstrap types and handles preserve defining loader");
            return new ConstantCallSite(MethodHandles.constant(type, null));
        }
    }

    static class Loader extends ClassLoader {
        Class<?> define(String name, byte[] bytes) { return defineClass(name, bytes, 0, bytes.length); }
    }

    public static void check(boolean condition, String message) {
        if (!condition) throw new AssertionError(message);
    }

    static class Pool {
        final ByteArrayOutputStream bytes = new ByteArrayOutputStream();
        final DataOutputStream out = new DataOutputStream(bytes);
        int count = 1;
        int utf(String text) throws Exception { out.writeByte(1); out.writeUTF(text); return count++; }

        int one(int tag, int index) throws Exception { out.writeByte(tag); out.writeShort(index); return count++; }

        int two(int tag, int first, int second) throws Exception {
            out.writeByte(tag); out.writeShort(first); out.writeShort(second); return count++;
        }

        int clazz(String name) throws Exception { return one(7, utf(name)); }

        int nameType(String name, String type) throws Exception { return two(12, utf(name), utf(type)); }

        int handle(int kind, int ref) throws Exception {
            out.writeByte(15); out.writeByte(kind); out.writeShort(ref); return count++;
        }
    }

    static void method(DataOutputStream out, int name, int descriptor, int codeName,
            int locals, byte[] code) throws Exception {
        out.writeShort(9); out.writeShort(name); out.writeShort(descriptor); out.writeShort(1);
        out.writeShort(codeName); out.writeInt(12 + code.length);
        out.writeShort(1); out.writeShort(locals); out.writeInt(code.length); out.write(code);
        out.writeShort(0); out.writeShort(0);
    }

    static byte[] bytes() throws Exception {
        Pool pool = new Pool();
        int self = pool.clazz(NAME), parent = pool.clazz("java/lang/Object");
        int run = pool.utf("run"), returnsSelf = pool.utf("()L" + NAME + ";"), code = pool.utf("Code");
        int identity = pool.utf("identity"), signature = pool.utf("(L" + NAME + ";)L" + NAME + ";");
        int identityRef = pool.two(10, self, pool.two(12, identity, signature));
        int identityHandle = pool.handle(6, identityRef);
        int value = pool.utf("value"), fieldType = pool.utf("L" + NAME + ";");
        int getter = pool.handle(2, pool.two(9, self, pool.two(12, value, fieldType)));
        int methodType = pool.one(16, signature);
        int owner = pool.clazz("Test$Bootstrap");
        int bootstrap = pool.two(10, owner, pool.nameType("bootstrap",
                "(Ljava/lang/invoke/MethodHandles$Lookup;Ljava/lang/String;Ljava/lang/invoke/MethodType;"
                + "Ljava/lang/Class;Ljava/lang/invoke/MethodType;Ljava/lang/invoke/MethodHandle;"
                + "Ljava/lang/invoke/MethodHandle;)Ljava/lang/invoke/CallSite;"));
        int bootstrapHandle = pool.handle(6, bootstrap);
        int bootstrapName = pool.utf("BootstrapMethods");
        int dynamic = pool.two(18, 0, pool.two(12, run, returnsSelf));
        ByteArrayOutputStream buffer = new ByteArrayOutputStream();
        DataOutputStream out = new DataOutputStream(buffer);
        out.writeInt(0xcafebabe); out.writeShort(0); out.writeShort(52);
        out.writeShort(pool.count); out.write(pool.bytes.toByteArray());
        out.writeShort(0x21); out.writeShort(self); out.writeShort(parent); out.writeShort(0);
        out.writeShort(1); out.writeShort(9); out.writeShort(value); out.writeShort(fieldType); out.writeShort(0);
        out.writeShort(2);
        method(out, run, returnsSelf, code, 0,
                new byte[] {(byte) 0xba, (byte) (dynamic >> 8), (byte) dynamic, 0, 0, (byte) 0xb0});
        method(out, identity, signature, code, 1, new byte[] {0x2a, (byte) 0xb0});
        out.writeShort(1); out.writeShort(bootstrapName); out.writeInt(14);
        out.writeShort(1); out.writeShort(bootstrapHandle); out.writeShort(4);
        out.writeShort(self); out.writeShort(methodType); out.writeShort(identityHandle); out.writeShort(getter);
        return buffer.toByteArray();
    }

    static void run(Class<?> type) throws Exception {
        check(type.getMethod("run").invoke(null) == null, "call-site result");
        check(type.getMethod("run").invoke(null) == null, "cached call-site result");
    }

    public static void main(String[] args) throws Throwable {
        byte[] dynamic = bytes();
        run(MethodHandles.lookup().defineClass(dynamic));
        for (int i = 0; i < 2; i++) {
            Loader loader = new Loader();
            try (InputStream in = Test.class.getResourceAsStream("/Test$Bootstrap.class")) {
                if (in == null) throw new AssertionError("missing bootstrap fixture");
                loader.define("Test$Bootstrap", in.readAllBytes());
            }
            run(loader.define(NAME, dynamic));
        }
        check(bootstrapInitializations == 3, "bootstrap initialization");
        System.out.println("bootstrap initialized once per definition; call sites remain isolated");
    }
}
