/** A classic inherited main is valid on every supported Java release. */
class Parent {
    static {
        System.out.println("parent initialized");
    }

    public static void main(String[] args) {
        System.out.println("inherited classic main");
        System.out.println(args.length);
    }
}

class Test extends Parent {
    static {
        System.out.println("child initialized");
    }

    private Test() {
        throw new AssertionError("static main must not construct an instance");
    }
}
