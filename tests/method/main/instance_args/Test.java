/** An instance main(String[]) takes priority over a static main(). */
class Test {
    Test() {
        System.out.println("constructed");
    }

    protected void main(String... args) {
        System.out.println("instance main with arguments");
        System.out.println(args.length);
    }

    public static void main() {
        throw new AssertionError("main(String[]) must take priority");
    }
}
