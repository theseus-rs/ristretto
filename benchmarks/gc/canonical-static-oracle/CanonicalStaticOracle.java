/** Peer-only driver; never admitted into canonical Program. Each process is a fresh Java world. */
public final class CanonicalStaticOracle {
    public static void main(String[] args) throws Exception {
        String owner = args[0], method = args[1];
        int argument = Integer.parseInt(args[2]);
        try {
            Object result = Class.forName(owner).getDeclaredMethod(method, int.class).invoke(null, argument);
            System.out.println(owner + "\t" + method + "\t" + argument + "\t" + result);
        } catch (java.lang.reflect.InvocationTargetException failure) {
            Throwable error = failure.getCause();
            System.out.println(owner + "\t" + method + "\t" + argument + "\t" + error.getClass().getSimpleName() + ":" + (error.getCause() == null ? "none" : error.getCause().getClass().getSimpleName()));
        }
    }
}
