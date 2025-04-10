class Test {
    public static void main(String[] args) throws Exception {
        System.out.println(0.0d);
        System.out.println(1.0d);
        System.out.println(Double.MIN_VALUE);
        System.out.println(Double.MAX_VALUE);
        System.out.println(Double.valueOf(0.0d).hashCode());
        System.out.println(Double.valueOf(Double.MIN_VALUE).hashCode());
        System.out.println(Double.valueOf(Double.MAX_VALUE).hashCode());
        System.out.println(1.0d + 4.0d);
        System.out.println(12.0d - 7.0d);
        System.out.println(5.0d * 7.0d);
        System.out.println(4.0d / 2.0d);
        System.out.println(5.0d % 2.0d);
        double value = 8.0d;
        System.out.println(++value);
        value++;
        System.out.println(value);
        System.out.println(--value);
        value--;
        System.out.println(value);
    }
}
