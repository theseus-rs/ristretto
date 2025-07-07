/** Test Object equals and hashCode behavior. */
public class Test {
    public static void main(String[] args) {
        Object o1 = new Object();
        Object o2 = new Object();
        System.out.println(o1.equals(o1));
        System.out.println(o1.equals(o2));
        System.out.println(o1.hashCode() != o2.hashCode());
    }
}
