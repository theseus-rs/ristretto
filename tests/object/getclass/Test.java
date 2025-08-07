/** Test Object.getClass() behavior. */
public class Test {
    public static void main(String[] args) {
        Object o = new Object();
        System.out.println(o.getClass());

        String s = "abc";
        System.out.println(s.getClass());
    }
}
