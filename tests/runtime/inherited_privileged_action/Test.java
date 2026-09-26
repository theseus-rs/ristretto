import java.security.AccessController;
import java.security.PrivilegedAction;
import java.security.PrivilegedExceptionAction;
import java.io.ByteArrayOutputStream;
import java.io.DataOutputStream;

/** Native callbacks must invoke inherited implementations, including interface defaults. */
public class Test {
    public static class Action implements PrivilegedAction<String> {
        public String run() { return "base"; }
    }
    static class Middle extends Action {}
    static class Leaf extends Middle {}
    static class Override extends Leaf {
        public String run() { return "override"; }
    }
    interface DefaultAction extends PrivilegedAction<String> {
        default String run() { return "default"; }
    }
    static class DefaultLeaf implements DefaultAction {}
    static class ExceptionAction implements PrivilegedExceptionAction<String> {
        public String run() { return "exception action"; }
    }
    static class ExceptionLeaf extends ExceptionAction {}

    static class Loader extends ClassLoader {
        PrivilegedAction<?> action(int flags) throws Exception {
            String name = "GeneratedAction" + flags;
            ByteArrayOutputStream buffer = new ByteArrayOutputStream();
            DataOutputStream out = new DataOutputStream(buffer);
            out.writeInt(0xcafebabe); out.writeShort(0); out.writeShort(52);
            out.writeShort(12);
            out.writeByte(1); out.writeUTF(name); // 1
            out.writeByte(7); out.writeShort(1); // 2
            out.writeByte(1); out.writeUTF("Test$Action"); // 3
            out.writeByte(7); out.writeShort(3); // 4
            out.writeByte(1); out.writeUTF("<init>"); // 5
            out.writeByte(1); out.writeUTF("()V"); // 6
            out.writeByte(12); out.writeShort(5); out.writeShort(6); // 7
            out.writeByte(10); out.writeShort(4); out.writeShort(7); // 8
            out.writeByte(1); out.writeUTF("Code"); // 9
            out.writeByte(1); out.writeUTF("run"); // 10
            out.writeByte(1); out.writeUTF("()Ljava/lang/Object;"); // 11
            out.writeShort(0x21); out.writeShort(2); out.writeShort(4);
            out.writeShort(0); out.writeShort(0); out.writeShort(2);
            method(out, 1, 5, 6, new byte[]{0x2a, (byte) 0xb7, 0, 8, (byte) 0xb1});
            method(out, flags, 10, 11, new byte[]{0x01, (byte) 0xb0});
            out.writeShort(0);
            byte[] bytes = buffer.toByteArray();
            return (PrivilegedAction<?>) defineClass(name, bytes, 0, bytes.length).getConstructor().newInstance();
        }
        void method(DataOutputStream out, int flags, int name, int descriptor, byte[] code) throws Exception {
            out.writeShort(flags); out.writeShort(name); out.writeShort(descriptor);
            out.writeShort(1); out.writeShort(9); out.writeInt(12 + code.length);
            out.writeShort(1); out.writeShort(1); out.writeInt(code.length); out.write(code);
            out.writeShort(0); out.writeShort(0);
        }
    }

    public static void main(String[] args) throws Exception {
        System.out.println(AccessController.doPrivileged(new Leaf()));
        System.out.println(AccessController.doPrivileged(new Leaf(), null));
        System.out.println(AccessController.doPrivileged(new Override()));
        System.out.println(AccessController.doPrivileged(new DefaultLeaf()));
        System.out.println(AccessController.doPrivileged(new ExceptionLeaf()));
        System.out.println(AccessController.doPrivileged(new ExceptionLeaf(), null));
        Loader loader = new Loader();
        for (int flags : new int[]{2, 9}) {
            try {
                AccessController.doPrivileged(loader.action(flags));
                throw new AssertionError("legacy native entry must reject a non-public or static run");
            } catch (InternalError expected) {
                System.out.println("invalid run " + flags + ": " + expected.getMessage());
            }
        }
    }
}
