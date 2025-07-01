/** Test the integer primitive type and its wrapper class. */
class Test {
    public static void main(String[] args) throws Exception {
        System.out.println(-1);
        System.out.println(0);
        System.out.println(1);
        System.out.println(2);
        System.out.println(3);
        System.out.println(4);
        System.out.println(5);
        System.out.println(Integer.MIN_VALUE);
        System.out.println(Integer.MAX_VALUE);
        System.out.println(Integer.valueOf(0).hashCode());
        System.out.println(Integer.valueOf(Integer.MIN_VALUE).hashCode());
        System.out.println(Integer.valueOf(Integer.MAX_VALUE).hashCode());
        System.out.println(1 + 4);
        System.out.println(12 - 7);
        System.out.println(5 * 7);
        System.out.println(4 / 2);
        System.out.println(5 % 2);
        int value = 8;
        System.out.println(++value);
        value++;
        System.out.println(value);
        System.out.println(--value);
        value--;
        System.out.println(value);
    }
}
