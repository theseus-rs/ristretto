/** Instance main must run after class initialization and the no-argument constructor. */
class Test {
    static {
        System.out.println("class initialized");
    }

    private final int value;

    protected Test() {
        value = 42;
        System.out.println("constructed");
    }

    void main() {
        System.out.println("instance main");
        System.out.println(value);
    }
}
