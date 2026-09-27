import java.io.ByteArrayOutputStream;
import java.io.DataOutputStream;
import java.lang.invoke.MethodHandle;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.MethodType;
import java.lang.reflect.Method;

/** Valid class files can declare overrides and missing implementations that javac forbids. */
public class Test {
    public interface Value { Object value(); }
    public static class Base implements Value {
        public Object value() { return "base"; }
    }
    public static class Loader extends ClassLoader {
        Class<?> define(String name, String parent, int methodFlags) throws Exception {
            ByteArrayOutputStream buffer = new ByteArrayOutputStream();
            DataOutputStream out = new DataOutputStream(buffer);
            out.writeInt(0xcafebabe); out.writeShort(0); out.writeShort(52);
            out.writeShort(12);
            out.writeByte(1); out.writeUTF(name); // 1
            out.writeByte(7); out.writeShort(1); // 2
            out.writeByte(1); out.writeUTF(parent); // 3
            out.writeByte(7); out.writeShort(3); // 4
            out.writeByte(1); out.writeUTF("<init>"); // 5
            out.writeByte(1); out.writeUTF("()V"); // 6
            out.writeByte(12); out.writeShort(5); out.writeShort(6); // 7
            out.writeByte(10); out.writeShort(4); out.writeShort(7); // 8
            out.writeByte(1); out.writeUTF("Code"); // 9
            out.writeByte(1); out.writeUTF("value"); // 10
            out.writeByte(1); out.writeUTF("()Ljava/lang/Object;"); // 11
            out.writeShort((methodFlags & 0x400) != 0 ? 0x421 : 0x21);
            out.writeShort(2); out.writeShort(4);
            out.writeShort(0); out.writeShort(0);
            out.writeShort(methodFlags == 0 ? 1 : 2);
            method(out, 1, 5, 6, new byte[]{0x2a, (byte) 0xb7, 0, 8, (byte) 0xb1});
            if (methodFlags != 0) {
                method(out, methodFlags, 10, 11, new byte[]{0x01, (byte) 0xb0});
            }
            out.writeShort(0);
            byte[] bytes = buffer.toByteArray();
            return defineClass(name, bytes, 0, bytes.length);
        }
        void method(DataOutputStream out, int flags, int name, int descriptor, byte[] code) throws Exception {
            out.writeShort(flags); out.writeShort(name); out.writeShort(descriptor);
            if ((flags & 0x400) != 0) { out.writeShort(0); return; }
            out.writeShort(1); out.writeShort(9); out.writeInt(12 + code.length);
            out.writeShort(1); out.writeShort(1);
            out.writeInt(code.length); out.write(code);
            out.writeShort(0); out.writeShort(0);
        }
    }
    public static void main(String[] args) throws Throwable {
        Loader loader = new Loader();
        Method method = Base.class.getMethod("value");
        MethodHandle handle = MethodHandles.lookup().findVirtual(Base.class, "value", MethodType.methodType(Object.class));
        for (int flags : new int[]{2, 9}) {
            String name = "Receiver" + flags;
            loader.define(name, "Test$Base", flags);
            Base receiver = (Base) loader.define(name + "Leaf", name, 0).getConstructor().newInstance();
            System.out.println("virtual " + flags + ": " + receiver.value());
            System.out.println("interface " + flags + ": " + ((Value) receiver).value());
            System.out.println("reflection " + flags + ": " + method.invoke(receiver));
            System.out.println("handle " + flags + ": " + (Object) handle.invoke(receiver));
        }
        loader.define("AbstractReceiver", "Test$Base", 0x401);
        Base receiver = (Base) loader.define("MissingImplementation", "AbstractReceiver", 0).getConstructor().newInstance();
        try {
            receiver.value();
            throw new AssertionError("abstract override must not fall back to Base.value");
        } catch (AbstractMethodError expected) {
            System.out.println("abstract override rejected");
        }
        try {
            Object value = handle.invoke(receiver);
            throw new AssertionError("method handle must reject abstract override");
        } catch (AbstractMethodError expected) {
            System.out.println("abstract method handle rejected");
        }
        // invokeinterface must check access before rejecting an abstract implementation.
        // The concrete leaf inherits its generated parent's abstract method.
        for (int flags : new int[]{0x401, 0x404, 0x400}) {
            String name = "AbstractInterfaceReceiver" + flags;
            loader.define(name, "Test$Base", flags);
            Value missing = (Value) loader.define(name + "Leaf", name, 0).getConstructor().newInstance();
            for (int attempt = 0; attempt < 2; attempt++) {
                try {
                    missing.value();
                    throw new AssertionError("abstract interface implementation must be rejected");
                } catch (IncompatibleClassChangeError expected) {
                    System.out.println("abstract interface " + flags + " attempt " + attempt + ": "
                        + expected.getClass().getSimpleName());
                }
            }
        }
    }
}
