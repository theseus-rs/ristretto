/** Test the boolean primitive type and its wrapper class. */
class Test {
    public static void main(String[] args) throws Exception {
        System.out.println(Boolean.TRUE);
        System.out.println(Boolean.FALSE);
        System.out.println(Boolean.valueOf(true));
        System.out.println(Boolean.valueOf(false));
        System.out.println(Boolean.valueOf("true"));
        System.out.println(Boolean.valueOf("false"));
        System.out.println(Boolean.TRUE.hashCode());
        System.out.println(Boolean.FALSE.hashCode());
        System.out.println(Boolean.TRUE.equals(Boolean.TRUE));
        System.out.println(Boolean.TRUE.equals(Boolean.FALSE));
    }
}
