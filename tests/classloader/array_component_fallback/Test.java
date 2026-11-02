import java.io.ByteArrayOutputStream;
import java.io.DataOutputStream;
import java.lang.reflect.Method;

/** Array components available only through a Java loader must reach findClass. */
public class Test {
    static byte[] bytes(String name, boolean factory) throws Exception {
        ByteArrayOutputStream buffer = new ByteArrayOutputStream();
        DataOutputStream out = new DataOutputStream(buffer);
        out.writeInt(0xcafebabe); out.writeShort(0); out.writeShort(52);
        out.writeShort(13);
        out.writeByte(1); out.writeUTF(name); // 1
        out.writeByte(7); out.writeShort(1); // 2
        out.writeByte(1); out.writeUTF("java/lang/Object"); // 3
        out.writeByte(7); out.writeShort(3); // 4
        out.writeByte(1); out.writeUTF("make"); // 5
        out.writeByte(1); out.writeUTF("()Ljava/lang/Class;"); // 6
        out.writeByte(1); out.writeUTF("Code"); // 7
        out.writeByte(1); out.writeUTF("Element"); // 8
        out.writeByte(7); out.writeShort(8); // 9
        out.writeByte(1); out.writeUTF("getClass"); // 10
        out.writeByte(12); out.writeShort(10); out.writeShort(6); // 11
        out.writeByte(10); out.writeShort(4); out.writeShort(11); // 12
        out.writeShort(0x21); out.writeShort(2); out.writeShort(4);
        out.writeShort(0); out.writeShort(0); out.writeShort(factory ? 1 : 0);
        if (factory) {
            // public static Class<?> make() { return new Element[1].getClass(); }
            // The signature must not resolve Element before the anewarray instruction.
            byte[] code = {0x04, (byte) 0xbd, 0, 9, (byte) 0xb6, 0, 12, (byte) 0xb0};
            out.writeShort(9); out.writeShort(5); out.writeShort(6);
            out.writeShort(1); out.writeShort(7); out.writeInt(12 + code.length);
            out.writeShort(1); out.writeShort(0); out.writeInt(code.length); out.write(code);
            out.writeShort(0); out.writeShort(0);
        }
        out.writeShort(0);
        return buffer.toByteArray();
    }

    static class Loader extends ClassLoader {
        Class<?> element;
        int elementFinds;

        @Override
        protected Class<?> findClass(String name) throws ClassNotFoundException {
            System.out.println("findClass: " + name);
            if (!name.equals("ArrayFactory") && !name.equals("Element")) {
                // In particular, the VM must not ask the Java loader for "[LElement;".
                throw new ClassNotFoundException(name);
            }
            try {
                byte[] code = bytes(name, name.equals("ArrayFactory"));
                Class<?> defined = defineClass(name, code, 0, code.length);
                if (name.equals("Element")) {
                    element = defined;
                    elementFinds++;
                }
                return defined;
            } catch (Exception error) {
                throw new ClassNotFoundException(name, error);
            }
        }
    }

    public static void main(String[] args) throws Exception {
        Loader loader = new Loader();
        Class<?> factory = loader.loadClass("ArrayFactory");
        Method make = factory.getMethod("make");
        System.out.println("component deferred: " + (loader.element == null));
        Class<?> array = (Class<?>) make.invoke(null);
        System.out.println("array name: " + array.getName());
        System.out.println("component identity: " + (array.getComponentType() == loader.element));
        System.out.println("component loader: " + (loader.element.getClassLoader() == loader));
        System.out.println("cached array: " + (make.invoke(null) == array));
        System.out.println("component definitions: " + loader.elementFinds);
    }
}
