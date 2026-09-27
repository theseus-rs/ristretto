import java.io.ByteArrayOutputStream;
import java.io.DataOutputStream;
import java.lang.invoke.CallSite;
import java.lang.invoke.ConstantCallSite;
import java.lang.invoke.MethodHandle;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.MethodType;

/** Bootstrap method references may name a subclass that inherits the static method. */
public class Test {
    public static class Bootstrap {
        public static CallSite bootstrap(MethodHandles.Lookup lookup, String name, MethodType type) {
            return new LeafSite(MethodHandles.constant(String.class, "inherited bootstrap"));
        }
    }
    public static class ChildBootstrap extends Bootstrap {}
    public static class Site extends ConstantCallSite {
        Site(MethodHandle handle) { super(handle); }
    }
    public static class LeafSite extends Site {
        LeafSite(MethodHandle handle) { super(handle); }
    }
    static class Loader extends ClassLoader {
        Class<?> define(byte[] bytes) { return defineClass("Dynamic", bytes, 0, bytes.length); }
    }
    public static void main(String[] args) throws Exception {
        ByteArrayOutputStream buffer = new ByteArrayOutputStream();
        DataOutputStream out = new DataOutputStream(buffer);
        out.writeInt(0xcafebabe); out.writeShort(0); out.writeShort(52);
        out.writeShort(18);
        out.writeByte(1); out.writeUTF("Dynamic"); // 1
        out.writeByte(7); out.writeShort(1); // 2
        out.writeByte(1); out.writeUTF("java/lang/Object"); // 3
        out.writeByte(7); out.writeShort(3); // 4
        out.writeByte(1); out.writeUTF("call"); // 5
        out.writeByte(1); out.writeUTF("()Ljava/lang/String;"); // 6
        out.writeByte(1); out.writeUTF("Code"); // 7
        out.writeByte(1); out.writeUTF("Test$ChildBootstrap"); // 8
        out.writeByte(7); out.writeShort(8); // 9
        out.writeByte(1); out.writeUTF("bootstrap"); // 10
        out.writeByte(1); out.writeUTF("(Ljava/lang/invoke/MethodHandles$Lookup;Ljava/lang/String;Ljava/lang/invoke/MethodType;)Ljava/lang/invoke/CallSite;"); // 11
        out.writeByte(12); out.writeShort(10); out.writeShort(11); // 12
        out.writeByte(10); out.writeShort(9); out.writeShort(12); // 13
        out.writeByte(15); out.writeByte(6); out.writeShort(13); // 14
        out.writeByte(12); out.writeShort(5); out.writeShort(6); // 15
        out.writeByte(18); out.writeShort(0); out.writeShort(15); // 16
        out.writeByte(1); out.writeUTF("BootstrapMethods"); // 17
        out.writeShort(0x21); out.writeShort(2); out.writeShort(4);
        out.writeShort(0); out.writeShort(0); out.writeShort(1);
        out.writeShort(9); out.writeShort(5); out.writeShort(6);
        out.writeShort(1); out.writeShort(7); out.writeInt(18);
        out.writeShort(1); out.writeShort(0); out.writeInt(6);
        out.write(new byte[]{(byte) 0xba, 0, 16, 0, 0, (byte) 0xb0});
        out.writeShort(0); out.writeShort(0);
        out.writeShort(1); out.writeShort(17); out.writeInt(6);
        out.writeShort(1); out.writeShort(14); out.writeShort(0);
        Class<?> dynamic = new Loader().define(buffer.toByteArray());
        // Exercise both resolution and the cached call site.
        System.out.println(dynamic.getMethod("call").invoke(null));
        System.out.println(dynamic.getMethod("call").invoke(null));
    }
}
