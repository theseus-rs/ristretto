import java.io.ByteArrayOutputStream;
import java.io.DataOutputStream;
import java.lang.reflect.Method;

/** Non-initializing array lookup must load, but not initialize, a Java-only component. */
public class Test {
    static byte[] bytes(String name, String descriptor, boolean factory) throws Exception {
        ByteArrayOutputStream buffer = new ByteArrayOutputStream();
        DataOutputStream out = new DataOutputStream(buffer);
        out.writeInt(0xcafebabe); out.writeShort(0); out.writeShort(52);
        out.writeShort(22);
        out.writeByte(1); out.writeUTF(name); // 1
        out.writeByte(7); out.writeShort(1); // 2
        out.writeByte(1); out.writeUTF("java/lang/Object"); // 3
        out.writeByte(7); out.writeShort(3); // 4
        out.writeByte(1); out.writeUTF("lookup"); // 5
        out.writeByte(1); out.writeUTF("()Ljava/lang/Class;"); // 6
        out.writeByte(1); out.writeUTF("Code"); // 7
        out.writeByte(1); out.writeUTF(descriptor); // 8
        out.writeByte(8); out.writeShort(8); // 9
        out.writeByte(1); out.writeUTF("java/lang/Class"); // 10
        out.writeByte(7); out.writeShort(10); // 11
        out.writeByte(1); out.writeUTF("getClassLoader"); // 12
        out.writeByte(1); out.writeUTF("()Ljava/lang/ClassLoader;"); // 13
        out.writeByte(12); out.writeShort(12); out.writeShort(13); // 14
        out.writeByte(10); out.writeShort(11); out.writeShort(14); // 15
        out.writeByte(1); out.writeUTF("forName"); // 16
        out.writeByte(1); out.writeUTF("(Ljava/lang/String;ZLjava/lang/ClassLoader;)Ljava/lang/Class;"); // 17
        out.writeByte(12); out.writeShort(16); out.writeShort(17); // 18
        out.writeByte(10); out.writeShort(11); out.writeShort(18); // 19
        out.writeByte(1); out.writeUTF("<clinit>"); // 20
        out.writeByte(1); out.writeUTF("()V"); // 21
        out.writeShort(0x21); out.writeShort(2); out.writeShort(4);
        out.writeShort(0); out.writeShort(0); out.writeShort(1);
        // Factory.lookup calls Class.forName(descriptor, false, Factory.class.getClassLoader()).
        // Element.<clinit> throws, so any accidental component initialization fails the lookup.
        byte[] code = factory
                ? new byte[] {0x12, 9, 0x03, 0x12, 2, (byte) 0xb6, 0, 15,
                              (byte) 0xb8, 0, 19, (byte) 0xb0}
                : new byte[] {0x01, (byte) 0xbf};
        out.writeShort(factory ? 9 : 8);
        out.writeShort(factory ? 5 : 20); out.writeShort(factory ? 6 : 21);
        out.writeShort(1); out.writeShort(7); out.writeInt(12 + code.length);
        out.writeShort(3); out.writeShort(0); out.writeInt(code.length); out.write(code);
        out.writeShort(0); out.writeShort(0); out.writeShort(0);
        return buffer.toByteArray();
    }

    static class Loader extends ClassLoader {
        final String factoryName;
        final String elementName;
        final String descriptor;
        Class<?> element;
        int elementFinds;

        Loader(int dimensions) {
            factoryName = "ArrayLookup" + dimensions;
            elementName = "DeferredElement" + dimensions;
            descriptor = (dimensions == 1 ? "[L" : "[[L") + elementName + ";";
        }

        @Override
        protected Class<?> findClass(String name) throws ClassNotFoundException {
            System.out.println("findClass: " + name);
            if (!name.equals(factoryName) && !name.equals(elementName)) {
                // Array descriptors must never be passed to the Java loader.
                throw new ClassNotFoundException(name);
            }
            try {
                byte[] code = bytes(name, descriptor, name.equals(factoryName));
                Class<?> defined = defineClass(name, code, 0, code.length);
                if (name.equals(elementName)) {
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
        for (int dimensions = 1; dimensions <= 2; dimensions++) {
            Loader loader = new Loader(dimensions);
            Method lookup = loader.loadClass(loader.factoryName).getMethod("lookup");
            System.out.println("component deferred: " + (loader.element == null));
            Class<?> array = (Class<?>) lookup.invoke(null);
            System.out.println("array name: " + array.getName());
            Class<?> component = array;
            for (int i = 0; i < dimensions; i++) component = component.getComponentType();
            System.out.println("component identity: " + (component == loader.element));
            System.out.println("component loader: " + (component.getClassLoader() == loader));
            System.out.println("cached array: " + (lookup.invoke(null) == array));
            System.out.println("component definitions: " + loader.elementFinds);
            try {
                Class.forName(loader.elementName, true, loader);
                throw new AssertionError("component initializer did not run");
            } catch (ExceptionInInitializerError expected) {
                System.out.println("component initialization deferred until explicit request");
            }
        }
    }
}
