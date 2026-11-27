/** Peer-only same-world sequence; never admitted into Program. */
public final class CanonicalSharedStaticsOracle {
    public static void main(String[] args) throws Exception {
        Class<?> owner = Class.forName("CanonicalSharedStatics");
        String[] methods = {"read","write","read","write","read","clear","slow","slow","waitWithRoot","nested","cycleA","cycleB","bad","bad"};
        int[] values = {0,21,0,-5,0,0,0,3,11,0,0,0,0,0};
        for (int i=0; i<methods.length; i++) {
            String result;
            try { result = owner.getDeclaredMethod(methods[i],int.class).invoke(null,values[i]).toString(); }
            catch (java.lang.reflect.InvocationTargetException error) { result = error.getCause().getClass().getSimpleName(); }
            System.out.println(methods[i]+"\t"+values[i]+"\t"+result);
        }
    }
}
