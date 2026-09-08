public class Test {
    interface Value {
        private String value() { return "private interface"; }
        default String direct() { return value(); }
        default java.util.function.Supplier<String> reference() { return this::value; }
    }
    static class Implementation implements Value {
        public String value() { return "implementation"; }
    }
    public static void main(String[] args) {
        Value value = new Implementation();
        System.out.println(value.direct());
        System.out.println(value.reference().get());
    }
}
