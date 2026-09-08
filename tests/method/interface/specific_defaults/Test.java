public class Test {
    interface Parent {
        default String value() { return value(true); }
        default String value(boolean flag) { return "parent"; }
    }
    interface Child extends Parent { default String value(boolean flag) { return "child"; } }
    interface Sibling extends Parent { }
    static class Base implements Child { }
    static class First implements Child, Sibling { }
    static class Second implements Sibling, Child { }
    static class Inherited extends Base implements Parent { }
    public static void main(String[] args) {
        for (Parent item : new Parent[]{new First(), new Second(), new Inherited()})
            System.out.println(item.value());
    }
}
