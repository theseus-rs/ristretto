public class Test {
    public static void main(String[] args) {
        System.out.println(Parent.data());
    }
}

class Parent {
    static {
        System.out.println(Child.DATA);
        Child.DATA += "-parent";
    }

    public static String data() {
        return Child.DATA;
    }
}

class Child extends Parent {
    static String DATA;

    static {
        System.out.println(DATA);
        DATA += "-child";
    }
}
