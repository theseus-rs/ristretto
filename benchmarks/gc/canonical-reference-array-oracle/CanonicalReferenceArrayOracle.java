import java.lang.reflect.InvocationTargetException;
public class CanonicalReferenceArrayOracle {
    static void run(String method, int argument) throws Exception {
        String result;
        try {
            result = CanonicalReferenceArrayWorkload.class.getMethod(method, int.class).invoke(null, argument).toString();
        } catch (InvocationTargetException error) { result = error.getCause().getClass().getName(); }
        System.out.println(method + "\t" + argument + "\t" + result);
    }
    public static void main(String[] ignored) throws Exception {
        for (String method : new String[] { "nodes", "mixed", "returned" })
            for (int length : new int[] { 0, 1, 8, 9, 65, 2048 }) run(method, length);
        run("failNegative", -1);
        run("failNegative", Integer.MIN_VALUE);
        for (String method : new String[] { "failBounds", "failStoreBounds", "failNull", "failNullStore" })
            for (int index : new int[] { -1, 1, Integer.MAX_VALUE }) run(method, index);
        run("failType", 0);
        run("failNullLength", 0);
    }
}
