class Test {
    public static void main(String[] args) throws Exception {
        System.out.println(0L);
        System.out.println(1L);
        System.out.println(Long.MIN_VALUE);
        System.out.println(Long.MAX_VALUE);
        System.out.println(Long.valueOf(0L).hashCode());
        System.out.println(Long.valueOf(Long.MIN_VALUE).hashCode());
        System.out.println(Long.valueOf(Long.MAX_VALUE).hashCode());
        System.out.println(1L + 4L);
        System.out.println(12L - 7L);
        System.out.println(5L * 7L);
        System.out.println(4L / 2L);
        System.out.println(5L % 2L);
        long value = 8;
        System.out.println(++value);
        value++;
        System.out.println(value);
        System.out.println(--value);
        value--;
        System.out.println(value);
    }
}
