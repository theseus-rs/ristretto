public class Test {
    static class GreatGrandParent {
        public String name = "GreatGrandParentName";
    }

    static class GrandParent extends GreatGrandParent {
        public String name = "GrandParentName";
    }

    static class Parent extends GrandParent{
        public String name = "ParentName";
    }

    static class Child extends Parent {
        public String name = "ChildName";
    }

    public static void main(String[] args) {
        GreatGrandParent greatGrandParent = new GreatGrandParent();
        GrandParent grandParent = new GrandParent();
        Parent parent = new Parent();
        Child child = new Child();
        GreatGrandParent reference = new Child();

        System.out.print("greatGrandParent.name = ");
        System.out.println(greatGrandParent.name);

        System.out.print("grandParent.name = ");
        System.out.println(grandParent.name);

        System.out.print("parent.name = ");
        System.out.println(parent.name);

        System.out.print("child.name = ");
        System.out.println(child.name);

        System.out.print("reference.name = ");
        System.out.println(reference.name);

        System.out.print("((Child) reference).name = ");
        System.out.println(((Child) reference).name);

        System.out.print("((Parent) reference).name = ");
        System.out.println(((Parent) reference).name);

        System.out.print("((GrandParent) reference).name = ");
        System.out.println(((GrandParent) reference).name);

        System.out.print("((GreatGrandParent) reference).name = ");
        System.out.println(((GreatGrandParent) reference).name);
    }
}
