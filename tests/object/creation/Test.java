/** Test Object creation and toString behavior. */
public class Test {
    public static void main(String[] args) {
        Object o = new Object();
        System.out.println(o != null);
        System.out.println(o.toString().startsWith("java.lang.Object@"));
    }
}
