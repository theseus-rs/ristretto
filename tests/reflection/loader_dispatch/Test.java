import java.io.ByteArrayOutputStream;
import java.io.DataOutputStream;

/** VM callbacks must honor both direct and inherited ClassLoader.loadClass overrides. */
public class Test {
    static class Loader extends ClassLoader {
        int calls;
        int dependencyCalls;
        public Class<?> loadClass(String name) throws ClassNotFoundException {
            if (name.equals("java.lang.String")) calls++;
            if (name.equals("OnlyInLoader")) {
                dependencyCalls++;
                try { return define(name, false); }
                catch (Exception error) { throw new ClassNotFoundException(name, error); }
            }
            return super.loadClass(name);
        }
        Class<?> define(String name, boolean entry) throws Exception {
            ByteArrayOutputStream buffer = new ByteArrayOutputStream();
            DataOutputStream out = new DataOutputStream(buffer);
            out.writeInt(0xcafebabe); out.writeShort(0); out.writeShort(52);
            out.writeShort(10);
            out.writeByte(1); out.writeUTF(name); // 1
            out.writeByte(7); out.writeShort(1); // 2
            out.writeByte(1); out.writeUTF("java/lang/Object"); // 3
            out.writeByte(7); out.writeShort(3); // 4
            out.writeByte(1); out.writeUTF("dependency"); // 5
            out.writeByte(1); out.writeUTF("()Ljava/lang/Class;"); // 6
            out.writeByte(1); out.writeUTF("Code"); // 7
            out.writeByte(1); out.writeUTF("OnlyInLoader"); // 8
            out.writeByte(7); out.writeShort(8); // 9
            out.writeShort(0x21); out.writeShort(2); out.writeShort(4);
            out.writeShort(0); out.writeShort(0); out.writeShort(entry ? 1 : 0);
            if (entry) {
                out.writeShort(9); out.writeShort(5); out.writeShort(6);
                out.writeShort(1); out.writeShort(7); out.writeInt(15);
                out.writeShort(1); out.writeShort(0); out.writeInt(3);
                out.write(new byte[]{0x12, 9, (byte) 0xb0});
                out.writeShort(0); out.writeShort(0);
            }
            out.writeShort(0);
            byte[] bytes = buffer.toByteArray();
            return defineClass(name, bytes, 0, bytes.length);
        }
    }
    static class Middle extends Loader {}
    static class Leaf extends Middle {}

    public static void main(String[] args) throws Exception {
        Loader direct = new Loader();
        System.out.println(Class.forName("java.lang.String", false, direct).getName());
        System.out.println("direct override: " + direct.calls);
        Loader inherited = new Leaf();
        System.out.println(Class.forName("java.lang.String", true, inherited).getName());
        System.out.println("inherited override: " + inherited.calls);
        Class<?> entry = inherited.define("LoaderEntry", true);
        Class<?> dependency = (Class<?>) entry.getMethod("dependency").invoke(null);
        System.out.println("symbolic dependency: " + dependency.getName());
        System.out.println("symbolic override: " + inherited.dependencyCalls);
    }
}
