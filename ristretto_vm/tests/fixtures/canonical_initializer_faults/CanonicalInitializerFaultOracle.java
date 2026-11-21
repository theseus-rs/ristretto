public class CanonicalInitializerFaultOracle {
    public static void main(String[] args) throws Exception {
        for (String method : new String[]{"negative", "negativeReference", "bounds", "boundsStore", "store", "cast", "nullArray", "nested"}) {
            for (int attempt = 0; attempt < 2; attempt++) {
                try { CanonicalInitializerFaults.class.getMethod(method, int.class).invoke(null, 0); throw new AssertionError("no fault"); }
                catch (java.lang.reflect.InvocationTargetException wrapped) {
                    Throwable fault = wrapped.getCause();
                    String cause = fault instanceof ExceptionInInitializerError ? fault.getCause().getClass().getSimpleName() : "-";
                    System.out.println(method + " " + attempt + " " + fault.getClass().getSimpleName() + " " + cause);
                }
            }
        }
    }
}
