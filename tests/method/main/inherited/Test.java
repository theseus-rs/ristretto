/** An inherited main(String[]) takes priority and runs on an instance of the launch class. */
class Parent {
    protected void main(String[] args) {
        System.out.println("inherited main");
        System.out.println(getClass().getName());
        System.out.println(args.length);
        System.out.println(value());
    }

    int value() {
        return -1;
    }
}

class Test extends Parent {
    private final int value;

    Test() {
        value = 42;
        System.out.println("child constructed");
    }

    int value() {
        return value;
    }

    static void main() {
        throw new AssertionError("inherited main(String[]) must take priority");
    }
}
