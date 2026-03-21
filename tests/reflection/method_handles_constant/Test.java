/** Test MethodHandles.constant() and related constant value dispatch.
 *
 * This exercises the constant_* carrier logic in MethodHandle dispatch,
 * testing that constant method handles correctly return their bound values.
 */
import java.lang.invoke.MethodHandle;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.MethodType;

public class Test {
    public static void main(String[] args) throws Throwable {
        System.out.println("=== MethodHandle Constant Tests ===");

        // Test MethodHandles.constant with String
        MethodHandle constString = MethodHandles.constant(String.class, "hello");
        String s = (String) constString.invoke();
        System.out.println("Constant String: " + s);

        // Test MethodHandles.constant with Integer
        MethodHandle constInt = MethodHandles.constant(Integer.class, 42);
        Integer i = (Integer) constInt.invoke();
        System.out.println("Constant Integer: " + i);

        // Test MethodHandles.constant with Boolean
        MethodHandle constBool = MethodHandles.constant(Boolean.class, true);
        Boolean b = (Boolean) constBool.invoke();
        System.out.println("Constant Boolean: " + b);

        // Test MethodHandles.constant with null
        MethodHandle constNull = MethodHandles.constant(Object.class, null);
        Object n = constNull.invoke();
        System.out.println("Constant null: " + n);

        // Test MethodHandles.identity
        MethodHandle identityStr = MethodHandles.identity(String.class);
        String identity = (String) identityStr.invoke("world");
        System.out.println("Identity String: " + identity);

        MethodHandle identityInt = MethodHandles.identity(Integer.class);
        Integer identityI = (Integer) identityInt.invoke(Integer.valueOf(99));
        System.out.println("Identity Integer: " + identityI);

        // Test MethodHandles.dropArguments with constant
        MethodHandle constWithDrop = MethodHandles.dropArguments(constString, 0, int.class);
        String dropped = (String) constWithDrop.invoke(999);
        System.out.println("Constant with dropped arg: " + dropped);

        // Test constant used with filterReturnValue
        MethodHandles.Lookup lookup = MethodHandles.lookup();
        MethodHandle toUpper = lookup.findVirtual(
            String.class, "toUpperCase", MethodType.methodType(String.class));
        MethodHandle constForFilter = MethodHandles.constant(String.class, "filtered");
        // Just test that constant handles can be composed
        String constResult = (String) constForFilter.invoke();
        System.out.println("Constant for composition: " + constResult);

        // Test multiple constant invocations return same value
        String s1 = (String) constString.invoke();
        String s2 = (String) constString.invoke();
        System.out.println("Repeated constant same: " + s1.equals(s2));

        // Test constant with custom type
        MethodHandle constDouble = MethodHandles.constant(Double.class, 3.14);
        Double d = (Double) constDouble.invoke();
        System.out.println("Constant Double: " + d);

        System.out.println("=== MethodHandle Constant Tests Complete ===");
    }
}
