import java.io.ByteArrayOutputStream;
import java.io.DataOutputStream;
import java.lang.reflect.Array;
import java.lang.reflect.Method;

/** Array resolution must retain components supplied by loaders outside the parent chain. */
public class Test {
    static final String ELEMENT = "SiblingElement";
    static final String FACTORY = "SiblingFactory";

    static byte[] bytes(String name, boolean factory) throws Exception {
        ByteArrayOutputStream buffer = new ByteArrayOutputStream();
        DataOutputStream out = new DataOutputStream(buffer);
        out.writeInt(0xcafebabe); out.writeShort(0); out.writeShort(52);
        out.writeShort(20);
        out.writeByte(1); out.writeUTF(name); // 1
        out.writeByte(7); out.writeShort(1); // 2
        out.writeByte(1); out.writeUTF("java/lang/Object"); // 3
        out.writeByte(7); out.writeShort(3); // 4
        out.writeByte(1); out.writeUTF("make"); // 5
        out.writeByte(1); out.writeUTF("()Ljava/lang/Object;"); // 6
        out.writeByte(1); out.writeUTF("Code"); // 7
        out.writeByte(1); out.writeUTF(ELEMENT); // 8
        out.writeByte(7); out.writeShort(8); // 9
        out.writeByte(1); out.writeUTF("[[L" + ELEMENT + ";"); // 10
        out.writeByte(7); out.writeShort(10); // 11
        out.writeByte(1); out.writeUTF("multi"); // 12
        out.writeByte(1); out.writeUTF("<clinit>"); // 13
        out.writeByte(1); out.writeUTF("()V"); // 14
        out.writeByte(1); out.writeUTF("literal"); // 15
        out.writeByte(1); out.writeUTF("()Ljava/lang/Class;"); // 16
        out.writeByte(1); out.writeUTF("[L" + ELEMENT + ";"); // 17
        out.writeByte(7); out.writeShort(17); // 18
        out.writeByte(1); out.writeUTF("multiLiteral"); // 19
        out.writeShort(0x21); out.writeShort(2); out.writeShort(4);
        out.writeShort(0); out.writeShort(0); out.writeShort(factory ? 4 : 1);
        if (factory) {
            method(out, 5, 6, new byte[] {4, (byte) 0xbd, 0, 9, (byte) 0xb0});
            method(out, 12, 6, new byte[] {4, 5, (byte) 0xc5, 0, 11, 2, (byte) 0xb0});
            method(out, 15, 16, new byte[] {0x12, 18, (byte) 0xb0});
            method(out, 19, 16, new byte[] {0x12, 11, (byte) 0xb0});
        } else {
            // Any accidental component initialization makes the test fail.
            method(out, 13, 14, new byte[] {1, (byte) 0xbf});
        }
        out.writeShort(0);
        return buffer.toByteArray();
    }

    static void method(DataOutputStream out, int name, int descriptor, byte[] code)
            throws Exception {
        out.writeShort(9); out.writeShort(name); out.writeShort(descriptor);
        out.writeShort(1); out.writeShort(7); out.writeInt(12 + code.length);
        out.writeShort(2); out.writeShort(0); out.writeInt(code.length); out.write(code);
        out.writeShort(0); out.writeShort(0);
    }

    static class Loader extends ClassLoader {
        final ClassLoader sibling;
        Class<?> element;
        int finds;

        Loader(ClassLoader sibling) { this.sibling = sibling; }

        Class<?> loaded(String name) { return findLoadedClass(name); }

        @Override
        protected Class<?> findClass(String name) throws ClassNotFoundException {
            finds++;
            if (name.equals(ELEMENT) && sibling != null) return sibling.loadClass(name);
            if (!name.equals(ELEMENT) && !name.equals(FACTORY)) {
                throw new ClassNotFoundException(name);
            }
            try {
                byte[] code = bytes(name, name.equals(FACTORY));
                Class<?> defined = defineClass(name, code, 0, code.length);
                if (name.equals(ELEMENT)) element = defined;
                return defined;
            } catch (Exception error) {
                throw new ClassNotFoundException(name, error);
            }
        }
    }

    static void check(boolean condition) {
        if (!condition) throw new AssertionError("array identity or dimensions");
    }

    static void checkInitiatingLoader() throws Exception {
        Loader defining = new Loader(null);
        Loader initiating = new Loader(defining);
        Loader unrelated = new Loader(null);
        check(initiating.loaded(ELEMENT) == null && initiating.finds == 0);
        Class<?> element = Class.forName(ELEMENT, false, initiating);
        int finds = initiating.finds;
        check(element.getClassLoader() == defining);
        check(defining.loaded(ELEMENT) == element);
        check(initiating.loaded(ELEMENT) == element);
        check(unrelated.loaded(ELEMENT) == null && unrelated.finds == 0);
        check(initiating.loadClass(ELEMENT) == element && initiating.finds == finds);
        // Initialization failure must not erase a successful initiating association.
        try {
            Class.forName(ELEMENT, true, initiating);
            throw new AssertionError("component initialized successfully");
        } catch (ExceptionInInitializerError expected) {}
        check(initiating.loaded(ELEMENT) == element);
        check(Class.forName(ELEMENT, false, initiating) == element);
        check(initiating.finds == finds);
        System.out.println("initiating-loader lookups retain delegated definitions");
    }

    static Class<?>[] checkPair(boolean multiFirst) throws Exception {
        Loader defining = new Loader(null);
        Loader initiating = new Loader(defining);
        Class<?> factory = initiating.loadClass(FACTORY);
        check(defining.element == null);
        Method make = factory.getMethod("make");
        Method multi = factory.getMethod("multi");
        // Exercise each bytecode's fallback before the sibling component has been loaded.
        Object first = (multiFirst ? multi : make).invoke(null);
        check(initiating.loaded(ELEMENT) == defining.element);
        int finds = initiating.finds;
        Object second = (multiFirst ? make : multi).invoke(null);
        check(initiating.finds == finds);
        Object single = multiFirst ? second : first;
        Object matrix = multiFirst ? first : second;
        Class<?> element = defining.element;
        // Build mirrors through ldc before Object.getClass or Class.forName has cached them.
        Class<?> literal = (Class<?>) factory.getMethod("literal").invoke(null);
        Object fromLiteral = Array.newInstance(literal, 1);
        Class<?> multiLiteral = (Class<?>) factory.getMethod("multiLiteral").invoke(null);
        Class<?> array = single.getClass();
        Class<?> arrays = matrix.getClass();
        check(literal == array && multiLiteral == arrays && fromLiteral.getClass() == arrays);
        check(element != null && element.getClassLoader() == defining);
        check(array.getComponentType() == element && array.getClassLoader() == defining);
        check(arrays.getComponentType() == array && arrays.getClassLoader() == defining);
        check(Array.getLength(single) == 1 && Array.getLength(Array.get(matrix, 0)) == 2);
        check(make.invoke(null).getClass() == array && multi.invoke(null).getClass() == arrays);
        check(Class.forName("[L" + ELEMENT + ";", true, initiating) == array);
        check(Class.forName("[[L" + ELEMENT + ";", false, initiating) == arrays);
        check(Class.forName("[L" + ELEMENT + ";", false, defining) == array);
        check(Array.newInstance(element, 2).getClass() == array);
        check(Array.newInstance(element, new int[] {2}).getClass() == array);
        Object reflected = Array.newInstance(element, new int[] {1, 2});
        check(reflected.getClass() == arrays && Array.get(reflected, 0).getClass() == array);
        check(Array.newInstance(element, new int[] {0, 2}).getClass() == arrays);
        check(Array.newInstance(array, 1).getClass() == arrays);
        check(Array.newInstance(array, new int[] {1}).getClass() == arrays);
        try {
            Class.forName("[L[I;", false, initiating);
            throw new AssertionError("invalid object-array descriptor accepted");
        } catch (ClassNotFoundException expected) {}
        System.out.println("sibling arrays preserve identity and defer initialization");
        return new Class<?>[] {element, array, arrays};
    }

    public static void main(String[] args) throws Exception {
        checkInitiatingLoader();
        Class<?>[] first = checkPair(false);
        Class<?>[] second = checkPair(true);
        for (int i = 0; i < first.length; i++) {
            check(first[i] != second[i]);
            check(first[i].getName().equals(second[i].getName()));
        }
        // The second loader must not change reflection results for the first definition.
        check(Array.newInstance(first[0], 1).getClass() == first[1]);
        check(Array.newInstance(first[0], new int[] {1, 1}).getClass() == first[2]);
        check(Array.newInstance(first[1], 1).getClass() == first[2]);
        System.out.println("same-named sibling definitions remain distinct");
    }
}
