/** Test instanceof behavior with class inheritance hierarchies */
class GrandParent {
    public void grandParentMethod() {}
}

class Parent extends GrandParent {
    public void parentMethod() {}
}

class Child extends Parent {
    public void childMethod() {}
}

class Sibling extends Parent {
    public void siblingMethod() {}
}

class UnrelatedClass {
    public void unrelatedMethod() {}
}

public class Test {
    public static void main(String[] args) {
        // Test inheritance chain instanceof
        Child child = new Child();
        System.out.println("Child instanceof Child: " + (child instanceof Child));
        System.out.println("Child instanceof Parent: " + (child instanceof Parent));
        System.out.println("Child instanceof GrandParent: " + (child instanceof GrandParent));
        System.out.println("Child instanceof Object: " + (child instanceof Object));

        // Test parent instanceof
        Parent parent = new Parent();
        System.out.println("Parent instanceof Parent: " + (parent instanceof Parent));
        System.out.println("Parent instanceof GrandParent: " + (parent instanceof GrandParent));
        System.out.println("Parent instanceof Object: " + (parent instanceof Object));
        System.out.println("Parent instanceof Child: " + (parent instanceof Child));
        System.out.println("Parent instanceof Sibling: " + (parent instanceof Sibling));

        // Test grandparent instanceof
        GrandParent grandParent = new GrandParent();
        System.out.println("GrandParent instanceof GrandParent: " + (grandParent instanceof GrandParent));
        System.out.println("GrandParent instanceof Object: " + (grandParent instanceof Object));
        System.out.println("GrandParent instanceof Parent: " + (grandParent instanceof Parent));
        System.out.println("GrandParent instanceof Child: " + (grandParent instanceof Child));

        // Test sibling instanceof
        Sibling sibling = new Sibling();
        System.out.println("Sibling instanceof Sibling: " + (sibling instanceof Sibling));
        System.out.println("Sibling instanceof Parent: " + (sibling instanceof Parent));
        System.out.println("Sibling instanceof GrandParent: " + (sibling instanceof GrandParent));
        System.out.println("Sibling instanceof Object: " + (sibling instanceof Object));

        // Test unrelated class instanceof
        UnrelatedClass unrelated = new UnrelatedClass();
        System.out.println("UnrelatedClass instanceof UnrelatedClass: " + (unrelated instanceof UnrelatedClass));
        System.out.println("UnrelatedClass instanceof Object: " + (unrelated instanceof Object));

        // Test polymorphic references
        Parent polymorphicChild = new Child();
        System.out.println("Parent ref (Child) instanceof Parent: " + (polymorphicChild instanceof Parent));
        System.out.println("Parent ref (Child) instanceof Child: " + (polymorphicChild instanceof Child));
        System.out.println("Parent ref (Child) instanceof GrandParent: " + (polymorphicChild instanceof GrandParent));
        System.out.println("Parent ref (Child) instanceof Sibling: " + (polymorphicChild instanceof Sibling));

        GrandParent polymorphicParent = new Parent();
        System.out.println("GrandParent ref (Parent) instanceof GrandParent: " + (polymorphicParent instanceof GrandParent));
        System.out.println("GrandParent ref (Parent) instanceof Parent: " + (polymorphicParent instanceof Parent));
        System.out.println("GrandParent ref (Parent) instanceof Child: " + (polymorphicParent instanceof Child));

        Object polymorphicObject = new Child();
        System.out.println("Object ref (Child) instanceof Object: " + (polymorphicObject instanceof Object));
        System.out.println("Object ref (Child) instanceof GrandParent: " + (polymorphicObject instanceof GrandParent));
        System.out.println("Object ref (Child) instanceof Parent: " + (polymorphicObject instanceof Parent));
        System.out.println("Object ref (Child) instanceof Child: " + (polymorphicObject instanceof Child));
        System.out.println("Object ref (Child) instanceof UnrelatedClass: " + (polymorphicObject instanceof UnrelatedClass));
    }
}
