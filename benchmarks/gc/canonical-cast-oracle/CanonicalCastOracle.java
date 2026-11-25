import java.lang.reflect.InvocationTargetException;
public class CanonicalCastOracle {
    static void run(String method, int value) throws Exception {
        String result;
        try { result = CanonicalCastWorkload.class.getMethod(method, int.class).invoke(null, value).toString(); }
        catch (InvocationTargetException error) { result = error.getCause().getClass().getName(); }
        System.out.println(method + "\t" + value + "\t" + result);
    }
    public static void main(String[] ignored) throws Exception {
        for (String method : new String[] { "objects", "nullable", "booleans", "bytes", "chars", "shorts", "ints", "longs", "floats", "doubles", "referenceArrays", "objectArrays" })
            for (int value : new int[] { -257, -1, 0, 1, 257, 65535 }) run(method, value);
        for (String method : new String[] { "failClass", "failPrimitive", "failObjectArray", "failCovariance", "failNodeFromArray", "failArrayFromNode", "failArrayStore" }) run(method, 0);
        for (String method : new String[] { "failArrayBounds", "failNullArray" })
            for (int value : new int[] { -1, 1, Integer.MAX_VALUE }) run(method, value);
    }
}
