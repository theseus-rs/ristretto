/** Test complex interface inheritance hierarchies */
interface GrandParent {
    void grandParentMethod();
    default void sharedDefault() {
        System.out.println("GrandParent.sharedDefault");
    }
}

interface Parent1 extends GrandParent {
    void parent1Method();
    @Override
    default void sharedDefault() {
        System.out.println("Parent1.sharedDefault");
    }
}

interface Parent2 extends GrandParent {
    void parent2Method();
    @Override
    default void sharedDefault() {
        System.out.println("Parent2.sharedDefault");
    }
}

interface Child extends Parent1, Parent2 {
    void childMethod();
    // Must override sharedDefault due to conflict from Parent1 and Parent2
    @Override
    default void sharedDefault() {
        System.out.println("Child.sharedDefault - resolving conflict");
        Parent1.super.sharedDefault();
        Parent2.super.sharedDefault();
    }
}

interface SiblingInterface {
    void siblingMethod();
    default void anotherDefault() {
        System.out.println("SiblingInterface.anotherDefault");
    }
}

class ComplexImplementation implements Child, SiblingInterface {
    public void grandParentMethod() {
        System.out.println("ComplexImplementation.grandParentMethod");
    }

    public void parent1Method() {
        System.out.println("ComplexImplementation.parent1Method");
    }

    public void parent2Method() {
        System.out.println("ComplexImplementation.parent2Method");
    }

    public void childMethod() {
        System.out.println("ComplexImplementation.childMethod");
    }

    public void siblingMethod() {
        System.out.println("ComplexImplementation.siblingMethod");
    }
}

// Test diamond inheritance resolution
interface DiamondTop {
    default void diamondMethod() {
        System.out.println("DiamondTop.diamondMethod");
    }
}

interface DiamondLeft extends DiamondTop {
    @Override
    default void diamondMethod() {
        System.out.println("DiamondLeft.diamondMethod");
        DiamondTop.super.diamondMethod();
    }
}

interface DiamondRight extends DiamondTop {
    @Override
    default void diamondMethod() {
        System.out.println("DiamondRight.diamondMethod");
        DiamondTop.super.diamondMethod();
    }
}

interface DiamondBottom extends DiamondLeft, DiamondRight {
    @Override
    default void diamondMethod() {
        System.out.println("DiamondBottom.diamondMethod - resolving diamond");
        DiamondLeft.super.diamondMethod();
        DiamondRight.super.diamondMethod();
    }
}

class DiamondImplementation implements DiamondBottom {
    // Inherits the resolved diamond method
}

public class Test {
    public static void main(String[] args) {
        System.out.println("=== Complex Interface Inheritance Test ===");

        ComplexImplementation complex = new ComplexImplementation();
        complex.grandParentMethod();
        complex.parent1Method();
        complex.parent2Method();
        complex.childMethod();
        complex.siblingMethod();
        complex.sharedDefault();
        complex.anotherDefault();

        // Test instanceof with complex hierarchy
        System.out.println("complex instanceof GrandParent: " + (complex instanceof GrandParent));
        System.out.println("complex instanceof Parent1: " + (complex instanceof Parent1));
        System.out.println("complex instanceof Parent2: " + (complex instanceof Parent2));
        System.out.println("complex instanceof Child: " + (complex instanceof Child));
        System.out.println("complex instanceof SiblingInterface: " + (complex instanceof SiblingInterface));

        // Test references at different levels
        GrandParent gpRef = complex;
        Parent1 p1Ref = complex;
        Parent2 p2Ref = complex;
        Child childRef = complex;
        SiblingInterface sibRef = complex;

        System.out.println("=== Via different interface references ===");
        gpRef.grandParentMethod();
        p1Ref.parent1Method();
        p2Ref.parent2Method();
        childRef.childMethod();
        sibRef.siblingMethod();

        // Test diamond inheritance
        System.out.println("=== Diamond Inheritance Test ===");
        DiamondImplementation diamond = new DiamondImplementation();
        diamond.diamondMethod();

        System.out.println("diamond instanceof DiamondTop: " + (diamond instanceof DiamondTop));
        System.out.println("diamond instanceof DiamondLeft: " + (diamond instanceof DiamondLeft));
        System.out.println("diamond instanceof DiamondRight: " + (diamond instanceof DiamondRight));
        System.out.println("diamond instanceof DiamondBottom: " + (diamond instanceof DiamondBottom));
    }
}
