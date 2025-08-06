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

        System.out.println("greatGrandParent.name = " + greatGrandParent.name);
        System.out.println("grandParent.name = " + grandParent.name);
        System.out.println("parent.name = " + parent.name);
        System.out.println("child.name = " + child.name);
        System.out.println("reference.name = " + reference.name);
        System.out.println("((Child) reference).name = " + ((Child) reference).name);
        System.out.println("((Parent) reference).name = " + ((Parent) reference).name);
        System.out.println("((GrandParent) reference).name = " + ((GrandParent) reference).name);
        System.out.println ("((GreatGrandParent) reference).name = " + ((GreatGrandParent) reference).name);
    }
}
