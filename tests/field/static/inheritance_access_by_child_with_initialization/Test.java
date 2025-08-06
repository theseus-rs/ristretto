public class Test {
    public static void main(String[] args) {
        System.out.println(Child.data());
    }
}

class Parent {
    static {
        System.out.println(Child.DATA);
        Child.DATA += "-parent";
    }
}

class Child extends Parent {
    static String DATA = "initial";

    static {
        System.out.println(DATA);
        DATA += "-child";
    }

    public static String data() {
        return DATA;
    }
}
