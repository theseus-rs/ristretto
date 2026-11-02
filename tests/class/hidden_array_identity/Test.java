import java.io.ByteArrayOutputStream;
import java.io.DataOutputStream;
import java.lang.invoke.MethodHandles;
import java.lang.reflect.Array;
import java.util.function.Predicate;

/** Both Array.newInstance overloads must use the same hidden component identity. */
public class Test {
    static byte[] bytes() throws Exception {
        ByteArrayOutputStream buffer = new ByteArrayOutputStream();
        DataOutputStream out = new DataOutputStream(buffer);
        out.writeInt(0xcafebabe); out.writeShort(0); out.writeShort(52);
        out.writeShort(5);
        out.writeByte(1); out.writeUTF("Generated"); // 1
        out.writeByte(7); out.writeShort(1); // 2
        out.writeByte(1); out.writeUTF("java/lang/Object"); // 3
        out.writeByte(7); out.writeShort(3); // 4
        out.writeShort(0x21); out.writeShort(2); out.writeShort(4);
        out.writeShort(0); out.writeShort(0); out.writeShort(0); out.writeShort(0);
        return buffer.toByteArray();
    }

    static Class<?> check(Class<?> hidden, boolean dimensionsFirst) {
        Object single;
        Object dimensions;
        // Exercise both orders of populating the array-class cache.
        if (dimensionsFirst) {
            dimensions = Array.newInstance(hidden, new int[] {1});
            single = Array.newInstance(hidden, 1);
        } else {
            single = Array.newInstance(hidden, 1);
            dimensions = Array.newInstance(hidden, new int[] {1});
        }
        Class<?> singleType = single.getClass();
        Class<?> dimensionsType = dimensions.getClass();
        String expectedName = "[L" + hidden.getName() + ";";
        // Compare names relative to the hidden class; its suffix varies between VMs.
        System.out.println("overload identity: " + (singleType == dimensionsType));
        System.out.println("single name: " + singleType.getName().equals(expectedName));
        System.out.println("dimensions name: " + dimensionsType.getName().equals(expectedName));
        System.out.println("single component: " + (singleType.getComponentType() == hidden));
        System.out.println("dimensions component: " + (dimensionsType.getComponentType() == hidden));
        System.out.println("single loader: " + (singleType.getClassLoader() == hidden.getClassLoader()));
        System.out.println("dimensions loader: " + (dimensionsType.getClassLoader() == hidden.getClassLoader()));
        System.out.println("array is not hidden: " + (!singleType.isHidden() && !dimensionsType.isHidden()));
        System.out.println("cached single: " + (Array.newInstance(hidden, 2).getClass() == singleType));
        System.out.println("cached dimensions: "
                + (Array.newInstance(hidden, new int[] {2}).getClass() == dimensionsType));

        Object matrix = Array.newInstance(hidden, new int[] {1, 1});
        Class<?> matrixType = matrix.getClass();
        String expectedMatrixName = "[" + expectedName;
        System.out.println("matrix name: " + matrixType.getName().equals(expectedMatrixName));
        System.out.println("matrix component: " + (matrixType.getComponentType() == singleType));
        System.out.println("matrix row: " + (Array.get(matrix, 0).getClass() == singleType));
        System.out.println("matrix loader: " + (matrixType.getClassLoader() == hidden.getClassLoader()));
        return singleType;
    }

    public static void main(String[] args) throws Exception {
        MethodHandles.Lookup lookup = MethodHandles.lookup();
        Class<?> first = lookup.defineHiddenClass(bytes(), false).lookupClass();
        Class<?> second = lookup.defineHiddenClass(bytes(), false).lookupClass();
        System.out.println("single overload first");
        Class<?> firstArray = check(first, false);
        System.out.println("dimensions overload first");
        Class<?> secondArray = check(second, true);
        System.out.println("distinct hidden arrays: " + (firstArray != secondArray));

        // Predicate.negate creates its lambda in a bootstrap-defined class.
        Class<?> bootstrapHidden = Predicate.not(new Predicate<Object>() {
            public boolean test(Object value) { return false; }
        }).getClass();
        System.out.println("bootstrap component hidden: " + bootstrapHidden.isHidden());
        System.out.println("bootstrap component loader: " + (bootstrapHidden.getClassLoader() == null));
        check(bootstrapHidden, false);
    }
}
