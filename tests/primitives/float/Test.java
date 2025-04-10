class Test {
    public static void main(String[] args) throws Exception {
        System.out.println(0.0f);
        System.out.println(1.0f);
        System.out.println(Float.MIN_VALUE);
        System.out.println(Float.MAX_VALUE);
        System.out.println(Float.valueOf(0.0f).hashCode());
        System.out.println(Float.valueOf(Float.MIN_VALUE).hashCode());
        System.out.println(Float.valueOf(Float.MAX_VALUE).hashCode());
        System.out.println(1.0f + 4.0f);
        System.out.println(12.0f - 7.0f);
        System.out.println(5.0f * 7.0f);
        System.out.println(4.0f / 2.0f);
        System.out.println(5.0f % 2.0f);
        float value = 8.0f;
        System.out.println(++value);
        value++;
        System.out.println(value);
        System.out.println(--value);
        value--;
        System.out.println(value);
    }
}
