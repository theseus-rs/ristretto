/** A non-public static main(String[]) takes priority over an instance main(). */
class Test {
    private Test(int ignored) {
        throw new AssertionError("static main must not construct an instance");
    }

    static void main(String[] args) {
        System.out.println("static main with arguments");
        System.out.println(args.length);
    }

    public void main() {
        throw new AssertionError("main(String[]) must take priority");
    }
}
