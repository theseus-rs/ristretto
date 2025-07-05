/** Test the byte primitive type and its wrapper class. */
class Test {
    public static void main(String[] args) throws Exception {
        byte zero = 0;
        byte one = 1;
        System.out.println(zero);
        System.out.println(one);
        System.out.println(Byte.MIN_VALUE);
        System.out.println(Byte.MAX_VALUE);
        System.out.println(Byte.valueOf(zero).hashCode());
        System.out.println(Byte.valueOf(Byte.MIN_VALUE).hashCode());
        System.out.println(Byte.valueOf(Byte.MAX_VALUE).hashCode());
    }
}
