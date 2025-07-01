/** Test the short primitive type and its wrapper class. */
class Test {
    public static void main(String[] args) throws Exception {
        short zero = 0;
        short one = 1;
        System.out.println(zero);
        System.out.println(one);
        System.out.println(Short.MIN_VALUE);
        System.out.println(Short.MAX_VALUE);
        System.out.println(Short.valueOf(zero).hashCode());
        System.out.println(Short.valueOf(Short.MIN_VALUE).hashCode());
        System.out.println(Short.valueOf(Short.MAX_VALUE).hashCode());
    }
}
