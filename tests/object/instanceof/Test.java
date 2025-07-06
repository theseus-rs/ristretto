/** Test Object instanceof behavior. */
public class Test {
    public static void main(String[] args) {
        Object o = new Object();
        System.out.println(o instanceof Object);
        String s = "abc";
        System.out.println(s instanceof Object);
    }
}
