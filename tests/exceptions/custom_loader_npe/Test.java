import java.io.ByteArrayOutputStream;
import java.io.DataOutputStream;
import java.lang.invoke.MethodHandle;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.MethodType;

/** Saved NPE metadata must identify the definition after its throwing frame has returned. */
public class Test {
    static class Loader extends ClassLoader {
        Class<?> define(boolean arrayLength) throws Exception {
            ByteArrayOutputStream buffer = new ByteArrayOutputStream();
            DataOutputStream out = new DataOutputStream(buffer);
            out.writeInt(0xcafebabe); out.writeShort(0); out.writeShort(52);
            out.writeShort(8);
            out.writeByte(1); out.writeUTF("CustomNpe"); // 1
            out.writeByte(7); out.writeShort(1); // 2
            out.writeByte(1); out.writeUTF("java/lang/Object"); // 3
            out.writeByte(7); out.writeShort(3); // 4
            out.writeByte(1); out.writeUTF("fail"); // 5
            out.writeByte(1); out.writeUTF("()I"); // 6
            out.writeByte(1); out.writeUTF("Code"); // 7
            out.writeShort(0x21); out.writeShort(2); out.writeShort(4);
            out.writeShort(0); out.writeShort(0); out.writeShort(1);
            byte[] code = arrayLength
                    ? new byte[] {1, (byte) 0xbe, (byte) 0xac}
                    : new byte[] {1, 3, 0x2e, (byte) 0xac};
            out.writeShort(9); out.writeShort(5); out.writeShort(6);
            out.writeShort(1); out.writeShort(7); out.writeInt(12 + code.length);
            out.writeShort(2); out.writeShort(0); out.writeInt(code.length); out.write(code);
            out.writeShort(0); out.writeShort(0); out.writeShort(0);
            byte[] bytes = buffer.toByteArray();
            return defineClass("CustomNpe", bytes, 0, bytes.length);
        }
    }

    static NullPointerException capture(Class<?> type) throws Throwable {
        MethodHandle fail = MethodHandles.publicLookup().findStatic(
                type, "fail", MethodType.methodType(int.class));
        try {
            int ignored = (int) fail.invokeExact();
            throw new AssertionError("missing NPE");
        } catch (NullPointerException expected) {
            return expected;
        }
    }

    public static void main(String[] args) throws Throwable {
        Class<?> first = new Loader().define(true);
        Class<?> second = new Loader().define(false);
        NullPointerException length = capture(first);
        NullPointerException load = capture(second);
        // Neither custom definition has an active frame when the messages are requested.
        if (!"Cannot read the array length because \"null\" is null".equals(length.getMessage())) {
            throw new AssertionError("array length message: " + length.getMessage());
        }
        if (!"Cannot load from int array because \"null\" is null".equals(load.getMessage())) {
            throw new AssertionError("array load message: " + load.getMessage());
        }
        System.out.println(length.getMessage());
        System.out.println(load.getMessage());
    }
}
