import java.lang.reflect.InvocationTargetException;
public class CanonicalCastAccessOracle {
    public static void main(String[] args) throws Exception {
        for (String name : new String[] { "casttarget.CanonicalCastSamePackage", "castoutside.CanonicalCastOtherPackage" }) {
            Class<?> type = Class.forName(name);
            for (String method : new String[] { "classCast", "arrayCast", "arrayNew", "objectNew" }) {
                String result;
                try { result = type.getMethod(method, int.class).invoke(null, 3).toString(); }
                catch (InvocationTargetException error) { result = error.getCause().getClass().getName(); }
                System.out.println(args[0] + "\t" + name + "\t" + method + "\t" + result);
            }
        }
    }
}
