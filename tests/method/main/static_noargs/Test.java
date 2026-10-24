/** A static main() requires no accessible no-argument constructor. */
class Test {
    private Test() {
        throw new AssertionError("static main must not construct an instance");
    }

    static void main() {
        System.out.println("static main without arguments");
    }
}
