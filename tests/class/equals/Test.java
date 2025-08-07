/** Test Object class equality behavior (if_acmpeq/if_acmpne). */
public class Test {
    public static void main(String[] args) {
        Object o = new Object();
        System.out.println(o.getClass() == Object.class);
        System.out.println(o.getClass() == Integer.class);

        String s = "abc";
        System.out.println(s.getClass() == String.class);
        System.out.println(((Object) s.getClass()) == Object.class);
    }
}
